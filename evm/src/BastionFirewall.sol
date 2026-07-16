// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;
// @notice: UNDER ACTIVE DEVELOPMENT — Not production-ready. Bastion's primary deployment target is Solana.

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { Pausable } from "@openzeppelin/contracts/utils/Pausable.sol";
import { ReentrancyGuard } from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import { EIP712 } from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { IBastionFirewall, PackedUserOperation } from "./interfaces/IBastionFirewall.sol";
import { IBastionPolicy } from "./interfaces/IBastionPolicy.sol";
import { IBastionAudit } from "./interfaces/IBastionAudit.sol";

/// @title BastionFirewall
/// @notice ERC-7579 compatible validator that enforces agent transaction policies.
/// Every agent transaction passes through this firewall before execution.
/// Blocks unauthorized targets, selectors, value limits, gas limits,
/// rate limits, and cooldown violations.
contract BastionFirewall is IBastionFirewall, Ownable, Pausable, ReentrancyGuard, EIP712 {
    using ECDSA for bytes32;

    bytes32 private constant _VALIDATOR_TYPEHASH =
        keccak256("BastionValidator(address account,uint256 chainId)");

    bytes4 internal constant ERC1271_MAGIC_VALUE = 0x1626ba7e;
    bytes4 internal constant VALIDATOR_VALID = 0x7b3b2015;

    /// @dev ERC-4337 validation return codes.
    uint internal constant SIG_VALIDATION_SUCCESS = 0;
    uint internal constant SIG_VALIDATION_FAILED = 1;

    /// @dev Bastion execution calldata layout:
    ///   [0:32]  target  (ABI word, address right-aligned)
    ///   [32:64] value   (uint256)
    ///   [64:]   inner   (the actual call, starting with its 4-byte selector)
    /// `_HEADER_LEN` is the fixed prefix; `_MIN_CALLDATA_LEN` also requires the 4-byte
    /// selector so the target/value/selector decode never reads past the slice.
    uint internal constant _HEADER_LEN = 64;
    uint internal constant _MIN_CALLDATA_LEN = 68;

    IBastionPolicy public immutable policyEngine;
    IBastionAudit public immutable auditLog;

    mapping(address account => bytes32 validatorHash) private _installedAccounts;

    /// @param _policyEngine The BastionPolicy contract address.
    /// @param _auditLog The BastionAudit contract address.
    /// @param _owner The owner of the firewall contract.
    constructor(
        IBastionPolicy _policyEngine,
        IBastionAudit _auditLog,
        address _owner
    ) Ownable(_owner) EIP712("BastionFirewall", "1.0.0") {
        require(address(_policyEngine) != address(0), "zero policy engine");
        require(address(_auditLog) != address(0), "zero audit log");
        policyEngine = _policyEngine;
        auditLog = _auditLog;
    }

    // ──────────────────────────────────────────────────────────────
    // ERC-7579 IValidator Interface
    // ──────────────────────────────────────────────────────────────

    /// @inheritdoc IBastionFirewall
    /// @dev ERC-4337 validation MUST be side-effect free with respect to external
    /// contract storage (bundlers reject ops whose validation touches storage outside
    /// the validator/account). We therefore only read the policy (a view call) here and
    /// return a validation code — 0 when allowed, SIG_VALIDATION_FAILED when the policy
    /// blocks. The audit-trail write and the hard revert-on-block happen in {enforce},
    /// which the account calls in the execution phase where state writes are permitted.
    function validateUserOp(
        PackedUserOperation calldata userOp,
        bytes32 /* userOpHash */
    ) external view override whenNotPaused returns (uint validationData) {
        address agent = userOp.sender;

        if (_installedAccounts[agent] == bytes32(0)) {
            return SIG_VALIDATION_FAILED;
        }
        if (userOp.callData.length < _MIN_CALLDATA_LEN) {
            return SIG_VALIDATION_FAILED;
        }

        (address target, uint value,,) = _decodeCallData(userOp.callData);

        // Policy reads the selector from the start of the inner calldata.
        (bool allowed,) =
            policyEngine.checkTransaction(agent, target, value, userOp.callData[_HEADER_LEN:]);

        return allowed ? SIG_VALIDATION_SUCCESS : SIG_VALIDATION_FAILED;
    }

    /// @notice Execution-phase enforcement: re-checks the policy, writes the audit
    /// entry, and reverts if the transaction is not allowed. Called by the smart
    /// account during execution (where external state writes are permitted), so the
    /// audit trail is recorded atomically with the enforced decision.
    /// @param userOp The user operation being executed.
    /// @return target Decoded call target.
    /// @return value Decoded call value.
    /// @return selector Decoded call selector.
    function enforce(
        PackedUserOperation calldata userOp
    )
        external
        override
        whenNotPaused
        nonReentrant
        returns (address target, uint value, bytes4 selector)
    {
        address agent = userOp.sender;

        if (_installedAccounts[agent] == bytes32(0)) {
            revert NotAuthorized(agent, address(0), bytes4(0));
        }

        (target, value, selector,) = _decodeCallData(userOp.callData);

        (bool allowed, bytes memory reason) =
            policyEngine.checkTransaction(agent, target, value, userOp.callData[_HEADER_LEN:]);

        uint gasBefore = gasleft();

        if (!allowed) {
            auditLog.record(
                agent,
                target,
                selector,
                value,
                gasBefore - gasleft(),
                false,
                reason,
                userOp.signature
            );
            emit TransactionBlocked(agent, target, selector, value, block.timestamp, reason);
            revert NotAuthorized(agent, target, selector);
        }

        auditLog.record(
            agent, target, selector, value, gasBefore - gasleft(), true, "", userOp.signature
        );

        emit TransactionAllowed(agent, target, selector, value, block.timestamp);
    }

    /// @inheritdoc IBastionFirewall
    function isValidForAccount(
        address account
    ) external view override returns (bytes4) {
        return _installedAccounts[account] != bytes32(0) ? VALIDATOR_VALID : bytes4(0);
    }

    /// @inheritdoc IBastionFirewall
    function onInstall(
        bytes calldata /* data */
    ) external override {
        address account = msg.sender;
        bytes32 hash = keccak256(abi.encode(_VALIDATOR_TYPEHASH, account, block.chainid));
        _installedAccounts[account] = hash;
        emit PolicyUpdated(account, address(0), bytes4(0), true, block.timestamp);
    }

    /// @inheritdoc IBastionFirewall
    function onUninstall(
        bytes calldata
    ) external override {
        address account = msg.sender;
        delete _installedAccounts[account];
    }

    // ──────────────────────────────────────────────────────────────
    // Admin Functions
    // ──────────────────────────────────────────────────────────────

    /// @notice Pause the firewall. No transactions pass while paused.
    function pause() external onlyOwner {
        _pause();
    }

    /// @notice Unpause the firewall.
    function unpause() external onlyOwner {
        _unpause();
    }

    /// @notice Check if an account has the Bastion validator installed.
    function isInstalled(
        address account
    ) external view returns (bool) {
        return _installedAccounts[account] != bytes32(0);
    }

    // ──────────────────────────────────────────────────────────────
    // Internal Helpers
    // ──────────────────────────────────────────────────────────────

    function _decodeCallData(
        bytes calldata callData
    ) internal pure returns (address target, uint value, bytes4 selector, bytes memory params) {
        // Require the full fixed header + selector so the assembly reads never run
        // past the slice and the inner-calldata slice below cannot underflow.
        require(callData.length >= _MIN_CALLDATA_LEN, "callData too short");
        // solhint-disable-next-line no-inline-assembly
        assembly {
            // Mask to the low 20 bytes so a dirty high word cannot spoof the target.
            target := and(calldataload(callData.offset), 0xffffffffffffffffffffffffffffffffffffffff)
            value := calldataload(add(callData.offset, 32))
            // selector is the high 4 bytes of the inner calldata at offset 64.
            selector := calldataload(add(callData.offset, _HEADER_LEN))
        }
        // Inner calldata (starts with the 4-byte selector), forwarded to the policy.
        params = callData[_HEADER_LEN:];
    }
}
