// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import { Test } from "forge-std/Test.sol";
import { IERC165 } from "@openzeppelin/contracts/utils/introspection/IERC165.sol";

import { BastionConfidentialVerdict } from "../src/erc8354/BastionConfidentialVerdict.sol";
import { BastionPolicyDomainRegistry } from "../src/erc8354/BastionPolicyDomainRegistry.sol";
import { BastionGuardedExecutor } from "../src/erc8354/BastionGuardedExecutor.sol";
import {
    IConfidentialPolicyVerdict,
    Verdict,
    PolicyKind
} from "../src/erc8354/IConfidentialPolicyVerdict.sol";
import { IPolicyDomainRegistry } from "../src/erc8354/IPolicyDomainRegistry.sol";
import { IPolicyGuarded } from "../src/erc8354/IPolicyGuarded.sol";
import { PolicyAction, PolicyActionLib } from "../src/erc8354/PolicyAction.sol";
import {
    PolicyAttestation,
    VerdictAttestation,
    IValidationRegistry
} from "../src/erc8354/IPolicyAttestation.sol";
import { MockVerifier } from "../src/erc8354/mocks/MockVerifier.sol";
import { MockValidationRegistry } from "../src/erc8354/mocks/MockValidationRegistry.sol";

/// @notice Minimal call target for BastionGuardedExecutor tests.
contract Sink {
    uint public pings;

    function ping() external {
        pings++;
    }
}

/// @notice The Test Cases from ERC-8354, as an executable Foundry suite against the
/// Bastion-named conforming contracts.
contract ERC8354Test is Test {
    BastionPolicyDomainRegistry registry;
    BastionConfidentialVerdict guard;
    MockVerifier verifier;

    bytes32 constant DOMAIN = keccak256("acme-compliance");
    bytes32 constant ROOT = keccak256("root-v1");
    bytes32 constant PROGRAM = keccak256("interpreter-vkey");
    address constant EXECUTOR = address(0xE0);

    function setUp() public {
        vm.warp(1_700_000_000);
        registry = new BastionPolicyDomainRegistry();
        verifier = new MockVerifier();
        guard = new BastionConfidentialVerdict(registry);
        registry.registerDomain(DOMAIN, address(0xA11CE), address(verifier), PROGRAM, 1 hours);
        registry.updateRoot(DOMAIN, ROOT);
    }

    function _verdict() internal view returns (Verdict memory v) {
        v = Verdict({
            agentId: 1,
            domainId: DOMAIN,
            policyRoot: ROOT,
            actionCommitment: keccak256("action"),
            executor: EXECUTOR,
            expiry: uint64(block.timestamp + 1 hours),
            nullifier: keccak256("nf-1"),
            decision: 1,
            policyKind: PolicyKind.ALLOWED
        });
    }

    // ── 1. Happy path ────────────────────────────────────────────────
    function test_HappyPath() public {
        Verdict memory v = _verdict();
        vm.prank(EXECUTOR);
        guard.consume(v, "proof");
        assertTrue(guard.isConsumed(DOMAIN, v.nullifier));
    }

    // ── 2. Replay ────────────────────────────────────────────────────
    function test_Replay() public {
        Verdict memory v = _verdict();
        vm.startPrank(EXECUTOR);
        guard.consume(v, "proof");
        vm.expectRevert(
            abi.encodeWithSelector(IConfidentialPolicyVerdict.VerdictReplayed.selector, v.nullifier)
        );
        guard.consume(v, "proof");
        vm.stopPrank();
    }

    // ── 3. Expiry ────────────────────────────────────────────────────
    function test_Expired() public {
        Verdict memory v = _verdict();
        vm.warp(v.expiry); // block.timestamp >= expiry
        vm.prank(EXECUTOR);
        vm.expectRevert(
            abi.encodeWithSelector(IConfidentialPolicyVerdict.VerdictExpired.selector, v.expiry)
        );
        guard.consume(v, "proof");
    }

    // ── 4. Executor authorization ────────────────────────────────────
    function test_ExecutorMismatch() public {
        Verdict memory v = _verdict();
        vm.prank(address(0xBAD));
        vm.expectRevert(
            abi.encodeWithSelector(
                IConfidentialPolicyVerdict.ExecutorMismatch.selector, EXECUTOR, address(0xBAD)
            )
        );
        guard.consume(v, "proof");
    }

    function test_RelayedConsumeBadSignature() public {
        uint pk = 0xA11CE;
        Verdict memory v = _verdict();
        v.executor = vm.addr(pk);
        (uint8 sv, bytes32 sr, bytes32 ss) = vm.sign(uint(0xB0B), guard.verdictDigest(v)); // wrong key
        bytes memory sig = abi.encodePacked(sr, ss, sv);
        vm.prank(address(0xBEEF));
        vm.expectRevert(IConfidentialPolicyVerdict.ExecutorAuthInvalid.selector);
        guard.consume(v, "proof", sig);
    }

    // ── 5. DENY not consumable ───────────────────────────────────────
    function test_DenyNotConsumable() public {
        Verdict memory v = _verdict();
        v.decision = 0;
        v.policyKind = PolicyKind.DENIED; // a well-formed refusal, not a malformed envelope
        vm.prank(EXECUTOR);
        vm.expectRevert(IConfidentialPolicyVerdict.VerdictDenied.selector);
        guard.consume(v, "proof");
    }

    // A verdict whose decision and kind disagree is refused before anything else is read.
    function test_DecisionKindMismatchRefused() public {
        Verdict memory v = _verdict(); // decision 1, kind ALLOWED
        v.decision = 0; // claims a refusal while still carrying the ALLOWED kind
        vm.prank(EXECUTOR);
        vm.expectRevert(
            abi.encodeWithSelector(
                IConfidentialPolicyVerdict.VerdictKindMismatch.selector,
                uint8(0),
                PolicyKind.ALLOWED
            )
        );
        guard.consume(v, "proof");
    }

    function test_AllowCarryingRefusalKindRefused() public {
        Verdict memory v = _verdict();
        v.policyKind = PolicyKind.NOT_PERMITTED;
        vm.prank(EXECUTOR);
        vm.expectRevert(
            abi.encodeWithSelector(
                IConfidentialPolicyVerdict.VerdictKindMismatch.selector,
                uint8(1),
                PolicyKind.NOT_PERMITTED
            )
        );
        guard.consume(v, "proof");
    }

    // ── 6. Action binding (guarded executor) ─────────────────────────
    function test_GuardedExecutorCommitmentMismatch() public {
        BastionGuardedExecutor gx =
            new BastionGuardedExecutor(guard, DOMAIN, IValidationRegistry(address(0)));
        Sink sink = new Sink();
        bytes memory cd = abi.encodeWithSignature("ping()");
        Verdict memory v = _verdict();
        v.executor = address(gx);
        v.actionCommitment = bytes32(uint(1)); // wrong
        bytes32 expected = gx.actionCommitmentOf(v.agentId, address(sink), 0, cd);
        vm.expectRevert(
            abi.encodeWithSelector(
                BastionGuardedExecutor.ActionCommitmentMismatch.selector,
                expected,
                v.actionCommitment
            )
        );
        gx.execute(v, "proof", "", address(sink), 0, cd);
    }

    // ── 7. Cross-chain / cross-domain replay separation ──────────────
    // Structural: the commitment preimage leads with chainId and domainId, so the same action
    // on a different chain or under a different domain produces a different commitment.
    function test_CrossChainCrossDomainReplaySeparation() public pure {
        PolicyAction memory base = PolicyAction({
            chainId: 1,
            domainId: bytes32(uint(0xD1)),
            agentId: 7,
            target: address(uint160(0x1234)),
            value: 0,
            callDataHash: bytes32(0),
            actionNonce: 3
        });
        bytes32 c11 = PolicyActionLib.commit(base);

        PolicyAction memory otherChain = base;
        otherChain.chainId = 2;
        assertTrue(PolicyActionLib.commit(otherChain) != c11, "chainId must separate");

        PolicyAction memory otherDomain = base;
        otherDomain.domainId = bytes32(uint(0xD2));
        assertTrue(PolicyActionLib.commit(otherDomain) != c11, "domainId must separate");
    }

    // ── 8. Stale-root grace, then rejection past maxRootAge ──────────
    function test_StaleRootGraceThenReject() public {
        Verdict memory v = _verdict(); // against ROOT
        registry.updateRoot(DOMAIN, keccak256("root-v2")); // ROOT becomes previous
        vm.prank(EXECUTOR);
        guard.consume(v, "proof"); // within grace → ok

        vm.warp(block.timestamp + 2 hours); // past maxRootAge
        Verdict memory v2 = _verdict(); // built after warp → fresh expiry, still points at old ROOT
        v2.nullifier = keccak256("nf-2");
        vm.prank(EXECUTOR);
        vm.expectRevert(
            abi.encodeWithSelector(IConfidentialPolicyVerdict.PolicyRootRejected.selector, ROOT)
        );
        guard.consume(v2, "proof");
    }

    // ── 9. Revocation is immediate, no grace ─────────────────────────
    function test_RevocationImmediate() public {
        registry.revokeDomain(DOMAIN);
        Verdict memory v = _verdict();
        vm.prank(EXECUTOR);
        vm.expectRevert(
            abi.encodeWithSelector(IConfidentialPolicyVerdict.DomainInactive.selector, DOMAIN)
        );
        guard.consume(v, "proof");
    }

    // ── 10. Malformed proof → verify returns false (no revert) ───────
    function test_VerifyMalformedReturnsFalse() public {
        verifier.setRevert(true);
        Verdict memory v = _verdict();
        assertFalse(guard.verify(v, "garbage"));
    }

    // ── Invalid proof → consume reverts InvalidProof ─────────────────
    function test_InvalidProofReverts() public {
        verifier.setResult(false);
        Verdict memory v = _verdict();
        vm.prank(EXECUTOR);
        vm.expectRevert(IConfidentialPolicyVerdict.InvalidProof.selector);
        guard.consume(v, "proof");
    }

    function test_VerifyHappy() public view {
        assertTrue(guard.verify(_verdict(), "proof"));
    }

    // ── Signature-relay path ─────────────────────────────────────────
    function test_RelayedConsumeWithSignature() public {
        uint pk = 0xA11CE;
        address exec = vm.addr(pk);
        Verdict memory v = _verdict();
        v.executor = exec;

        (uint8 sv, bytes32 sr, bytes32 ss) = vm.sign(pk, guard.verdictDigest(v));
        bytes memory sig = abi.encodePacked(sr, ss, sv);

        vm.prank(address(0xBEEF)); // relayer, not the executor
        guard.consume(v, "proof", sig);
        assertTrue(guard.isConsumed(DOMAIN, v.nullifier));
    }

    function test_RelayNoAuthReverts() public {
        Verdict memory v = _verdict(); // executor == EXECUTOR
        vm.prank(address(0xBEEF));
        vm.expectRevert(
            abi.encodeWithSelector(
                IConfidentialPolicyVerdict.ExecutorMismatch.selector, EXECUTOR, address(0xBEEF)
            )
        );
        guard.consume(v, "proof", "");
    }

    // ── Canonical action-commitment binding through a guarded contract ──
    function test_GuardedExecutorCanonicalCommitment() public {
        BastionGuardedExecutor gx =
            new BastionGuardedExecutor(guard, DOMAIN, IValidationRegistry(address(0)));
        Sink sink = new Sink();
        bytes memory cd = abi.encodeWithSignature("ping()");

        Verdict memory v = _verdict();
        v.executor = address(gx); // consumes on its own behalf
        v.actionCommitment = gx.actionCommitmentOf(v.agentId, address(sink), 0, cd);

        gx.execute(v, "proof", "", address(sink), 0, cd);
        assertEq(sink.pings(), 1);
        assertTrue(guard.isConsumed(DOMAIN, v.nullifier));
    }

    function test_GuardedExecutorRelayedByEOA() public {
        BastionGuardedExecutor gx =
            new BastionGuardedExecutor(guard, DOMAIN, IValidationRegistry(address(0)));
        Sink sink = new Sink();
        bytes memory cd = abi.encodeWithSignature("ping()");

        uint pk = 0xA11CE;
        Verdict memory v = _verdict();
        v.executor = vm.addr(pk); // the end user
        v.actionCommitment = gx.actionCommitmentOf(v.agentId, address(sink), 0, cd);

        (uint8 sv, bytes32 sr, bytes32 ss) = vm.sign(pk, guard.verdictDigest(v));
        bytes memory sig = abi.encodePacked(sr, ss, sv);

        gx.execute(v, "proof", sig, address(sink), 0, cd);
        assertEq(sink.pings(), 1);
        assertTrue(guard.isConsumed(DOMAIN, v.nullifier));
    }

    // ── IPolicyGuarded surface ───────────────────────────────────────
    function test_GuardedExecutorPolicyDomain() public {
        BastionGuardedExecutor gx =
            new BastionGuardedExecutor(guard, DOMAIN, IValidationRegistry(address(0)));
        assertEq(IPolicyGuarded(address(gx)).policyDomain(), DOMAIN);
    }

    // ── ERC-165 ──────────────────────────────────────────────────────
    function test_SupportsInterface() public view {
        assertTrue(guard.supportsInterface(type(IConfidentialPolicyVerdict).interfaceId), "own");
        assertTrue(guard.supportsInterface(type(IERC165).interfaceId), "erc165");
        assertFalse(guard.supportsInterface(0xffffffff), "0xffffffff must be false");
        assertFalse(guard.supportsInterface(0xdeadbeef), "random");
    }

    function test_InterfaceIdMatchesSpec() public pure {
        assertTrue(
            type(IConfidentialPolicyVerdict).interfaceId == bytes4(0xd6da8150), "spec interfaceId"
        );
    }

    // ── ERC-8004 Validation Registry handoff ─────────────────────────
    function test_VerdictAttestationHandoff() public {
        Verdict memory v = _verdict();
        vm.prank(EXECUTOR);
        guard.consume(v, "proof");

        VerdictAttestation memory att = PolicyAttestation.attestationFor(v);
        MockValidationRegistry vr = new MockValidationRegistry();
        vr.recordVerdict(att);

        VerdictAttestation memory got = vr.get(v.agentId, v.nullifier);
        assertEq(got.artifactHash, v.actionCommitment, "artifactHash must be the action commitment");
        assertEq(got.mechanism, keccak256("zk-secret-policy"), "mechanism must be tagged");
        assertEq(got.policyRoot, v.policyRoot);
        assertEq(uint(got.decision), 1);
        assertEq(got.agentId, v.agentId);
        assertEq(got.expiry, v.expiry);
        assertEq(got.domainId, v.domainId);
        assertEq(got.nullifier, v.nullifier);
    }

    function test_GuardedExecutorWritesAttestation() public {
        MockValidationRegistry vr = new MockValidationRegistry();
        BastionGuardedExecutor gx = new BastionGuardedExecutor(guard, DOMAIN, vr);
        Sink sink = new Sink();
        bytes memory cd = abi.encodeWithSignature("ping()");

        Verdict memory v = _verdict();
        v.executor = address(gx);
        v.actionCommitment = gx.actionCommitmentOf(v.agentId, address(sink), 0, cd);

        gx.execute(v, "proof", "", address(sink), 0, cd);
        assertEq(sink.pings(), 1);
        assertTrue(guard.isConsumed(DOMAIN, v.nullifier));

        VerdictAttestation memory got = vr.get(v.agentId, v.nullifier);
        assertEq(got.artifactHash, v.actionCommitment, "handoff artifactHash");
        assertEq(got.mechanism, keccak256("zk-secret-policy"), "handoff mechanism");
    }
}
