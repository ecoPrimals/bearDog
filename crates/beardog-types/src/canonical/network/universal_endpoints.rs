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
        Self {
            discovery_endpoint: env::var("BEARDOG_DISCOVERY_ENDPOINT")
                .or_else(|_| env::var("UNIVERSAL_DISCOVERY_ENDPOINT"))
                .unwrap_or_else(|_| "http://discovery.ecosystem.internal:8080".to_string()),
            service_endpoints: HashMap::new(),
            development_fallbacks: DevelopmentFallbacks::default(),
            internal_dns_patterns: InternalDnsPatterns::default(),
        }
    }
}

impl Default for DevelopmentFallbacks {
    fn default() -> Self {
        Self {
            base_discovery: std::env::var("BEARDOG_LOCAL_DISCOVERY")
        .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string()),
            service_ports: {
                let mut ports = HashMap::new();
                ports.insert("compute".to_string(), 8001);
                ports.insert("mesh".to_string(), 8002);
                ports.insert("ai".to_string(), 8003);
                ports.insert("storage".to_string(), 8004);
                ports.insert("security".to_string(), 8005);
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
                ports.insert("http".to_string(), 8080);
                ports.insert("https".to_string(), 8443);
                ports.insert("grpc".to_string(), 9090);
                ports.insert("metrics".to_string(), 9091);
                ports.insert("health".to_string(), 8081);
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
            return format!("http://127.0.0.1:{}", port);
        }
        
        // Ultimate fallback to discovery endpoint
        format!("{}/{}", self.discovery_endpoint, service_name)
    }
    
    /// Get bind address using universal patterns
    /// Gets bind_address
    /// Gets bind_address
    pub fn get_bind_address(&self, default_port: u16) -> String {
        let host = if self.development_fallbacks.bind_to_all_interfaces {
            "0.0.0.0"
        } else {
            "127.0.0.1"
        };
        
        let port = env::var("BEARDOG_PORT")
            .and_then(|p| p.parse().ok())
            .unwrap_or(default_port);
            
        format!("{}:{}", host, port)
    }
    
    /// Get database endpoint using universal discovery
    /// Gets database_endpoint
    /// Gets database_endpoint
    pub fn get_database_endpoint(&self) -> String {
        env::var("DATABASE_ENDPOINT")
            .or_else(|_| env::var("BEARDOG_DB_HOST").map(|host| {
                let port = env::var("BEARDOG_DB_PORT")
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(5432);
                format!("{}:{}", host, port)
            }))
            .unwrap_or_else(|_| "database.ecosystem.internal:5432".to_string())
    }
    
    /// Get monitoring endpoints using universal discovery
    /// Gets monitoring_endpoints
    /// Gets monitoring_endpoints
    pub fn get_monitoring_endpoints(&self) -> MonitoringEndpoints {
        MonitoringEndpoints {
            prometheus: env::var("PROMETHEUS_ENDPOINT")
                .unwrap_or_else(|_| "http://prometheus.ecosystem.internal:9090".to_string()),
            grafana: env::var("GRAFANA_ENDPOINT")
                .unwrap_or_else(|_| "http://grafana.ecosystem.internal:3000".to_string()),
            jaeger: env::var("JAEGER_ENDPOINT")
                .unwrap_or_else(|_| "http://jaeger.ecosystem.internal:14268/api/traces".to_string()),
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
            cache_ttl: std::time::Duration::from_secs(300), // 5 minutes
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
    }
    
    #[tokio::test]
    fn test_bind_address_generation() {
        let config = UniversalEndpointConfig::default();
        let bind_addr = config.get_bind_address(8080);
        assert!(bind_addr.contains("8080"));
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
        
        let endpoint = result.unwrap();
        assert!(!endpoint.is_empty());
    }
} 
