# Bastion Project Plan

Bastion is the Programmable Trust Runtime for autonomous systems. It sits between AI agents and their execution environments, enforcing programmable policy, running pre-execution simulation, gating actions behind human approval, and recording every decision as a verifiable audit trail.

## What Bastion Answers

```text
Agent Framework (LangGraph, CrewAI, ElizaOS, OpenAI Agents SDK)
        |
        v
   Bastion Trust Runtime
   Policy + Identity + HITL + Simulation + Audit
        |
        v
Ethereum | Monad | zkSync | Base | Celo | Arbitrum | Polygon | Robinhood | Solana
```

Bastion does not build agents. Bastion does not orchestrate multi-agent workflows. Bastion answers one question: **can this action execute safely under the trust policies in place?**

## Current State

The core is a multichain transaction firewall (EVM testnets + Solana). Agents submit intended actions, Bastion evaluates them against programmable policy (11 rule types) via the chain-agnostic `TrustAdapter` abstraction, simulates them against live chain state, applies human-in-the-loop review when required, and writes a verifiable audit record. The legacy Solana on-chain Anchor audit program and the Arcium MPC stub are retired in favor of EVM contracts and ERC-8354 respectively (see `docs/ARCHIVE.md`) — Solana *settlement* itself is active, via RPC-based simulation.

| Capability | Status |
|-----------|--------|
| Programmable policy engine (11 rules) | Shipped |
| Transaction simulation (EVM) | Shipped |
| Transaction simulation (Solana, RPC-based) | Shipped |
| Human-in-the-loop approvals | Shipped |
| Verifiable on-chain audit (EIP-712) | Shipped |
| ERC-8004 agent identity + delegation | Shipped |
| MCP server (15 tools, 3 prompts) | Shipped |
| Web2 API policy adapter | Shipped |
| TypeScript SDK | Shipped |
| Confidential ZK policy verdicts (ERC-8354, draft) | Integrated (experimental) |
| Unclonable agent credentials (ERC-8380, draft) | Integrated (experimental) |
| trustless-ai ERC stack (8004, 8263, 8281, 8299) | Integrated |
| Durable workflow engine | Planned |
| Cross-chain settlement router | Planned |

## License and Sustainability

Apache 2.0. Free to use, modify, self-host, and redistribute. No protocol fees, tokens, ICOs, or mandatory services. The hosted sidecar at bastion-agentique.fly.dev is provided as a convenience. A small number of compute-intensive operations have optional usage-based pricing via USDT/USDC after a generous free tier. Self-hosting Bastion retains full functionality without any platform fees. Community support through GitHub Sponsors.

## Architecture

```
crates/core/             Chain-agnostic policy engine, 11 rule types
crates/policy-engine/    Kyverno-style TrustPolicy YAML, lifecycle, scanner
crates/sidecar/          Axum HTTP server, REST API, simulation, audit
crates/web2-firewall/    Web2 API proxy with provider adapters
evm/                     Solidity contracts (Foundry, incl. ERC-8354/8380)
apps/web/                React dashboard (Vite, TailwindCSS)
packages/sdk/            TypeScript SDK (@zkos-labs/bastion-sdk)
packages/web2-sdk/       Web2 adapter SDK (@zkos-labs/web2-sdk)
packages/mcp-server/     MCP server (SSE transport)
fv/                      Formal verification (TLA+ + Certora + property tests)
```

## Formal Verification

Following Vitalik's focused verification thesis: prove the trust-critical components correct rather than auditing everything.

| Layer | Tool | What is Proven |
|-------|------|---------------|
| Policy engine | TLA+ | Rule ordering, completeness, determinism |
| EVM contracts | Certora | Append-only audit, firewall gate, policy enforcement |
| TrustPolicy mapping | Property tests | All 11 rule types preserved, lossless YAML to PolicyRule |

## Key Repositories and PRs

| Integration | PR |
|------------|-----|
| Policy ERC proposal to trustless-ai/agent-ercs | [#12](https://github.com/trustless-ai/agent-ercs/pull/12) |
| Bastion skill contributed to trustless-ai/agent-sdk | [#8](https://github.com/trustless-ai/agent-sdk/pull/8) |
| Bastion composed gate example to CAPV | [#1](https://github.com/zexoverz/confidential-agent-policy-verdicts/pull/1) |

## Next 4 Weeks

| Week | Focus |
|------|-------|
| 1 | Durable workflow engine. Multi-step agent actions with crash recovery and deterministic replay |
| 2 | Cross-chain settlement router. Decompose TrustIntents into chain-specific execution plans |
| 3 | Background scanner. Continuous trust scanning for expired approvals, policy drift, unsettled transactions |
| 4 | Deploy to EVM mainnet (Monad + existing chains). External security audit gating production deployment |

## Get Involved

Clone and build: `git clone --recurse-submodules https://github.com/zkos-labs/bastion && cd bastion && cargo build`

Run the sidecar: `cargo run -p bastion-sidecar`

Start the dashboard: `pnpm --filter bastion-dashboard dev`

Contribute: open an issue to discuss before submitting large PRs. See `docs/CONTRIBUTING.md`.
