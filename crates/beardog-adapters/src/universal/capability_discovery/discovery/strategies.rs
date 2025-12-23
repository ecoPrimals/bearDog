//! # Discovery Strategies
//!
//! Different strategies for discovering capabilities from various sources.

use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::{
    CapabilityType, UniversalCapability, ProviderInfo, ProviderType, EndpointConfig, 
    AuthConfig, AuthType, HealthStatus, PerformanceMetrics, SecurityLevel, 
    CircuitBreakerConfig,
};
use std::collections::HashMap;
use tracing::{debug, warn};

use super::core::DiscoveryStrategy;

/// Environment-based discovery strategy
#[derive(Debug)]
pub struct EnvironmentDiscoveryStrategy {
    endpoints: Vec<String>,
}

impl EnvironmentDiscoveryStrategy {
    pub fn new() -> Self {
        Self {
            endpoints: vec![
                std::env::var("BEARDOG_DISCOVERY_ENDPOINT")
                    .unwrap_or_else(|_| format!("https://discovery.ecosystem.internal:{}", 
                        beardog_types::constants::domains::network::ports::DEFAULT_API_PORT)),
                std::env::var("FALLBACK_DISCOVERY_ENDPOINT")
                    .unwrap_or_else(|_| format!("http://discovery.ecosystem.internal:{}", 
                        beardog_types::constants::domains::network::ports::DEFAULT_API_PORT)),
            ],
        }
    }
    
    /// Create capability from environment configuration
    fn create_env_capability(&self, capability_type: &CapabilityType, _endpoint: &str) -> Option<UniversalCapability> {
        let env_key = match capability_type {
            CapabilityType::ServiceMesh => "SERVICE_MESH_ENDPOINT",
            CapabilityType::ComputeIntelligence => "ORCHESTRATION_ENDPOINT", 
            CapabilityType::KeyManagement => "KMS_ENDPOINT",
            CapabilityType::SecretsManagement => "SECRETS_ENDPOINT",
            _ => return None,
        };
        
        if let Ok(service_endpoint) = std::env::var(env_key) {
            Some(UniversalCapability {
                capability_type: capability_type.clone(),
                provider: ProviderInfo {
                    provider_id: format!("env_{}", capability_type.name().to_lowercase().replace(' ', "_")),
                    provider_name: format!("Environment {}", capability_type.name()),
                    provider_type: ProviderType::Custom,
                    version: "1.0.0".to_string(),
                    region: std::env::var("REGION").ok(),
                },
                endpoint: EndpointConfig {
                    base_url: service_endpoint,
                    api_version: Some("v1".to_string()),
                    timeout_ms: beardog_types::constants::domains::network::timeouts::DEFAULT_REQUEST_TIMEOUT.as_millis() as u64,
                    max_retries: 3,
                    circuit_breaker: CircuitBreakerConfig::default(),
                },
                auth_config: AuthConfig {
                    auth_type: AuthType::None,
                    api_key: std::env::var(&format!("{}_API_KEY", env_key)).ok(),
                    bearer_token: std::env::var(&format!("{}_TOKEN", env_key)).ok(),
                    cert_path: None,
                    custom_params: HashMap::new(),
                },
                health_status: HealthStatus::Unknown,
                performance: PerformanceMetrics::default(),
                security_level: SecurityLevel::Standard,
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }
}

impl DiscoveryStrategy for EnvironmentDiscoveryStrategy {
    async fn discover_capabilities(
        &self, 
        capability_types: &[CapabilityType]
    ) -> Result<Vec<UniversalCapability>, BearDogError> {
        let mut capabilities = Vec::new();
        
        for endpoint in &self.endpoints {
            debug!("Querying discovery endpoint: {}", endpoint);
            
            for capability_type in capability_types {
                if let Some(capability) = self.create_env_capability(capability_type, endpoint) {
                    capabilities.push(capability);
                }
            }
        }
        
        Ok(capabilities)
    }
    
    fn strategy_name(&self) -> &'static str {
        "EnvironmentDiscovery"
    }
    
    fn is_available(&self) -> bool {
        !self.endpoints.is_empty()
    }
}

/// Universal service mesh discovery strategy
#[derive(Debug)]
pub struct ServiceMeshDiscoveryStrategy;

impl ServiceMeshDiscoveryStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl DiscoveryStrategy for ServiceMeshDiscoveryStrategy {
    async fn discover_capabilities(
        &self, 
        capability_types: &[CapabilityType]
    ) -> Result<Vec<UniversalCapability>, BearDogError> {
        let mut capabilities = Vec::new();
        
        for capability_type in capability_types {
            if matches!(capability_type, CapabilityType::ServiceMesh | CapabilityType::Network) {
                if let Some(mesh_endpoint) = std::env::var("SERVICE_MESH_ENDPOINT").ok() {
                    capabilities.push(UniversalCapability {
                        capability_type: capability_type.clone(),
                        provider: ProviderInfo {
                            provider_id: "service_mesh".to_string(),
                            provider_name: "Service Mesh Provider".to_string(),
                            provider_type: ProviderType::Primal,
                            version: "1.0.0".to_string(),
                            region: None,
                        },
                        endpoint: EndpointConfig {
                            base_url: mesh_endpoint,
                            api_version: Some("v1".to_string()),
                            timeout_ms: beardog_types::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT.as_millis() as u64,
                            max_retries: 3,
                            circuit_breaker: CircuitBreakerConfig::default(),
                        },
                        auth_config: AuthConfig {
                            auth_type: AuthType::None,
                            api_key: None,
                            bearer_token: None,
                            cert_path: None,
                            custom_params: HashMap::new(),
                        },
                        health_status: HealthStatus::Unknown,
                        performance: PerformanceMetrics::default(),
                        security_level: SecurityLevel::High,
                        metadata: HashMap::new(),
                    });
                }
            }
        }
        
        Ok(capabilities)
    }
    
    fn strategy_name(&self) -> &'static str {
        "ServiceMeshDiscovery"
    }
    
    fn is_available(&self) -> bool {
        std::env::var("SERVICE_MESH_ENDPOINT").is_ok()
    }
}

/// Cloud vendor discovery strategy
#[derive(Debug)]
pub struct CloudVendorDiscoveryStrategy;

impl CloudVendorDiscoveryStrategy {
    pub fn new() -> Self {
        Self
    }
    
    /// Detect cloud environment without hardcoding provider names
    fn detect_cloud_environment(&self) -> bool {
        // Check for cloud environment variables
        std::env::var("CLOUD_PROVIDER").is_ok() ||
        std::env::var("AWS_REGION").is_ok() ||
        std::env::var("AZURE_RESOURCE_GROUP").is_ok() ||
        std::env::var("GCP_PROJECT").is_ok()
    }
    
    /// Discover KMS capability from available providers
    fn discover_kms_capability(&self) -> Option<UniversalCapability> {
        if let Ok(kms_endpoint) = std::env::var("KMS_ENDPOINT") {
            return Some(UniversalCapability {
                capability_type: CapabilityType::KeyManagement,
                provider: ProviderInfo {
                    provider_id: "cloud_kms".to_string(),
                    provider_name: "Cloud Key Management Service".to_string(),
                    provider_type: ProviderType::Vendor,
                    version: "1.0.0".to_string(),
                    region: std::env::var("REGION").ok(),
                },
                endpoint: EndpointConfig {
                    base_url: kms_endpoint,
                    api_version: Some("v1".to_string()),
                    timeout_ms: 10000,
                    max_retries: 3,
                    circuit_breaker: CircuitBreakerConfig::default(),
                },
                auth_config: AuthConfig {
                    auth_type: AuthType::ApiKey,
                    api_key: std::env::var("KMS_API_KEY").ok(),
                    bearer_token: None,
                    cert_path: None,
                    custom_params: HashMap::new(),
                },
                health_status: HealthStatus::Unknown,
                performance: PerformanceMetrics::default(),
                security_level: SecurityLevel::Critical,
                metadata: HashMap::new(),
            });
        }
        None
    }
    
    /// Discover secrets management capability
    fn discover_secrets_capability(&self) -> Option<UniversalCapability> {
        if let Ok(secrets_endpoint) = std::env::var("SECRETS_ENDPOINT") {
            return Some(UniversalCapability {
                capability_type: CapabilityType::SecretsManagement,
                provider: ProviderInfo {
                    provider_id: "cloud_secrets".to_string(),
                    provider_name: "Cloud Secrets Manager".to_string(),
                    provider_type: ProviderType::Vendor,
                    version: "1.0.0".to_string(),
                    region: std::env::var("REGION").ok(),
                },
                endpoint: EndpointConfig {
                    base_url: secrets_endpoint,
                    api_version: Some("v1".to_string()),
                    timeout_ms: 10000,
                    max_retries: 3,
                    circuit_breaker: CircuitBreakerConfig::default(),
                },
                auth_config: AuthConfig {
                    auth_type: AuthType::ApiKey,
                    api_key: std::env::var("SECRETS_API_KEY").ok(),
                    bearer_token: None,
                    cert_path: None,
                    custom_params: HashMap::new(),
                },
                health_status: HealthStatus::Unknown,
                performance: PerformanceMetrics::default(),
                security_level: SecurityLevel::Critical,
                metadata: HashMap::new(),
            });
        }
        None
    }
    
    /// Discover storage capability
    fn discover_storage_capability(&self) -> Option<UniversalCapability> {
        if let Ok(storage_endpoint) = std::env::var("STORAGE_ENDPOINT") {
            return Some(UniversalCapability {
                capability_type: CapabilityType::CloudStorage,
                provider: ProviderInfo {
                    provider_id: "cloud_storage".to_string(),
                    provider_name: "Cloud Storage Service".to_string(),
                    provider_type: ProviderType::Vendor,
                    version: "1.0.0".to_string(),
                    region: std::env::var("REGION").ok(),
                },
                endpoint: EndpointConfig {
                    base_url: storage_endpoint,
                    api_version: Some("v1".to_string()),
                    timeout_ms: 15000,
                    max_retries: 3,
                    circuit_breaker: CircuitBreakerConfig::default(),
                },
                auth_config: AuthConfig {
                    auth_type: AuthType::ApiKey,
                    api_key: std::env::var("STORAGE_API_KEY").ok(),
                    bearer_token: None,
                    cert_path: None,
                    custom_params: HashMap::new(),
                },
                health_status: HealthStatus::Unknown,
                performance: PerformanceMetrics::default(),
                security_level: SecurityLevel::High,
                metadata: HashMap::new(),
            });
        }
        None
    }
}

impl DiscoveryStrategy for CloudVendorDiscoveryStrategy {
    async fn discover_capabilities(
        &self, 
        capability_types: &[CapabilityType]
    ) -> Result<Vec<UniversalCapability>, BearDogError> {
        let mut capabilities = Vec::new();
        
        for capability_type in capability_types {
            match capability_type {
                CapabilityType::KeyManagement => {
                    if let Some(kms_capability) = self.discover_kms_capability() {
                        capabilities.push(kms_capability);
                    }
                }
                CapabilityType::SecretsManagement => {
                    if let Some(secrets_capability) = self.discover_secrets_capability() {
                        capabilities.push(secrets_capability);
                    }
                }
                CapabilityType::CloudStorage => {
                    if let Some(storage_capability) = self.discover_storage_capability() {
                        capabilities.push(storage_capability);
                    }
                }
                _ => {}
            }
        }
        
        Ok(capabilities)
    }
    
    fn strategy_name(&self) -> &'static str {
        "CloudVendorDiscovery"
    }
    
    fn is_available(&self) -> bool {
        self.detect_cloud_environment()
    }
}

/// Container orchestration discovery strategy
#[derive(Debug)]
pub struct ContainerDiscoveryStrategy;

impl ContainerDiscoveryStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl DiscoveryStrategy for ContainerDiscoveryStrategy {
    async fn discover_capabilities(
        &self, 
        capability_types: &[CapabilityType]
    ) -> Result<Vec<UniversalCapability>, BearDogError> {
        let mut capabilities = Vec::new();
        
        for capability_type in capability_types {
            if matches!(capability_type, CapabilityType::ContainerOrchestration) {
                if let Some(orchestrator_endpoint) = std::env::var("ORCHESTRATION_ENDPOINT").ok() {
                    capabilities.push(UniversalCapability {
                        capability_type: capability_type.clone(),
                        provider: ProviderInfo {
                            provider_id: "container_orchestrator".to_string(),
                            provider_name: "Container Orchestration Service".to_string(),
                            provider_type: ProviderType::Primal,
                            version: "1.0.0".to_string(),
                            region: None,
                        },
                        endpoint: EndpointConfig {
                            base_url: orchestrator_endpoint,
                            api_version: Some("v1".to_string()),
                            timeout_ms: beardog_types::constants::domains::network::timeouts::DEFAULT_REQUEST_TIMEOUT.as_millis() as u64,
                            max_retries: 3,
                            circuit_breaker: CircuitBreakerConfig::default(),
                        },
                        auth_config: AuthConfig {
                            auth_type: AuthType::None,
                            api_key: None,
                            bearer_token: None,
                            cert_path: None,
                            custom_params: HashMap::new(),
                        },
                        health_status: HealthStatus::Unknown,
                        performance: PerformanceMetrics::default(),
                        security_level: SecurityLevel::High,
                        metadata: HashMap::new(),
                    });
                }
            }
        }
        
        Ok(capabilities)
    }
    
    fn strategy_name(&self) -> &'static str {
        "ContainerDiscovery"
    }
    
    fn is_available(&self) -> bool {
        std::env::var("ORCHESTRATION_ENDPOINT").is_ok()
    }
}

/// Network discovery strategy
#[derive(Debug)]
pub struct NetworkDiscoveryStrategy;

impl NetworkDiscoveryStrategy {
    pub fn new() -> Self {
        Self
    }
    
    /// Discover network services without hardcoding
    fn discover_network_services(&self) -> Option<UniversalCapability> {
        if let Ok(network_endpoint) = std::env::var("NETWORK_SERVICE_ENDPOINT") {
            Some(UniversalCapability {
                capability_type: CapabilityType::Network,
                provider: ProviderInfo {
                    provider_id: "network_service".to_string(),
                    provider_name: "Network Service Provider".to_string(),
                    provider_type: ProviderType::Custom,
                    version: "1.0.0".to_string(),
                    region: None,
                },
                endpoint: EndpointConfig {
                    base_url: network_endpoint,
                    api_version: Some("v1".to_string()),
                    timeout_ms: beardog_types::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT.as_millis() as u64,
                    max_retries: 3,
                    circuit_breaker: CircuitBreakerConfig::default(),
                },
                auth_config: AuthConfig {
                    auth_type: AuthType::None,
                    api_key: None,
                    bearer_token: None,
                    cert_path: None,
                    custom_params: HashMap::new(),
                },
                health_status: HealthStatus::Unknown,
                performance: PerformanceMetrics::default(),
                security_level: SecurityLevel::Standard,
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }
}

impl DiscoveryStrategy for NetworkDiscoveryStrategy {
    async fn discover_capabilities(
        &self, 
        capability_types: &[CapabilityType]
    ) -> Result<Vec<UniversalCapability>, BearDogError> {
        let mut capabilities = Vec::new();
        
        for capability_type in capability_types {
            if matches!(capability_type, CapabilityType::Network | CapabilityType::ServiceMesh) {
                if let Some(network_capability) = self.discover_network_services() {
                    capabilities.push(network_capability);
                }
            }
        }
        
        Ok(capabilities)
    }
    
    fn strategy_name(&self) -> &'static str {
        "NetworkDiscovery"
    }
    
    fn is_available(&self) -> bool {
        true // Always available for network discovery
    }
} 