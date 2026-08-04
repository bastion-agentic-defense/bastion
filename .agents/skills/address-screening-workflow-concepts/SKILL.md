---
name: address-screening-workflow-concepts
description: Educational map of address-centric compliance screening-tags vs markers, single and bulk import, address list and detail views, audit trails, blacklist and whitelist policy effects, and delete behavior. Use when the user asks how commercial screening UIs organize wallets/contracts, KYA-style exports, or rescreen policies-not for legal advice or bypassing controls.
---

# Address screening workflow (concepts)

**Educational reference only.** Exact limits (row counts, credit rules, field names) vary by **product** and **release**-confirm in live documentation (**phalcon-compliance-documentation** for Phalcon Compliance). Pair with **risk-exposure-screening-concepts** and **behavioral-risk-screening-concepts** for what engines measure.

## What is being screened?

**Addresses** usually mean **wallets or contracts** on specific **chains**. In a VASP or platform context, these are often **customer** or **counterparty** addresses that **touch** your service. **Proactive** screening is meant to surface high-risk entities early so teams can apply **policy** (review, block, escalate)-not to replace **legal** process or **law-enforcement** channels.

## Tags vs markers

| Concept | Typical role |
|---------|----------------|
| **Tags** | Human-readable **labels** to distinguish addresses in the UI. Often split into **default** tags (from the **provider’s** curated or verified dataset) and **user-defined** tags. User-defined values commonly **override** defaults when both exist. |
| **Markers** | **Categories** by attribute or behavior (for example **deposit** address, **frozen**, **VIP**, **pending review**). |

**System markers** (often immutable in the product):

- Sourced from the **provider’s** risk or reference **database** (for example sanctioned, mixer-per vendor taxonomy).
- May be **non-editable** in the UI.

**Custom markers** (tenant-defined):

- Internal classifications your team adds.
- Usually **editable**; products may cap **total** markers per address (for example **system + custom** combined ≤ **5**-verify in docs).

Naming of fields differs by vendor; treat the table as **pattern**, not a spec.

## Import and screening methods

Common operational paths:

1. **Single address** - Enter an address; the product **resolves** chain metadata and pulls **tags** / **system markers** from its integrations, then you **select** it for ongoing screening.
2. **Bulk CSV** - Upload many addresses using a **template** from the product (chain name, address, optional tag/marker/customer id).

Typical **CSV-style** columns (names vary):

| Field | Purpose |
|-------|---------|
| Chain | Exact chain identifier required by the template |
| Address | On-chain identifier |
| Tag | Optional private user tag |
| Marker | Optional custom marker |
| Customer ID | Optional internal customer key (new or existing) |

Bulk uploads often have a **row limit per file** (for example on the order of **100** addresses-confirm current limit). After import, review **success/failure** rows in the UI.

## Address list

Screened addresses usually appear in a **list** with columns such as: address, **risk summary**, **open alerts**, **last screened** time, **markers**, **customer**, **time added**, **rescreen** status-exact columns depend on the product.

## Address details page

Drilling into one address, products commonly expose:

- **Basic information** - Entity/category (if modeled), chain, markers, tags, balances, aggregate **inflow** / **outflow**.
- **Risk summary** - High-level rollup of triggered alerts.
- **Risk overview** - Finer breakdown: identified risk types, exposure-style views (for example share of tainted notionals), lists of **exposed** or **related** addresses, and links to **graph** or **trace** tools **where the product integrates them** (vendor-specific).
- **Alerts** - Filterable alert history for that address.
- **Audit log** - **Comments** (collaboration) and **system events** (screening runs, risk changes, automation).

Common **actions**:

- **Rescreen** - Force a fresh screening run (subject to product rules and **credits**).
- **Export KYA-style report** - Pack address metadata and risk findings for **internal** or **audit** communication (not a substitute for regulatory filings).

## Blacklist vs whitelist (policy lists)

Products often let operators put addresses on **internal** lists. **Effects are policy- and product-specific**; a common **pattern** looks like:

**Blacklist (blocked / high-friction list):**

- May force a **high** internal severity (for example **critical** risk tier).
- May **skip** ordinary periodic screening (to save credits) while still treating the address as **blocked** or **escalated**.
- May **disable** automatic **rescreen** cycles.
- May **propagate** severity to **direct** counterparties in some configurations (strong compliance implications-follow your program’s rulebook).

**Whitelist (trusted / low-friction list):**

- May mark the address as **no** or **low** internal risk for **policy** purposes.
- May **exclude** from standard engine passes and **rescreen** loops to save credits.

**Caution:** whitelist entries are **operational** decisions inside the tool-they do **not** change real-world illicit activity off-platform. Document **approvals** and **review cadence**.

## Deleting an address

Removing an address from the **inventory** typically **expires** or **closes** **alerts** tied to that record in the product. It does **not** erase public-chain history. Confirm retention and audit requirements before deletion.

## Guardrails

- **Do not** paste **production** customer lists, CSVs, or internal IDs into **public** chats.  
- **Do not** assist with **evading** sanctions, **mis-labeling** peers, or **gaming** whitelist policies.  
- Prefer **primary** legal and sanctions sources for **regulatory** obligations; UI risk tiers are **heuristics**.

## See also

- **transaction-screening-workflow-concepts** - tx hash / transfer-level screening, **deposit** vs **withdrawal**, STR-style exports.

**Goal:** a portable mental model of **address screening** UIs and **list** semantics aligned with common compliance products, without binding a specific **vendor** implementation.
