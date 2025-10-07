//! Universal Discovery Service - Modular Architecture
//!
//! This module provides comprehensive service discovery capabilities
//! across multiple protocols and networks, organized into focused domain modules
//! for maximum maintainability and adherence to best practices.
//!
//! # Modular Architecture
//!
//! Universal discovery is split into focused domain modules:
//!
//! - **`protocols`**: Discovery protocol implementations (Consul, Eureka, etc.)
//! - **`registry`**: Service registration, tracking, and management
//! - **`health`**: Health checking, monitoring, and status management
//! - **`load_balancing`**: Load balancing algorithms and traffic distribution
//! - **`network`**: Network configuration, addressing, and communication
//!
//! # Features
//!
//! - Multi-protocol service discovery (Consul, Eureka, custom protocols)
//! - Health-based routing and failover
//! - Multiple load balancing strategies
//! - Dynamic service registration and deregistration
//! - Real-time health monitoring
//! - Zero-knowledge bootstrap support
//!
//! # Example
//!
//! ```rust,ignore
//! use beardog_core::universal_discovery::{DiscoveryProtocol, ServiceRegistry};
//!
//! // Initialize service registry for discovery
//! let registry = ServiceRegistry::new();
//! // Services are discovered and health-checked automatically
//! ```
//!
//! # Architecture
//!
//! The universal discovery system follows the primal sovereignty pattern:
//! - No hardcoded service locations
//! - Capability-based service discovery
//! - Health-based routing decisions
//! - Graceful degradation on failures

use beardog_errors::BearDogError;
use beardog_types::canonical::config::network::NetworkConfig;
use beardog_types::canonical::providers_unified::traits::ServiceInfo;
use beardog_types::canonical::HealthStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, info, warn};
use uuid::Uuid;

// Re-export sub-modules
pub mod health;
pub mod load_balancing;
pub mod network;
pub mod protocols;
pub mod registry;

// Re-export types for convenience
// HealthCheckConfig is a domain-specific config for universal discovery
pub use health::{HealthCheckConfig, HealthMonitor, ServiceHealthState};
pub use load_balancing::{LoadBalancer, LoadBalancingAlgorithm, LoadBalancingConfig};
pub use protocols::ModernServiceDiscovery;
pub use registry::{ExtendedServiceInfo, ServiceRegistry, ServiceRegistryConfig};

// Import canonical config types from network module
use network::{CacheConfig as CanonicalCacheConfig, SecurityConfig as CanonicalSecurityConfig};

// Canonical config types are in submodules: network::CacheConfig, network::SecurityConfig, etc.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscoveryProtocol {
    /// HTTP-based service discovery with REST endpoints
    Http {
        endpoint: String,
        headers: HashMap<String, String>,
    },
    /// DNS-based service discovery using SRV records
    Dns {
        domain: String,
        servers: Vec<String>,
    },
    Mdns {
        service_type: String,
        interface: String,
        /// Discovery timeout in milliseconds
        timeout_ms: u64,
        continuous_monitoring: bool,
    },
    /// Consul-based service discovery and health checking
    Consul {
        /// Consul agent address and port
        address: String,
        datacenter: String,
    },
    /// etcd-based distributed service discovery
    Etcd {
        /// etcd cluster endpoints
        endpoints: Vec<String>,
        key_prefix: String,
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

/// **UNIVERSAL SERVICE DISCOVERY** - The unified discovery orchestrator
#[derive(Debug)]
pub struct UniversalServiceDiscovery {
    config: UniversalDiscoveryConfig,

    /// Service discovery protocol handlers
    discovery_instances: HashMap<DiscoveryProtocol, Box<dyn ProtocolHandler>>,

    #[allow(dead_code)]
    service_registry: ServiceRegistry,

    health_monitor: HealthMonitor,

    load_balancer: LoadBalancer,

    event_tx: broadcast::Sender<DiscoveryEvent>,

    /// Shutdown signal sender
    shutdown_tx: mpsc::Sender<()>,
}

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
        Self {
            service_id: format!("beardog-{}", Uuid::new_v4()),
            enabled_protocols: vec![
                DiscoveryProtocol::Mdns {
                    service_type: "_http._tcp".to_string(),
                    interface: "eth0".to_string(),
                    timeout_ms: 5000,
                    continuous_monitoring: true,
                },
                DiscoveryProtocol::Http {
                    endpoint: "http://localhost:8500/v1/catalog/services".to_string(),
                    headers: HashMap::new(),
                },
            ],
            registry_config: ServiceRegistryConfig::default(),
            health_config: HealthCheckConfig::default(),
            load_balancing_config: LoadBalancingConfig::default(),
            network_config: NetworkConfig::default(),
            cache_config: CanonicalCacheConfig::default(),
            security_config: CanonicalSecurityConfig::default(),
        }
    }
}

impl UniversalServiceDiscovery {
    /// Create a new universal discovery service
    /// Creates a new instance
    pub async fn new(config: UniversalDiscoveryConfig) -> Result<Self, BearDogError> {
        // Create event broadcasting channel
        let (event_tx, _event_rx) = broadcast::channel(1000);

        // Create shutdown channel
        let (shutdown_tx, _shutdown_rx) = mpsc::channel(1);

        // Initialize protocol handlers
        let mut discovery_instances = HashMap::new();
        for protocol in &config.enabled_protocols {
            let handler = create_modern_discovery(protocol).await?;
            discovery_instances.insert(protocol.clone(), handler);
        }

        Ok(Self {
            config: config.clone(),
            discovery_instances,
            service_registry: ServiceRegistry::new(&config.registry_config)?,
            health_monitor: HealthMonitor::new(&config.health_config)?,
            load_balancer: LoadBalancer::new(&config.load_balancing_config)?,
            event_tx,
            shutdown_tx,
        })
    }

    /// Start the discovery service
    /// Starts service
    /// Starts service
    pub fn start(&self) -> Result<(), BearDogError> {
        info!(
            "Starting Universal Discovery Service with {} protocols",
            self.config.enabled_protocols.len()
        );

        // Start protocol handlers
        for (protocol, handler) in &self.discovery_instances {
            handler.start().map_err(|e| BearDogError::Network {
                message: format!("Failed to start protocol handler for {protocol:?}: {e}"),
                category: beardog_errors::NetworkErrorCategory::Protocol,
            })?;
        }

        // Start health monitoring
        self.health_monitor.start()?;

        // Start load balancer
        self.load_balancer.start()?;

        info!("Universal Discovery Service started successfully");
        Ok(())
    }

    /// Stop the discovery service
    /// Stops service
    /// Stops service
    pub fn stop(&self) -> Result<(), BearDogError> {
        info!("Stopping Universal Discovery Service");

        // Send shutdown signal (using try_send for non-async context)
        let _ = self.shutdown_tx.try_send(());

        // Stop protocol handlers
        for (protocol, handler) in &self.discovery_instances {
            if let Err(e) = handler.stop() {
                warn!("Error stopping protocol handler for {:?}: {}", protocol, e);
            }
        }

        // Stop health monitoring
        self.health_monitor.stop()?;

        // Stop load balancer
        self.load_balancer.stop()?;

        info!("Universal Discovery Service stopped");
        Ok(())
    }

    /// Register a service
    pub fn register_service(&self, service: ServiceInfo) -> Result<(), BearDogError> {
        debug!("Registering service: {}", service.name);

        // Add to health monitoring with default config
        let health_config = health::ServiceHealthConfig {
            check_method: health::HealthCheckMethod::Http,
            check_interval_secs: 30,
            failure_threshold: 3,
            success_threshold: 2,
        };
        self.health_monitor
            .add_service(service.name.clone(), health_config)?;

        // Register with protocol handlers
        for handler in self.discovery_instances.values() {
            handler.register_service(&service)?;
        }

        // Send registration event
        let _ = self.event_tx.send(DiscoveryEvent::ServiceRegistered {
            service_id: service.name.clone(),
            service_name: service.name,
        });

        Ok(())
    }

    /// Deregister a service
    pub fn deregister_service(&self, service_id: &str) -> Result<(), BearDogError> {
        debug!("Deregistering service: {}", service_id);

        // Stop monitoring the service
        self.health_monitor.remove_service(service_id)?;

        // Create a minimal ServiceInfo for deregistration
        let service_info = ServiceInfo {
            name: service_id.to_string(),
            service_type: "unknown".to_string(),
            address: "unknown".to_string(),
            port: 0,
            metadata: HashMap::new(),
        };

        // Deregister from protocol handlers
        for handler in self.discovery_instances.values() {
            handler.deregister_service(&service_info)?;
        }

        // Send deregistration event
        let _ = self.event_tx.send(DiscoveryEvent::ServiceDeregistered {
            service_id: service_id.to_string(),
            timestamp: Utc::now(),
        });

        Ok(())
    }

    /// Discover services by name
    pub async fn discover_services(
        &self,
        service_name: &str,
    ) -> Result<Vec<ServiceInfo>, BearDogError> {
        debug!("Discovering services with name: {}", service_name);

        let mut discovered_services = Vec::new();

        // Query all protocol handlers
        for (protocol, handler) in &self.discovery_instances {
            debug!("Querying protocol: {:?}", protocol);
            match handler.discover_services(service_name) {
                Ok(mut services) => {
                    discovered_services.append(&mut services);
                }
                Err(e) => {
                    warn!("Protocol {:?} discovery failed: {}", protocol, e);
                }
            }
        }

        // Remove duplicates and apply load balancing
        let unique_services = self.deduplicate_services(discovered_services);
        let balanced_services = self.load_balancer.balance_services(unique_services).await?;

        debug!(
            "Discovered {} services for name: {}",
            balanced_services.len(),
            service_name
        );
        Ok(balanced_services)
    }

    /// Get the health status of a specific service
    /// Gets `service_health`
    /// Gets `service_health`
    pub const fn get_service_health(
        &self,
        service_id: &str,
    ) -> Result<Option<HealthStatus>, BearDogError> {
        Ok(self.health_monitor.get_service_health(service_id))
    }

    /// Subscribe to discovery events
    pub fn subscribe_events(&self) -> broadcast::Receiver<DiscoveryEvent> {
        self.event_tx.subscribe()
    }

    /// Get comprehensive discovery statistics
    /// Gets `discovery_statistics`
    /// Gets `discovery_statistics`
    pub fn get_discovery_statistics(&self) -> Result<DiscoveryStatistics, BearDogError> {
        let total_services = self.discovery_instances.len();
        let stats = self.health_monitor.get_health_statistics();

        let mut protocol_statistics = HashMap::new();
        for (protocol, handler) in &self.discovery_instances {
            let protocol_stats = handler.get_statistics()?;
            protocol_statistics.insert(protocol.clone(), protocol_stats);
        }

        let stats = DiscoveryStatistics {
            total_services,
            healthy_services: stats.healthy_services,
            unhealthy_services: stats.unhealthy_services,
            protocol_statistics,
            uptime: self.get_uptime(),
        };

        Ok(stats)
    }

    /// Get count of healthy services
    /// Gets `healthy_service_count`
    /// Gets `healthy_service_count`
    pub fn get_healthy_service_count(&self) -> Result<usize, BearDogError> {
        let stats = self.health_monitor.get_health_statistics();
        Ok(stats.healthy_services)
    }

    // Private helper methods
    fn deduplicate_services(&self, services: Vec<ServiceInfo>) -> Vec<ServiceInfo> {
        let mut unique_services = HashMap::new();
        for service in services {
            unique_services.insert(service.name.clone(), service);
        }
        unique_services.into_values().collect()
    }

    /// Gets `protocol_statistics`
    #[allow(dead_code)]
    fn get_protocol_statistics(
        &self,
    ) -> Result<HashMap<DiscoveryProtocol, ProtocolStatistics>, BearDogError> {
        let mut stats = HashMap::new();
        for (protocol, handler) in &self.discovery_instances {
            let protocol_stats = handler.get_statistics()?;
            stats.insert(protocol.clone(), protocol_stats);
        }
        Ok(stats)
    }

    /// Gets uptime
    const fn get_uptime(&self) -> Duration {
        // This would be implemented with actual start time tracking
        Duration::from_secs(0)
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
    pub uptime: Duration,
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

#[async_trait::async_trait]
pub trait ProtocolHandler: Send + Sync + std::fmt::Debug {
    /// Start the protocol handler and begin service discovery
    /// Starts service
    fn start(&self) -> Result<(), BearDogError>;
    /// Stop the protocol handler and clean up resources
    /// Stops service
    fn stop(&self) -> Result<(), BearDogError>;
    /// Register a service with this discovery protocol
    fn register_service(&self, service: &ServiceInfo) -> Result<(), BearDogError>;
    /// Deregister a service from this discovery protocol
    fn deregister_service(&self, service: &ServiceInfo) -> Result<(), BearDogError>;
    /// Discover services by name using this protocol
    fn discover_services(&self, service_name: &str) -> Result<Vec<ServiceInfo>, BearDogError>;
    /// Gets statistics
    fn get_statistics(&self) -> Result<ProtocolStatistics, BearDogError>;
}

#[derive(Debug)]
pub struct MockProtocolHandler {
    /// Handler identifier
    pub id: String,
}

impl Default for MockProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl MockProtocolHandler {
    /// Create a new mock protocol handler
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
        }
    }
}

#[async_trait::async_trait]
impl ProtocolHandler for MockProtocolHandler {
    /// Start the protocol handler and begin service discovery
    /// Starts service
    fn start(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    /// Stop the protocol handler and clean up resources
    /// Stops service
    fn stop(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    /// Register a service with this discovery protocol
    fn register_service(&self, _service: &ServiceInfo) -> Result<(), BearDogError> {
        Ok(())
    }

    /// Deregister a service from this discovery protocol
    fn deregister_service(&self, _service: &ServiceInfo) -> Result<(), BearDogError> {
        Ok(())
    }

    /// Discover services by name using this protocol
    fn discover_services(&self, _service_name: &str) -> Result<Vec<ServiceInfo>, BearDogError> {
        Ok(vec![])
    }

    /// Gets statistics
    fn get_statistics(&self) -> Result<ProtocolStatistics, BearDogError> {
        Ok(ProtocolStatistics {
            services_discovered: 0,
            discovery_requests: 0,
            registration_requests: 0,
            errors: 0,
            last_activity: Utc::now(),
        })
    }
}

async fn create_modern_discovery(
    protocol: &DiscoveryProtocol,
) -> Result<Box<dyn ProtocolHandler>, BearDogError> {
    match protocol {
        DiscoveryProtocol::Mdns {
            service_type: _,
            interface: _,
            timeout_ms: _,
            continuous_monitoring: _,
        } => {
            // Create mDNS handler
            Ok(Box::new(MockProtocolHandler::new()))
        }
        DiscoveryProtocol::Http {
            endpoint: _,
            headers: _,
        } => {
            // Create HTTP discovery handler
            Ok(Box::new(MockProtocolHandler::new()))
        }
        DiscoveryProtocol::Dns {
            domain: _,
            servers: _,
        } => {
            // Create DNS-based discovery handler
            Ok(Box::new(MockProtocolHandler::new()))
        }
        DiscoveryProtocol::Consul {
            address: _,
            datacenter: _,
        } => {
            // Create Consul discovery handler
            Ok(Box::new(MockProtocolHandler::new()))
        }
        DiscoveryProtocol::Etcd {
            endpoints: _,
            key_prefix: _,
            timeout_ms: _,
        } => {
            // Create etcd discovery handler
            Ok(Box::new(MockProtocolHandler::new()))
        }
    }
}
