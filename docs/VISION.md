# Bastion — Vision

> This document describes the **north-star vision** for Bastion as a Programmable Trust Runtime.
> It is intentionally aspirational. For what ships today vs. what is planned, see the root
> [`README.md`](../README.md) status markers, [`ROADMAP.md`](ROADMAP.md), and the
> [`COMPETITIVE_LANDSCAPE.md`](COMPETITIVE_LANDSCAPE.md) for ecosystem positioning.

## The thesis

AI agents can reason, but production systems require trust. Autonomous workflows must survive
failures, prevent duplicate actions, enforce organizational policies, preserve privacy, and
provide verifiable audit trails.

Rather than rebuilding security, policy, and blockchain logic into every application, Bastion aims
to be a single runtime that orchestrates **identity, policy, privacy, durable execution, and
verifiable trust** across multiple execution environments.

Developers define **trust requirements**. Bastion determines **how those requirements are enforced**.

## The unified runtime

The target developer surface is a single facade:

```typescript
await bastion.execute({
  action: "swap",
  policy: "default",
  privacy: "public",
  settlement: "ethereum"
});
```

Instead of choosing infrastructure, developers declare the desired trust guarantees. Behind the
facade, Bastion coordinates specialized runtimes:

- **Identity Runtime** — agent identity, delegation, and credentials via ERC-8004, ERC-8126, DID.
- **Policy Runtime** — rules, approvals, limits, and governance powered by OPA and runtime rules.
- **Wallet Runtime** — composable smart accounts via ERC-4337, EIP-7702, ERC-7579.
- **Evidence Runtime** — verifiable attestations via EAS, Sign Protocol, zkTLS/Reclaim.
- **Durable Workflow Engine** — execution that survives failures and prevents duplicate actions.
- **Privacy Runtime** — confidential computation via Arcium MXE.
- **Trust Ledger** — verifiable, auditable records of every action.
- **Execution Planner** — multi-chain routing (Solana, EVM, Arcium, Midnight).
- **Settlement Router** — anchoring and settlement on Ethereum, with payment guarantees via Pact Network.

## What Bastion owns vs. what it composes

Bastion orchestrates existing Ethereum standards — it does not compete with them.

| Standard | Purpose | How Bastion Uses It |
|----------|---------|-------------------|
| **ERC-4337** | Smart Account / Account Abstraction | Execute programmable agent wallets, sponsored transactions, recovery, and spending policies |
| **EIP-7702** | Temporary smart account for EOAs | Support users who want programmable behavior without permanently migrating |
| **ERC-7579** | Modular smart account architecture | Plug Bastion's policy validator, firewall, or execution modules into compatible smart accounts |
| **ERC-8004** | AI agent identity, discovery, and reputation | Register agents, discover capabilities, retrieve reputation before execution |
| **ERC-8126** | AI Agent Verification | Consume standardized verification results and risk scores during policy evaluation |
| **EAS** | Ethereum Attestation Service | Store execution approvals, compliance attestations, human approvals, policy outcomes |
| **Sign Protocol** | Cross-chain attestations | Emit portable trust records verifiable across ecosystems |
| **ERC-20 / ERC-721 / ERC-1155** | Assets | Apply runtime policy to token interactions |
| **x402** | Machine-native HTTP payments | Monetize APIs and enable autonomous agent-to-agent payments |
| **Pact Network** | On-chain payment refunds | Insure agent API calls with automatic refunds when upstreams fail |

## What Bastion should NOT build

Avoid reimplementing capabilities that already have strong ecosystem support:

- ❌ Another agent identity registry → use **ERC-8004**
- ❌ Another attestation protocol → use **EAS** or **Sign Protocol**
- ❌ Another smart account → build on **ERC-4337** / **ERC-7579**
- ❌ Another MPC network → integrate **Arcium**
- ❌ Another reputation protocol → consume **ERC-8004**
- ❌ Another ZK identity protocol → integrate existing solutions
- ❌ Another payment insurance protocol → integrate **Pact Network**

## Runtime architecture

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
├── Privado ID                              └── Sigil
└── DID / VC

Wallet                  Execution           Settlement
├── ERC-4337            ├── Solana          ├── Ethereum
├── EIP-7702            ├── Starknet (ZK)   ├── Pact Network
└── ERC-7579            ├── Arcium (MPC)    └── EigenLayer
                        └── Midnight (ZK)
```

## Multi-chain by design

Different execution environments serve different purposes: durable coordination (Solana),
ZK-verified execution with native account abstraction (Starknet),
confidential computation (Arcium), trust anchoring & settlement (Ethereum), privacy-preserving
execution (Midnight), and provenance & attestations (Sigil). Applications interact with
Bastion — not individual blockchains.

## The trust lifecycle

Every workflow follows the same pipeline — Identity → Policy Evaluation → Privacy Enforcement →
Execution Planning → Verification → Settlement — so that every action is deterministic, auditable,
and policy-compliant.

## Roadmap horizon

- Unified `execute()` runtime facade
- Durable workflow runtime
- Cross-chain orchestration and a real settlement router
- Confidential execution with Arcium (real MXE)
- ZK-verified execution on Starknet (native AA, STARK proofs, Cairo VM)
- Zero-knowledge policy enforcement
- Decentralized identity integration
- A trust marketplace for AI agents
- Pact Network integration — auto-insured outbound API calls

---

**Built by zkOS Labs** — advancing programmable trust infrastructure for Ethereum and
interoperable open networks.
