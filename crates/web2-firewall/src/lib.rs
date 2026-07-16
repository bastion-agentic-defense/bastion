mod providers;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Returns the current hour (0-23) in UTC.
fn current_utc_hour() -> u8 {
    ((SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / 3600)
        % 24) as u8
}

/// A normalized API call event from an AI agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEvent {
    pub id: String,
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub provider: String,
    pub agent_id: String,
    pub timestamp: u64,
}

impl ApiEvent {
    pub fn path(&self) -> Option<String> {
        url::Url::parse(&self.url)
            .ok()
            .map(|u| u.path().to_string())
    }

    pub fn method_upper(&self) -> String {
        self.method.to_uppercase()
    }
}

/// What to do with an API call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProxyDecision {
    Pass,
    Block {
        reason: String,
        rule_id: Option<String>,
    },
    PendingHITL {
        approval_id: String,
        reason: String,
    },
    LogOnly,
}

impl ProxyDecision {
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Pass | Self::LogOnly)
    }

    pub fn is_blocked(&self) -> bool {
        matches!(self, Self::Block { .. } | Self::PendingHITL { .. })
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, Self::PendingHITL { .. })
    }
}

impl From<bastion_core::FirewallDecision> for ProxyDecision {
    fn from(fd: bastion_core::FirewallDecision) -> Self {
        match fd {
            bastion_core::FirewallDecision::Pass => ProxyDecision::Pass,
            bastion_core::FirewallDecision::Block { reason, policy_id } => ProxyDecision::Block {
                reason,
                rule_id: policy_id,
            },
            bastion_core::FirewallDecision::PendingHITL {
                approval_id,
                reason,
            } => ProxyDecision::PendingHITL {
                approval_id,
                reason,
            },
        }
    }
}

/// API-level policy rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiPolicyRule {
    EndpointAllowlist {
        paths: Vec<String>,
        methods: Vec<String>,
    },
    EndpointBlocklist {
        patterns: Vec<String>,
    },
    ProviderBudget {
        provider: String,
        max_usd_cents_per_window: u64,
        window_minutes: u64,
    },
    RateLimitPerProvider {
        provider: String,
        max_requests_per_minute: u32,
    },
    ContentInspection {
        detect_pii: bool,
        detect_secrets: bool,
        detect_prompt_injection: bool,
    },
    HeaderFilter {
        allow_headers: Vec<String>,
        block_headers: Vec<String>,
    },
    CostCap {
        max_usd_cents_per_month: u64,
    },
    TimeOfDayRestriction {
        min_hour_utc: u8,
        max_hour_utc: u8,
    },
}

impl ApiPolicyRule {
    pub fn rule_name(&self) -> &'static str {
        match self {
            Self::EndpointAllowlist { .. } => "endpoint_allowlist",
            Self::EndpointBlocklist { .. } => "endpoint_blocklist",
            Self::ProviderBudget { .. } => "provider_budget",
            Self::RateLimitPerProvider { .. } => "rate_limit_per_provider",
            Self::ContentInspection { .. } => "content_inspection",
            Self::HeaderFilter { .. } => "header_filter",
            Self::CostCap { .. } => "cost_cap",
            Self::TimeOfDayRestriction { .. } => "time_of_day_restriction",
        }
    }

    /// Returns true if this rule matches (i.e. should block) the event.
    pub fn matches(&self, event: &ApiEvent) -> bool {
        match self {
            Self::EndpointAllowlist { paths, methods } => {
                let path = event.path().unwrap_or_default();
                let method = event.method_upper();
                !(paths.contains(&path) && methods.contains(&method))
            }
            Self::EndpointBlocklist { patterns } => {
                let path = event.path().unwrap_or_default();
                patterns.iter().any(|p| path.contains(p.as_str()))
            }
            Self::ProviderBudget { .. } | Self::RateLimitPerProvider { .. } => {
                // Budget/rate rules always match (they track state internally)
                // The proxy engine calls match to check if the rule applies to the provider
                true
            }
            Self::ContentInspection {
                detect_pii: _,
                detect_secrets,
                detect_prompt_injection: _,
            } => {
                if *detect_secrets {
                    let body = &event.body;
                    if body.contains("sk-")
                        || body.contains("sk-proj-")
                        || body.contains("ghp_")
                        || body.contains("github_pat_")
                    {
                        return true;
                    }
                }
                false
            }
            Self::HeaderFilter {
                allow_headers: _allow_headers,
                block_headers: _block_headers,
            } => self.matches_any_blocked_header(event),
            Self::CostCap { .. } => false,
            Self::TimeOfDayRestriction {
                min_hour_utc,
                max_hour_utc,
            } => {
                // Blocks when the current UTC hour is outside [min, max].
                let hour = current_utc_hour();
                hour < *min_hour_utc || hour > *max_hour_utc
            }
        }
    }

    pub fn matches_any_blocked_header(&self, event: &ApiEvent) -> bool {
        if let Self::HeaderFilter {
            block_headers,
            allow_headers: _,
        } = self
        {
            let lower_headers: Vec<String> = event
                .headers
                .iter()
                .map(|(k, _)| k.to_lowercase())
                .collect();
            block_headers
                .iter()
                .any(|bh| lower_headers.contains(&bh.to_lowercase()))
        } else {
            false
        }
    }
}

/// Parsed OpenAPI specification.
#[derive(Debug, Clone)]
pub struct OpenApiSpec {
    title: String,
    version: String,
    endpoints: Vec<(String, String, Vec<String>)>, // (path, method, tags)
}

impl OpenApiSpec {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let spec: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {e}"))?;

        let title = spec["info"]["title"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();
        let version = spec["info"]["version"]
            .as_str()
            .unwrap_or("0.0.0")
            .to_string();

        let mut endpoints = Vec::new();
        if let Some(paths) = spec["paths"].as_object() {
            for (path, methods) in paths {
                if let Some(methods_obj) = methods.as_object() {
                    for method in methods_obj.keys() {
                        let method_upper = method.to_uppercase();
                        let tags = methods_obj[method]["tags"]
                            .as_array()
                            .map(|t| {
                                t.iter()
                                    .filter_map(|v| v.as_str().map(String::from))
                                    .collect()
                            })
                            .unwrap_or_default();
                        if matches!(
                            method_upper.as_str(),
                            "GET" | "POST" | "PUT" | "DELETE" | "PATCH"
                        ) {
                            endpoints.push((path.clone(), method_upper, tags));
                        }
                    }
                }
            }
        }

        Ok(OpenApiSpec {
            title,
            version,
            endpoints,
        })
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn endpoint_allowlist(&self) -> Vec<(String, String)> {
        self.endpoints
            .iter()
            .map(|(p, m, _)| (p.clone(), m.clone()))
            .collect()
    }

    pub fn to_allowlist_rule(&self) -> ApiPolicyRule {
        let mut paths = Vec::new();
        let mut methods = Vec::new();
        for (path, method, _) in &self.endpoints {
            if !paths.contains(path) {
                paths.push(path.clone());
            }
            if !methods.contains(method) {
                methods.push(method.clone());
            }
        }
        ApiPolicyRule::EndpointAllowlist { paths, methods }
    }
}

/// Mutable, per-engine tracking state for stateful rules (rate limits and spend
/// budgets). Kept behind a `Mutex` so `evaluate` can stay `&self`.
#[derive(Debug, Default)]
struct EngineState {
    /// provider -> recent request instants (sliding window for rate limiting)
    request_times: HashMap<String, Vec<Instant>>,
    /// provider -> (accumulated cents, window start) for per-provider budgets
    provider_spend: HashMap<String, (u64, Instant)>,
    /// global (accumulated cents, window start) for the monthly cost cap
    monthly_spend: Option<(u64, Instant)>,
}

/// The Web2 proxy engine that evaluates API events against rules.
#[derive(Debug)]
pub struct ProxyEngine {
    rules: Vec<ApiPolicyRule>,
    state: Mutex<EngineState>,
}

impl ProxyEngine {
    pub fn new(rules: Vec<ApiPolicyRule>) -> Self {
        ProxyEngine {
            rules,
            state: Mutex::new(EngineState::default()),
        }
    }

    /// Evaluate an event with no associated spend. Budget/cost rules only bite
    /// when a cost is supplied via [`evaluate_with_cost`].
    pub fn evaluate(&self, event: &ApiEvent) -> ProxyDecision {
        self.evaluate_with_cost(event, 0)
    }

    /// Evaluate an event, attributing `cost_usd_cents` to the calling provider for
    /// budget/cost-cap accounting. Stateless rules are checked first, then the
    /// stateful rate/budget rules under the engine lock.
    pub fn evaluate_with_cost(&self, event: &ApiEvent, cost_usd_cents: u64) -> ProxyDecision {
        // ── Pass 1: stateless rules ──
        for rule in &self.rules {
            match rule {
                ApiPolicyRule::EndpointAllowlist { .. } if rule.matches(event) => {
                    return ProxyDecision::Block {
                        reason: format!(
                            "Endpoint not in allowlist: {} {}",
                            event.method_upper(),
                            event.path().unwrap_or_default()
                        ),
                        rule_id: Some("endpoint_allowlist".to_string()),
                    };
                }
                ApiPolicyRule::EndpointBlocklist { .. } if rule.matches(event) => {
                    return ProxyDecision::Block {
                        reason: format!(
                            "Endpoint matches blocklist pattern: {}",
                            event.path().unwrap_or_default()
                        ),
                        rule_id: Some("endpoint_blocklist".to_string()),
                    };
                }
                ApiPolicyRule::ContentInspection { .. } if rule.matches(event) => {
                    return ProxyDecision::Block {
                        reason: "Request body contains sensitive data (secret/PII detected)"
                            .to_string(),
                        rule_id: Some("content_inspection".to_string()),
                    };
                }
                ApiPolicyRule::HeaderFilter { block_headers, .. }
                    if rule.matches_any_blocked_header(event) =>
                {
                    let _blocked_headers: Vec<&str> = event
                        .headers
                        .iter()
                        .filter_map(|(k, _)| {
                            if block_headers
                                .iter()
                                .any(|b| k.to_lowercase() == b.to_lowercase())
                            {
                                Some(k.as_str())
                            } else {
                                None
                            }
                        })
                        .collect();
                    let h: Vec<&str> = event
                        .headers
                        .iter()
                        .filter_map(|(k, _)| {
                            if block_headers
                                .iter()
                                .any(|b| k.to_lowercase() == b.to_lowercase())
                            {
                                Some(k.as_str())
                            } else {
                                None
                            }
                        })
                        .collect();
                    return ProxyDecision::Block {
                        reason: format!("Blocked headers present: {}", h.join(", ")),
                        rule_id: Some("header_filter".to_string()),
                    };
                }
                ApiPolicyRule::TimeOfDayRestriction {
                    min_hour_utc,
                    max_hour_utc,
                } if rule.matches(event) => {
                    return ProxyDecision::Block {
                        reason: format!(
                            "Outside permitted hours: {} UTC not in {}-{} UTC",
                            current_utc_hour(),
                            min_hour_utc,
                            max_hour_utc
                        ),
                        rule_id: Some("time_of_day_restriction".to_string()),
                    };
                }
                _ => {}
            }
        }

        // ── Pass 2: stateful rate/budget rules ──
        let now = Instant::now();
        let mut st = self.state.lock().unwrap_or_else(|p| p.into_inner());
        for rule in &self.rules {
            match rule {
                ApiPolicyRule::RateLimitPerProvider {
                    provider,
                    max_requests_per_minute,
                } if event.provider == *provider => {
                    let window = Duration::from_secs(60);
                    let times = st.request_times.entry(provider.clone()).or_default();
                    times.retain(|t| now.duration_since(*t) < window);
                    times.push(now);
                    if times.len() as u32 > *max_requests_per_minute {
                        return ProxyDecision::Block {
                            reason: format!(
                                "Rate limit exceeded for {}: {} requests/min > {} allowed",
                                provider,
                                times.len(),
                                max_requests_per_minute
                            ),
                            rule_id: Some("rate_limit_per_provider".to_string()),
                        };
                    }
                }
                ApiPolicyRule::ProviderBudget {
                    provider,
                    max_usd_cents_per_window,
                    window_minutes,
                } if event.provider == *provider && cost_usd_cents > 0 => {
                    let window = Duration::from_secs(window_minutes.saturating_mul(60));
                    let entry = st
                        .provider_spend
                        .entry(provider.clone())
                        .or_insert((0, now));
                    if now.duration_since(entry.1) >= window {
                        *entry = (0, now);
                    }
                    entry.0 = entry.0.saturating_add(cost_usd_cents);
                    if entry.0 > *max_usd_cents_per_window {
                        return ProxyDecision::Block {
                            reason: format!(
                                "Provider budget exceeded for {}: {}¢ > {}¢ per {}min",
                                provider, entry.0, max_usd_cents_per_window, window_minutes
                            ),
                            rule_id: Some("provider_budget".to_string()),
                        };
                    }
                }
                ApiPolicyRule::CostCap {
                    max_usd_cents_per_month,
                } if cost_usd_cents > 0 => {
                    let window = Duration::from_secs(30 * 24 * 3600);
                    let entry = st.monthly_spend.get_or_insert((0, now));
                    if now.duration_since(entry.1) >= window {
                        *entry = (0, now);
                    }
                    entry.0 = entry.0.saturating_add(cost_usd_cents);
                    if entry.0 > *max_usd_cents_per_month {
                        return ProxyDecision::Block {
                            reason: format!(
                                "Monthly cost cap exceeded: {}¢ > {}¢",
                                entry.0, max_usd_cents_per_month
                            ),
                            rule_id: Some("cost_cap".to_string()),
                        };
                    }
                }
                _ => {}
            }
        }

        ProxyDecision::Pass
    }

    pub fn detect_provider(url: &str) -> Option<&'static str> {
        if url.contains("api.openai.com") {
            Some("openai")
        } else if url.contains("api.stripe.com") {
            Some("stripe")
        } else if url.contains("api.github.com") {
            Some("github")
        } else if url.contains("slack.com") {
            Some("slack")
        } else if url.contains("amazonaws.com") {
            Some("aws")
        } else {
            None
        }
    }
}

#[cfg(test)]
#[path = "lib_test.rs"]
mod tests;
