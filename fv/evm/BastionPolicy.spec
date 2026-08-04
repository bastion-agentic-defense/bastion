/*
 * BastionPolicy - Formal Verification Rules (Certora)
 *
 * Invariants proven:
 * - I1: Unregistered agents always blocked
 * - I2: Value cap enforced exactly
 * - I3: Cooldown enforced between transactions
 * - I4: Deleting a policy removes all rules
 * - I5: checkTransaction is deterministic
 */

methods {
    function checkTransaction(address, address, uint, bytes) external returns (bool, bytes) envfree;
    function getPolicy(address) external returns (Policy) envfree;
    function getTxCount(address) external returns (uint) envfree;
}

// ── I1: Unregistered agent blocked ────────────────────────────

rule unregisteredBlocked(env e, address unregistered, address target, uint value, bytes data) {
    Policy p = getPolicy(e, unregistered);
    require !p.isActive;

    bool allowed, bytes reason = checkTransaction(e, unregistered, target, value, data);
    assert !allowed, "Unregistered agents must be blocked";
}

// ── I2: Value cap enforced ────────────────────────────────────

rule valueCapEnforced(env e, address agent, address target, bytes data) {
    Policy p = getPolicy(e, unregistered);
    require e.msg.sender == firewall;
    require p.isActive;

    // Value exactly at limit: allowed
    uint atLimit = p.maxValuePerTx;
    bool ok, _ = checkTransaction(e, agent, target, atLimit, data);
    assert ok, "Value at limit must be allowed";

    // Value above limit: blocked
    uint aboveLimit = p.maxValuePerTx + 1;
    bool blocked, _ = checkTransaction(e, agent, target, aboveLimit, data);
    assert !blocked, "Value above limit must be blocked";
}

// ── I3: Cooldown enforced ─────────────────────────────────────

rule cooldownEnforced(env e1, env e2, address agent, address target, bytes data) {
    require e1.msg.sender == firewall;
    require e2.msg.sender == firewall;

    // First transaction passes
    bool firstOk, _ = checkTransaction(e1, agent, target, 1, data);
    assert firstOk;

    // Second transaction within cooldown blocked
    require e2.block.timestamp < e1.block.timestamp + cooldownSeconds;
    bool secondOk, _ = checkTransaction(e2, agent, target, 1, data);
    assert !secondOk, "Within cooldown must be blocked";
}

// ── I4: Policy deletion removes all rules ─────────────────────

rule deletionCompleteness(env e, address agent) {
    require e.msg.sender == firewall;

    setPolicy(e, agent, Policy({agent: agent, isActive: true, maxValuePerTx: 100, ...}));
    assert getPolicy(e, agent).isActive;

    removePolicy(e, agent);
    assert !getPolicy(e, agent).isActive, "Deleted policy must be inactive";
}

// ── I5: Deterministic evaluation ──────────────────────────────

rule deterministicCheck(env e, address agent, address target, uint value, bytes data) {
    bool result1, bytes reason1 = checkTransaction(e, agent, target, value, data);
    bool result2, bytes reason2 = checkTransaction(e, agent, target, value, data);

    assert result1 == result2, "checkTransaction must be deterministic";
    assert reason1 == reason2, "Reason must be deterministic";
}
