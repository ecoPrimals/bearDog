//! Health check timeout configuration
//!
//! Provides timeout settings for health check operations.

use super::core::{read_env_timeout_secs, FromEnvironment};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Health check timeout configuration
///
/// Default: 5 seconds (industry standard for health checks)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct HealthTimeouts {
    /// Health check timeout in seconds
    ///
    /// Default: 5 seconds
    /// Environment: `BEARDOG_HEALTH_CHECK_TIMEOUT_SECS`
    pub check_timeout_secs: u64,
}

impl Default for HealthTimeouts {
    fn default() -> Self {
        Self {
            check_timeout_secs: 5,
        }
    }
}

impl FromEnvironment for HealthTimeouts {
    fn from_env() -> Self {
        Self {
            check_timeout_secs: read_env_timeout_secs(
                "BEARDOG_HEALTH_CHECK_TIMEOUT_SECS",
                Self::default().check_timeout_secs,
            ),
        }
    }

    fn try_from_env() -> Option<Self> {
        std::env::var("BEARDOG_HEALTH_CHECK_TIMEOUT_SECS")
            .ok()
            .map(|_| Self::from_env())
    }
}

impl HealthTimeouts {
    /// Get health check timeout as Duration
    pub fn check_timeout(&self) -> Duration {
        Duration::from_secs(self.check_timeout_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let timeouts = HealthTimeouts::default();
        assert_eq!(timeouts.check_timeout_secs, 5);
    }

    #[test]
    fn test_check_timeout_duration() {
        let timeouts = HealthTimeouts {
            check_timeout_secs: 10,
        };
        assert_eq!(timeouts.check_timeout(), Duration::from_secs(10));
    }

    #[test]
    fn test_serialization() {
        let timeouts = HealthTimeouts::default();
        let json = serde_json::to_string(&timeouts).unwrap();
        let deserialized: HealthTimeouts = serde_json::from_str(&json).unwrap();
        assert_eq!(timeouts, deserialized);
    }
}
