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

1. **Transaction Interception**, All Solana transactions pass through policy checks before signing
2. **Helius Simulation**, State change prediction via Helius API for Solana
3. **EVM Simulation**, Celo eth_call simulation for EVM transactions
4. **Program Allowlists**, Only approved programs can be called (whitelist mode)
5. **Native Token Caps**, Configurable SOL per transaction and 24h limits
6. **Rate Limiting**, Per-minute transaction frequency caps
7. **Balance Drain Detection**, Blocks transactions exceeding configured lamport drain limits
8. **Emergency Pause**, Fleet-wide circuit breaker via `/circuit-breaker/engage`

### Daemon BlockInt Security Checks

9. **Flash Loan Detection**, Flags near-equal large inflow/outflow within same atomic transaction
10. **High Slippage Detection**, Blocks trades exceeding configurable basis point threshold (default 500 bps)
11. **Mint Authority Change Block**, Prevents unauthorized SPL token mint authority transfers
12. **Freeze Authority Change Block**, Prevents unauthorized token freeze authority transfers
13. **Risk Labeled Address Screening**, Blocks transactions involving flagged addresses from GrondOSINT
14. **Intent Classification**, Analyzes transaction intent descriptions for malicious patterns

### Agent Identity and Access Control

15. **W3C DID Identity**, Every agent receives a did:bastion identifier with cryptographic verification
16. **On-Chain Agent Registry**, Anchor PDA accounts with name, capability bitmap, reputation score
17. **DID Authentication**, Nonce challenge-response with Ed25519 signature verification
18. **Delegation Constraints**, Max 3 levels deep, child capabilities must be subset of parent
19. **Delegation Budget**, Per-sub-agent lamport ceilings with running spend counters
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

28. **On-Chain Audit Trail**, Anchor program writes every decision to Solana as immutable records
29. **Local Audit Logging**, Sled DB for fast local querying with pagination and filtering
30. **SSE Event Stream**, Real-time audit events via `/events` (Server-Sent Events)
31. **Human Override Queue**, Blocked transactions held for human review with UUID tracking
32. **Case Management**, Investigation workflow with evidence attachment and status tracking

## Known Issues

### Solana SDK Transitive Dependencies

Bastion pins the Solana 1.18.26 SDK (required by Anchor 0.30.1), which carries transitive RustSec advisories. These are inherited from the SDK, not Bastion's own code, and are allowlisted in `.cargo/audit.toml` pending a Solana SDK bump (tracked in `docs/MAINNET_READINESS.md` §5):

- **RUSTSEC-2026-0204** — crossbeam-epoch 0.9.18 (invalid pointer deref)
- **RUSTSEC-2024-0344** — curve25519-dalek 3.2.1 (timing variability)
- **RUSTSEC-2022-0093** — ed25519-dalek 1.0.1 (double-pubkey signing oracle)
- **RUSTSEC-2026-0258** — h2 0.3.27 / 0.4.14 (unbounded empty DATA frames)
- **RUSTSEC-2026-0037** — quinn-proto 0.10.6 (DoS, high)
- **RUSTSEC-2026-0185** — quinn-proto 0.10.6 (remote memory exhaustion, high)
- **RUSTSEC-2025-0009** — ring 0.16.20 (AES panic under overflow checks)
- **RUSTSEC-2026-0098 / -0099 / -0104** — rustls-webpki 0.101.7 (name-constraint / wildcard / CRL panics)
- **RUSTSEC-2026-0009** — time 0.3.36 (DoS via stack exhaustion)

**Mitigation**: Allowlisted in `.cargo/audit.toml`; the CI `audit` job still fails on any *new* advisory. Resolve via a Solana SDK upgrade before go-live.

## Threat Model

Bastion protects against six threat actor classes:

1. **Compromised Agent**, LLM manipulated through prompt injection, trust runtime provides the final policy enforcement layer
2. **Malicious Operator**, On-chain policy lives where operator cannot modify it unilaterally
3. **Policy Bypass**, Aggregate behavioral analysis with sliding window counters
4. **Intent Observer** (Arcium MXE — *preview, not yet enforcing*): once a live MXE client ships, MPC confidentiality will prevent strategy extraction from transaction metadata. Today the no-op client means evaluation is **not** confidential (see `docs/MAINNET_READINESS.md` §6).
5. **Cross-Chain Correlator** (Base spoke), Randomized delays and batching obscure cross-chain patterns
6. **Governance Attacker**, Time-locked multisig policy upgrades prevent hostile governance capture

## Disclosure Timeline

We aim to respond to vulnerability reports within 48 hours. Critical issues will receive a patch within 7 days. We publish security advisories on our GitHub repository and npm packages.
