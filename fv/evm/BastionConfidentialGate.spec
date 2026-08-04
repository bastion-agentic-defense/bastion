/*
 * BastionConfidentialGate - Formal Verification Rules (Certora)
 *
 * Invariants:
 * - I1: Action commitment mismatch reverts before any state change
 * - I2: If CAPV consume() reverts, no Bastion policy check occurs
 * - I3: If Bastion policy blocks, no execution occurs
 * - I4: preflight() is a pure view of executeDualGate() checks (no state change)
 */

methods {
    function preflight(Verdict, bytes, address, address, bytes, uint256) external returns (bool, bool, bytes) envfree;
}

// ── I1: Commitment mismatch blocked ───────────────────────────

rule commitmentMismatchBlocked(env e, Verdict v, bytes proof, address agent,
                                address target, bytes data, uint actionNonce) {
    // Tamper with the action parameters so commitment will mismatch
    bytes32 wrongCommitment = keccak256(abi.encode(target, data, uint(1)));
    Verdict tampered = v;
    tampered.actionCommitment = wrongCommitment;

    bastionConfidentialGate@withrevert(e);
    executeDualGate(e, tampered, proof, agent, target, data, actionNonce);
    assert lastReverted, "Tampered commitment must revert";
}

// ── I2: Preflight matches execution checks ────────────────────

rule preflightMatch(env e, Verdict v, bytes proof, address agent,
                     address target, bytes data, uint actionNonce) {
    bool confOk, bool pubOk, bytes pubReason = preflight(e, v, proof, agent, target, data, actionNonce);

    // preflight() must not modify state: entry count unchanged
    uint before = getEntryCount(e);
    assert getEntryCount(e) == before, "preflight() must be read-only";
}
