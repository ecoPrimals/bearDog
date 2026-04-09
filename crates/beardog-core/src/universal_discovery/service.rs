// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal service discovery orchestrator.

use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use beardog_types::canonical::providers_unified::traits::other_traits::ServiceInfo;
use chrono::Utc;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, info, warn};

use super::health::{self, HealthMonitor};
use super::load_balancing::LoadBalancer;
use super::protocol_handlers::{ProtocolHandler, create_modern_discovery};
use super::registry::ServiceRegistry;
use super::types::{
    DiscoveryEvent, DiscoveryProtocol, DiscoveryStatistics, ProtocolStatistics,
    UniversalDiscoveryConfig,
};

/// **UNIVERSAL SERVICE DISCOVERY** - The unified discovery orchestrator
#[derive(Debug)]
pub struct UniversalServiceDiscovery {
    config: UniversalDiscoveryConfig,

    /// Service discovery protocol handlers
    discovery_instances: HashMap<DiscoveryProtocol, Box<dyn ProtocolHandler>>,

    _service_registry: ServiceRegistry,

    health_monitor: HealthMonitor,

    load_balancer: LoadBalancer,

    event_tx: broadcast::Sender<DiscoveryEvent>,

    /// Shutdown signal sender
    shutdown_tx: mpsc::Sender<()>,
}

impl UniversalServiceDiscovery {
    /// Create a new universal discovery service
    ///
    /// # Errors
    /// Returns an error if protocol handlers fail to initialize, if the configuration is invalid,
    /// or if any of the internal components (registry, health monitor, load balancer) fail to start.
    pub fn new(config: UniversalDiscoveryConfig) -> Result<Self, BearDogError> {
        // Create event broadcasting channel
        let (event_tx, _event_rx) = broadcast::channel(1000);

        // Create shutdown channel
        let (shutdown_tx, _shutdown_rx) = mpsc::channel(1);

        // Initialize protocol handlers
        let mut discovery_instances = HashMap::new();
        for protocol in &config.enabled_protocols {
            let handler = create_modern_discovery(protocol)?;
            discovery_instances.insert(protocol.clone(), handler);
        }

        Ok(Self {
            config: config.clone(),
            discovery_instances,
            _service_registry: ServiceRegistry::new(&config.registry_config)?,
            health_monitor: HealthMonitor::new(&config.health_config)?,
            load_balancer: LoadBalancer::new(&config.load_balancing_config)?,
            event_tx,
            shutdown_tx,
        })
    }

    /// Start the discovery service
    ///
    /// # Errors
    /// Returns an error if any protocol handler fails to start or if service initialization encounters issues.
    pub fn start(&self) -> Result<(), BearDogError> {
        info!(
            "Starting Universal Discovery Service with {} protocols",
            self.config.enabled_protocols.len()
        );

        self.start_protocol_handlers()?;
        self.start_monitoring_services()?;

        info!("Universal Discovery Service started successfully");
        Ok(())
    }

    /// Start all protocol handlers
    fn start_protocol_handlers(&self) -> Result<(), BearDogError> {
        for (protocol, handler) in &self.discovery_instances {
            handler.start().map_err(|e| BearDogError::Network {
                message: format!("Failed to start protocol handler for {protocol:?}: {e}"),
                category: beardog_errors::NetworkErrorCategory::Protocol,
            })?;
        }
        Ok(())
    }

    /// Start health monitoring and load balancer
    fn start_monitoring_services(&self) -> Result<(), BearDogError> {
        self.health_monitor.start()?;
        self.load_balancer.start()?;
        Ok(())
    }

    /// Stop the discovery service
    ///
    /// # Errors
    /// Returns an error if any protocol handler fails to stop gracefully or if shutdown procedures encounter issues.
    pub fn stop(&self) -> Result<(), BearDogError> {
        info!("Stopping Universal Discovery Service");

        // Send shutdown signal (using try_send for non-async context)
        let _ = self.shutdown_tx.try_send(());

        self.stop_protocol_handlers();
        self.stop_monitoring_services()?;

        info!("Universal Discovery Service stopped");
        Ok(())
    }

    /// Stop all protocol handlers (best-effort, logs errors)
    fn stop_protocol_handlers(&self) {
        for (protocol, handler) in &self.discovery_instances {
            if let Err(e) = handler.stop() {
                warn!("Error stopping protocol handler for {:?}: {}", protocol, e);
            }
        }
    }

    /// Stop health monitoring and load balancer
    fn stop_monitoring_services(&self) -> Result<(), BearDogError> {
        self.health_monitor.stop()?;
        self.load_balancer.stop()?;
        Ok(())
    }

    /// Register a service
    ///
    /// # Errors
    /// Returns an error if the service information is invalid, if registration capacity is exceeded,
    /// or if the service registry encounters issues during registration.
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
    ///
    /// # Errors
    /// Returns an error if the service is not found, if deregistration fails, or if cleanup operations encounter issues.
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
    ///
    /// # Errors
    /// Returns an error if the service name is invalid, if discovery operations fail, or if network issues occur.
    pub async fn discover_services(
        &self,
        service_name: &str,
    ) -> Result<Vec<ServiceInfo>, BearDogError> {
        debug!("Discovering services with name: {}", service_name);

        let discovered_services = self.query_all_protocols(service_name);
        let balanced_services = self
            .process_discovered_services(discovered_services)
            .await?;

        debug!(
            "Discovered {} services for name: {}",
            balanced_services.len(),
            service_name
        );
        Ok(balanced_services)
    }

    /// Query all protocol handlers for services
    fn query_all_protocols(&self, service_name: &str) -> Vec<ServiceInfo> {
        let mut discovered_services = Vec::new();

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

        discovered_services
    }

    /// Deduplicate and balance discovered services
    async fn process_discovered_services(
        &self,
        discovered_services: Vec<ServiceInfo>,
    ) -> Result<Vec<ServiceInfo>, BearDogError> {
        let unique_services = Self::deduplicate_services(discovered_services);
        self.load_balancer.balance_services(unique_services).await
    }

    /// Get the health status of a specific service
    ///
    /// # Errors
    /// Returns an error if the service ID is not found, if health monitoring is unavailable,
    /// or if the health check operation fails.
    pub const fn get_service_health(
        &self,
        service_id: &str,
    ) -> Result<Option<HealthStatus>, BearDogError> {
        Ok(self.health_monitor.get_service_health(service_id))
    }

    /// Subscribe to discovery events
    #[must_use]
    pub fn subscribe_events(&self) -> broadcast::Receiver<DiscoveryEvent> {
        self.event_tx.subscribe()
    }

    /// Get comprehensive discovery statistics
    ///
    /// # Errors
    /// Returns an error if statistics collection fails or if any protocol handler reports errors.
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
            uptime: Self::get_uptime(),
        };

        Ok(stats)
    }

    /// Get count of healthy services
    ///
    /// # Errors
    /// Returns an error if health monitoring statistics collection fails or if internal state is inconsistent.
    pub fn get_healthy_service_count(&self) -> Result<usize, BearDogError> {
        let stats = self.health_monitor.get_health_statistics();
        Ok(stats.healthy_services)
    }

    // Private helper methods
    fn deduplicate_services(services: Vec<ServiceInfo>) -> Vec<ServiceInfo> {
        let mut unique_services = HashMap::new();
        for service in services {
            unique_services.insert(service.name.clone(), service);
        }
        unique_services.into_values().collect()
    }

    /// Gets `protocol_statistics`
    fn _get_protocol_statistics(
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
    const fn get_uptime() -> Duration {
        // This would be implemented with actual start time tracking
        Duration::from_secs(0)
    }
}
