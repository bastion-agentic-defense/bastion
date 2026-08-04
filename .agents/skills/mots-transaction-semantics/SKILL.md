---
name: mots-transaction-semantics
description: Points to the MoTS (Know Your Transactions) open-source Python/Scrapy research codebase-EVM transaction acquisition, semantic-vector extraction, and Etherscan-backed action labels (WWW 2023). Use when the user cites the MoTS paper/repo, needs legacy spider names (blocks.eth, blocks.semantic.eth, labels.action), or wants to relate transaction semantics to blockchain-analytics-operations-not as a substitute for current BlockchainSpider docs, RPC ToS, or legal review.
---

# MoTS - transaction semantic representation (research codebase)

**Reference skill.** Upstream states that **MoTS is merged into [BlockchainSpider](https://github.com/wuzhy1ng/BlockchainSpider)**-prefer the [BlockchainSpider documentation](https://github.com/wuzhy1ng/BlockchainSpider/tree/master/docs) for **current** spiders and maintenance. This skill documents the **standalone** [MoTS repository](https://github.com/wuzhy1ng/MoTS) for paper reproduction, historical commands, and skill discovery.

- **Repository:** [github.com/wuzhy1ng/MoTS](https://github.com/wuzhy1ng/MoTS)  
- **Paper (in repo):** [MoTS_WWW_2023_manuscript.pdf](https://github.com/wuzhy1ng/MoTS/blob/master/MoTS_WWW_2023_manuscript.pdf) - *Know Your Transactions: Real-time and Generic Transaction Semantic Representation on Blockchain & Web3 Ecosystem* (ACM WWW 2023).

## What the standalone repo implements

| Step | Spider (examples from upstream README) | Role |
|------|----------------------------------------|------|
| **Acquisition** | `blocks.eth` | Collect txs from RPC over block ranges; `types` can include `external`, `internal`, `erc20`, `erc721`, `erc1155`. |
| **Semantics** | `blocks.semantic.eth` | Same shape of inputs plus **transaction semantic vectors** (per upstream description). |
| **Labels** | `labels.action` | Transaction semantics **labels** sourced via **Etherscan-class** APIs (block-range oriented in examples). |

**Storage:** outputs under `./data` (per upstream README). **Config:** upstream points to `APIKEYS`, `PROVIDERS`, and `CONCURRENT_REQUESTS` in `settings.py`.

## Install (clone upstream)

The dependency file is named **`requiremets.txt`** in the repository (upstream spelling). After clone:

```bash
pip install -r requiremets.txt
```

Confirm the filename on the branch you use-it may be renamed in a future commit.

## Example command shapes (placeholders only)

```bash
# Block-range acquisition (replace RPC and range; confirm spider names in upstream README)
scrapy crawl blocks.eth -a start_blk=<N> -a end_blk=<M> -a types=external,internal,erc20,erc721,erc1155

# Semantic vectors + data (same placeholders)
scrapy crawl blocks.semantic.eth -a start_blk=<N> -a end_blk=<M> -a types=external,internal,erc20,erc721,erc1155

# Label-oriented crawl (Etherscan-class API key must be set per settings)
scrapy crawl labels.action -a start_blk=<N> -a end_blk=<M>
```

If `end_blk` is omitted in upstream examples, collection may **follow the chain tip**-mind **RPC cost** and **rate limits**.

## How to combine with blockint

| Task | Skill |
|------|--------|
| **Current** Scrapy toolkit, Solana + EVM breadth | **blockchain-spider-toolkit** (BlockchainSpider) |
| Analytics / AML framing of “transaction meaning” | **blockchain-analytics-operations** |
| EVM DeFi triage | **evm-solidity-defi-triage-agent** |

## Guardrails

- **Prefer BlockchainSpider** for new work if upstream merge note still applies-avoid duplicating unmaintained forks.  
- Configure **your own** `APIKEYS` and `PROVIDERS`; never commit or paste **live** keys.  
- Respect **Etherscan** (and similar) **ToS** and **rate limits**; semantic labels are **research artifacts**, not compliance verdicts.  
- **Lawful use only**; outputs are **not** a substitute for licensed analytics or legal conclusions.

**Goal:** a discoverable pointer to [MoTS](https://github.com/wuzhy1ng/MoTS) and its relationship to [BlockchainSpider](https://github.com/wuzhy1ng/BlockchainSpider) for blockint users researching **transaction semantics** and **KYT-style** representations.
