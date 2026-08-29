// SPDX-License-Identifier: CC0-1.0
pragma solidity ^0.8.28;

import { IValidationRegistry, VerdictAttestation } from "../IPolicyAttestation.sol";

/// @notice Test double for an ERC-8004-style Validation Registry — records the last attestation
/// per (agentId, nullifier) so a test can assert the handoff payload.
contract MockValidationRegistry is IValidationRegistry {
    mapping(uint => mapping(bytes32 => VerdictAttestation)) public recorded;

    function recordVerdict(
        VerdictAttestation calldata a
    ) external {
        recorded[a.agentId][a.nullifier] = a;
        emit VerdictRecorded(a.agentId, a.nullifier, a.mechanism);
    }

    function get(
        uint agentId,
        bytes32 nullifier
    ) external view returns (VerdictAttestation memory) {
        return recorded[agentId][nullifier];
    }
}
