---
name: neodyme-solana-security-workshop
description: Points to Neodyme’s public Solana Security Workshop (workshop.neodyme.io) and its open-source mdBook source on GitHub (neodyme-labs/neodyme-breakpoint-workshop)-hands-on levels, PoC framework, and intentionally vulnerable programs for learning attack and defense on Solana. Use when the user names Neodyme workshop, the Breakpoint workshop repo, or wants a structured Solana program security curriculum alongside sealevel-attacks-solana-not for reusing vulnerable code in production or attacking live systems without authorization.
---

# Neodyme - Solana Security Workshop (reference)

**Educational routing only.** Content and challenges live on the **live** site; follow the workshop’s **Legal Notice** and setup instructions there.

## Canonical URL

- **[Solana Security Workshop](https://workshop.neodyme.io/)** - hosted by **[Neodyme](https://neodyme.io/)** (security research firm; has supported Solana ecosystem peer reviews per workshop intro).

## Source repository (mdBook)

The static site is built from the public repo:

- **[github.com/neodyme-labs/neodyme-breakpoint-workshop](https://github.com/neodyme-labs/neodyme-breakpoint-workshop)** - **Solana Security Workshop for Breakpoint** (Neodyme); contains **`docs/`** (mdBook content), **`level0`–`level4`** / **`pocs`** challenge layout, **Docker** files, and **`book.toml`**.

Per upstream [README](https://github.com/neodyme-labs/neodyme-breakpoint-workshop/blob/main/README.md): install **mdBook** (`cargo install mdbook`), then `mdbook serve` to browse locally; all narrative detail is under **`docs/`**.

## What it is

The workshop teaches Solana **on-chain programs** from an **attacker-minded** perspective: understanding exploitation paths to write **safer** code. The published outline includes:

- **Introduction** and general security concepts  
- **Setup** and a **PoC framework**  
- **Level 0** - first vulnerability (presentation + solution)  
- **Levels 1–4** - progressive challenges (e.g. vaults, tip pool, **SPL token** vault), each with hints, bug discussion, and solution sections  

Upstream text stresses that **example code is intentionally vulnerable** and **not** production-quality-**do not** ship it as-is.

## Prerequisites (per site)

- Familiarity with **Rust** and **writing Solana programs**  
- A environment that can **compile** the examples and run **attacks** (the workshop documents **Setup**, including optional prebuilt environments)

## How to combine with blockint

| Need | Skill |
|------|--------|
| Anchor-centric exploit/mitigation snippets | **sealevel-attacks-solana** |
| Solana DeFi audit posture and review | **solana-defi-vulnerability-analyst-agent** |
| Infra for local testing / docs indexes | **solana-onchain-intelligence-resources** |
| Forensic reconstruction of incidents | **solana-tracing-specialist**, **flash-loan-exploit-investigator-agent** |

## Guardrails

- **Authorized environments only** - run challenges on **local** / **test** clusters unless the workshop explicitly targets public networks.  
- **No theft or harassment** - skills are for **learning** and **defensive** security.  
- **Responsible disclosure** - findings in **third-party** mainnet programs follow project **bug bounty** / disclosure rules.  
- **Legal** - comply with computer misuse and platform terms in your jurisdiction.

**Goal:** stable pointers to **[workshop.neodyme.io](https://workshop.neodyme.io/)** and **[neodyme-breakpoint-workshop](https://github.com/neodyme-labs/neodyme-breakpoint-workshop)** for **structured Solana program security** practice and **local/offline** mdBook builds inside blockint.
