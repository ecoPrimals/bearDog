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


/// Network Configuration Types
///
/// Unified network configuration for all `BearDog` services
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub mod external_services;
pub mod load_balancing;
pub mod security;
// Re-export all network configuration types for backward compatibility
pub use external_services::{
    ExternalServiceConfig, ExternalServicesConfig, ServiceHealthConfig, ServiceMetadata,
    ServiceRegistrationConfig,
};
pub use load_balancing::{LoadBalancingAlgorithm, LoadBalancingConfig};
// Legacy types removed - use CommunicationMeshConfig and related unified types directly
// Use canonical NetworkSecurityConfig from network/security module
pub use crate::config::network::security::NetworkSecurityConfig;
/// Communication mesh configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationMeshConfig {
    /// Mesh service discovery endpoint
    pub discovery_endpoint: String,
    /// Authentication configuration
    pub auth_config: MeshAuthConfig,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
    /// Health check configuration
    pub health_check: crate::canonical::network::HealthCheckConfig,
    /// Custom mesh properties
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
            custom_properties: HashMap::new(),
        }
    }
/// Mesh authentication configuration
pub struct MeshAuthConfig {
    /// Authentication type
    pub auth_type: MeshAuthType,
    /// API key for authentication
    pub api_key: Option<String>,
    /// Certificate path for mTLS
    pub cert_path: Option<String>,
    /// Private key path for mTLS
    pub key_path: Option<String>,
    /// CA certificate path
    pub ca_cert_path: Option<String>,
    /// Custom authentication headers
    pub custom_headers: HashMap<String, String>,}


impl Default for MeshAuthConfig {
            auth_type: MeshAuthType::None,
            api_key: None,
            cert_path: None,
            key_path: None,
            ca_cert_path: None,
            custom_headers: HashMap::new(),
/// Mesh authentication types}


pub enum MeshAuthType {
    /// No authentication
    None,
    /// API key authentication
    ApiKey,
    /// Mutual TLS authentication
    MutualTLS,
    /// JWT token authentication
    Jwt,
    /// Custom authentication
    Custom(String),
/// Retry configuration for mesh operations}


pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Base delay between retries
    pub base_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Jitter to add to delays
    pub jitter: bool,}


impl Default for RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            jitter: true,
/// Load balancing strategies}


pub enum LoadBalancingStrategy {
    /// Round-robin load balancing
    RoundRobin,
    /// Weighted round-robin
    WeightedRoundRobin,
    /// Least connections
    LeastConnections,
    /// Random selection
    Random,
    /// Health-based selection
    HealthBased,
    /// Custom strategy
/// Network endpoint configuration}


pub struct EndpointConfig {
    /// Endpoint URL
    pub url: String,
    /// Endpoint weight for load balancing
    pub weight: u32,
    /// Endpoint priority
    pub priority: u32,
    /// Whether endpoint is enabled
    pub enabled: bool,
    /// Custom endpoint metadata
    pub metadata: HashMap<String, String>,
/// Service discovery configuration
pub struct ServiceDiscoveryConfig {
    /// Discovery provider type
    pub provider: DiscoveryProvider,
    /// Discovery endpoint
    pub endpoint: String,
    /// Refresh interval for service list
    pub refresh_interval: Duration,
    /// Cache TTL for discovered services
    pub cache_ttl: Duration,
    /// Discovery authentication
    pub auth: Option<MeshAuthConfig>,
/// Service discovery providers
pub enum DiscoveryProvider {
    /// Static configuration
    Static,
    /// Consul service discovery
    Consul,
    /// Kubernetes service discovery
    Kubernetes,
    /// Custom discovery provider
