---
name: sealevel-attacks-solana
description: Points to the coral-xyz sealevel-attacks repository-minimal Anchor programs demonstrating common Solana (Sealevel) exploit patterns and recommended mitigations. Use when auditing or learning Solana program security, pairing with solana-defi-vulnerability-analyst-agent-not for deploying attacks against live systems or evading law.
---

# Sealevel Attacks - Solana exploit patterns (reference)

**Reference skill.** This bundle does **not** vendor the repo; clone and build from [upstream](https://github.com/coral-xyz/sealevel-attacks).

- **Repository:** [github.com/coral-xyz/sealevel-attacks](https://github.com/coral-xyz/sealevel-attacks)  
- **Maintainer context:** Published under the **Coral** org (Anchor ecosystem).  
- **Stack:** **Anchor** / Rust programs under `programs/`, tests, migrations-see [README](https://github.com/coral-xyz/sealevel-attacks/blob/master/README.md).

## What it is

Upstream describes **examples of common exploits** that arise from the **Solana programming model** (account model, CPIs, sysvars, etc.), plus **idioms** to avoid them using **Anchor**. Each example is **intentionally incomplete**: one **isolated** issue and fix per program-not a production template.

Use it to **recognize** vulnerability classes when reading DeFi code or **post-mortems**, not as a copy-paste base for new protocols.

## How to combine with blockint

| Task | Skill |
|------|--------|
| Solana DeFi review posture, Anchor/PDAs/CPIs | **solana-defi-vulnerability-analyst-agent** |
| Incident narratives, tx reconstruction | **solana-tracing-specialist**, **flash-loan-exploit-investigator-agent** |
| Broader DeFi / rug triage | **defi-security-audit-agent** |
| Surfpool / local testing stacks | **solana-onchain-intelligence-resources** (Helius docs index) |

## Guardrails

- **Authorized use only** - study and **defensive** coding in **devnet**/local environments; **no** guidance for exploiting **third-party** mainnet programs.  
- **Legal** - unauthorized access or theft of funds is **criminal** in most jurisdictions; this skill is **education and audit support** only.  
- **Completeness** - patterns evolve with runtime and program versions; treat the repo as **illustrative**, not an exhaustive checklist.  
- **Disclosure** - if you find a bug in a **live** project, follow **responsible disclosure** and the project’s policy.

**Goal:** a discoverable pointer to **[sealevel-attacks](https://github.com/coral-xyz/sealevel-attacks)** for Solana **security** learning and audits inside blockint.
