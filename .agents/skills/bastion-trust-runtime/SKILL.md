---
name: bastion-trust-runtime
description: "Integrate Bastion programmable policy enforcement with trustless-ai agents. Compose ERC-8004 identity + ERC-8263 anchor proofs + ERC-8281 OCP commitments with Bastion's policy engine and human-in-the-loop gate. Use when building agents that need enforceable trust rules, multi-chain execution, or recompute-able audit trails."
---

# Bastion Trust Runtime — Agent Skill

This skill teaches agents to compose Bastion's programmable trust layer with the trustless-ai ERC stack.

## What Bastion Adds

trustless-ai provides **verification** (ERC-8263 anchor proofs, ERC-8274 proof verification, ERC-8281 commitments). Bastion adds the layer between them:

```
ERC-8004 (identity) → Bastion Policy Engine → ERC-8301 (dispatch) → ERC-8281 (provenance) → ERC-8299 (WYRIWE) → ERC-8274 (verify) → ERC-8263 (anchor)
```

| trustless-ai gives you | Bastion adds |
|------------------------|--------------|
| ERC-8004 agent identity | Programmable policy evaluation (11 rule types) |
| ERC-8263 anchor proofs | Pre-execution simulation (Solana + EVM) |
| ERC-8281 OCP commitments | Human-in-the-loop override gate |
| ERC-8299 WYRIWE provenance | Multi-chain TrustAdapter (8 chains) |
| ERC-8275 settlement | Web2 API policy enforcement |

## Architecture

```text
Agent Framework (LangGraph / CrewAI / OpenAI Agents SDK)
        │
        ▼
trustless-ai agent-sdk (ERC clients + recompute)
        │
        ▼
Bastion Trust Runtime
├── Identity (ERC-8004 compat)
├── Policy Engine (11 rules)
├── Simulation (Helius / eth_call)
├── HITL Gate (/override)
└── Audit (Anchor PDA + EIP-712 + ERC-8263)
        │
        ▼
Execution Environments (Ethereum, zkSync, Solana, Base, ...)
```

## TrustIntent: Declare What, Not How

Instead of calling low-level ERC methods directly, agents declare a TrustIntent:

```yaml
intent: transfer
asset: USDC
amount: 500
recipient: 0x...
requirements:
  - humanApproval
  - sanctionsCheck
  - maxRisk: medium
  - settlement: ethereum
```

Bastion resolves:
- Which `IPolicyRule` instances to evaluate
- Whether `PendingHITL` is required
- Which chain adapter to use
- Whether to anchor via ERC-8263

## Using Bastion with agent-sdk

```typescript
// 1. Register identity via ERC-8004 (trustless-ai)
import { IdentityRegistryClient } from "@trustless-ai/agent-sdk/identity/ERC8004";
const identity = new IdentityRegistryClient({ rpcUrl, address: registryAddress }, account);
const { agentId } = await identity.register("ipfs://Qm...");

// 2. Simulate + policy-check via Bastion (programmable trust)
import { BastionSidecar } from "@zkos-labs/sdk";
const sidecar = new BastionSidecar({ baseUrl: "https://bastion-agentique.fly.dev" });
const decision = await sidecar.simulate({ transaction, intent: "swap 1 SOL to USDC" });

// 3. If passed, execute. Bastion writes an audit record with ERC-8263 AnchorProof.
//    The audit ID can be recomputed offline using verify.recomputeObservationDigest().

// 4. Recompute verification (trustless-ai philosophy)
import { verify } from "@zkos-labs/sdk";
const result = verify.verifyAuditRecord({
  rawInput: JSON.stringify(transaction),
  decision: "Pass",
  payloadHash: txHash,
  expectedObservationDigest: auditRecord.observation_digest,
  expectedWyriweHash: auditRecord.wyriwe_hash,
});
// result.valid === true → no trust in Bastion required
```

## Recompute Guarantee

Every Bastion audit record carries:
- **`observation_digest`** (ERC-8281 OCP): sha256 of "Decision:payload_hash"
- **`wyriwe_hash`** (ERC-8299 WYRIWE): triple-hash binding raw input → sanitized input

Both are recompute-able from public inputs. The `@zkos-labs/sdk` `verify` module provides pure functions that reproduce these hashes without any network access — implementing trustless-ai's "Don't trust. Recompute." guarantee for the policy layer.

## When to Use Bastion

- You need **enforceable rules** (not just advisory): amount caps, allowlists, rate limits
- You need **human approval gates** for high-value or suspicious actions
- You need **multi-chain execution** (Ethereum + Solana + zkSync + Base from one interface)
- You need **recompute-able audit trails** that external parties can verify independently
- You need **Web2 API policy enforcement** alongside on-chain policy (OpenAI, Stripe, Slack)
