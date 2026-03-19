// SPDX-License-Identifier: AGPL-3.0-only

//! Database and connection pool timeout configuration
//!
//! Provides timeout settings for database connections and pool management.

use super::core::{read_env_timeout_secs, FromEnvironment};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Database and pool timeout configuration
///
/// Provides timeouts for connection pools and database operations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct DatabaseTimeouts {
    /// Pool idle timeout in seconds
    ///
    /// Default: 300 seconds (5 minutes)
    /// Environment: `BEARDOG_POOL_IDLE_TIMEOUT_SECS`
    pub pool_idle_timeout_secs: u64,

    /// Maximum connection age in seconds
    ///
    /// Default: 3600 seconds (1 hour)
    /// Environment: `BEARDOG_MAX_CONNECTION_AGE_SECS`
    pub max_connection_age_secs: u64,

    /// Service discovery operation timeout in seconds
    ///
    /// Default: 10 seconds (allows for network latency)
    /// Environment: `BEARDOG_DISCOVERY_TIMEOUT_SECS`
    pub discovery_timeout_secs: u64,
}

impl Default for DatabaseTimeouts {
    fn default() -> Self {
        Self {
            pool_idle_timeout_secs: 300,   // 5 minutes
            max_connection_age_secs: 3600, // 1 hour
            discovery_timeout_secs: 10,
        }
    }
}

impl FromEnvironment for DatabaseTimeouts {
    fn from_env() -> Self {
        Self {
            pool_idle_timeout_secs: read_env_timeout_secs(
                "BEARDOG_POOL_IDLE_TIMEOUT_SECS",
                Self::default().pool_idle_timeout_secs,
            ),
            max_connection_age_secs: read_env_timeout_secs(
                "BEARDOG_MAX_CONNECTION_AGE_SECS",
                Self::default().max_connection_age_secs,
            ),
            discovery_timeout_secs: read_env_timeout_secs(
                "BEARDOG_DISCOVERY_TIMEOUT_SECS",
                Self::default().discovery_timeout_secs,
            ),
        }
    }

    fn try_from_env() -> Option<Self> {
        if std::env::var("BEARDOG_POOL_IDLE_TIMEOUT_SECS").is_ok()
            || std::env::var("BEARDOG_MAX_CONNECTION_AGE_SECS").is_ok()
            || std::env::var("BEARDOG_DISCOVERY_TIMEOUT_SECS").is_ok()
        {
            Some(Self::from_env())
        } else {
            None
        }
    }
}

impl DatabaseTimeouts {
    /// Get pool idle timeout as Duration
    pub fn pool_idle_timeout(&self) -> Duration {
        Duration::from_secs(self.pool_idle_timeout_secs)
    }

    /// Get max connection age as Duration
    pub fn max_connection_age(&self) -> Duration {
        Duration::from_secs(self.max_connection_age_secs)
    }

    /// Get discovery timeout as Duration
    pub fn discovery_timeout(&self) -> Duration {
        Duration::from_secs(self.discovery_timeout_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let timeouts = DatabaseTimeouts::default();
        assert_eq!(timeouts.pool_idle_timeout_secs, 300);
        assert_eq!(timeouts.max_connection_age_secs, 3600);
        assert_eq!(timeouts.discovery_timeout_secs, 10);
    }

    #[test]
    fn test_timeouts() {
        let timeouts = DatabaseTimeouts {
            pool_idle_timeout_secs: 600,
            max_connection_age_secs: 7200,
            discovery_timeout_secs: 20,
        };
        assert_eq!(timeouts.pool_idle_timeout(), Duration::from_secs(600));
        assert_eq!(timeouts.max_connection_age(), Duration::from_secs(7200));
        assert_eq!(timeouts.discovery_timeout(), Duration::from_secs(20));
    }
}
