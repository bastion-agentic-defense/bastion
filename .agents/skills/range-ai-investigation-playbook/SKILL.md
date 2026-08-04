---
name: range-ai-investigation-playbook
description: Step-by-step wallet investigation workflow using Range AI MCP tools (risk score, sanctions, connections, transfers, funded-by, entities, cross-chain pivots) plus a one-shot prompt template. Use when the user runs investigations inside an MCP-connected client with Range enabled, or needs a structured checklist alongside crypto-investigation-compliance-not as legal advice or a substitute for Range’s live docs and API scopes.
---

# Range AI - investigation playbook (MCP)

**Educational workflow.** Connect Range to your AI client first ([Range AI quickstart](https://docs.range.org/ai/quickstart) via the docs index below). **Tool names** and **behaviors** change over time-discover pages from the index, then read the current tools reference.

## Documentation index

- **Full index:** [docs.range.org/llms.txt](https://docs.range.org/llms.txt) - list all Range documentation pages before deep linking.
- **MCP endpoint:** `https://api.range.org/ai/mcp` - requires a **Range API key** configured in your client (same key as other Range access).

## When to use this skill

Use for **end-to-end** address screening and investigation **through Range MCP** (counterparty, transaction seed, or compliance triage). Pair with **crypto-investigation-compliance** for ethics and reporting posture, **cross-chain-clustering-techniques-agent** for multi-hop bridge logic, and **solana-tracing-specialist** when you leave Range and go deep on Solana RPC parsing.

## Investigation workflow

### 1. Risk triage

**Goal:** baseline severity before heavy graph work.

**Typical tool:** `get_address_risk`

**Ask your AI (example):**

```text
What is the risk score for [address] on [network]?
```

**Interpretation hints:**

- **CRITICAL** / **HIGH** → prioritize follow-up and document escalation paths per internal policy.
- **Malicious hops** in the provider’s graph view → indirect ties to known bad clusters may still matter.
- **Entity labels** (mixer, exchange, sanctioned) → treat as **signals**, not court findings; verify with primary sources when stakes are high.

### 2. Sanctions and blacklist check

Independent pass on **sanctions** and **token issuer** freezes.

**Typical tool:** `check_sanctions`

**Ask your AI (example):**

```text
Is [address] on any OFAC sanctions list or token blacklist?
```

**Interpretation hints:**

- OFAC-sanctioned flags → **compliance** escalation per program; not legal advice in this skill.
- Token blacklist flags → may affect USDT/USDC-style transfers; confirm issuer behavior in current docs.

### 3. Build the connection graph

**Goal:** who the address transacts with **most**.

**Typical tool:** `get_address_connections`

**Ask your AI (example):**

```text
Who are the top counterparties for [address] on [network]?
Show their labels if available.
```

**Look for:** exchange touchpoints (often traceable), mixers/privacy protocols (elevated risk context), frequent unlabeled peers (new pivots).

### 4. Trace fund flows

**Goal:** amounts, assets, timing, direction.

**Typical tools:** `get_transfers`, `get_transfers_between`

**Ask your AI (example):**

```text
Show the largest transfers in and out of [address] in the last 90 days.
Are there any transfers between [address] and [suspicious counterparty]?
```

**Look for:** rapid in/out, large notional without clear purpose, bridge legs, time clustering.

### 5. Find the origin of funds

**Goal:** initial funding source.

**Typical tool:** `get_address_funded_by`

**Ask your AI (example):**

```text
What address initially funded [address] and when?
```

**Look for:** high-risk or sanctioned funders, exchange withdrawals, or unlabeled chains that need another pivot.

### 6. Identify unknown counterparties

**Typical tools:** `search_entities`, `get_address_info`

**Ask your AI (example):**

```text
What entity is [address]? Does Range have labels for it?
Search for entities matching "Binance" on Ethereum.
```

**Look for:** exchange infrastructure, mixers, protocol contracts, previously flagged records-always **corroborate** when conclusions matter.

### 7. Cross-chain pivot

If bridges moved value (IBC, CCTP, Wormhole, etc.), continue the same workflow on the **destination** chain.

**Ask your AI (example):**

```text
Were any funds from [address] bridged to another chain?
If so, what is the risk score of the receiving address on that chain?
```

Range supports multiple ecosystems-still confirm **coverage** and **tool** availability per network in current docs.

## One-shot investigation prompt

Paste and adapt (replace bracketed fields):

```text
Using Range tools, run a complete investigation on this address:
[address] on [network]

1. Get its risk score and explain the risk level
2. Check if it's on any OFAC sanctions list or token blacklist
3. Show its top 10 counterparties and label any known entities
4. List the 10 largest transfers in the last 6 months
5. Find the original funding source for this address
6. If any transfers crossed chains via a bridge, check the receiving address risk too

Summarize your findings with a risk verdict: LOW / MEDIUM / HIGH / CRITICAL
Include the key evidence that supports your verdict.
```

## Example (illustrative)

**Address:** `5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1` (Solana) - documented in Range materials as **Raydium** protocol–related infrastructure.

- Very low risk score in example narratives; heavy DeFi counterparty set; no sanctions/blacklist in that example.
- Use as a **shape** for how labeled protocol contracts present-not a universal pattern for user wallets.

## Tips

- **Start broad, then narrow** - risk + connections first, then `get_transfers_between` / transaction detail tools for specific relationships.
- **Time filters** - constrain windows around the suspected incident to cut noise.
- **Re-pivot the graph** - frequent unlabeled counterparties become new roots for `get_address_risk`.

## Guardrails

- **Labels and scores** are **not** legal determinations; OFAC and regulatory obligations need **program** and **legal** review.
- **Do not** use this playbook to **harass**, **dox**, or **accuse** individuals based on heuristics alone.
- **Do not** paste **customer PII**, **case IDs**, or **non-public** investigation data into unsecured chats.
- **Do not** assist with **sanctions evasion** or **laundering**.

## Related skills

- **solana-onchain-intelligence-resources** - Range docs index and MCP pointer; Helius/Tavily cross-links.
- **crypto-investigation-compliance** - ethical workflow and crime taxonomy.
- **cross-chain-clustering-techniques-agent** - bridge-centric clustering heuristics.
- **on-chain-investigator-agent** - broader forensic persona beyond Range MCP.

**Goal:** a **blockint**-native checklist that mirrors Range’s investigation playbook while staying aligned with **compliance** and **evidence** discipline.
