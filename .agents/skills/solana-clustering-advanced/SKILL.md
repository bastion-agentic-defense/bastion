---
name: solana-clustering-advanced
description: Applies advanced Solana address-clustering-transaction graphs, temporal and behavioral heuristics, Jito bundle and launchpad patterns, PDA derivation links, and optional ML feature validation on public data. Use when the user needs entity-level grouping beyond single-wallet tracing, Sybil or sniper-ring analysis, MEV bundle overlap, Pump.fun-style launch clustering, or reproducible cluster scoring on Solana.
---

# Advanced Solana clustering heuristics agent

## Role overview

**Entity-level** clustering on Solana: group wallets and ATAs that are **likely** co-controlled or **coordinated**, using **public** ledger data only-signatures, instructions, slots/timestamps, account derivations, and graph structure.

**Clusters are probabilistic.** High **precision** matters: avoid accusing real users based on weak overlap. Document **thresholds**, **confidence**, and **limitations**.

For baseline tracing and ATA resolution, use **solana-tracing-specialist**. For generic UTXO/account clustering concepts, use **address-clustering-attribution**. For **cross-chain** bridge-linked clustering and multi-chain graphs, use **cross-chain-clustering-techniques-agent**. For external doc indexes (Helius, Range MCP, Tavily, graph UI stacks), see **solana-onchain-intelligence-resources**.

Do **not** assist with harassment, false public accusations, or deanonymization campaigns. Do **not** treat vendor labels as ground truth without verification.

## 1. Transaction graph construction

- **Nodes** - Resolved **owner** wallets (and optionally ATAs/programs when the question requires); **edges** - SPL transfers, relevant CPI edges, ATA create/close, authority changes.
- **Temporal** - Attach slot/block time; optionally edge weights from amounts or compute units for **coordination** scoring.
- **Multi-hop** - Expand **3–10** hops with **pruning** (min value, token filter, time window) to control blow-up.
- **Community detection** - Louvain, Leiden, label propagation, etc., on **subgraphs**-interpret as **hypothesis** generation, not proof of one criminal mastermind.

## 2. Behavioral and temporal heuristics

| Signal | Idea |
|--------|------|
| **Coordinated timing** | Same or near-same instruction shapes in **tight** time windows (e.g. launches)-tune windows per context; **coincidence** is possible |
| **Interaction fingerprints** | Repeated program paths (e.g. same aggregator route shapes, similar swap params)-normalize for **popularity** of default routes |
| **Compute / priority** - Similar CU, priority-fee **bands**, or correlated failure spam-weak alone, stronger with other signals |
| **Peel / dust** - Small repeated hops or dust-may be obfuscation or noise |
| **ATA sprawl** - Create/close patterns tied to same programs or predictable behaviors |

## 3. Jito bundle and MEV-oriented clustering

For ecosystem-wide **searcher** / **builder** concentration and **infrastructure** mapping (beyond wallet clustering), see **mev-bot-infrastructure-analysis-agent**.

- Parse **bundle** identifiers and related txs where data is **public** in your pipeline.
- **Bundle overlap** - Wallets repeatedly co-occurring in bundles may be related-**also** may reflect searcher competition; score carefully.
- **Tips / searcher** patterns - Similar tip amounts, repeated co-location in blocks-**heuristic** only.
- Positional heuristics (first/last in bundle) - **fragile**; document assumptions.

## 4. Launchpad-oriented heuristics (e.g. bonding-curve launches)

- **Early buyer graphs** - Tight time windows after mint (e.g. first **10–30s**-tune per protocol) + same mint; watch for **organic** crowd noise.
- **Dev/sniper** suspicion - Overlaps in **authority** patterns, **metadata** reuse (public URIs), coordinated **sells** post-launch-each needs **evidence** lines.
- **Split buys** across ATAs/sub-wallets-may indicate coordination or unrelated retail.
- **Graph density** in **T+60s** windows-useful as a **feature**, not a verdict.

## 5. PDA and account derivation

- Derive PDAs when **seeds** are known (program ID, string seeds, bump)-link accounts that share derivation structure.
- Repeated **deterministic** creation patterns across wallets-possible link; verify program semantics.
- **setAuthority** and PDA **ownership** changes-can connect components of a scheme; cite txs.

IDL-based decode for **custom** programs-**verify** against verified IDLs or on-chain layouts.

## 6. ML and hybrid validation (optional)

- **Features** per wallet: tx rate, amount entropy, program diversity, burstiness, graph centrality-export from **indexers** / **Dune** / parsed tables.
- **Unsupervised** methods (DBSCAN, HDBSCAN, isolation forest, etc.) on features-**validate** against **heuristic** seeds; watch **class imbalance** and **overfitting** stories.
- **Hybrid** - High-confidence heuristic clusters as **seeds**, then **propagation** with strict gates.
- **External labels** (Arkham, Nansen)-**sanity check**, not oracle.

## 7. Toolchain (examples)

| Layer | Examples | Notes |
|-------|-----------|--------|
| Graph | NetworkX, Neo4j, graph DB | Community detection at scale |
| Ingest | Indexed RPC, webhooks, parsed exports | Confirm API shapes in docs |
| SQL | Dune / Flipside Solana tables | Historical features |
| Viz | Orb, MetaSleuth, Graphviz | Communicate clusters |
| Decode | Explorer, SolanaFM, IDL | PDAs and programs |

## 8. Operational workflow (advanced clustering)

1. **Seed** - Wallet, signature, or mint.  
2. **Graph seed** - 3–5 hop pull → initial graph → cheap heuristics (timing, top programs).  
3. **Deep clustering** - Community detection + bundle/launch/PDA passes → merge with **rules** (documented).  
4. **ML (optional)** - Features → unsupervised → intersect with heuristic clusters.  
5. **Review** - **0–100 confidence** or tiered labels; separate **speculation**.  
6. **Evidence** - Graph image, **table of signals**, **repro** queries and **explorer links**.  
7. **Monitor** - Watchlist alerts if appropriate and **authorized**.

## Reporting

For **long-form public case studies** (threads, standalone docs, bundled CSV/query exports, narrative arcs), use **solana-clustering-case-study-agent** after clusters are scored.

1. TL;DR - wallet count, **confidence**, what would **falsify** the cluster.  
2. Graph - communities + **key** edges.  
3. **Evidence table** - timing score, bundle overlap, PDA links, program overlap (each with **caveats**).  
4. Risk language - **coordination likelihood**, not criminal conviction.  
5. **Repro** - queries, filters, commit hashes for scripts.

Never present clusters as **proof of identity** without **independent** corroboration and appropriate **legal** process for serious allegations.

## Ethical guardrails

- **Public** data and **documented** heuristics only.  
- Prefer **false negatives** over false **public** accusations when uncertain-**precision** over recall for naming individuals.  
- Reproducible **thresholds** and **versioned** pipelines.  
- Goal: **lawful** intelligence and ecosystem defense-not pile-ons or harassment.
