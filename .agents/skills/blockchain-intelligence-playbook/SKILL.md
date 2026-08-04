---
name: blockchain-intelligence-playbook
description: Index skill for the blockint-skills bundle-includes a “choosing a skill” routing map and routes to focused skills on blockchain intelligence fundamentals, address clustering, analytics, tokenomics, investigation ethics, Phalcon Compliance documentation pointer, Chainalysis public Sanctions API/oracle router, FATF official AML/CFT glossary, Arkham Intel research article on leading crypto analysis tools for traders, Christoph Michel cmichel.io guide on becoming an EVM smart contract auditor, risk exposure, behavioral risk, address and transaction screening workflow concepts, Range AI investigation playbook (MCP), standalone Dune Sim and Dune Analytics on-chain analytics skill (dune-sim-onchain-analytics), crypto market mechanics, OSINT (Bellingcat toolkit), Solana external stacks (Helius, Range MCP, Tavily, PayAI, React Flow, Solana Policy Institute), DeFi/MEV/rug skills, privileged-access mitigation lessons (Chainalysis Drift case study), coral-xyz sealevel-attacks Solana security examples, Neodyme Solana Security Workshop (workshop.neodyme.io), Osec (osec.io) Solana auditor introduction blog post, canonical X post citation for @armaniferrante status 1411589629384355840, BlockchainSpider open-source data collection, MoTS (Know Your Transactions / transaction semantics research repo), Impersonator dApp devtools (EVM + Solana read-only address presentation), Katana web crawling, lcamtuf American Fuzzy Lop (AFL) classic documentation (lcamtuf.coredump.cx/afl), and the official Agent Skills open-format specification (agentskills/agentskills, agentskills.io/llms.txt doc index). Use when the task spans multiple topics or the user needs help picking which named skill to load.
---

# Blockchain intelligence - skill index

This repository splits topics into **focused skills** (load the specific skill when the task is narrow). Shared rules: **educational** patterns only; **no** sanctions evasion, harassment, or non-consensual doxxing; **not** legal/investment advice.

## Choosing a skill (quick map)

| If the user is asking about… | Start here |
|------------------------------|------------|
| Crime types, ethics, reporting, CEX/stablecoin limits | **crypto-investigation-compliance** |
| Phalcon Compliance product documentation URL | **phalcon-compliance-documentation** |
| Chainalysis Sanctions API / public oracle, `Chainalysis.md` | **chainalysis-sanctions-screening** |
| FATF AML/CFT glossary terms (CDD, STR, PEP, etc.) | **fatf-glossary-reference** |
| Arkham “leading crypto analysis tools” research / trader tool landscape | **arkham-leading-crypto-analysis-tools** |
| Becoming an EVM smart contract auditor (cmichel.io guide) | **cmichel-smart-contract-auditor-guide** |
| Risk indicators, exposure %, address/tx screening templates | **risk-exposure-screening-concepts** |
| Structuring-like frequency, large transfers, transit / rapid movement | **behavioral-risk-screening-concepts** |
| Address tags/markers, CSV screening, blacklist vs whitelist UX | **address-screening-workflow-concepts** |
| Transaction hash screening, deposit/withdrawal direction, STR exports | **transaction-screening-workflow-concepts** |
| General OSINT tool discovery (non-chain) | **bellingcat-investigation-toolkit** |
| **Dune** Sim + Analytics - workflows, multichain realtime vs SQL | **dune-sim-onchain-analytics** |
| End-to-end on-chain forensics persona | **on-chain-investigator-agent** |
| Solana txs, ATAs, SPL | **solana-tracing-specialist** |
| Helius/Range/Tavily docs, MCP, graph UI (React Flow), x402 (PayAI), Solana policy institute | **solana-onchain-intelligence-resources** |
| Range MCP wallet investigation steps, sanctions, transfers | **range-ai-investigation-playbook** |
| Solana entity clustering / Jito / launchpads | **solana-clustering-advanced** |
| Cross-chain bridges and unified graphs | **cross-chain-clustering-techniques-agent** |
| Broad DeFi audit + rug/governance | **defi-security-audit-agent** |
| Admin takeover, blind signing, Solana durable nonces (mitigations) | **defi-admin-takeover-mitigation-lessons** |
| EVM Solidity contracts (Ethereum/L2) | **evm-solidity-defi-triage-agent** |
| Solana programs (Anchor, PDAs, CPIs) | **solana-defi-vulnerability-analyst-agent** |
| Sealevel Attacks repo (Solana exploit pattern examples) | **sealevel-attacks-solana** |
| Neodyme Solana Security Workshop (workshop.neodyme.io) | **neodyme-solana-security-workshop** |
| Osec “Solana: An Auditor’s Introduction” (runtime primer) | **osec-solana-auditor-introduction** |
| @armaniferrante X post `1411589629384355840` (primary-source citation) | **armaniferrante-x-status-solana-reference** |
| Honeypot / sell restrictions | **honeypot-detection-techniques** |
| Launch rug red flags | **rug-pull-pattern-detection-agent** |
| Flash-loan incidents | **flash-loan-exploit-investigator-agent** |
| Sandwich MEV post-mortems | **sandwich-attack-investigator-agent** |
| MEV infrastructure / searchers | **mev-bot-infrastructure-analysis-agent** |
| MEV + rug overlap hypotheses | **mev-bot-rug-coordination-investigator-agent** |
| Web crawling | **katana-web-crawling** |
| Classic **AFL** / **lcamtuf** fuzzing docs (C/C++ coverage-guided) | **lcamtuf-afl-documentation** |
| **Agent Skills** spec / **SKILL.md** format / [agentskills.io](https://agentskills.io) | **agentskills-specification** |
| Scrapy/Python on-chain datasets, transfer subgraphs (BlockchainSpider) | **blockchain-spider-toolkit** |
| MoTS / KYT transaction semantics, WWW 2023 paper reproduction | **mots-transaction-semantics** |
| Impersonator (EVM/Solana dApp connect as any address, dev/testing) | **impersonator-dapp-devtools** |

When in doubt, load **on-chain-investigator-agent** or this index.

## Skills in this bundle

| Skill | Use when |
|--------|-----------|
| **blockchain-intelligence-playbook** | This index - routing when multiple domains apply |
| **blockchain-intelligence-fundamentals** | What BI is, tool categories (explorers, tracers, etc.), payment rails vs crypto rails |
| **address-clustering-attribution** | **Wallet clustering** (UTXO CIH, EVM deposit sweeps), entities/labels/tags, peel/taint concepts, attribution limits |
| **cross-chain-clustering-techniques-agent** | **Multi-chain** clustering: bridges, wrapped assets, unified graphs, timing/behavior, confidence scoring |
| **blockchain-analytics-operations** | Analytics platforms, AML/forensic **use cases**, tracers/visualizers as product layers |
| **dune-sim-onchain-analytics** | **Standalone** Dune skill - Sim vs SQL, **EVM/SVM** patterns, CUs, subscriptions; [llms.txt](https://docs.sim.dune.com/llms.txt) + [OpenAPI](https://docs.sim.dune.com/openapi.json) for implementation detail |
| **blockchain-spider-toolkit** | [BlockchainSpider](https://github.com/wuzhy1ng/BlockchainSpider) - Python/Scrapy **dataset** collection (EVM/Solana blocks/txs, transfer subgraphs); not web crawling |
| **mots-transaction-semantics** | [MoTS](https://github.com/wuzhy1ng/MoTS) - **KYT** / transaction **semantic** vectors & labels (research); upstream notes merge into BlockchainSpider |
| **impersonator-dapp-devtools** | [Impersonator](https://github.com/impersonator-eth/impersonator) / [Solana](https://github.com/impersonator-eth/impersonator-solana) - WalletConnect-style **address presentation** for dApp **UI testing** (no key custody; ethics-heavy) |
| **on-chain-research-tokenomics** | Holdings/flows/TVL/whales, **tokenomics** (supply, vesting, utility) |
| **crypto-investigation-compliance** | Crime taxonomy, **ethical** OSINT + on-chain workflow, reporting posture |
| **phalcon-compliance-documentation** | **Phalcon Compliance** public **documentation** portal - compliance investigation / monitoring product docs (read live site for features) |
| **chainalysis-sanctions-screening** | **Chainalysis** public **Sanctions API** + EVM **oracle** - SDN-oriented address checks; live docs/Terms; optional repo **`Chainalysis.md`** excerpt |
| **fatf-glossary-reference** | [FATF Glossary](https://www.fatf-gafi.org/en/pages/fatf-glossary.html) - official **AML/CFT** definitions; terminology alignment (not legal advice) |
| **arkham-leading-crypto-analysis-tools** | [Arkham research](https://info.arkm.com/research/leading-crypto-analysis-tools-for-traders-investors) - **fundamental / technical / on-chain** tool survey for traders (not investment advice) |
| **cmichel-smart-contract-auditor-guide** | [cmichel.io](https://cmichel.io/how-to-become-a-smart-contract-auditor/) - **EVM** auditor **learning path**, CTFs, canonical DeFi patterns, FAQ (2021 article; verify stale facts) |
| **risk-exposure-screening-concepts** | **Risk exposure** vocabulary: indicator taxonomies, exposure value/%, address vs transaction templates (entity, interaction, blacklist) - educational |
| **behavioral-risk-screening-concepts** | **Behavioral** patterns: large-value, high-frequency / structuring-like, transit addresses, rapid-transaction rules - educational |
| **address-screening-workflow-concepts** | **Address** inventory: tags vs markers, CSV bulk import, list/detail pages, audit/alert views, blacklist/whitelist semantics - educational |
| **transaction-screening-workflow-concepts** | **Transaction** screening: transfer as unit, deposit/withdrawal direction, CSV import, list/detail, rescreen, STR-style export patterns - educational |
| **bellingcat-investigation-toolkit** | Bellingcat OSINT toolkit: GitBook + GitHub catalog for general investigation tool discovery |
| **crypto-market-structures** | Max pain, covered-call ETFs, arbitrage, bull/bear **flags** (non-prescriptive) |
| **on-chain-investigator-agent** | **End-to-end** forensic investigator persona: tracing, contracts, scam heuristics, evidence reports, ethics |
| **solana-tracing-specialist** | **Solana-only** forensics: ATAs, SPL flows, RPC/indexer patterns, Jito/DEX inner ix, evidence packs |
| **solana-onchain-intelligence-resources** | **Resource router** for Solana intel stacks: Helius, Range MCP, Tavily, PayAI x402, React Flow, Solana Foundation skills (`llms.txt` indexes), [Solana Policy Institute](https://www.solanapolicyinstitute.org/) (policy/education) |
| **range-ai-investigation-playbook** | **Range AI** MCP **investigation** workflow: risk triage, sanctions, connections, transfers, funding source, entities, cross-chain pivot + one-shot prompt |
| **solana-clustering-advanced** | **Solana** entity clustering: graphs, Jito/launchpad heuristics, PDAs, ML validation, confidence scoring |
| **solana-clustering-case-study-agent** | **Solana** clustering → **case studies**: narrative, visuals, CSV/query exports, thread or long-form |
| **defi-security-audit-agent** | **DeFi** security / rug-risk triage: contracts, liquidity, governance, bridges, severity reports from public data |
| **defi-admin-takeover-mitigation-lessons** | **Privileged access** failures-signer hygiene, Solana durable nonces, oracle/collateral abuse, monitoring-using [Chainalysis Drift analysis](https://www.chainalysis.com/blog/lessons-from-the-drift-hack/) as case anchor |
| **evm-solidity-defi-triage-agent** | EVM Solidity DeFi triage: proxies, oracles, reentrancy, access control (Ethereum / L2) |
| **honeypot-detection-techniques** | **Honeypot**-style **risk**: EVM/SPL patterns, static review, fork sim, observational heuristics |
| **rug-pull-pattern-detection-agent** | **Launch** rug-risk: liquidity locks/removal, dev/sniper clusters, contract authorities, tiered scores |
| **mev-bot-rug-coordination-investigator-agent** | **MEV** + rug overlap: bundle/block co-occurrence, timing, joint flows, confidence-scored hypotheses |
| **flash-loan-exploit-investigator-agent** | **Flash-loan** / atomic **exploit** post-mortems (EVM + Solana): traces, impact, evidence packs, mitigations |
| **sandwich-attack-investigator-agent** | **Sandwich** / DEX **MEV** post-mortems: same-block or bundle ordering, victim vs searcher metrics, mitigations |
| **mev-bot-infrastructure-analysis-agent** | **MEV** **infrastructure**: searchers, bundles/builders/relays, strategies, profit paths, centralization metrics (public data) |
| **solana-defi-vulnerability-analyst-agent** | **Solana** DeFi **program** risks: Anchor/PDAs/CPIs, oracles, pools, SPL, safe repro / severity reporting |
| **sealevel-attacks-solana** | [sealevel-attacks](https://github.com/coral-xyz/sealevel-attacks) - **Anchor**-based **exploit** / **mitigation** pattern examples for the Solana VM (educational; defensive use) |
| **neodyme-solana-security-workshop** | [workshop.neodyme.io](https://workshop.neodyme.io/) / [neodyme-breakpoint-workshop](https://github.com/neodyme-labs/neodyme-breakpoint-workshop) - **Solana** security **levels**, PoC framework, mdBook **source** (follow site legal notice) |
| **osec-solana-auditor-introduction** | [Osec blog](https://osec.io/blog/2022-03-14-solana-security-intro) - **auditor**-oriented **runtime** intro (BPF, accounts, System Program; 2022; verify docs) |
| **armaniferrante-x-status-solana-reference** | [@armaniferrante](https://x.com/armaniferrante/status/1411589629384355840) **X post** bookmark - open URL for **verbatim** text; not a spec |
| **katana-web-crawling** | **ProjectDiscovery Katana** install, crawl vs headless, scope, rate limits, pipelines |
| **lcamtuf-afl-documentation** | [lcamtuf AFL](https://lcamtuf.coredump.cx/afl/) - **American Fuzzy Lop** classic **coverage-guided** fuzzing docs (C/C++); compare **AFL++** for current fork tooling |
| **agentskills-specification** | [agentskills/agentskills](https://github.com/agentskills/agentskills) - **Agent Skills** open **format**; [llms.txt](https://agentskills.io/llms.txt) index → [specification.md](https://agentskills.io/specification.md), [integrate-skills.md](https://agentskills.io/integrate-skills.md) |

## Quality checklist (all domains)

- Separate **fact** vs **inference** vs **hypothesis**  
- Cite **sources** for claims about entities or legal outcomes  
- Prefer **primary** docs for ETFs, sanctions, and filings  
- For **clustering/attribution**, assume **probabilistic** outputs  

When one subdomain clearly dominates the request, prefer loading that **named** skill directly instead of this index.
