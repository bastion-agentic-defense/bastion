---
name: cmichel-smart-contract-auditor-guide
description: Points to Christoph Michel’s (cmichel.io) long-form guide on becoming a smart contract security auditor-EVM-centric learning path, CTFs, canonical DeFi contracts, finance basics, and an FAQ (tools, scoping, compensation). Use when the user asks how to start in Solidity/EVM auditing or cites this article-not as current salary data, job placement advice, or a substitute for hands-on practice and primary documentation.
---

# cmichel.io - How to become a smart contract auditor (reference)

**Educational routing only.** This skill does **not** reproduce the full article. Read the **live** page for complete detail, links, and any author updates.

## Canonical URL

- **[How to become a smart contract auditor](https://cmichel.io/how-to-become-a-smart-contract-auditor/)** - Christoph Michel, **30 October 2021** (per page metadata).

## What the guide covers (outline)

The post is **Ethereum / EVM–oriented** (most paid audit demand at time of writing, per author). Rough structure:

| Section | Topics (high level) |
|---------|---------------------|
| **Prerequisites** | Programming first; suggests **JavaScript** as a gateway if new; argues reading code is foundational for review work. |
| **Solidity & security** | Learn by doing; recommends **CTF-style** challenges (e.g. **Damn Vulnerable DeFi**, **Ethernaut**, **Capture The Ether**) and notes overlap / legacy Solidity caveats; mentions harder contests (e.g. **Paradigm CTF**) and permissionless venues (**Immunefi**, **Code4rena**). |
| **Common contracts** | **ERC-20 / ERC-721** nuances, **proxies** / `delegatecall`, **MasterChef**-style rewards, **Compound**-style lending, **Uniswap V2** as AMM baseline-author frames these as recurring patterns in real audits. |
| **Finance vocabulary** | Points to a **Khan Academy** derivatives chapter for options/futures/perp-style language used in DeFi. |
| **FAQ** | Staying current (e.g. **Twitter**, **BlockThreat** newsletter), **rough** hourly bands (treat as **historical**), **LOC/hour** scoping heuristics, when to stop reviewing, tooling (**Solidity Visual Developer**), traits (e.g. conscientiousness), **Solana** as a harder pivot (Rust + account model). |

## How to combine with blockint

| Need | Skill |
|------|--------|
| EVM DeFi triage patterns | **evm-solidity-defi-triage-agent** |
| Broader DeFi audit / rug posture | **defi-security-audit-agent** |
| Exploit post-mortems | **flash-loan-exploit-investigator-agent**, **honeypot-detection-techniques** |
| Solana program security (different stack) | **solana-defi-vulnerability-analyst-agent**, **sealevel-attacks-solana** |

## Guardrails

- **Stale data** - compensation ranges and tool landscape are **2021-era**; verify **current** market and tooling.  
- **Not career or legal advice** - hiring, visas, and contracts need **professional** counsel where relevant.  
- **Jurisdiction** - bug bounties and contests have **rules**; follow each program’s terms.  
- **Ethics** - use skills for **defensive** security and responsible disclosure, not theft or harassment.

**Goal:** a discoverable pointer to **[cmichel.io/how-to-become-a-smart-contract-auditor](https://cmichel.io/how-to-become-a-smart-contract-auditor/)** for **EVM auditor education** context inside blockint.
