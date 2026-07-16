# Bastion vs TrustFlow — Architecture Comparison

> TrustFlow is a Perplexity-generated PRD for a platform combining **Temporal + HashiCorp Vault + OPA + EigenLayer** into a secure workflow orchestration platform. This document compares it against what Bastion already ships and where the gaps are.

---

## 1. Side-by-Side: The Four Pillars

| Component | TrustFlow (Temporal+Vault+OPA+EigenLayer) | Bastion Status | Gap |
|-----------|------------------------------------------|----------------|-----|
| **Durable Workflow** | Temporal: deterministic replay, retries, timeouts, signals, child workflows, crash recovery via event history | 🚧 **Epic A — not yet built.** `execute()` is synchronous single-step (simulate → decide). No state machine, no retry, no crash recovery. See [`WORKFLOW_ENGINE_DESIGN.md`](WORKFLOW_ENGINE_DESIGN.md). | **Critical.** Largest net-new subsystem. |
| **Secrets Management** | Vault: short-lived dynamic credentials, identity-based auth, encryption-as-a-service, key rotation | ❌ **Absent.** No secrets layer. `crates/sidecar/src/auth.rs` is API-key auth for the sidecar itself, not a secrets broker for workloads. | **High.** Needed for production agent workflows that need API keys, DB creds, signing keys. |
| **Policy Engine** | OPA/Rego: general-purpose, versioned, structured decisions with obligations, dry-run, policy-as-code | ✅ **Shipped — simpler.** 11 hardcoded rule types in Rust (`PolicyEvaluator` at `crates/core/src/policy/evaluator.rs`). Pass/Block/PendingHITL outcomes. No Rego, no custom rules without recompile. | **Medium.** Functional for transaction firewalling. Needs general-purpose policy for broader runtime. |
| **Operator Trust / AVS** | EigenLayer: AVS operator slashing, rewards, operator sets, cryptoeconomic security | ❌ **Referenced in docs only.** Zero integration code. Mentioned in `COMPETITIVE_LANDSCAPE.md`. | **Low.** Requires durable workflows (Epic A) first. |

---

## 2. Where Bastion Already Leads TrustFlow

| Capability | Bastion Has | TrustFlow Doesn't |
|-----------|-------------|-------------------|
| Multi-chain simulation (Solana + EVM) | ✅ `crates/sidecar/src/simulation.rs` + `simulation_evm.rs` | ❌ Never described |
| On-chain audit program | ✅ Anchor program (Solana) + Solidity contracts (EVM, EIP-712) | ❌ Off-chain audit only |
| Human-in-the-loop (HITL) | ✅ `FirewallDecision::PendingHITL` + `/override` endpoint | ❌ Mentioned but not designed |
| Physical agent rules | ✅ Geofence, SpeedLimit, EnergyBudget, OperatingHours in `PolicyRule` | ❌ Not applicable / not designed |
| Web2 API firewall | ✅ `bastion-web2-firewall` crate with rate/budget/cost rules | ❌ Not described |
| MCP server | ✅ 15 tools + 3 prompts on port 3001 | ❌ No LLM integration |
| x402 payments | ✅ Pay-per-call with SOL + free tier, pay.sh provider | ❌ Assumes internal platform, no payment model |
| Agent identity + delegation | ✅ W3C DID, ERC-8004 compatible, delegation tree via `POST /agents/:did/delegate` | ❌ Relies on Vault identity only |

---

## 3. Where Bastion Needs to Catch Up

### 3.1 Durable Execution — The Critical Gap

**TrustFlow:** Temporal provides deterministic, replayable workflows. Every state transition is recorded in event history. Worker crashes → replay from history. Activities do all I/O; workflow code stays pure.

**Bastion:** `execute()` (at `packages/sdk/src/execute.ts`) is a single synchronous call:

```typescript
// The entire execution path today:
const simulation = await this.sidecar.simulate({ transaction, intent });
// returns { decision: "pass" | "block" | "pending_hitl" }
```

No multi-step workflows. No durable state. No retry. No crash recovery.

**What TrustFlow does that Bastion cannot (today):**
- `POST /workflows/start` → multi-step process with durable state
- `POST /workflows/{id}/signal` → human approval gates that survive crashes
- `GET /workflows/{id}/history` → full event replay for audit
- Worker failure → automatic recovery with no data loss

See [`WORKFLOW_ENGINE_DESIGN.md`](WORKFLOW_ENGINE_DESIGN.md) for the detailed architecture spec to close this gap.

### 3.2 Dynamic Secrets

**TrustFlow:** Vault issues short-lived database credentials, API keys, and signing keys scoped to the exact operation and TTL. Workflow code never holds long-lived secrets.

**Bastion:** No secrets management. If a Bastion-protected agent needs to call an API, it must handle its own API keys. Bastion's Web2 firewall can intercept the call but not manage the credential.

**What's needed:** A secrets broker that:
- Maps Bastion agent identity to Vault entities
- Issues short-lived credentials within Activities (once workflows exist)
- Rotates API keys for Web2-proxied endpoints
- Encrypts sensitive payload fields

### 3.3 General-Purpose Policy (OPA/Rego)

**TrustFlow:** OPA evaluates Rego policies. Any rule composes naturally. Policies are versioned as code. Structured decisions include obligations.

**Bastion `PolicyEvaluator` comparison:**

| Feature | OPA/Rego | Bastion |
|---------|---------|---------|
| Language | Rego (Turing-complete) | Rust enum (11 hardcoded variants) |
| Custom rules | Write any Rego policy | Must add Rust code + recompile |
| Versioning | Bundle versioning built-in | Not versioned |
| Dry-run / test | OPA test framework | Rust unit tests only |
| Obligations | `allow` + `obligations` array | Not supported |
| Runtime | Sidecar or service | Embedded in `crates/core` |
| Physical rules | Not designed for it | ✅ Geofence, SpeedLimit, etc. |

**Recommendation:** Evolve `PolicyEvaluator` to support a pluggable backend model — native Rust rules for performance-critical paths + optional OPA sidecar integration for general-purpose policy-as-code.

### 3.4 EigenLayer AVS

**TrustFlow:** Operator registration, sets, slashing/rewards, trust model selection (whitelisted → permissionless → economically secured).

**Bastion:** Mentioned in docs. No code exists.

**Dependency:** Requires durable workflows (Epic A) first — operator tasks are Activities that must be scheduled, tracked, and settled durably.

---

## 4. Policy Engine Deep Dive

Bastion's `PolicyEvaluator<O: RiskOracle>` at `crates/core/src/policy/evaluator.rs`:

```
evaluate(tx, policy) → iterate rules → first violation stops → return decision
```

**Current rule types (11):**

| Rule | Type | Purpose |
|------|------|---------|
| `AmountLimit` | Financial | Cap per-tx + optional 24h volume |
| `Destination` | Network | Allowlist + blocklist of addresses |
| `Frequency` | Rate limit | Max transactions per hour |
| `HITL` | Approval | Trigger above amount threshold |
| `Reputation` | Trust | Minimum oracle score required |
| `TxTypeAllowlist` | Type safety | Restrict transfer/deploy/etc. |
| `StakeWeighted` | Financial | SOL-stake-based limit scaling |
| `Geofence` | Physical | Lat/lon bounding box |
| `SpeedLimit` | Physical | Max speed in m/s |
| `EnergyBudget` | Physical | Max Joules per 24h |
| `OperatingHours` | Physical | UTC hour window |

**State model:** In-memory `Mutex<RateLimitState>`. Survives process restarts = reset to zero. Not durable.

---

## 5. TrustFlow Architecture Mapped to Bastion

```
TrustFlow                                    Bastion Today           Bastion Target
─────────────────────────────────────────────────────────────────────────────────
Client / UI / API                            REST sidecar :3000      Same + workflow routes
        │                                           │
TrustFlow Control API                       `crates/sidecar`        + `crates/workflow`
        │                                           │
Temporal (durable orchestration)            ❌ absent               `crates/workflow`
Vault (secrets, crypto)                     ❌ absent               `crates/vault` or external
OPA (policy decisions)                      `PolicyEvaluator`       Pluggable: native + OPA
EigenLayer (AVS operators)                  ❌ absent               `crates/eigenlayer`
Worker / Activity Runtime                   ❌ absent               `crates/workflow::Activity`
External Systems (DB, API, chains)          Simulation + Web2 FW    Same + Activity exec
Audit / Observability                       Sled DB + on-chain      Same + workflow events
```

---

## 6. Build Order Recommendation

| Priority | Component | Why |
|----------|-----------|-----|
| **1** | Durable Workflow Engine | Required before secrets, EigenLayer, or multi-step execution make sense. See [design doc](WORKFLOW_ENGINE_DESIGN.md). |
| **2** | OPA Integration | Pluggable policy backend to unlock general-purpose policy-as-code. |
| **3** | Secrets Management | Vault integration within Activities, once workflows exist. |
| **4** | EigenLayer AVS | Operator accountability, post-workflows. |

---

## 7. References

- TrustFlow PRD source: Perplexity deep-dive analysis (July 2026)
- Bastion policy engine: `crates/core/src/policy/evaluator.rs`
- Bastion executor: `packages/sdk/src/execute.ts`
- Bastion roadmap: `docs/ROADMAP.md` (Epic A)
- Workflow engine design: [`docs/WORKFLOW_ENGINE_DESIGN.md`](WORKFLOW_ENGINE_DESIGN.md)
