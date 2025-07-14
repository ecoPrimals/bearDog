//! Configuration structures for the Capability Manager
//!
//! This module contains all configuration options for the comprehensive
//! capability management system, including monitoring intervals, timeouts,
//! and feature toggles.

use std::time::Duration;

/// Configuration for the Capability Manager
#[derive(Debug, Clone)]
pub struct CapabilityManagerConfig {
    /// Interval between monitoring cycles
    pub monitoring_interval: Duration,
    /// Timeout for health checks
    pub health_check_timeout: Duration,
    /// Number of performance snapshots to keep in history
    pub performance_history_size: usize,
    /// Whether to enable emergent capability discovery
    pub emergent_discovery_enabled: bool,
    /// Whether to enable genetic capability tracking
    pub genetic_tracking_enabled: bool,
    /// Whether to enable advanced matching algorithms
    pub advanced_matching_enabled: bool,
    /// Timeout for dependency resolution
    pub dependency_resolution_timeout: Duration,
    /// Whether to enable alert notifications
    pub alert_notification_enabled: bool,
}

impl Default for CapabilityManagerConfig {
    fn default() -> Self {
        Self {
            monitoring_interval: Duration::from_secs(30),
            health_check_timeout: Duration::from_secs(5),
            performance_history_size: 100,
            emergent_discovery_enabled: true,
            genetic_tracking_enabled: true,
            advanced_matching_enabled: true,
            dependency_resolution_timeout: Duration::from_secs(10),
            alert_notification_enabled: true,
        }
    }
} 