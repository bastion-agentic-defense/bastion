---
name: honeypot-detection-techniques
description: Educational techniques to assess honeypot-style token risk from verified source, bytecode clues, and observational on-chain history-EVM ERC-20 patterns (transfer gates, fees, blacklists), Solana SPL and Token-2022 hooks, and safe validation paths. Use when the user asks how to detect honeypots, sell-restricted tokens, scam token mechanics, or static review checklists-not for deploying scams, stealing funds, or advising high-risk mainnet test trades on unknown contracts.
---

# Honeypot detection techniques

## Role overview

A **honeypot** (colloquial) usually means a token or pool setup where buys appear to work but sells or transfers are blocked, taxed to an effective 100%, or gated so most users cannot exit-often implemented in custom ERC-20 logic on EVM or via mint, freeze, or Token-2022 extensions on Solana.

This skill is defensive and educational: static review, read-only simulation on forks or test networks, and observational metrics from public transaction history. It does not replace a professional audit.

For broad DeFi triage, **defi-security-audit-agent**. For EVM token or pool contracts (Solidity), **evm-solidity-defi-triage-agent** complements static honeypot checks. For launch-phase rug heuristics, **rug-pull-pattern-detection-agent**. For Solana program-centric review, **solana-defi-vulnerability-analyst-agent**.

Do not assist with building honeypots, evading detection for malicious launches, or deceiving users. Do not recommend using large approvals or swaps on unknown mainnet contracts to “test” sellability-funds can be lost.

## 1. What to verify first

- **Source** - Verified source on explorers when available; otherwise decompilation with explicit uncertainty.
- **Proxies** - Implementation can change; check upgrade authority and history.
- **Scope** - Restriction may live in the pool, router wrapper, or bonding-curve program-not only the token contract or mint.

## 2. EVM (Solidity / ERC-20) - common patterns

| Category | Examples to look for in code |
|----------|------------------------------|
| **Transfer gates** | `require` on `transfer` / `transferFrom` that fails for non-whitelisted or non-owner addresses |
| **Trading windows** | `tradingEnabled`, launch block, cooldowns that block sells early or for most holders |
| **Fees** | Excessive sell tax or dynamic tax to owner; max wallet or tx limits that prevent exiting |
| **Blacklists** | Lists blocking transfers from arbitrary addresses |
| **Approval traps** | Logic that burns or steals on `transferFrom` |
| **Hidden paths** | External calls in `_transfer` tied to router-only allowlists |

Automated scanners and Slither-style tools can help; false positives and negatives are common-manually read `_transfer` and any hooks.

## 3. Solana (SPL / Token-2022)

- **Authorities** - Mint and freeze authority; frozen ATAs cannot move funds.
- **Token-2022** - Transfer hooks, permanent delegate, pausable extensions-inspect on-chain mint layout and IDL when available.
- **Custom programs** - Launchpads and bonding curves wrap SPL transfers; review that program, not only the mint account.

## 4. Observational checks (no private keys)

- **Transaction history** - Ratio of successful sells versus buys from retail-like wallets (heuristic; wash trading exists).
- **Holders** - Extreme concentration plus few organic sell transactions may warrant suspicion-not proof.

## 5. Safe validation paths

- **Fork simulation** - Replay or simulate transfer and swap paths against historical state in a controlled environment; label outputs as simulation.
- **Testnet** - Only when the user controls deployment and understands limitations; mainnet can diverge after proxy upgrades.
- **Small mainnet tests** - Not a default recommendation; even small amounts can be lost to fees, traps, or malicious routers.

## 6. Reporting

- **Finding** - Mechanism (function, modifier, authority) plus code or account reference.
- **Confidence** - Static proof versus heuristic versus unknown bytecode.
- **User-facing takeaway** - When in doubt, avoid interaction; prefer audited interfaces and official router addresses.

## Ethical guardrails

- Educational only; not investment or legal advice.
- Do not provide weaponized instructions for scammers.
- False honeypot accusations harm projects with unusual but honest tokenomics-state uncertainty clearly.

**Goal:** Help users and researchers recognize sell-restriction patterns before committing funds, using evidence and safe methods.
