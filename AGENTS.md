# AGENTS.md, Bastion

> **Trust your Agent, but Verify every Transaction.**
>
> Bastion is a **Programmable Trust Runtime** for AI agents and autonomous systems. It orchestrates identity, policy, durable execution, human approval, multi-chain settlement, and trust observability - composing existing standards (ERC-4337, ERC-7579, ERC-8004, ERC-8126, EAS, Sign Protocol, x402) rather than replacing them.
>
> **If you are an AI coding agent reading this:** see [`SKILL.md`](SKILL.md) for self-registration and integration instructions. You can register yourself with a running Bastion instance via `POST /agents` to receive a W3C DID, trackable identity, and delegation capabilities.
>
> **Alpha software.** Bastion is in active development and not yet production hardened. Use with caution.
>
> **Ethereum ecosystem knowledge:** For building on Ethereum, Solidity contracts, ERC standards, agent identity (ERC-8004), x402 payments, and DeFi composability, load [`https://ethskills.com/SKILL.md`](https://ethskills.com/SKILL.md). For Base-specific tooling (Base MCP, wallet auth, contract deployment), load [`npx skills add base/skills`](https://github.com/base/skills). For the full Ethereum trust primitives map (wallets, oracles, bridges, attestations, indexing, agent platforms), see [`.agents/skills/ethereum-ecosystem/SKILL.md`](.agents/skills/ethereum-ecosystem/SKILL.md). For the restructured capability architecture, see [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).

---

## 1. Tech Stack

| Layer | Technology | Version |
|-------|-----------|---------|
| **Rust Sidecar** | Rust (edition 2024), Axum 0.7, Tokio 1, Sled 0.34 | 1.85+ |
| **Rust Core** | serde, thiserror, uuid, async-trait | 0.1.0 |
| **Rust Web2 Firewall** | bastion-web2-firewall, http, url, reqwest | 0.1.0 |
| **EVM Contracts** | Solidity 0.8.28, Foundry, OpenZeppelin, Solady | - |
| **Dashboard** | React 18, Vite 5, TailwindCSS 3.4, TypeScript 5 | 0.2.0 |
| **SDK** | TypeScript 5, viem 2, chain-agnostic HTTP (`@zkos-labs/bastion-sdk`) | 1.0.0 |
| **Web2 SDK** | TypeScript 5, BastionWeb2Client | 0.1.0 |
| **EVM Wallet** | wagmi 2.12, viem 2.21, RainbowKit 2.2, TanStack Query 5 | - |
| **Ethereum Standards** | ERC-8004, ERC-7579, ERC-4337, ERC-8126, EIP-7702, x402, EAS, Sign Protocol, Pact Network, ERC-8354 (draft), ERC-8380 (draft) | composed |
| **Ethereum Tooling** | ETHSKILLS, Base MCP/Skills, Blockscout MCP, Foundry, Scaffold-ETH 2 | - |
| **Package Manager** | pnpm 9+ (workspaces) | - |
| **CI/CD** | GitHub Actions, Netlify, Vercel | - |

---

## 2. Project Structure

```
bastion/
├── apps/web/                  ← React dashboard (Vite + TailwindCSS)
│   ├── src/
│   │   ├── pages/             ← Landing, Dashboard, Integrate
│   │   ├── hooks/             ← useBastionEVM (wagmi/viem read+write)
│   │   ├── components/        ← Navbar, VideoBackground, EvmProviderErrorBoundary
│   │   ├── context/           ← ThemeContext, ChainContext
│   │   ├── abi/               ← EVM contract ABIs (JSON, on main branch)
│   │   └── lib/               ← chains.ts, evmConfig.ts (EVM chains incl. Monad)
│   └── dist/                  ← Built output (Netlify/Vercel publish dir)
├── packages/sdk/              ← @zkos-labs/bastion-sdk (TypeScript, EVM + HTTP)
│   └── src/
│       ├── index.ts           ← BastionEVMClient + HTTP exports
│       ├── evm.ts             ← viem EVM contract client
│       ├── erc8354.ts         ← ERC-8354 verdict wrappers
│       ├── erc8380.ts         ← ERC-8380 credential wrappers
│       └── types.ts           ← AuditState, AuditEntry, Agent, Policy types
├── crates/                    ← Rust workspace
│   ├── core/                  ← Chain-agnostic policy engine (bastion-core)
│   ├── sidecar/               ← HTTP evaluator server (Axum, bastion-sidecar)
│   └── web2-firewall/         ← Web2 API proxy firewall (bastion-web2-firewall)
├── evm/                       ← Solidity contracts (Foundry)
│   ├── src/                   ← BastionFirewall, BastionPolicy, BastionAudit,
│   │   │                         BastionRegistry, BastionERC8004Registry, BastionSidecar
│   │   └── erc8354/           ← ERC-8354 confidential verdict contracts
│   │   └── erc8380/           ← ERC-8380 unclonable credential contracts
│   ├── test/                  ← Foundry test files (62 + ERC-8354/8380 tests)
│   ├── script/                ← DeployBastion.s.sol
│   └── lib/                   ← forge-std, openzeppelin-contracts, solady (submodules)
├── docs/                      ← Architecture, contributing, deployment plans
├── netlify/                   ← Netlify edge functions
├── config.toml                ← Sidecar policy config
├── docker-compose.yml         ← Docker compose for sidecar
├── Dockerfile                 ← Sidecar Docker image
├── netlify.toml               ← Netlify deploy config (root)
├── pnpm-workspace.yaml        ← pnpm workspace definition
├── Cargo.toml                 ← Rust workspace manifest
└── .github/workflows/ci.yml   ← GitHub Actions CI
```

---

## 3. Getting Started

### Prerequisites

- **Node.js** >= 20
- **pnpm** >= 9 (`corepack enable && corepack prepare pnpm@latest --activate`)
- **Rust** >= 1.85 (`rustup`)
- **Foundry** (`foundryup`, for EVM contracts)

### Quick Setup

```bash
git clone --recurse-submodules https://github.com/zkos-labs/bastion.git
cd bastion

# Install JS dependencies
pnpm install

# Build all JS packages
pnpm build

# Build Rust
cargo build

# Build EVM contracts
cd evm && forge build
```

### Environment Variables

Create `evm/.env`:
```
PRIVATE_KEY=
CELO_RPC_URL=https://forno.celo.org
CELO_TESTNET_RPC_URL=https://alfajores-forno.celo-testnet.org
BASE_RPC_URL=https://mainnet.base.org
ETH_RPC_URL=https://eth.llamarpc.com
POLYGON_RPC_URL=https://polygon-rpc.com
```

For the sidecar, set the per-chain EVM RPC env vars (e.g. `ETH_RPC_URL`,
`BASE_RPC_URL`, `CELO_RPC_URL`, `ZKSYNC_RPC_URL`, `ROBINHOOD_RPC_URL`,
`ETH_SEPOLIA_RPC_URL`, `MONAD_RPC_URL`) to enable EVM simulation. No Helius/Solana
key is required.

---

## 4. Build Commands

| Scope | Command | Notes |
|-------|---------|-------|
| **All JS** | `pnpm build` | Recursive across workspaces |
| **Dashboard** | `pnpm --filter bastion-dashboard build` | Vite production build → `apps/web/dist/` |
| **Dashboard dev** | `pnpm --filter bastion-dashboard dev` | Vite dev server on port 3000 |
| **SDK** | `pnpm --filter @zkos-labs/bastion-sdk build` | `tsc` → `packages/sdk/dist/` |
| **All Rust** | `cargo build` | From workspace root |
| **Rust release** | `cargo build --release` | Optimized binary in `target/release/` |
| **Rust check** | `cargo check` | Fast type-check only |
| **EVM contracts** | `cd evm && forge build` | Foundry compile → `evm/out/` |
| **Docker** | `docker build -t bastion-sidecar .` | Sidecar container |

---

## 5. Test Commands

| Scope | Command | Notes |
|-------|---------|-------|
| **All Rust** | `cargo test` | Workspace-level |
| **Core crate** | `cargo test -p bastion-core` | Unit tests |
| **Sidecar** | `cargo test -p bastion-sidecar` | Integration tests |
| **EVM contracts** | `cd evm && forge test -vvv` | Foundry tests (incl. ERC-8354/8380) |
| **EVM gas report** | `cd evm && forge test --gas-report` | Gas usage analysis |

> The SDK has a Jest suite (`packages/sdk/src/*.test.ts`, 20 tests). The dashboard has no component tests yet.

---

## 6. Lint & Format Commands

| Scope | Command | Notes |
|-------|---------|-------|
| **Rust format** | `cargo fmt --all -- --check` | CI check mode |
| **Rust format fix** | `cargo fmt` | Auto-fix |
| **Rust clippy** | `cargo clippy -- -D warnings` | All crates |
| **Per-crate clippy** | `cargo clippy -p bastion-core -- -D warnings` | Single crate |
| **EVM format** | `cd evm && forge fmt --check` | CI check mode |
| **EVM format fix** | `cd evm && forge fmt` | Auto-fix |

> No TS/JS linting is configured yet. Run `pnpm lint` is defined at root but not per-package.

---

## 7. Architecture

```
Agent Operator (policy config, HITL review)
       │
       ▼
┌──────────────────────────────────────────────────────────────────┐
│                         Bastion Monorepo                         │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────────────┐ │
│  │ crates/core  │   │   SDK + CLI  │   │  Dashboard (React)   │ │
│  │ (chain-agn.) │   │  (TypeScript)│   │  (apps/web)           │ │
│  └──────┬───────┘   └──────────────┘   └──────────────────────┘ │
│         │                                                        │
│    ┌────┴─────────────────────────────┐                          │
│    ▼                                  ▼                          │
│  ┌──────────────┐               ┌───────────────────┐           │
│  │crates/sidecar│               │crates/web2-firewall│          │
│  │(EVM policy)  │               │(Web2 proxy engine) │          │
│  └──────┬───────┘               └───────────────────┘           │
│         │                                                        │
│         ▼                                                        │
│  ┌───────────────────────────────┐                               │
│  │            EVM (Solidity)     │                               │
│  │  Firewall · Policy · Audit ·  │                               │
│  │  Registry · ERC-8004 · ERC-8354 · ERC-8380                    │
│  └───────────────────────────────┘                               │
└──────────────────────────────────────────────────────────────────┘
```

### How components relate

1. **`crates/core`**, Chain-agnostic policy engine. Defines `NormalizedTransaction`, `FirewallDecision`, `PolicyEvaluator<P: TrustSignalProvider>`, `PolicyRule` (11 variants), `PolicySet`, `TrustAdapter` trait, `TrustSignalProvider` trait, `Chain` enum (EVM chains: Ethereum, Base, Polygon, Arbitrum, Celo, ZkSync, Robinhood, Monad), and `AuditRecord`.

2. **`crates/sidecar`**, Axum HTTP server (port 3000) that runs the policy evaluator. Exposes REST API for EVM simulation (`/api/v2/simulate-evm`), audit logging, policy management, circuit breaker, and human override. Uses per-chain EVM RPC for simulation, Sled DB for audit logs, GrondOSINT for risk oracle, and can issue ERC-8354 verdicts (feature-flagged).

3. **`evm/`**, Solidity contracts implementing the ERC-7579 validator module, policy engine, immutable EIP-712 audit trail, agent registry, ERC-8004 identity, sidecar oracle, plus the **ERC-8354** confidential-verdict and **ERC-8380** unclonable-credential contracts (see `docs/ERCS.md`).

4. **`apps/web/`**, React dashboard with EVM (RainbowKit/wagmi) wallet connection across Sepolia/Base/Celo/zkSync/Robinhood/Monad. Shows audit logs, policy settings, stats.

5. **`packages/sdk/`**, TypeScript SDK (`@zkos-labs/bastion-sdk`) — viem EVM contract client + chain-agnostic HTTP layer + ERC-8354/8380 wrappers.

> **Archived:** Solana (Anchor program + wallet stack) and Arcium are retired —
> see [`docs/ARCHIVE.md`](docs/ARCHIVE.md). Their files remain on disk for history
> but are not part of the active workspace/CI.

---

## 8. Key Code Paths

### Rust Sidecar

- **Entry:** `crates/sidecar/src/main.rs`, binds `0.0.0.0:3000`
- **Routes:** `crates/sidecar/src/lib.rs`, all HTTP handlers
- **Policy engine:** `crates/core/`, chain-agnostic evaluation logic
- **EVM simulation:** `crates/sidecar/src/simulation_evm.rs`, per-chain RPC simulation
- **Audit DB:** `crates/sidecar/src/audit.rs`, Sled-based log store
- **Risk oracle:** `crates/sidecar/src/grond_oracle.rs`, GrondOSINT integration

### EVM Contracts

- **Firewall:** `evm/src/BastionFirewall.sol`, ERC-7579 validator, gates UserOperations
- **Policy:** `evm/src/BastionPolicy.sol`, Per-agent rules (allowlists, limits, cooldowns)
- **Audit:** `evm/src/BastionAudit.sol`, EIP-712 signed audit entries
- **Registry:** `evm/src/BastionRegistry.sol`, Agent + target directory
- **ERC-8004:** `evm/src/BastionERC8004Registry.sol`, Agent identity (ERC-721 + EIP-712)
- **Sidecar:** `evm/src/BastionSidecar.sol`, Oracle request/fulfill pattern
- **ERC-8354:** `evm/src/erc8354/`, confidential verdict contracts
- **ERC-8380:** `evm/src/erc8380/`, unclonable credential contracts
- **Deploy:** `evm/script/DeployBastion.s.sol`

### Web Dashboard

- **Entry:** `apps/web/src/main.tsx`
- **App shell:** `apps/web/src/App.tsx`, providers, wallet setup, routing
- **Pages:** `apps/web/src/pages/Landing.tsx`, `Dashboard.tsx`, `integrate/Integrate.tsx`
- **EVM hooks:** `apps/web/src/hooks/useBastionEVM.ts` (wagmi/viem read+write)
- **Chain config:** `apps/web/src/lib/chains.ts`, `apps/web/src/lib/evmConfig.ts`
- **Theme:** `apps/web/src/context/ThemeContext.tsx`

### TypeScript SDK

- **Entry:** `packages/sdk/src/index.ts`, `BastionEVMClient` + HTTP exports
- **EVM client:** `packages/sdk/src/evm.ts` (viem)
- **ERC-8354:** `packages/sdk/src/erc8354.ts`
- **ERC-8380:** `packages/sdk/src/erc8380.ts`
- **Types:** `packages/sdk/src/types.ts`

---

## 9. Environment Variables

### Sidecar

| Variable | Default | Purpose |
|----------|---------|---------|
| `ETH_RPC_URL` | (unset) | Ethereum mainnet RPC (EVM simulation) |
| `ETH_SEPOLIA_RPC_URL` | (unset) | Sepolia RPC |
| `BASE_RPC_URL` | (unset) | Base mainnet RPC |
| `CELO_RPC_URL` | (unset) | Celo mainnet RPC |
| `ZKSYNC_RPC_URL` | (unset) | zkSync Era RPC |
| `ROBINHOOD_RPC_URL` | (unset) | Robinhood RPC |
| `MONAD_RPC_URL` | (unset) | Monad RPC |
| `GROND_API_URL` | (unset) | GrondOSINT base URL |
| `BASTION_REQUIRE_AUTH` | (unset) | Fail closed on unauthenticated requests |

### EVM / Foundry (in `evm/.env`)

| Variable | Example | Purpose |
|----------|---------|---------|
| `PRIVATE_KEY` | `0x...` | Deployer private key |
| `CELO_RPC_URL` | `https://forno.celo.org` | Celo mainnet RPC |
| `CELO_TESTNET_RPC_URL` | `https://alfajores-forno.celo-testnet.org` | Celo Alfajores RPC |
| `BASE_RPC_URL` | `https://mainnet.base.org` | Base mainnet RPC |
| `ETH_RPC_URL` | `https://eth.llamarpc.com` | Ethereum mainnet RPC |
| `POLYGON_RPC_URL` | `https://polygon-rpc.com` | Polygon mainnet RPC |

### Dashboard

| Variable | Purpose |
|----------|---------|
| `VITE_BASTION_AUDIT_ADDRESS` | EVM audit contract address |
| `VITE_BASTION_POLICY_ADDRESS` | EVM policy contract address |
| `VITE_BASTION_FIREWALL_ADDRESS` | EVM firewall contract address |
| `VITE_BASTION_REGISTRY_ADDRESS` | EVM registry contract address |
| `VITE_BASTION_ERC8004_ADDRESS` | EVM ERC-8004 contract address |

---

## 10. Deploying

### Netlify (primary)

Root `netlify.toml` is configured. Pushes to `main` auto-deploy.
```bash
# Manual: netlify deploy --prod --dir=apps/web/dist
```

### Vercel

Project: `muhammad-zidan-fatonies-projects/bastion-web`. Deploys from `main` branch.
```bash
# Manual (from apps/web/):
vercel --prod
```

### Docker (sidecar)

```bash
docker build -t bastion-sidecar .
docker run -p 3000:3000 -e ETH_RPC_URL=... bastion-sidecar
# Or: docker compose up
```

### EVM Contracts (Foundry)

`evm/script/DeployBastion.s.sol` deploys all contracts in order.
```bash
# Celo mainnet
cd evm
source .env
forge script script/DeployBastion.s.sol --rpc-url celo --broadcast --verify

# Polygon/Base/Monad: adjust --rpc-url (e.g. --rpc-url monad)
```

---

## 11. CI/CD (GitHub Actions)

**File:** `.github/workflows/ci.yml`

Triggers on push/PR to `main` (ignoring `.md` and `docs/`).

| Job | What it does |
|-----|-------------|
| `core` | cargo check, clippy, test for `bastion-core` |
| `sidecar` | cargo check, clippy, test for `bastion-sidecar` |
| `policy-engine` | cargo check, clippy, test for `bastion-policy-engine` |
| `web2-crate` | cargo check, clippy, test for `bastion-web2-firewall` |
| `audit` | `cargo audit` |
| `fmt` | `cargo fmt --all -- --check` |
| `evm` | Checkout submodules, `forge build`, `forge test -vvv` |
| `web` | `pnpm install`, `pnpm --filter bastion-dashboard build` |
| `sdk` | `pnpm install`, `pnpm --filter @zkos-labs/bastion-sdk build` + test |
| `web2-sdk` | `pnpm install`, `pnpm --filter @zkos-labs/web2-sdk build` |
| `mcp-server` | `pnpm install`, `pnpm --filter @zkos-labs/mcp-server build` |

---

## 12. Known Gotchas

### `tuple()` ABI format breaks abitype

**Problem:** Human-readable ABIs using `tuple(type name, ...)` syntax crash with `InvalidParameterError` from `abitype@1.2.3`.

**Fix:** Use `(type name, ...)` without the `tuple` prefix. Example:
```
// BROKEN:
'function getEntry(uint256 id) returns (tuple(address agent, string reason))'

// FIXED:
'function getEntry(uint256 id) returns ((address agent, string reason))'
```

### `buffer/` alias is fragile

`vite.config.ts` aliases `buffer` → `buffer/`. This is needed for EVM wallet dependencies. If the build fails with `Could not load buffer/`, ensure `buffer` is in `apps/web/package.json` dependencies and pnpm's node_modules are intact.

### EVM dashboard hooks import the committed ABIs

`apps/web/src/hooks/useBastionEVM.ts` reads/writes the deployed contracts through wagmi/viem. It imports the committed JSON ABIs from `apps/web/src/abi/*.ts` (the single source of truth, regenerated from the forge build). Do not re-declare `parseAbi([...])` strings in the hook — the function/event names then drift from the on-chain contracts (e.g. `getEntryCount` vs `entryCount`, the `AuditRecorded` field list).

### ERC-8354/8380 are draft standards

The confidential-verdict (ERC-8354) and unclonable-credential (ERC-8380) contracts
and SDK wrappers track the current **draft** specs (see `docs/ERCS.md`). Their ABIs
are pinned to a draft commit and may change before finalization. The `"ERC-1953/..."`
domain-separator tag strings in ERC-8380 are frozen as-is for compatibility even
though the assigned number is 8380.

### Forge submodules required

EVM contracts depend on git submodules (`forge-std`, `openzeppelin-contracts`, `solady`). Always clone with `--recurse-submodules` or run `git submodule update --init --recursive`.

### `pnpm build` may fail on workspace lifecycle

If `pnpm build` fails with `pnpm install` errors, run `pnpm install` separately first, then build individual packages directly:
```bash
pnpm --filter bastion-dashboard build
```

---

## 13. Development Workflow

### Branch Strategy

- **`main`**, production branch, deploys to Netlify/Vercel. All merges must pass CI.
- **`run`**, development/experimental branch.

### Pre-commit Checklist

Before committing, run:

```bash
# Rust
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test

# EVM
cd evm && forge build && forge test -vvv && forge fmt --check

# Dashboard
pnpm --filter bastion-dashboard build

# SDK
pnpm --filter @zkos-labs/bastion-sdk build && pnpm --filter @zkos-labs/bastion-sdk test
```

### PR Process

1. Create feature branch from `main` (or `run`)
2. Implement changes
3. Run pre-commit checks above
4. Push and create PR
5. CI must pass all 11 jobs
6. Merge to `main` via squash or rebase

### After merging to main

- Netlify auto-deploys from `main`
- Vercel auto-deploys from `main` (project: `bastion-web`)
- If EVM contracts changed: re-deploy via `forge script`

---

## 14. Browser / Wallet Compatibility

The dashboard is EVM-only and supports MetaMask, WalletConnect, and other
RainbowKit/wagmi wallets across Sepolia, Base, Celo, zkSync, Robinhood, and Monad.
Chain config lives in `apps/web/src/lib/evmConfig.ts`; chain switching is handled
by wagmi/RainbowKit.

---

## 15. License & Security

- **License:** Apache-2.0
- **Security policy:** `SECURITY.md`
- **Protocols:** ERC-7579 (validator module), ERC-4337 (account abstraction), ERC-8004 (agent identity), ERC-7812 (evidence), EIP-712 (typed structured data), ERC-8354 (draft), ERC-8380 (draft)
- **Retired:** Solana + Arcium — see [`docs/ARCHIVE.md`](docs/ARCHIVE.md)
