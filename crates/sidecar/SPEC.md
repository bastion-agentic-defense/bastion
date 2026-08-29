# Sidecar Simulation Spec

> Full-EVM. The legacy Solana `/simulate` route and its Helius/Alchemy wiring were
> removed in the full-EVM pivot (see [`docs/ARCHIVE.md`](../../docs/ARCHIVE.md)).
> The active simulation path is `POST /api/v2/simulate-evm`.

## /api/v2/simulate-evm Flow

```
POST /api/v2/simulate-evm { transaction: EvmTxParams, intent?, chain?, agentId? }
  |
  1. Normalize `chain` (trim, lowercase); default "celo" for back-compat.
  2. Select the per-chain `EvmSimulator` from the runtime map.
     No configured RPC for the chain -> HTTP 503 naming the missing <X>_RPC_URL.
  3. Run policy evaluation (allowlist, rate limit per agent, value caps).
  4. Simulate via `EvmSimulator::simulate_evm_tx` (`eth_call` + balance diff).
  5. Post-simulation checks (chain-agnostic):
     NoErrorCheck        -> simulation.error must be None
     MaxUnitsCheck       -> units_consumed <= max_units
     MaxBalanceDrainCheck-> |balance_change| <= max_drain
     FlashLoanCheck      -> detect flash loan patterns in logs
     HighSlippageCheck   -> detect high slippage in swap logs
  6. Log audit entry (sled DB).
  7. Return EvmSimulateResponse { allowed, decision, reason?, simulation_result?,
     risk_score?, risk_summary? }.
```

`EvmTxParams` is `{ from, to, value?, data?, gas?, gasPrice?, maxFeePerGas?,
maxPriorityFeePerGas?, nonce? }` (all hex strings; `from`/`to` required).

## Request / Response (JSON)

```json
// Request
{
  "chain": "base",
  "intent": "swap 1 ETH for USDC",
  "transaction": {
    "from": "0xAgent0000...",
    "to": "0xTarget0000...",
    "value": "0x0",
    "data": "0x"
  }
}

// Response (snake_case fields)
{
  "allowed": true,
  "decision": "pass",
  "reason": null,
  "simulation_result": { "logs": ["..."], "balance_changes": {}, "simulation_hash": null },
  "risk_score": 12,
  "risk_summary": "low"
}
```

## Post-Simulation Checks

| Check | Condition | Block Reason |
|-------|-----------|--------------|
| NoErrorCheck | simulation.error is None | "Transaction simulation failed: {error}" |
| MaxUnitsCheck | units_consumed <= max_units | "Exceeds max compute units" |
| MaxBalanceDrainCheck | \|balance_change\| <= max_drain | "Exceeds max balance drain" |
| FlashLoanCheck | no flash loan pattern in logs | "Flash loan pattern detected" |
| HighSlippageCheck | slippage <= threshold | "High slippage detected" |

## Chain Routing

EVM transactions route to a per-chain `EvmSimulator` selected by the request's
`chain` field (normalized lowercase). Each chain's simulator is enabled by its own
RPC env var; a chain with no configured RPC returns HTTP 503 naming the missing
var (it is **not** silently routed elsewhere).

| Input chain | Simulator | Enabled by |
|-------------|-----------|------------|
| "ethereum"  | `EvmSimulator` (`eth_call` + balance diff) | `ETH_RPC_URL` |
| "sepolia"   | `EvmSimulator` | `ETH_SEPOLIA_RPC_URL` |
| "base"      | `EvmSimulator` | `BASE_RPC_URL` |
| "celo" (default) | `EvmSimulator` | `CELO_RPC_URL` |
| "zksync"    | `EvmSimulator` | `ZKSYNC_RPC_URL` |
| "robinhood" | `EvmSimulator` | `ROBINHOOD_RPC_URL` |
| "monad"     | `EvmSimulator` | `MONAD_RPC_URL` |

Any other chain string with no configured RPC → 503 `EVM simulation for chain
'<x>' not configured (set <X>_RPC_URL to enable).`

## Existing Test Coverage

| Test File | Tests | Coverage |
|-----------|-------|----------|
| api_integration.rs | 4 | Homepage, policy round-trip, EVM 503 routing |
| execute_intent.rs | 4 | Chain-agnostic `/execute` |
| background_scan.rs | 2 | Background trust scan |
| simulation_checks.rs | 15 | Post-sim checks (chain-agnostic) |
| simulation_evm.rs (inline) | ~5 | `EvmSimulator` creation, chain labels, tx serialization |

## Test Coverage Gaps

| Gap | Priority | New Test |
|-----|----------|----------|
| EVM simulation happy path (live RPC) | Medium | test_simulate_evm_base, test_simulate_evm_celo |
| All chain routing variants | Medium | test_chain_routing_all_variants |
| Circuit breaker + simulate | Medium | test_circuit_breaker_blocks_simulate |
| Intent classification edges | Medium | test_intent_classification_edge_cases |
| Concurrent rate limits | Low | test_concurrent_simulate_rate_limits |