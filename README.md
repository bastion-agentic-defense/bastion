# Bastion

> **The Programmable Trust Runtime for Autonomous Systems.**

[![npm](https://img.shields.io/npm/v/@zkos-labs/sdk?label=sdk)](https://www.npmjs.com/package/@zkos-labs/sdk)
[![npm](https://img.shields.io/npm/v/@zkos-labs/web2-sdk?label=web2-sdk)](https://www.npmjs.com/package/@zkos-labs/web2-sdk)

> ⚠️ **Alpha Software** — APIs and runtime behavior may change before the first stable release.
> This document marks what ships today vs. what is planned. **No component is on mainnet with
> real value:** production deployment is gated behind an external security audit — see
> [`docs/MAINNET_READINESS.md`](docs/MAINNET_READINESS.md) and [`docs/EVM_READINESS.md`](docs/EVM_READINESS.md).

**Status legend:** ✅ shipped & tested · 🟡 partial / not fully wired · 🚧 planned / stubbed

For endpoint-level technical documentation, see [`docs/OVERVIEW.md`](docs/OVERVIEW.md).

---

## Overview

Bastion is a **Programmable Trust Runtime** for AI agents, autonomous systems, and enterprise applications.

Rather than building security, policy, and blockchain logic into every application, Bastion provides a unified runtime that orchestrates identity, policy, privacy, durable execution, and verifiable trust across multiple execution environments.

Developers define **trust requirements**.

Bastion determines **how those requirements are enforced**.

Today, the shipped core is a **transaction firewall**: agents submit intended actions, Bastion
evaluates them against programmable policy, simulates them, applies human-in-the-loop review when
required, and writes a verifiable audit record. The broader runtime (durable workflows,
confidential compute, cross-chain settlement) is under active development — see the roadmap below.

---

## Why Bastion?

AI agents can reason.

Production systems require trust.

Autonomous workflows must survive failures, prevent duplicate actions, enforce organizational policies, preserve privacy, and provide verifiable audit trails.

Bastion provides the runtime that makes autonomous systems safe to deploy in production.

---

## Core Capabilities

| Capability | Status |
| --- | --- |
| Programmable policy engine | ✅ |
| Transaction simulation & verification | ✅ |
| Human-in-the-loop approvals | ✅ |
| Verifiable trust ledger | ✅ |
| TypeScript SDK | ✅ |
| AI agent identity & delegation | 🟡 |
| Multi-chain execution planning | 🟡 |
| Web2 API policy gateway | 🟡 |
| MCP server | 🟡 |
| Dashboard & monitoring | 🟡 |
| Durable workflow execution | 🚧 design phase — see [Epic A](docs/IMPLEMENTATION_PLAN.md#phase-1-durable-workflow-engine--epic-a-1700-loc-2-3-weeks) |
| Confidential computation (Arcium) | 🚧 stubbed — noop client, behind audit gate |
| Payment guarantees (Pact Network) | 🚧 planned — see [Epic D](docs/IMPLEMENTATION_PLAN.md#phase-5-pact-network--epic-d-300-loc-3-5-days) |
| Secrets management (Vault) | 🚧 planned — see [Epic E](docs/IMPLEMENTATION_PLAN.md#phase-3-secrets-management--epic-e-500-loc-1-week) |
| General-purpose policy (OPA) | 🚧 planned — see [Epic F](docs/IMPLEMENTATION_PLAN.md#phase-2-general-purpose-policy--epic-f-600-loc-1-week) |

---

## How Bastion Composes Existing Standards

Bastion orchestrates — it does not replace — the Ethereum ecosystem's trust primitives.

| Standard | Provides | Bastion Adds |
|----------|---------|-------------|
| ERC-4337 | Smart accounts | Policy-aware execution, recovery, multi-chain routing |
| ERC-7579 | Wallet modules | Trust modules, policy validators, execution planning |
| ERC-8004 | Agent identity | Runtime authorization and cross-standard orchestration |
| ERC-8126 | Agent verification | Automatic policy decisions from verification results |
| EAS | Attestations | Lifecycle orchestration and execution evidence |
| Sign Protocol | Cross-chain attestations | Runtime-generated portable trust records |
| Lit Protocol | Key management | Confidential execution policies |
| EigenLayer | Shared trust | Runtime coordination using cryptoeconomic trust |
| Pact Network | Payment refunds | Auto-insured outbound API calls for agent payments |
| trustless-ai / agent-ercs | ERC-8004, ERC-8263, ERC-8281, ERC-8299 | Standard agent identity, anchor proofs, OCP/WYRIWE provenance — recompute-able |
| CAPV (ZK verdicts) | Confidential agent policy verdicts | ZK proofs that a committed (secret) policy allows an action — Noir + UltraHonk |

See [`docs/COMPETITIVE_LANDSCAPE.md`](docs/COMPETITIVE_LANDSCAPE.md) for the full competitive analysis.

Bastion composes with the [trustless-ai](https://github.com/trustless-ai) ERC stack and [confidential-agent-policy-verdicts](https://github.com/zexoverz/confidential-agent-policy-verdicts) for the full confidentiality spectrum: secret organizational policies (ZK) layered with transparent programmable trust rules (Bastion). See [`docs/TRUSTLESS_AI_INTEGRATION.md`](docs/TRUSTLESS_AI_INTEGRATION.md).

---

## Architecture

```text
Applications
    │
AI Agents · DAEMON · Enterprise Systems
    │
──────────── Bastion Runtime ────────────
Identity Runtime            🟡  ERC-8004, ERC-8126, DID/VC
Policy Runtime              ✅  OPA + Runtime Rules
Wallet Runtime              🚧  ERC-4337, EIP-7702, ERC-7579
Durable Workflow Engine     🚧
Privacy Runtime             🚧  Arcium MXE
ZK-Verified Execution       🚧  Starknet (STARK proofs, native AA)
Trust Ledger                ✅  EAS, Sign Protocol
Execution Planner           🟡  Solana, EVM, Arcium, Starknet
Settlement Router           🚧  Ethereum, Pact Network
─────────────────────────────────────────
    │
Solana · Starknet · Arcium · Ethereum · Midnight · Pact Network
```

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the complete capability architecture with Bastion's proprietary runtime layer, composable infrastructure mapping, and data flow diagrams.

---

## Multi-Chain by Design

Different execution environments serve different purposes.

| Capability                    | Network  | Status |
| ----------------------------- | -------- | ------ |
| Durable workflow coordination | Solana   | ✅ on-chain audit program (devnet) |
| ZK-verified execution (native AA) | Starknet | 🚧 planned — Ethereum ZK-rollup, STARK proofs, Cairo VM, every account is a smart account |
| Confidential computation      | Arcium   | 🚧 integration stubbed (no-op MPC client today) |
| Trust anchoring & settlement  | Ethereum | 🟡 per-chain sim wired (`settlement:"ethereum"`); contracts written & tested; testnet-only, mainnet 🚧 behind audit gate |
| Privacy-preserving execution  | Midnight | 🚧 planned |
| Provenance & attestations     | Sigil    | 🚧 planned |
| Payment guarantees            | Pact Network | 🚧 planned — on-chain refunds for x402 agent payments |

Applications interact with Bastion—not individual blockchains.

---

## Trust Lifecycle

Every workflow follows the same trust pipeline:

```text
Identity
    ↓
Policy Evaluation
    ↓
Privacy Enforcement
    ↓
Execution Planning
    ↓
Verification
    ↓
Settlement
```

This ensures every action is deterministic, auditable, and policy-compliant.

---

## Features

| Component            | Purpose                                 | Status |
| -------------------- | --------------------------------------- | ------ |
| Policy Runtime       | Rules, approvals, limits, governance    | ✅ |
| Transaction Firewall | Transaction validation and simulation   | ✅ |
| Trust Ledger         | Verifiable audit records                | ✅ |
| Identity Runtime     | Agent identity, delegation, credentials | 🟡 |
| Execution Planner    | Multi-chain routing                     | 🟡 |
| Web2 Gateway         | Secure API mediation                    | 🟡 |
| MCP Server           | Native AI agent integration             | 🟡 |
| Dashboard            | Monitoring and policy management        | 🟡 |

---

## Getting Started

```bash
git clone https://github.com/zkos-labs/bastion.git
cd bastion

# Build the Rust workspace
cargo build --release

# Run the Bastion sidecar (the trust runtime HTTP service)
cargo run --release -p bastion-sidecar
```

Dashboard:

```bash
pnpm --filter bastion-dashboard dev
```

SDK:

```bash
npm install @zkos-labs/sdk
```

---

## Example

**Available today** — simulate and policy-check an agent transaction through the firewall, and
register an agent identity:

```typescript
import { BastionClient, BastionSidecar } from "@zkos-labs/sdk";

const sidecar = new BastionSidecar({ baseUrl: "https://bastion-agentique.fly.dev" });

// Simulate + policy-check an agent transaction (returns Pass / Block / PendingHITL)
const decision = await sidecar.simulate({ transaction, intent: "swap" });

// Register an agent identity on-chain
const client = new BastionClient({ connection });
const tx = await client.registerAgent({ /* ... */ });
```

**Unified runtime facade (🟡 shipped in the SDK, thin composition)** — a single `execute()` call
where developers declare trust guarantees instead of choosing infrastructure:

```typescript
import { Bastion, BastionSidecar, BastionClient } from "@zkos-labs/sdk";

const bastion = new Bastion({ sidecar, client });

const result = await bastion.execute({
  action: "swap",
  privacy: "public",       // "confidential" is refused unless real Arcium MPC is active
  settlement: "ethereum",  // "solana" | "ethereum" | "base" | "celo"
  transaction,             // base64 Solana tx, or EVM tx params
});
// result.decision → "pass" | "block" | "pending_hitl"
```

`execute()` composes the existing firewall primitives (policy evaluation, per-chain simulation,
audit) behind one call — it adds no new backend. The cross-chain settlement **router/planner** is
still minimal (chain selection + simulation); true execution planning remains 🚧.

---

## Roadmap

* ✅ Programmable policy engine + transaction firewall (Solana + EVM simulation)
* ✅ Verifiable trust ledger (on-chain audit program, devnet)
* 🟡 Agent identity, delegation & multi-chain planning
* 🟡 Web2 API policy gateway (rate / budget / cost / time rules enforced, wired into the sidecar) + MCP integration
* 🟡 Unified `execute()` runtime facade (shipped in the SDK; settlement planner still minimal)
* 🚧 Durable workflow runtime
* 🚧 Confidential execution with Arcium (real MXE, replacing the no-op client)
* 🚧 Ethereum trust anchoring & settlement router (post-audit)
* 🚧 Zero-knowledge policy enforcement · decentralized identity · trust marketplace

See [`docs/ROADMAP.md`](docs/ROADMAP.md) for detail, [`docs/IMPLEMENTATION_PLAN.md`](docs/IMPLEMENTATION_PLAN.md) for the complete implementation plan (~5,780 LOC across 7 epics), [`docs/VISION.md`](docs/VISION.md) for the long-form vision, and [`docs/COMPETITIVE_LANDSCAPE.md`](docs/COMPETITIVE_LANDSCAPE.md) for the ecosystem analysis.

---

## Contributing

Contributions are welcome.

Please open an issue to discuss new features, architecture proposals, or bug reports before submitting large pull requests. See [`docs/CONTRIBUTING.md`](docs/CONTRIBUTING.md).

---

## License

Apache-2.0

---

**Built by zkOS Labs**

**Bastion is the programmable Trust Runtime for the agentic internet. It provides the identity, policy, execution, and observability layer that autonomous systems rely on to act safely across programmable networks.**

While agent frameworks determine **what** autonomous systems do, Bastion determines **how** those actions are executed safely, verifiably, and under programmable trust policies.

Bastion is foundational trust infrastructure for agentic applications.
