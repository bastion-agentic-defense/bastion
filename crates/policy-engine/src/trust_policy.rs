use bastion_core::policy::types::PolicyRule;
use bastion_core::transaction::Address;
use serde::{Deserialize, Serialize};

/// The enforcement mode of a TrustPolicy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PolicyMode {
    /// Log decisions but do not block.
    Audit,
    /// Block non-compliant actions.
    Enforce,
}

/// A Kyverno-style declarative TrustPolicy resource.
///
/// Policies are first-class YAML resources, not embedded code.
/// The runtime evaluates every TrustIntent against matching TrustPolicies
/// and returns a combined decision.
///
/// ```yaml
/// apiVersion: bastion.io/v1
/// kind: TrustPolicy
/// metadata:
///   name: treasury-guard
/// spec:
///   match:
///     intent: transfer
///   validate:
///     maxPerTransaction: 5000
///   mutate:
///     injectHumanApproval:
///       triggerAbove: 1000
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrustPolicy {
    /// API version: bastion.io/v1
    pub api_version: String,

    /// Resource kind: TrustPolicy
    pub kind: String,

    /// Policy metadata (name, agent binding, labels)
    pub metadata: PolicyMetadata,

    /// Policy specification (match, validate, mutate, generate, exceptions)
    pub spec: PolicySpec,

    /// Runtime state (managed by the engine)
    #[serde(skip_deserializing, default)]
    pub status: Option<PolicyStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyMetadata {
    /// Unique policy name.
    pub name: String,

    /// Agent DID this policy applies to. Empty = applies to all agents.
    #[serde(default)]
    pub agent: Option<String>,

    /// Human-readable labels for categorization.
    #[serde(default)]
    pub labels: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PolicySpec {
    /// Matching criteria. Only actions matching these criteria are subject to this policy.
    #[serde(default)]
    pub r#match: MatchCriteria,

    /// Validation rules. Each rule must pass for the action to be allowed.
    #[serde(default)]
    pub validate: ValidateRules,

    /// Mutation rules. Inject additional checks or transforms into the execution plan.
    #[serde(default)]
    pub mutate: MutateRules,

    /// Generation rules. Produce derived resources (attestations, audit events, settlement records).
    #[serde(default)]
    pub generate: GenerateRules,

    /// Background scanning configuration.
    #[serde(default)]
    pub background: BackgroundConfig,

    /// Time-bound trust exceptions.
    #[serde(default)]
    pub exceptions: Vec<TrustException>,
}

/// Criteria for matching a TrustPolicy to an action.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchCriteria {
    /// Match by intent type: transfer, swap, stake, mint, etc.
    #[serde(default)]
    pub intent: Option<String>,

    /// Match by execution chain. Empty = all chains.
    #[serde(default)]
    pub chains: Vec<String>,

    /// Match by currency/token. Empty = all currencies.
    #[serde(default)]
    pub currency: Option<String>,
}

/// Validation rules mapped from PolicyRule enum variants.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidateRules {
    /// Cap the maximum value per single transaction.
    pub max_per_transaction: Option<u64>,

    /// Cap the maximum value per 24-hour window.
    pub max_per_24h: Option<u64>,

    /// Allowed destination addresses.
    #[serde(default)]
    pub allowlist: Vec<String>,

    /// Blocked destination addresses.
    #[serde(default)]
    pub blocklist: Vec<String>,

    /// Max transactions per hour.
    pub max_transactions_per_hour: Option<u32>,

    /// Allowed chain IDs.
    #[serde(default)]
    pub allowed_chains: Vec<String>,

    /// Allowed transaction types.
    #[serde(default)]
    pub allowed_tx_types: Vec<String>,

    /// Minimum agent reputation score.
    pub min_reputation_score: Option<u8>,

    /// Geographic boundary (robot/physical agents).
    pub geofence: Option<GeofenceBounds>,

    /// Maximum physical speed (robot agents).
    pub max_speed_mps: Option<f64>,

    /// Maximum energy budget per 24h (robot agents).
    pub max_joules_24h: Option<u64>,

    /// Operating hours (robot agents).
    pub operating_hours: Option<OperatingHoursRange>,

    /// Minimum SOL staked.
    pub min_stake_sol: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeofenceBounds {
    pub lat_min: f64,
    pub lon_min: f64,
    pub lat_max: f64,
    pub lon_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatingHoursRange {
    pub min_hour: u8,
    pub max_hour: u8,
}

/// Mutation rules. Inject checks into the execution plan.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutateRules {
    /// Require human approval for transactions above a threshold.
    pub inject_human_approval: Option<HitlConfig>,

    /// Inject a budget check against the agent's remaining allowance.
    pub inject_budget_check: Option<bool>,

    /// Inject pre-execution simulation.
    pub inject_simulation: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HitlConfig {
    pub trigger_above: u64,
    pub timeout_seconds: u64,
}

/// Generation rules. Produce derived trust resources.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerateRules {
    /// Generate an attestation record.
    pub attestation: Option<bool>,

    /// Generate an approval request for human review.
    pub approval_request: Option<bool>,

    /// Generate an audit event.
    pub audit_event: Option<bool>,

    /// Generate a settlement record.
    pub settlement_record: Option<bool>,
}

/// Background scanning configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundConfig {
    /// Scan interval in seconds.
    #[serde(default = "default_scan_interval")]
    pub scan_interval: u64,

    /// Detect expired HITL approvals.
    #[serde(default = "default_true")]
    pub detect_expired_approvals: bool,

    /// Detect expired delegations.
    #[serde(default = "default_true")]
    pub detect_expired_delegations: bool,

    /// Detect policy drift (config vs on-chain).
    #[serde(default = "default_true")]
    pub detect_policy_drift: bool,

    /// Detect unsettled transactions.
    #[serde(default = "default_true")]
    pub detect_unsettled_transactions: bool,
}

impl Default for BackgroundConfig {
    fn default() -> Self {
        Self {
            scan_interval: default_scan_interval(),
            detect_expired_approvals: default_true(),
            detect_expired_delegations: default_true(),
            detect_policy_drift: default_true(),
            detect_unsettled_transactions: default_true(),
        }
    }
}

const fn default_scan_interval() -> u64 { 3600 }
const fn default_true() -> bool { true }

/// A time-bound exception to a TrustPolicy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrustException {
    /// Human-readable reason for the exception.
    pub reason: String,

    /// ISO 8601 duration string (e.g. "24h", "7d").
    pub expires: String,

    /// Who approved this exception.
    pub approved_by: String,
}

/// Runtime status of a TrustPolicy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyStatus {
    /// Current enforcement mode.
    pub mode: PolicyMode,

    /// Number of actions evaluated.
    pub evaluations: u64,

    /// Number of actions blocked.
    pub blocks: u64,

    /// Last evaluation timestamp (Unix seconds).
    pub last_evaluation: u64,

    /// Last scan results.
    pub last_scan: Option<ScanResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    pub timestamp: u64,
    pub expired_approvals: u64,
    pub expired_delegations: u64,
    pub policy_drifts: u64,
    pub unsettled_transactions: u64,
}

impl TrustPolicy {
    /// Parse a TrustPolicy from YAML bytes.
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    /// Convert this TrustPolicy into a Vec of PolicyRule for the existing engine.
    pub fn to_policy_rules(&self) -> Vec<PolicyRule> {
        let mut rules = Vec::new();

        // AmountLimit
        if let Some(max) = self.spec.validate.max_per_transaction {
            rules.push(PolicyRule::AmountLimit {
                max_per_transaction: max,
                max_per_24h: self.spec.validate.max_per_24h,
                currency: self.spec.r#match.currency.clone().unwrap_or_else(|| "SOL".into()),
            });
        }

        // Destination
        if !self.spec.validate.allowlist.is_empty() || !self.spec.validate.blocklist.is_empty() {
            rules.push(PolicyRule::Destination {
                allowlist: self.spec.validate.allowlist.iter().map(|a| Address::new(a.clone())).collect(),
                blocklist: self.spec.validate.blocklist.iter().map(|a| Address::new(a.clone())).collect(),
            });
        }

        // Frequency
        if let Some(max) = self.spec.validate.max_transactions_per_hour {
            rules.push(PolicyRule::Frequency { max_transactions_per_hour: max });
        }

        // HITL
        if let Some(hitl) = &self.spec.mutate.inject_human_approval {
            rules.push(PolicyRule::HITL {
                trigger_above: hitl.trigger_above,
                timeout_seconds: hitl.timeout_seconds,
            });
        }

        // Reputation
        if let Some(score) = self.spec.validate.min_reputation_score {
            rules.push(PolicyRule::Reputation { minimum_score: score, elevated_limit_multiplier: None });
        }

        // TxTypeAllowlist
        if !self.spec.validate.allowed_tx_types.is_empty() {
            rules.push(PolicyRule::TxTypeAllowlist {
                allowed: self.spec.validate.allowed_tx_types.clone(),
            });
        }

        // StakeWeighted
        if let Some(min_stake) = self.spec.validate.min_stake_sol {
            rules.push(PolicyRule::StakeWeighted {
                base_limit: self.spec.validate.max_per_transaction.unwrap_or(1000),
                min_stake,
                stake_multiplier: 1.0,
                depth_decay_factor: 0.5,
            });
        }

        // Geofence
        if let Some(geo) = &self.spec.validate.geofence {
            rules.push(PolicyRule::Geofence {
                lat_min: geo.lat_min,
                lon_min: geo.lon_min,
                lat_max: geo.lat_max,
                lon_max: geo.lon_max,
            });
        }

        // SpeedLimit
        if let Some(speed) = self.spec.validate.max_speed_mps {
            rules.push(PolicyRule::SpeedLimit { max_speed_mps: speed });
        }

        // EnergyBudget
        if let Some(joules) = self.spec.validate.max_joules_24h {
            rules.push(PolicyRule::EnergyBudget { max_joules_24h: joules });
        }

        // OperatingHours
        if let Some(hours) = &self.spec.validate.operating_hours {
            rules.push(PolicyRule::OperatingHours { min_hour: hours.min_hour, max_hour: hours.max_hour });
        }

        rules
    }

    /// Check if this policy matches a given intent and chain.
    pub fn matches(&self, intent: &str, chain: &str) -> bool {
        let m = &self.spec.r#match;
        if let Some(ref required_intent) = m.intent {
            if required_intent != intent {
                return false;
            }
        }
        if !m.chains.is_empty() && !m.chains.contains(&chain.to_string()) {
            return false;
        }
        true
    }
}
