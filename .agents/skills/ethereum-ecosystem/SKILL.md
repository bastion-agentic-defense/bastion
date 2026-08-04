---
name: ethereum-ecosystem
description: >-
  Comprehensive map of Ethereum trust primitives Bastion orchestrates.
  Covers wallets (Safe, EOA, AA), identity (ERC-8004, ENS, A2A),
  payments (x402, Pact), attestations (EAS, Sign, zkTLS), shared trust
  (EigenLayer), oracles (Chainlink, Pyth), bridges (Across, LayerZero),
  indexing (The Graph, Blockscout MCP), agent platforms (Olas, Virtuals),
  and developer tooling (ethskills, Base MCP, Foundry, Scaffold-ETH 2).
  Use when building on Ethereum trust standards.
---

# Ethereum Ecosystem Standards for Bastion

Bastion is the **Programmable Trust Runtime** - it orchestrates, not replaces, Ethereum's trust primitives. This skill maps every standard, protocol, and tool Bastion composes.

---

## 1. Agent Identity & Reputation

### ERC-8004 - Agent Identity Registry
**Deployed:** January 29, 2026. On 20+ chains. Same addresses everywhere.

- **IdentityRegistry:** `0x8004A169FB4a3325136EB29fA0ceB6D2e539a432`
- **ReputationRegistry:** `0x8004BAa17C55a88189AE136b182e5fdA19dE9b63`
- **ValidationRegistry:** Crypto-economic, zkML, TEE attestation models
- Agent identity = ERC-721 NFT. Multi-dimensional reputation (uptime, quality, success rate).
- **Resource:** https://www.8004.org | https://eips.ethereum.org/EIPS/eip-8004

### ERC-8126 - AI Agent Verification
Standardized verification results and risk scores consumed during policy evaluation.
- **Resource:** https://eips.ethereum.org/EIPS/eip-8126

### ENS - Ethereum Name Service
Human-readable names for agent identities (`agent.eth`). Composable with ERC-8004.
- **Resource:** https://ens.domains

### A2A - Agent-to-Agent Protocol (Google)
Standard for autonomous agent discovery and communication. ERC-8004's service registry maps A2A endpoints.
- **Resource:** https://github.com/google/A2A

### Privado ID / Polygon ID - Decentralized Identity
Privacy-preserving identity for agents and users. VC-based claims (KYC, credentials).
- **Resource:** https://privado.id

---

## 2. Wallets & Account Abstraction

### ERC-4337 - Account Abstraction
UserOperations, bundlers, paymasters. EntryPoint v0.7: `0x0000000071727De22E5E9d8BAf0edAc6f37da032`.
Major implementations: Kernel (ZeroDev), Biconomy, Alchemy Account Kit, Pimlico.

### ERC-7579 - Modular Smart Accounts
Validators, executors, hooks. Bastion's policy validator plugs as an ERC-7579 module.

### EIP-7702 - Smart EOAs (Live May 2025)
EOAs get smart-contract delegation without migration. Batching, gas sponsorship, session keys.
Still early for production agents - use standard EOAs or Safe until tooling matures.

### Safe (Gnosis Safe) - Multisig Wallets
**For AI agents:** 2-of-3 Safe (agent hot wallet + human hot wallet + human cold wallet).
If agent key is compromised, human removes it. Agent can batch transactions.
- **Singleton:** `0x41675C099F32341bf84BFc5382aF534df5C7461a` (same on all chains)
- **Resource:** https://docs.safe.global

### Coinbase Smart Wallet / Base Account
Embedded wallet with passkey auth. Used by Base MCP for agent wallets.
- **Resource:** https://www.smart-wallet.xyz

### Privy - Embedded Wallets
Wallet infrastructure for apps. Embedded wallets, email/social login, user-owned accounts.
- **Resource:** https://privy.io

### Dynamic - Wallet-Based Auth
Embedded wallets + multi-wallet auth + smart accounts.
- **Resource:** https://dynamic.xyz

### wagmi / viem - Frontend Wallet Libraries
React hooks (wagmi) + low-level primitives (viem). The standard stack for EVM dApp frontends.
- **Resource:** https://wagmi.sh | https://viem.sh

---

## 3. Payments & Settlement

### x402 - HTTP Payment Protocol (Coinbase)
Production-ready HTTP 402 micropayments. Server returns 402 Payment Required → client pays via EIP-3009 → server settles onchain. Bastion's Web2 firewall proxies x402 calls.
- **SDKs:** `@x402/core @x402/evm @x402/fetch @x402/express` (TS) | `pip install x402` (Python)
- **Resource:** https://www.x402.org

### Pact Network - Payment Refunds
On-chain chargeback for x402. Solana mainnet program `5bCJcdWdKLJ7arrMVMFh3z99rQDxV785fnD9XGcr3xwc`.
Premium per call → refund (principal+premium) on API failure from USDC coverage pool.
- **Resource:** https://pactnetwork.io/docs

### EIP-3009 - Transfer With Authorization
Gasless token transfers via signed authorizations. USDC implements it. Powers x402.
- **Resource:** https://eips.ethereum.org/EIPS/eip-3009

### ERC-2612 - Permit (Gasless Approvals)
Gasless token approvals via signatures. Widely adopted across DeFi.
- **Resource:** https://eips.ethereum.org/EIPS/eip-2612

---

## 4. Attestations & Provenance

### EAS - Ethereum Attestation Service
On-chain attestation registry. Bastion stores execution approvals, compliance attestations, human approvals, and policy outcomes.
- **Resource:** https://attest.org

### Sign Protocol - Cross-Chain Attestations
Portable trust records verifiable across ecosystems. Bastion emits Sign attestations for cross-chain trust.
- **Resource:** https://sign.global

### zkTLS / Reclaim / zkPass - Verified Web Data
Cryptographic proofs of web API responses. Bastion uses verified data as policy inputs (e.g., "this agent has a verified GitHub org membership").
- **Resource:** https://reclaimprotocol.org | https://zkpass.org

### Sigil - On-Chain Provenance
Verifiable records of agent actions and decisions.
- **Resource:** https://sigil.sh

---

## 5. Shared Trust & Cryptoeconomic Security

### EigenLayer - Restaking & AVS
Cryptoeconomic trust for verifiable services. Bastion coordinates workflows against EigenLayer-secured oracles.
- **Resource:** https://eigenlayer.xyz

### Lit Protocol - Threshold Cryptography
Programmable key management and threshold decryption. Bastion uses Lit as a trust primitive for confidential execution policies.
- **Resource:** https://litprotocol.com

---

## 6. Oracles & Data Feeds

### Chainlink - Decentralized Oracle Network
Price feeds, VRF (verifiable randomness), Automation (keeper network), CCIP (cross-chain).
Bastion consumes Chainlink price feeds for policy thresholds and VRF for unbiased randomness.
- **Feed Registry (mainnet):** `0x47Fb2585D2C56Fe188D0E6ec628a38b74fCeeeDf`
- **Resource:** https://docs.chain.link

### Pyth Network - Low-Latency Oracles
High-frequency price feeds optimized for Solana and EVM. Pull-based model (cheaper than push).
- **Resource:** https://pyth.network

### Chronicle - MakerDAO-Backed Oracles
Verifiable oracle protocol originally built by MakerDAO. Scribe model for gas-efficient updates.
- **Resource:** https://chroniclelabs.org

---

## 7. Bridges & Interoperability

### Across - Intents-Based Bridging
Fast cross-chain transfers via relayers and intents. Optimistic oracle for dispute resolution.
- **Resource:** https://across.to

### LayerZero - Omnichain Messaging
Arbitrary cross-chain message passing. Powers cross-chain agent actions.
- **Endpoints deployed on 80+ chains. Resource:** https://layerzero.network

### Wormhole - Cross-Chain Messaging
Generic message passing + token bridging. Solana-native origins, now multichain.
- **Resource:** https://wormhole.com

### Hyperlane - Permissionless Interop
Permissionless cross-chain messaging. Anyone can deploy to new chains without governance.
- **Resource:** https://hyperlane.xyz

---

## 8. Indexing & Data

### The Graph - Blockchain Indexing
Subgraph-based indexing for fast on-chain data queries. ERC-8004 agent registrations can be indexed for fast discovery.
- **Resource:** https://thegraph.com

### Blockscout MCP - AI Agent Blockchain Data
Model Context Protocol server for structured blockchain data. Multi-chain (mainnet + all major L2s).
- **Endpoint:** `https://mcp.blockscout.com/mcp`
- **Resource:** https://docs.blockscout.com

### Dune Analytics - On-Chain SQL
Query blockchain data with SQL. Dashboards for protocol analytics.
- **Resource:** https://dune.com

### Dune Sim - Realtime On-Chain APIs
REST/WebSocket APIs for wallet intelligence, token analytics, and real-time data.
- **Resource:** https://sim.dune.com

---

## 9. Agent Platforms & Frameworks

### Virtuals Protocol - Agent Economy
Agent identity, capital formation, tokenized agents, and agent marketplaces.
- **Resource:** https://virtuals.io

### ElizaOS (ai16z) - Agent Framework
Multi-chain agent framework with plugin system. Bastion provides the trust runtime for ElizaOS agents.
- **Resource:** https://elizaos.ai

### Olas (Autonolas) - Autonomous Agent Services
On-chain agent service registry. Agent bonding, staking, and service composability.
- **Resource:** https://olas.network

### Wayfinder - Agent Routing
Agent discovery and routing infrastructure.
- **Resource:** https://wayfinder.ai

---

## 10. Developer Tooling

### ETHSKILLS
19 Ethereum skills for AI agents (Solidity, wallets, gas, L2s, standards, DeFi, security, testing, deployment).
- **Install:** `/plugin install ethskills@ethskills` (Claude Code)
- **Resource:** https://ethskills.com/SKILL.md

### Base MCP
Base Account wallet via MCP server. Send, swap, sign, batched calls, x402 payments, 20+ DeFi plugins.
- **Install:** `npx skills add base/skills --skill base-mcp`
- **Resource:** https://docs.base.org/ai-agents/quickstart

### Build on Base
Complete Base development playbook: network config, contract deployment, wallet auth, payments, Farcaster.
- **Install:** `npx skills add base/skills --skill build-on-base`
- **Resource:** https://github.com/base/skills

### Foundry
Solidity-native dev toolchain (`forge`, `cast`, `anvil`). Fast compilation, testing, fuzzing, deployment.
- **Resource:** https://book.getfoundry.sh

### Scaffold-ETH 2
Full-stack Ethereum toolkit: Solidity + Next.js + Foundry. Auto-generates TypeScript types.
- **Resource:** https://docs.scaffoldeth.io

### Hardhat 3
TypeScript-first Ethereum dev environment. Solidity testing, fuzzing, Rust internals (Aug 2025).
- **Resource:** https://hardhat.org

### abi.ninja
Paste any verified contract address → interactive UI for all functions. Multi-chain. Zero setup.
- **Resource:** https://abi.ninja

### RPC Providers
- **LlamaNodes:** `https://eth.llamarpc.com` (free)
- **Alchemy:** 300M CU/month free tier
- **Infura:** MetaMask default
- **QuickNode:** Performance-focused

---

## 11. Bastion's Trust Lifecycle Map

```text
                         Applications
              AI Agents · Enterprises · DAEMON
                                │
    ────────────────────────────────────────────────────────────
                         Bastion Runtime
    ────────────────────────────────────────────────────────────
    Policy Compiler  · Risk Engine      · Transaction Firewall
    Trust Graph      · Evidence Aggregator · Audit Log
    Agent Session Manager · Human Approval Engine
    ────────────────────────────────────────────────────────────

    Identity & Trust              Agent Interoperability
    ├── ERC-8004 · ERC-8126      ├── A2A
    ├── ENS · DID/VC             ├── MCP
    └── Privado ID               └── ACP (future)

    Policy Backends               Wallet Infrastructure
    ├── OPA · Cedar              ├── ERC-4337 · EIP-7702
    ├── Runtime Rules            ├── ERC-7579 · ERC-7715
    └── Human Approval           ├── Session Keys · Safe
                                 └── Coinbase Smart Wallet

    Evidence & Provenance         Confidential Compute
    ├── EAS · Sign Protocol      ├── Arcium MXE (MPC)
    ├── Sigil · zkTLS            ├── SP1 (ZKVM)
    └── Reclaim · zkPass         ├── Lit (Threshold Crypto)
                                 ├── Nillion (Blind Compute)
    Trust Services               └── Aztec (ZK Private State)
    ├── Attestation: EAS · Sign
    ├── Identity: ERC-8004 · ENS  Execution
    ├── Oracles: Chainlink · Pyth ├── Solana
    │   · Chronicle              ├── Starknet (ZK + native AA)
    ├── Randomness: VRF          ├── Ethereum
    └── Restaking: EigenLayer    └── Midnight (ZK privacy)

    Settlement & Security         Connectivity & Bridging
    ├── Ethereum                 ├── Across
    ├── Pact Network             ├── LayerZero
    └── EigenLayer               ├── Wormhole
                                 └── Hyperlane

    Payments
    ├── x402
    ├── Pact Network
    └── EIP-3009 · ERC-2612
    ────────────────────────────────────────────────────────────
```
                    Applications
        AI Agents · Enterprises · DAEMON

                          │

                   Bastion Runtime
────────────────────────────────────────────────────

Identity                Policy              Wallet
├── ERC-8004            ├── OPA             ├── ERC-4337
├── ERC-8126            ├── Runtime Rules   ├── EIP-7702
├── ENS                 ├── Human Approval  ├── ERC-7579
├── A2A                 └── Chainlink VRF   ├── Safe (multisig)
├── Privado ID                              ├── Coinbase Smart Wallet
└── DID/VC                                 └── wagmi/viem

Evidence                Execution           Settlement
├── EAS                 ├── Solana          ├── Ethereum
├── Sign Protocol       ├── Arcium (MPC)    ├── Pact Network
├── zkTLS/Reclaim       └── Midnight (ZK)   └── EigenLayer
├── Sigil
└── The Graph

Trust Primitives         Bridging            Payments
├── EigenLayer           ├── Across          ├── x402
├── Lit Protocol         ├── LayerZero       ├── Pact Network
├── Chainlink/Pyth       ├── Wormhole        └── EIP-3009
└── Chronicle            └── Hyperlane
```

---

## 12. Resources

| Category | Resource |
|----------|----------|
| All Ethereum skills | https://ethskills.com/SKILL.md |
| Base agent skills | https://github.com/base/skills |
| ERC-8004 agent identity | https://www.8004.org |
| x402 payments | https://www.x402.org |
| Pact Network | https://pactnetwork.io/docs |
| Blockscout MCP | https://mcp.blockscout.com/mcp |
| Scaffold-ETH 2 | https://docs.scaffoldeth.io |
| Foundry | https://book.getfoundry.sh |
| SpeedRun Ethereum | https://speedrunethereum.com |
| Ethereum.org dev | https://ethereum.org/en/developers |
| ETH Tech Tree | https://www.ethtechtree.com |

---

**Built by zkOS Labs** - the Programmable Trust Runtime for Autonomous Systems.
