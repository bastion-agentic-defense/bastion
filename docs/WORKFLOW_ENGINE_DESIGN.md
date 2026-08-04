# Bastion - Durable Workflow Engine Design (Epic A)

> Architecture spec for Bastion's **Temporal-equivalent** durable workflow runtime.
> This is the largest net-new subsystem on the roadmap - it turns Bastion from a transactional firewall into a true **Programmable Trust Runtime** with multi-step execution, crash recovery, and deterministic replay.
>
> **Status:** 🚧 Design phase. Zero code shipped. This document is the reference architecture.

---

## 1. Why This Exists

Today, `execute()` is synchronous single-step - policy evaluation + simulation + decide. A durable workflow engine enables:

- **Multi-step agent actions** - swap → bridge → stake (3 steps, 3 chains, 1 workflow)
- **Crash recovery** - sidecar crash mid-workflow → resume where it left off
- **Deterministic replay** - reconstruct state from event history for audit
- **Human approval gates** - workflows pause on HITL, resume when human approves
- **Retry with backoff** - failed RPC calls retry automatically with configurable policy

This is what turns Bastion from a firewall into a **runtime**.

---

## 2. Design Principles

| Principle | Why |
|-----------|-----|
| **Deterministic workflows, I/O in Activities** | Workflow code must replay identically. All external calls (simulation, chain RPC, secrets) live in Activities. Same split as Temporal. |
| **Event-sourced state** | Every state transition is an event in the log. Replay = replay events. No mutable workflow state outside the log. |
| **Idempotency by construction** | Every step carries an idempotency key. Re-delivery = no-op for completed steps. |
| **Reuse Sled for persistence** | `crates/sidecar/src/audit.rs` already depends on Sled. Workflow state lives in the same store. No new database. |
| **Chain-agnostic core** | `crates/workflow` has zero Solana or EVM imports. Same contract as `crates/core`. |

---

## 3. Architecture

```
                    POST /workflows
                    GET  /workflows/:id
                    POST /workflows/:id/signal
                         │
                         ▼
              ┌──────────────────────┐
              │   crates/sidecar     │
              │  (workflow routes +  │
              │   existing API)      │
              └──────────┬───────────┘
                         │
                         ▼
              ┌──────────────────────┐
              │   crates/workflow    │
              │                      │
              │  WorkflowEngine      │
              │  WorkflowDefinition  │
              │  ActivityRegistry    │
              │  WorkflowState       │
              │  EventLog            │
              └──────────┬───────────┘
                         │
                    ┌────┴────┐
                    ▼         ▼
           ┌──────────┐  ┌──────────┐
           │   Sled   │  │  Policy  │
           │ (persist)│  │Evaluator │
           └──────────┘  │(existing)│
                         └──────────┘
```

### Crate: `crates/workflow`

```
crates/workflow/
├── Cargo.toml
├── src/
│   ├── lib.rs          ← Public API
│   ├── engine.rs       ← WorkflowEngine: start, resume, replay
│   ├── definition.rs   ← WorkflowDefinition, Step trait
│   ├── state.rs        ← WorkflowState, StepState
│   ├── event.rs        ← WorkflowEvent enum (all event types)
│   ├── activity.rs     ← Activity trait + ActivityRegistry
│   ├── retry.rs        ← RetryPolicy: attempts, backoff, timeout
│   └── error.rs        ← WorkflowError
└── tests/
    ├── engine_tests.rs
    └── replay_tests.rs
```

---

## 4. Core Types

### 4.1 WorkflowDefinition

```rust
/// A user-defined workflow that composes steps.
pub trait WorkflowDefinition: Send + Sync {
    fn name(&self) -> &str;
    fn steps(&self) -> Vec<WorkflowStep>;
}

/// A single step in a workflow - what to execute, how to retry.
pub struct WorkflowStep {
    pub id: String,            // "swap", "bridge", "stake"
    pub activity: String,      // registered activity name
    pub input: serde_json::Value,
    pub retry: RetryPolicy,
    pub timeout: Duration,
    pub requires_approval: bool, // pause for HITL before executing
}
```

### 4.2 WorkflowState (Persisted to Sled)

```rust
/// Durable state of one workflow run.
pub struct WorkflowState {
    pub id: String,                    // UUID
    pub definition: String,            // workflow name
    pub status: WorkflowStatus,
    pub current_step: usize,           // index into steps
    pub step_states: Vec<StepState>,   // per-step outcomes
    pub created_at: u64,
    pub updated_at: u64,
}

pub enum WorkflowStatus {
    Running,
    Paused,          // waiting for HITL approval
    Completed,
    Failed(String),  // unrecoverable error
    Cancelled,
}

pub struct StepState {
    pub step_id: String,
    pub status: StepStatus,
    pub input: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub attempt: u32,
    pub started_at: Option<u64>,
    pub completed_at: Option<u64>,
}

pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    Paused,  // waiting for external signal
    Skipped,
}
```

### 4.3 Event Log

```rust
/// Every state transition emits an event.
pub enum WorkflowEvent {
    WorkflowStarted { id: String, definition: String },
    StepStarted { id: String, step: String, attempt: u32 },
    StepCompleted { id: String, step: String, output: serde_json::Value },
    StepFailed { id: String, step: String, error: String, attempt: u32 },
    StepRetrying { id: String, step: String, attempt: u32, backoff_ms: u64 },
    WorkflowPaused { id: String, step: String, reason: String },
    WorkflowResumed { id: String, by: String },
    WorkflowCompleted { id: String },
    WorkflowFailed { id: String, error: String },
    WorkflowCancelled { id: String },
}
```

### 4.4 Activity Trait

```rust
/// Activities perform all I/O. They are NOT deterministic - they run once, and
/// their output is recorded in workflow history. During replay, Activities are
/// skipped and their recorded output is reused.
#[async_trait]
pub trait Activity: Send + Sync {
    fn name(&self) -> &str;
    async fn execute(
        &self,
        input: serde_json::Value,
        ctx: ActivityContext,
    ) -> Result<serde_json::Value, ActivityError>;
}

pub struct ActivityContext {
    pub workflow_id: String,
    pub step_id: String,
    pub attempt: u32,
    pub agent_id: Option<String>,
}
```

### 4.5 RetryPolicy

```rust
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_backoff_ms: u64,
    pub max_backoff_ms: u64,
    pub backoff_multiplier: f64,   // 2.0 = exponential
    pub timeout_ms: u64,           // per-attempt timeout
}
```

---

## 5. Built-in Activities

Bastion ships these Activities out of the box. Each wraps existing primitives.

| Activity | Wraps | I/O |
|----------|-------|-----|
| `simulate` | `POST /simulate` (Solana) | Firewall decision |
| `simulate_evm` | `POST /api/v2/simulate-evm` | EVM firewall decision |
| `approve` | `POST /override` | Human approval via HITL |
| `settle` | `BastionClient.logAudit()` | On-chain audit record |
| `fetch_secret` | Vault / env-based | Short-lived credential |
| `http_call` | Web2 firewall proxy | Proxied API call |
| `sleep` | tokio::time::sleep | Timer (deterministic replay via event timestamp) |

---

## 6. WorkflowEngine

The engine is the runtime loop. It runs on a Tokio task per workflow.

```rust
pub struct WorkflowEngine {
    db: sled::Db,                           // persisted state
    activities: ActivityRegistry,
    policy: Arc<PolicyEvaluator<impl RiskOracle>>,
    active_runs: DashMap<String, JoinHandle<()>>,
}

impl WorkflowEngine {
    /// Start a new workflow. Returns the workflow ID immediately.
    pub async fn start(
        &self,
        definition: &dyn WorkflowDefinition,
        agent_id: Option<String>,
    ) -> Result<String, WorkflowError>;

    /// Resume a paused workflow (e.g., after HITL approval).
    pub async fn resume(
        &self,
        workflow_id: &str,
        signal: Signal,
    ) -> Result<(), WorkflowError>;

    /// Query workflow state.
    pub fn state(&self, workflow_id: &str) -> Result<WorkflowState, WorkflowError>;

    /// List active/paused workflows for an agent.
    pub fn list(&self, agent_id: &str) -> Vec<WorkflowState>;

    /// Replay a workflow from its event log - reconstructs state without
    /// re-executing Activities. Used for audit and crash recovery.
    pub fn replay(&self, workflow_id: &str) -> Result<Vec<WorkflowEvent>, WorkflowError>;
}
```

### Execution Loop (per workflow)

```
1. Load WorkflowState from Sled
2. If status == Running:
   a. Get current step
   b. If step requires_approval and not yet approved → set Paused, emit event
   c. Else → run Activity (with retry)
   d. On success → record output, advance to next step
   e. On failure → retry or mark Failed
   f. On completion → mark Completed, emit event
3. If status == Paused → wait for resume signal
```

### Crash Recovery

On sidecar boot, the engine scans Sled for all `Running | Paused` workflows and spawns a Tokio task for each. The task:
1. Replays events from the log to reconstruct current step index + step states
2. Continues from the current step (completed steps are skipped - their outputs are in the log)
3. This is the same replay model as Temporal

---

## 7. HITL Integration

When a step has `requires_approval: true`, the workflow suspends:

```
Step: "swap_10k_sol"
  ├── Policy evaluates → PendingHITL (amount > trigger_above)
  ├── Workflow status → Paused
  ├── Engine emits WorkflowPaused with approval_id
  │
  ... human reviews and calls POST /override { block_id, action: "ALLOW" } ...
  │
  ├── Sidecar `/override` handler calls workflow_engine.resume()
  ├── Workflow status → Running
  ├── Step executes (approval recorded in Activity input)
  └── Engine continues to next step (or completes)
```

The approval_id from the existing `FirewallDecision::PendingHITL` becomes the signal key for resuming.

---

## 8. Retry Strategy

```
Attempt 1: immediate
  ↓ fail
Attempt 2: wait initial_backoff_ms
  ↓ fail
Attempt 3: wait initial_backoff_ms * backoff_multiplier
  ↓ fail
...
Attempt N: wait min(max_backoff_ms, initial * multiplier^(n-1))
  ↓ fail (N == max_attempts)
Step marked Failed, workflow continues or halts per configuration
```

Each Activity execution is wrapped in `tokio::time::timeout(timeout_ms)`. Timeout = failure → retry.

---

## 9. SDK Surface

```typescript
// packages/sdk/src/workflow.ts

interface WorkflowConfig {
  name: string;
  steps: WorkflowStep[];
}

interface WorkflowStep {
  id: string;
  activity: "simulate" | "simulate_evm" | "approve" | "settle" | "http_call";
  input: Record<string, unknown>;
  retry?: { maxAttempts: number; backoffMs: number };
  timeoutMs?: number;
  requiresApproval?: boolean;
}

class BastionWorkflow {
  constructor(config: { sidecar: BastionSidecar; client?: BastionClient });

  // Start a multi-step workflow.
  start(config: WorkflowConfig): Promise<{ workflowId: string }>;

  // Resume a paused workflow with a signal.
  signal(workflowId: string, signal: Signal): Promise<void>;

  // Get workflow state, including per-step outcomes.
  state(workflowId: string): Promise<WorkflowState>;

  // List active/paused workflows for the current agent.
  list(): Promise<WorkflowState[]>;

  // Replay event history for audit.
  replay(workflowId: string): Promise<WorkflowEvent[]>;
}
```

---

## 10. Sidecar Routes (New)

| Method | Path | Purpose |
|--------|------|---------|
| `POST` | `/workflows` | Start a new workflow. Body: `{ name, steps, agent_id }`. Returns `{ workflow_id }`. |
| `GET` | `/workflows/:id` | Query workflow state + step outcomes. |
| `GET` | `/workflows/:id/events` | Replay event history. |
| `POST` | `/workflows/:id/signal` | Resume a paused workflow. Body: `{ signal: "approve", approval_id }`. |
| `GET` | `/workflows` | List workflows. Query: `?agent_id=...&status=running`. |

---

## 11. Integration with Existing Systems

### PolicyEvaluator

Workflow steps that need policy checks call the existing evaluator. The step's Activity input includes the `NormalizedTransaction` fields needed for evaluation. The `FirewallDecision::PendingHITL` outcome triggers the workflow pause flow.

### Web2 Firewall

The `http_call` Activity proxies calls through the existing `bastion-web2-firewall`. Rate limits, budgets, and content inspection apply automatically.

### Audit Log

Every `WorkflowEvent` is recorded in the existing Sled audit store. `GET /logs?workflow_id=...` returns the full event chain. On-chain settlement (`settle` Activity) calls the existing `BastionClient.logAudit()`.

### Arcium (Future)

A `confidential_evaluate` Activity can replace `simulate` for privacy-preserving policy evaluation via Arcium MXE, once the real MPC client ships.

---

## 12. Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| Sled, not Postgres | Already a dependency. Zero new infra. Embeddable. |
| Tokio tasks per workflow, not a central scheduler | Simpler. Each workflow is independent. Backpressure via semaphore. |
| Activity trait (not enum) | Extensible. Users can register custom Activities. Same pattern as Temporal. |
| Events are append-only | Enables replay, audit, and crash recovery. No mutable state outside the log. |
| Idempotency keys per step | Re-delivery safe. Same key = same outcome. |
| No cross-workflow dependencies (v1) | Simpler. DAG / fan-out / fan-in is a v2 feature. |

---

## 13. Example: Multi-Step Agent Workflow

```typescript
const bastion = new Bastion({ sidecar, client });
const wf = new BastionWorkflow({ sidecar, client });

// Define a cross-chain workflow: swap on Solana → bridge to Base → stake
const { workflowId } = await wf.start({
  name: "swap-bridge-stake",
  steps: [
    {
      id: "swap",
      activity: "simulate",
      input: { transaction: solanaSwapTx, intent: "swap SOL to USDC" },
      retry: { maxAttempts: 3, backoffMs: 1000 },
      timeoutMs: 30000,
    },
    {
      id: "bridge",
      activity: "simulate",
      input: { transaction: bridgeTx, intent: "bridge USDC to Base" },
      requiresApproval: true,  // pause for human before bridging
      retry: { maxAttempts: 3, backoffMs: 2000 },
      timeoutMs: 60000,
    },
    {
      id: "stake",
      activity: "simulate_evm",
      input: { transaction: stakeTx, intent: "stake on Base", chain: "base" },
      retry: { maxAttempts: 2, backoffMs: 1000 },
      timeoutMs: 30000,
    },
    {
      id: "settle",
      activity: "settle",
      input: {},
    },
  ],
});

// Later, human approves the bridge step:
await wf.signal(workflowId, {
  signal: "approve",
  approvalId: "...",
});
```

Workflow survives a sidecar crash between any of these steps and resumes where it left off.

---

## 14. Implementation Plan

| Phase | Scope | Estimate |
|-------|-------|----------|
| **Phase 1 - Engine core** | `crates/workflow` crate, `WorkflowEngine`, `WorkflowState`, Sled persistence, basic execution loop | ~500 LOC Rust |
| **Phase 2 - Replay & recovery** | Event log, replay from history, crash recovery on boot | ~300 LOC Rust |
| **Phase 3 - Retry & HITL** | `RetryPolicy`, pause/resume via existing `/override`, signal routes | ~300 LOC Rust |
| **Phase 4 - SDK surface** | `BastionWorkflow` class, typed step config, event streaming | ~200 LOC TS |
| **Phase 5 - Sidecar routes** | `POST /workflows`, `GET /workflows/:id`, signal, list | ~200 LOC Rust |
| **Phase 6 - Built-in Activities** | `simulate`, `simulate_evm`, `settle`, `http_call`, `sleep` | ~200 LOC Rust |

**Total:** ~1,700 LOC across Rust + TypeScript.

---

## 15. Success Criteria

- [ ] A 4-step workflow (swap → bridge → stake → settle) completes end-to-end
- [ ] Sidecar crash mid-workflow → workflow resumes from correct step after restart
- [ ] A step with `requiresApproval: true` pauses until `/override` ALLOW is called
- [ ] A step with `maxAttempts: 3` retries twice on failure, then marks workflow as failed
- [ ] `GET /workflows/:id/events` returns the full deterministic event replay
- [ ] `GET /workflows?agent_id=...` lists active and paused workflows

---

## 16. Out of Scope (v1)

- Cross-workflow DAG / fan-out / fan-in
- Temporal-style child workflows
- Workflow versioning / migration
- gRPC / external worker protocol
- Workflow scheduling (cron-style triggers)

---

## 17. References

- TrustFlow comparison: [`docs/TRUSTFLOW_COMPARISON.md`](TRUSTFLOW_COMPARISON.md)
- Existing policy engine: `crates/core/src/policy/evaluator.rs`
- Existing SDK executor: `packages/sdk/src/execute.ts`
- Existing audit store: `crates/sidecar/src/audit.rs`
- Roadmap Epic A: `docs/ROADMAP.md`
