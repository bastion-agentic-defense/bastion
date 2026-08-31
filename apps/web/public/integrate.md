# Integrate Bastion - SDK & API

Bastion provides a TypeScript SDK and REST API for integrating the
Programmable Trust Runtime into your application.

## Install the SDK

```bash
npm install @zkos-labs/bastion-agentique
```

## Basic Usage (SDK)

```typescript
import { BastionClient } from "@zkos-labs/bastion-agentique";

const client = new BastionClient({ baseUrl: "https://bastion-agentique.fly.dev/" });

// Simulate a Solana transaction
const result = await client.simulate({
  transaction: base64Tx,
  intent: "Swap 1 SOL for USDC on Jupiter",
});

if (result.status === "allowed") {
  // Proceed to sign and broadcast
} else if (result.blockId) {
  // Human approval needed - show block_id to user
  console.log("Block reason:", result.error);
}
```

## EVM Usage (SDK)

```typescript
// Simulate an EVM transaction
const evmResult = await client.simulateEvm({
  transaction: { to: "0x...", data: "0x...", value: "1000000" },
  intent: "Swap 0.1 ETH for USDC on Uniswap",
  chain: "sepolia",
});
```

## REST API

```bash
# Simulate a transaction
curl -X POST https://bastion-agentique.fly.dev/simulate \
  -H "Content-Type: application/json" \
  -d '{"transaction": "...", "intent": "Swap 1 SOL for USDC"}'

# Get current policy
curl https://bastion-agentique.fly.dev/policy

# Get audit logs
curl https://bastion-agentique.fly.dev/logs?limit=50

# Human override
curl -X POST https://bastion-agentique.fly.dev/override \
  -H "Content-Type: application/json" \
  -d '{"block_id": "...", "action": "ALLOW"}'
```

## Supported Chains

- Solana
- EVM (Celo, Base, Ethereum, Polygon)
- Arcium MXE (confidential computing)

## Links

- **NPM**: https://www.npmjs.com/package/@zkos-labs/bastion-agentique
- **API Reference**: https://github.com/zkos-labs/bastion#readme
