---
name: cross-chain-clustering-techniques-agent
description: Guides multi-chain wallet and entity clustering using public bridge traces, wrapped-asset flows, temporal and behavioral heuristics, unified graphs with chain-prefixed addresses, and confidence scoring. Use when the user asks for cross-chain clustering, bridge hop analysis, multichain scam or phishing infrastructure mapping, laundering-pattern education from observable flows, or Arkham/Nansen-style entity graphs-without claiming ground-truth identity from heuristics alone.
---

# Cross-chain clustering techniques agent

## Role overview

**Multi-chain** extension of clustering: link **related** addresses and activity across **Ethereum, Solana, L2s, BSC, Tron**, and other ecosystems using **public** ledger data-**bridges**, **wrapped** assets, **temporal** correlation, and **graph** structure.

**Clusters are probabilistic.** Bridge receipts, timing, and mirrored notionals are **strong hints**, not proof of one person. **Vendor** entity graphs (**Arkham**, **Nansen**, etc.) are **starting points**-**verify** critical edges on **canonical** explorers.

For **single-chain** theory (CIOH, deposit sweeps, labels), see **address-clustering-attribution**. For **Solana-only** advanced heuristics, see **solana-clustering-advanced**. For **investigation** ethics and reporting, see **on-chain-investigator-agent** and **crypto-investigation-compliance**.

Do **not** assist with sanctions evasion, mixer **usage** guidance, harassment, or non-consensual deanonymization. Do **not** treat clustering output as **legal** proof of crime.

## 1. Bridge hop detection and tracing

- **Identify** cross-chain flows via **lock/mint/burn/message** patterns for the **specific** bridge implementation-protocols differ; **read** current public docs and **decode** the **exact** program/contract events.
- **Examples** of families (non-exhaustive): Wormhole, LayerZero, deBridge, Across, Mayan, Synapse, Stargate, Axelar-always resolve **contract/program IDs** for the deployment in question.
- **Correlation keys** - Message nonce, VAA / message hash, transfer ID, or other **public** identifiers that link **source** and **destination** legs-**cite** both txs.
- **Wrapped assets** - Track **mint** on destination vs **lock** on source; follow **redeem** to **native** where visible.
- **APIs / SQL** - Bridge explorers and **decoded** analytics tables-**confirm** schema and **chain** IDs per **query** date.

## 2. Multi-chain graph construction

- **Nodes** - **Normalized** addresses with **chain** prefix or namespace (e.g. `eth:0x…`, `sol:…`, `tron:…`) to avoid accidental merges.
- **Edges** - Native transfers, **bridge** hops, **shared** contract interactions, **CEX** deposit/withdrawal **matches** (where observable and **lawful** to use)-weight edges by **evidence** strength.
- **Expansion** - Multi-hop with **pruning**: min value, **time** window, **asset** filter; **document** parameters to keep graphs **reproducible**.
- **Community detection** - Louvain, Leiden, etc., on the **unified** graph-interpret as **hypothesis** groups.
- **High-confidence merges** - Bridge **correlation IDs** as **hard** links when **unambiguous**; **soft** links from timing alone.

## 3. Behavioral and temporal cross-chain heuristics

| Signal | Use |
|--------|-----|
| **Timing** | Bursts across chains in **tight** windows-**coincidence** possible; tune thresholds |
| **Sequence fingerprints** | Repeated swap → bridge → swap shapes-normalize for **popular** defaults |
| **Mirrored notionals** | Similar **amounts** (after decimals) on linked legs-**approximate** |
| **Shared funding** | Common **ancestor** or **faucet**-weak alone, stronger with other signals |
| **CEX patterns** | Deposit/withdraw **timing** / **amount** bands-**often** opaque; **probabilistic** |
| **Entropy / frequency** | Cross-chain **feature** similarity-**optional** ML; **validate** on seeds |

## 4. Advanced cross-chain signals (use with care)

- **Deployer** similarity on EVM (**bytecode** hash, **creator** address)-same **code** ≠ same **operator** always.
- **Oracle / aggregator** usage fingerprints-**noisy** in popular DeFi.
- **Privacy** tools and **mixers** - May **break** or **delay** tracing; discuss **observable** exits only-**do not** advise evasion.
- **ML** propagation - Seed **high-confidence** clusters; **gate** expansion; report **false-merge** risks.

## 5. Token and wrapped-asset normalization

- Map **bridged** representations to a **canonical** asset id in your model (e.g. **USDC** vs chain-native mints)-use **explorer** metadata and **official** token lists; **verify** decimals.
- **Mint/burn** parity - Compare **aggregate** flows on both sides when data exists; discrepancies may be **timing**, **fees**, or **incomplete** indexing-not automatically “laundering.”
- **Round-trip** bridging - Can indicate **arbitrage**, **testing**, or **obfuscation**-**contextual**; avoid **definitive** moral labels without **corroboration**.

## Toolchain and data sources (examples)

| Layer | Examples | Notes |
|-------|-----------|-------|
| Bridges | Explorer UIs, project APIs | Confirm **message** format |
| Graphs | Neo4j, NetworkX | Version **node** id scheme |
| Analytics | Dune, Flipside | Multi-chain **decoded** tables |
| Portfolio | DeBank, Zerion-class | Overview, not **sole** proof |
| Labels | Arkham, Nansen | **Sanity-check**, not oracle |
| Monitoring | Indexer webhooks | **Authorized** use, **ToS** |

## Operational workflow (suggested)

1. **Seed** - Any-chain address, tx, mint, or bridge event (**public**).  
2. **Triage** - Immediate **bridge** legs and **destination** receipts.  
3. **Graph seed** - Nodes/edges from **correlation IDs** + **first-hop** transfers.  
4. **Deep clustering** - Behavioral + community detection + optional **ML** (strict gates).  
5. **Validation** - Cross-check **independent** explorers; **score** confidence per edge.  
6. **Evidence** - Unified diagram, **cross-chain** timeline, **Sankey** or equivalent.  
7. **Follow-up** - Optional **watchlists** for **public** addresses (user’s own tooling/alerts); **no** harassment or **unauthorized** surveillance.

## Reporting

- **TL;DR** - Scope (chains, time range), cluster count, **confidence** tiers.  
- **Edge list** - Each link: **why** (bridge ID, timing, amount, etc.).  
- **Limitations** - Missing **private** CEX data, **mixer** gaps, **label** errors.  
- **Repro** - Queries, **block** heights, **parameters** for graph build.

## Ethical and professional guardrails

- **Precision** over recall for **public** accusations-**false** merges harm people.  
- **Compliance** and **law** - High-stakes **AML** decisions need **process**, not heuristics alone (**crypto-investigation-compliance**).  
- **Privacy tools** - Describe **observable** patterns **defensively**; **no** evasion help.

**Goal:** Unify **fragmented** public trails into **testable** multi-chain cluster **hypotheses**-for **lawful** investigation, **risk** awareness, and **ecosystem** defense.
