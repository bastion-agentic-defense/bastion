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
| Durable workflow execution | 🚧 |
| Confidential computation (Arcium) | 🚧 |
| Payment guarantees (Pact Network) | 🚧 |

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

See [`docs/COMPETITIVE_LANDSCAPE.md`](docs/COMPETITIVE_LANDSCAPE.md) for the full competitive analysis.

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
Trust Ledger                ✅  EAS, Sign Protocol
Execution Planner           🟡  Solana, EVM, Arcium
Settlement Router           🚧  Ethereum, Pact Network
─────────────────────────────────────────
    │
Solana · Arcium · Ethereum · Midnight · Pact Network
```

Bastion presents a single runtime while coordinating specialized infrastructure behind the scenes.

---

## Multi-Chain by Design

Different execution environments serve different purposes.

| Capability                    | Network  | Status |
| ----------------------------- | -------- | ------ |
| Durable workflow coordination | Solana   | ✅ on-chain audit program (devnet) |
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

See [`docs/ROADMAP.md`](docs/ROADMAP.md) for detail, [`docs/VISION.md`](docs/VISION.md) for the long-form vision, and [`docs/COMPETITIVE_LANDSCAPE.md`](docs/COMPETITIVE_LANDSCAPE.md) for the ecosystem analysis.

---

## Contributing

Contributions are welcome.

Please open an issue to discuss new features, architecture proposals, or bug reports before submitting large pull requests. See [`docs/CONTRIBUTING.md`](docs/CONTRIBUTING.md).

---

## License

Apache-2.0

---

**Built by zkOS Labs**

*Advancing programmable trust infrastructure for Ethereum and interoperable open networks.*
