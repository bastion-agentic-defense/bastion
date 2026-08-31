# Quickstart

Get an agent behind the Bastion runtime in about ten minutes.

> Bastion is alpha software. No component is deployed to mainnet with real value -
> production use is gated behind an external security audit. Run this against
> devnet or a testnet.

## What you are building

By the end of this guide, every transaction your agent intends will be evaluated
against a policy you control, simulated against live chain state, and recorded in
a verifiable audit trail. Nothing reaches a signer until the runtime allows it.

The flow has four moving parts:

- **Your agent** decides what it wants to do.
- **The SDK** submits that intent instead of signing it directly.
- **The sidecar** evaluates, simulates, and decides.
- **The audit trail** records the verdict and its reasoning.

## 1. Install

```bash
npm install @zkos-labs/bastion-sdk
```

For Web2 egress control - inspecting outbound API calls rather than chain
transactions - install the companion package instead:

```bash
npm install @zkos-labs/bastion-web2
```

## 2. Point the SDK at a sidecar

The hosted alpha sidecar is fine for evaluation. For anything real, run your own;
see Self-hosting below.

```ts
import { Bastion } from '@zkos-labs/bastion-sdk';

const bastion = new Bastion({
  url: process.env.BASTION_URL ?? 'https://bastion-agentique.fly.dev',
});
```

## 3. Generate an identity

Every agent gets a decentralised identifier. The runtime uses it to attach policy,
reputation, and audit history to a specific agent rather than to a raw keypair.

```ts
const { did, secretKey } = await bastion.generateDid();
// did → "did:bastion:solana:7xKX...9mPq"
```

Store `secretKey` the way you would store any signing key. It is the only thing
that can prove this agent's identity, and the runtime never sees it.

## 4. Authenticate

Authentication is challenge-response: request a nonce, sign it, exchange the
signature for a session token.

```ts
const { nonce } = await bastion.auth.nonce(did);
const token = await bastion.auth.verify({
  did,
  signature: sign(nonce, secretKey),
});
```

The SDK caches the token and refreshes it automatically. Calling the HTTP API
directly, send it as `Authorization: Bearer <token>`.

## 5. Register the agent and set a policy

The policy is the whole point. Everything not permitted here is refused.

```ts
await bastion.agents.register({
  did,
  label: 'treasury-01',
  policy: {
    maxNativePerTx: 5.0,
    rateLimitPerMinute: 10,
    programAllowlist: [
      '11111111111111111111111111111111',
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
    ],
    requireReviewAbove: 1.0,
  },
});
```

| Field | Meaning |
| --- | --- |
| `maxNativePerTx` | Hard ceiling on native token moved in a single transaction. |
| `rateLimitPerMinute` | Transactions per minute before the agent is throttled. |
| `programAllowlist` | Programs the agent may invoke. Everything else is blocked. |
| `requireReviewAbove` | Amount above which a decision is held for human approval. |

## 6. Route intents through the runtime

Replace direct signing with an evaluation call. This is the only change to your
agent's hot path.

```ts
const decision = await bastion.evaluate({
  did,
  transaction: tx,
});

if (decision.verdict === 'allow') {
  await sendTransaction(tx);
} else if (decision.verdict === 'review') {
  console.log('awaiting approval:', decision.id);
} else {
  console.warn('blocked by', decision.rule, '-', decision.reason);
}
```

A decision always carries the rule that produced it. When something is blocked,
you know which rule blocked it and why, not merely that it failed.

## 7. Read the audit trail

```ts
const entries = await bastion.audit.list({ did, limit: 50 });
```

Every entry is also written on-chain, so the record does not depend on trusting
the sidecar that produced it.

## Self-hosting

The sidecar is a single Rust binary. To run it locally:

```bash
git clone https://github.com/zkos-labs/bastion
cd bastion
cargo run -p bastion-sidecar
```

Or with Docker:

```bash
docker compose up sidecar
```

It listens on `:8080` by default. Configure policy defaults, RPC endpoints, and
the circuit breaker in `config.toml`.

## Next

- API reference covers every endpoint, with request and response shapes.
- MCP server exposes the runtime to a coding agent as callable tools.
