---
name: crypto-investigation-compliance
description: Maps high-level crypto crime categories, safe and ethical OSINT plus on-chain investigation workflow, and victim reporting posture. Use when the user asks about scam types, pig butchering, rug pulls, tracing stolen funds ethically, compliance-adjacent investigation, or how to document cases for authorities.
---

# Crypto investigation and compliance awareness

Educational workflow guidance only. **Not** legal advice. Do **not** assist with laundering, sanctions evasion, harassment, or non-consensual deanonymization.

## Crypto crime - conceptual map (high level)

**Crypto crime** spans theft, fraud, laundering, ransomware, sanctions evasion, terrorist financing, market abuse, and more. Responses blend **chain tracing**, **OSINT**, and **legal** process.

**Common terms (non-exhaustive):**

- **Pig butchering** - long-build trust → fake investment platform  
- **Rug pull** - insiders drain or abandon  
- **Ponzi / yield scam** - returns from new money  
- **DeFi exploit / bridge hack** - contract or infra failure  
- **Mixer / tumbler** - trail obfuscation (lawful privacy vs illicit use is context-specific)  
- **Drainer / phishing** - malicious signing  
- **Pump-and-dump / wash trading** - manipulation  

Chains attract misuse due to **pseudonymity**, **speed**, **cross-border** reach, **programmability**; obfuscation complicates but rarely perfects hiding.

**Victims:** report quickly via **official** national cybercrime channels where applicable; on-chain timelines can support **law enforcement** filings-check **local** rules.

## CEX deposits, stablecoins, and off-chain gaps

- **Exchange deposits and withdrawals** - On-chain you often see transfers into a labeled hot-wallet cluster; tying that to a specific customer account usually requires exchange cooperation or legal process-do not treat analytics labels as proof alone.
- **Stablecoins (USDC, USDT, etc.)** - Track mints, burns, and large transfers on-chain; issuer blacklists and freezes are not fully observable from public RPC alone.
- **Documentation** - Separate on-chain facts from what requires custodial or legal follow-up.

## Ethical investigation workflow

1. **Anchor** - hashes, addresses, contracts, amounts, times  
2. **Explore** - explorers; contract path  
3. **Cluster** - treat as probabilistic (see **address-clustering-attribution**)  
4. **Attribute cautiously** - strong evidence only  
5. **Document** - sources, fact vs inference  
6. **Escalate** - professionals / authorities for recovery  

**Do not:** doxx, accuse without evidence, bypass legal process.

## Related skills

- **address-clustering-attribution** - clustering mechanics  
- **blockchain-analytics-operations** - AML-style platform context  
- **bellingcat-investigation-toolkit** - [Bellingcat’s OSINT tool catalog](https://bellingcat.gitbook.io/toolkit) (general open-source investigation tools; verify live links)  
- **evm-solidity-defi-triage-agent** - EVM Solidity DeFi contract triage (complements chain tracing)  
- **solana-onchain-intelligence-resources** - includes [Solana Policy Institute](https://www.solanapolicyinstitute.org/) for **public** policy and regulatory **education** on Solana (not legal advice); use for context separate from per-case facts  
- **range-ai-investigation-playbook** - structured **Range MCP** investigation steps (risk, sanctions, flows, cross-chain) and prompt template; labels are **not** legal findings  
- **phalcon-compliance-documentation** - canonical URL for **Phalcon Compliance** **public** documentation (product operator reference; not legal advice)  
- **risk-exposure-screening-concepts** - risk **indicator** categories, exposure metrics, address vs transaction screening templates (commercial screening vocabulary; not a legal finding)  
- **behavioral-risk-screening-concepts** - volume, velocity, transit-style **behavior** rules at address and transaction level (alerts are not proof of crime)  
- **address-screening-workflow-concepts** - tags/markers, bulk import, address list/detail, blacklist/whitelist **policy** patterns in screening UIs (not legal advice)  
- **transaction-screening-workflow-concepts** - tx/transfer screening, **deposit** vs **withdrawal**, STR-style **export** hooks in products (not legal filing advice)  
