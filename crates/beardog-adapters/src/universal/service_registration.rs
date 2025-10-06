//! Service Registration Module
//!
//! Provides service registration and discovery functionality for universal adapters.

use crate::ecosystem::{EcosystemService, IntegrationResult};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Service registry for managing registered services
#[derive(Debug)]
pub struct ServiceRegistry {
    /// Registered services
    services: Arc<RwLock<HashMap<String, EcosystemService>>>,
    /// Registry configuration
    config: ServiceRegistryConfig,
}

/// Configuration for service registry
/// Renamed from RegistryConfig to ServiceRegistryConfig for clarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistryConfig {
    /// Maximum number of services
    pub max_services: usize,
    /// Service timeout in seconds
    pub service_timeout_seconds: u64,
    /// Enable service health checks
    pub enable_health_checks: bool,
}

/// Backward compatibility alias
#[deprecated(since = "3.2.0", note = "Use ServiceRegistryConfig instead")]
pub type RegistryConfig = ServiceRegistryConfig;

/// Service registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {
    /// Service to register
    pub service: EcosystemService,
    /// Registration metadata
    pub metadata: HashMap<String, String>,
}

impl ServiceRegistry {
    /// Create a new service registry
    pub fn new(config: ServiceRegistryConfig) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Register a service
    pub async fn register_service(&self, request: RegistrationRequest) -> Result<IntegrationResult, BearDogError> {
        let mut services = self.services.write().await;
        
        if services.len() >= self.config.max_services {
            return Ok(IntegrationResult {
                success: false,
                message: "Maximum services reached".to_string(),
                data: None,
            });
        }

        services.insert(request.service.id.clone(), request.service.clone());
        
        Ok(IntegrationResult {
            success: true,
            message: format!("Service {} registered successfully", request.service.id),
            data: Some(serde_json::to_value(&request.service).unwrap_or_default()),
        })
    }

    /// Discover services by type
    pub async fn discover_services(&self, service_type: &str) -> Result<Vec<EcosystemService>, BearDogError> {
        let services = self.services.read().await;
        let matching_services: Vec<EcosystemService> = services
            .values()
            .filter(|service| service.service_type == service_type)
            .cloned()
            .collect();
        
        Ok(matching_services)
    }

    /// Get service by ID
    pub async fn get_service(&self, service_id: &str) -> Result<Option<EcosystemService>, BearDogError> {
        let services = self.services.read().await;
        Ok(services.get(service_id).cloned())
    }
}

impl Default for ServiceRegistryConfig {
    fn default() -> Self {
        Self {
            max_services: 100,
            service_timeout_seconds: 30,
            enable_health_checks: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_registration() -> Result<(), BearDogError> {
        let registry = ServiceRegistry::new(RegistryConfig::default());
        
        let service = EcosystemService {
            id: "test-service".to_string(),
            name: "Test Service".to_string(),
            service_type: "test".to_string(),
            endpoints: HashMap::new(),
            capabilities: vec!["test".to_string()],
        };

        let request = RegistrationRequest {
            service,
            metadata: HashMap::new(),
        };

        let result = registry.register_service(request).await?;
        assert!(result.success);
        
        Ok(())
    }
}
