---
name: blockchain-intelligence-fundamentals
description: Explains what blockchain intelligence is, standard tool categories (explorers, dashboards, tracers, visualizers), and traditional vs crypto payment rails context. Use when the user asks what blockchain intelligence means, how to read on-chain data at a high level, SWIFT vs settlement, stablecoin rails, or which classes of tools exist for chain analysis.
---

# Blockchain intelligence fundamentals

Educational only. Does **not** replace investigators, counsel, or compliance programs.

## What blockchain intelligence is

Blockchains are public, but **raw data** (hashes, calldata, addresses) is hard to use without indexing. **Blockchain intelligence** combines:

- **Indexed chain data** (explorers, APIs)
- **Clustering and heuristics** (see **address-clustering-attribution** skill)
- **Labels** when available
- **Off-chain context** (domains, filings, reputable news, official alerts)

## Tool categories

| Category | Use |
|----------|-----|
| **Explorers** | Blocks, txs, addresses, contract reads; often chain-specific |
| **Dashboards** | Aggregated metrics (flows, balances, network activity) |
| **Tracers / graph tools** | Follow funds across hops and chains |
| **Visualizers** | Graph view of wallets/transfers for pattern spotting |
| **TXID lookup** | Receipt-level detail: time, fees, counterparties, token movements |

## Payment rails - traditional vs crypto (conceptual)

**Payment rails** move value: rules, messaging, settlement, operators.

**Traditional (simplified):** **SWIFT** (messaging, not cash movement), **ACH**, **RTGS/Fedwire**, **cards**, **SEPA**. Often batch-based, hours-limited, intermediary-heavy; cross-border can be slow/costly.

**Crypto rails:** Shared **ledger** settlement; **24/7**; **programmability**. **Stablecoins** often used for payments due to relative price stability vs volatile L1 assets.

**Tradeoffs:** speed/cost on some corridors vs **fee spikes**, **regulatory** fragmentation, **privacy** vs **auditability**, **issuer** risk for centralized stablecoins.

**Transparency spectrum:** public chains → selective disclosure / ZK designs → strong privacy tools (often heavily regulated).

Verify statistics and claims with **primary** sources.
