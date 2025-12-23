// Universal Endpoint Configuration
//
// This module provides universal endpoint patterns that replace hardcoded localhost
// and IP addresses with discoverable, environment-based configurations.

use beardog_config::domains::network_ports::{DEFAULT_DISCOVERY_PORT, DEFAULT_API_PORT};
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
            use super::super::constants::domains::network::config;
            let host = std::env::var("BEARDOG_LOCALHOST")
                .unwrap_or_else(|_| config::default_service_host());
            format!("http://{}:{}", host, network_config.service_ports.discovery_port)
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
                // ✅ MODERN IDIOMATIC: Use config constants instead of hardcoded values
                use beardog_config::domains::network_ports;
                
                // HTTP port from config
                ports.insert("http".to_string(), 
                    env::var("BEARDOG_API_PORT")
                        .ok()
                        .and_then(|p| p.parse().ok())
                        .unwrap_or_else(|| {
                            use beardog_config::global::BEARDOG_CONFIG;
                            BEARDOG_CONFIG.network.api.port
                        }));
                
                // HTTPS port from config
                ports.insert("https".to_string(), 
                    env::var("BEARDOG_HTTPS_PORT")
                        .ok()
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(network_ports::DEFAULT_HTTPS_PORT));
                
                // gRPC port from config
                ports.insert("grpc".to_string(), 
                    env::var("BEARDOG_DEFAULT_GRPC_PORT")
                        .ok()
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(DEFAULT_DISCOVERY_PORT));
                
                // Metrics port from config
                ports.insert("metrics".to_string(), 
                    env::var("BEARDOG_DEFAULT_METRICS_PORT")
                        .ok()
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(network_ports::DEFAULT_METRICS_PORT));
                
                // Health port from config
                ports.insert("health".to_string(), 
                    env::var("BEARDOG_DEFAULT_HEALTH_PORT")
                        .ok()
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(network_ports::DEFAULT_HEALTH_PORT));
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
            let port = self.internal_dns_patterns.default_ports.get("http").unwrap_or(&DEFAULT_API_PORT);
            return format!("http://{}:{}", endpoint, port);
        }
        
        // Fallback to development configuration
        if let Some(port) = self.development_fallbacks.service_ports.get(service_name) {
            use super::super::constants::domains::network::config;
            let host = std::env::var("BEARDOG_LOCALHOST")
                .unwrap_or_else(|_| config::default_service_host());
            return format!("http://{}:{}", host, port);
        }
        
        // Ultimate fallback to discovery endpoint
        format!("{}/{}", self.discovery_endpoint, service_name)
    }
    
    /// Get bind address using universal patterns
    /// Gets bind_address
    pub fn get_bind_address(&self, default_port: u16) -> String {
        use super::super::constants::domains::network::config;
        
        let host = if self.development_fallbacks.bind_to_all_interfaces {
            std::env::var("BEARDOG_BIND_ADDRESS")
                .unwrap_or_else(|_| config::default_service_host())
        } else {
            std::env::var("BEARDOG_LOCALHOST")
                .unwrap_or_else(|_| config::default_service_host())
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
    use std::env;
    
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
    }
    
    #[tokio::test]
    fn test_bind_address_generation() {
        const TEST_PORT: u16 = 8080;
        let config = UniversalEndpointConfig::default();
        let bind_addr = config.get_bind_address(TEST_PORT);
        assert!(bind_addr.contains(&TEST_PORT.to_string()));
        assert!(bind_addr.contains("0.0.0.0") || bind_addr.contains("127.0.0.1"));
    }
    
    #[tokio::test]
    fn test_database_endpoint_resolution() {
        let config = UniversalEndpointConfig::default();
        let db_endpoint = config.get_database_endpoint();
        assert!(!db_endpoint.is_empty());
        assert!(db_endpoint.contains(":"));
    }
    
    #[tokio::test]
    fn test_endpoint_resolver() {
        let resolver = UniversalEndpointResolver::new();
        let result = resolver.resolve_endpoint("compute");
        assert!(result.is_ok());
        
        if let Ok(endpoint) = result {
            assert!(!endpoint.is_empty());
        }
    }
    
    // NEW COMPREHENSIVE TESTS BELOW
    
    #[tokio::test]
    fn test_development_fallbacks_default() {
        let fallbacks = DevelopmentFallbacks::default();
        assert!(!fallbacks.base_discovery.is_empty());
        assert!(!fallbacks.service_ports.is_empty());
        assert!(fallbacks.service_ports.contains_key("compute"));
        assert!(fallbacks.service_ports.contains_key("storage"));
    }
    
    #[tokio::test]
    fn test_internal_dns_patterns_default() {
        let patterns = InternalDnsPatterns::default();
        assert_eq!(patterns.base_domain, "ecosystem.internal");
        assert!(patterns.service_patterns.contains_key("compute"));
        assert!(patterns.default_ports.contains_key("http"));
    }
    
    #[tokio::test]
    fn test_get_service_endpoint_with_explicit_mapping() {
        let mut config = UniversalEndpointConfig::default();
        config.service_endpoints.insert(
            "test-service".to_string(),
            "http://explicit-endpoint:9000".to_string()
        );
        
        let endpoint = config.get_service_endpoint("test-service");
        assert_eq!(endpoint, "http://explicit-endpoint:9000");
    }
    
    #[tokio::test]
    fn test_get_service_endpoint_with_env_var() {
        env::set_var("COMPUTE_ENDPOINT", "http://env-compute:8080");
        let config = UniversalEndpointConfig::default();
        let endpoint = config.get_service_endpoint("compute");
        assert_eq!(endpoint, "http://env-compute:8080");
        env::remove_var("COMPUTE_ENDPOINT");
    }
    
    #[tokio::test]
    fn test_get_service_endpoint_with_legacy_env() {
        env::set_var("BEARDOG_STORAGE_ENDPOINT", "http://legacy-storage:9000");
        let config = UniversalEndpointConfig::default();
        let endpoint = config.get_service_endpoint("storage");
        assert_eq!(endpoint, "http://legacy-storage:9000");
        env::remove_var("BEARDOG_STORAGE_ENDPOINT");
    }
    
    #[tokio::test]
    fn test_get_service_endpoint_fallback_to_development() {
        let config = UniversalEndpointConfig::default();
        let endpoint = config.get_service_endpoint("compute");
        // Should use development fallback port
        assert!(endpoint.contains(&config.development_fallbacks.service_ports["compute"].to_string()));
    }
    
    #[tokio::test]
    fn test_get_bind_address_all_interfaces() {
        env::set_var("BEARDOG_BIND_ALL_INTERFACES", "true");
        const TEST_PORT: u16 = 9000;
        let config = UniversalEndpointConfig::default();
        let bind_addr = config.get_bind_address(TEST_PORT);
        assert_eq!(bind_addr, "0.0.0.0:9000");
        env::remove_var("BEARDOG_BIND_ALL_INTERFACES");
    }
    
    #[tokio::test]
    fn test_get_bind_address_localhost_only() {
        env::set_var("BEARDOG_BIND_ALL_INTERFACES", "false");
        const TEST_PORT: u16 = 8500;
        let config = UniversalEndpointConfig::default();
        let bind_addr = config.get_bind_address(TEST_PORT);
        assert!(bind_addr.starts_with("127.0.0.1:") || bind_addr.starts_with("localhost:"));
        env::remove_var("BEARDOG_BIND_ALL_INTERFACES");
    }
    
    #[tokio::test]
    fn test_get_database_endpoint_with_env() {
        env::set_var("BEARDOG_DATABASE_ENDPOINT", "postgresql://db-server:5432/beardog");
        let config = UniversalEndpointConfig::default();
        let endpoint = config.get_database_endpoint();
        assert_eq!(endpoint, "postgresql://db-server:5432/beardog");
        env::remove_var("BEARDOG_DATABASE_ENDPOINT");
    }
    
    #[tokio::test]
    fn test_get_database_endpoint_default() {
        let config = UniversalEndpointConfig::default();
        let endpoint = config.get_database_endpoint();
        assert!(!endpoint.is_empty());
        assert!(endpoint.contains("postgresql://") || endpoint.contains("127.0.0.1"));
    }
    
    #[tokio::test]
    fn test_endpoint_resolver_caching() {
        let resolver = UniversalEndpointResolver::new();
        
        // First resolution
        let result1 = resolver.resolve_endpoint("compute");
        assert!(result1.is_ok());
        
        // Second resolution should use cache
        let result2 = resolver.resolve_endpoint("compute");
        assert!(result2.is_ok());
        assert_eq!(result1.unwrap(), result2.unwrap());
    }
    
    #[tokio::test]
    fn test_endpoint_resolver_clear_cache() {
        let resolver = UniversalEndpointResolver::new();
        
        let _ = resolver.resolve_endpoint("storage");
        resolver.clear_cache();
        
        // Should resolve again after cache clear
        let result = resolver.resolve_endpoint("storage");
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    fn test_endpoint_resolver_cache_ttl() {
        env::set_var("BEARDOG_ENDPOINT_CACHE_TTL_SECS", "1");
        let resolver = UniversalEndpointResolver::new();
        
        let _ = resolver.resolve_endpoint("ai");
        // Cache should exist but we can't easily test TTL expiration without waiting
        assert!(resolver.resolve_endpoint("ai").is_ok());
        
        env::remove_var("BEARDOG_ENDPOINT_CACHE_TTL_SECS");
    }
    
    #[tokio::test]
    fn test_multiple_service_endpoint_resolutions() {
        let config = UniversalEndpointConfig::default();
        
        let services = vec!["compute", "storage", "ai", "mesh", "security"];
        for service in services {
            let endpoint = config.get_service_endpoint(service);
            assert!(!endpoint.is_empty(), "Endpoint for {} should not be empty", service);
            assert!(endpoint.contains(service) || endpoint.contains("127.0.0.1"));
        }
    }
    
    #[tokio::test]
    fn test_unknown_service_endpoint() {
        let config = UniversalEndpointConfig::default();
        let endpoint = config.get_service_endpoint("unknown-service");
        // Should fall back to discovery endpoint pattern
        assert!(endpoint.contains("unknown-service"));
    }
    
    #[tokio::test]
    fn test_service_ports_all_populated() {
        let fallbacks = DevelopmentFallbacks::default();
        let required_services = vec!["compute", "mesh", "ai", "storage", "security"];
        
        for service in required_services {
            assert!(
                fallbacks.service_ports.contains_key(service),
                "Service port for {} should be present", 
                service
            );
        }
    }
    
    #[tokio::test]
    fn test_internal_dns_pattern_substitution() {
        let config = UniversalEndpointConfig::default();
        
        // Test that DNS patterns are properly formed
        assert!(config.internal_dns_patterns.service_patterns.contains_key("compute"));
        
        let pattern = &config.internal_dns_patterns.service_patterns["compute"];
        assert!(pattern.contains("{domain}"), "Pattern should contain domain placeholder");
    }
} 
