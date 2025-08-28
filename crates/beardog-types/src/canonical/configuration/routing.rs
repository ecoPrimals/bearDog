// Removed unused imports: DiscoveryConfig, SecurityConfig
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Consolidated router configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RouterConfig {
    pub strategy: RoutingStrategy,
    pub health_check_interval: Duration,
    pub timeout: Duration,
}

// Re-export canonical CircuitBreakerConfig
pub use crate::canonical::providers::CircuitBreakerConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RoutingStrategy {
    #[default]
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    HealthBased,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancingConfig {
    pub strategy: LoadBalancingStrategy,
    pub health_check_interval: Duration,
    pub max_failures: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub interval: Duration,
    pub timeout: Duration,
    pub healthy_threshold: u32,
    pub unhealthy_threshold: u32,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimeoutConfig {
    pub connect_timeout: Duration,
    pub request_timeout: Duration,
    pub idle_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub backoff_multiplier: f64,
    pub retry_conditions: Vec<String>,
}

/// Consolidated model configuration for ML/AI routing
/// Model configuration for routing decisions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelConfig {
    pub model_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub update_interval: Duration,
}

/// Consolidated capability configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CapabilityConfig {
    pub capability_name: String,
    pub version: String,
    pub endpoints: Vec<CapabilityEndpoint>,
    pub authentication: OAuth2Config,
    pub rate_limits: RateLimitConfig,
    pub feature_flags: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CapabilityEndpoint {
    pub name: String,
    pub url: String,
    pub methods: Vec<String>,
    pub timeout: Duration,
    pub retry_policy: RetryPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
    pub window_size: Duration,
}

/// Consolidated OAuth2 configuration
/// OAuth2 configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LoadBalancingStrategy {
    #[default]
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    IpHash,
}
