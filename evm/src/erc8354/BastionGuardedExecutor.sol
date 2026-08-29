// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import { IConfidentialPolicyVerdict, Verdict } from "./IConfidentialPolicyVerdict.sol";
import { IPolicyGuarded } from "./IPolicyGuarded.sol";
import { PolicyAction, PolicyActionLib } from "./PolicyAction.sol";
import { IValidationRegistry, PolicyAttestation } from "./IPolicyAttestation.sol";

/// @notice Example guarded contract. Recomputes the canonical actionCommitment from the action
/// it is about to dispatch, requires it match the verdict, consumes the verdict, writes the
/// ERC-8004 validation attestation, then executes.
/// @dev Action-binding lives here (not in the Guard): `consume` receives a `Verdict` but not the
/// action params, so this contract recomputes the commitment via the canonical `PolicyActionLib`
/// — the same preimage the proving program uses. The executor question is resolved cryptographically:
/// pass `executorAuth = ""` to consume on this contract's own behalf (v.executor == this), or pass a
/// signature by an end-user `v.executor` and this contract relays it. Because the action is committed
/// and the executor is bound by signature, front-running `execute()` is neutral.
contract BastionGuardedExecutor is IPolicyGuarded {
    using PolicyActionLib for PolicyAction;

    IConfidentialPolicyVerdict public immutable guard;
    bytes32 public immutable domainId;
    /// @dev ERC-8004 Validation Registry the consumed verdict's attestation is written to.
    ///      address(0) disables the handoff.
    IValidationRegistry public immutable validationRegistry;

    mapping(uint => uint) public actionNonce; // per ERC-8004 agentId, monotonic

    error ActionCommitmentMismatch(bytes32 expected, bytes32 got);
    error ActionFailed();

    constructor(
        IConfidentialPolicyVerdict _guard,
        bytes32 _domainId,
        IValidationRegistry _validationRegistry
    ) {
        guard = _guard;
        domainId = _domainId;
        validationRegistry = _validationRegistry;
    }

    function policyDomain() external view returns (bytes32) {
        return domainId;
    }

    /// @dev The canonical commitment: binds chainId + domainId (replay separation), agentId,
    /// the action, and a monotonic per-agent nonce.
    function actionCommitmentOf(
        uint agentId,
        address target,
        uint value,
        bytes calldata callData
    ) public view returns (bytes32) {
        return PolicyAction({
                chainId: block.chainid,
                domainId: domainId,
                agentId: agentId,
                target: target,
                value: value,
                callDataHash: keccak256(callData),
                actionNonce: actionNonce[agentId]
            }).commit();
    }

    function execute(
        Verdict calldata v,
        bytes calldata proof,
        bytes calldata executorAuth,
        address target,
        uint value,
        bytes calldata callData
    ) external returns (bytes memory) {
        bytes32 expected = actionCommitmentOf(v.agentId, target, value, callData);
        if (expected != v.actionCommitment) {
            revert ActionCommitmentMismatch(expected, v.actionCommitment);
        }
        actionNonce[v.agentId] += 1;

        guard.consume(v, proof, executorAuth); // reverts the whole tx on any failure

        // CEI: the nullifier is burned (in consume) before the authorized call. The attestation
        // handoff also precedes the call so a consumed verdict is always recorded, even if the
        // target subsequently reverts (the whole tx reverts together, keeping the two atomic).
        if (address(validationRegistry) != address(0)) {
            validationRegistry.recordVerdict(PolicyAttestation.attestationFor(v));
        }

        (bool ok, bytes memory ret) = target.call{ value: value }(callData);
        if (!ok) revert ActionFailed();
        return ret;
    }
}
