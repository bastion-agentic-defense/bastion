# Bastion Web2 SDK

Web2 adapter for the Bastion Programmable Trust Runtime. Policy-enforced API calls for autonomous systems.

## Installation

```bash
npm install @zkos-labs/web2-sdk
pnpm add @zkos-labs/web2-sdk
```

## Quick Start

```typescript
import { BastionWeb2Client } from "@zkos-labs/web2-sdk";

const client = new BastionWeb2Client({
  proxyUrl: "http://localhost:3000",
  apiKey: "your-api-key",     // optional
});

// Build a proxy request
const req = client.buildRequest(
  "POST",
  "https://api.openai.com/v1/chat/completions",
  { "Content-Type": "application/json" },
  JSON.stringify({ model: "gpt-4", messages: [...] }),
  "agent-01",                 // optional agent ID
);

// Evaluates against policy rules without proxying
const decision = await client.evaluate(req);
// decision.decision = "pass" | "block" | "pending_hitl" | "log_only"

// Proxy the request through Bastion (blocked if policy fails)
const response = await client.proxyRequest(req);
// response.proxied = true | false
```

## API

| Method | Description |
|--------|-------------|
| `buildRequest(method, url, headers, body?, agentId?)` | Build a ProxyRequest object with provider auto-detection |
| `detectProvider(url)` | Auto-detect provider from URL (openai, stripe, github, slack, aws) |
| `proxyRequest(req)` | Send request through the proxy for evaluation and forwarding |
| `evaluate(req)` | Check if a request passes policy without proxying |
| `getPolicy()` | Fetch current proxy policy configuration |
| `updatePolicy(rules)` | Update proxy policy rules (provider budgets, allowlists, rate limits) |

## Policy Rules

Configure via the Bastion sidecar API. Supported rule types:

- Endpoint path and method allowlists
- Provider budget enforcement (spend caps per time window)
- Rate limiting per provider
- PII and secrets detection
- Prompt injection detection
- Header allowlist and blocklist filtering

## License

Apache-2.0
