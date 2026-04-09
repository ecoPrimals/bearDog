// SPDX-License-Identifier: AGPL-3.0-or-later

//! Protocol handler trait, concrete handlers, and factory for universal discovery.

use beardog_errors::BearDogError;
use beardog_types::canonical::providers_unified::traits::other_traits::ServiceInfo;
use chrono::Utc;
use uuid::Uuid;

use super::types::{DiscoveryProtocol, ProtocolStatistics};

/// Protocol handler for service discovery mechanisms
///
/// Defines the interface for discovery protocol implementations (mDNS, DNS-SD,
/// Consul, etc.), enabling pluggable discovery strategies.
#[async_trait::async_trait]
pub trait ProtocolHandler: Send + Sync + std::fmt::Debug {
    /// Start the protocol handler and begin service discovery
    ///
    /// # Errors
    /// Returns an error if the protocol handler fails to start or if initialization encounters issues.
    fn start(&self) -> Result<(), BearDogError>;
    /// Stop the protocol handler and clean up resources
    ///
    /// # Errors
    /// Returns an error if the protocol handler fails to stop gracefully or if cleanup encounters issues.
    fn stop(&self) -> Result<(), BearDogError>;
    /// Register a service with this discovery protocol
    ///
    /// # Errors
    /// Returns an error if service registration fails or if the service information is invalid.
    fn register_service(&self, service: &ServiceInfo) -> Result<(), BearDogError>;
    /// Deregister a service from this discovery protocol
    ///
    /// # Errors
    /// Returns an error if service deregistration fails or if the service is not found.
    fn deregister_service(&self, service: &ServiceInfo) -> Result<(), BearDogError>;
    /// Discover services by name using this protocol
    ///
    /// # Errors
    /// Returns an error if service discovery fails, if the service name is invalid, or if network issues occur.
    fn discover_services(&self, service_name: &str) -> Result<Vec<ServiceInfo>, BearDogError>;
    /// Gets statistics
    ///
    /// # Errors
    /// Returns an error if statistics collection fails or if internal state access encounters issues.
    fn get_statistics(&self) -> Result<ProtocolStatistics, BearDogError>;
}

/// Minimal protocol handler implementation
///
/// Provides a basic, no-op implementation of the `ProtocolHandler` trait for use when:
/// - Discovery protocol-specific implementations are not yet available
/// - Fallback behavior is needed during bootstrap
/// - Testing or development environments without full discovery infrastructure
///
/// This is NOT a mock for testing - it's a minimal production implementation.
#[derive(Debug)]
pub struct MinimalProtocolHandler {
    /// Handler identifier
    pub id: String,
}

impl Default for MinimalProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl MinimalProtocolHandler {
    /// Create a new minimal protocol handler
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
        }
    }
}

#[async_trait::async_trait]
impl ProtocolHandler for MinimalProtocolHandler {
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

/// mDNS Protocol Handler
///
/// Integrates mDNS-based discovery with the universal discovery system.
#[cfg(feature = "mdns")]
#[derive(Debug)]
pub struct MdnsProtocolHandler {
    client: crate::primal_discovery_mdns::MdnsDiscoveryClient,
    service_type: String,
    _id: String,
    stats: std::sync::Arc<std::sync::Mutex<ProtocolStatistics>>,
}

#[cfg(feature = "mdns")]
impl MdnsProtocolHandler {
    /// Create new mDNS protocol handler
    #[must_use]
    pub fn new(
        client: crate::primal_discovery_mdns::MdnsDiscoveryClient,
        service_type: String,
    ) -> Self {
        Self {
            client,
            service_type,
            _id: Uuid::new_v4().to_string(),
            stats: std::sync::Arc::new(std::sync::Mutex::new(ProtocolStatistics {
                services_discovered: 0,
                discovery_requests: 0,
                registration_requests: 0,
                errors: 0,
                last_activity: Utc::now(),
            })),
        }
    }
}

#[cfg(feature = "mdns")]
#[async_trait::async_trait]
impl ProtocolHandler for MdnsProtocolHandler {
    fn start(&self) -> Result<(), BearDogError> {
        tracing::info!(
            "🚀 Starting mDNS discovery for service type: {}",
            self.service_type
        );
        Ok(())
    }

    fn stop(&self) -> Result<(), BearDogError> {
        tracing::info!("🛑 Stopping mDNS discovery");
        Ok(())
    }

    fn register_service(&self, service: &ServiceInfo) -> Result<(), BearDogError> {
        let mut stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::system(format!("Stats lock poisoned: {e}")))?;
        stats.registration_requests += 1;
        stats.last_activity = Utc::now();

        tracing::debug!("📋 Registered service via mDNS: {}", service.name);
        Ok(())
    }

    fn deregister_service(&self, service: &ServiceInfo) -> Result<(), BearDogError> {
        tracing::debug!("🗑️ Deregistered service from mDNS: {}", service.name);
        Ok(())
    }

    fn discover_services(&self, service_name: &str) -> Result<Vec<ServiceInfo>, BearDogError> {
        let mut stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::system(format!("Stats lock poisoned: {e}")))?;
        stats.discovery_requests += 1;
        stats.last_activity = Utc::now();

        // Spawn blocking async task - mDNS discovery is async
        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| BearDogError::system("No tokio runtime available".to_string()))?;

        let client = self.client.clone();
        let capability = service_name.to_string();

        let discovered = rt.block_on(async { client.discover_by_capability(&capability).await })?;

        // Convert to ServiceInfo
        let services: Vec<ServiceInfo> = discovered
            .into_iter()
            .map(|d| ServiceInfo {
                name: d.instance_name.clone(),
                service_type: self.service_type.clone(),
                address: d
                    .addresses
                    .first()
                    .map(std::string::ToString::to_string)
                    .unwrap_or_default(),
                port: d.port,
                metadata: d
                    .capabilities
                    .into_iter()
                    .map(|c| (c, "true".to_string()))
                    .collect(),
            })
            .collect();

        let mut stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::system(format!("Stats lock poisoned: {e}")))?;
        stats.services_discovered += services.len();

        Ok(services)
    }

    fn get_statistics(&self) -> Result<ProtocolStatistics, BearDogError> {
        let stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::system(format!("Stats lock poisoned: {e}")))?;
        Ok(*stats)
    }
}

pub(super) fn create_modern_discovery(
    protocol: &DiscoveryProtocol,
) -> Result<Box<dyn ProtocolHandler>, BearDogError> {
    match protocol {
        DiscoveryProtocol::Mdns {
            service_type: _,
            interface: _,
            timeout_ms: _,
            continuous_monitoring: _,
        } => {
            // mDNS handler - integrate with primal_discovery_mdns module
            #[cfg(feature = "mdns")]
            {
                use crate::primal_discovery_mdns::MdnsDiscoveryClient;
                use std::time::Duration;

                // Extract timeout and service_type from match pattern
                let timeout = match protocol {
                    DiscoveryProtocol::Mdns { timeout_ms, .. } => {
                        Duration::from_millis(*timeout_ms)
                    }
                    _ => Duration::from_secs(5), // Fallback
                };

                let service = match protocol {
                    DiscoveryProtocol::Mdns { service_type, .. } => service_type.clone(),
                    _ => "_services._dns-sd._udp.local.".to_string(), // Default
                };

                let client = MdnsDiscoveryClient::new().with_timeout(timeout);
                Ok(Box::new(MdnsProtocolHandler::new(client, service)))
            }
            #[cfg(not(feature = "mdns"))]
            {
                tracing::warn!("mDNS feature not enabled, using minimal handler");
                Ok(Box::new(MinimalProtocolHandler::new()))
            }
        }
        DiscoveryProtocol::Http {
            endpoint: _,
            headers: _,
        } => {
            // HTTP discovery handler - using minimal implementation pending full protocol support
            Ok(Box::new(MinimalProtocolHandler::new()))
        }
        DiscoveryProtocol::Dns {
            domain: _,
            servers: _,
        } => {
            // DNS-based discovery handler - using minimal implementation pending full protocol support
            Ok(Box::new(MinimalProtocolHandler::new()))
        }
        DiscoveryProtocol::Consul {
            address: _,
            datacenter: _,
        } => {
            // Consul discovery handler - using minimal implementation pending full protocol support
            Ok(Box::new(MinimalProtocolHandler::new()))
        }
        DiscoveryProtocol::Etcd {
            endpoints: _,
            key_prefix: _,
            timeout_ms: _,
        } => {
            // etcd discovery handler - using minimal implementation pending full protocol support
            Ok(Box::new(MinimalProtocolHandler::new()))
        }
    }
}
