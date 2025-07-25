//! Discovery Configuration
//!
//! Configuration types and defaults for ecosystem discovery

/// Discovery configuration
#[derive(Debug, Clone)]
pub struct EcosystemDiscoveryConfig {
    /// Interval between discovery attempts in seconds
    pub discovery_interval_seconds: u64,
    /// Interval between health checks in seconds
    pub health_check_interval_seconds: u64,
    /// Timeout for service operations in seconds
    pub service_timeout_seconds: u64,
    /// Maximum number of discovery attempts before giving up
    pub max_discovery_attempts: u32,
    /// List of ecosystems to discover services for
    pub enabled_ecosystems: Vec<String>,
}

impl Default for EcosystemDiscoveryConfig {
    fn default() -> Self {
        Self {
            discovery_interval_seconds: 30,
            health_check_interval_seconds: 60,
            service_timeout_seconds: 10,
            max_discovery_attempts: 3,
            enabled_ecosystems: vec![
                "toadstool".to_string(),
                "songbird".to_string(),
                "nestgate".to_string(),
                "squirrel".to_string(),
                "biomeos".to_string(),
            ],
        }
    }
} 