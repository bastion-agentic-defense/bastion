---
name: osec-solana-auditor-introduction
description: "Points to Otter Sec (osec.io) blog post “Solana: An Auditor’s Introduction”-runtime-oriented primer on Sealevel execution, accounts, BPF entrypoint deserialization, attacker-controlled vs runtime-enforced fields, and native programs (e.g. System Program). Use when the user cites this article or needs fundamentals next to neodyme-solana-security-workshop and sealevel-attacks-solana-not as current protocol spec (verify Solana docs and post-2022 runtime changes)."
---

# Osec - Solana: An Auditor’s Introduction (reference)

**Educational routing only.** This skill does **not** reproduce the full article or code excerpts. Read the **live** post for complete detail.

## Canonical URL

- **[Solana: An Auditor’s Introduction](https://osec.io/blog/2022-03-14-solana-security-intro)** - [osec.io](https://osec.io/) blog, **14 March 2022**.

**Publisher:** [Otter Sec](https://osec.io/) / Otter Audits LLC (smart contract security audits; per site footer).

## What the article covers (outline)

The post frames **Solana program security** from a **researcher / auditor** lens: how the **runtime** executes contracts, where **trust boundaries** sit, and what an attacker can influence.

| Theme | Topics (high level) |
|-------|---------------------|
| **Execution model** | Programs as **eBPF ELF** loaded via **BPF Loader**; accounts as **pubkey-addressed** state; invocations = program id + **account list** + **instruction data**; no EVM-style “methods” at the syscall boundary-dispatch via **instruction bytes** (e.g. enums). **Memory map** regions (code/stack/heap/inputs) at a glance; **Rust** reducing typical memory-corruption focus. |
| **Entry / deserialization** | Common **`entrypoint`** and deserialization helpers; distinction between data **serialized by the runtime** vs fields under **attacker control** (e.g. **instruction data**; **account list** selection)-and metadata enforced by the runtime (**signers**, **writable**, **owner**). Links **type confusion** / wrong-account issues to lack of execution-level typing (mitigations such as **hardcoded pubkeys**, **discriminators**-often formalized today via **Anchor**). |
| **Native programs** | **System Program** and illustrative **CreateAccount** vs **Transfer** **signer** requirements; **ownership** constraints (e.g. SPL token accounts vs system-owned accounts). |

The introduction explicitly recommends **[Neodyme’s Security Workshop](https://workshop.neodyme.io/)** for hands-on vulnerability classes; this article complements it with **fundamentals-first** runtime context.

## How to combine with blockint

| Need | Skill |
|------|--------|
| Hands-on vulnerable levels | **neodyme-solana-security-workshop** |
| Anchor exploit/mitigation snippets | **sealevel-attacks-solana** |
| Audit / review workflow | **solana-defi-vulnerability-analyst-agent** |
| Doc indexes & tooling | **solana-onchain-intelligence-resources** |

## Guardrails

- **Staleness** - published **2022**; Solana **runtime**, **program IDs**, and **best practices** evolve-cross-check **[Solana documentation](https://solana.com/docs)** and current **SIMD** / release notes for material facts.  
- **Not a specification** - blog is **pedagogical**; final authority is **source code** and **on-chain behavior**.  
- **Ethics** - use understanding for **defense**, **audits**, and **responsible disclosure**, not theft or unauthorized exploitation.

**Goal:** a stable pointer to **[osec.io Solana security intro](https://osec.io/blog/2022-03-14-solana-security-intro)** for **runtime-aware** Solana security reasoning in blockint.
