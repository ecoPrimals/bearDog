// Service Registry Module
//
// This module contains service registration, tracking, and management functionality.

use beardog_errors::BearDogError;
use beardog_types::canonical::providers_unified::traits::ServiceInfo;
use beardog_types::canonical::HealthStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;

/// Service registry configuration parameters
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ServiceRegistryConfig {
    /// Maximum number of services that can be registered
    /// Number of max_services
    pub max_services: usize,
    /// Number of heartbeat_interval_secs
    pub heartbeat_interval_secs: u64,
    /// Service timeout after which services are considered dead
    pub service_timeout_secs: u64,
    /// Whether to enable automatic health checks
    /// Whether enable_health_checks is enabled
    pub enable_health_checks: bool,
    /// Whether to enable service tagging and metadata
    /// Whether enable_tagging is enabled
    pub enable_tagging: bool,
}

impl Default for ServiceRegistryConfig {
    fn default() -> Self {
        Self {
            max_services: 1000,
            heartbeat_interval_secs: 30,
            service_timeout_secs: 90,
            enable_health_checks: true,
            enable_tagging: true,
        }
    }
}

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
    /// Creates a new instance
    pub fn new(config: &ServiceRegistryConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            config: config.clone(),
            services: HashMap::new(),
        })
    }

    /// Register a new service in the registry
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
    pub fn deregister_service(
        &mut self,
        service_id: &str,
    ) -> Result<Option<ExtendedServiceInfo>, BearDogError> {
        Ok(self.services.remove(service_id))
    }

    /// Find all services with the specified name
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
    /// Gets service_count
    /// Gets service_count
    pub fn get_service_count(&self) -> Result<usize, BearDogError> {
        Ok(self.services.len())
    }

    /// Gets service
    /// Gets service
    pub fn get_service(
        &self,
        service_id: &str,
    ) -> Result<Option<ExtendedServiceInfo>, BearDogError> {
        Ok(self.services.get(service_id).cloned())
    }

    /// Update the health status of a registered service
    /// Updates service_health
    /// Updates service_health
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
    /// Cleans up expired_services
    /// Cleans up expired_services
    pub fn cleanup_expired_services(&mut self) -> Result<Vec<String>, BearDogError> {
        let timeout_duration = chrono::Duration::seconds(self.config.service_timeout_secs as i64);
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
