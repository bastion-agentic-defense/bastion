# Security Policy

> Bastion is in alpha testing. Use with caution in production environments.

## Supported Versions

| Version | Supported |
|---------|-----------|
| latest | Yes |

## Reporting a Vulnerability

If you find a security vulnerability, please do NOT open a public issue. Report it to hello@bastionagentique.com. We appreciate responsible disclosure and will acknowledge receipt within 48 hours.

## Security Architecture

Bastion provides multiple defense layers across blockchain transactions and Web2 API calls.

### Blockchain Transaction Security

1. **Transaction Interception**, All EVM transactions pass through policy checks before broadcast
2. **EVM Simulation**, Per-chain `eth_call` simulation (Sepolia, Base, Celo, zkSync, Robinhood, Monad) before execution
3. **Program / Target Allowlists**, Only approved targets can be called (whitelist mode)
4. **Value Caps**, Configurable per-transaction and 24h value limits
5. **Rate Limiting**, Per-minute transaction frequency caps
6. **Balance Drain Detection**, Blocks transactions exceeding configured drain limits
7. **Emergency Pause**, Fleet-wide circuit breaker via `/circuit-breaker/engage`

### Daemon BlockInt Security Checks

9. **Flash Loan Detection**, Flags near-equal large inflow/outflow within same atomic transaction
10. **High Slippage Detection**, Blocks trades exceeding configurable basis point threshold (default 500 bps)
11. **Mint Authority Change Block**, Prevents unauthorized SPL token mint authority transfers
12. **Freeze Authority Change Block**, Prevents unauthorized token freeze authority transfers
13. **Risk Labeled Address Screening**, Blocks transactions involving flagged addresses from GrondOSINT
14. **Intent Classification**, Analyzes transaction intent descriptions for malicious patterns

### Agent Identity and Access Control

15. **W3C DID Identity**, Every agent receives a did:bastion identifier with cryptographic verification
16. **On-Chain Agent Registry**, EVM registry accounts with name, capability bitmap, reputation score
17. **DID Authentication**, Nonce challenge-response with Ed25519 signature verification
18. **Delegation Constraints**, Max 3 levels deep, child capabilities must be subset of parent
19. **Delegation Budget**, Per-sub-agent value ceilings with running spend counters
20. **ERC-8004 Identity**, Soulbound ERC-721 tokens with EIP-712 wallet binding for EVM agents

### Web2 API Firewall Security

21. **Endpoint Allowlists**, Only approved URL paths and HTTP methods can be called
22. **Endpoint Blocklists**, Blocks API calls to prohibited patterns
23. **Provider Budgets**, Spending caps per provider per time window
24. **Content Inspection**, Detects PII, API keys (sk-*, ghp_*, github_pat_*), and prompt injection
25. **Header Filtering**, Blocks or allows specific HTTP headers on outbound requests
26. **Rate Limiting**, Per-provider request frequency caps
27. **OpenAPI Auto-Configuration**, Parses OpenAPI 3.0 specs to auto-generate allowlist rules

### Audit and Compliance

28. **On-Chain Audit Trail**, EIP-712 signed audit entries written on EVM as immutable records
29. **Local Audit Logging**, Sled DB for fast local querying with pagination and filtering
30. **SSE Event Stream**, Real-time audit events via `/events` (Server-Sent Events)
31. **Human Override Queue**, Blocked transactions held for human review with UUID tracking
32. **Case Management**, Investigation workflow with evidence attachment and status tracking

## Known Issues

### Draft ERC integrations (ERC-8354, ERC-8380)

The confidential-verdict (ERC-8354) and unclonable-credential (ERC-8380)
integrations track the current **draft** specs (see `docs/ERCS.md`) and are
experimental, feature-flagged, and ABI-pinned to a draft commit. They are not
yet covered by an external audit. The `"ERC-1953/..."` domain-separator tag
strings in ERC-8380 are frozen for compatibility.

> **Retired:** the Solana SDK transitive-advisory allowlist was removed when the
> legacy `solana-sdk`/Anchor-dependent `crates/solana` crate left the workspace
> (see `docs/ARCHIVE.md`). Solana *settlement* was restored separately via a
> lightweight `bs58` + `reqwest` RPC client (no `solana-sdk`), so the dependency
> graph stays small. `cargo audit` now fails on any advisory present in the
> (EVM + lightweight Solana RPC) dependency graph.

## Threat Model

Bastion protects against six threat actor classes:

1. **Compromised Agent**, LLM manipulated through prompt injection, trust runtime provides the final policy enforcement layer
2. **Malicious Operator**, On-chain policy lives where operator cannot modify it unilaterally
3. **Policy Bypass**, Aggregate behavioral analysis with sliding window counters
4. **Intent Observer** (ERC-8354 confidential verdicts — *draft, not yet enforcing*): once the ZK verdict path ships, a committed (secret) policy is proven without revealing the rules that permitted the action. See `docs/ERCS.md`.
5. **Cross-Chain Correlator** (Base spoke), Randomized delays and batching obscure cross-chain patterns
6. **Governance Attacker**, Time-locked multisig policy upgrades prevent hostile governance capture

## Disclosure Timeline

We aim to respond to vulnerability reports within 48 hours. Critical issues will receive a patch within 7 days. We publish security advisories on our GitHub repository and npm packages.
