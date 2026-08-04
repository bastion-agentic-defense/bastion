# API reference

The sidecar exposes a JSON over HTTP API. All routes are relative to your sidecar
base URL - `https://bastion-agentique.fly.dev` for the hosted alpha, or
`http://localhost:8080` when self-hosted.

## Authentication

Every route except `/health`, `/auth/*` and the `.well-known` discovery documents
requires a bearer token.

```
Authorization: Bearer <token>
```

Tokens are obtained by proving control of an agent's DID.

### POST /auth/nonce

Request a challenge to sign.

```json
{ "did": "did:bastion:solana:7xKX...9mPq" }
```

Returns a single-use nonce with a short expiry.

```json
{ "nonce": "b7f3...", "expires_in": 120 }
```

### POST /auth/verify

Exchange a signed nonce for a session token.

```json
{
  "did": "did:bastion:solana:7xKX...9mPq",
  "signature": "5Kd9...",
  "nonce": "b7f3..."
}
```

```json
{ "token": "eyJ...", "expires_in": 3600 }
```

### POST /did/generate

Generate a new agent DID and keypair. The secret key is returned once and never
stored by the runtime.

### GET /did/resolve/:did

Resolve a DID to its public document.

## Evaluation

The core of the runtime. These endpoints decide whether an intent is permitted.

### POST /api/v2/evaluate

Evaluate a chain transaction against the agent's policy.

```json
{
  "did": "did:bastion:solana:7xKX...9mPq",
  "transaction": "<base64 serialized transaction>",
  "simulate": true
}
```

```json
{
  "id": "dec_01H8...",
  "verdict": "allow",
  "rule": "max_native_per_tx",
  "reason": "4.2 SOL within 5.0 SOL ceiling",
  "latency_ms": 8,
  "simulation": { "ok": true, "compute_units": 12043 },
  "audit_digest": "9c1af02b"
}
```

`verdict` is one of:

| Verdict | Meaning |
| --- | --- |
| `allow` | Permitted. Sign and submit. |
| `review` | Held for human approval. Poll `/pending` or subscribe to `/events`. |
| `block` | Refused. `rule` and `reason` explain which control fired. |

### POST /api/v2/evaluate-web2

The same decision path for outbound HTTP. Used by the egress proxy to inspect API
calls before they leave.

```json
{
  "did": "did:bastion:solana:7xKX...9mPq",
  "method": "POST",
  "url": "https://api.stripe.com/v1/charges",
  "headers": { "content-type": "application/json" },
  "body_sha256": "e3b0c442..."
}
```

### POST /api/v2/simulate-evm

Simulate an EVM transaction against live state without evaluating policy.

### POST /simulate

Legacy Solana simulation endpoint. Prefer `/api/v2/evaluate` with `simulate: true`.

## Agents

### POST /agents

Register an agent and attach its initial policy.

### GET /agents

List registered agents.

### GET /agents/:did

Fetch one agent, including its current policy and reputation score.

### GET /agents/:did/audit

Audit entries scoped to a single agent.

### POST /agents/:did/delegate

Delegate a bounded subset of an agent's authority to a child agent. The child can
never exceed the parent's policy.

### GET /agents/:did/tree

The full delegation tree beneath an agent.

### GET /agents/:did/children

Direct children only.

### DELETE /agents/:did/delegation/:child_did

Revoke a delegation. Takes effect on the next evaluation.

## Audit trail

### GET /audit/logs

Paginated decision history.

Query parameters: `did`, `verdict`, `limit`, `before`.

### GET /audit/logs/tx/:transaction_id

Look up the decision for a specific transaction.

### GET /audit/logs/signature/:signature

Look up by on-chain signature.

### GET /audit/stats

Aggregate counts by verdict and rule.

### DELETE /audit/logs/:id

Delete a single local audit record. The on-chain entry is immutable and is not
affected.

## Human review

### GET /pending

Decisions currently held for approval.

### POST /override

Approve or reject a held decision.

```json
{ "id": "dec_01H8...", "action": "approve", "note": "verified with treasury" }
```

## Circuit breaker

A fleet-wide stop. When engaged, every evaluation returns `block` regardless of
policy.

### GET /circuit-breaker/status

### POST /circuit-breaker/engage

### POST /circuit-breaker/disengage

## Policy

### GET /policy

Current effective policy.

### GET /policy/export

Export policy as TOML, suitable for committing to version control.

## Events

### GET /events

Server-sent event stream of decisions as they are made. Use this instead of
polling.

```ts
const es = new EventSource(`${base}/events`);
es.addEventListener('decision', e => {
  const decision = JSON.parse(e.data);
});
```

## Operations

### GET /health

Liveness probe. Unauthenticated.

### POST /ingest

Ingest an external security event into the correlation engine.

## Errors

Errors use conventional status codes with a JSON body.

```json
{
  "error": "policy_violation",
  "message": "96.0 SOL exceeds 5.0 SOL ceiling",
  "rule": "max_native_per_tx"
}
```

| Status | Meaning |
| --- | --- |
| `400` | Malformed request or unparseable transaction. |
| `401` | Missing, expired, or invalid token. |
| `403` | Blocked by policy, or circuit breaker engaged. |
| `404` | Unknown agent, decision, or audit entry. |
| `429` | Rate limit exceeded. |
| `503` | A dependency such as an RPC endpoint is unavailable. |
