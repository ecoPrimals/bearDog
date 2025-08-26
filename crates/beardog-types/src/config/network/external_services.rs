

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalServicesConfig {

    pub service_discovery: Option<ServiceDiscoveryConfig>,

    pub service_registration: Option<ServiceRegistrationConfig>,

    pub health_check: Option<ServiceHealthConfig>,

    pub load_balancing: Option<ExternalLoadBalancingConfig>,

    pub circuit_breaker: Option<ExternalCircuitBreakerConfig>,

    pub retry: Option<ExternalRetryConfig>,

    pub services: HashMap<String, ExternalServiceConfig>,

    pub songbird_endpoint: Option<String>,

    pub storage_endpoint: Option<String>,

    pub squirrel_endpoint: Option<String>,

    pub toadstool_endpoint: Option<String>,
}

pub struct ServiceDiscoveryConfig {

    pub enabled: bool,

    pub method: ServiceDiscoveryMethod,

    pub interval: Duration,

    pub timeout: Duration,

    pub registry_endpoints: Vec<String>,

    pub metadata: HashMap<String, String>,

pub struct ServiceRegistrationConfig {

    pub service_name: String,

    pub service_id: String,

    pub address: String,

    pub port: u16,

    pub tags: Vec<String>,

    pub metadata: ServiceMetadata,

    pub health_check_url: Option<String>,

    pub ttl: Option<Duration>,

pub struct ServiceHealthConfig {

    pub url: String,

    pub expected_status_codes: Vec<u16>,

    pub method: String,

    pub headers: HashMap<String, String>,

    pub unhealthy_threshold: u32,

    pub healthy_threshold: u32,

pub struct ServiceMetadata {

    pub version: Option<String>,

    pub environment: Option<String>,

    pub region: Option<String>,

    pub zone: Option<String>,

    pub custom: HashMap<String, String>,

pub struct ExternalServiceConfig {
    pub name: String,

    pub endpoints: Vec<String>,

    pub auth: Option<ExternalServiceAuth>,

pub struct ExternalServiceAuth {

    pub auth_type: ExternalAuthType,

    pub api_key: Option<String>,

    pub bearer_token: Option<String>,

    pub username: Option<String>,

    pub password: Option<String>,

    pub custom_headers: HashMap<String, String>,

pub enum ServiceDiscoveryMethod {
    Consul,
    Etcd,
    Kubernetes,
    Zookeeper,
    Dns,
    Static,

pub enum ExternalAuthType {
    None,
    ApiKey,
    Bearer,
    Basic,
    OAuth2,
    Custom,

pub struct ExternalLoadBalancingConfig {

    pub algorithm: ExternalLoadBalancingAlgorithm,

    pub sticky_sessions: bool,

    pub health_check_integration: bool,

pub enum ExternalLoadBalancingAlgorithm {
    RoundRobin,
    LeastConnections,
    Random,
    WeightedRoundRobin,
    IpHash,

pub struct ExternalCircuitBreakerConfig {

    pub failure_threshold: u32,

    pub timeout_threshold: Duration,

    pub reset_timeout: Duration,

    pub half_open_max_calls: u32,

pub struct ExternalRetryConfig {

    pub max_attempts: u32,

    pub initial_delay: Duration,

    pub max_delay: Duration,

    pub backoff_multiplier: f64,

    pub retryable_status_codes: Vec<u16>,}

impl Default for ExternalServicesConfig {}

    fn default() -> Self {
        Self {
            service_discovery: None,
            service_registration: None,
            health_check: None,
            load_balancing: None,
            circuit_breaker: None,
            retry: Some(ExternalRetryConfig::default()),
            services: HashMap::with_capacity(16),

            songbird_endpoint: None,
            storage_endpoint: None,
            squirrel_endpoint: None,
            toadstool_endpoint: None,
        }
    }
impl Default for ServiceDiscoveryConfig {
            enabled: true,
            method: ServiceDiscoveryMethod::Static,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            registry_endpoints: vec!["127.0.0.1:8500".to_string()],
            metadata: HashMap::with_capacity(16),}

impl Default for ServiceMetadata {
            version: Some("1.0.0".to_string()),
            environment: Some("development".to_string()),
            region: None,
            zone: None,
            custom: HashMap::with_capacity(16),
impl Default for ExternalRetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            retryable_status_codes: vec![500, 502, 503, 504],
