---
name: on-chain-research-tokenomics
description: Covers on-chain analysis for trading and research (holdings, flows, whales, TVL, multichain), tokenomics basics (supply types, vesting, utility, incentives), and caveats about labels and price causality. Use when the user asks about on-chain metrics, exchange flows, whale tracking, TVL, token unlocks, vesting, governance tokens, or comparing on-chain vs technical analysis.
---

# On-chain research and tokenomics

**Not investment advice.**

## On-chain analysis (trading & research)

Uses **public ledger data** (balances, transfers, contract events, timestamps) alongside **technical** and **fundamental** lenses.

**Building blocks:** **Holdings**; **transactions** (audit trail via tx hashes; can monitor in near real time).

| Lens | Notes |
|------|--------|
| **Top holders / concentration** | Float risk; often aggregated to **entities** when labeled |
| **Exchange flows** | **Heuristic only**-inflows/outflows need **context** (stablecoins, market-making, era) |
| **ETF / treasury** | Flows where addresses or filings are public |
| **Whales** | Large relative size; **following** whales is risky |
| **Multichain** | L2s, bridges-partial views mislead |

**Macro-style metrics (examples):** active addresses, tx count/volume, **TVL**, unique holders-definitions vary by indexer.

**Visualization** helps for complex paths; tables alone miss structure.

## Tokenomics (educational)

**Tokenomics** = issuance, distribution, **utility**, and **incentives**.

**Supply:** **Circulating** vs **total** vs **max** (if any). **Float** = unlocked vs locked-watch **vesting** calendars (not deterministic for price).

**Distribution:** Mining/staking rewards; public sales; **airdrops**; team/investor **locks**; DAO/treasury.

**Utility:** Fees; access; staking; **LSTs**; **restaking** (protocol-specific); governance.

**Demand levers (examples):** incentives, burns, buyback-and-burn, liquidity mining, buybacks-verify in **docs and contracts**.

**Governance:** Token voting exists on a spectrum; not always fully decentralized.

## Caveats

Labels and clusters can be **wrong**; correlation ≠ causation; past patterns ≠ future prices.
