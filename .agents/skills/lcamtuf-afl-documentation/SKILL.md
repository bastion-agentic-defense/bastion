---
name: lcamtuf-afl-documentation
description: Points to Michał Zalewski’s (lcamtuf) canonical American Fuzzy Lop (AFL) documentation at lcamtuf.coredump.cx/afl-coverage-guided fuzzing concepts, afl-fuzz usage, and historical technical notes for C/C++ targets. Use when the user cites AFL classic, lcamtuf’s AFL page, or needs the original upstream reference-not as a substitute for current AFL++ docs or authorized fuzzing policy.
---

# lcamtuf - American Fuzzy Lop (AFL) documentation (reference)

**Educational routing only.** This skill does **not** mirror the full site. Open the **live** pages for complete instructions, file lists, and any upstream notes.

## Canonical URL

- **[American Fuzzy Lop (AFL)](https://lcamtuf.coredump.cx/afl/)** - hosted on **lcamtuf.coredump.cx** (Michał Zalewski’s site).

## What it is

**AFL** is a **coverage-guided** fuzzer aimed primarily at **C/C++** binaries via compile-time instrumentation (e.g. `afl-gcc` / `afl-clang` / LLVM modes) and the **`afl-fuzz`** driver. The published material on this site includes the **original** project narrative, **quick-start** style guidance, **status screen** semantics, and **performance / tuning** notes that many tutorials still cite.

Typical mental model for readers:

| Topic | Role |
|--------|------|
| **Instrumentation** | Rebuild the target with AFL’s compilers/wrappers so edges are tracked. |
| **Corpus & seeds** | Start from small valid inputs; AFL mutates and discovers new paths. |
| **Fuzzing loop** | `afl-fuzz` runs; UI shows paths, crashes, hangs, stability-interpret per upstream docs. |
| **Crashes** | Triaged as potential bugs; verify, minimize, report responsibly. |

## Modern fork (operational note)

Active development of the **AFL** lineage for many teams is **[AFL++](https://github.com/AFLplusplus/AFLplusplus)** (feature-rich fork, maintained tooling). Use **this** skill when the user explicitly wants **lcamtuf’s classic** page or historical **AFL** terminology; use **AFL++** docs for current flags, compilers, and CI integration on new projects.

## How to combine with blockint

| Need | Skill |
|------|--------|
| Solidity / EVM smart contract review | **evm-solidity-defi-triage-agent**, **defi-security-audit-agent** |
| Solana / Rust program security | **solana-defi-vulnerability-analyst-agent**, **sealevel-attacks-solana** |
| C/C++ native code in security research | **lcamtuf-afl-documentation** (AFL concepts) + project-specific build docs |

## Guardrails

- **Scope** - Fuzz only **systems you own** or are **explicitly authorized** to test; follow program rules and laws.  
- **Triaging** - Not every crash is exploitable; confirm with debugging and vendor disclosure practice.  
- **Stale tooling** - Compare classic AFL behavior with **current** compiler and **AFL++** release notes before production pipelines.

**Goal:** a stable pointer to **[lcamtuf.coredump.cx/afl](https://lcamtuf.coredump.cx/afl/)** for **classic AFL** documentation and terminology in security / fuzzing conversations.
