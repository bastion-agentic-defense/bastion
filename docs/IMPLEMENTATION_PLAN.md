# Bastion - Complete Implementation Plan

> Covers all 7 epics, their phases, dependencies, effort estimates, and acceptance criteria.
> Target: close every gap identified in [`TRUSTFLOW_COMPARISON.md`](TRUSTFLOW_COMPARISON.md) and [`ROADMAP.md`](ROADMAP.md).
>
> **Total estimate:** ~5,780 LOC across Rust + TypeScript, 8-12 weeks.

---

## Gap Inventory

| # | Epic | TrustFlow Equivalent | Severity | Current Status | Blocks |
|---|------|---------------------|----------|---------------|--------|
| A | Durable Workflow Engine | Temporal | **Critical** | 🚧 Design phase | C, E, G |
| B | Real Arcium MPC | Vault (kinda - privacy layer) | **Medium** | 🚧 NoopArciumClient | Confidential claims |
| C | Settlement Router | Temporal routing | **Medium** | 🟡 Minimal router | `execute()` vision |
| D | Pact Network | - | **Low** | 🚧 Planned | - |
| E | Secrets Management | Vault | **High** | ❌ Absent | Production agents |
| F | General-Purpose Policy | OPA/Rego | **High** | ✅ Native only | Enterprise adoption |
| G | EigenLayer AVS | EigenLayer | **Low** | ❌ Absent | Operator trust |
| H | Post-Production Backlog | - | **Low** | Open | - |
| I | Starknet ZK-Verified Execution | - | **Medium** | 🚧 Planned | Provable execution + native AA |

---

## Phase Order & Dependencies

```
 Phase 0: Foundation ───────────────────────────────┐
  Post-prod backlog (6 items)                       │
  ReputationWeighted policy rule                    │
  Durable policy state (Sled-backed)                │
                                                    │
 Phase 1: Durable Workflow Engine (Epic A) ─────────┤
  `crates/workflow`, ~1,700 LOC, 6 sub-phases       │
  UNLOCKS: Epics C, E, G                            │
       │                                             │
       ├── Phase 2: Policy Upgrade (Epic F) ────────┤
       │    OPA sidecar, pluggable backends          │
       │                                             │
       ├── Phase 3: Secrets Mgmt (Epic E) ───────────┤
       │    Vault client, Activity-level injection   │
       │                                             │
       ├── Phase 4: Settlement Router (Epic C) ──────┤
       │    Cross-chain plan decomposition           │
       │                                             │
       └── Phase 7: EigenLayer (Epic G) ────────────┘
            Operator registry, AVS slashing

 Phase 5: Pact Network (Epic D) - INDEPENDENT
 Phase 6: Arcium MPC (Epic B) - BLOCKED on external audit gate
```

---

## Phase 0: Foundation (~780 LOC, 3-5 days)

**Goal:** Shore up the existing codebase before building new subsystems.

| # | Task | Files | Effort | Depends On |
|---|------|-------|--------|------------|
| 0.1 | Add `ReputationWeighted` policy rule | `crates/core/src/policy/types.rs` + `evaluator.rs` | ~80 LOC | None |
| 0.2 | Populate `agent_reputation` in `NormalizedTransaction` | `crates/sidecar/src/lib.rs` (via `AgentStore`) | ~30 LOC | 0.1 |
| 0.3 | Durable policy state (Sled-backed, not `Mutex`) | `crates/core/src/policy/evaluator.rs` | ~100 LOC | None |
| 0.4 | Reputation feedback loop (auto-strike on block) | `crates/sidecar/src/lib.rs` | ~100 LOC | 0.2 |
| 0.5 | Webhook on block events | `crates/sidecar/src/lib.rs` | ~80 LOC | None |
| 0.6 | HITL approval UI in dashboard | `apps/web/src/pages/Dashboard.tsx` | ~150 LOC | None |
| 0.7 | EVM hooks wired in dashboard | `apps/web/src/hooks/useBastionEVM.ts` | ~150 LOC | None |
| 0.8 | AI intent scoring (heuristic) | `crates/core/src/policy/intent.rs` | ~90 LOC | None |

**Acceptance criteria:**
- [ ] `ReputationWeighted` rule scales transaction limits by agent reputation score
- [ ] Rate limit state survives sidecar restart (Sled-backed, not in-memory `Mutex`)
- [ ] Dashboard shows pending HITL approvals with allow/reject buttons
- [ ] Webhook fires on block events to configured URL
- [ ] Heuristic intent scoring flags mismatched intents (e.g., `"transfer"` vs actual tx data)

---

## Phase 1: Durable Workflow Engine - Epic A (~1,700 LOC, 2-3 weeks)

**Goal:** Bastion's Temporal-equivalent. Turns the runtime from single-shot `execute()` into multi-step durable workflows with deterministic replay, crash recovery, retry, and HITL suspension.

Per [`docs/WORKFLOW_ENGINE_DESIGN.md`](WORKFLOW_ENGINE_DESIGN.md) for full architecture.

### Sub-Phase 1.1: Engine Core (~500 LOC)

| Task | Files | Effort |
|------|-------|--------|
| `WorkflowEngine` struct + start / resume / list / state queries | `crates/workflow/src/engine.rs` | ~150 LOC |
| `WorkflowDefinition` trait + `WorkflowStep` type | `crates/workflow/src/definition.rs` | ~80 LOC |
| `WorkflowState` + `StepState` (Sled-persisted) | `crates/workflow/src/state.rs` | ~100 LOC |
| `WorkflowEngine` execution loop (per-workflow Tokio task) | `crates/workflow/src/engine.rs` | ~100 LOC |
| `WorkflowError` error type | `crates/workflow/src/error.rs` | ~70 LOC |

### Sub-Phase 1.2: Replay & Crash Recovery (~300 LOC)

| Task | Files | Effort |
|------|-------|--------|
| `WorkflowEvent` enum (all event types) | `crates/workflow/src/event.rs` | ~80 LOC |
| Event log append + replay reconstructor | `crates/workflow/src/event.rs` | ~100 LOC |
| Crash recovery: scan Sled for `Running | Paused` on boot, spawn tasks | `crates/workflow/src/engine.rs` | ~120 LOC |

### Sub-Phase 1.3: Retry & HITL (~300 LOC)

| Task | Files | Effort |
|------|-------|--------|
| `RetryPolicy` type + exponential backoff | `crates/workflow/src/retry.rs` | ~80 LOC |
| `Activity` trait + `ActivityRegistry` | `crates/workflow/src/activity.rs` | ~100 LOC |
| HITL integration: `PendingHITL` → suspend workflow → `/override` resumes | `crates/sidecar/src/lib.rs` + `crates/workflow/src/engine.rs` | ~120 LOC |

### Sub-Phase 1.4: Built-in Activities (~200 LOC)

| Activity | Wraps | File |
|----------|-------|------|
| `simulate` | `POST /simulate` (Solana) | `crates/workflow/src/activities/simulate.rs` |
| `simulate_evm` | `POST /api/v2/simulate-evm` | `crates/workflow/src/activities/simulate_evm.rs` |
| `approve` | `POST /override` | `crates/workflow/src/activities/approve.rs` |
| `settle` | `BastionClient.logAudit()` | `crates/workflow/src/activities/settle.rs` |
| `http_call` | Web2 firewall proxy | `crates/workflow/src/activities/http_call.rs` |
| `sleep` | `tokio::time::sleep` | `crates/workflow/src/activities/sleep.rs` |
| `fetch_secret` | Vault (stub in P1, real in P3) | `crates/workflow/src/activities/fetch_secret.rs` |

### Sub-Phase 1.5: SDK Surface (~200 LOC)

| Task | File | Effort |
|------|------|--------|
| `BastionWorkflow` class with `start()`, `signal()`, `state()`, `list()`, `replay()` | `packages/sdk/src/workflow.ts` | ~200 LOC |

### Sub-Phase 1.6: Sidecar Routes (~200 LOC)

| Method | Path | Purpose |
|--------|------|---------|
| `POST` | `/workflows` | Start a new workflow |
| `GET` | `/workflows/:id` | Query state + step outcomes |
| `GET` | `/workflows/:id/events` | Replay event history |
| `POST` | `/workflows/:id/signal` | Resume a paused workflow |
| `GET` | `/workflows` | List workflows (filterable) |

**Acceptance criteria:**
- [ ] A 4-step workflow (swap → bridge → stake → settle) completes end-to-end
- [ ] Sidecar crash mid-workflow → workflow resumes from correct step after restart
- [ ] Step with `requiresApproval: true` pauses until `/override` ALLOW is signaled
- [ ] Step with `maxAttempts: 3` retries twice then marks workflow as failed
- [ ] `GET /workflows/:id/events` returns full deterministic event replay
- [ ] `GET /workflows?agent_id=...&status=running` returns filtered active workflows

**Unlocks:** Epics C (Settlement Router), E (Secrets), G (EigenLayer).

---

## Phase 2: General-Purpose Policy - Epic F (~600 LOC, 1 week)

**Goal:** Complement the 11 hardcoded Rust rules with an optional OPA Rego sidecar for custom policy-as-code.

**Architecture:**
```
PolicyEvaluator
    ├── NativePolicyBackend  (fast path, existing 11 rules, no external dep)
    └── OpaPolicyBackend     (HTTP → OPA sidecar, Rego, versioned bundles)
```

| Task | Files | Effort |
|------|-------|--------|
| `PluggablePolicy` trait (evaluate + dry_run + version) | `crates/core/src/policy/backend.rs` | ~80 LOC |
| `NativePolicyBackend` - wraps existing 11 rules | `crates/core/src/policy/native.rs` | ~50 LOC |
| `OpaPolicyClient` - HTTP client to OPA sidecar | `crates/core/src/policy/opa.rs` | ~150 LOC |
| `PolicyConfig` - select backend per policy set | `crates/core/src/policy/config.rs` | ~50 LOC |
| Docker compose OPA sidecar with example bundle | `docker-compose.opa.yml` | ~30 LOC |
| Example Rego policies (amount-limit, time-window, role-based) | `policies/` directory | ~100 LOC |
| SDK: `bastion.policy.evaluate(opaqueInput)` + `bastion.policy.dryRun(input)` | `packages/sdk/src/policy.ts` | ~100 LOC |
| Sidecar: `GET /policy/backends`, `POST /policy/evaluate` (dry-run) | `crates/sidecar/src/lib.rs` | ~40 LOC |

**Acceptance criteria:**
- [ ] `NativePolicyBackend` passes all existing 11 rule types unchanged
- [ ] `OpaPolicyBackend` evaluates custom Rego rules via HTTP
- [ ] Policy decisions include policy version + backend identifier
- [ ] Dry-run returns decision without blocking execution
- [ ] Fallback: OPA unavailable → default-deny for high-risk, built-in for low-risk

**Depends on:** Phase 0 (durable policy state, so OPA decisions are cached with TTL).

---

## Phase 3: Secrets Management - Epic E (~500 LOC, 1 week)

**Goal:** Vault-equivalent. Activities fetch short-lived, scoped credentials automatically.

| Task | Files | Effort |
|------|-------|--------|
| `VaultClient` - authenticate, read KV v2, issue DB creds, revoke | `crates/vault/src/client.rs` | ~200 LOC |
| `SecretBroker` trait (abstraction: Vault or env fallback) | `crates/vault/src/broker.rs` | ~80 LOC |
| Bastion agent identity → Vault entity/alias mapping | `crates/vault/src/identity.rs` | ~80 LOC |
| `FetchSecret` Activity (real implementation) | `crates/workflow/src/activities/fetch_secret.rs` | ~50 LOC |
| Docker compose Vault dev server + setup script | `docker-compose.vault.yml` + `scripts/vault-init.sh` | ~50 LOC |
| Sidecar: `GET /secrets/health` (Vault connectivity) | `crates/sidecar/src/lib.rs` | ~40 LOC |

**Depends on:** Phase 1 (workflows, because secrets are injected inside Activities).

**Acceptance criteria:**
- [ ] Agent workflow step can request a short-lived DB credential scoped to the step's TTL
- [ ] Credential is revoked at workflow completion or after TTL expiry
- [ ] Secrets are never exposed in workflow event log (redacted before persistence)
- [ ] `GET /secrets/health` reports Vault connectivity status
- [ ] Fallback: `env` backend works without Vault for dev/testing

---

## Phase 4: Settlement Router - Epic C (~800 LOC, 1-2 weeks)

**Goal:** Decompose a high-level intent into an ordered, chain-spanning execution plan.

| Task | Files | Effort |
|------|-------|--------|
| `ExecutionPlan` type - ordered list of per-chain legs with dependencies | `crates/core/src/execution/plan.rs` | ~100 LOC |
| `RouteSelector` - choose chains by cost, latency, reputation-weighted policy | `crates/core/src/execution/router.rs` | ~200 LOC |
| Intent → plan decomposition | `crates/core/src/execution/decomposer.rs` | ~150 LOC |
| Atomicity semantics: compensating actions for partial success | `crates/core/src/execution/atomicity.rs` | ~150 LOC |
| Promote minimal router to real planner | `packages/sdk/src/execute.ts` | ~100 LOC |
| Plan → multi-step workflow adapter | `crates/workflow/src/plan_adapter.rs` | ~100 LOC |

**Depends on:** Phase 1 (atomic multi-leg execution needs durable workflows + compensating actions).

**Acceptance criteria:**
- [ ] `"swap USDC to ETH, bridge to Base, stake in Aave"` → 3-leg plan with chain routing
- [ ] Plan includes compensating actions: `if leg 2 fails → reverse leg 1`
- [ ] Route selection prefers lower-cost chains when policy allows
- [ ] Plan validates against existing per-chain simulation before execution begins
- [ ] `execute()` call signature unchanged - intent-based interface preserved

---

## Phase 5: Pact Network - Epic D (~300 LOC, 3-5 days)

**Goal:** Auto-wrap x402 outbound calls with Pact on-chain refund insurance.

**Independent of all other phases.**

| Task | Files | Effort |
|------|-------|--------|
| `pact pay curl` wrapper in Web2 proxy | `crates/web2-firewall/src/pact.rs` | ~80 LOC |
| Pact `settle_batch` event ingestion into audit log | `crates/sidecar/src/audit.rs` | ~80 LOC |
| SDK: `coverage` option in `execute()` | `packages/sdk/src/execute.ts` | ~60 LOC |
| Policy rule: `CoverageRequired` for specific endpoints | `crates/core/src/policy/types.rs` | ~50 LOC |
| Market route for curated Pact endpoints | `crates/web2-firewall/src/market.rs` | ~30 LOC |

**Acceptance criteria:**
- [ ] Web2-proxied call to a Pact-covered endpoint auto-wraps with `pact pay`
- [ ] `CoverageRequired` policy rule blocks calls to uninsured endpoints
- [ ] Pact `settle_batch` refund events appear in Bastion audit log
- [ ] `coverage: { provider: "pact", tier: "standard" }` in `execute()` works

---

## Phase 6: Real Arcium MPC - Epic B (~600 LOC, 1-2 weeks)

**Goal:** Replace `NoopArciumClient` with genuine MPC-backed confidential policy evaluation.

**Gate:** Mainnet deployment behind external audit (per `docs/MAINNET_READINESS.md` §7).

| Task | Files | Effort |
|------|-------|--------|
| Arcis circuits: private thresholds, private allowlists | `crates/arcium/src/circuits/` | ~200 LOC |
| Solana callback: submit MXE, await result, land attested outcome | `crates/arcium/src/solana/` | ~150 LOC |
| Live MXE client (replaces `NoopArciumClient`) | `crates/arcium/src/client.rs` | ~150 LOC |
| Feature gate flip: `confidential_compute: false → true` | `crates/arcium/src/evaluator.rs` + `crates/sidecar/src/lib.rs` | ~50 LOC |
| Integration test against testnet MXE cluster | `crates/arcium/tests/` | ~100 LOC |

**Acceptance criteria:**
- [ ] Confidential policy evaluation runs on Arcium MXE (not noop)
- [ ] Attested result lands on-chain via Solana callback
- [ ] `/health` reports `confidential_compute: true`
- [ ] `execute({ privacy: "confidential" })` proceeds (not refused)
- [ ] Noop client remains available as default build; live client is feature-gated

---

## Phase 7: EigenLayer AVS - Epic G (~500 LOC, 1-2 weeks)

**Goal:** Operator accountability via EigenLayer AVS slashing/rewards for distributed worker nodes.

**Depends on:** Phase 1 (operator tasks are Activities within durable workflows).

---

## Phase 8: Starknet ZK-Verified Execution - Epic I (~700 LOC, 1-2 weeks)

**Goal:** Add Starknet as an execution layer in Bastion's multi-chain planner. Starknet is an Ethereum ZK-rollup with native account abstraction (every account is a smart account), STARK proofs for provably correct execution, and L1-L2 messaging for trust anchoring to Ethereum.

**Why Starknet + Arcium together:**
- Arcium (Solana) = confidential computation (private policy evaluation via MPC)
- Starknet (Ethereum) = ZK-verified public execution (provably correct + native AA agent wallets)
- They serve different purposes and both are needed for a complete multi-chain runtime.

**Key Starknet properties Bastion leverages:**

| Property | How Bastion Uses It |
|----------|---------------------|
| Native Account Abstraction | Agent wallets are smart accounts by default - no ERC-4337 bundler, no paymaster complexity |
| STARK validity proofs | Policy enforcement on Starknet is provably correct by cryptographic proof, not trust |
| L1-L2 messaging | Execute on Starknet, settle trust records to Ethereum L1 via native bridge |
| Cairo VM | ZK-optimized VM - different from EVM, requires Cairo language for contracts |
| Starkzap SDK | TypeScript SDK with explicit LLM integration docs for AI agents |

**Requirements:**

| Task | Files | Effort |
|------|-------|--------|
| Starknet chain config + RPC integration | `crates/sidecar/src/simulation_starknet.rs` | ~150 LOC |
| Cairo agent wallet contract (native AA) | `crates/starknet/contracts/` | ~150 LOC |
| Per-chain simulator for Starknet transactions | `crates/sidecar/src/simulation_starknet.rs` | ~150 LOC |
| SDK: `settlement: "starknet"` + `settlement: "starknet_sepolia"` | `packages/sdk/src/execute.ts` | ~100 LOC |
| Starkzap SDK integration for agent wallet management | `packages/sdk/src/starknet.ts` | ~100 LOC |
| L1-L2 messaging for trust settlement | `crates/starknet/src/messaging.rs` | ~50 LOC |

**Depends on:** Phase 4 (Settlement Router - Starknet becomes a routing target in the cross-chain planner).

**Acceptance criteria:**
- [ ] `execute({ settlement: "starknet_sepolia", transaction })` simulates against Starknet Sepolia
- [ ] Cairo agent wallet contract deploys and accepts policy-gated transactions
- [ ] L1 → L2 message lands trust record on Ethereum from Starknet
- [ ] Settlement router includes Starknet as a routing option in cross-chain plans
- [ ] `/health` reports `starknet_connected: true` when RPC is reachable

| Task | Files | Effort |
|------|-------|--------|
| Operator registry: register, remove, query | `crates/eigenlayer/src/registry.rs` | ~150 LOC |
| AVS trust model selector: whitelisted → permissionless → economic | `crates/eigenlayer/src/trust.rs` | ~100 LOC |
| Evidence collection: operator task output + policy decision | `crates/eigenlayer/src/evidence.rs` | ~100 LOC |
| Slashing hook integration | `crates/eigenlayer/src/slash.rs` | ~100 LOC |
| `AvsOperator` Activity: delegate task to operator set | `crates/workflow/src/activities/avs_operator.rs` | ~50 LOC |

**Acceptance criteria:**
- [ ] Operators can register with the service
- [ ] Tasks are dispatched to operator set members
- [ ] Misbehavior evidence is collected and linked to operator identity
- [ ] Slashing conditions fire when evidence threshold is met
- [ ] Three operator trust models are selectable per deployment

---

## Post-Production Backlog (~980 LOC total)

These are independent improvements. All are Phase 0 candidates.

| # | Task | Files | Effort |
|---|------|-------|--------|
| PB1 | Reputation auto-strike on block | `crates/sidecar/src/lib.rs` | ~100 LOC |
| PB2 | Webhook on block events | `crates/sidecar/src/lib.rs` | ~80 LOC |
| PB3 | HITL approval UI in dashboard | `apps/web/src/pages/Dashboard.tsx` | ~150 LOC |
| PB4 | AI intent scoring (heuristic mismatch detection) | `crates/core/src/policy/intent.rs` | ~90 LOC |
| PB5 | EVM dashboard hooks (`useBastionEVM.ts`) | `apps/web/src/hooks/useBastionEVM.ts` | ~150 LOC |
| PB6 | Behavioral baseline per agent | `crates/sidecar/src/behavior.rs` | ~300 LOC |

---

## Total Summary

| Phase | Epic | LOC | Duration | Depends On |
|-------|------|-----|----------|------------|
| 0 | Foundation + Backlog | ~780 | 3-5 days | None |
| 1 | A - Durable Workflow Engine | ~1,700 | 2-3 weeks | Phase 0 |
| 2 | F - OPA Policy Integration | ~600 | 1 week | Phase 0 |
| 3 | E - Secrets Management | ~500 | 1 week | Phase 1 |
| 4 | C - Settlement Router | ~800 | 1-2 weeks | Phase 1 |
| 5 | D - Pact Network | ~300 | 3-5 days | **Independent** |
| 6 | B - Real Arcium MPC | ~600 | 1-2 weeks | Audit gate |
| 7 | G - EigenLayer AVS | ~500 | 1-2 weeks | Phase 1 |
| 8 | I - Starknet ZK Execution | ~700 | 1-2 weeks | Phase 4 |
| **Total** | | **~6,480 LOC** | **10-14 weeks sequential** | |

**Parallelism opportunity:** Phases 2+3 can run in parallel after Phase 1. Phases 5 and 6 can run in parallel at any time. Phases 7 and 8 can run in parallel with Phase 4 (all depend on Phase 1 + Phase 4).

---

## Invariants (from ROADMAP.md)

- `crates/core` must stay chain-agnostic - no Solana or EVM imports
- On-chain program must only contain audit/identity/policy primitives - no financial instruments
- SDK major version bump required if IDL instruction set changes
- `main` branch must always pass all 7 CI jobs before merge

---

## References

- TrustFlow comparison: [`docs/TRUSTFLOW_COMPARISON.md`](TRUSTFLOW_COMPARISON.md)
- Workflow engine design: [`docs/WORKFLOW_ENGINE_DESIGN.md`](WORKFLOW_ENGINE_DESIGN.md)
- Competitive landscape: [`docs/COMPETITIVE_LANDSCAPE.md`](COMPETITIVE_LANDSCAPE.md)
- Vision: [`docs/VISION.md`](VISION.md)
- Roadmap: [`docs/ROADMAP.md`](ROADMAP.md)
