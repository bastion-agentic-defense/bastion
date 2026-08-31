# Bastion - Programmable Trust Runtime for AI Agents

Bastion is a **Programmable Trust Runtime** for autonomous AI agents
on Solana, EVM (Celo, Base, Ethereum, Polygon), and Arcium. It orchestrates
identity, policy, privacy, durable execution, and multi-chain settlement.

## What Bastion does

- **Transaction simulation** - Predicts balance changes via Helius before signing
- **Policy engine** - Program whitelists, native token caps, rate limits, blockint rules
- **On-chain audit** - Immutable audit trail on Solana and EVM (Anchor + Solidity)
- **Agent registry** - On-chain agent identity with W3C DID + reputation
- **Human-in-the-loop** - Manual override for suspicious transactions
- **Circuit breaker** - Emergency pause for the entire protocol
- **GrondOSINT oracle** - Address risk scoring via agentic OSINT pipeline

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/simulate` | Simulate and evaluate a transaction |
| GET | `/policy` | Get current policy configuration |
| POST | `/policy` | Update policy settings |
| GET | `/logs` | Retrieve audit logs |
| POST | `/override` | Human override for blocked transactions |
| GET | `/health` | Server health check |
| GET | `/circuit-breaker/status` | Check circuit breaker state |
| POST | `/circuit-breaker/engage` | Activate circuit breaker |
| POST | `/circuit-breaker/disengage` | Deactivate circuit breaker |
| POST | `/api/v2/evaluate` | Chain-agnostic policy evaluation |

## Quick Start

```bash
git clone https://github.com/zkos-labs/bastion.git
cd bastion
cargo build --release
export HELIUS_API_KEY="your-api-key"
export GROND_API_URL="http://localhost:8000"  # optional
cargo run --release
```

## Links

- **GitHub**: https://github.com/zkos-labs/bastion
- **Docs**: https://github.com/zkos-labs/bastion#readme
- **SDK**: https://www.npmjs.com/package/@zkos-labs/bastion-agentique
- **Grond OSINT**: https://github.com/zkos-labs/Grond
