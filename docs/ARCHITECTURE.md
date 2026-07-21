# Bastion Architecture

> **The Programmable Trust Runtime for Autonomous Systems.**
>
> Bastion is the enforcement and execution layer in the ZKOS Labs ecosystem.
> It sits beside DAEMON and ARES — not inside them.
>
> **DAEMON** decides. **ARES** analyzes. **Bastion** orchestrates trusted execution.

---

## 1. Ecosystem Positioning

```text
                 DAEMON
        Decision Intelligence
           "What should happen?"
                     │
                     ▼
                  ARES
      Security Intelligence & Analysis
           "What needs attention?"
                     │
                     ▼
                Bastion
         Programmable Trust Runtime
           "Can we do this safely?"
                     │
                     ▼
Ethereum · zkSync · Base · Celo · Robinhood · Solana · Arcium
```

| Product | Telemetry | Question |
|---------|-----------|----------|
| **DAEMON** | Business telemetry | *What should happen?* |
| **ARES** | Security telemetry | *What needs attention?* |
| **Bastion** | Trust observability | *Did the runtime execute safely?* |

---

## 2. What Bastion Owns

Everything related to **trusted execution**:

| Capability | Implementation |
|-----------|---------------|
| Identity verification | DID resolution, ERC-8004 agent registry, delegation tree |
| Policy enforcement | `PolicyEvaluator` (11 rule types), `PolicySet`, `TrustSignalProvider` |
| Human approval | HITL gate with `/override` endpoint, timeout-aware |
| Durable execution | Transaction simulation (Helius + EVM `eth_call`), balance-change prediction |
| Wallet orchestration | ERC-7579 validator module, ERC-4337 UserOps, multi-chain routing |
| Multi-chain execution | 8 execution environments (Solana, Ethereum, Base, Celo, Arbitrum, zkSync Era, Robinhood, Arcium) |
| Trust ledger | On-chain audit (Anchor PDA + EIP-712), off-chain Sled DB |
| Cross-chain settlement | `TrustAdapter` trait for chain-independent execution |
| Confidential execution | Arcium MXE integration for MPC-backed policy evaluation |
| Trust observability | Metrics, traces, policy decisions, approval history, attestations |

---

## 3. What Bastion Does NOT Own

Boundaries kept clean by design:

| Concern | Owner |
|---------|-------|
| Ontology management | Altius |
| Data pipelines / ETL | Altius |
| AML investigation | DAEMON |
| Case management | DAEMON |
| Security scanning | ARES |
| Threat intelligence / OSINT | ARES |
| SIEM correlation | ARES |
| Vulnerability analysis | ARES |
| Kubernetes deployment | Orion |
| Graph analytics | DAEMON |

**Rule of thumb:** If it answers *"Should we do this?"* → DAEMON or ARES. If it answers *"Can we do this safely, and how do we execute it?"* → Bastion.

---

## 4. Bastion Runtime Components

These are the components Bastion **owns**. Everything else is composable infrastructure.

```text
                    Applications
         AI Agents · Enterprises · DAEMON
                           │
────────────────────────────────────────────────────────
                   Bastion Runtime
────────────────────────────────────────────────────────
Policy Compiler    · Trust Signal Consumer · Transaction Firewall
Trust Graph        · Evidence Aggregator   · Audit Log
Agent Session Manager · Human Approval Engine
────────────────────────────────────────────────────────
```

| Component | Crate/File | Purpose |
|-----------|-----------|---------|
| **Policy Compiler** | `crates/core/src/policy/evaluator.rs` | Normalizes chain-specific transactions, evaluates 11 rule types, returns Pass/Block/PendingHITL |
| **Trust Signal Consumer** | `crates/core/src/risk/` | `TrustSignalProvider` trait — Bastion consumes signals from ARES or any provider; never computes intelligence |
| **Transaction Firewall** | `crates/sidecar/src/simulation.rs` + `simulation_evm.rs` | Pre-execution simulation (Helius for Solana, eth_call for EVM). Predicts balance changes before signing |
| **Trust Graph** | `crates/sidecar/src/did.rs` + `AgentStore` | Hierarchical agent delegation tree (parent → sub-agent → sub-sub). Capability inheritance, budget enforcement |
| **Evidence Aggregator** | `crates/sidecar/src/audit.rs` | Every policy decision, simulation result, and approval action → Sled DB + optional on-chain record |
| **Audit Log** | `crates/solana/` (Anchor) + `evm/` (EIP-712) | Immutable on-chain audit trail. Cryptographic provenance for every trust decision |
| **Agent Session Manager** | `crates/sidecar/src/lib.rs` `/agents` routes | Agent lifecycle: register (DID), delegate, query tree, revoke. Budget, reputation, expiry |
| **Human Approval Engine** | `crates/sidecar/src/lib.rs` `/override` route | HITL gate — `PendingHITL` pauses execution until human approves/rejects. Timeout-aware |

---

## 5. Trust Observability

Bastion records runtime-centric telemetry to prove correct execution — distinct from ARES security telemetry and DAEMON business telemetry.

```
Trust Observability
├── Execution Traces       (per-transaction lifecycle)
├── Policy Decisions       (Pass / Block / PendingHITL)
├── Approval History       (human-in-the-loop events)
├── Trust Signal Lookups   (source, score, confidence, TTL)
├── Settlement Status      (on-chain finality)
├── Attestations           (EAS / Sign Protocol / EIP-712)
├── Cross-Chain Traces     (multi-chain execution chain)
└── Runtime Metrics        (latency, throughput, reliability)
```

**What Bastion does NOT record here:** Threat indicators, IOC correlation, vulnerability findings, security incidents — those are ARES territory.

---

## 6. TrustAdapter — Chain Independence

```rust
/// Each execution environment implements this trait, making Bastion's
/// runtime chain-independent.
#[async_trait]
pub trait TrustAdapter: Send + Sync {
    async fn authenticate(&self, address: &Address) -> Result<AgentIdentity, TrustAdapterError>;
    async fn verify(&self, tx: &NormalizedTransaction) -> Result<SimulationOutcome, TrustAdapterError>;
    async fn execute(&self, tx: &NormalizedTransaction) -> Result<ExecutionReceipt, TrustAdapterError>;
    async fn settle(&self, receipt: &ExecutionReceipt) -> Result<(), TrustAdapterError>;
    fn chain_name(&self) -> &str;
    fn chain(&self) -> Chain;
}
```

| Adapter (planned) | Strengths |
|-------------------|-----------|
| `EthereumAdapter` | Trust anchor, final settlement, EAS/Sign attestations |
| `ZkSyncAdapter` | Native AA, paymasters, gas sponsorship, ZK verification |
| `SolanaAdapter` | High-perf workflow coordination, Helius simulation |
| `ArciumAdapter` | Confidential MPC computation |
| `MidnightAdapter` | Privacy-preserving selective disclosure |
| `RobinhoodAdapter` | RWA settlement, Arbitrum Orbit L2 |

---

## 7. Execution Environments

8 chains supported across 3 families:

| Chain | Type | Chain ID | Role |
|-------|------|----------|------|
| **Solana** | High-perf L1 | — | Durable workflow coordination, on-chain audit program |
| **Ethereum** | L1 | 1 | Trust anchoring, final settlement, attestations |
| **Base** | OP Stack L2 | 8453 | Consumer applications, embedded wallets |
| **Celo** | Mobile-first L1 | 42220 | Mobile-first agent operations |
| **Arbitrum** | Optimistic rollup | 42161 | High-throughput EVM execution |
| **zkSync Era** | ZK rollup, native AA | 324 | Native account abstraction, paymasters, gas sponsorship |
| **Robinhood** | Arbitrum Orbit L2 | 4663 | Tokenized RWA settlement |
| **Arcium MXE** | MPC network | — | Confidential policy evaluation |

---

## 8. Wallet Infrastructure

| Standard | Provides | Bastion's Role |
|----------|---------|---------------|
| **ERC-4337** | Account abstraction — UserOps, bundlers, paymasters | Policy-aware execution, multi-chain routing |
| **EIP-7702** | Smart EOAs — delegation without migration | Policy enforcement for EOA-based agents |
| **ERC-7579** | Modular smart accounts — validators, executors, hooks | Bastion's policy validator as a pluggable module |
| **Session Keys** | Temporary, scoped keys for agent sessions | Limit blast radius of compromised agent keys |
| **ERC-7715** (future) | Standardized session key grants | When mature, standardized session key management |
| **Safe** | Multisig (2-of-3 for agents) | Recovery and batching for high-value operations |
| **Coinbase Smart Wallet** | Embedded wallet with passkey auth | Used by Base MCP for agent wallets |

---

## 9. Future: Trust Intent

Rather than passing low-level execution requests, upstream systems (DAEMON, ARES) submit
a **Trust Intent** — a declarative specification of *what* should happen. Bastion resolves
*how* to carry it out under trust constraints.

```yaml
intent: transfer
asset: USDC
amount: 5000
recipient: 0x...
requirements:
  - humanApproval
  - sanctionsCheck
  - maxRisk: medium
  - settlement: ethereum
```

Bastion then resolves:
- which `TrustSignalProvider`s to query
- which `PolicySet` rules apply
- whether human approval is required
- which chain and account abstraction flow to use
- how to execute and observe the workflow

This keeps DAEMON and ARES focused on producing *what* should happen, while Bastion owns
*how* it is carried out under trust constraints.

---

## 10. Capability Architecture

```text
────────────────────────────────────────────────────────────────────
                      Bastion Runtime
────────────────────────────────────────────────────────────────────
  Bastion Owns                           Composable Infrastructure
────────────────────────────────────────────────────────────────────

IDENTITY & TRUST                       AGENT INTEROPERABILITY
  Trust Graph                            A2A · MCP · ACP (future)
  Agent Session Manager

POLICY ENGINE                          TRUST SIGNAL PROVIDERS
  Policy Compiler                        GrondOSINT (ARES-owned endpoint)
  TrustSignalConsumer                    Chainalysis · TRM · Webacy
  Human Approval Engine

WALLET & AUTHORIZATION                 WALLET INFRASTRUCTURE
  Transaction Firewall                   ERC-4337 · EIP-7702 · ERC-7579
                                         Session Keys · ERC-7715
                                         Safe · Coinbase Smart Wallet

EVIDENCE & PROVENANCE                  ATTESTATION PROTOCOLS
  Evidence Aggregator                    EAS · Sign Protocol · Sigil
  Trust Ledger (on-chain + off-chain)    zkTLS · Reclaim · zkPass
                                         The Graph

EXECUTION                              EXECUTION ENVIRONMENTS
  TrustAdapter (chain abstraction)       Solana · Ethereum · Base · Celo
                                         Arbitrum · zkSync Era · Robinhood
                                         Arcium MXE

CONFIDENTIAL COMPUTE                   PRIVACY INFRASTRUCTURE
  (Planned: Privacy Runtime)             Arcium MXE · SP1 · Lit
                                         Nillion · Aztec

SETTLEMENT                             SETTLEMENT LAYERS
  (Planned: Settlement Router)           Ethereum · Pact Network
                                         EigenLayer

CONNECTIVITY                           BRIDGING
  Web2 Firewall                          Across · LayerZero
                                         Wormhole · Hyperlane

PAYMENTS                               PAYMENT RAILS
  x402 integration                       x402 · Pact Network
  Web2 Firewall budget rules             EIP-3009 · ERC-2612
```

---

## 11. Data Flow

### Transaction Evaluation

```
Agent → Chain adapter → normalize → PolicyEvaluator::evaluate()
  ├── Pass → sign + broadcast
  ├── Block → reject + audit log
  └── PendingHITL → suspend + wait for /override
```

### Trust Signal Flow

```
ARES (Threat Intelligence)
  └── Trust Signals API (risk scores, sanctions, reputation)
         │
         ▼
Bastion (Policy Engine)
  └── TrustSignalProvider trait — queries, caches (5 min TTL), evaluates
```

### Web2 API Proxy

```
Agent → BastionWeb2Client → ProxyEngine::evaluate(ApiEvent)
  ├── Pass → forward to upstream API
  ├── Block → 403 + audit log
  └── PendingHITL → human approval gate
```

---

## 12. Component Architecture (Existing Codebase)

### crates/core — Chain-Agnostic Policy Engine

The shared foundation. Every chain normalizes its native transactions into `NormalizedTransaction` for evaluation against `PolicySet`.

| Type | Purpose |
|------|---------|
| `NormalizedTransaction` | Chain-agnostic tx (agent_id, from, to, amount, currency, tx_type, chain, metadata) |
| `FirewallDecision` | Enum: Pass, Block, PendingHITL |
| `PolicyRule` | 11 variants: AmountLimit, Destination, Frequency, HITL, Reputation, TxTypeAllowlist, StakeWeighted, Geofence, SpeedLimit, EnergyBudget, OperatingHours |
| `PolicySet` | Ordered, composable rule collection |
| `PolicyEvaluator<P: TrustSignalProvider>` | Core evaluation loop with optional trust signal provider |
| `TrustSignalProvider` | Trait for consuming trust signals (ARES-owned; Bastion queries) |
| `TrustAdapter` | Trait for chain-independent execution environments |
| `Chain` | Enum: Solana, Base, Ethereum, Polygon, Arbitrum, Celo, ZkSync, Robinhood |
| `AuditRecord` | Chain-agnostic audit event structure |

### crates/sidecar — Off-Chain Evaluator (Axum HTTP)

HTTP server (port 3000) that runs the policy evaluator. Bridges non-Rust chains to the Rust policy engine.

**Key endpoints:** `/simulate`, `/api/v2/simulate-evm`, `/api/v2/evaluate`, `/events` (SSE), `/agents`, `/policy`, `/circuit-breaker`, `/override`, `/did/resolve`, `/health`.

### crates/web2-firewall — Web2 API Proxy

Proxies AI agent HTTP calls through policy evaluation before forwarding to target providers. Provider adapters for OpenAI, Stripe, Slack, GitHub. OpenAPI spec-based auto-configuration.

### crates/solana — Anchor On-Chain Program

Solana devnet program (`A29V5MUVs73y7XBHHxPpPcAW7h4gGHupbDdwYSwA2n9D`). Provides `AuditState`, `AuditEntry`, `Agent`, `Policy` accounts. Instructions: `initialize`, `logAudit`, `registerAgent`, `updateAgentReputation`, `setPolicy`, `emergencyPause`, `emergencyResume`.

### evm/ — Solidity Contracts (Foundry)

Six contracts targeting 8 EVM chains. ~62 Foundry tests.

| Contract | Standard | Purpose |
|----------|----------|---------|
| `BastionFirewall` | ERC-7579 | Validator module — gates UserOps through policy |
| `BastionPolicy` | — | Per-agent rules (allowlists, limits, cooldowns) |
| `BastionAudit` | EIP-712 | Immutable audit trail with signed entries |
| `BastionRegistry` | — | Agent + target directory |
| `BastionERC8004Registry` | ERC-8004 | Soulbound agent identity |
| `BastionSidecar` | — | Oracle request/fulfill pattern |

---

## 13. Agent Delegation System

Hierarchical: parent → sub-agent → sub-sub-agent (max depth 3).

- `POST /agents` — register root agent
- `POST /agents/:did/delegate` — spawn sub-agent (validates: parent exists, depth < 3, capabilities ⊆ parent)
- `GET /agents/:did/tree` — full delegation tree

**Policy constraints:** max 3 levels, capability inheritance, budget enforcement (`delegation_spent ≤ delegation_budget`), expiry timestamps.

---

## 14. Technology Stack

| Component | Technology | Version |
|-----------|-----------|---------|
| Rust Sidecar | Rust (edition 2024), Axum, Tokio, Sled | 1.85+ |
| Rust Core | serde, thiserror, uuid, async-trait | 0.1.0 |
| Rust Web2 Firewall | bastion-web2-firewall, http, url, reqwest | 0.1.0 |
| Solana On-Chain | Anchor, solana-program, borsh | 0.30.1 / 1.18 / 1 |
| EVM Contracts | Solidity, Foundry, OpenZeppelin, Solady | 0.8.28 |
| Arcium MXE | Arcis (Rust MPC circuits) | mainnet-alpha |
| Dashboard | React, Vite, TailwindCSS, wagmi, RainbowKit | 18 / 5 / 3.4 / 2.12 / 2.2 |
| SDK | TypeScript, Anchor, @solana/web3.js | 5 / 0.30.1 / 1.91 |
| MCP Server | TypeScript, @modelcontextprotocol/sdk, SSE | — |
| CI/CD | GitHub Actions, Netlify, Vercel, Fly.io, Docker | — |
