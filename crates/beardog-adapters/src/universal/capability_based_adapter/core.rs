//! # Core Capability Adapter
//!
//! Core structure and initialization logic for the Universal Capability Adapter.

use super::types::*;
use crate::ecosystem::primal_types::{DiscoveredPrimal, PrimalMetrics, UniversalEndpoint};
use crate::universal::types::{AdapterConfig, AdapterMetrics, CapabilityConnection};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::capabilities::{
    ServiceCapabilityType, UniversalCapability,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Universal Capability-Based Adapter
///
/// This adapter revolutionizes ecosystem integration by:
/// 1. Eliminating ALL hardcoded vendor dependencies
/// 2. Enabling dynamic capability-based discovery  
/// 3. Supporting infinite ecosystem scalability (O(1) vs 2^n)
/// 4. Maintaining true primal sovereignty
pub struct UniversalCapabilityAdapter {
    /// Discovered capabilities indexed by type
    capabilities: Arc<RwLock<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>>>,
    /// Discovered primals indexed by ID
    primals: Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,
    /// Adapter configuration
    config: AdapterConfig,
    metrics: AdapterMetrics,
    /// Active connections to discovered capabilities
    connections: Arc<RwLock<HashMap<String, CapabilityConnection>>>,
}

impl UniversalCapabilityAdapter {
    /// Create new universal capability adapter
    pub async fn new() -> BearDogResult<Self> {
        Self::with_config(AdapterConfig::default()).await
    }

    /// Create universal adapter with custom configuration
    pub async fn with_config(config: AdapterConfig) -> BearDogResult<Self> {
        info!("🔌 Initializing Universal Capability Adapter");
        info!("🎯 Mission: Replace ALL hardcoded integrations with dynamic discovery");
        info!("📋 Configuration:");
        info!(
            "   📊 Max providers per capability: {}",
            config.max_providers_per_capability
        );
        info!(
            "   ❤️  Health check interval: {}s",
            config.health_check_interval_secs
        );
        info!(
            "   ⏱️  Connection timeout: {}ms",
            config.connection_timeout_ms
        );

        let adapter = Self {
            capabilities: Arc::new(RwLock::new(HashMap::new())),
            primals: Arc::new(RwLock::new(HashMap::new())),
            config,
            metrics: AdapterMetrics::default(),
            connections: Arc::new(RwLock::new(HashMap::new())),
        };

        info!("✅ Universal Capability Adapter initialized successfully");
        info!("🚀 Ready to discover and integrate capabilities dynamically");

        Ok(adapter)
    }

    /// Get available capabilities
    pub async fn get_available_capabilities(&self) -> BearDogResult<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>> {
        let capabilities = self.capabilities.read().await;
        Ok(capabilities.clone())
    }

    /// Get discovered primals
    pub async fn get_discovered_primals(&self) -> BearDogResult<HashMap<String, DiscoveredPrimal>> {
        let primals = self.primals.read().await;
        Ok(primals.clone())
    }

    /// Get adapter metrics
    pub fn get_metrics(&self) -> &AdapterMetrics {
        &self.metrics
    }

    /// Health check all active connections
    pub async fn health_check_all_connections(&self) -> BearDogResult<HashMap<String, bool>> {
        let connections = self.connections.read().await;
        let mut health_results = HashMap::new();

        for (connection_id, connection) in connections.iter() {
            // Perform health check on each connection
            let is_healthy = self.check_connection_health(connection).await?;
            health_results.insert(connection_id.clone(), is_healthy);
        }

        Ok(health_results)
    }

    /// Check health of a specific connection
    async fn check_connection_health(&self, connection: &CapabilityConnection) -> BearDogResult<bool> {
        // Implementation would depend on the connection type
        // For now, return true as a placeholder
        Ok(true)
    }

    /// Get configuration
    pub fn config(&self) -> &AdapterConfig {
        &self.config
    }

    /// Update configuration
    pub async fn update_config(&mut self, new_config: AdapterConfig) -> BearDogResult<()> {
        info!("🔄 Updating Universal Capability Adapter configuration");
        self.config = new_config;
        info!("✅ Configuration updated successfully");
        Ok(())
    }

    /// Shutdown the adapter and clean up resources
    pub async fn shutdown(&self) -> BearDogResult<()> {
        info!("🛑 Shutting down Universal Capability Adapter");
        
        // Close all connections
        let mut connections = self.connections.write().await;
        connections.clear();
        
        info!("✅ Universal Capability Adapter shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_adapter_creation() {
        let adapter = UniversalCapabilityAdapter::new().await;
        assert!(adapter.is_ok());
    }

    #[tokio::test]
    async fn test_adapter_with_config() {
        let config = AdapterConfig::default();
        let adapter = UniversalCapabilityAdapter::with_config(config).await;
        assert!(adapter.is_ok());
    }

    #[test]
    fn test_selection_criteria_default() {
        let criteria = SelectionCriteria::default();
        assert_eq!(criteria.performance_weight, 0.3);
        assert_eq!(criteria.availability_weight, 0.3);
        assert_eq!(criteria.security_weight, 0.2);
    }
} 