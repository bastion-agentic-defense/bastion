---
name: chainalysis-sanctions-screening
description: Routes to Chainalysis public Sanctions Screening surfaces-the free Sanctions REST API (address checks against OFAC SDN crypto listings) and the EVM on-chain oracle-plus how they differ from paid Chainalysis customer data. Use when the user references Chainalysis sanctions tooling, public.chainalysis.com, or repo notes in Chainalysis.md-not for legal conclusions, full vendor ToU reproduction, or substituting official government sanctions lists.
---

# Chainalysis - public sanctions screening (reference)

**Educational routing only.** This skill does **not** reproduce Chainalysis **Terms of Use**, **Privacy Policy**, or other legal text. For binding terms, rate limits, signup URLs, and contract addresses, use **Chainalysis’s current public documentation** and dashboards.

## What the public product family covers (high level)

| Surface | Role (conceptual) |
|---------|-------------------|
| **Sanctions API** | **REST** service: check whether a **cryptocurrency address** appears on **sanctions-related** listings Chainalysis exposes for this API (commonly described in relation to **OFAC SDN** crypto addresses). Authenticate with an **API key** (typically `X-API-Key` header). |
| **Oracle (EVM)** | **On-chain** smart contract aimed at dApps that need **in-protocol** screening-upstream docs describe a narrower scope than the API (e.g. **Ethereum/EVM** address checks vs API response shape). |

**Customers:** If you use paid Chainalysis products, upstream docs note you may receive **broader** identifications (e.g. **Chainalysis Identifications**, clustering beyond what the free public API exposes). Treat **customer** docs and contracts as authoritative for your org.

## Integration facts to verify live (do not trust stale copies)

- **No browser CORS:** Public docs state the API is **not** meant for direct **cross-origin** browser calls-use a **backend** or server-side caller.
- **Authentication:** Requests typically send `X-API-KEY` / `X-API-Key` (confirm header name in current docs).
- **Example host pattern** seen in public materials: `https://public.chainalysis.com/api/v1/address/<address>` - **paths and versions change**; copy from official docs before shipping.
- **Rate limits:** Documentation cites high-volume windows (e.g. thousands of requests per few minutes)-**confirm the current limit** in official docs; **429/403** handling belongs in your integration.
- **Support (public channel listed in materials):** `sanctions-api-support@chainalysis.com` - verify on the **live** support page.

## Local excerpt in this repository

The repo may include **`Chainalysis.md`** - a **local excerpt / scratch** of public-facing API and terms text. Use it only as a **convenience pointer**; **integration details** (endpoints, fields, legal terms) must match **current** Chainalysis publications.

## How to use with blockint

| Need | Skill |
|------|--------|
| Ethical investigation workflow, reporting, sanctions **process** context | **crypto-investigation-compliance** |
| Sanctions + risk tools via Range MCP (separate product) | **range-ai-investigation-playbook** |
| Risk indicator / exposure vocabulary (educational) | **risk-exposure-screening-concepts** |
| Address/transaction screening **workflow** concepts (educational) | **address-screening-workflow-concepts**, **transaction-screening-workflow-concepts** |
| AML / analytics platforms (vendor-neutral) | **blockchain-analytics-operations** |

## Guardrails

- **Sanctions compliance is legal and jurisdictional** - API output is **not** a substitute for **OFAC** / **EU** / **UN** lists, **licensed counsel**, or your institution’s **compliance program**.
- **Accuracy:** Vendor docs disclaim **guarantees** on timeliness and completeness-design **human review** and **primary-source** checks for material decisions.
- **Secrets:** Never commit or paste **API keys** or **internal** investigation exports into public repos or chats.
- **Prohibited use:** Respect Chainalysis **API Terms** (no scraping, no abuse, no competing “clone” of their product features-read the **live** terms).

**Goal:** a neutral blockint router for **Chainalysis public Sanctions Screening** surfaces and the optional **`Chainalysis.md`** excerpt, without mirroring vendor legal text or implying non-public product scope.
