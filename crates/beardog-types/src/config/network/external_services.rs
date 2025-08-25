// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # External Services Configuration
///
/// **SERVICE DISCOVERY AND EXTERNAL INTEGRATION CONFIGS**
/// Contains external service configuration structs including service discovery,
/// registration, and third-party integration settings.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **CANONICAL EXTERNAL SERVICES CONFIGURATION**
/// Consolidates external service configurations from multiple locations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalServicesConfig {
    /// Service discovery configuration
    pub service_discovery: Option<ServiceDiscoveryConfig>,
    /// Service registration configuration
    pub service_registration: Option<ServiceRegistrationConfig>,
    /// Health check configuration for external services
    pub health_check: Option<ServiceHealthConfig>,
    /// Load balancing for external services
    pub load_balancing: Option<ExternalLoadBalancingConfig>,
    /// Circuit breaker for external services
    pub circuit_breaker: Option<ExternalCircuitBreakerConfig>,
    /// Retry configuration for external services
    pub retry: Option<ExternalRetryConfig>,
    /// External service metadata
    pub services: HashMap<String, ExternalServiceConfig>,
    // **ECOSYSTEM INTEGRATION ENDPOINTS** - Specific primal endpoints
    /// Songbird service endpoint
    pub songbird_endpoint: Option<String>,
    /// NestGate service endpoint  
    pub storage_endpoint: Option<String>,
    /// Squirrel service endpoint
    pub squirrel_endpoint: Option<String>,
    /// ToadStool service endpoint
    pub toadstool_endpoint: Option<String>,
}
/// Service discovery configuration
pub struct ServiceDiscoveryConfig {
    /// Service discovery enabled
    pub enabled: bool,
    /// Discovery method
    pub method: ServiceDiscoveryMethod,
    /// Discovery interval
    pub interval: Duration,
    /// Discovery timeout
    pub timeout: Duration,
    /// Service registry endpoints
    pub registry_endpoints: Vec<String>,
    /// Discovery metadata
    pub metadata: HashMap<String, String>,
/// Service registration configuration
pub struct ServiceRegistrationConfig {
    /// Service registration enabled
    /// Service name
    pub service_name: String,
    /// Service ID
    pub service_id: String,
    /// Service address
    pub address: String,
    /// Service port
    pub port: u16,
    /// Service tags
    pub tags: Vec<String>,
    /// Registration metadata
    pub metadata: ServiceMetadata,
    /// Health check URL for registration
    pub health_check_url: Option<String>,
    /// Registration TTL
    pub ttl: Option<Duration>,
/// Service health check configuration
pub struct ServiceHealthConfig {
    /// Health check enabled
    /// Health check interval
    /// Health check timeout
    /// Health check URL
    pub url: String,
    /// Expected status codes
    pub expected_status_codes: Vec<u16>,
    /// Health check method
    pub method: String,
    /// Custom health check headers
    pub headers: HashMap<String, String>,
    /// Unhealthy threshold
    pub unhealthy_threshold: u32,
    /// Healthy threshold
    pub healthy_threshold: u32,
/// Service metadata
pub struct ServiceMetadata {
    /// Service version
    pub version: Option<String>,
    /// Service environment
    pub environment: Option<String>,
    /// Service region
    pub region: Option<String>,
    /// Service zone
    pub zone: Option<String>,
    /// Custom metadata
    pub custom: HashMap<String, String>,
/// Individual external service configuration
pub struct ExternalServiceConfig {
    pub name: String,
    /// Service endpoints
    pub endpoints: Vec<String>,
    /// Service authentication
    pub auth: Option<ExternalServiceAuth>,
    /// Service timeout configuration
    /// Service retry configuration
    /// Service-specific headers
    /// Service metadata
/// External service authentication
pub struct ExternalServiceAuth {
    /// Authentication type
    pub auth_type: ExternalAuthType,
    /// API key (for API key auth)
    pub api_key: Option<String>,
    /// Bearer token (for bearer auth)
    pub bearer_token: Option<String>,
    /// Username (for basic auth)
    pub username: Option<String>,
    /// Password (for basic auth)
    pub password: Option<String>,
    /// Custom auth headers
    pub custom_headers: HashMap<String, String>,
/// Service discovery methods
pub enum ServiceDiscoveryMethod {
    Consul,
    Etcd,
    Kubernetes,
    Zookeeper,
    Dns,
    Static,
/// External authentication types}


pub enum ExternalAuthType {
    None,
    ApiKey,
    Bearer,
    Basic,
    OAuth2,
    Custom,
/// External service load balancing configuration
pub struct ExternalLoadBalancingConfig {
    /// Load balancing enabled
    /// Load balancing algorithm
    pub algorithm: ExternalLoadBalancingAlgorithm,
    /// Sticky sessions enabled
    pub sticky_sessions: bool,
    /// Health check integration
    pub health_check_integration: bool,
/// External load balancing algorithms
pub enum ExternalLoadBalancingAlgorithm {
    RoundRobin,
    LeastConnections,
    Random,
    WeightedRoundRobin,
    IpHash,
/// External service circuit breaker configuration}


pub struct ExternalCircuitBreakerConfig {
    /// Circuit breaker enabled
    /// Failure threshold
    pub failure_threshold: u32,
    /// Timeout threshold
    pub timeout_threshold: Duration,
    /// Reset timeout
    pub reset_timeout: Duration,
    /// Half-open max calls
    pub half_open_max_calls: u32,
/// External service retry configuration
pub struct ExternalRetryConfig {
    /// Retry enabled
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Initial delay
    pub initial_delay: Duration,
    /// Maximum delay
    pub max_delay: Duration,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Retryable status codes
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
            services: HashMap::new(),
            // Ecosystem integration endpoints - disabled by default
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
            metadata: HashMap::new(),}


impl Default for ServiceMetadata {
            version: Some("1.0.0".to_string()),
            environment: Some("development".to_string()),
            region: None,
            zone: None,
            custom: HashMap::new(),
impl Default for ExternalRetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            retryable_status_codes: vec![500, 502, 503, 504],
