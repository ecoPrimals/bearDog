// SPDX-License-Identifier: AGPL-3.0-only

//! Core types for universal discovery (protocols, config, events, statistics).

use beardog_types::canonical::HealthStatus;
use beardog_types::canonical::config::network::NetworkConfig;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

use super::health::HealthCheckConfig;
use super::load_balancing::{LoadBalancingAlgorithm, LoadBalancingConfig};
use super::network::{
    CacheConfig as CanonicalCacheConfig, SecurityConfig as CanonicalSecurityConfig,
};
use super::registry::ServiceRegistryConfig;

/// Service discovery protocol types for universal service mesh
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscoveryProtocol {
    /// HTTP-based service discovery with REST endpoints
    Http {
        /// HTTP endpoint URL for service discovery
        endpoint: String,
        /// HTTP headers to include in discovery requests
        headers: HashMap<String, String>,
    },
    /// DNS-based service discovery using SRV records
    Dns {
        /// Domain name to query for SRV records
        domain: String,
        /// DNS server addresses to query
        servers: Vec<String>,
    },
    /// Multicast DNS for local network discovery
    Mdns {
        /// Service type identifier for mDNS queries
        service_type: String,
        /// Network interface for mDNS broadcasting
        interface: String,
        /// Discovery timeout in milliseconds
        timeout_ms: u64,
        /// Enable continuous monitoring mode
        continuous_monitoring: bool,
    },
    /// Consul-based service discovery and health checking
    Consul {
        /// Consul agent address and port
        address: String,
        /// Consul datacenter name
        datacenter: String,
    },
    /// etcd-based distributed service discovery
    Etcd {
        /// etcd cluster endpoints
        endpoints: Vec<String>,
        /// Key prefix for service discovery entries
        key_prefix: String,
        /// Request timeout in milliseconds
        timeout_ms: u64,
    },
}

impl Hash for DiscoveryProtocol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Http {
                endpoint,
                headers: _,
            } => {
                "Http".hash(state);
                endpoint.hash(state);
                // Skip headers as they're not hashable
            }
            Self::Dns { domain, servers } => {
                "Dns".hash(state);
                domain.hash(state);
                servers.hash(state);
            }
            Self::Mdns {
                service_type,
                interface,
                timeout_ms,
                continuous_monitoring,
            } => {
                "Mdns".hash(state);
                service_type.hash(state);
                interface.hash(state);
                timeout_ms.hash(state);
                continuous_monitoring.hash(state);
            }
            Self::Consul {
                address,
                datacenter,
            } => {
                "Consul".hash(state);
                address.hash(state);
                datacenter.hash(state);
            }
            Self::Etcd {
                endpoints,
                key_prefix,
                timeout_ms,
            } => {
                "Etcd".hash(state);
                endpoints.hash(state);
                key_prefix.hash(state);
                timeout_ms.hash(state);
            }
        }
    }
}

/// **UNIVERSAL DISCOVERY CONFIGURATION** - Primary discovery config
///
/// This consolidates all discovery configurations into a single, comprehensive system
/// that provides enterprise-grade service discovery capabilities across all environments.
///
/// **Updated**: Now uses canonical config types from the network module for cache and security.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalDiscoveryConfig {
    /// Service identifier
    pub service_id: String,
    /// Discovery protocols to enable
    /// Whether `feature_protocols` is enabled
    pub enabled_protocols: Vec<DiscoveryProtocol>,
    /// Service registry configuration
    pub registry_config: ServiceRegistryConfig,
    /// Health check configuration
    pub health_config: HealthCheckConfig,
    /// Load balancing configuration
    pub load_balancing_config: LoadBalancingConfig,
    /// Network configuration
    pub network_config: NetworkConfig,
    /// Cache configuration (canonical from network module)
    pub cache_config: CanonicalCacheConfig,
    /// Security configuration (canonical from network module)
    pub security_config: CanonicalSecurityConfig,
}

/// Discovery events for service lifecycle monitoring
///
/// Notifies subscribers of service registration, deregistration, and health changes
/// to enable dynamic service mesh management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryEvent {
    /// Service registration event notification
    ServiceRegistered {
        /// Unique identifier of the registered service
        service_id: String,
        /// Human-readable name of the service
        service_name: String,
    },
    /// Service deregistration event notification
    ServiceDeregistered {
        /// Unique identifier of the deregistered service
        service_id: String,
        /// Timestamp when the service was deregistered
        timestamp: DateTime<Utc>,
    },
    /// Service health status change event notification
    ServiceHealthChanged {
        /// Unique identifier of the service with health change
        service_id: String,
        /// Previous health status of the service
        old_status: HealthStatus,
        /// New health status of the service
        new_status: HealthStatus,
        /// Timestamp when the health status changed
        timestamp: DateTime<Utc>,
    },
    /// Load balancer configuration change event notification
    LoadBalancerConfigChanged {
        /// New load balancing algorithm being used
        algorithm: LoadBalancingAlgorithm,
        /// Timestamp when the configuration changed
        timestamp: DateTime<Utc>,
    },
    /// Protocol error event notification
    ProtocolError {
        /// Discovery protocol that encountered the error
        protocol: DiscoveryProtocol,
        /// Error message describing what went wrong
        error: String,
        /// Timestamp when the error occurred
        timestamp: DateTime<Utc>,
    },
}

impl Default for UniversalDiscoveryConfig {
    fn default() -> Self {
        let network_config = NetworkConfig::default();
        let endpoint = format!(
            "http://{}:{}/v1/catalog/services",
            network_config.default_host, 8500
        );
        Self {
            service_id: format!("primal-{}", Uuid::new_v4()),
            enabled_protocols: vec![
                DiscoveryProtocol::Mdns {
                    service_type: "_http._tcp".to_string(),
                    interface: "eth0".to_string(),
                    timeout_ms: 5000,
                    continuous_monitoring: true,
                },
                DiscoveryProtocol::Http {
                    endpoint,
                    headers: HashMap::new(),
                },
            ],
            registry_config: ServiceRegistryConfig::default(),
            health_config: HealthCheckConfig::default(),
            load_balancing_config: LoadBalancingConfig::default(),
            network_config,
            cache_config: CanonicalCacheConfig::default(),
            security_config: CanonicalSecurityConfig::default(),
        }
    }
}

impl UniversalDiscoveryConfig {
    /// Build configuration from process environment (`std::env::var`).
    #[must_use]
    pub fn from_env() -> Self {
        let primal_type = std::env::var("PRIMAL_TYPE")
            .or_else(|_| std::env::var("SERVICE_TYPE"))
            .unwrap_or_else(|_| "primal".to_string());

        let network_config = NetworkConfig::default();
        let http_endpoint = std::env::var("SERVICE_REGISTRY_ENDPOINT")
            .or_else(|_| std::env::var("DISCOVERY_SERVICE_ENDPOINT"))
            .or_else(|_| std::env::var("BEARDOG_CONSUL_ENDPOINT"))
            .or_else(|_| std::env::var("CONSUL_HTTP_ADDR"))
            .unwrap_or_else(|_| {
                let registry_host = std::env::var("SERVICE_REGISTRY_HOST")
                    .or_else(|_| std::env::var("CONSUL_HOST"))
                    .unwrap_or_else(|_| network_config.default_host.clone());
                let registry_port = std::env::var("SERVICE_REGISTRY_PORT")
                    .or_else(|_| std::env::var("CONSUL_PORT"))
                    .ok()
                    .and_then(|p| p.parse::<u16>().ok())
                    .unwrap_or(8500);
                format!("http://{registry_host}:{registry_port}/v1/catalog/services")
            });

        Self {
            service_id: format!("{primal_type}-{}", Uuid::new_v4()),
            enabled_protocols: vec![
                DiscoveryProtocol::Mdns {
                    service_type: "_http._tcp".to_string(),
                    interface: "eth0".to_string(),
                    timeout_ms: 5000,
                    continuous_monitoring: true,
                },
                DiscoveryProtocol::Http {
                    endpoint: http_endpoint,
                    headers: HashMap::new(),
                },
            ],
            registry_config: ServiceRegistryConfig::default(),
            health_config: HealthCheckConfig::default(),
            load_balancing_config: LoadBalancingConfig::default(),
            network_config,
            cache_config: CanonicalCacheConfig::default(),
            security_config: CanonicalSecurityConfig::default(),
        }
    }
}

/// Discovery statistics and operational metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryStatistics {
    /// Total number of registered services
    /// Number of `total_services`
    pub total_services: usize,
    /// Number of services currently healthy
    /// Number of `healthy_services`
    pub healthy_services: usize,
    /// Number of services currently unhealthy
    /// Number of `unhealthy_services`
    pub unhealthy_services: usize,
    /// Statistics broken down by discovery protocol
    /// Mapping of protocol statistics
    pub protocol_statistics: HashMap<DiscoveryProtocol, ProtocolStatistics>,
    /// Total system uptime since initialization
    pub uptime: std::time::Duration,
}

/// Protocol-specific operational statistics
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProtocolStatistics {
    /// Total number of services discovered via this protocol
    /// Number of `services_discovered`
    pub services_discovered: usize,
    /// Total number of discovery requests processed
    /// Number of `discovery_requests`
    pub discovery_requests: usize,
    /// Total number of service registration requests
    /// Number of `registration_requests`
    pub registration_requests: usize,
    /// Total number of errors encountered
    /// Number of errors
    pub errors: usize,
    /// Timestamp of last protocol activity
    /// The last activity value
    pub last_activity: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::collections::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash_of(p: &DiscoveryProtocol) -> u64 {
        let mut h = DefaultHasher::new();
        p.hash(&mut h);
        h.finish()
    }

    #[test]
    fn discovery_protocol_hash_variants_distinct() {
        let http = DiscoveryProtocol::Http {
            endpoint: "http://a".into(),
            headers: std::collections::HashMap::from([("h".into(), "v".into())]),
        };
        let dns = DiscoveryProtocol::Dns {
            domain: "d".into(),
            servers: vec!["8.8.8.8".into()],
        };
        assert_ne!(hash_of(&http), hash_of(&dns));
        let mdns = DiscoveryProtocol::Mdns {
            service_type: "_t._tcp".into(),
            interface: "eth0".into(),
            timeout_ms: 1,
            continuous_monitoring: false,
        };
        let consul = DiscoveryProtocol::Consul {
            address: "c".into(),
            datacenter: "dc".into(),
        };
        let etcd = DiscoveryProtocol::Etcd {
            endpoints: vec!["e".into()],
            key_prefix: "/p".into(),
            timeout_ms: 1,
        };
        assert_ne!(hash_of(&mdns), hash_of(&consul));
        assert_ne!(hash_of(&consul), hash_of(&etcd));
    }

    #[test]
    fn universal_discovery_config_default_and_serde() {
        let c = UniversalDiscoveryConfig::default();
        let v = serde_json::to_value(&c).expect("serialize config");
        let _: UniversalDiscoveryConfig = serde_json::from_value(v).expect("deserialize");
    }

    #[test]
    fn universal_discovery_from_env_smoke() {
        let _ = UniversalDiscoveryConfig::from_env();
    }

    #[test]
    fn discovery_events_roundtrip() {
        let ev = DiscoveryEvent::ServiceRegistered {
            service_id: "s".into(),
            service_name: "n".into(),
        };
        let v = serde_json::to_value(&ev).expect("serialize event");
        let back: DiscoveryEvent = serde_json::from_value(v).expect("deserialize");
        let _ = format!("{back:?}");

        let ev2 = DiscoveryEvent::ProtocolError {
            protocol: DiscoveryProtocol::Dns {
                domain: "x".into(),
                servers: vec![],
            },
            error: "e".into(),
            timestamp: Utc::now(),
        };
        let v = serde_json::to_value(&ev2).expect("serialize protocol error");
        let _: DiscoveryEvent = serde_json::from_value(v).expect("deserialize");
    }

    #[test]
    fn discovery_statistics_default_construct() {
        let s = DiscoveryStatistics {
            total_services: 0,
            healthy_services: 0,
            unhealthy_services: 0,
            protocol_statistics: HashMap::new(),
            uptime: std::time::Duration::ZERO,
        };
        let v = serde_json::to_value(&s).expect("serialize empty protocol stats");
        let _: DiscoveryStatistics = serde_json::from_value(v).expect("deserialize stats");
    }
}
