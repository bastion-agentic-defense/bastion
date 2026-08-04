// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;
// @notice: UNDER ACTIVE DEVELOPMENT — Not production-ready.

import {IConfidentialPolicyVerdict, Verdict} from "@capv/IConfidentialPolicyVerdict.sol";
import {PolicyAction, PolicyActionLib} from "@capv/PolicyAction.sol";
import {IBastionPolicy} from "./interfaces/IBastionPolicy.sol";

/// @title BastionConfidentialGate
/// @notice Composable gate that combines confidential ZK policy verdicts (CAPV)
///         with Bastion's programmable public policy engine.
///
///         An action must survive TWO independent evaluations before execution:
///
///         1. CONFIDENTIAL (ZK): A Noir circuit proves the action was evaluated
///            against a committed secret policy and permitted. The policy rules
///            are NEVER revealed on-chain — only the ZK proof.
///
///         2. PUBLIC (Bastion): Bastion's transparent policy engine evaluates
///            the action against programmable rules (amount caps, allowlists,
///            rate limits, HITL gates, etc.) — fully auditable.
///
///         This gives deployments the full confidentiality spectrum: secret
///         organizational policies guarded by ZK proofs, layered with
///         transparent programmable trust rules for verifiable compliance.
contract BastionConfidentialGate {
    using PolicyActionLib for PolicyAction;

    IConfidentialPolicyVerdict public immutable capvGuard;
    IBastionPolicy public immutable bastionPolicy;
    bytes32 public immutable domainId;

    error ConfidentialVerdictFailed();
    error BastionPolicyBlocked(bytes reason);
    error ActionCommitmentMismatch(bytes32 expected, bytes32 actual);

    event ConfidentialVerdictConsumed(
        bytes32 indexed nullifier,
        uint256 indexed agentId,
        bytes32 policyRoot
    );
    event BastionPolicyPassed(
        address indexed agent,
        address indexed target,
        bytes4 selector
    );

    constructor(
        IConfidentialPolicyVerdict _capv,
        IBastionPolicy _bastion,
        bytes32 _domainId
    ) {
        capvGuard = _capv;
        bastionPolicy = _bastion;
        domainId = _domainId;
    }

    /// @notice Execute an action gated by both confidential and public policy.
    /// @param v          The CAPV verdict (ZK proof that secret policy allows this).
    /// @param proof      The ZK proof bytes (UltraHonk proof from Noir circuit).
    /// @param agent      The agent address (for Bastion's public policy lookup).
    /// @param target     The call target address.
    /// @param data       The call data to execute.
    /// @param actionNonce Monotonic nonce for replay protection.
    function executeDualGate(
        Verdict calldata v,
        bytes calldata proof,
        address agent,
        address target,
        bytes calldata data,
        uint256 actionNonce
    )
        external
        payable
    {
        // ── Layer 1: Confidential ZK verdict ──────────────────────────
        bytes32 commitment = PolicyAction({
            chainId: block.chainid,
            domainId: domainId,
            agentId: v.agentId,
            target: target,
            value: msg.value,
            callDataHash: keccak256(data),
            actionNonce: actionNonce
        }).commit();

        if (commitment != v.actionCommitment) {
            revert ActionCommitmentMismatch(v.actionCommitment, commitment);
        }

        capvGuard.consume(v, proof);
        emit ConfidentialVerdictConsumed(v.nullifier, v.agentId, v.policyRoot);

        // ── Layer 2: Public Bastion policy ────────────────────────────
        (bool allowed, bytes memory reason) = bastionPolicy.checkTransaction(
            agent, target, msg.value, data
        );

        if (!allowed) {
            revert BastionPolicyBlocked(reason);
        }
        emit BastionPolicyPassed(agent, target, bytes4(data));

        // ── Execute ───────────────────────────────────────────────────
        (bool ok,) = target.call{value: msg.value}(data);
        require(ok, "execution failed");
    }

    /// @notice Verify both policy layers without executing.
    /// @return confidentialOk True if the ZK proof verifies.
    /// @return publicOk      True if Bastion's public policy passes.
    /// @return publicReason  Reason if Bastion's policy does not pass.
    function preflight(
        Verdict calldata v,
        bytes calldata proof,
        address agent,
        address target,
        bytes calldata data,
        uint256 actionNonce
    )
        external
        view
        returns (bool confidentialOk, bool publicOk, bytes memory publicReason)
    {
        bytes32 commitment = PolicyAction({
            chainId: block.chainid,
            domainId: domainId,
            agentId: v.agentId,
            target: target,
            value: 0,
            callDataHash: keccak256(data),
            actionNonce: actionNonce
        }).commit();

        if (commitment != v.actionCommitment) return (false, false, "action commitment mismatch");

        confidentialOk = capvGuard.verify(v, proof);
        (publicOk, publicReason) = bastionPolicy.checkTransaction(
            agent, target, 0, data
        );
    }
}
