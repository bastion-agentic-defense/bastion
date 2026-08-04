// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;
// @notice: UNDER ACTIVE DEVELOPMENT - Not production-ready. Bastion's primary deployment target is Solana.

import { EIP712 } from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { IBastionAudit } from "./interfaces/IBastionAudit.sol";
import { IOnChainProof } from "@agent-ercs/anchor/ERC8263/IOnChainProof.sol";

/// @title BastionAudit
/// @notice Immutable on-chain audit trail for all agent transactions.
/// Implements ERC-8263 (OnChain Proof Anchor) from trustless-ai/agent-ercs.
/// Every transaction that passes through the firewall is recorded here
/// with both EIP-712 typed structured data and an AnchorProof event
/// for trustless recompute verification.
/// Entries are append-only and never deleted.
/// @dev Writes are restricted to the configured firewall so entries cannot be
/// forged or spammed by arbitrary callers. The owner (a multisig on mainnet)
/// sets the firewall once after deployment via {setFirewall}.
contract BastionAudit is IBastionAudit, IOnChainProof, EIP712, Ownable {
    bytes32 public constant AUDIT_ENTRY_TYPEHASH = keccak256(
        "AuditEntry(bytes32 id,address agent,address target,bytes4 selector,uint256 value,uint256 gasUsed,bool allowed,bytes reason,uint256 timestamp,uint256 blockNumber)"
    );

    /// @notice The only address permitted to write audit entries (the firewall).
    address public firewall;

    uint private _entryCount;
    mapping(bytes32 entryId => AuditEntry) private _entries;
    mapping(address agent => bytes32[] entryIds) private _agentEntries;
    mapping(bytes32 entryId => uint index) private _agentEntryIndex;

    bytes32 private immutable _DOMAIN_SEPARATOR;

    /// @notice Thrown when a non-firewall address attempts to write an entry.
    error NotFirewall(address caller);
    /// @notice Thrown when attempting to set the firewall to the zero address.
    error ZeroFirewall();

    /// @notice Emitted when the authorized firewall address is set or changed.
    event FirewallUpdated(address indexed previousFirewall, address indexed newFirewall);

    modifier onlyFirewall() {
        if (msg.sender != firewall) revert NotFirewall(msg.sender);
        _;
    }

    /// @param _owner Contract owner (a multisig on mainnet) authorized to set the firewall.
    constructor(
        address _owner
    ) EIP712("BastionAudit", "1.0.0") Ownable(_owner) {
        _DOMAIN_SEPARATOR = _domainSeparatorV4();
    }

    // ──────────────────────────────────────────────────────────────
    // Admin
    // ──────────────────────────────────────────────────────────────

    /// @notice Set the firewall permitted to record audit entries.
    /// @dev Deploy order is Audit -> Firewall, so the firewall address is not
    /// known at construction and must be wired here by the owner afterwards.
    function setFirewall(
        address _firewall
    ) external onlyOwner {
        if (_firewall == address(0)) revert ZeroFirewall();
        emit FirewallUpdated(firewall, _firewall);
        firewall = _firewall;
    }

    // ──────────────────────────────────────────────────────────────
    // Write (callable by Firewall only)
    // ──────────────────────────────────────────────────────────────

    /// @inheritdoc IBastionAudit
    function record(
        address agent,
        address target,
        bytes4 selector,
        uint value,
        uint gasUsed,
        bool allowed,
        bytes calldata reason,
        bytes calldata signature
    ) external override onlyFirewall returns (bytes32 entryId) {
        uint _count = _entryCount;
        bytes32 _id = keccak256(
            abi.encodePacked(
                agent, target, selector, value, gasUsed, allowed, reason, block.timestamp, _count
            )
        );

        AuditEntry memory entry = AuditEntry({
            id: _id,
            agent: agent,
            target: target,
            selector: selector,
            value: value,
            gasUsed: gasUsed,
            allowed: allowed,
            reason: reason,
            timestamp: block.timestamp,
            blockNumber: block.number,
            signature: signature
        });

        _entries[_id] = entry;
        _agentEntries[agent].push(_id);
        _agentEntryIndex[_id] = _agentEntries[agent].length;
        _entryCount = _count + 1;

        emit AuditRecorded(_id, agent, target, selector, allowed, block.timestamp);

        // ERC-8263 AnchorProof: makes this audit entry recompute-able
        // by trustless-ai/agent-sdk. agentIdScheme 0x01 = ERC-8004 REGISTRY.
        emit AnchorProof(
            0x01,
            bytes32(uint256(uint160(agent))),
            _id,
            msg.sender,
            reason
        );

        return _id;
    }

    // ──────────────────────────────────────────────────────────────
    // ERC-8263 - OnChain Proof Anchor
    // ──────────────────────────────────────────────────────────────

    /// @notice Anchor a proof without recording a full audit entry.
    /// Callable by the firewall. Emits an AnchorProof event for
    /// trustless recompute verification by external indexers and
    /// trustless-ai/agent-sdk.
    function anchor(
        uint8 agentIdScheme,
        bytes32 agentId,
        bytes32 proofHash
    ) external override onlyFirewall {
        emit AnchorProof(agentIdScheme, agentId, proofHash, msg.sender, "");
    }

    /// @notice Anchor a proof with opaque aux bytes for adjacent protocols.
    /// Callable by the firewall. The aux field can carry OCP digests
    /// (ERC-8281), session IDs, or parent-proof references without
    /// breaking the canonical event signature.
    function anchorWithAux(
        uint8 agentIdScheme,
        bytes32 agentId,
        bytes32 proofHash,
        bytes calldata aux
    ) external override onlyFirewall {
        emit AnchorProof(agentIdScheme, agentId, proofHash, msg.sender, aux);
    }

    // ──────────────────────────────────────────────────────────────
    // Read
    // ──────────────────────────────────────────────────────────────

    /// @inheritdoc IBastionAudit
    function getEntry(
        bytes32 entryId
    ) external view override returns (AuditEntry memory) {
        return _entries[entryId];
    }

    /// @inheritdoc IBastionAudit
    function getEntriesByAgent(
        address agent,
        uint fromTimestamp,
        uint toTimestamp
    ) external view override returns (AuditEntry[] memory) {
        bytes32[] storage ids = _agentEntries[agent];
        uint count;
        for (uint i = 0; i < ids.length; i++) {
            AuditEntry storage e = _entries[ids[i]];
            if (e.timestamp >= fromTimestamp && e.timestamp <= toTimestamp) {
                count++;
            }
        }

        AuditEntry[] memory results = new AuditEntry[](count);
        uint idx;
        for (uint i = 0; i < ids.length; i++) {
            AuditEntry storage e = _entries[ids[i]];
            if (e.timestamp >= fromTimestamp && e.timestamp <= toTimestamp) {
                results[idx] = e;
                idx++;
            }
        }

        return results;
    }

    /// @inheritdoc IBastionAudit
    function getEntryCount() external view override returns (uint) {
        return _entryCount;
    }

    /// @notice Get the total number of entries for a specific agent.
    function getAgentEntryCount(
        address agent
    ) external view returns (uint) {
        return _agentEntries[agent].length;
    }

    /// @notice Get the domain separator for EIP-712 verification.
    function DOMAIN_SEPARATOR() external view returns (bytes32) {
        return _DOMAIN_SEPARATOR;
    }
}
