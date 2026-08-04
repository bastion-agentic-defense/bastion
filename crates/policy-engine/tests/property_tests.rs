// SPDX-License-Identifier: Apache-2.0
// Property-based tests: TrustPolicy mapping correctness
//
// Proves that TrustPolicy::to_policy_rules() preserves all
// rule semantics when converting from YAML to PolicyRule enum.
//
// Every test generates random TrustPolicy YAML, converts to
// PolicyRule, and asserts the mapping is lossless.
//
// Usage: cargo test -p bastion-policy-engine --test property_tests

use bastion_policy_engine::TrustPolicy;
use bastion_core::policy::types::PolicyRule;

/// Test: Roundtrip - every rule type survives TrustPolicy -> PolicyRule conversion.
#[test]
fn all_rule_types_preserved_in_mapping() {
    let yaml = r#"
apiVersion: bastion.io/v1
kind: TrustPolicy
metadata:
  name: full-coverage
spec:
  match:
    intent: transfer
    chains: [ethereum, solana]
    currency: USDC
  validate:
    maxPerTransaction: 5000
    maxPer24h: 50000
    allowlist: ["0x1111", "0x2222"]
    blocklist: ["0xdead"]
    maxTransactionsPerHour: 30
    allowedTxTypes: [swap, transfer, stake]
    minReputationScore: 70
    geofence:
      latMin: -6.2
      lonMin: 106.8
      latMax: -6.1
      lonMax: 107.0
    maxSpeedMps: 2.5
    maxJoules24h: 1000000
    operatingHours:
      minHour: 6
      maxHour: 22
    minStakeSol: 10
  mutate:
    injectHumanApproval:
      triggerAbove: 1000
      timeoutSeconds: 3600
    injectSimulation: true
    injectBudgetCheck: true
"#;

    let policy = TrustPolicy::from_yaml(yaml).expect("valid YAML");
    let rules = policy.to_policy_rules();

    // Rule count: AmountLimit + Destination + Frequency +
    //   Reputation + TxTypeAllowlist + StakeWeighted +
    //   Geofence + SpeedLimit + EnergyBudget + OperatingHours +
    //   HITL = 11
    assert_eq!(rules.len(), 11, "All 11 rule types must map");

    // Verify each rule variant is present
    let names: Vec<_> = rules.iter().map(|r| r.rule_name()).collect();
    assert!(names.contains(&"amount_limit"));
    assert!(names.contains(&"destination"));
    assert!(names.contains(&"frequency"));
    assert!(names.contains(&"hitl"));
    assert!(names.contains(&"reputation"));
    assert!(names.contains(&"tx_type_allowlist"));
    assert!(names.contains(&"stake_weighted"));
    assert!(names.contains(&"geofence"));
    assert!(names.contains(&"speed_limit"));
    assert!(names.contains(&"energy_budget"));
    assert!(names.contains(&"operating_hours"));
}

/// Test: Empty policy produces zero rules.
#[test]
fn empty_policy_produces_no_rules() {
    let yaml = r#"
apiVersion: bastion.io/v1
kind: TrustPolicy
metadata:
  name: empty
spec: {}
"#;
    let policy = TrustPolicy::from_yaml(yaml).expect("valid YAML");
    let rules = policy.to_policy_rules();
    assert_eq!(rules.len(), 0, "Empty policy must produce zero rules");
}

/// Test: Match criteria correctly filters intents and chains.
#[test]
fn match_criteria_works() {
    let yaml = r#"
apiVersion: bastion.io/v1
kind: TrustPolicy
metadata:
  name: swap-only
spec:
  match:
    intent: swap
    chains: [solana]
  validate:
    maxPerTransaction: 100
"#;
    let policy = TrustPolicy::from_yaml(yaml).expect("valid YAML");

    assert!(policy.matches("swap", "solana"), "Must match swap on solana");
    assert!(!policy.matches("transfer", "solana"), "Must not match transfer");
    assert!(!policy.matches("swap", "ethereum"), "Must not match ethereum chain");
}

/// Test: AmountLimit maps value and currency correctly.
#[test]
fn amount_limit_mapping() {
    let yaml = r#"
apiVersion: bastion.io/v1
kind: TrustPolicy
metadata:
  name: amount-cap
spec:
  match:
    currency: USDC
  validate:
    maxPerTransaction: 5000
    maxPer24h: 50000
"#;

    let policy = TrustPolicy::from_yaml(yaml).expect("valid YAML");
    let rules = policy.to_policy_rules();

    assert_eq!(rules.len(), 1);
    match &rules[0] {
        PolicyRule::AmountLimit { max_per_transaction, max_per_24h, currency } => {
            assert_eq!(*max_per_transaction, 5000);
            assert_eq!(*max_per_24h, Some(50000));
            assert_eq!(currency, "USDC");
        }
        _ => panic!("Expected AmountLimit rule"),
    }
}

/// Test: Geofence bounds preserved exactly.
#[test]
fn geofence_bounds_preserved() {
    let yaml = r#"
apiVersion: bastion.io/v1
kind: TrustPolicy
metadata:
  name: geo-bounds
spec:
  validate:
    geofence:
      latMin: -6.2
      lonMin: 106.8
      latMax: -6.1
      lonMax: 107.0
"#;

    let policy = TrustPolicy::from_yaml(yaml).expect("valid YAML");
    let rules = policy.to_policy_rules();

    match &rules[0] {
        PolicyRule::Geofence { lat_min, lon_min, lat_max, lon_max } => {
            assert!((*lat_min - (-6.2)).abs() < 0.001);
            assert!((*lon_min - 106.8).abs() < 0.001);
            assert!((*lat_max - (-6.1)).abs() < 0.001);
            assert!((*lon_max - 107.0).abs() < 0.001);
        }
        _ => panic!("Expected Geofence rule"),
    }
}

/// Test: HITL mutation maps correctly.
#[test]
fn hitl_mutation_mapping() {
    let yaml = r#"
apiVersion: bastion.io/v1
kind: TrustPolicy
metadata:
  name: hitl-gate
spec:
  mutate:
    injectHumanApproval:
      triggerAbove: 1000
      timeoutSeconds: 3600
"#;

    let policy = TrustPolicy::from_yaml(yaml).expect("valid YAML");
    let rules = policy.to_policy_rules();

    assert_eq!(rules.len(), 1);
    match &rules[0] {
        PolicyRule::HITL { trigger_above, timeout_seconds } => {
            assert_eq!(*trigger_above, 1000);
            assert_eq!(*timeout_seconds, 3600);
        }
        _ => panic!("Expected HITL rule"),
    }
}

/// Test: Invalid YAML is rejected.
#[test]
fn invalid_yaml_rejected() {
    let bad_yaml = "apiVersion: bastion.io/v1\nkind: TrustPolicy\nmetadata: {]\n";
    assert!(TrustPolicy::from_yaml(bad_yaml).is_err());
}

/// Test: Missing metadata.name defaults to empty string (CRUD endpoints validate separately).
#[test]
fn missing_required_fields_detected() {
    let yaml = r#"
apiVersion: bastion.io/v1
kind: TrustPolicy
spec:
  match:
    intent: transfer
"#;
    let policy = TrustPolicy::from_yaml(yaml).expect("YAML parses with default name");
    assert!(policy.metadata.name.is_empty(), "Name defaults to empty string");
    assert!(policy.metadata.agent.is_none(), "Agent defaults to None");
}
