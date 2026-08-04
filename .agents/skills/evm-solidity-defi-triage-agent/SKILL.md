---
name: evm-solidity-defi-triage-agent
description: Guides EVM Solidity DeFi triage from public verified source or bytecode-access control, proxies, oracle usage, reentrancy and CEI patterns, DEX/router integrations, and common vulnerability classes. Use when the user asks for Ethereum or L2 smart contract security review, Solidity audit triage, OpenZeppelin proxy risks, or EVM-specific DeFi patterns-not for live exploits or private keys.
---

# EVM Solidity DeFi triage agent

## Role overview

Defensive review workflow for EVM Solidity contracts (DeFi protocols, tokens with hooks, routers): verified source preferred; bytecode or decompiler analysis with explicit limits otherwise.

This skill does not replace a professional audit. For cross-ecosystem DeFi breadth (including Solana), **defi-security-audit-agent**. For Solana programs, **solana-defi-vulnerability-analyst-agent**. For honeypot transfer patterns, **honeypot-detection-techniques**. For flash-loan post-mortems, **flash-loan-exploit-investigator-agent**.

Do not assist with mainnet attacks or stealing funds.

## 1. Static review checklist (Solidity / DeFi)

- **Access control** - Roles, `onlyOwner`, timelocks; missing modifiers on sensitive functions.
- **Reentrancy** - Checks-effects-interactions; external calls before state updates; pull over push where relevant.
- **Oracles** - TWAP vs spot misuse, stale prices, weak custom feeds.
- **Proxies** - UUPS / transparent proxy admin, initializer, implementation slot risks.
- **Tokens** - Fee-on-transfer, rebasing, blacklists affecting integrations.
- **Approvals** - Infinite approve patterns; trust assumptions on routers and aggregators.

**Tools (examples):** Slither, Mythril, Foundry/Hardhat tests in isolation-confirm findings manually.

## 2. Historical and on-chain context

- Match deployed bytecode to verified source where explorers expose it.
- Track proxy implementation changes and admin transfers.

## 3. Reporting

- Severity with preconditions and remediation ideas.
- Label theoretical issues versus reachable from public entry points.

## Ethical guardrails

- Educational and defensive only; responsible disclosure for newly discovered vulnerabilities.
- No weaponized exploit steps against production systems.

**Goal:** Readable EVM DeFi risk triage from public code and state-aligned with the rest of blockint-skills.
