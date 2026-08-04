---
name: dune-sim-onchain-analytics
description: "Standalone skill for on-chain analytics and product work with Dune-choose Sim realtime APIs (EVM and SVM including Solana) vs Dune Analytics SQL, design CU-aware flows, subscriptions and dashboards, and safe key handling. Use when building or analyzing with Dune Sim, wallet or token intelligence, multichain balances and activity, or SQL over Dune datasets-not for legal sanctions conclusions or exposing API keys in browsers."
---

# Dune Sim and Dune Analytics - on-chain analytics skill

This skill is the **primary home** for Dune-powered workflows in blockint: **how to think**, **what to call when**, and **how to ship** analytics that combine realtime **Sim** data with **historical SQL** where appropriate. It is **not** a thin link list-use it to structure tasks; open **[docs.sim.dune.com/llms.txt](https://docs.sim.dune.com/llms.txt)** and **[openapi.json](https://docs.sim.dune.com/openapi.json)** for exact parameters and auth as you implement.

Educational and integration guidance-users remain responsible for **API keys**, **billing**, and **Terms**.

## Mental model: two engines

| Engine | Best for | Typical outputs |
|--------|-----------|-----------------|
| **Dune Sim** | **Live** wallet and token state-balances, activity feeds, transactions, NFT collectibles, DeFi positions, stablecoin views, **webhook subscriptions** | Apps, agents, alerts, “what does this address hold / do **now**” |
| **Dune Analytics API** | **Historical** exploration-**SQL** over Dune’s curated blockchain tables | Research, cohorts, backtests, dashboards backed by queries |

**Rule of thumb:** Need **streaming or low-latency** address/token views across many **EVM** chains or **SVM** (Solana / Eclipse, etc.) → start with **Sim** and chain-specific docs under `chains/`. Need **long-horizon** or **custom aggregations** across Dune SQL models → **Analytics API** and query execution docs.

## When to use which surface (decision flow)

1. **Single address or small set** - balances, recent activity, NFTs, token metadata → Sim **EVM** or **SVM** endpoints (see overviews below).  
2. **Ongoing monitoring** - balance or activity changes for a watchlist → **Subscriptions** / webhooks (create, update addresses, lifecycle per docs).  
3. **Token distribution or “top holders” style** - Sim **token holders** / related EVM docs; confirm **spam / filtering** parameters (e.g. exclude low-liquidity noise) in live API reference.  
4. **Cross-wallet analytics, custom metrics, or historical slices** - execute or manage **SQL** via **Dune Analytics API**; combine results with Sim in the application layer if needed.  
5. **Multichain** - confirm **chain ID** and **supported chains** in docs before assuming parity across EVM L2s or SVM deployments.

## EVM vs SVM (Solana-class)

- **EVM:** Broad endpoint family-**activity**, **balances**, **transactions**, **collectibles**, **DeFi positions**, **stablecoins**, **subscriptions**, **token info** / holders. Use **[EVM overview](https://docs.sim.dune.com/evm/overview.md)** as the map.  
- **SVM:** **[SVM overview](https://docs.sim.dune.com/svm/overview.md)** - realtime **balances** and **transactions** for Solana-class addresses; align address encoding and chain scope with docs. For **Solana-only RPC-depth** debugging, pair with **solana-tracing-specialist** and provider docs (e.g. Helius)-Sim is **product API**, not a full RPC replacement.

## Operations: CUs, errors, keys

- **Compute Units (CUs)** - Billing is CU-based; estimate cost per workflow (polling vs webhooks, batch sizes). Read **[Compute Units](https://docs.sim.dune.com/compute-units.md)** before production sizing.  
- **Errors** - Implement retries with backoff per **[Error handling](https://docs.sim.dune.com/error-handling.md)**; never treat 429/5xx as permanent failure without reading response bodies.  
- **Secrets** - **No** Sim API keys in front-end bundles. Use server routes or **[Cloudflare proxy](https://docs.sim.dune.com/proxy.md)** pattern from docs.  
- **Spam / noise** - Use documented **token filtering** / `exclude_spam_tokens`-style controls where available; labels and prices change-revalidate for reporting.

## Agent and IDE integration

- **[Agent reference](https://docs.sim.dune.com/agent-reference.md)** - drop-in file for Claude Code, Cursor, Codex, Gemini CLI, etc.  
- **[Build with AI](https://docs.sim.dune.com/build-with-ai.md)** - `llms.txt`, Markdown pages, OpenAPI for tool-calling designs.

## How this skill fits blockint

| Task | Pair with |
|------|-----------|
| Abstract “what is blockchain analytics” | **blockchain-analytics-operations** |
| Solana forensic tracing patterns | **solana-tracing-specialist** |
| Full investigator narrative | **on-chain-investigator-agent** |
| Sanctions / compliance screening products | **range-ai-investigation-playbook**, **chainalysis-sanctions-screening** - different stacks; do not conflate with Dune outputs |

## Guardrails

- **Compliance** - Dune data supports **research**; **sanctions** and **legal** outcomes need designated tools and counsel.  
- **Attribution** - Wallet labels and heuristics can be wrong; separate **fact** (chain events) from **inference** (entity names).  
- **ToS** - Respect Dune’s terms, rate limits, and attribution for published queries or embeds.

## Authoritative specs (keep open while coding)

| Resource | URL |
|----------|-----|
| Doc index | [docs.sim.dune.com/llms.txt](https://docs.sim.dune.com/llms.txt) |
| Quickstart | [docs.sim.dune.com/index.md](https://docs.sim.dune.com/index.md) |
| Dune Analytics API (SQL) | [docs.sim.dune.com/dune-analytics-api.md](https://docs.sim.dune.com/dune-analytics-api.md) |
| OpenAPI | [docs.sim.dune.com/openapi.json](https://docs.sim.dune.com/openapi.json) |

**EVM:** [overview](https://docs.sim.dune.com/evm/overview.md) · [activity](https://docs.sim.dune.com/evm/activity.md) · [balances](https://docs.sim.dune.com/evm/balances.md) · [transactions](https://docs.sim.dune.com/evm/transactions.md) · [subscriptions](https://docs.sim.dune.com/evm/subscriptions.md) · [supported chains](https://docs.sim.dune.com/evm/supported-chains.md) · [token filtering](https://docs.sim.dune.com/token-filtering.md)

**SVM:** [overview](https://docs.sim.dune.com/svm/overview.md) · [balances](https://docs.sim.dune.com/svm/balances.md) · [transactions](https://docs.sim.dune.com/svm/transactions.md)
