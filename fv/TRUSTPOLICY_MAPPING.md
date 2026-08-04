# TrustPolicy to PolicyRule Mapping

Formal specification of how declarative TrustPolicy YAML fields map to
the Rust `PolicyRule` enum variants. This mapping MUST be lossless: every
field that maps to a rule is represented, and no rule is silently dropped.

## Mapping Table

| TrustPolicy YAML field | PolicyRule variant | Required condition |
|------------------------|-------------------|-------------------|
| `validate.maxPerTransaction` | `AmountLimit { max_per_transaction }` | Non-None |
| `validate.maxPer24h` | `AmountLimit { max_per_24h }` | Non-None (paired with maxPerTransaction) |
| `match.currency` | `AmountLimit { currency }` | Non-None (defaults to "SOL") |
| `validate.allowlist` | `Destination { allowlist }` | Non-empty |
| `validate.blocklist` | `Destination { blocklist }` | Non-empty |
| `validate.maxTransactionsPerHour` | `Frequency { max_transactions_per_hour }` | Non-None |
| `mutate.injectHumanApproval.triggerAbove` | `HITL { trigger_above }` | Non-None (both fields) |
| `mutate.injectHumanApproval.timeoutSeconds` | `HITL { timeout_seconds }` | Non-None (both fields) |
| `validate.minReputationScore` | `Reputation { minimum_score }` | Non-None |
| `validate.allowedTxTypes` | `TxTypeAllowlist { allowed }` | Non-empty |
| `validate.minStakeSol` | `StakeWeighted { min_stake }` | Non-None |
| `validate.geofence` | `Geofence { lat_min, lon_min, lat_max, lon_max }` | Non-None |
| `validate.maxSpeedMps` | `SpeedLimit { max_speed_mps }` | Non-None |
| `validate.maxJoules24h` | `EnergyBudget { max_joules_24h }` | Non-None |
| `validate.operatingHours` | `OperatingHours { min_hour, max_hour }` | Non-None |

## Invariants (proven by property tests)

1. **Roundtrip completeness**: A TrustPolicy with all 11 fields set produces exactly 11 PolicyRule variants. No rules are dropped. (`tests/property_tests.rs::all_rule_types_preserved_in_mapping`)

2. **Empty policy safety**: A TrustPolicy with an empty spec produces zero rules. No phantom rules are created. (`tests/property_tests.rs::empty_policy_produces_no_rules`)

3. **Value preservation**: Numeric and bounds values are preserved exactly through the mapping. (`tests/property_tests.rs::amount_limit_mapping`, `geofence_bounds_preserved`)

4. **HITL correctness**: HITL trigger and timeout values are preserved through the mapping. (`tests/property_tests.rs::hitl_mutation_mapping`)

5. **Match filtering**: The `matches()` method correctly filters by intent and chain. (`tests/property_tests.rs::match_criteria_works`)

## Future work

- TLA+ proof that `to_policy_rules()` is injective (no two different TrustPolicies produce identical PolicyRule sets)
- Fuzzing with proptest for edge cases (negative amounts, zero values, boundary conditions)
- Integration with the PolicyEvaluator TLA+ spec to prove end-to-end correctness
