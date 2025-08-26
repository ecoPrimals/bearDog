

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub mod external_services;
pub mod load_balancing;
pub mod security;

pub use external_services::{
    ExternalServiceConfig, ExternalServicesConfig, ServiceHealthConfig, ServiceMetadata,
    ServiceRegistrationConfig,
};
pub use load_balancing::{LoadBalancingAlgorithm, LoadBalancingConfig};

pub use crate::config::network::security::NetworkSecurityConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationMeshConfig {

    pub discovery_endpoint: String,

    pub auth_config: MeshAuthConfig,

    pub connection_timeout: Duration,

    pub request_timeout: Duration,

    pub retry_config: RetryConfig,

    pub load_balancing: LoadBalancingStrategy,

    pub health_check: crate::canonical::network::HealthCheckConfig,

    pub custom_properties: HashMap<String, serde_json::Value>,
}
impl Default for CommunicationMeshConfig {}

    fn default() -> Self {
        Self {
            discovery_endpoint: "http://localhost:8080/discovery".to_string(),
            auth_config: MeshAuthConfig::default(),
            connection_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(30),
            retry_config: RetryConfig::default(),
            load_balancing: LoadBalancingStrategy::RoundRobin,
            health_check: crate::canonical::network::HealthCheckConfig::default(),
            custom_properties: HashMap::with_capacity(16),
        }
    }

pub struct MeshAuthConfig {

    pub auth_type: MeshAuthType,

    pub api_key: Option<String>,

    pub cert_path: Option<String>,

    pub key_path: Option<String>,

    pub ca_cert_path: Option<String>,

    pub custom_headers: HashMap<String, String>,}

impl Default for MeshAuthConfig {
            auth_type: MeshAuthType::None,
            api_key: None,
            cert_path: None,
            key_path: None,
            ca_cert_path: None,
            custom_headers: HashMap::with_capacity(16),

pub enum MeshAuthType {

    None,

    ApiKey,

    MutualTLS,

    Jwt,

    Custom(String),

pub struct RetryConfig {

    pub max_retries: u32,

    pub base_delay: Duration,

    pub max_delay: Duration,

    pub backoff_multiplier: f64,

    pub jitter: bool,}

impl Default for RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            jitter: true,

pub enum LoadBalancingStrategy {

    RoundRobin,

    WeightedRoundRobin,

    LeastConnections,

    Random,

    HealthBased,

pub struct EndpointConfig {

    pub url: String,

    pub weight: u32,

    pub priority: u32,

    pub enabled: bool,

    pub metadata: HashMap<String, String>,

pub struct ServiceDiscoveryConfig {

    pub provider: DiscoveryProvider,

    pub endpoint: String,

    pub refresh_interval: Duration,

    pub cache_ttl: Duration,

    pub auth: Option<MeshAuthConfig>,

pub enum DiscoveryProvider {

    Static,

    Consul,

    Kubernetes,

