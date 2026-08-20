// SPDX-License-Identifier: CC0-1.0
pragma solidity ^0.8.28;

/// @notice A contract gated by a policy verdict.
interface IPolicyGuarded {
    function policyDomain() external view returns (bytes32);
}
