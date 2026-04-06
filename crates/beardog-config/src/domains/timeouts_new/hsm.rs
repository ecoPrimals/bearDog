// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM (Hardware Security Module) timeout configuration
//!
//! Provides timeout settings for HSM operations.

use super::core::{read_env_timeout_millis, read_env_timeout_secs, FromEnvironment};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// HSM operation timeout configuration
///
/// Provides timeouts for hardware security module operations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct HsmTimeouts {
    /// HSM operation timeout in seconds
    ///
    /// Default: 2 seconds (typical HSM operation latency)
    /// Environment: `BEARDOG_HSM_OPERATION_TIMEOUT_SECS`
    pub operation_timeout_secs: u64,

    /// HSM probe timeout in milliseconds
    ///
    /// Default: 500 milliseconds (fast probe for availability check)
    /// Environment: `BEARDOG_HSM_PROBE_TIMEOUT_MILLIS`
    pub probe_timeout_millis: u64,
}

impl Default for HsmTimeouts {
    fn default() -> Self {
        Self {
            operation_timeout_secs: 2,
            probe_timeout_millis: 500,
        }
    }
}

impl FromEnvironment for HsmTimeouts {
    fn from_env() -> Self {
        Self {
            operation_timeout_secs: read_env_timeout_secs(
                "BEARDOG_HSM_OPERATION_TIMEOUT_SECS",
                Self::default().operation_timeout_secs,
            ),
            probe_timeout_millis: read_env_timeout_millis(
                "BEARDOG_HSM_PROBE_TIMEOUT_MILLIS",
                Self::default().probe_timeout_millis,
            ),
        }
    }

    fn try_from_env() -> Option<Self> {
        if std::env::var("BEARDOG_HSM_OPERATION_TIMEOUT_SECS").is_ok()
            || std::env::var("BEARDOG_HSM_PROBE_TIMEOUT_MILLIS").is_ok()
        {
            Some(Self::from_env())
        } else {
            None
        }
    }
}

impl HsmTimeouts {
    /// Get HSM operation timeout as Duration
    pub fn operation_timeout(&self) -> Duration {
        Duration::from_secs(self.operation_timeout_secs)
    }

    /// Get HSM probe timeout as Duration
    pub fn probe_timeout(&self) -> Duration {
        Duration::from_millis(self.probe_timeout_millis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let timeouts = HsmTimeouts::default();
        assert_eq!(timeouts.operation_timeout_secs, 2);
        assert_eq!(timeouts.probe_timeout_millis, 500);
    }

    #[test]
    fn test_operation_timeout() {
        let timeouts = HsmTimeouts {
            operation_timeout_secs: 5,
            probe_timeout_millis: 1000,
        };
        assert_eq!(timeouts.operation_timeout(), Duration::from_secs(5));
        assert_eq!(timeouts.probe_timeout(), Duration::from_millis(1000));
    }
}
