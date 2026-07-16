# Bastion — Production Roadmap

> Target: Production-ready by **2026-06-18** (one week from 2026-06-11)

---

## Strategic Decisions (Locked)

| Decision | Rationale |
|---|---|
| **Remove staking mechanism** | SOL-lamport PDAs are Solana-only. Staking duplicates `reputation_score` which already exists on `Agent`. True multichain means chain-agnostic primitives only. |
| **Migrate Solana program to Quasar** | `#![no_std]` + zero-copy reduces binary size ~50–70%, lowering mainnet deployment cost from ~2 SOL → ~0.8–1 SOL. Smaller binary = lower CU per instruction. |
| **Reputation as the universal primitive** | `reputation_score: u64` on `Agent` is representable on Solana (Anchor/Quasar), EVM (uint256 in registry), and Base (via Arcium). All limit enforcement runs through `crates/core` policy engine, not on-chain staking. |

---

## Week of 2026-06-11 → 2026-06-18

### Day 1–2: Remove Staking (Solana Program Cleanup) — DONE

**Goal:** Trim the on-chain program to the minimum viable audit surface.

**What to delete from `crates/solana/programs/bastion-audit/src/lib.rs`:**
- Instructions: `stake_lamports`, `request_unstake`, `claim_unstake`, `slash_stake`
- Context structs: `StakeLamports`, `RequestUnstake`, `ClaimUnstake`, `SlashStake`
- Account: `AgentStake`
- Events: `StakeChanged`, `UnstakeRequested`, `StakeSlashed`
- Errors: `InsufficientStake`, `StakeTooRecent`, `StakeCooldownNotMet`, `NoUnstakeRequested`, `MaxDelegationDepth`

**What stays:** `initialize`, `log_audit`, `register_agent`, `update_agent_reputation`, `set_policy`, `emergency_pause`, `emergency_resume` — the pure audit/identity/policy surface.

**Also remove** the `stake_lamports` instruction reference from `README.md` on-chain table, `packages/sdk/src/index.ts`, and `packages/sdk/src/types.ts`.

**Acceptance:** `cargo check -p bastion-audit` passes, IDL regenerates cleanly, SDK builds.

---

### Day 2–3: Quasar Migration (Solana Program) — BLOCKED on crate maturity

**Status:** Attempted migration to `quasar-lang` v0.0.0 on 2026-06-12. The crate compiles (`#[derive(Accounts)]`, `#[program]`, `Ctx<T>`, `#![no_std]` all resolve) but the published version is missing features documented on quasar-lang.com:

| Feature (docs) | v0.0.0 status |
|---|---|
| `quasar_lang::String<N>` | Not exported |
| `quasar_lang::Vec<T, N>` | Not exported |
| `Clock::get()` via `Sysvar` | Trait not in scope |
| Seed field shorthand (`agent` vs `agent.key().as_ref()`) | Partial |
| `Option<[u8; 32]>: WriteBytes` | Not implemented |
| PodU64/PodBool ergonomic `.into()` | Manual `.into()` needed everywhere |

**When ready:** The migration guide at `quasar-lang.com/docs/getting-started/migrating-from-anchor` is production-quality and the API is well-designed. The program was fully ported (see git history) and will compile once the crate reaches parity with the docs.

**Estimated savings:** Binary size ~50-70% smaller, deploy cost ~2 SOL → ~0.8-1 SOL.

**Steps:**

1. Add `quasar-lang` to `crates/solana/programs/bastion-audit/Cargo.toml`, remove `anchor-lang`.
2. Replace `use anchor_lang::prelude::*` → `use quasar_lang::prelude::*`.
3. Replace `#[program]` module + `pub fn` handlers with `#[instruction(discriminator = N)]` pattern.
4. Replace `#[derive(Accounts)]` structs with Quasar's `#[derive(Accounts)]` (API is similar but constraints differ — see [Quasar accounts docs](https://quasar-lang.com/docs/core-concepts/accounts-and-validation)).
5. Replace `#[account]` data structs — use `&'a str` tail fields where possible (e.g. `AuditEntry.reasoning`, `Agent.name`) to avoid length-prefix overhead.
6. Replace `emit!` → `emit_cpi!` (Quasar event model).
7. Add `#![no_std]` at crate root.
8. Replace `std::mem::size_of` in `space` calculations with explicit byte counts (already done in most places).
9. Regenerate IDL with `quasar build` (outputs compatible JSON — verify against SDK types).
10. Update `crates/solana/Anchor.toml` or replace with `quasar.toml` per Quasar project config.

**Key differences from Anchor to watch:**
- Quasar uses `Ctx<T>` not `Context<T>`
- Discriminators are explicit (`#[instruction(discriminator = N)]`), not hash-derived
- `Result<(), ProgramError>` not `Result<()>` (Anchor's re-export)
- No `require!` macro — use standard `if !cond { return Err(...) }`
- Build for production without `--debug` flag (strips validation log overhead)

**Acceptance:** `quasar build` succeeds, binary is <200 KB, IDL JSON diff is minimal (field names/types unchanged), SDK still compiles against new IDL.

---

### Day 3–4: Wire Reputation as the Cross-Chain Limit Gate

**Goal:** Replace the staking-based limit override with reputation-based limits in `crates/core`.

The staking mechanism gated higher `max_sol_per_tx` by staked SOL amount. Replace this with a `ReputationWeighted` policy rule in the chain-agnostic engine:

**In `crates/core/src/policy/types.rs`**, add:
```rust
/// Scale transaction limits by agent reputation score.
/// Works on any chain — no SOL staking required.
ReputationWeighted {
    /// Base limit (applied when reputation_score = 0)
    base_limit_lamports: u64,
    /// Additional lamports per reputation point
    lamports_per_point: u64,
    /// Hard cap regardless of reputation
    max_limit_lamports: u64,
},
```

**In `crates/sidecar/src/lib.rs`**, populate `NormalizedTransaction.agent_reputation` from `AgentStore` before calling the evaluator (not from the stake PDA, just the sidecar's local agent store or on-chain `Agent.reputation_score`).

**Acceptance:** A simulated transaction from an agent with `reputation_score = 100` passes a higher limit than one with `reputation_score = 0`, with no on-chain staking involved.

---

### Day 4–5: SDK + IDL Sync

**Goal:** Ensure `packages/sdk` reflects the trimmed program surface.

- Remove `stakeAgent()`, `requestUnstake()`, `claimUnstake()` from `packages/sdk/src/index.ts`
- Remove `AgentStake` from `packages/sdk/src/types.ts`
- Update `packages/sdk/src/idl.json` with the new Quasar-generated IDL
- Bump SDK version to `0.6.0` in `packages/sdk/package.json`
- Update `packages/sdk/README.md` if it references staking

**Acceptance:** `pnpm --filter @zkos-labs/sdk build` passes with no type errors.

---

### Day 5–6: Dashboard Cleanup + Mainnet Config

**Goal:** Remove staking UI, add mainnet deploy config.

**Dashboard (`apps/web`):**
- Remove any staking-related UI from `Dashboard.tsx` or `Integrate.tsx`
- Update on-chain instruction table in `README.md` to reflect final instruction set
- Add `VITE_SOLANA_CLUSTER=mainnet-beta` to `.env.production`

**Deployment prep:**
```bash
# Verify binary size after Quasar build
quasar build
ls -lh target/deploy/bastion_audit.so

# Dry-run mainnet deploy cost estimate
solana program deploy --dry-run target/deploy/bastion_audit.so --url mainnet-beta
```

**Acceptance:** Dashboard builds (`pnpm --filter bastion-dashboard build`), no console errors referencing removed staking instructions.

---

### Day 6–7: Mainnet Deploy + Smoke Test

**Goal:** Program live on mainnet, all CI green.

1. Fund deployer wallet with ~2.5 SOL (buffer above estimated cost)
2. `anchor deploy --provider.cluster mainnet-beta` (or `quasar deploy` if migration is complete)
3. Update `declare_id!` in `lib.rs` with new mainnet program ID
4. Update `packages/sdk/src/idl.json` and `apps/web/src/idl.json` with mainnet program ID
5. Push to `main` → CI runs all 7 jobs → Netlify/Vercel auto-deploy
6. Smoke test: `POST /simulate` against fly.dev sidecar with a real devnet tx, verify audit entry lands on mainnet

---

## Post-Production Backlog (after 2026-06-18)

These are from `IMPROVEMENTS.md` — prioritized but not blocking production:

| # | Feature | Notes |
|---|---|---|
| 1 | Reputation feedback loop (auto-strike on block) | `~100 lines Rust`, closes on-chain accountability gap |
| 2 | Webhook on block events | `~80 lines Rust`, immediate operator value |
| 3 | HITL approval UI in dashboard | Completes `PendingHITL` flow that already exists in core |
| 4 | AI intent scoring (heuristic) | Differentiates from rule-only firewalls |
| 5 | EVM hooks wired in dashboard | `useBastionEVM.ts` stubs need `useReadContract`/`useWriteContract` |
| 6 | Behavioral baseline per agent | Requires data pipeline, strongest long-term moat |

---

## Future Epics — The Runtime Vision (🚧)

These are the large net-new subsystems behind the 🚧 markers in the root `README.md` and
[`docs/VISION.md`](VISION.md). They are **not** in any current milestone — they are captured here so
the vision has an honest home and the README markers reconcile against a real backlog. Each sits
**behind the mainnet/EVM external-audit hard gate** (`docs/MAINNET_READINESS.md` §7,
`docs/EVM_READINESS.md` §6); none ships to mainnet real-value traffic before that gate clears.

### Epic A — Durable Workflow Engine (🚧 absent)

**Status today:** No workflow / orchestration / state-machine code exists anywhere in the repo. This
is the **largest net-new subsystem** on the roadmap.

**Why:** `execute()` today is a single synchronous decision (policy → simulate → decide). A durable
engine is what turns Bastion from a firewall into a *runtime* — multi-step agent actions that
survive process restarts, retry deterministically, and resume where they left off.

**Requirements:**
- **Persistent state machine** — each workflow run is durably recorded (step, status, inputs,
  outputs) so a crash mid-run is recoverable. Reuse the existing `sled` store the sidecar audit log
  already depends on (`crates/sidecar/src/audit.rs`) rather than introducing a new datastore.
- **Idempotency / dedupe** — every step carries an idempotency key; re-delivery of the same step is a
  no-op, so at-least-once execution is safe.
- **Retry & resume** — failed steps retry with backoff; a resumed run replays completed steps from
  the log instead of re-executing side effects.
- **Failure survival** — a killed sidecar reconstructs in-flight runs from the persisted log on boot.
- **HITL integration** — a `PendingHITL` decision suspends the workflow durably until an
  `/override` resolves it, rather than blocking a request handler.

**Shape:** likely a new `crates/workflow` crate (chain-agnostic, like `crates/core`) plus sidecar
routes to start / query / resume runs, and an SDK surface (`bastion.workflow(...)`) composing
`execute()` per step.

### Epic B — Real Arcium Confidential Compute (🚧 stubbed)

**Status today:** `crates/arcium/` ships only `NoopArciumClient`, which always returns `Pass`. The
Arcis circuits (`crates/arcium/src/circuits/`) and the Solana callback (`crates/arcium/src/solana/`)
are empty placeholders. Per MAINNET_READINESS §6, the runtime **must not advertise "confidential"
while only the noop is active** — this is now enforced: `/health` reports
`confidential_compute: false` and `bastion.execute({ privacy: "confidential" })` refuses rather than
evaluating in the clear.

**Why:** confidential policy evaluation (evaluating a transaction against private limits/rules
without revealing them) is the headline privacy guarantee. A no-op cannot back that claim.

**Requirements:**
- **Arcis circuits** — implement the real MPC circuits in `crates/arcium/src/circuits/` for the
  confidential policy-evaluation path (private thresholds, private allowlists).
- **Solana callback** — wire `crates/arcium/src/solana/` to submit/await the MXE computation and
  land the attested result on-chain.
- **Live MXE client** — replace `NoopArciumClient` with a client that talks to a real Arcium MXE
  cluster, implementing `is_confidential() -> true` only when genuinely backed by MPC.
- **Feature-gated rollout** — keep the noop as the default build; the live client is opt-in and, once
  active, flips `confidential_compute` to `true` so `execute()` will proceed.

**Gate:** confidential execution touching real value is behind the external-audit hard gate.

### Epic C — True Settlement Router / Cross-Chain Execution Planner (🟡 → 🚧)

**Status today:** `execute()` ships a **minimal** router — it selects a chain and runs that chain's
simulator (`Chain` enum + per-chain simulators in `crates/sidecar`). There is no execution
*planner*: no cross-chain sequencing, no route optimization, no atomic multi-leg settlement.

**Why:** the vision's "declare the outcome, not the infrastructure" promise needs a planner that can
decompose an intent into an ordered, chain-spanning execution plan — the minimal router only answers
"is this one tx on this one chain allowed?".

**Requirements:**
- **Plan decomposition** — turn a single high-level intent into an ordered set of per-chain legs.
- **Route selection** — choose chains/venues by cost, latency, and reputation-weighted policy.
- **Atomicity / rollback semantics** — define what happens when leg N fails after legs 1..N-1
  settled (compensating actions, or all-or-nothing where the chains support it). This depends on
  **Epic A** for durability.
- **Promotion path** — grow the Phase-4 minimal router in `packages/sdk/src/execute.ts` into a real
  planner without changing the `execute()` call signature (declarative in stays the same).

### Epic D — Pact Network Payment Guarantees (🚧 planned)

**Status today:** No Pact integration exists. This is a net-new integration epic.

**Why:** Pact Network provides on-chain chargebacks for x402 agent payments — when an agent pays
an API and it fails, Pact refunds principal + premium from a coverage pool. Bastion's Web2 firewall
already intercepts outbound API calls; wrapping them with Pact insurance closes the economic trust
loop (policy decides whether to call, Pact guarantees the outcome).

**What Pact is:**
- Solana mainnet Pinocchio program (`5bCJcdWdKLJ7arrMVMFh3z99rQDxV785fnD9XGcr3xwc`)
- USDC-denominated coverage pools with per-endpoint premium rates
- Deterministic classifier: `success` → Pact earns premium, `server_error` → agent refunded
- Batched on-chain settlement via `settle_batch` instruction (up to 50 calls/tx)
- Currently in private beta; upgrade authority and settler are protocol-team-held (v1 centralization is acknowledged)

**Requirements:**
- **CLI integration** — auto-wrap `pay curl` → `pact pay curl` in Bastion's Web2 proxy for covered endpoints
- **SDK surface** — `bastion.execute({ ..., coverage: { provider: "pact", tier: "standard" } })`
- **Policy integration** — policy rules can require Pact coverage for specific endpoints
- **Audit trail** — ingest Pact `settle_batch` events + `CallRecord` PDAs into Bastion's audit log
- **Market integration** — route covered calls through `market.pactnetwork.io` for curated endpoints

**Gate:** pact-network mainnet is in private beta. Integration follows Pact's public beta milestone.

---

## Invariants (don't break these)

- `crates/core` must stay chain-agnostic — no Solana or EVM imports
- On-chain program must only contain audit/identity/policy primitives — no financial instruments
- SDK major version bump required if IDL instruction set changes
- `main` branch must always pass all 7 CI jobs before merge
