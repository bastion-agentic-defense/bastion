use crate::transaction::Address;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Trust signal for an address. 0 = trusted, 100 = high risk.
///
/// Scores are produced by ARES (or any trust-signal provider); Bastion
/// consumes them to make policy decisions. Bastion never computes scores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskScore(pub u8);

impl RiskScore {
    pub fn new(score: u8) -> Self {
        Self(score.min(100))
    }

    pub fn is_low_risk(&self) -> bool {
        self.0 <= 25
    }

    pub fn is_medium_risk(&self) -> bool {
        self.0 > 25 && self.0 <= 60
    }

    pub fn is_high_risk(&self) -> bool {
        self.0 > 60
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

/// Error type for trust signal provider operations.
#[derive(Debug, Error)]
pub enum TrustSignalError {
    #[error("provider error: {0}")]
    ProviderError(String),
    #[error("timeout")]
    Timeout,
    #[error("rate limited")]
    RateLimited,
}

/// Trait for trust signal providers.
///
/// Bastion **consumes** trust signals from ARES (or any provider)
/// to enforce policy — it never computes intelligence.
///
/// Implementations include GrondOSINT (owned by ARES), Chainalysis,
/// TRM Labs, and internal reputation models (ARES-owned).
#[async_trait::async_trait]
pub trait TrustSignalProvider: Send + Sync {
    /// Returns a trust signal (risk 0-100) for the given address.
    async fn address_risk(&self, address: &Address) -> Result<RiskScore, TrustSignalError>;

    /// Human-readable name of this provider.
    fn provider_name(&self) -> &str;
}
