---
name: ethereum-ecosystem
description: >-
  Ethiopia's standard-based agent trust infrastructure. Maps ERC-8004 (agent identity),
  ERC-7579 (modular wallets), ERC-4337 (account abstraction), ERC-8126 (agent verification),
  EIP-7702 (smart EOAs), x402 (HTTP payments), EAS (attestations), and Sign Protocol.
  Use when building on Ethereum trust standards.
---

# Ethereum Ecosystem Standards for Bastion

Bastion is the **Programmable Trust Runtime** — it orchestrates, not replaces, Ethereum's trust primitives. This skill maps the standards Bastion composes.

## ERC-8004 — Agent Identity Registry

**Deployed:** January 29, 2026. On 20+ chains. Same contract addresses everywhere.

- **IdentityRegistry:** `0x8004A169FB4a3325136EB29fA0ceB6D2e539a432`
- **ReputationRegistry:** `0x8004BAa17C55a88189AE136b182e5fdA19dE9b63`

Agent identity is an ERC-721 NFT. Registration URI points to JSON with service endpoints (A2A, MCP, x402, etc.). Reputation is multi-dimensional (uptime, quality, success rate) with signed fixed-point feedback. Validation registry supports crypto-economic, zkML, and TEE attestation models.

**Resource:** `https://ethskills.com/standards/SKILL.md` (ERC-8004 section) | https://www.8004.org

## ERC-4337 / ERC-7579 — Smart Accounts & Modular Wallets

ERC-4337 provides account abstraction (UserOperations, bundlers, paymasters). ERC-7579 defines modular smart account architecture — validators, executors, hooks. Bastion's policy validators plug as ERC-7579 modules.

## ERC-8126 — AI Agent Verification

Consume standardized verification results and risk scores during policy evaluation before allowing an agent to execute.

## EIP-7702 — Smart EOAs

Live since May 2025. EOAs get smart-contract capabilities without migration. Bastion can extend policy enforcement to EOA-based agents.

## x402 — HTTP Payment Protocol

Production-ready open standard from Coinbase. Uses HTTP 402 "Payment Required" for internet-native micropayments. Bastion's Web2 firewall proxies x402 calls; Pact Network insures them.

**SDKs:** `@x402/core @x402/evm @x402/fetch @x402/express` (TS) | `pip install x402` (Python)

**Resource:** https://www.x402.org | https://ethskills.com/standards/SKILL.md (x402 section)

## EAS — Ethereum Attestation Service

On-chain attestation registry. Bastion stores execution approvals, compliance attestations, human approvals, and policy outcomes as EAS attestations.

## Sign Protocol — Cross-Chain Attestations

Portable trust records verifiable across ecosystems. Bastion emits Sign attestations for cross-chain trust decisions.

## Pact Network — Payment Refunds

On-chain chargeback protocol for x402 payments. Solana mainnet Pinocchio program (`5bCJcdWdKLJ7arrMVMFh3z99rQDxV785fnD9XGcr3xwc`). Agents pay a premium; if the API fails, Pact refunds principal+premium from a USDC coverage pool.

## Key Ecosystem Skills

| Skill | Install | Purpose |
|-------|---------|---------|
| ETHSKILLS | `/plugin install ethskills@ethskills` | Full Ethereum development knowledge (Claude Code) |
| Base MCP | `npx skills add base/skills --skill base-mcp` | Wallet + x402 + 20+ DeFi plugins |
| Base Skills (full) | `npx skills add base/skills --skill build-on-base` | Complete Base development playbook |

## Resources

- https://ethskills.com/SKILL.md — all 19 Ethereum skills
- https://github.com/base/skills — Base agent skills
- https://www.8004.org — ERC-8004 agent identity
- https://www.x402.org — HTTP payment protocol
- https://eips.ethereum.org/EIPS/eip-8004 — ERC-8004 spec
- https://eips.ethereum.org/EIPS/eip-8126 — ERC-8126 spec
- https://pactnetwork.io/docs — Pact Network docs
