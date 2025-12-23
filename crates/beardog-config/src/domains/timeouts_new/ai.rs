//! AI operation timeout configuration
//!
//! Provides timeout settings for AI decision-making and inference operations.

use super::core::{read_env_timeout_millis, read_env_timeout_secs, FromEnvironment};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// AI operation timeout configuration
///
/// Provides timeouts for AI decision-making and inference
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct AiTimeouts {
    /// AI decision timeout in seconds
    ///
    /// Default: 30 seconds (allows for complex AI decisions)
    /// Environment: `BEARDOG_DECISION_TIMEOUT_SECS`
    pub decision_timeout_secs: u64,

    /// AI request timeout in seconds
    ///
    /// Default: 30 seconds (inference operations)
    /// Environment: `BEARDOG_AI_REQUEST_TIMEOUT_SECS`
    pub request_timeout_secs: u64,

    /// AI batch timeout in milliseconds
    ///
    /// Default: 10 milliseconds (fast batching)
    /// Environment: `BEARDOG_AI_BATCH_TIMEOUT_MS`
    pub batch_timeout_millis: u64,
}

impl Default for AiTimeouts {
    fn default() -> Self {
        Self {
            decision_timeout_secs: 30,
            request_timeout_secs: 30,
            batch_timeout_millis: 10,
        }
    }
}

impl FromEnvironment for AiTimeouts {
    fn from_env() -> Self {
        Self {
            decision_timeout_secs: read_env_timeout_secs(
                "BEARDOG_DECISION_TIMEOUT_SECS",
                Self::default().decision_timeout_secs,
            ),
            request_timeout_secs: read_env_timeout_secs(
                "BEARDOG_AI_REQUEST_TIMEOUT_SECS",
                Self::default().request_timeout_secs,
            ),
            batch_timeout_millis: read_env_timeout_millis(
                "BEARDOG_AI_BATCH_TIMEOUT_MS",
                Self::default().batch_timeout_millis,
            ),
        }
    }

    fn try_from_env() -> Option<Self> {
        if std::env::var("BEARDOG_DECISION_TIMEOUT_SECS").is_ok()
            || std::env::var("BEARDOG_AI_REQUEST_TIMEOUT_SECS").is_ok()
            || std::env::var("BEARDOG_AI_BATCH_TIMEOUT_MS").is_ok()
        {
            Some(Self::from_env())
        } else {
            None
        }
    }
}

impl AiTimeouts {
    /// Get AI decision timeout as Duration
    pub fn decision_timeout(&self) -> Duration {
        Duration::from_secs(self.decision_timeout_secs)
    }

    /// Get AI request timeout as Duration
    pub fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.request_timeout_secs)
    }

    /// Get AI batch timeout as Duration
    pub fn batch_timeout(&self) -> Duration {
        Duration::from_millis(self.batch_timeout_millis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let timeouts = AiTimeouts::default();
        assert_eq!(timeouts.decision_timeout_secs, 30);
        assert_eq!(timeouts.request_timeout_secs, 30);
        assert_eq!(timeouts.batch_timeout_millis, 10);
    }

    #[test]
    fn test_timeouts() {
        let timeouts = AiTimeouts {
            decision_timeout_secs: 60,
            request_timeout_secs: 45,
            batch_timeout_millis: 20,
        };
        assert_eq!(timeouts.decision_timeout(), Duration::from_secs(60));
        assert_eq!(timeouts.request_timeout(), Duration::from_secs(45));
        assert_eq!(timeouts.batch_timeout(), Duration::from_millis(20));
    }
}
