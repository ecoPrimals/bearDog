use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Consolidated BiomeOS adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BiomeOSConfig {
    pub auth: BiomeOSAuthConfig,
    pub connection: BiomeOSConnectionConfig,
    pub timeout: Duration,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BiomeOSAuthConfig {
    pub auth_type: String,
    pub credentials: HashMap<String, String>,
    pub token_refresh_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BiomeOSConnectionConfig {
    pub endpoint: String,
    pub port: u16,
    pub use_tls: bool,
    pub connection_pool_size: usize,
}

/// Consolidated SongBird service configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SongBirdConfig {
    pub service_discovery_endpoint: String,
    pub handoff: HandoffConfig,
    pub timeout: Duration,
    pub health_check_interval: Duration,
}

/// Consolidated SongBird handoff configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HandoffConfig {
    pub endpoint: String,
    pub timeout: Duration,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_multiplier: f64,
    pub initial_delay: Duration,
}

/// Consolidated Kubernetes adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesConfig {
    pub kubeconfig_path: String,
    pub default_namespace: String,
    pub timeout_seconds: u64,
}

impl Default for KubernetesConfig {
    fn default() -> Self {
        Self {
            kubeconfig_path: std::env::var("KUBECONFIG")
                .unwrap_or_else(|_| "~/.kube/config".to_string()),
            default_namespace: "default".to_string(),
            timeout_seconds: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TlsConfig {
    pub ca_cert_path: String,
    pub client_cert_path: String,
    pub client_key_path: String,
    pub verify_ssl: bool,
}

/// Consolidated vendor HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VendorHsmConfig {
    pub vendor_type: String,
    pub connection_string: String,
    pub auth_config: HashMap<String, String>,
    pub performance_config: VendorPerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VendorPerformanceConfig {
    pub max_concurrent_operations: usize,
    pub operation_timeout: Duration,
    pub connection_pool_size: usize,
}

/// Consolidated BearDog ecosystem configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BearDogEcosystemConfig {
    pub primal_id: String,
    pub discovery_endpoints: Vec<String>,
    pub capabilities: Vec<String>,
    pub security_level: SecurityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SecurityLevel {
    #[default]
    Standard,
    Enhanced,
    Maximum,
}

// Re-export the canonical UniversalAdapterConfig from providers module
pub use crate::canonical::providers::UniversalAdapterConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionConfig {
    pub endpoint: String,
    pub timeout: Duration,
    pub max_retries: u32,
    pub use_compression: bool,
}

/// Bridge configuration for security provider integration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BridgeConfig {
    pub enabled: bool,
    pub bridge_endpoint: String,
    pub timeout: Duration,
    pub retry_attempts: u32,
    pub max_sessions: usize,
    pub session_timeout_seconds: u64,
    pub enable_metrics: bool,
    pub vendor_integrations_enabled: bool,
}

/// Evolution configuration for adaptive systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionConfig {
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub population_size: usize,
    pub generation_limit: u32,
    pub fitness_threshold: f64,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            mutation_rate: 0.01,
            crossover_rate: 0.8,
            population_size: 100,
            generation_limit: 1000,
            fitness_threshold: 0.95,
        }
    }
}

/// Consolidated ToadStool client configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToadStoolClientConfig {
    pub compute_endpoint: String,
    pub auth_token: String,
    pub request_timeout: Duration,
    pub max_concurrent_requests: usize,
}
