---
name: address-clustering-attribution
description: Explains blockchain address clustering heuristics (UTXO common-input ownership, account-based deposit sweeps), entity and label layers, attribution confidence, peel chains and taint-style tracing concepts, and operational caveats. Use when the user asks about wallet clustering, linking addresses to the same owner, exchange deposit patterns, attribution, deanonymization limits, or how analytics firms group addresses.
---

# Address clustering and attribution

Educational only. Do **not** assist with harassment, non-consensual doxxing, or sanctions evasion. High-stakes conclusions require **legal** and **compliance** process-not a vendor UI alone.

## Why clustering exists

Addresses are **pseudonymous**. **Clustering** infers that multiple addresses are **likely** controlled by the **same** real-world actor, so analysts can reason about “wallets” rather than isolated strings.

All clustering is **probabilistic**-wrong merges and missed links happen.

## UTXO chains (e.g. Bitcoin)

- **Common input ownership heuristic (CIOH)** - If two (or more) addresses appear as **inputs** to the **same** transaction, they are often assumed to be controlled by the **same** spender (standard wallet behavior). Exceptions exist (coinjoin, collaborative txs, some privacy techniques).
- **Change outputs** - Many spends send value to a recipient **and** “change” back to a new address controlled by the sender; change-detection heuristics try to link those outputs to the same wallet cluster over time.

Privacy practices (coinjoin, careful coin selection) **weaken** naive clustering.

## Account-based chains (e.g. EVM)

- **Deposit–sweep patterns** - Exchanges often give users **unique deposit addresses**; funds later **sweep** to **hot/cold** pools. Analytics tools model “user deposit → exchange cluster” links using repeated patterns and timing.
- **Smart contract wallets** - Multisig, account abstraction, and relayers can complicate “one EOA = one person” assumptions.

## Entity, label, and tag (metadata layers)

| Layer | Typical meaning |
|-------|------------------|
| **Entity** | A **cluster** of addresses grouped as one actor (person, fund, exchange)-often **multichain** |
| **Label** | Name for a **specific** address (e.g. named hot wallet); may include **private** user labels |
| **Tags** | Behavioral or risk descriptors (many per address)-often mixed **automation**, **analyst review**, **community** input |

**Attribution** maps a cluster to a **real-world name** using OSINT, subpoenas, exchange cooperation, or leaks-**confidence varies**.

## Flow tracing concepts

- **Peel chains** - Series of rapid partial withdrawals/movements sometimes used to **obfuscate**; pattern-based detection is **heuristic**.
- **Taint / proximity scoring** - Some tools score how closely funds relate to a **flagged** source along a path; **definitions differ by vendor** and are not legal verdicts.

## Quality practices

- Treat clusters as **hypotheses**; seek **independent** corroboration for accusations or compliance actions.
- **Stale or wrong** labels and merges occur-especially after protocol upgrades or custodial restructuring.
- **Mixer / privacy** and **cross-chain bridges** break simple narratives-trace may be incomplete.

## Related

For investigation **workflow** (OSINT steps), see **crypto-investigation-compliance**. For **platform use cases** (AML dashboards), see **blockchain-analytics-operations**. For **bridge-linked** and **multi-chain** unified clustering (graphs across chains, wrapped-asset normalization), see **cross-chain-clustering-techniques-agent**.
