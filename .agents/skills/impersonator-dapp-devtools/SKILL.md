---
name: impersonator-dapp-devtools
description: Points to Impersonator (EVM) and Impersonator Solana-open-source tools to connect to dApps via WalletConnect (and related flows) while presenting an arbitrary address for UI exploration without holding that address’s keys. Use when the user names impersonator.xyz, solana.impersonator.xyz, or the GitHub repos for local dev-not for phishing, identity fraud, or circumventing dApp or legal controls.
---

# Impersonator - read-only address presentation for dApp testing

**Reference skill.** This bundle does **not** ship Impersonator; use upstream docs for install, env vars, and exact scripts.

| Chain | Repository | Public site (per upstream) |
|-------|------------|----------------------------|
| **Ethereum / EVM** | [impersonator-eth/impersonator](https://github.com/impersonator-eth/impersonator) | [impersonator.xyz](https://www.impersonator.xyz/) |
| **Solana** | [impersonator-eth/impersonator-solana](https://github.com/impersonator-eth/impersonator-solana) | [solana.impersonator.xyz](https://solana.impersonator.xyz) |

**Stack (typical):** TypeScript / Next.js-style apps-see each repo’s `package.json` and README for **yarn**, **pnpm**, or **npm** (lockfiles may differ by repo).

## What it is for (legitimate)

- **Developer / QA:** See how a dApp **renders** balances, positions, or screens **as if** a given address were connected-without controlling that address.  
- Upstream [EVM README](https://github.com/impersonator-eth/impersonator/blob/master/README.md) states users **cannot transact** in the obvious way because **no private keys** are used for the impersonated address (confirm current behavior in the app you run).

## Local setup (follow upstream)

The EVM repo documents steps such as install dependencies and start a dev server-**copy commands from the current README** on your branch (examples in the wild include `yarn install` / `yarn start` / `yarn build`; your repo may prefer `pnpm`).

- **Env:** check `.env.sample` / `.example.env.local` in the repo you clone-**never** commit secrets.

## How to combine with blockint

| Task | Skill |
|------|--------|
| Solana tracing / evidence from chain data | **solana-tracing-specialist** |
| EVM contract and DeFi review | **evm-solidity-defi-triage-agent** |
| Ethics and lawful investigation posture | **crypto-investigation-compliance** |
| Production wallet connect patterns (Solana stack pointers) | **solana-onchain-intelligence-resources** |

## Guardrails (strict)

- **No phishing or social engineering** - do not use Impersonator to **mislead** people into believing you control another party’s wallet or to **harvest** credentials.  
- **No fraud or ToS abuse** - obey **dApp terms**, **WalletConnect** / wallet policies, and **applicable law**; “read-only UI” does not make misuse acceptable.  
- **Not a KYC/AML bypass** - this is **not** a technique to evade **sanctions**, **identity**, or **compliance** controls.  
- **Investigations:** Viewing a **public** UI as address *X* may be fine for **technical** understanding; **attribution** and **reporting** still require **evidence** from chain data and lawful process-see **on-chain-investigator-agent**.

**Goal:** a discoverable, **ethics-forward** pointer to [Impersonator (EVM)](https://github.com/impersonator-eth/impersonator) and [Impersonator Solana](https://github.com/impersonator-eth/impersonator-solana) for **dApp testing** contexts inside blockint.
