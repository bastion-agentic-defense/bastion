use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryPolicy {
    #[serde(default = "default_max_attempts")]
    pub max_attempts: u32,
    #[serde(default = "default_initial_backoff")]
    pub initial_backoff_ms: u64,
    #[serde(default = "default_max_backoff")]
    pub max_backoff_ms: u64,
    #[serde(default = "default_multiplier")]
    pub backoff_multiplier: f64,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

fn default_max_attempts() -> u32 { 1 }
fn default_initial_backoff() -> u64 { 1000 }
fn default_max_backoff() -> u64 { 30000 }
fn default_multiplier() -> f64 { 2.0 }
fn default_timeout() -> u64 { 30000 }

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: default_max_attempts(),
            initial_backoff_ms: default_initial_backoff(),
            max_backoff_ms: default_max_backoff(),
            backoff_multiplier: default_multiplier(),
            timeout_ms: default_timeout(),
        }
    }
}

impl RetryPolicy {
    pub fn backoff_ms(&self, attempt: u32) -> u64 {
        if attempt <= 1 {
            return 0;
        }
        let delay = self.initial_backoff_ms as f64
            * self.backoff_multiplier.powi((attempt as i32) - 2);
        (delay as u64).min(self.max_backoff_ms)
    }

    pub fn can_retry(&self, attempt: u32) -> bool {
        attempt < self.max_attempts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_no_retry_backoff_zero() {
        let rp = RetryPolicy::default();
        assert_eq!(rp.backoff_ms(1), 0);
    }

    #[test]
    fn exponential_backoff() {
        let rp = RetryPolicy {
            max_attempts: 5,
            initial_backoff_ms: 1000,
            max_backoff_ms: 30000,
            backoff_multiplier: 2.0,
            timeout_ms: 30000,
        };
        assert_eq!(rp.backoff_ms(1), 0);
        assert_eq!(rp.backoff_ms(2), 1000);
        assert_eq!(rp.backoff_ms(3), 2000);
        assert_eq!(rp.backoff_ms(4), 4000);
    }

    #[test]
    fn backoff_capped() {
        let rp = RetryPolicy {
            max_attempts: 10,
            initial_backoff_ms: 1000,
            max_backoff_ms: 5000,
            backoff_multiplier: 2.0,
            timeout_ms: 30000,
        };
        assert_eq!(rp.backoff_ms(5), 5000);
        assert_eq!(rp.backoff_ms(6), 5000);
    }

    #[test]
    fn can_retry_boundary() {
        let rp = RetryPolicy { max_attempts: 3, ..Default::default() };
        assert!(rp.can_retry(0));
        assert!(rp.can_retry(1));
        assert!(rp.can_retry(2));
        assert!(!rp.can_retry(3));
    }
}
