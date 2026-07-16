# Bastion Architecture

> **The Programmable Trust Runtime for Autonomous Systems.**
> Bastion orchestrates trust — it does not replace the protocols underneath.

---

## 1. Bastion's Proprietary Runtime

These are the components Bastion **owns**. Everything else is composable infrastructure.

```text
                    Applications
         AI Agents · Enterprises · DAEMON
                           │
────────────────────────────────────────────────────────
                   Bastion Runtime
────────────────────────────────────────────────────────
Policy Compiler    · Risk Engine      · Transaction Firewall
Trust Graph        · Evidence Aggregator · Audit Log
Agent Session Manager · Human Approval Engine
────────────────────────────────────────────────────────
```

| Component | Crate/File | What It Does |
|-----------|-----------|--------------|
| **Policy Compiler** | `crates/core/src/policy/evaluator.rs` | Normalizes chain-specific transactions, evaluates 11+ rule types against configured policy sets, returns Pass/Block/PendingHITL |
| **Risk Engine** | `crates/core/src/risk/` | Pluggable oracle abstraction (Webacy, GrondOSINT) for address/agent risk scoring. Feeds scores into policy decisions. |
| **Transaction Firewall** | `crates/sidecar/src/simulation.rs` + `simulation_evm.rs` | Pre-execution simulation (Helius for Solana, eth_call for EVM). Predicts balance changes before signing. |
| **Trust Graph** | `crates/sidecar/src/did.rs` + `AgentStore` | Hierarchical agent delegation tree (parent → sub → sub-sub). Capability inheritance, budget enforcement, expiry. |
| **Evidence Aggregator** | `crates/sidecar/src/audit.rs` | Collects every policy decision, simulation result, and approval action into the Sled DB audit log + optional on-chain record. |
| **Audit Log** | `crates/solana/` (Anchor) + `evm/` (EIP-712) | Immutable on-chain audit trail. Every Pass/Block/HITL decision is recorded with cryptographic provenance. |
| **Agent Session Manager** | `crates/sidecar/src/lib.rs` `/agents` routes | Agent lifecycle: register (DID), delegate, query tree, revoke. TrackedAgent with reputation, budget, expiry. |
| **Human Approval Engine** | `crates/sidecar/src/lib.rs` `/override` route | HITL gate: `PendingHITL` decisions pause execution until human approves/rejects via `/override`. Timeout-aware. |

---

## 2. Capability Architecture

Bastion's capabilities are organized into layers. Each layer has a **Bastion-owned component** (left) and **composable infrastructure** (right).

```text
────────────────────────────────────────────────────────────────────
                     Bastion Runtime
────────────────────────────────────────────────────────────────────
  Bastion Owns                           Composable Infrastructure
────────────────────────────────────────────────────────────────────

IDENTITY & TRUST                       AGENT INTEROPERABILITY
  Trust Graph                            A2A · MCP · ACP (future)
  Agent Session Manager

POLICY ENGINE                          POLICY BACKENDS
  Policy Compiler                        OPA · Cedar · Runtime Rules
  Risk Engine                            Human Approval Engine

WALLET & AUTHORIZATION                 WALLET INFRASTRUCTURE
  Transaction Firewall                   ERC-4337 · EIP-7702 · ERC-7579
                                         Session Keys · ERC-7715
                                         Safe · Coinbase Smart Wallet

EVIDENCE & PROVENANCE                  ATTESTATION PROTOCOLS
  Evidence Aggregator                    EAS · Sign Protocol · Sigil
  Audit Log                              zkTLS · Reclaim · zkPass
                                         The Graph

EXECUTION                              EXECUTION ENVIRONMENTS
  (Planned: Execution Planner)           Solana · Ethereum · Midnight
                                         Starknet (ZK + native AA)

CONFIDENTIAL COMPUTE                   PRIVACY INFRASTRUCTURE
  (Planned: Privacy Runtime)             Arcium MXE · SP1 · Lit
                                         Nillion · Aztec

SETTLEMENT & SECURITY                  SETTLEMENT LAYERS
  (Planned: Settlement Router)           Ethereum · Pact Network
                                         EigenLayer (cryptoeconomic security)

CONNECTIVITY                           BRIDGING
  Web2 Firewall                          Across · LayerZero
                                         Wormhole · Hyperlane

PAYMENTS                               PAYMENT RAILS
  x402 integration                       x402 · Pact Network
  Web2 Firewall budget rules             EIP-3009 · ERC-2612
```

---

## 3. Agent Interoperability

Agent-to-agent communication protocols that Bastion composes:

| Protocol | Purpose | Integration |
|----------|---------|-------------|
| **A2A** (Google) | Agent discovery and communication | ERC-8004 service registry maps A2A endpoints |
| **MCP** (Anthropic) | Model Context Protocol — LLM tool access | Bastion MCP server on port 3001, 15 tools + 3 prompts |
| **ACP** (future) | Agent Communication Protocol | Planned |

---

## 4. Evidence & Provenance

Bastion's biggest differentiator. Every trust decision is verifiable.

| Protocol | Provides | Bastion's Role |
|----------|---------|---------------|
| **EAS** | On-chain attestation registry | Stores execution approvals, compliance attestations, HITL outcomes |
| **Sign Protocol** | Cross-chain attestations | Emits portable trust records verifiable across ecosystems |
| **Sigil** | On-chain provenance | Verifiable records of agent actions and decisions |
| **zkTLS / Reclaim / zkPass** | Verified web data proofs | Policy inputs: "this agent has verified GitHub org membership" |
| **The Graph** | Blockchain indexing | Fast ERC-8004 agent discovery via subgraph queries |

---

## 5. Confidential Compute (Separate from Execution)

Confidential compute is about **how** computation happens (private). Execution is about **where** it happens (which chain).

| Protocol | Model | Chain | Bastion's Role |
|----------|-------|-------|---------------|
| **Arcium MXE** | MPC (dishonest majority) | Solana | Confidential policy evaluation — private thresholds, private allowlists |
| **SP1** | ZKVM (Succinct) | Ethereum | General-purpose ZK proof generation and verification |
| **Lit Protocol** | Threshold cryptography | Multichain | Key management + decryption for confidential execution policies |
| **Nillion** | MPC + blind compute | Multichain | Privacy-preserving computation for sensitive data |
| **Aztec** | ZK-rollup with private state | Ethereum L2 | Private smart contract execution on Ethereum |

---

## 6. Execution Environments

Where Bastion executes agent workflows, each chosen for its properties.

| Network | Properties | Bastion's Role |
|---------|-----------|---------------|
| **Solana** | High throughput, low cost | Durable workflow coordination, on-chain audit program |
| **Starknet** | ZK-verified, native AA, L1-L2 messaging | Provably correct execution with smart-account agent wallets by default |
| **Ethereum** | Highest security, L1 finality | Trust anchoring and settlement |
| **Midnight** | ZK privacy, selective disclosure | Privacy-preserving execution for sensitive workflows |

---

## 7. Settlement & Security

| Protocol | Category | Bastion's Role |
|----------|---------|---------------|
| **Ethereum** | Settlement | Final trust anchoring. Contracts deployed, testnet-only |
| **Pact Network** | Settlement | On-chain x402 refund guarantees for agent API calls |
| **EigenLayer** | Cryptoeconomic Security | Operator accountability via AVS slashing/rewards (planned) |

---

## 8. Trust Services

Pluggable infrastructure organized by function:

### Attestation
- **EAS** — on-chain attestations
- **Sign Protocol** — cross-chain attestations

### Identity
- **ERC-8004** — agent identity + reputation (20+ chains)
- **ERC-8126** — agent verification
- **Privado ID / Polygon ID** — privacy-preserving identity
- **ENS** — human-readable names
- **DID / VC** — W3C decentralized identifiers

### Oracles
- **Chainlink** — price feeds, VRF, Automation, CCIP
- **Pyth** — low-latency price feeds (Solana + EVM)
- **Chronicle** — MakerDAO-backed verifiable oracles

### Randomness
- **Chainlink VRF** — provably random, verifiable on-chain

### Restaking
- **EigenLayer** — cryptoeconomic security for AVS operators

### Key Management
- **Lit Protocol** — threshold cryptography, programmable key management

---

## 9. Wallet Infrastructure

| Standard | Provides | Bastion's Role |
|----------|---------|---------------|
| **ERC-4337** | Account abstraction — UserOps, bundlers, paymasters | Policy-aware execution, multi-chain routing |
| **EIP-7702** | Smart EOAs (live May 2025) — delegation without migration | Policy enforcement for EOA-based agents |
| **ERC-7579** | Modular smart accounts — validators, executors, hooks | Bastion's policy validator as a pluggable module |
| **Session Keys** | Temporary, scoped keys for agent sessions | Limit blast radius of compromised agent keys |
| **ERC-7715** (future) | Standardized session key grants | When mature, standardized session key management |
| **Safe** | Multisig (2-of-3 for agents: agent + human hot + human cold) | Recovery and batching for high-value operations |
| **Coinbase Smart Wallet** | Embedded wallet with passkey auth | Used by Base MCP for agent wallets |

---

## 10. Connectivity & Bridging

| Protocol | Model | Purpose |
|----------|-------|---------|
| **Across** | Intents-based, optimistic oracle | Fast cross-chain transfers |
| **LayerZero** | Omnichain messaging (80+ chains) | Cross-chain agent actions |
| **Wormhole** | Generic message passing + token bridging | Solana-native, now multichain |
| **Hyperlane** | Permissionless interop | Deploy to new chains without governance |

---

## 11. Complete Runtime Architecture

```text
                         Applications
              AI Agents · Enterprises · DAEMON
                                │
    ────────────────────────────────────────────────────────────
                         Bastion Runtime
    ────────────────────────────────────────────────────────────
    Policy Compiler  · Risk Engine      · Transaction Firewall
    Trust Graph      · Evidence Aggregator · Audit Log
    Agent Session Manager · Human Approval Engine
    ────────────────────────────────────────────────────────────

    Identity & Trust              Agent Interoperability
    ├── ERC-8004 · ERC-8126      ├── A2A
    ├── ENS · DID/VC             ├── MCP
    └── Privado ID               └── ACP (future)

    Policy Backends               Wallet Infrastructure
    ├── OPA · Cedar              ├── ERC-4337 · EIP-7702
    ├── Runtime Rules            ├── ERC-7579 · ERC-7715
    └── Human Approval           ├── Session Keys · Safe
                                 └── Coinbase Smart Wallet

    Evidence & Provenance         Confidential Compute
    ├── EAS · Sign Protocol      ├── Arcium MXE (MPC)
    ├── Sigil · zkTLS            ├── SP1 (ZKVM)
    └── Reclaim · zkPass         ├── Lit (Threshold Crypto)
                                 ├── Nillion (Blind Compute)
    Trust Services               └── Aztec (ZK Private State)
    ├── Attestation: EAS · Sign
    ├── Identity: ERC-8004 · ENS  Execution
    ├── Oracles: Chainlink · Pyth ├── Solana
    │   · Chronicle              ├── Starknet (ZK + native AA)
    ├── Randomness: VRF          ├── Ethereum
    └── Restaking: EigenLayer    └── Midnight (ZK privacy)

    Settlement & Security         Connectivity & Bridging
    ├── Ethereum                 ├── Across
    ├── Pact Network             ├── LayerZero
    └── EigenLayer               ├── Wormhole
                                 └── Hyperlane

    Payments
    ├── x402
    ├── Pact Network
    └── EIP-3009 · ERC-2612
    ────────────────────────────────────────────────────────────
```

---

## 12. Component Architecture (Existing Codebase)

### crates/core — Chain-Agnostic Policy Engine

The shared foundation. Every chain-specific adapter normalizes its native transaction format into `NormalizedTransaction` and passes it through `PolicyEvaluator`. Returns `FirewallDecision`: `Pass`, `Block { reason, policy_id }`, or `PendingHITL { approval_id, reason }`.

| Type | Purpose |
|------|---------|
| `NormalizedTransaction` | Chain-agnostic tx representation (agent_id, from, to, amount, currency, tx_type, chain, metadata) |
| `FirewallDecision` | Enum: Pass, Block, PendingHITL |
| `PolicyRule` | 11 enum variants: AmountLimit, Destination, Frequency, HITL, Reputation, TxTypeAllowlist, StakeWeighted, Geofence, SpeedLimit, EnergyBudget, OperatingHours |
| `PolicySet` | Ordered, composable rule collection |
| `PolicyEvaluator<O: RiskOracle>` | Core evaluation loop with optional risk oracle |
| `RiskOracle` | Trait for address risk scoring (Webacy, GrondOSINT) |
| `AuditRecord` | Chain-agnostic audit event structure |

### crates/sidecar — Off-Chain Evaluator (Axum HTTP)

Bridges non-Rust chain implementations to the Rust policy engine. Also hosts Web2 proxy endpoints, MCP reverse proxy, agent registry, case management, DID resolution, and robot telemetry.

**Key endpoints:** `/simulate`, `/api/v2/simulate-evm`, `/api/v2/evaluate`, `/events` (SSE), `/agents`, `/policy`, `/circuit-breaker`, `/cases`, `/ingest`, `/did/resolve`, `/robots/:did/telemetry`.

### crates/web2-firewall — Web2 API Proxy

Proxies AI agent HTTP calls through policy evaluation before forwarding to target providers. Provider adapters for OpenAI, Stripe, Slack, GitHub. OpenAPI spec-based auto-configuration.

### crates/correlation — SIEM Correlation Engine

Sliding time window event correlation. Matches SecurityEvent sequences against YAML-defined rules. Integrates with GrondOSINT for threat enrichment and MITRE ATT&CK mapping.

### crates/solana — Anchor On-Chain Program

Solana devnet program (`A29V5MUVs73y7XBHHxPpPcAW7h4gGHupbDdwYSwA2n9D`). Provides `AuditState`, `AuditEntry`, `Agent`, `Policy` accounts. Instructions: `initialize`, `logAudit`, `registerAgent`, `updateAgentReputation`, `setPolicy`, `emergencyPause`, `emergencyResume`.

### evm/ — Solidity Contracts (Foundry)

Six contracts: `BastionFirewall` (ERC-7579 validator), `BastionPolicy` (per-agent rules), `BastionAudit` (EIP-712 audit), `BastionRegistry` (agent directory), `BastionERC8004Registry` (ERC-8004 identity), `BastionSidecar` (oracle pattern). ~54 Foundry tests.

---

## 13. Data Flow

### Transaction Evaluation

```
Agent → Chain adapter → normalize → PolicyEvaluator::evaluate()
  ├── Pass → sign + broadcast
  ├── Block → reject + audit log
  └── PendingHITL → suspend + wait for /override
```

### Web2 API Proxy

```
Agent → BastionWeb2Client → ProxyEngine::evaluate(ApiEvent)
  ├── Pass → forward to upstream API
  ├── Block → 403 + audit log
  └── PendingHITL → human approval gate
```

---

## 14. Agent Delegation System

Hierarchical: parent → sub-agent → sub-sub-agent (max depth 3).

- `POST /agents` — register root agent
- `POST /agents/:did/delegate` — spawn sub-agent (validates: parent exists, depth < 3, capabilities ⊆ parent)
- `GET /agents/:did/tree` — full delegation tree

**Policy constraints:** max 3 levels, capability inheritance, budget enforcement (`delegation_spent ≤ delegation_budget`), expiry timestamps.

---

## 15. Technology Stack

| Component | Technology | Version |
|-----------|-----------|---------|
| Rust Sidecar | Rust (edition 2024), Axum, Tokio, Sled | 1.85+ |
| Rust Core | serde, thiserror, uuid, async-trait | 0.1.0 |
| Rust Web2 Firewall | bastion-web2-firewall, http, url, reqwest | 0.1.0 |
| Rust Correlation | bastion-correlation | — |
| Solana On-Chain | Anchor, solana-program, borsh | 0.30.1 / 1.18 / 1 |
| EVM Contracts | Solidity, Foundry, OpenZeppelin, Solady | 0.8.28 |
| Arcium MXE | Arcis (Rust MPC circuits) | mainnet-alpha |
| Dashboard | React, Vite, TailwindCSS, TypeScript | 18 / 5 / 3.4 / 5 |
| SDK | TypeScript, Anchor, @solana/web3.js | 5 / 0.30.1 / 1.91 |
| MCP Server | TypeScript, @modelcontextprotocol/sdk, SSE | — |
| CI/CD | GitHub Actions, Netlify, Vercel, Fly.io, Docker | — |
