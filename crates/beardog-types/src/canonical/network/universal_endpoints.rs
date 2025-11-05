// Universal Endpoint Configuration
//
// This module provides universal endpoint patterns that replace hardcoded localhost
// and IP addresses with discoverable, environment-based configurations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

/// Universal endpoint configuration (replaces hardcoded localhost patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalEndpointConfig {
    /// Primary discovery endpoint
    /// The discovery endpoint value
    pub discovery_endpoint: String,
    
    /// Service-specific endpoints discovered dynamically
    /// Mapping of service endpoints
    pub service_endpoints: HashMap<String, String>,
    
    /// The development fallbacks value
    pub development_fallbacks: DevelopmentFallbacks,
    
    /// Production-ready internal DNS patterns
    /// The internal dns patterns value
    pub internal_dns_patterns: InternalDnsPatterns,
}

/// Development fallback configuration (replaces localhost hardcoding)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentFallbacks {
    /// The base discovery value
    pub base_discovery: String,
    
    /// Mapping of service ports
    pub service_ports: HashMap<String, u16>,
    
    /// Whether bind_to_all_interfaces is enabled
    pub bind_to_all_interfaces: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalDnsPatterns {
    /// Base internal domain (e.g., "ecosystem.internal")
    /// The base domain value
    pub base_domain: String,
    
    /// Service subdomain patterns
    /// Mapping of service patterns
    pub service_patterns: HashMap<String, String>,
    
    /// Mapping of default ports
    pub default_ports: HashMap<String, u16>,
}

impl Default for UniversalEndpointConfig {
    fn default() -> Self {
        let network_config = super::super::config::network::NetworkConfig::default();
        
        Self {
            discovery_endpoint: env::var("BEARDOG_DISCOVERY_ENDPOINT")
                .or_else(|_| env::var("UNIVERSAL_DISCOVERY_ENDPOINT"))
                .unwrap_or_else(|_| {
                    let discovery_host = env::var("DISCOVERY_HOST")
                        .unwrap_or_else(|_| "discovery.ecosystem.internal".to_string());
                    format!("http://{}:{}", discovery_host, network_config.service_ports.discovery_port)
                }),
            service_endpoints: HashMap::new(),
            development_fallbacks: DevelopmentFallbacks::default(),
            internal_dns_patterns: InternalDnsPatterns::default(),
        }
    }
}

impl Default for DevelopmentFallbacks {
    fn default() -> Self {
        // Use NetworkConfig for consistent port configuration
        let network_config = super::super::config::network::NetworkConfig::default();
        
        Self {
            base_discovery: std::env::var("BEARDOG_LOCAL_DISCOVERY")
        .unwrap_or_else(|_| {
            let localhost = std::env::var("BEARDOG_LOCALHOST")
                .unwrap_or_else(|| network_config.default_host.clone());
            format!("http://{}:{}", localhost, network_config.service_ports.discovery_port)
        }),
            service_ports: {
                let mut ports = HashMap::new();
                ports.insert("compute".to_string(), network_config.service_ports.compute_port);
                ports.insert("mesh".to_string(), network_config.service_ports.mesh_port);
                ports.insert("ai".to_string(), network_config.service_ports.ai_port);
                ports.insert("storage".to_string(), network_config.service_ports.storage_port);
                ports.insert("security".to_string(), network_config.service_ports.security_port);
                ports
            },
            bind_to_all_interfaces: env::var("BEARDOG_BIND_ALL_INTERFACES")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(true), // Default to true for containers
        }
    }
}

impl Default for InternalDnsPatterns {
    fn default() -> Self {
        Self {
            base_domain: env::var("ECOSYSTEM_INTERNAL_DOMAIN")
                .unwrap_or_else(|_| "ecosystem.internal".to_string()),
            service_patterns: {
                let mut patterns = HashMap::new();
                patterns.insert("compute".to_string(), "compute.{domain}".to_string());
                patterns.insert("mesh".to_string(), "mesh.{domain}".to_string());
                patterns.insert("ai".to_string(), "ai.{domain}".to_string());
                patterns.insert("storage".to_string(), "storage.{domain}".to_string());
                patterns.insert("security".to_string(), "security.{domain}".to_string());
                patterns.insert("discovery".to_string(), "discovery.{domain}".to_string());
                patterns.insert("registry".to_string(), "registry.{domain}".to_string());
                patterns
            },
            default_ports: {
                let mut ports = HashMap::new();
                ports.insert("http".to_string(), 
                    env::var("BEARDOG_DEFAULT_HTTP_PORT")
                        .ok()
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(8080));
                ports.insert("https".to_string(), 
                    env::var("BEARDOG_DEFAULT_HTTPS_PORT")
                        .ok()
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(8443));
                ports.insert("grpc".to_string(), 
                    env::var("BEARDOG_DEFAULT_GRPC_PORT")
                        .ok()
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(9090));
                ports.insert("metrics".to_string(), 
                    env::var("BEARDOG_DEFAULT_METRICS_PORT")
                        .ok()
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(9091));
                ports.insert("health".to_string(), 
                    env::var("BEARDOG_DEFAULT_HEALTH_PORT")
                        .ok()
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(8081));
                ports
            },
        }
    }
}

impl UniversalEndpointConfig {
    /// Create a new universal endpoint configuration
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Gets service_endpoint
    /// Gets service_endpoint
    pub fn get_service_endpoint(&self, service_name: &str) -> String {
        // Check explicit service endpoints first
        if let Some(endpoint) = self.service_endpoints.get(service_name) {
            return endpoint.clone();
        }
        
        // Check environment variables with capability-based naming
        let capability_env = format!("{}_ENDPOINT", service_name.to_uppercase());
        if let Ok(endpoint) = env::var(&capability_env) {
            return endpoint;
        }
        
        // Check legacy BearDog environment variables
        let legacy_env = format!("BEARDOG_{}_ENDPOINT", service_name.to_uppercase());
        if let Ok(endpoint) = env::var(&legacy_env) {
            return endpoint;
        }
        
        // Use internal DNS pattern for production
        if let Some(pattern) = self.internal_dns_patterns.service_patterns.get(service_name) {
            let endpoint = pattern.replace("{domain}", &self.internal_dns_patterns.base_domain);
            let port = self.internal_dns_patterns.default_ports.get("http").unwrap_or(&8080);
            return format!("http://{}:{}", endpoint, port);
        }
        
        // Fallback to development configuration
        if let Some(port) = self.development_fallbacks.service_ports.get(service_name) {
            use super::super::config::network::NetworkConfig;
            let network_config = NetworkConfig::default();
            let localhost = std::env::var("BEARDOG_LOCALHOST")
                .unwrap_or_else(|| network_config.default_host.clone());
            return format!("http://{}:{}", localhost, port);
        }
        
        // Ultimate fallback to discovery endpoint
        format!("{}/{}", self.discovery_endpoint, service_name)
    }
    
    /// Get bind address using universal patterns
    /// Gets bind_address
    pub fn get_bind_address(&self, default_port: u16) -> String {
        use super::super::config::network::NetworkConfig;
        let network_config = NetworkConfig::default();
        
        let host = if self.development_fallbacks.bind_to_all_interfaces {
            std::env::var("BEARDOG_BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string())
        } else {
            std::env::var("BEARDOG_LOCALHOST").unwrap_or_else(|| network_config.default_host.clone())
        };
        
        let port = env::var("BEARDOG_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(default_port);
            
        format!("{}:{}", host, port)
    }
    
    /// Get database endpoint using universal discovery
    /// Gets database_endpoint
    pub fn get_database_endpoint(&self) -> String {
        let network_config = super::super::config::network::NetworkConfig::default();
        
        env::var("DATABASE_ENDPOINT")
            .or_else(|_| env::var("BEARDOG_DB_HOST").map(|host| {
                let port = env::var("BEARDOG_DB_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(network_config.service_ports.database_port);
                format!("{}:{}", host, port)
            }))
            .unwrap_or_else(|_| {
                let db_host = env::var("DATABASE_HOST")
                    .unwrap_or_else(|_| "database.ecosystem.internal".to_string());
                format!("{}:{}", db_host, network_config.service_ports.database_port)
            })
    }
    
    /// Get monitoring endpoints using universal discovery
    /// Gets monitoring_endpoints
    pub fn get_monitoring_endpoints(&self) -> MonitoringEndpoints {
        let network_config = super::super::config::network::NetworkConfig::default();
        
        MonitoringEndpoints {
            prometheus: env::var("BEARDOG_METRICS_ENDPOINT")
                .or_else(|_| env::var("PROMETHEUS_ENDPOINT"))
                .unwrap_or_else(|_| {
                    let metrics_host = env::var("METRICS_HOST")
                        .unwrap_or_else(|_| "metrics.ecosystem.internal".to_string());
                    format!("http://{}:{}", metrics_host, network_config.service_ports.metrics_port)
                }),
            grafana: env::var("GRAFANA_ENDPOINT")
                .unwrap_or_else(|_| {
                    let grafana_host = env::var("GRAFANA_HOST")
                        .unwrap_or_else(|_| "grafana.ecosystem.internal".to_string());
                    format!("http://{}:{}", grafana_host, network_config.service_ports.grafana_port)
                }),
            jaeger: env::var("JAEGER_ENDPOINT")
                .unwrap_or_else(|_| {
                    let jaeger_host = env::var("JAEGER_HOST")
                        .unwrap_or_else(|_| "jaeger.ecosystem.internal".to_string());
                    format!("http://{}:{}/api/traces", jaeger_host, network_config.service_ports.jaeger_port)
                }),
        }
    }
    
    /// Validate all configured endpoints
    /// Validates endpoints
    /// Validates endpoints
    pub fn validate_endpoints(&self) -> impl std::future::Future<Output = ValidationResult {
        let mut results = ValidationResult::default();
        
        // Validate discovery endpoint
        results.discovery_reachable = self.check_endpoint_health(&self.discovery_endpoint);
        
        // Validate service endpoints
        for (service, endpoint) in &self.service_endpoints {
            let reachable = self.check_endpoint_health(endpoint);
            results.service_health.insert(service.clone(), reachable);
        }
        
        results
    }
    
    
    fn check_endpoint_health(&self, endpoint: &str) -> impl std::future::Future<Output = bool {
        // Simple health check - in production this would be more sophisticated
        match reqwest::get(&format!("{}/health", endpoint)) {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
}

/// Monitoring endpoints configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringEndpoints {
    /// Prometheus
    /// The prometheus value
    pub prometheus: String,
    /// Grafana
    /// The grafana value
    pub grafana: String,
    /// Jaeger
    /// The jaeger value
    pub jaeger: String,
}

/// Endpoint validation results
#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    /// Discovery Reachable
    /// Whether discovery_reachable is enabled
    pub discovery_reachable: bool,
    /// Service Health
    /// Mapping of service health
    pub service_health: HashMap<String, bool>,
}

pub struct UniversalEndpointResolver {
    config: UniversalEndpointConfig,
    cache: std::sync::RwLock<HashMap<String, (String, std::time::Instant)>>,
    cache_ttl: std::time::Duration,
}

impl UniversalEndpointResolver {
    /// Create a new universal endpoint resolver
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            config: UniversalEndpointConfig::new(),
            cache: std::sync::RwLock::new(HashMap::new()),
            cache_ttl: std::time::Duration::from_secs(
                env::var("BEARDOG_ENDPOINT_CACHE_TTL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300) // 5 minutes
            ),
        }
    }
    
    /// Resolve endpoint with caching
    pub fn resolve_endpoint(&self, service_name: &str) -> Result<String, crate::errors::BearDogError> {
        // Check cache first
        if let Ok(cache) = self.cache.read() {
            if let Some((endpoint, timestamp)) = cache.get(service_name) {
                if timestamp.elapsed() < self.cache_ttl {
                    return Ok(endpoint.clone());
                }
            }
        }
        
        // Resolve endpoint
        let endpoint = self.config.get_service_endpoint(service_name);
        
        // Update cache
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(service_name.to_string(), (endpoint.clone(), std::time::Instant::now()));
        }
        
        Ok(endpoint)
    }
    
    /// Clear endpoint cache
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }
}

impl Default for UniversalEndpointResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    fn test_universal_endpoint_config_creation() {
        let config = UniversalEndpointConfig::new();
        assert!(!config.discovery_endpoint.is_empty());
        assert!(!config.internal_dns_patterns.base_domain.is_empty());
    }
    
    #[tokio::test]
    fn test_service_endpoint_resolution() {
        let config = UniversalEndpointConfig::default();
        let endpoint = config.get_service_endpoint("compute");
        assert!(endpoint.contains("compute") || endpoint.contains("127.0.0.1"));
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }
    
    #[tokio::test]
    fn test_bind_address_generation() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let config = UniversalEndpointConfig::default();
        let bind_addr = config.get_bind_address(8080);
        assert!(bind_addr.contains("8080"));
        assert!(bind_addr.contains("0.0.0.0") || bind_addr.contains("127.0.0.1"));
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }
    
    #[tokio::test]
    fn test_database_endpoint_resolution() {
        let config = UniversalEndpointConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let db_endpoint = config.get_database_endpoint();
        assert!(!db_endpoint.is_empty());
        assert!(db_endpoint.contains(":"));
    }
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[tokio::test]
    fn test_endpoint_resolver() {
        let resolver = UniversalEndpointResolver::new();
        let result = resolver.resolve_endpoint("compute");
        assert!(result.is_ok());
        
        let endpoint = result?;
        assert!(!endpoint.is_empty());
    }
} 
