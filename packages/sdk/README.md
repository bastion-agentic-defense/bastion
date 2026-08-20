# Bastion SDK

EVM + HTTP TypeScript SDK for the Bastion Programmable Trust Runtime. Identity, policy, execution, and observability for autonomous systems.

> **Package moved:** this package was previously published as `@zkos-labs/sdk`
> (and, before that, `@bastion-agentique/sdk`). Following Bastion's full-EVM
> pivot it now ships as **`@zkos-labs/bastion-sdk`**. The Solana Anchor program
> client (`BastionClient`) is gone — update your imports to the EVM/HTTP surface
> below.

## Installation

```bash
npm install @zkos-labs/bastion-sdk viem
pnpm add @zkos-labs/bastion-sdk viem
yarn add @zkos-labs/bastion-sdk viem
```

## Quick Start

### HTTP (sidecar) — simulate, policy, audit

```typescript
import { BastionSidecar, Bastion } from "@zkos-labs/bastion-sdk";

const sidecar = new BastionSidecar({
  baseUrl: "https://bastion-agentique.fly.dev/",
  apiKey: process.env.BASTION_API_KEY,
});

// One-line trust decision for an EVM transaction.
const bastion = new Bastion({ sidecar });
const result = await bastion.execute({
  action: "swap",
  settlement: "base",
  transaction: {
    from: "0x...",
    to: "0x...",
    value: "0x0",
    data: "0x...",
  },
  agentId: "agent-001",
});
console.log(result.decision); // "pass" | "block" | "pending_hitl"
```

### EVM contracts — read/write (viem)

```typescript
import { createPublicClient, createWalletClient, http } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { base } from "viem/chains";
import { BastionEVMClient } from "@zkos-labs/bastion-sdk";

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
import { commitPolicyAction, verdictDigest, verifyVerdict } from "@zkos-labs/bastion-sdk";

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
import { issueCapability, isConsumed } from "@zkos-labs/bastion-sdk";

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
| `simulate(req)` | Run a transaction through the firewall (`POST /simulate`) |
| `simulateEvm(req)` | Run an EVM tx through the firewall (`POST /api/v2/simulate-evm`) |
| `logs(query?)` | Fetch audit log entries (`GET /logs`) |
| `getPolicy()` / `updatePolicy()` | Read/update the sidecar policy |
| `approve(req)` | Human-in-the-loop override (`POST /override`) |
| `circuitBreakerStatus()` | Circuit breaker state |
| `triggerScan()` / `scanResults()` | Background trust scans |
| `events()` | SSE event stream |

### `Bastion` (runtime facade)

`execute(req)` composes privacy enforcement → EVM simulation/policy evaluation →
verification. Settlement networks: `ethereum | base | celo | zksync | robinhood | monad`.

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
