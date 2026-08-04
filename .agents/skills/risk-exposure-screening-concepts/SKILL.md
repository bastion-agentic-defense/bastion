---
name: risk-exposure-screening-concepts
description: Educational map of risk exposure screening-typical risk indicator taxonomies, exposure value and percentage, address-level vs transaction-level engines, and common template families (entity label, multi-hop interaction, blacklist). Use when the user asks how commercial screening tools reason about labeled addresses, tainted flows, or deposit vs withdrawal checks-not for legal sanctions determinations or substituting a vendor’s live rules.
---

# Risk exposure screening (concepts)

**Educational reference only.** Labels and scores from analytics or compliance tools are **not** legal findings. Verify **primary** sanctions lists and jurisdictional requirements with qualified teams. Pair with **crypto-investigation-compliance** and **blockchain-analytics-operations**.

## Risk Exposure Engine (idea)

A **risk exposure engine** combines a **database of address or entity labels** with **on-chain interaction graphs** to estimate whether a screened **address** or **transaction** is associated with **known risk categories**. Implementation details differ by product; confirm hop counts, thresholds, and asset scope in your provider’s documentation (see **phalcon-compliance-documentation** where relevant).

## Risk indicators (typical taxonomy)

Risk indicators are **categories** assigned to addresses or entities. Names vary by vendor; the table below lists **common** families used in industry documentation.

| Risk indicator | Description |
|----------------|-------------|
| Sanctioned | Associated with entities on official sanctions lists (for example OFAC SDN), where applicable. |
| Terrorism financing | Associated with designated terrorist organizations or financing of terrorism. |
| Human trafficking | Associated with trafficking-related organizations or flows. |
| Drug trafficking | Associated with illicit production, transport, or distribution of controlled substances. |
| Attack | Related to exploit actors, attacker contracts, or associated fund movements. |
| Scam | Fraudulent schemes: phishing, Ponzi, honeypots, pig-butchering, etc. |
| Ransomware | Controlled by ransomware operators or used to pay ransom. |
| Child sexual abuse material (CSAM) facilitation | Facilitating payments for platforms distributing CSAM. |
| Money laundering | Suspected laundering of illicit proceeds. |
| Mixing | Mixer or privacy services that obscure trails. |
| Darknet market | Operators of darknet markets. |
| Darknet business | Other illicit darknet commerce (for example weapons, identity theft). |
| Frozen / contract blacklist | Blacklisted by major contracts or issuers (for example stablecoin freezes). |
| Gambling | Online gambling service addresses. |
| No-KYC exchange | Virtual asset service providers with weak KYC, per provider policy. |
| FATF high-risk jurisdiction | Entities tied to jurisdictions on FATF “black” lists (as used by the data provider). |
| FATF grey-list jurisdiction | Entities tied to FATF grey-list jurisdictions (as used by the data provider). |

## Exposure metrics

| Term | Meaning |
|------|---------|
| **Exposure value** | Total USD value (per provider’s pricing source) of assets that **originated from** or **interacted with** a specified risk-labeled source, under the engine’s rules. |
| **Exposure percentage** | Share of **tainted** value relative to total **inflow** or **outflow** value for the screened address, under the configured window and rules. |

Interpretation is **policy-dependent**-same raw hops can score differently by direction, hop depth, and minimum amounts.

## Address-level screening (common templates)

Many platforms expose three **template** families for **addresses**:

1. **Entity / direct label risk** - The screened address **itself** carries a risk label (for example sanctioned or scam).  
   *Illustrative:* an address is flagged because it matches an OFAC-listed identifier in the provider’s dataset.

2. **Interaction risk** - Traces **incoming or outgoing** value across **multiple hops**. If any counterparty in the path carries a risk label, the engine may flag exposure **subject to** direction, hop limits, amount thresholds, and decay rules.

3. **Blacklist interaction** - Detects interaction with addresses on a **customer-defined** or **tenant** blacklist (policy-specific).

## Transaction-level screening (common templates)

For a **single transaction**, engines often provide:

1. **Participant risk** - Whether **addresses participating** in the transaction carry configured risk indicators.  
   **Deposit vs withdrawal (typical convention):** for a directional screen, **deposit** flows may screen only the **from** side, and **withdrawal** flows only the **to** side-confirm in product docs.

2. **Interaction / flow risk** - Traces fund **provenance** or **destination** of the transaction’s value to detect prior exposure (for example receiving from a phishing-labeled cluster). Deposit/withdrawal modes may restrict whether **source** or **destination** tracing applies.

3. **Blacklist interaction** - Whether the transaction touches **blacklisted** addresses per policy.

## Guardrails

- **Do not** treat a commercial **label** as a court finding or automatic **sanctions** violation.  
- **Do not** assist with **evading** screening, mixers for illicit purpose, or **circumventing** law enforcement processes.  
- **Separate** on-chain **facts** from **vendor scoring**-document both when reporting.

## See also

- **behavioral-risk-screening-concepts** - volume, velocity, and transit-style **behavior** heuristics (complements label-based exposure).

**Goal:** give investigators a **shared vocabulary** for exposure-style screening without binding any specific **product** behavior or **legal** outcome.
