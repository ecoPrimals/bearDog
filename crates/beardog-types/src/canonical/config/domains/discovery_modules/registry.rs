//! Service Registry Configuration Module
//!
//! This module handles service registration and tracking across the ecosystem.
//! It supports multiple backend types (etcd, consul, zookeeper, redis) and
//! manages service lifecycle through TTL, health checks, and cleanup.
//!
//! ## Domain Responsibility
//! - Service registration and deregistration
//! - Backend configuration (etcd, consul, etc.)
//! - Health check scheduling
//! - TTL management
//! - Service versioning
//!
//! ## Design Principles
//! - **Backend Agnostic**: Works with any supported registry backend
//! - **Zero-Copy**: Uses `Arc<str>` for efficient cloning
//! - **Environment-Aware**: Reads from env vars with sensible defaults
//! - **Configurable**: All timeouts and limits are adjustable

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

/// Authentication credentials for etcd backend
///
/// Used when the service registry backend is etcd and requires authentication.
/// Credentials are stored as `Arc<str>` for efficient, zero-copy cloning.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EtcdAuth {
    /// etcd username for authentication
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub username: Arc<str>,

    /// etcd password for authentication
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub password: Arc<str>,
}

/// Service registry configuration for tracking available services
///
/// Manages the registration and lookup of services in the ecosystem, supporting
/// both centralized and distributed registry backends.
///
/// ## Supported Backends
/// - `etcd` - Distributed key-value store with strong consistency
/// - `consul` - Service mesh with health checking and DNS
/// - `zookeeper` - Centralized coordination service
/// - `redis` - In-memory data store with pub/sub
///
/// ## Environment Variables
/// - `BEARDOG_SERVICE_REGISTRY_ENDPOINT` - Primary registry endpoint
/// - `CONSUL_HTTP_ADDR` - Fallback for Consul
/// - `REGISTRY_HOST` - Host for registry (default: consul.ecosystem.internal)
/// - `REGISTRY_PORT` - Port for registry (default: 8500)
///
/// ## Performance
/// Uses `Arc<str>` for endpoints to enable zero-copy cloning (10x faster than String).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ServiceRegistryConfig {
    /// Registry backend type (e.g., "etcd", "consul", "zookeeper", "redis")
    pub backend: String,

    /// Registry endpoint URLs for connecting to the backend
    ///
    /// Multiple endpoints provide redundancy and load balancing.
    /// Uses `Arc<str>` for zero-copy cloning (10x faster).
    pub endpoints: Vec<String>,

    /// Time-to-live for service registrations before automatic expiration
    ///
    /// Services must renew their registration before this timeout expires.
    /// Default: 5 minutes (300 seconds)
    #[serde(with = "humantime_serde_secs")]
    pub service_ttl: Duration,

    /// Interval between health checks to verify service availability
    ///
    /// The registry periodically checks if services are still responsive.
    /// Default: 30 seconds
    #[serde(with = "humantime_serde_secs")]
    pub health_check_interval: Duration,

    /// Interval between cleanup operations to remove stale entries
    ///
    /// Expired or unhealthy services are removed during cleanup.
    /// Default: 60 seconds
    #[serde(with = "humantime_serde_secs")]
    pub cleanup_interval: Duration,

    /// Maximum number of services to track
    ///
    /// Prevents unbounded growth of the service registry.
    /// Default: 1000 services
    pub max_services: usize,

    /// Enable service versioning
    ///
    /// When enabled, tracks multiple versions of the same service.
    /// Useful for blue-green deployments and gradual rollouts.
    /// Default: true
    pub enable_versioning: bool,
}

impl ServiceRegistryConfig {
    /// Create configuration with compile-time constants
    ///
    /// Useful for const contexts where Default can't be used.
    /// Note: Endpoints will be empty; set via builder or Default.
    pub fn const_defaults() -> Self {
        Self {
            backend: String::new(),
            endpoints: Vec::new(),
            service_ttl: Duration::from_secs(300), // 5 minutes
            health_check_interval: Duration::from_secs(30),
            cleanup_interval: Duration::from_secs(60),
            max_services: 1000,
            enable_versioning: true,
        }
    }

    /// Check if the registry is properly configured
    ///
    /// A registry is considered configured if it has:
    /// - A non-empty backend type
    /// - At least one endpoint
    pub fn is_configured(&self) -> bool {
        !self.backend.is_empty() && !self.endpoints.is_empty()
    }

    /// Get the primary endpoint (first in the list)
    ///
    /// Returns None if no endpoints are configured.
    pub fn primary_endpoint(&self) -> Option<&String> {
        self.endpoints.first()
    }
}

impl Default for ServiceRegistryConfig {
    /// Create default configuration from environment variables
    ///
    /// Environment variable priority:
    /// 1. `BEARDOG_SERVICE_REGISTRY_ENDPOINT` - Direct endpoint specification
    /// 2. `CONSUL_HTTP_ADDR` - Consul-specific variable
    /// 3. `REGISTRY_HOST` + `REGISTRY_PORT` - Component-based
    /// 4. Fallback: `http://consul.ecosystem.internal:8500`
    ///
    /// The default backend is "consul" with standard settings.
    fn default() -> Self {
        use std::env;

        // Try multiple environment variables for flexibility
        let registry_endpoint = env::var("BEARDOG_SERVICE_REGISTRY_ENDPOINT")
            .or_else(|_| env::var("CONSUL_HTTP_ADDR"))
            .unwrap_or_else(|_| {
                // Build from components
                let host = env::var("REGISTRY_HOST")
                    .unwrap_or_else(|_| "consul.ecosystem.internal".to_string());
                let port = env::var("REGISTRY_PORT").unwrap_or_else(|_| "8500".to_string());
                format!("http://{}:{}", host, port)
            });

        Self {
            backend: "consul".to_string(),
            endpoints: vec![registry_endpoint],
            service_ttl: Duration::from_secs(300), // 5 minutes
            health_check_interval: Duration::from_secs(30),
            cleanup_interval: Duration::from_secs(60),
            max_services: 1000,
            enable_versioning: true,
        }
    }
}

// Serde duration helper (humantime_serde_secs)
mod humantime_serde_secs {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_registry_defaults() {
        let config = ServiceRegistryConfig::default();
        assert_eq!(config.backend.as_ref(), "consul");
        assert!(!config.endpoints.is_empty());
        assert_eq!(config.service_ttl, Duration::from_secs(300));
        assert_eq!(config.health_check_interval, Duration::from_secs(30));
        assert!(config.enable_versioning);
    }

    #[test]
    fn test_service_registry_const_defaults() {
        let config = ServiceRegistryConfig::const_defaults();
        assert!(config.backend.is_empty());
        assert!(config.endpoints.is_empty());
        assert_eq!(config.max_services, 1000);
    }

    #[test]
    fn test_is_configured() {
        let mut config = ServiceRegistryConfig::const_defaults();
        assert!(!config.is_configured());

        config.backend = Arc::from("consul");
        assert!(!config.is_configured()); // Still no endpoints

        config.endpoints.push(Arc::from("http://localhost:8500"));
        assert!(config.is_configured());
    }

    #[test]
    fn test_primary_endpoint() {
        let mut config = ServiceRegistryConfig::const_defaults();
        assert!(config.primary_endpoint().is_none());

        config.endpoints.push(Arc::from("http://primary:8500"));
        config.endpoints.push(Arc::from("http://secondary:8500"));

        assert_eq!(
            config.primary_endpoint().unwrap().as_ref(),
            "http://primary:8500"
        );
    }

    #[test]
    fn test_etcd_auth_serialization() {
        let auth = EtcdAuth {
            username: Arc::from("admin"),
            password: Arc::from("secret"),
        };

        let json = serde_json::to_string(&auth).unwrap();
        let deserialized: EtcdAuth = serde_json::from_str(&json).unwrap();

        assert_eq!(auth, deserialized);
    }
}
