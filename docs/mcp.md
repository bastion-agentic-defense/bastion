# MCP server

Bastion ships a Model Context Protocol server, so a coding agent can query
policy, inspect the audit trail, and operate the circuit breaker as ordinary
tool calls.

> The MCP server is partial. Read-only tools are stable; the mutating tools work
> but their schemas may still change before the first stable release.

## Install

```bash
npm install -g @zkos-labs/mcp-server
```

## Claude Code

Add it once and it loads on every session:

```bash
claude mcp add bastion -- npx -y @zkos-labs/mcp-server
```

Or configure it by hand in `~/.claude.json`:

```json
{
  "mcpServers": {
    "bastion": {
      "command": "npx",
      "args": ["-y", "@zkos-labs/mcp-server"],
      "env": {
        "BASTION_URL": "https://bastion-agentique.fly.dev",
        "BASTION_TOKEN": "<your session token>"
      }
    }
  }
}
```

## Other clients

Any MCP-capable client works. The server speaks stdio and takes its configuration
from the environment:

| Variable | Purpose |
| --- | --- |
| `BASTION_URL` | Sidecar base URL. |
| `BASTION_TOKEN` | Session token from `POST /auth/verify`. |
| `BASTION_DID` | Default agent DID for tools that take one. |

A discovery document is published at `/.well-known/mcp/server-card.json`.

## Tools

### Read-only

| Tool | Returns |
| --- | --- |
| `bastion_get_policy` | Current effective policy. |
| `bastion_get_audit_logs` | Decision history, filterable by agent and verdict. |
| `bastion_get_audit_stats` | Aggregate counts by verdict and rule. |
| `bastion_get_pending_approvals` | Decisions held for human review. |
| `bastion_circuit_breaker_status` | Whether the fleet-wide stop is engaged. |
| `bastion_get_correlation_alerts` | Alerts raised by the correlation engine. |
| `bastion_get_token_balances` | Token balances for an agent's accounts. |
| `bastion_resolve_did` | Resolve a DID to its document. |
| `bastion_list_cases` | Open investigation cases. |

### Mutating

These change runtime state. Treat them as privileged.

| Tool | Effect |
| --- | --- |
| `bastion_simulate_transaction` | Simulate against live chain state. No state change. |
| `bastion_verify_transaction` | Full policy evaluation, returning a verdict. |
| `bastion_update_policy` | Replace the effective policy. |
| `bastion_override_block` | Approve or reject a held decision. |
| `bastion_circuit_breaker_toggle` | Engage or disengage the fleet-wide stop. |
| `bastion_ingest_event` | Push an external security event into correlation. |
| `bastion_create_case` | Open an investigation case. |
| `bastion_update_case` | Update an existing case. |
| `bastion_register_robot` | Register a robotic agent. |
| `bastion_robot_telemetry` | Report telemetry for a robotic agent. |
| `bastion_security_review` | Run a structured security review. |
| `bastion_incident_response` | Execute an incident response playbook. |

## Browser-native use

The server card and a browser shim are served from the web app, so a page can
expose the same tools to an in-page agent without a local process:

```html
<script src="https://bastionagentique.com/webmcp.js"></script>
```

## A note on trust

Giving an agent `bastion_update_policy` or `bastion_circuit_breaker_toggle` hands
it the controls that constrain it. Issue a token scoped to the read-only tools
unless you specifically intend otherwise.
