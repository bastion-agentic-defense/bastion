# Bastion — Competitive Landscape

> Bastion's defensible position: **orchestration, not replacement.**
> The Ethereum ecosystem is building trust primitives. Bastion composes them into a unified runtime.

---

## The Landscape

### AI Agent Economy & Frameworks

| Project | Primary Category | Identity | Policy | Durable Execution | Provenance | Privacy | Multi-chain | AI Runtime |
|---------|-----------------|----------|--------|-------------------|------------|---------|-------------|------------|
| **Virtuals Protocol** | Agent economy / launchpad | ✅ | Limited | ❌ | Limited | ❌ | Base ecosystem | ✅ |
| **ElizaOS (ai16z)** | Agent framework | Limited | Limited | ❌ | ❌ | ❌ | Multi-chain | ✅ |

**Bastion's relationship:** Both are complementary. Virtuals creates agent marketplaces; ElizaOS provides agent frameworks. Bastion enforces trust and policy at runtime for agents built with either. Bastion is closer to Temporal + HashiCorp Vault + OPA + EigenLayer than to Virtuals.

### Cryptography & Key Management

| Project | Primary Category | Identity | Policy | Durable Execution | Provenance | Privacy | Multi-chain |
|---------|-----------------|----------|--------|-------------------|------------|---------|-------------|
| **Lit Protocol** | Threshold cryptography & key management | ✅ | Partial | ❌ | Partial | ✅ | ✅ |

**Bastion's relationship:** Bastion can consume Lit as one trust primitive rather than replacing it. Confidential execution policies can use Lit-managed keys.

### Shared Trust & Verifiability

| Project | Primary Category | Identity | Policy | Durable Execution | Provenance | Privacy |
|---------|-----------------|----------|--------|-------------------|------------|----------|
| **EigenLayer / EigenCloud** | Shared trust & verifiable services | ❌ | Partial | ❌ | Partial | ❌ |
| **EAS** | Attestation registry | Partial | ❌ | ❌ | ✅ | ❌ |
| **Sign Protocol** | Cross-chain attestations | Partial | ❌ | ❌ | ✅ | Partial |
| **Reclaim / zkPass / zkTLS** | Data verification | ❌ | ❌ | ❌ | ✅ | ✅ |

**Bastion's relationship:** Bastion orchestrates trust decisions that consume these primitives — it issues EAS/Sign attestations as part of execution, uses EigenLayer for cryptoeconomic trust, and feeds verified data from zkTLS into policy evaluation.

### Identity & Verification

| Project | Primary Category | Identity | Policy | Durable Execution | Provenance | Privacy | Multi-chain |
|---------|-----------------|----------|--------|-------------------|------------|---------|-------------|
| **Privado ID / Polygon ID** | Identity | ✅ | Partial | ❌ | Partial | ✅ | ✅ |

**Bastion's relationship:** Identity becomes one input into Bastion's policy runtime — agent identity is verified, then combined with policy, attestations, and execution decisions.

### Infrastructure Primitives

| Project | Primary Category | Identity | Policy | Durable Execution | Provenance | Privacy |
|---------|-----------------|----------|--------|-------------------|------------|----------|
| **OPA** | Policy engine | ❌ | ✅ | ❌ | ❌ | ❌ |
| **Temporal** | Durable workflow runtime | ❌ | ❌ | ✅ | ❌ | ❌ |
| **Pact Network** | Payment refund protocol | ❌ | ❌ | ❌ | ❌ | ❌ |

**Bastion's relationship:** Bastion embeds policy (like OPA) into autonomous workflows (like Temporal) with identity and provenance. Pact Network adds payment guarantees — Bastion's Web2 firewall can auto-wrap outbound x402 calls with Pact for insured agent API payments.

---

## The Gap: Nobody Combines These

Almost every project specializes in **one primitive**:
- Identity
- Attestations
- Privacy
- Workflow orchestration
- Shared security
- Agent frameworks
- Agent marketplaces
- Payment refunds

**None of them combine these into a unified runtime.**

That is the gap Bastion occupies: the orchestration layer that coordinates identity, policy, privacy, durable execution, provenance, and multi-chain settlement into a single programmable runtime.

---

## What Bastion Should NOT Build

Avoid reimplementing capabilities with strong ecosystem support:

- ❌ Another agent identity registry → use **ERC-8004**
- ❌ Another attestation protocol → use **EAS** or **Sign Protocol**
- ❌ Another smart account → build on **ERC-4337** / **ERC-7579**
- ❌ Another MPC network → integrate **Arcium**
- ❌ Another reputation protocol → consume **ERC-8004**
- ❌ Another ZK identity protocol → integrate existing solutions
- ❌ Another payment insurance protocol → integrate **Pact Network**

---

## What Bastion SHOULD Own

These are the pieces existing standards don't unify:

- **Durable Trust Runtime** — workflow orchestration with trust guarantees
- **Policy Runtime** — evaluating identity, verification, and governance before execution
- **Execution Planner** — choosing Solana, Arcium, Ethereum, etc. based on workload
- **Trust Lifecycle Management** — identity → policy → execution → provenance → settlement
- **Cross-standard orchestration** — making ERC-4337, ERC-8004, ERC-8126, EAS, Sign Protocol, and Pact Network work together seamlessly

---

## Integration: Pact Network

[Pact Network](https://pactnetwork.io/) provides on-chain chargebacks for x402 agent payments. When an AI agent pays an API and it fails, Pact refunds principal + premium from a coverage pool — automatically, on-chain.

### Technical Summary

| Dimension | Detail |
|-----------|--------|
| Chain | Solana mainnet-beta |
| Program ID | `5bCJcdWdKLJ7arrMVMFh3z99rQDxV785fnD9XGcr3xwc` |
| Framework | Pinocchio (manual account layout control) |
| Settlement token | USDC (SPL) |
| Capital model (v1) | Pact treasury-funded coverage pools |
| Capital model (v2) | Third-party underwriter LP deposits (12-25% target APY) |
| Classification | Off-chain, deterministic, auditable (success / client_error / server_error) |
| Batch settlement | `settle_batch` instruction, up to 50 calls per tx |
| Refund SLA | Agent receives principal + premium back on server_error |
| Phase | Private beta on mainnet |

### How Bastion Composes with Pact

```
Bastion Trust Runtime
Policy Runtime        ← OPA, Bastion rules
    │
Web2 Firewall         ← Bastion intercepts API calls
    │                    ┌─────────────────┐
    └── x402 payment ──▶│  Pact Network    │
                        │  (refund layer)  │
                        └─────────────────┘
    │
Trust / Settlement    ← Pact + EigenLayer
```

| Bastion | Pact | Relationship |
|---------|------|-------------|
| Gates whether to execute | Refunds after failure | Pre + post execution |
| Runs before execution | Runs after execution | Complementary lifecycle |
| SDK-level integration | `pact pay curl` wrapping | Bastion can auto-insure calls |
| Policy-level integration | Coverage required rules | Policy can mandate insurance |

---

## Ecosystem Architecture

```
                    Applications
        AI Agents · Enterprises · DAEMON

                          │

                   Bastion Runtime
────────────────────────────────────────────────────

Identity                Policy              Evidence
├── ERC-8004            ├── OPA             ├── EAS
├── ERC-8126            ├── Runtime Rules   ├── Sign Protocol
├── ERC-7579            ├── Human Approval  ├── zkTLS / Reclaim
├── Privado ID                              ├── Sigil
└── DID / VC

Wallet                  Execution           Settlement
├── ERC-4337            ├── Solana          ├── Ethereum
├── EIP-7702            ├── Arcium          ├── Pact Network
└── ERC-7579            └── Midnight        └── EigenLayer
```

---

**Built by zkOS Labs** — advancing programmable trust infrastructure for Ethereum and interoperable open networks.
