# Bastion Audit Anchor Program Spec

Program ID (devnet): `A29V5MUVs73y7XBHHxPpPcAW7h4gGHupbDdwYSwA2n9D`
Anchor version: 0.30.1
Solana SDK: 1.18

> This spec is kept in sync with `src/lib.rs`. A fresh mainnet program ID is
> generated at deploy time — see `docs/MAINNET_READINESS.md`.

## PDA seeds (as implemented)

| PDA | Seeds |
|-----|-------|
| `AuditState` | `["bastion_audit"]` |
| `AuditEntry` | `["bastion_audit", total_audits.to_le_bytes()]` |
| `Agent` | `["bastion_agent", authority.key().as_ref()]` |
| `Policy` | `["bastion_policy"]` |

Seed constants: `AUDIT_SEED`, `AGENT_SEED`, `POLICY_SEED` in `src/lib.rs`.

## Instructions

### 1. initialize(admin: Pubkey)

Creates the master `AuditState` PDA and sets `authority = admin`.

**Accounts:** `audit_state` (init, PDA `["bastion_audit"]`), `authority` (signer, payer), `system_program`.

**Constraints:** `admin != Pubkey::default()`; `init` prevents re-initialization.
**Security:** On mainnet `admin` MUST be the governance (Squads) multisig vault, and
`initialize` is invoked once by the deploy script immediately after `anchor deploy`.

### 2. log_audit(decision: u8, simulation_result: [u8;32], reasoning: String, program_id: Option<[u8;32]>)

Records an immutable `AuditEntry` and bumps counters (with checked arithmetic).

**Accounts:** `audit_entry` (init, PDA `["bastion_audit", total_audits.to_le_bytes()]`),
`audit_state` (mut), `signer`, `system_program`.
**Constraints:** `signer == audit_state.authority` (`Unauthorized`); `!audit_state.paused`
(`IsPaused`). `decision == 0` increments `allowed_count`, else `blocked_count`;
`total_audits` always increments. All increments are `checked_add` (`MathOverflow`).

### 3. register_agent(name: String, capability_bitmask: u64)

Registers an agent PDA keyed by the signer.

**Accounts:** `agent` (init, PDA `["bastion_agent", signer]`), `signer` (payer), `system_program`.
**Events:** `AgentRegistered { agent, authority, name }`. `reputation_score` starts at 0.

### 4. update_agent_reputation(delta: i64)

Adjusts the agent reputation score.

**Accounts:** `agent` (mut, PDA `["bastion_agent", agent.authority]`), `signer`.
**Constraints:** `signer == agent.authority` (`Unauthorized`). The new score must be in
`[0, MAX_REPUTATION]` where `MAX_REPUTATION = 100`; out-of-range updates are **rejected**
with `InvalidReputation` (the score is not silently clamped).
**Events:** `ReputationUpdated { agent, new_score }`.

### 5. set_policy(allowed_programs: Vec<[u8;32]>, max_sol_per_tx: u64, rate_limit_per_minute: u32)

Creates or overwrites the global policy PDA.

**Accounts:** `policy` (init_if_needed, PDA `["bastion_policy"]`), `signer` (payer), `system_program`.
**Constraints:** `policy.authority == Pubkey::default()` (uninitialized) OR
`signer == policy.authority` (`Unauthorized`). Max `allowed_programs` length is 20.

### 6. emergency_pause / 7. emergency_resume

Toggle `audit_state.paused`.

**Accounts:** `audit_state` (mut), `signer`.
**Constraints:** `signer == audit_state.authority` (`Unauthorized`). Pause requires not
already paused (`AlreadyPaused`); resume requires currently paused (`NotPaused`).
**Events:** `ProtocolPaused { authority }`, `ProtocolResumed { authority }`.

## Accounts (as implemented)

```rust
pub struct AuditState {
    pub authority: Pubkey,   // 32
    pub bump: u8,            // 1
    pub total_audits: u64,   // 8
    pub allowed_count: u64,  // 8
    pub blocked_count: u64,  // 8
    pub paused: bool,        // 1
    pub paused_at: i64,      // 8
    pub resumed_at: i64,     // 8
}

pub struct AuditEntry {
    pub authority: Pubkey,             // 32
    pub timestamp: i64,                // 8
    pub decision: u8,                  // 1  (0=Pass, 1=Block, 2=PendingHITL)
    pub simulation_result: [u8; 32],   // 32
    pub reasoning: String,             // 4 + len (<= 256)
    pub program_id: Option<[u8; 32]>,  // 1 + 32
    pub bump: u8,                      // 1
}

pub struct Agent {
    pub authority: Pubkey,        // 32
    pub name: String,             // 4 + len (<= 64)
    pub capability_bitmask: u64,  // 8
    pub reputation_score: u64,    // 8  (bounded [0, 100])
    pub delegation_depth: u8,     // 1
    pub registered_at: i64,       // 8
    pub bump: u8,                 // 1
}

pub struct Policy {
    pub authority: Pubkey,               // 32
    pub allowed_programs: Vec<[u8; 32]>, // 4 + len*32 (len <= 20)
    pub max_sol_per_tx: u64,             // 8
    pub rate_limit_per_minute: u32,      // 4
    pub bump: u8,                        // 1
}
```

## Errors

| Error | Code | Meaning |
|-------|------|---------|
| InvalidReputation | 6000 | New reputation would fall outside [0, 100] |
| NotPaused | 6001 | Tried to resume while not paused |
| IsPaused | 6002 | Tried to log_audit while paused |
| AlreadyPaused | 6003 | Tried to pause while already paused |
| Unauthorized | 6004 | Signer is not the required authority |
| MathOverflow | 6005 | Checked counter arithmetic overflowed |

## Events

| Event | Fields |
|-------|--------|
| AgentRegistered | agent: Pubkey, authority: Pubkey, name: String |
| ReputationUpdated | agent: Pubkey, new_score: u64 |
| ProtocolPaused | authority: Pubkey |
| ProtocolResumed | authority: Pubkey |
