use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Consolidated router configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RouterConfig {
    pub routing_strategy: RoutingStrategy,
    pub load_balancing: LoadBalancingConfig,
    pub health_checks: HealthCheckConfig,
    pub timeout_config: TimeoutConfig,
    pub retry_policy: RetryPolicy,
}

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
    pub algorithm: String,
    pub weights: HashMap<String, f64>,
    pub sticky_sessions: bool,
    pub session_affinity_timeout: Duration,
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

/// Consolidated circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    pub success_threshold: u32,
    pub request_volume_threshold: u32,
    pub error_percentage_threshold: f64,
    pub metrics_window: Duration,
}

/// Consolidated model configuration for ML/AI routing
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelConfig {
    pub model_type: String,
    pub model_path: String,
    pub inference_timeout: Duration,
    pub batch_size: usize,
    pub gpu_enabled: bool,
    pub model_parameters: HashMap<String, serde_json::Value>,
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub scopes: Vec<String>,
    pub redirect_uri: String,
    pub token_expiry: Duration,
    pub refresh_token_enabled: bool,
} 