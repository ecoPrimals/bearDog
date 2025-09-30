//! # Discovery Configuration
//!
//! Configuration types and defaults for the capability discovery system.

/// Discovery configuration
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Discovery timeout in milliseconds
    pub timeout_ms: u64,
    /// Maximum concurrent discovery operations
    pub max_concurrent: usize,
    /// Cache duration in milliseconds
    pub cache_duration_ms: u64,
    /// Health check interval in milliseconds
    pub health_check_interval_ms: u64,
    /// Discovery endpoints
    pub discovery_endpoints: Vec<String>,
    /// Enable automatic capability registration
    pub auto_register: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            timeout_ms: beardog_types::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT.as_millis() as u64,
            max_concurrent: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
            cache_duration_ms: 300_000, // 5 minutes
            health_check_interval_ms: 60_000, // 1 minute
            discovery_endpoints: vec![
                std::env::var("BEARDOG_DISCOVERY_ENDPOINT")
                    .unwrap_or_else(|_| format!("https://discovery.ecosystem.internal:{}", 
                        beardog_types::constants::domains::network::ports::DEFAULT_API_PORT)),
                std::env::var("BEARDOG_CAPABILITY_REGISTRY")
                    .unwrap_or_else(|_| format!("https://capabilities.ecosystem.internal:{}", 
                        beardog_types::constants::domains::network::ports::DEFAULT_HTTPS_PORT)),
            ],
            auto_register: true,
        }
    }
} 