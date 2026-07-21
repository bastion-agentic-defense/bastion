# Bastion MCP Server

Model Context Protocol server for the Bastion Programmable Trust Runtime. 15 tools and 3 prompts for AI agents to interact with Bastion trust operations over SSE or stdio transport.

## Installation

```bash
npm install @zkos-labs/mcp-server
pnpm add @zkos-labs/mcp-server
```

## Quick Start

```bash
# SSE transport for remote agents
BASTION_SIDECAR_URL=https://bastion-agentique.fly.dev \
npx @zkos-labs/mcp-server dev:http

# stdio transport for Claude Desktop / Cursor / Codex
pnpm --filter @bastion/mcp-server dev
```

## Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /mcp/sse` | SSE connection |
| `POST /mcp/messages` | MCP JSON-RPC messages |
| `GET /mcp/health` | Health check |
| `GET /mcp/pricing` | Tool pricing and free tier info |

## Tools

### Firewall
| Tool | Description |
|------|-------------|
| `bastion_simulate_transaction` | Simulate a Solana transaction against Bastion policy engine |
| `bastion_ingest_event` | Ingest a universal SIEM event for correlation |
| `bastion_audit_entry` | Retrieve an audit log entry by TX signature |
| `bastion_recent_audit_logs` | Fetch recent audit log entries |
| `bastion_audit_stats` | Get audit statistics (total, allowed, blocked) |
| `bastion_analyze_transaction` | Simulate an EVM transaction against Bastion policy engine |

### Policy Management
| Tool | Description |
|------|-------------|
| `bastion_get_policy` | Fetch current policy configuration |
| `bastion_update_policy` | Update policy (allowlists, caps, rate limits) |
| `bastion_override_block` | Human override for a blocked transaction |

### Agent Identity
| Tool | Description |
|------|-------------|
| `bastion_list_agents` | List all registered agents |
| `bastion_get_agent_by_did` | Get agent details by DID |
| `bastion_get_agent_delegation_tree` | View agent delegation hierarchy |

### Operations
| Tool | Description |
|------|-------------|
| `bastion_circuit_breaker_toggle` | Toggle emergency circuit breaker |
| `bastion_health_check` | Check Bastion sidecar health |
| `bastion_system_stats` | Get system statistics |

## Pricing

Read operations are free. Write operations use x402 pay-per-call pricing with Solana SOL.

| Tool | Free per Month | Price (SOL) |
|------|---------------|-------------|
| `bastion_simulate_transaction` | 100 | 0.001 |
| `bastion_override_block` | 10 | 0.01 |
| `bastion_update_policy` | 5 | 0.05 |
| `bastion_circuit_breaker_toggle` | 3 | 0.1 |
| All read-only tools | Unlimited | Free |

## License

Apache-2.0
