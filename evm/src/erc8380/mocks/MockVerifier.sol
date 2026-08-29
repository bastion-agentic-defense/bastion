// SPDX-License-Identifier: CC0-1.0
pragma solidity ^0.8.28;

import { IVerifier } from "../IVerifier.sol";

/// @notice Test double for the ERC-8380 verifier. Returns a settable result.
/// @dev MUST NOT be deployed to a production network (see ERC-8380 Security Considerations).
contract MockVerifier is IVerifier {
    bool public result = true;

    function setResult(
        bool r
    ) external {
        result = r;
    }

    function verify(
        bytes calldata,
        bytes32[] calldata
    ) external view returns (bool) {
        return result;
    }
}
