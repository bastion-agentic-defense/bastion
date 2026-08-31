# Bastion Agentique SDK

Multichain TypeScript SDK for the Bastion Programmable Trust Runtime — identity,
policy, simulation, execution, and observability for autonomous systems, across
EVM chains and Solana.

> **Package history:** this package was previously published as
> `@zkos-labs/bastion-sdk`, and before that `@zkos-labs/sdk` (and, before
> that, `@bastion-agentique/sdk`). It now ships as
> **`@zkos-labs/bastion-agentique`**. The old Solana Anchor program client
> (`BastionClient`) is gone — this SDK talks to chains through the Bastion
> sidecar's HTTP API (`BastionSidecar`) and, for EVM, directly via `viem`
> (`BastionEVMClient`).

## What's new

- **Multichain settlement**: EVM (Ethereum, Base, Celo, zkSync, Robinhood,
  Monad, Polygon, Arbitrum) *and* Solana, through the same `Bastion.execute()`
  call and the sidecar's `simulateEvm` / `simulateSolana` endpoints.
- **ERC-8354** confidential policy verdict wrappers (`commitPolicyAction`,
  `verdictDigest`, `verifyVerdict`, `consumeVerdict`).
- **ERC-8380** unclonable capability credential wrappers (`computeNullifier`,
  `computeCapabilityCommitment`, `issueCapability`, `executeCapability`,
  `isConsumed`).
- **`BastionWorkflow`**: durable, resumable multi-step execution
  (`executeIntent`, `plan`, `compensate`, `signal`, `replay`, `start`,
  `state`, `list`).

## Installation

```bash
npm install @zkos-labs/bastion-agentique viem
pnpm add @zkos-labs/bastion-agentique viem
yarn add @zkos-labs/bastion-agentique viem
```

`viem` is a peer dependency, only required if you use `BastionEVMClient`
directly. The HTTP-only surface (`BastionSidecar`, `Bastion`, `BastionWorkflow`,
the ERC-8354/8380 wrappers) has no on-chain client dependency.

## Quick Start

### HTTP (sidecar) — simulate, policy, audit, multichain

```typescript
import { BastionSidecar, Bastion } from "@zkos-labs/bastion-agentique";

const sidecar = new BastionSidecar({
  baseUrl: "https://bastion-agentique.fly.dev/",
  apiKey: process.env.BASTION_API_KEY,
});

// One-line trust decision for an EVM transaction.
const bastion = new Bastion({ sidecar });
const evmResult = await bastion.execute({
  action: "swap",
  settlement: "base", // ethereum | base | celo | zksync | robinhood | monad | polygon | arbitrum | solana
  transaction: {
    from: "0x...",
    to: "0x...",
    value: "0x0",
    data: "0x...",
  },
  agentId: "agent-001",
});
console.log(evmResult.decision); // "pass" | "block" | "pending_hitl"

// Same call shape for Solana settlement.
const solResult = await bastion.execute({
  action: "transfer",
  settlement: "solana",
  solanaTx: { to: "11111111111111111111111111111111", amount: 1_000_000 },
  agentId: "agent-001",
});
```

### EVM contracts — read/write (viem)

```typescript
import { createPublicClient, createWalletClient, http } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { base } from "viem/chains";
import { BastionEVMClient } from "@zkos-labs/bastion-agentique";

const publicClient = createPublicClient({ chain: base, transport: http() });
const walletClient = createWalletClient({
  chain: base,
  transport: http(),
  account: privateKeyToAccount("0x..."),
});

const client = new BastionEVMClient({
  publicClient,
  walletClient,
  chain: base,
  contracts: {
    audit: "0x...",
    policy: "0x...",
    firewall: "0x...",
  },
});

const count = await client.getEntryCount();
const policy = await client.readPolicy("0xAgent...");
const { allowed, reason } = await client.validate(
  "0xAgent...",
  "0xTarget...",
  0n,
  "0x...",
);
const txHash = await client.writePolicy("0xAgent...", {
  agent: "0xAgent...",
  isActive: true,
  maxValuePerTx: 1000000n,
  maxGasPerTx: 500000n,
  dailyTxLimit: 100n,
  cooldownSeconds: 60n,
  allowedTargets: [],
  allowedSelectors: [],
  extraData: "0x",
});
```

### ERC-8354 (draft) — confidential verdicts

```typescript
import { commitPolicyAction, verdictDigest, verifyVerdict } from "@zkos-labs/bastion-agentique";

const commitment = commitPolicyAction({
  chainId: 8453n,
  domainId: "0x...",
  agentId: 7n,
  target: "0x...",
  value: 0n,
  callDataHash: "0x...",
  actionNonce: 1n,
});
```

### ERC-8380 (draft) — capability credentials

```typescript
import { issueCapability, isConsumed } from "@zkos-labs/bastion-agentique";

const capability = issueCapability({
  agentId: 7n,
  homeChainId: 8453n,
  homeDomainId: "0x...",
  capabilityIndex: 0n,
  actionCommitment: "0x...",
  executor: "0x...",
  expiry: 1893456000n,
});
```

## API Reference

### `BastionSidecar` (HTTP)

| Method | Description |
|--------|-------------|
| `health()` | Sidecar health check (`GET /health`) |
| `simulateEvm(req)` | Run an EVM tx through the firewall (`POST /api/v2/simulate-evm`) |
| `simulateSolana(req)` | Run a Solana operation through the firewall (`POST /api/v2/simulate-solana`) |
| `logs(query?)` | Fetch audit log entries (`GET /logs`) |
| `getPolicy()` / `updatePolicy(policy)` | Read/update the sidecar policy |
| `approve(req)` | Human-in-the-loop override (`POST /override`) |
| `circuitBreakerStatus()` | Read circuit breaker state |
| `engageCircuitBreaker()` / `disengageCircuitBreaker()` | Trip/reset the circuit breaker |
| `triggerScan()` / `scanResults()` | Background trust scans |
| `events()` | SSE event stream |

### `Bastion` (runtime facade)

`execute(req)` composes privacy enforcement → EVM/Solana simulation/policy
evaluation → verification. Settlement networks:
`ethereum | base | celo | zksync | robinhood | monad | polygon | arbitrum | solana`.

### `BastionWorkflow` (durable execution)

`executeIntent`, `plan`, `compensate`, `signal`, `replay`, `start`, `state`, `list`.

### `BastionEVMClient` (viem)

| Method | Description |
|--------|-------------|
| `getEntryCount()` | Total on-chain audit entries |
| `readAuditEntry(entryId)` | Read one audit entry |
| `readPolicy(agent)` | Read per-agent policy |
| `writePolicy(agent, policy)` | Set per-agent policy (wallet client required) |
| `validate(agent, target, value, callData)` | Check a transaction against policy |
| `isPaused()` / `pause()` / `unpause()` | Firewall circuit breaker |

### Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `AGENT_CAPABILITIES.TRANSFER` | `1 << 0` | Token transfers |
| `AGENT_CAPABILITIES.SWAP` | `1 << 1` | DEX swaps |
| `AGENT_CAPABILITIES.NFT_MINT` | `1 << 2` | NFT minting |
| `AGENT_CAPABILITIES.STAKE` | `1 << 4` | Staking |
| `AGENT_CAPABILITIES.DELEGATE` | `1 << 5` | Spawn sub-agents |
| `DECISION.ALLOWED` | `0` | Transaction allowed |
| `DECISION.BLOCKED` | `1` | Transaction blocked |
| `DECISION.PENDING` | `2` | Awaiting HITL override |

## License

Apache-2.0
