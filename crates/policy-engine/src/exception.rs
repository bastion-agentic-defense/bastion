use serde::{Deserialize, Serialize};

/// A time-bound exception to a TrustPolicy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrustException {
    pub id: String,
    pub policy_name: String,
    pub reason: String,
    pub expires_at: u64,
    pub approved_by: String,
    pub created_at: u64,
}

impl TrustException {
    pub fn new(policy_name: impl Into<String>, reason: impl Into<String>, expires_at: u64, approved_by: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            policy_name: policy_name.into(),
            reason: reason.into(),
            expires_at,
            approved_by: approved_by.into(),
            created_at: chrono::Utc::now().timestamp() as u64,
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp() as u64;
        now > self.expires_at
    }
}
