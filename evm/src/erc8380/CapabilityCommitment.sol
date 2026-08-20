// SPDX-License-Identifier: CC0-1.0
pragma solidity ^0.8.28;

/// @title CapabilityCommitment — Canonical Domain-Separated Hashing
/// @notice Computes nullifier and capability commitment exactly as specified
///         in ERC-8380 §Derivation. These functions MUST be used by both the circuit
///         and any Solidity-side parity checks.
/// @dev The domain-separator tag strings are frozen byte-for-byte as written. Even though the
///      assigned ERC number is 8380, the tags read "ERC-1953/..." and MUST NOT be renamed:
///      a Guard and a circuit that disagree on either tag silently accept nothing.
library CapabilityCommitment {
    bytes32 internal constant NULLIFIER_TAG = keccak256("ERC-1953/nullifier/v1");
    bytes32 internal constant CAPABILITY_TAG = keccak256("ERC-1953/capability/v1");

    /// @notice Compute nullifier = H(NULLIFIER_TAG, salt)
    /// @dev The chain id MUST NOT appear in this preimage, or the same credential could be spent
    ///      once per chain. Chain binding is enforced instead by `homeChainId == block.chainid`.
    function computeNullifier(
        bytes32 salt
    ) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(NULLIFIER_TAG, salt));
    }

    /// @notice Compute capabilityCommitment = H(CAPABILITY_TAG, salt, agentId, homeChainId,
    ///         homeDomainId, capabilityIndex, actionCommitment)
    function computeCapabilityCommitment(
        bytes32 salt,
        uint agentId,
        uint homeChainId,
        uint homeDomainId,
        uint capabilityIndex,
        bytes32 actionCommitment
    ) internal pure returns (bytes32) {
        return keccak256(
            abi.encodePacked(
                CAPABILITY_TAG,
                salt,
                bytes32(agentId),
                bytes32(homeChainId),
                bytes32(homeDomainId),
                bytes32(capabilityIndex),
                actionCommitment
            )
        );
    }
}
