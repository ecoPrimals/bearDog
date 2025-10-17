//! HSM Manager Types
//!
//! Provides placeholder types for HSM health monitoring and failover management.
//! These are simple marker types that will be replaced with full implementations.

use serde::{Deserialize, Serialize};

/// Default HSM health monitor
///
/// A simple health monitoring implementation for HSM systems.
/// This is a placeholder that will be expanded with full monitoring capabilities.
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

/// Default HSM failover manager
///
/// A simple failover management implementation for HSM systems.
/// This is a placeholder that will be expanded with full failover capabilities.
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
        assert_eq!(monitor.id, "default");
    }

    #[test]
    fn test_failover_manager_creation() {
        let manager = DefaultHsmFailoverManager::new("failover-1");
        assert_eq!(manager.id, "failover-1");
    }

    #[test]
    fn test_failover_manager_default() {
        let manager = DefaultHsmFailoverManager::default();
        assert_eq!(manager.id, "default");
    }
}
