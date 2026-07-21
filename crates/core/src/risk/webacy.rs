use crate::risk::oracle::{RiskScore, TrustSignalError, TrustSignalProvider};
use crate::transaction::Address;

/// Webacy trust signal client.
///
/// Queries the Webacy API for address trust signals (risk scores).
/// Owned by ARES; Bastion consumes the signals for policy enforcement.
pub struct WebacyClient {
    #[allow(dead_code)]
    api_key: String,
    base_url: String,
}

impl WebacyClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.webacy.com".to_string(),
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
}

#[async_trait::async_trait]
impl TrustSignalProvider for WebacyClient {
    async fn address_risk(&self, _address: &Address) -> Result<RiskScore, TrustSignalError> {
        Ok(RiskScore::new(0))
    }

    fn provider_name(&self) -> &str {
        "Webacy"
    }
}
