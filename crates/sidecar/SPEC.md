# Sidecar Simulation Spec

## /simulate Flow

```
POST /simulate { transaction: base64, intent?: string }
  |
  1. Decode base64 -> bincode -> solana_sdk::Transaction
  2. Check circuit breaker (is_paused? -> 503)
  3. Classify intent from transaction data (transfer/swap/stake/unknown)
  4. Policy check (allowlist, rate limit per agent, SOL caps)
  5. Simulate via Helius/Alchemy simulateTransaction RPC
  6. Post-simulation checks (parallel):
     NoErrorCheck        -> simulation.error must be None
     MaxUnitsCheck       -> units_consumed <= max_units
     MaxBalanceDrainCheck-> |balance_change| <= max_drain
     FlashLoanCheck      -> detect flash loan patterns in logs
     HighSlippageCheck   -> detect high slippage in swap logs
  7. Log audit entry (sled DB)
  8. Optionally write on-chain audit (Anchor program CPI)
  9. Return FirewallDecision { Pass | Block | PendingHITL }
```

## Post-Simulation Checks

| Check | Condition | Block Reason |
|-------|-----------|--------------|
| NoErrorCheck | simulation.error is None | "Transaction simulation failed: {error}" |
| MaxUnitsCheck | units_consumed <= max_units | "Exceeds max compute units" |
| MaxBalanceDrainCheck | |balance_change| <= max_drain | "Exceeds max balance drain" |
| FlashLoanCheck | no flash loan pattern in logs | "Flash loan pattern detected" |
| HighSlippageCheck | slippage <= threshold | "High slippage detected" |

## Chain Routing

EVM transactions (`/api/v2/simulate-evm`) route to a per-chain `EvmSimulator`
selected by the request's `chain` field (normalized lowercase). Each chain's
simulator is enabled by its own RPC env var; a chain with no configured RPC
returns HTTP 503 naming the missing var (it is **not** silently routed elsewhere).

| Input chain | Simulator | Enabled by |
|-------------|-----------|------------|
| "solana" or None | Helius/Alchemy `simulateTransaction` | `HELIUS_API_KEY` / `SOLANA_RPC_URL` |
| "ethereum" | `EvmSimulator` (`eth_call` + balance diff) | `ETH_RPC_URL` |
| "base" | `EvmSimulator` | `BASE_RPC_URL` |
| "celo" (default for EVM) | `EvmSimulator` | `CELO_RPC_URL` |
| "sepolia" | `EvmSimulator` (Ethereum testnet) | `ETH_SEPOLIA_RPC_URL` |

Any other chain string with no configured RPC → 503 `EVM simulation for chain
'<x>' not configured (set <X>_RPC_URL to enable).`

## Existing Test Coverage

| Test File | Lines | Tests | Coverage |
|-----------|-------|-------|----------|
| api_integration.rs | 1212 | 20+ | Full REST API |
| transaction_battery.rs | 814 | 15 | Transaction scenarios |
| policy_engine_suite.rs | 258 | 8 | Policy rules |
| simulation_checks.rs | 157 | 12 | Post-sim checks |
| core_adapter.rs (inline) | 90 | 5 | v2 evaluate + chain routing |

## Test Coverage Gaps

| Gap | Priority | New Test |
|-----|----------|----------|
| EVM simulation | Medium | test_simulate_evm_base, test_simulate_evm_celo |
| All chain routing | Medium | test_chain_routing_all_variants |
| Circuit breaker + simulate | Medium | test_circuit_breaker_blocks_simulate |
| Intent classification edges | Medium | test_intent_classification_edge_cases |
| Concurrent rate limits | Low | test_concurrent_simulate_rate_limits |
