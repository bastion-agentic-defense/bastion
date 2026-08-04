# Bastion auth.md

This document describes how AI agents authenticate with the Bastion sidecar API and MCP server.

## Agent Registration

1. **Generate a keypair** (Solana):
```bash
solana-keygen new --outfile ~/.config/solana/bastion-agent.json
```

2. **Derive your DID**: `did:bastion:solana:{agent_pda_base58}`

3. **Register with the sidecar**:
```bash
curl -s -X POST https://bastion-agentique.fly.dev/agents \
  -H "Content-Type: application/json" \
  -H "X-Api-Key: YOUR_BASTION_API_KEY" \
  -d '{
    "did": "did:bastion:solana:YOUR_AGENT_PDA",
    "authority_pubkey": "YOUR_SOLANA_PUBKEY",
    "sidecar_endpoint": null
  }'
```

## API Key Authentication

Bastion uses API key authentication for mutating endpoints:

| Header | Value | Required On |
|--------|-------|-------------|
| `X-Api-Key` | Bastion API key from environment `BASTION_API_KEY` | All mutating endpoints (POST/PUT/DELETE) |

**Read-only endpoints** (`GET /health`, `GET /logs`, `GET /agents`) do not require authentication.

**Auth is optional**: When `BASTION_API_KEY` is not set in the sidecar environment, all endpoints are open.

## Pricing

Bastion is a non-profit trust runtime, like Grafana. The core infrastructure is free and open source under Apache 2.0. Read operations are always free. Backend API calls are optionally paid via USDT or USDC to cover infrastructure costs. See `/mcp/pricing` for current prices.

## MCP Server Authentication

The MCP HTTP server supports direct access:
- Read operations: no auth required, always free
- Backend API calls: free tier with monthly reset, optional paid via USDT/USDC after exhaustion

## Token Format

No tokens are used. Authentication is header-based:
- **API key** for sidecar operations
- **Payment proof** (tx hash) for MCP tool calls

## Delegation

Parent agents can spawn sub-agents with delegated authority:

```bash
curl -s -X POST "https://bastion-agentique.fly.dev/agents/did:bastion:solana:PARENT_DID/delegate" \
  -H "Content-Type: application/json" \
  -H "X-Api-Key: YOUR_BASTION_API_KEY" \
  -d '{
    "child_did": "did:bastion:solana:CHILD_DID",
    "child_name": "SubAgent-ETH",
    "delegated_capabilities": ["TRANSFER"]
  }'
```

## Agent Identity (DID)

Every agent has a W3C DID: `did:bastion:solana:{agent_pda_base58}`

Resolve at: `GET /did/resolve/solana:{agent_pda_base58}`

The DID document includes verification methods (Ed25519 keys) and service endpoints (sidecar URL, agent metadata).

## Revocation

- Parent agents can revoke sub-agent delegations via `DELETE /agents/:did/delegation/:child_did`
- API key rotation: change `BASTION_API_KEY` env var and restart sidecar
- Payment replay protection: used tx hashes are tracked and rejected
