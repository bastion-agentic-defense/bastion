---
name: agentskills-specification
description: "Points to the official Agent Skills open-format-agentskills/agentskills on GitHub, agentskills.io docs (including llms.txt index: Overview, What are skills?, Specification, Integrate skills into your agent). Use when the user asks how Agent Skills work, SKILL.md structure, integration, contributing, or aligning blockint skills with the upstream format-not as a substitute for reading the live specification and LICENSE terms in the upstream repo."
---

# Agent Skills - open format (reference)

**Educational routing only.** This skill does **not** reproduce the specification. Use the **canonical** links below for authoritative text, schema details, and license terms.

## Canonical URLs

| Resource | URL |
|----------|-----|
| **Repository** | [github.com/agentskills/agentskills](https://github.com/agentskills/agentskills) |
| **Site** | [agentskills.io](https://agentskills.io) - documentation hub (per upstream README) |

The upstream project describes **Agent Skills** as a simple, open format: folders of instructions, scripts, and resources that agents can discover for task-specific expertise (“write once, use everywhere”). The repo holds the **specification**, **documentation**, **reference SDK** material, and pointers to **example skills** and community channels (e.g. Discord - see live README).

## Documentation index ([llms.txt](https://agentskills.io/llms.txt))

Use **[agentskills.io/llms.txt](https://agentskills.io/llms.txt)** as the machine-readable **index** of first-party docs (discover pages before deep-linking). It currently lists:

| Doc | URL |
|-----|-----|
| **Overview** | [agentskills.io/home.md](https://agentskills.io/home.md) - open format for agent capabilities and expertise |
| **What are skills?** | [agentskills.io/what-are-skills.md](https://agentskills.io/what-are-skills.md) - lightweight format for specialized knowledge and workflows |
| **Specification** | [agentskills.io/specification.md](https://agentskills.io/specification.md) - complete format specification |
| **Integrate skills into your agent** | [agentskills.io/integrate-skills.md](https://agentskills.io/integrate-skills.md) - adding Agent Skills support to an agent or tool |

## What to read there

| Area | Typical use |
|------|----------------|
| **Specification** | [specification.md](https://agentskills.io/specification.md) - exact format expectations for portable skills |
| **Integration** | [integrate-skills.md](https://agentskills.io/integrate-skills.md) - agent/tool implementers |
| **Documentation** | [home.md](https://agentskills.io/home.md), [what-are-skills.md](https://agentskills.io/what-are-skills.md) - concepts |
| **Example skills** | Patterns and breadth (see GitHub README + examples) |
| **Contributing** | [CONTRIBUTING.md](https://github.com/agentskills/agentskills/blob/main/CONTRIBUTING.md) in-repo |

## License (summary)

Upstream states: code in the repository is under **Apache-2.0**; documentation under **CC-BY-4.0** - confirm in-repo **LICENSE** and per-directory notices. The format is **community-open**; Anthropic is named as a maintainer in the public README.

## How to combine with blockint

| Need | Skill |
|------|--------|
| This repo’s BI / chain-focused skills | **blockchain-intelligence-playbook** |
| Portable skill format vs domain content | **agentskills-specification** (upstream spec); topical skills here for blockchain intel |

## Guardrails

- **Upstream wins** - If blockint layout ever diverges from the published spec, treat **agentskills** as the portability reference.  
- **No credential paste** - Spec work does not require embedding secrets in skills.

**Goal:** stable pointers to **[agentskills/agentskills](https://github.com/agentskills/agentskills)**, **[agentskills.io/llms.txt](https://agentskills.io/llms.txt)**, and **[agentskills.io/specification.md](https://agentskills.io/specification.md)** for **Agent Skills** format and documentation alignment.
