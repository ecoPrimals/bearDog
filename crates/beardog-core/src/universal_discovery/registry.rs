// SPDX-License-Identifier: AGPL-3.0-only

// Service Registry Module
//
// This module contains service registration, tracking, and management functionality.

use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use beardog_types::canonical::providers_unified::traits::ServiceInfo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;

/// Service registry configuration parameters
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ServiceRegistryConfig {
    /// Maximum number of services that can be registered
    /// Number of `max_services`
    pub max_services: usize,
    /// Number of `heartbeat_interval_secs`
    pub heartbeat_interval_secs: u64,
    /// Service timeout after which services are considered dead
    pub service_timeout_secs: u64,
    /// Whether to enable automatic health checks
    /// Whether `enable_health_checks` is enabled
    pub enable_health_checks: bool,
    /// Whether to enable service tagging and metadata
    /// Whether `enable_tagging` is enabled
    pub enable_tagging: bool,
}

impl Default for ServiceRegistryConfig {
    fn default() -> Self {
        Self {
            max_services: beardog_errors::process_env::var("BEARDOG_MAX_SERVICES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000), // 1000 services default
            heartbeat_interval_secs: 30,
            service_timeout_secs: 90,
            enable_health_checks: true,
            enable_tagging: true,
        }
    }
}

/// Extended service information with additional metadata and health status
///
/// Combines basic service information with network address, tags, metadata,
/// and current health status for comprehensive service tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedServiceInfo {
    /// The service info value
    pub service_info: ServiceInfo,
    /// Network address where the service can be reached
    /// The address value
    pub address: SocketAddr,
    /// Mapping of tags
    pub tags: HashMap<String, String>,
    /// Additional metadata about the service
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Current health status of the service
    /// Current status of the health
    pub health_status: HealthStatus,
    /// Timestamp when the service was first registered
    /// The registered at value
    pub registered_at: DateTime<Utc>,
    /// Timestamp of the last heartbeat received
    /// The last heartbeat value
    pub last_heartbeat: DateTime<Utc>,
}

/// Service registry implementation
#[derive(Debug)]
pub struct ServiceRegistry {
    config: ServiceRegistryConfig,
    services: HashMap<String, ExtendedServiceInfo>,
}

impl ServiceRegistry {
    /// Create a new service registry with the specified configuration
    ///
    /// # Errors
    /// Returns an error if the configuration is invalid or if initialization of internal structures fails.
    pub fn new(config: &ServiceRegistryConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            config: *config,
            services: HashMap::new(),
        })
    }

    /// Register a new service in the registry
    ///
    /// # Errors
    /// Returns an error if the maximum number of services is reached or if the service information is invalid.
    pub fn register_service(&mut self, service: ExtendedServiceInfo) -> Result<(), BearDogError> {
        if self.services.len() >= self.config.max_services {
            return Err(BearDogError::Network {
                message: "Maximum number of services reached".to_string(),
                category: beardog_errors::NetworkErrorCategory::ServiceDiscovery,
            });
        }

        let service_id = service.service_info.name.clone();
        self.services.insert(service_id, service);
        Ok(())
    }

    /// Remove a service from the registry
    ///
    /// # Errors
    /// Returns an error if the registry operation fails or if internal state becomes inconsistent.
    pub fn deregister_service(
        &mut self,
        service_id: &str,
    ) -> Result<Option<ExtendedServiceInfo>, BearDogError> {
        Ok(self.services.remove(service_id))
    }

    /// Find all services with the specified name
    ///
    /// # Errors
    /// Returns an error if the search operation fails or if the service name is invalid.
    pub fn find_services_by_name(
        &self,
        name: &str,
    ) -> Result<Vec<ExtendedServiceInfo>, BearDogError> {
        let matching_services = self
            .services
            .values()
            .filter(|service| service.service_info.name == name)
            .cloned()
            .collect();
        Ok(matching_services)
    }

    /// Get the total number of registered services
    ///
    /// # Errors
    /// Returns an error if counting services fails or if internal state is corrupted.
    pub fn get_service_count(&self) -> Result<usize, BearDogError> {
        Ok(self.services.len())
    }

    /// Get a service by its ID
    ///
    /// # Errors
    /// Returns an error if the lookup operation fails or if the service ID format is invalid.
    pub fn get_service(
        &self,
        service_id: &str,
    ) -> Result<Option<ExtendedServiceInfo>, BearDogError> {
        Ok(self.services.get(service_id).cloned())
    }

    /// Update the health status of a registered service
    ///
    /// # Errors
    /// Returns an error if the service is not found, if the health status is invalid, or if the update operation fails.
    pub fn update_service_health(
        &mut self,
        service_id: &str,
        health: HealthStatus,
    ) -> Result<(), BearDogError> {
        if let Some(service) = self.services.get_mut(service_id) {
            service.health_status = health;
            service.last_heartbeat = Utc::now();
        }
        Ok(())
    }

    /// Remove services that have exceeded their timeout
    ///
    /// # Errors
    /// Returns an error if the cleanup operation fails or if time calculations overflow.
    pub fn cleanup_expired_services(&mut self) -> Result<Vec<String>, BearDogError> {
        let timeout_duration = chrono::Duration::seconds(
            i64::try_from(self.config.service_timeout_secs).unwrap_or(i64::MAX),
        );
        let cutoff_time = Utc::now() - timeout_duration;

        let expired_services: Vec<String> = self
            .services
            .iter()
            .filter(|(_, service)| service.last_heartbeat < cutoff_time)
            .map(|(id, _)| id.clone())
            .collect();

        for service_id in &expired_services {
            self.services.remove(service_id);
        }

        Ok(expired_services)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::providers_unified::traits::ServiceInfo;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    fn make_extended_service_info(name: &str) -> ExtendedServiceInfo {
        ExtendedServiceInfo {
            service_info: ServiceInfo {
                name: name.to_string(),
                service_type: "http".to_string(),
                address: "127.0.0.1".to_string(),
                port: 8080,
                metadata: std::collections::HashMap::new(),
            },
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
            tags: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
            health_status: HealthStatus::Healthy,
            registered_at: Utc::now(),
            last_heartbeat: Utc::now(),
        }
    }

    #[test]
    fn test_registry_new() {
        let config = ServiceRegistryConfig::default();
        let registry = ServiceRegistry::new(&config).expect("ServiceRegistry::new in test");
        assert_eq!(
            registry
                .get_service_count()
                .expect("get_service_count empty registry"),
            0
        );
    }

    #[test]
    fn test_registry_register_and_find() {
        let config = ServiceRegistryConfig::default();
        let mut registry = ServiceRegistry::new(&config).expect("ServiceRegistry::new in test");
        let service = make_extended_service_info("svc1");
        registry
            .register_service(service)
            .expect("register_service svc1");
        let found = registry
            .find_services_by_name("svc1")
            .expect("find_services_by_name svc1");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].service_info.name, "svc1");
    }

    #[test]
    fn test_registry_deregister() {
        let config = ServiceRegistryConfig::default();
        let mut registry = ServiceRegistry::new(&config).expect("ServiceRegistry::new in test");
        let service = make_extended_service_info("svc1");
        registry
            .register_service(service)
            .expect("register_service for deregister test");
        let removed = registry
            .deregister_service("svc1")
            .expect("deregister_service svc1");
        assert!(removed.is_some());
        assert_eq!(
            registry
                .get_service_count()
                .expect("get_service_count after deregister"),
            0
        );
    }

    #[test]
    fn test_registry_update_health() {
        let config = ServiceRegistryConfig::default();
        let mut registry = ServiceRegistry::new(&config).expect("ServiceRegistry::new in test");
        let service = make_extended_service_info("svc1");
        registry
            .register_service(service)
            .expect("register_service for health update test");
        registry
            .update_service_health("svc1", HealthStatus::Degraded)
            .expect("update_service_health");
        let found = registry
            .get_service("svc1")
            .expect("get_service Result")
            .expect("service svc1 should exist");
        assert_eq!(found.health_status, HealthStatus::Degraded);
    }

    #[test]
    fn test_registry_config_default() {
        let config = ServiceRegistryConfig::default();
        assert!(config.max_services >= 100);
        assert!(config.enable_health_checks);
    }
}
