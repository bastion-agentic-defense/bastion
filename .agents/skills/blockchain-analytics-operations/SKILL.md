---
name: blockchain-analytics-operations
description: Describes how blockchain analytics platforms work in practice, typical use cases (markets, compliance, law enforcement, tax, market integrity), tool layers like visualizers and tracers, and limitations of heuristic attribution. Use when the user asks about blockchain analytics for AML, transaction monitoring, forensic tracing, institutional ops, or taint-style analysis at a high level.
---

# Blockchain analytics operations

**Blockchain analytics** is the **collection, interpretation, and presentation** of on-chain data, usually with **extra metadata** (clusters, labels, risk flags) beyond raw explorers.

Educational only-not a substitute for **sanctions lists**, **legal process**, or **licensed** compliance programs.

## Core techniques (conceptual)

- **Clustering** - See **address-clustering-attribution** skill for heuristics (UTXO vs account-based).
- **Attribution** - Naming clusters via OSINT, partnerships, investigations.
- **Risk / pattern flags** - Interactions with high-risk services, peel-like patterns, mixer proximity; **taint** scoring varies by product.

## Typical use-case buckets

| Bucket | Examples |
|--------|-----------|
| **Markets & asset management** | Treasury visibility, staking ops, ETF-related reconciliation |
| **Compliance & AML** | Counterparty screening, transaction monitoring-**official lists and policy** beat third-party tags |
| **Law enforcement / recovery** | Long-horizon tracing; **tx hashes** as verifiable evidence anchors |
| **Tax / reporting** | Declared vs observed activity where jurisdiction applies |
| **Market integrity** | Abuse-pattern research-often needs **statistics** + context |

## Tool layers (vendor-neutral)

Graph **visualizers**, **tracers** (directional / taint-style), **alerting dashboards**, **entity directories**. **AI** may summarize traces; **human review** for consequential decisions.

## Limits

Heuristics **misfire**; privacy tech and custodial hops obscure flows. Analytics is **decision support**, not infallible truth.

## See also

- **blockchain-spider-toolkit** - [BlockchainSpider](https://github.com/wuzhy1ng/BlockchainSpider) (open-source Scrapy-based) for **building** raw or structured on-chain **datasets** (EVM/Solana) when you need offline pipelines; pair with this skill for **interpretation** and **limits** of analytics.
