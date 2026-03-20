// SPDX-License-Identifier: AGPL-3.0-only

//! HSM manager types (DTOs)
//!
//! Serializable identifiers for health and failover components. Runtime monitoring and
//! failover logic are implemented in `beardog-tunnel` (and related crates), not in these structs.

use serde::{Deserialize, Serialize};

/// Default HSM health monitor (identity / serialization handle)
///
/// Used where a stable, serializable monitor id is required; live health state is tracked elsewhere.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultHsmHealthMonitor {
    /// Unique identifier for this monitor
    pub id: String,
}

impl DefaultHsmHealthMonitor {
    /// Creates a new health monitor with the given ID
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    /// Creates a health monitor with a default ID
    #[must_use]
    pub fn with_default_id() -> Self {
        Self::new("default")
    }
}

impl Default for DefaultHsmHealthMonitor {
    fn default() -> Self {
        Self::with_default_id()
    }
}

/// Default HSM failover manager (identity / serialization handle)
///
/// Used where a stable, serializable manager id is required; failover policy runs in the tunnel layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultHsmFailoverManager {
    /// Unique identifier for this failover manager
    pub id: String,
}

impl DefaultHsmFailoverManager {
    /// Creates a new failover manager with the given ID
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    /// Creates a failover manager with a default ID
    #[must_use]
    pub fn with_default_id() -> Self {
        Self::new("default")
    }
}

impl Default for DefaultHsmFailoverManager {
    fn default() -> Self {
        Self::with_default_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_monitor_creation() {
        let monitor = DefaultHsmHealthMonitor::new("monitor-1");
        assert_eq!(monitor.id, "monitor-1");
    }

    #[test]
    fn test_health_monitor_default() {
        let monitor = DefaultHsmHealthMonitor::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(monitor.id, "default");
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_failover_manager_creation() {
        let manager = DefaultHsmFailoverManager::new("failover-1");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: important
        assert_eq!(manager.id, "failover-1");
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: important
    #[test]
    fn test_failover_manager_default() {
        let manager = DefaultHsmFailoverManager::default();
        assert_eq!(manager.id, "default");
    }
}
