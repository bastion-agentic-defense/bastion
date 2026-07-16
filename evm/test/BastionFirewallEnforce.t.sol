// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import { Test } from "forge-std/Test.sol";
import { BastionPolicy } from "../src/BastionPolicy.sol";
import { BastionAudit } from "../src/BastionAudit.sol";
import { BastionFirewall } from "../src/BastionFirewall.sol";
import { IBastionPolicy } from "../src/interfaces/IBastionPolicy.sol";
import { IBastionAudit } from "../src/interfaces/IBastionAudit.sol";
import { IBastionFirewall, PackedUserOperation } from "../src/interfaces/IBastionFirewall.sol";

/// @notice Exercises the validation/enforcement split (B2), the calldata-decode
/// bound (B3), and the policy array cap (B4) added for mainnet readiness.
contract BastionFirewallEnforceTest is Test {
    BastionPolicy public policy;
    BastionAudit public audit;
    BastionFirewall public firewall;

    address public owner = makeAddr("owner");
    address public agent = makeAddr("agent");
    address public target = makeAddr("target");

    uint internal constant SIG_OK = 0;
    uint internal constant SIG_FAIL = 1;
    bytes4 internal constant TRANSFER_SEL = bytes4(keccak256("transfer(address,uint256)"));

    function setUp() public {
        vm.startPrank(owner);
        audit = new BastionAudit(owner);
        policy = new BastionPolicy(owner);
        firewall = new BastionFirewall(
            IBastionPolicy(address(policy)), IBastionAudit(address(audit)), owner
        );
        audit.setFirewall(address(firewall));

        address[] memory targets = new address[](1);
        targets[0] = target;
        bytes4[] memory selectors = new bytes4[](1);
        selectors[0] = TRANSFER_SEL;

        policy.setPolicy(
            agent,
            IBastionPolicy.Policy({
                agent: agent,
                isActive: true,
                maxValuePerTx: 10 ether,
                maxGasPerTx: 1_000_000,
                dailyTxLimit: 100,
                cooldownSeconds: 0,
                allowedTargets: targets,
                allowedSelectors: selectors,
                extraData: ""
            })
        );
        vm.stopPrank();

        vm.prank(agent);
        firewall.onInstall("");
    }

    // Bastion execution calldata layout: target word | value word | selector (+ params).
    function _callData(
        address _target,
        uint _value,
        bytes4 _selector
    ) internal pure returns (bytes memory) {
        return abi.encodePacked(bytes32(uint(uint160(_target))), _value, _selector);
    }

    function _userOp(
        bytes memory callData
    ) internal view returns (PackedUserOperation memory op) {
        op.sender = agent;
        op.callData = callData;
    }

    // ── B2: validateUserOp is view-only and returns codes, never writes/reverts on policy ──

    function test_ValidateUserOp_AllowedReturnsZero() public view {
        uint v =
            firewall.validateUserOp(_userOp(_callData(target, 1 ether, TRANSFER_SEL)), bytes32(0));
        assertEq(v, SIG_OK);
        // No audit entry written during validation.
        assertEq(audit.getEntryCount(), 0);
    }

    function test_ValidateUserOp_BlockedReturnsFailure() public view {
        // Disallowed selector -> policy blocks -> SIG_VALIDATION_FAILED, no revert.
        uint v = firewall.validateUserOp(
            _userOp(_callData(target, 1 ether, bytes4(0xdeadbeef))), bytes32(0)
        );
        assertEq(v, SIG_FAIL);
    }

    function test_ValidateUserOp_NotInstalledReturnsFailure() public {
        PackedUserOperation memory op = _userOp(_callData(target, 1 ether, TRANSFER_SEL));
        op.sender = makeAddr("uninstalled");
        assertEq(firewall.validateUserOp(op, bytes32(0)), SIG_FAIL);
    }

    // ── B2: enforce writes audit + reverts on block, records on allow ──

    function test_Enforce_AllowedRecordsAudit() public {
        vm.prank(agent);
        firewall.enforce(_userOp(_callData(target, 1 ether, TRANSFER_SEL)));
        assertEq(audit.getEntryCount(), 1);
    }

    function test_Enforce_BlockedRevertsAndRecords() public {
        vm.prank(agent);
        vm.expectRevert(
            abi.encodeWithSignature(
                "NotAuthorized(address,address,bytes4)", agent, target, bytes4(0xdeadbeef)
            )
        );
        firewall.enforce(_userOp(_callData(target, 1 ether, bytes4(0xdeadbeef))));
    }

    // ── B3: decode bound rejects short calldata instead of underflowing ──

    function test_Enforce_RevertsOnShortCallData() public {
        vm.prank(agent);
        vm.expectRevert(bytes("callData too short"));
        firewall.enforce(_userOp(hex"deadbeef")); // 4 bytes < 68
    }

    // ── B4: policy array cap ──

    function test_SetPolicy_RevertsOnTooManyTargets() public {
        address[] memory targets = new address[](33); // > _MAX_TARGETS
        bytes4[] memory selectors = new bytes4[](1);
        selectors[0] = TRANSFER_SEL;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSignature("TooManyEntries(uint256,uint256)", 33, 1));
        policy.setPolicy(
            agent,
            IBastionPolicy.Policy({
                agent: agent,
                isActive: true,
                maxValuePerTx: 1 ether,
                maxGasPerTx: 1_000_000,
                dailyTxLimit: 100,
                cooldownSeconds: 0,
                allowedTargets: targets,
                allowedSelectors: selectors,
                extraData: ""
            })
        );
    }
}
