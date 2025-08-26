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
    pub handoff: SongBirdHandoffConfig,
    pub timeout: Duration,
    pub health_check_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SongBirdHandoffConfig {
    pub enabled: bool,
    pub handoff_timeout: Duration,
    pub retry_policy: RetryPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_multiplier: f64,
    pub initial_delay: Duration,
}

/// Consolidated Kubernetes adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KubernetesConfig {
    pub cluster_endpoint: String,
    pub namespace: String,
    pub service_account: String,
    pub tls_config: TlsConfig,
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

/// Consolidated universal adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UniversalAdapterConfig {
    pub adapter_type: String,
    pub target_system: String,
    pub connection_config: ConnectionConfig,
    pub feature_flags: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionConfig {
    pub endpoint: String,
    pub timeout: Duration,
    pub max_retries: u32,
    pub use_compression: bool,
}

/// Consolidated ToadStool client configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToadStoolClientConfig {
    pub compute_endpoint: String,
    pub auth_token: String,
    pub request_timeout: Duration,
    pub max_concurrent_requests: usize,
} 