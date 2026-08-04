---
name: blockchain-spider-toolkit
description: Points to the BlockchainSpider open-source Python/Scrapy toolkit for collecting on-chain data-transfer subgraphs around an address or tx, EVM and Solana block/transaction ingestion, receipts/logs, and optional label plugins. Use when the user wants to build datasets, offline traces, or research pipelines alongside blockchain-analytics-operations and solana-tracing-specialist-not as a substitute for RPC provider ToS, rate limits, or legal review of sensitive crawls.
---

# BlockchainSpider - on-chain data collection toolkit

**Reference skill.** This repository does **not** vendor BlockchainSpider; read the [upstream README](https://github.com/wuzhy1ng/BlockchainSpider/blob/master/README.md) and [docs](https://github.com/wuzhy1ng/BlockchainSpider/tree/master/docs) for install, spiders, and options.

- **Repository:** [github.com/wuzhy1ng/BlockchainSpider](https://github.com/wuzhy1ng/BlockchainSpider) (MIT license)  
- **Stack:** Python, **Scrapy** spiders, CSV/JSON outputs under `./data` by default (paths per project config).

## What it is for

Typical capabilities described in the project (confirm against current docs):

| Area | Examples |
|------|-----------|
| **Transfer subgraph** | Money-flow graph centered on a **source address** or transaction (e.g. `txs.blockscan`-style crawls). |
| **EVM transactions** | Block ranges, latest listener, receipts, logs, token transfers; multiple EVM-compatible **providers**. |
| **Solana** | Slot ranges or live streams via **JSON-RPC** providers. |
| **Labels** | Optional plugins (e.g. label-oriented crawls)-scope and ethics depend on **source** and **law**. |

Academic background appears in project references (e.g. TRacer / transaction semantics papers-see repo **Reference** section).

## Prerequisites

- Python environment and `pip install -r requirements.txt` from the cloned repo.
- **RPC / indexer API** endpoints you are **authorized** to use (respect **ToS**, **rate limits**, and **billing**).
- **API keys** for third-party explorers (Etherscan-class APIs, etc.) **must** be supplied by you-**never** commit keys or paste live keys into chats.

## Example command shapes (placeholders only)

Upstream examples use `scrapy crawl <spider> -a ...`. Illustrative patterns (replace placeholders):

```bash
# EVM transfer / subgraph style (example spider name from upstream docs)
scrapy crawl txs.blockscan -a source=<ADDRESS> -a apikeys=<YOUR_ETHERSCAN_API_KEY> -a endpoint=<ETHERSCAN_COMPATIBLE_API_URL>

# EVM blocks / transactions over a range
scrapy crawl trans.block.evm -a start_blk=<N> -a end_blk=<M> -a providers=<YOUR_ETH_HTTP_RPC_URL>

# Solana slot range
scrapy crawl trans.block.solana -a start_slot=<S1> -a end_slot=<S2> -a providers=<YOUR_SOLANA_JSON_RPC_URL>
```

Exact **spider names** and **arguments** change with releases-always copy from the **current** README.

## How to combine with blockint

| Task | Skill |
|------|--------|
| High-level analytics / AML context | **blockchain-analytics-operations** |
| Solana forensic tracing methodology | **solana-tracing-specialist** |
| Multi-chain clustering | **cross-chain-clustering-techniques-agent** |
| Web surface crawling (HTTP), not chain RPC | **katana-web-crawling** |

## Guardrails

- **Lawful use only** - comply with **sanctions**, **privacy**, and **computer misuse** rules in your jurisdiction; do not use spiders to **harass** or **dox**.  
- **Darknet / sensitive label sources** - some demo commands in upstream docs point to **Tor** or sensitive data sources; obtain **legal** and **security** approval before running.  
- **Do not** store or share **API keys**, **customer** identifiers, or **non-public** investigation exports in **public** repos.  
- **Outputs** are **raw** or **heuristic**-validate **critical** facts against **primary** chain data.

## Related research codebase

- **mots-transaction-semantics** - [MoTS](https://github.com/wuzhy1ng/MoTS) (WWW 2023 “Know Your Transactions”); upstream notes **MoTS merged into BlockchainSpider**-use MoTS skill for **legacy** spider names (`blocks.eth`, `blocks.semantic.eth`, `labels.action`) and the bundled PDF.

**Goal:** a stable pointer and **safe** usage framing for [BlockchainSpider](https://github.com/wuzhy1ng/BlockchainSpider) inside blockint workflows.
