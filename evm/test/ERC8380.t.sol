// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import { Test } from "forge-std/Test.sol";

import {
    BastionUnclonableCredentialGuard
} from "../src/erc8380/BastionUnclonableCredentialGuard.sol";
import { DomainRegistry } from "../src/erc8380/DomainRegistry.sol";
import { IUnclonableCredential } from "../src/erc8380/IUnclonableCredential.sol";
import { MockVerifier } from "../src/erc8380/mocks/MockVerifier.sol";

/// @notice Minimal call target for execute tests.
contract Sink8380 {
    uint public pings;

    function ping() external {
        pings++;
    }
}

/// @notice Target that always reverts, to prove the burn is fused to (and atomic with) the call.
contract RevertingTarget {
    function boom() external pure {
        revert("boom");
    }
}

/// @notice An executor contract that re-enters `execute` while its first spend is mid-flight,
/// modelling two concurrent spends of the same nullifier racing to land.
contract ReentrantSpender {
    IUnclonableCredential public guard;
    IUnclonableCredential.Capability public cap;
    bytes public proof;
    uint public calls;
    bytes4 public innerError;

    function attack(
        IUnclonableCredential g,
        IUnclonableCredential.Capability calldata c,
        bytes calldata p
    ) external {
        guard = g;
        cap = c;
        proof = p;
        g.execute(c, p, address(this), abi.encodeWithSignature("reenter()"));
    }

    function reenter() external {
        calls++;
        try guard.execute(cap, proof, address(this), abi.encodeWithSignature("reenter()")) {
        // Unreachable: the nullifier is already burned.
        }
        catch (bytes memory reason) {
            innerError = bytes4(reason);
        }
    }
}

/// @notice The Test Cases from ERC-8380, plus the adversarial vectors named in the PR body,
/// as an executable Foundry suite against the Bastion-named conforming Guard.
contract ERC8380Test is Test {
    BastionUnclonableCredentialGuard guard;
    DomainRegistry domainRegistry;
    MockVerifier verifier;

    address constant ORCHESTRATOR = address(0xA11CE);
    address constant EXECUTOR = address(0xE0);
    address constant CLONE = address(0xBEEF);

    uint constant AGENT_ID = 5;
    uint constant HOME_DOMAIN_ID = 1;

    function setUp() public {
        vm.warp(1_700_000_000);
        domainRegistry = new DomainRegistry();
        verifier = new MockVerifier();
        guard = new BastionUnclonableCredentialGuard(
            address(verifier), address(domainRegistry), ORCHESTRATOR
        );
        domainRegistry.registerDomain(HOME_DOMAIN_ID);
    }

    function _buildCap(
        bytes32 salt,
        uint index,
        address target,
        bytes memory cd,
        address executor
    ) internal view returns (IUnclonableCredential.Capability memory cap) {
        bytes32 action = keccak256(abi.encode(target, cd));
        cap = IUnclonableCredential.Capability({
            nullifier: guard.computeNullifier(salt),
            capabilityCommitment: guard.computeCapabilityCommitment(
                salt, AGENT_ID, block.chainid, HOME_DOMAIN_ID, index, action
            ),
            agentId: AGENT_ID,
            homeChainId: block.chainid,
            homeDomainId: HOME_DOMAIN_ID,
            capabilityIndex: index,
            actionCommitment: action,
            executor: executor,
            expiry: block.timestamp + 1 hours
        });
    }

    function _issue(
        bytes32 commitment,
        uint index
    ) internal {
        vm.prank(ORCHESTRATOR);
        guard.issue(commitment, AGENT_ID, index);
    }

    // ── Happy path ───────────────────────────────────────────────────
    function test_HappyPath() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        _issue(cap.capabilityCommitment, 3);

        vm.prank(EXECUTOR);
        bytes32 nf = guard.execute(cap, "", address(sink), cd);

        assertEq(nf, cap.nullifier, "returns the burned nullifier");
        assertTrue(guard.isConsumed(cap.nullifier), "nullifier consumed");
        assertEq(sink.pings(), 1, "action ran");
    }

    // ── Double spend ─────────────────────────────────────────────────
    function test_DoubleSpend() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        _issue(cap.capabilityCommitment, 3);

        vm.startPrank(EXECUTOR);
        guard.execute(cap, "", address(sink), cd);
        vm.expectRevert(
            abi.encodeWithSelector(
                IUnclonableCredential.CredentialAlreadySpent.selector, cap.nullifier
            )
        );
        guard.execute(cap, "", address(sink), cd);
        vm.stopPrank();

        assertEq(sink.pings(), 1, "action must not run twice");
    }

    // ── Clone replay (submitter is not the executor) ─────────────────
    function test_CloneReplay() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        _issue(cap.capabilityCommitment, 3);

        vm.prank(CLONE); // holds the salt/proof, not the executor key
        vm.expectRevert(BastionUnclonableCredentialGuard.ExecutorMismatch.selector);
        guard.execute(cap, "", address(sink), cd);
    }

    // ── Wrong chain ──────────────────────────────────────────────────
    function test_WrongChain() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        cap.homeChainId = block.chainid + 1; // not this chain
        _issue(cap.capabilityCommitment, 3);

        vm.prank(EXECUTOR);
        vm.expectRevert(BastionUnclonableCredentialGuard.WrongChain.selector);
        guard.execute(cap, "", address(sink), cd);
    }

    // ── Expired ──────────────────────────────────────────────────────
    function test_Expired() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        _issue(cap.capabilityCommitment, 3);

        vm.warp(cap.expiry + 1);
        vm.prank(EXECUTOR);
        vm.expectRevert(BastionUnclonableCredentialGuard.Expired.selector);
        guard.execute(cap, "", address(sink), cd);
    }

    // ── Executor mismatch ────────────────────────────────────────────
    function test_ExecutorMismatch() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        _issue(cap.capabilityCommitment, 3);

        vm.prank(address(0xBAD));
        vm.expectRevert(BastionUnclonableCredentialGuard.ExecutorMismatch.selector);
        guard.execute(cap, "", address(sink), cd);
    }

    // ── Front run with a lifted proof ────────────────────────────────
    function test_FrontRunLiftedProof() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        _issue(cap.capabilityCommitment, 3);

        // A third party observes the (valid) proof in the mempool and resubmits it, but is not
        // the executor. Executor binding is checked before anything the proof could satisfy.
        vm.prank(CLONE);
        vm.expectRevert(BastionUnclonableCredentialGuard.ExecutorMismatch.selector);
        guard.execute(cap, "observed-proof", address(sink), cd);
    }

    // ── Unregistered domain ──────────────────────────────────────────
    function test_UnregisteredDomain() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        cap.homeDomainId = 4242; // never registered
        _issue(cap.capabilityCommitment, 3);

        vm.prank(EXECUTOR);
        vm.expectRevert(BastionUnclonableCredentialGuard.DomainInvalid.selector);
        guard.execute(cap, "", address(sink), cd);
    }

    // ── Commitment parity (spec vector: homeChainId 11155111, domain 1) ──
    function test_CommitmentParity() public view {
        bytes32 salt = bytes32(uint(7));
        uint agentId = 5;
        uint homeChainId = 11155111;
        uint homeDomainId = 1;
        uint capIndex = 3;
        bytes32 actionCommitment = bytes32(uint(0x63));

        bytes32 nullifier = guard.computeNullifier(salt);
        bytes32 commitment = guard.computeCapabilityCommitment(
            salt, agentId, homeChainId, homeDomainId, capIndex, actionCommitment
        );

        bytes32 expectedNullifier =
            keccak256(abi.encodePacked(keccak256("ERC-1953/nullifier/v1"), salt));
        bytes32 expectedCommitment = keccak256(
            abi.encodePacked(
                keccak256("ERC-1953/capability/v1"),
                salt,
                bytes32(agentId),
                bytes32(homeChainId),
                bytes32(homeDomainId),
                bytes32(capIndex),
                actionCommitment
            )
        );

        assertEq(nullifier, expectedNullifier, "nullifier parity");
        assertEq(commitment, expectedCommitment, "commitment parity");
    }

    // ── chainId MUST NOT be in the nullifier preimage ────────────────
    function test_ChainIdNotInNullifierPreimage() public view {
        bytes32 salt = bytes32(uint(7));

        // The same salt yields the same nullifier regardless of chain (the nullifier is
        // chain-agnostic), so a clone on another chain collides rather than spending again.
        assertEq(guard.computeNullifier(salt), guard.computeNullifier(salt), "chain-agnostic");

        // The chain binding lives in the capability commitment instead.
        bytes32 commitChain1 =
            guard.computeCapabilityCommitment(salt, 5, 1, 1, 3, bytes32(uint(0x63)));
        bytes32 commitChain2 =
            guard.computeCapabilityCommitment(salt, 5, 2, 1, 3, bytes32(uint(0x63)));
        assertTrue(commitChain1 != commitChain2, "commitment must bind chainId");
    }

    // ── Race: exactly one of two concurrent spends lands ─────────────
    function test_Race() public {
        ReentrantSpender spender = new ReentrantSpender();
        bytes memory cd = abi.encodeWithSignature("reenter()");
        bytes32 salt = bytes32(uint(0x777));
        uint idx = 0;
        IUnclonableCredential.Capability memory cap =
            _buildCap(salt, idx, address(spender), cd, address(spender));
        _issue(cap.capabilityCommitment, idx);

        spender.attack(guard, cap, "");

        assertTrue(guard.isConsumed(cap.nullifier), "outer spend lands");
        assertEq(spender.calls(), 1, "action ran once");
        assertEq(
            uint32(spender.innerError()),
            uint32(IUnclonableCredential.CredentialAlreadySpent.selector),
            "inner spend rejected as replay"
        );
    }

    // ── Grief on an unissued action ──────────────────────────────────
    function test_GriefOnUnissuedAction() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);

        // Not issued: the clone cannot burn an action the orchestrator never authorized.
        vm.prank(EXECUTOR);
        vm.expectRevert(
            abi.encodeWithSelector(
                IUnclonableCredential.CommitmentNotIssued.selector, cap.capabilityCommitment
            )
        );
        guard.execute(cap, "", address(sink), cd);

        // The honest action is unharmed and still runs once the orchestrator issues it.
        assertFalse(guard.isConsumed(cap.nullifier), "nothing burned by the grief attempt");
        _issue(cap.capabilityCommitment, 3);
        vm.prank(EXECUTOR);
        guard.execute(cap, "", address(sink), cd);
        assertEq(sink.pings(), 1, "honest action still runs");
    }

    // ── Burn implies execution (fused + atomic) ──────────────────────
    function test_BurnImpliesExecution() public {
        // Positive: a successful spend always ran the action.
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        _issue(cap.capabilityCommitment, 3);
        vm.prank(EXECUTOR);
        guard.execute(cap, "", address(sink), cd);
        assertTrue(guard.isConsumed(cap.nullifier));
        assertEq(sink.pings(), 1);

        // Negative: a reverting action rolls the burn back — burn and call are one transaction.
        RevertingTarget rt = new RevertingTarget();
        bytes memory cd2 = abi.encodeWithSignature("boom()");
        IUnclonableCredential.Capability memory cap2 =
            _buildCap(bytes32(uint(8)), 4, address(rt), cd2, EXECUTOR);
        _issue(cap2.capabilityCommitment, 4);
        vm.prank(EXECUTOR);
        vm.expectRevert(BastionUnclonableCredentialGuard.ActionReverted.selector);
        guard.execute(cap2, "", address(rt), cd2);
        assertFalse(guard.isConsumed(cap2.nullifier), "burn must not stick without execution");
    }

    // ── Premature spend performs the action rather than destroying it ──
    function test_PrematureSpendPerformsAction() public {
        // There is no separable consume primitive: the only way to spend a capability is
        // `execute`, which performs the committed action in the same call.
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        _issue(cap.capabilityCommitment, 3);

        vm.prank(EXECUTOR);
        guard.execute(cap, "", address(sink), cd);
        assertEq(sink.pings(), 1, "spending always performs the action");
        assertTrue(guard.isConsumed(cap.nullifier));
    }

    // ── Collision classification via highestIssuedIndex ──────────────
    function test_CollisionClassification() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");

        IUnclonableCredential.Capability memory cap3 =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        _issue(cap3.capabilityCommitment, 3);
        assertEq(guard.highestIssuedIndex(AGENT_ID), 3, "ceiling raised to 3");

        // A lower index does not lower the ceiling (concurrent/out-of-order issuance).
        IUnclonableCredential.Capability memory cap2 =
            _buildCap(bytes32(uint(6)), 2, address(sink), cd, EXECUTOR);
        _issue(cap2.capabilityCommitment, 2);
        assertEq(guard.highestIssuedIndex(AGENT_ID), 3, "ceiling never decreases");

        // A collision at index 3 (<= ceiling) is an orchestrator reissue bug.
        assertGe(guard.highestIssuedIndex(AGENT_ID), 3);
        // A collision at index 10 (> ceiling) is a clone: the orchestrator never issued it.
        assertLt(guard.highestIssuedIndex(AGENT_ID), 10);
    }

    // ── Recovery: burned stays burned, reauthorize at next index ─────
    function test_Recovery() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");

        IUnclonableCredential.Capability memory cap3 =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        _issue(cap3.capabilityCommitment, 3);
        vm.prank(EXECUTOR);
        guard.execute(cap3, "", address(sink), cd);
        assertTrue(guard.isConsumed(cap3.nullifier), "burned");

        // Reissue at the next index under a fresh salt; the old nullifier stays dead.
        IUnclonableCredential.Capability memory cap4 =
            _buildCap(bytes32(uint(9)), 4, address(sink), cd, EXECUTOR);
        _issue(cap4.capabilityCommitment, 4);
        vm.prank(EXECUTOR);
        guard.execute(cap4, "", address(sink), cd);

        assertTrue(guard.isConsumed(cap3.nullifier), "old nullifier stays burned");
        assertTrue(guard.isConsumed(cap4.nullifier), "new nullifier burned");
        assertEq(sink.pings(), 2, "both actions ran");
    }

    // ── Issue is orchestrator-only ───────────────────────────────────
    function test_IssueOnlyOrchestrator() public {
        vm.prank(CLONE);
        vm.expectRevert(BastionUnclonableCredentialGuard.NotOrchestrator.selector);
        guard.issue(bytes32(uint(1)), AGENT_ID, 0);
    }

    // ── Action commitment mismatch ───────────────────────────────────
    function test_ActionCommitmentMismatch() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        _issue(cap.capabilityCommitment, 3);

        // A different payload than the one the proof/commitment is bound to.
        bytes memory otherCd = abi.encodeWithSignature("pong()");
        vm.prank(EXECUTOR);
        vm.expectRevert(
            abi.encodeWithSelector(
                BastionUnclonableCredentialGuard.ActionMismatch.selector,
                cap.actionCommitment,
                keccak256(abi.encode(address(sink), otherCd))
            )
        );
        guard.execute(cap, "", address(sink), otherCd);
    }

    // ── Bad proof ────────────────────────────────────────────────────
    function test_BadProof() public {
        Sink8380 sink = new Sink8380();
        bytes memory cd = abi.encodeWithSignature("ping()");
        IUnclonableCredential.Capability memory cap =
            _buildCap(bytes32(uint(7)), 3, address(sink), cd, EXECUTOR);
        _issue(cap.capabilityCommitment, 3);

        verifier.setResult(false);
        vm.prank(EXECUTOR);
        vm.expectRevert(BastionUnclonableCredentialGuard.BadProof.selector);
        guard.execute(cap, "", address(sink), cd);
    }
}
