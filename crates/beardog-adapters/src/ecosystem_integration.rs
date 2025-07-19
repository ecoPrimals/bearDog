//! BearDog Ecosystem Integration
//!
//! This module provides high-level integration functions for BearDog with the
//! ecosystem following the Universal Primal Architecture Standard

use std::sync::Arc;
use crate::{EcosystemResult, universal::*};

/// BearDog ecosystem integration facade
pub struct BearDogEcosystemIntegration {
    /// Universal ecosystem provider
    provider: BearDogEcosystemProvider,
    
    /// Service discovery client
    discovery: ServiceDiscoveryClient,
}

impl BearDogEcosystemIntegration {
    /// Create new BearDog ecosystem integration
    pub fn new(config: BearDogEcosystemConfig) -> Self {
        let provider = BearDogEcosystemProvider::new(config);
        let mut discovery = ServiceDiscoveryClient::new(DiscoveryConfig::default());
        
        // Add in-memory backend for local testing
        discovery.add_backend(Box::new(InMemoryDiscoveryBackend::new()));
        
        Self {
            provider,
            discovery,
        }
    }
    
    /// Register BearDog in the ecosystem
    pub async fn register(&self) -> EcosystemResult<()> {
        let registration = self.provider.register().await?;
        self.discovery.register_service(&registration).await?;
        Ok(())
    }
    
    /// Discover services by capability
    pub async fn discover_capability(&mut self, capability: &str) -> EcosystemResult<Vec<UniversalServiceRegistration>> {
        self.discovery.discover_by_capability(capability).await
    }
    
    /// Get BearDog provider health status
    pub async fn health(&self) -> EcosystemResult<HealthStatus> {
        self.provider.health_check().await
    }
    
    /// Handle ecosystem request
    pub async fn handle_request(&self, request: EcosystemRequest) -> EcosystemResult<crate::AIFirstResponse<serde_json::Value>> {
        self.provider.handle_request(request).await
    }
}

impl Default for BearDogEcosystemIntegration {
    fn default() -> Self {
        Self::new(BearDogEcosystemConfig::default())
    }
}
