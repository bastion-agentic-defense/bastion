---
name: bellingcat-investigation-toolkit
description: Routes investigators to Bellingcat's Online Open Source Investigation Toolkit (tool catalog, GitBook, GitHub repo) for discovering and citing OSINT tools and workflows beyond chain-only work. Use when the user asks for Bellingcat toolkit, general OSINT tool discovery, open-source investigation resources, or methodology pointers to complement blockchain tracing-not for bypassing laws, harassment, or non-consensual deanonymization.
---

# Bellingcat investigation toolkit

## What it is

[Bellingcat’s Online Open Source Investigation Toolkit](https://github.com/bellingcat/toolkit) is the GitHub home of **Online Investigation Toolkit 2.0**: a curated, living collection of OSINT tools and references. Markdown for each tool is managed on GitBook and synced to the repository; automated workflows regenerate lists and pages. The public book is at [bellingcat.gitbook.io/toolkit](https://bellingcat.gitbook.io/toolkit).

**This skill does not mirror the full tool list**-entries, URLs, and categories change. Prefer live GitBook or GitHub `src` / generated pages when recommending a specific tool.

## When to use this skill

- Broaden OSINT beyond on-chain work: maps, imagery, social, archives, and other categories covered in the toolkit.
- Answer “what tool exists for X?” by pointing to the catalog and primary tool documentation, not by inventing tool names.
- Pair with **crypto-investigation-compliance** and **on-chain-investigator-agent** for ethical workflow; chain tracing remains in the other **blockint-skills** entries.

## How agents should apply it

1. Cite the toolkit as a directory, not an endorsement of every listed tool’s current safety or terms of service.  
2. Verify links and capabilities on the official page before prescribing steps (API keys, rate limits, jurisdiction).  
3. Separate fact (a tool exists) from advice (whether a use case is lawful for the user).  
4. For crypto-only tasks, load **solana-tracing-specialist**, **address-clustering-attribution**, and related skills first; use the Bellingcat toolkit when non-chain OSINT is in scope.

## Ethical guardrails

- Follow lawful, ethical open-source investigation practice: no harassment, stalking, non-consensual doxxing, or sanctions evasion. Align with **crypto-investigation-compliance** and the repository’s Code of Conduct on [GitHub](https://github.com/bellingcat/toolkit).  
- OSINT can harm people when misused; prefer accuracy, proportionality, and escalation to professionals where appropriate.

## Official references

| Resource | URL |
|----------|-----|
| GitHub repository | [github.com/bellingcat/toolkit](https://github.com/bellingcat/toolkit) |
| GitBook (toolkit) | [bellingcat.gitbook.io/toolkit](https://bellingcat.gitbook.io/toolkit) |

**Goal:** Give agents a stable pointer to Bellingcat’s investigation toolkit so users find up-to-date OSINT resources without embedding a stale tool list in this repo.
