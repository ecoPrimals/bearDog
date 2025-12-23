//! # Core Capability Adapter
//!
//! Core structure and initialization logic for the Universal Capability Adapter.

use super::types::*;
use crate::ecosystem::primal_types::{DiscoveredPrimal, PrimalMetrics, UniversalEndpoint};
use crate::universal::types::{AdapterConfig, AdapterMetrics, CapabilityConnection};
use beardog_errors::BearDogError;
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
    pub async fn new() -> Result<Self> {
        Self::with_config(AdapterConfig::default()).await
    }

    /// Create universal adapter with custom configuration
    pub async fn with_config(config: AdapterConfig) -> Result<Self> {
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

    /// Get reference to capabilities (cheap - just Arc clone)
    /// 
    /// **Performance**: This is a cheap operation (Arc reference count increment only).
    /// Use this for read-only access to capabilities.
    pub fn capabilities_ref(&self) -> Arc<RwLock<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>>> {
        Arc::clone(&self.capabilities)
    }

    /// Get reference to primals (cheap - just Arc clone)
    /// 
    /// **Performance**: This is a cheap operation (Arc reference count increment only).
    /// Use this for read-only access to discovered primals.
    pub fn primals_ref(&self) -> Arc<RwLock<HashMap<String, DiscoveredPrimal>>> {
        Arc::clone(&self.primals)
    }

    /// Get available capabilities snapshot
    /// 
    /// **Performance Note**: This clones the entire HashMap. For read-only access,
    /// prefer `capabilities_ref()` which is much cheaper.
    pub async fn get_available_capabilities(&self) -> Result<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>> {
        let capabilities = self.capabilities.read().await;
        Ok(capabilities.clone())
    }

    /// Get discovered primals snapshot
    /// 
    /// **Performance Note**: This clones the entire HashMap. For read-only access,
    /// prefer `primals_ref()` which is much cheaper.
    pub async fn get_discovered_primals(&self) -> Result<HashMap<String, DiscoveredPrimal>> {
        let primals = self.primals.read().await;
        Ok(primals.clone())
    }

    /// Get adapter metrics
    pub fn get_metrics(&self) -> &AdapterMetrics {
        &self.metrics
    }

    /// Health check all active connections
    pub async fn health_check_all_connections(&self) -> Result<HashMap<String, bool>> {
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
    async fn check_connection_health(&self, connection: &CapabilityConnection) -> Result<bool> {
        // Implementation would depend on the connection type
        // For now, return true as a placeholder
        Ok(true)
    }

    /// Get configuration
    pub fn config(&self) -> &AdapterConfig {
        &self.config
    }

    /// Update configuration
    pub async fn update_config(&mut self, new_config: AdapterConfig) -> Result<()> {
        info!("🔄 Updating Universal Capability Adapter configuration");
        self.config = new_config;
        info!("✅ Configuration updated successfully");
        Ok(())
    }

    /// Shutdown the adapter and clean up resources
    pub async fn shutdown(&self) -> Result<()> {
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

    // ========================================================================
    // Week 2 Test Expansion - October 25, 2025
    // Comprehensive Adapter Tests
    // ========================================================================

    #[tokio::test]
    async fn test_adapter_config_defaults() {
        let adapter = UniversalCapabilityAdapter::new().await?;
        let config = adapter.config();
        
        assert!(config.max_providers_per_capability > 0, "Should have positive max providers");
        assert!(config.health_check_interval_secs > 0, "Should have positive health check interval");
        assert!(config.connection_timeout_ms > 0, "Should have positive timeout");
    }

    #[tokio::test]
    async fn test_get_capabilities_empty_initial() {
        let adapter = UniversalCapabilityAdapter::new().await?;
        let capabilities = adapter.get_available_capabilities().await;
        
        assert!(capabilities.is_ok(), "Should list capabilities");
        assert!(capabilities?.is_empty(), "Should start with empty capabilities");
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_get_primals_empty_initial() {
        let adapter = UniversalCapabilityAdapter::new().await?;
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let primals = adapter.get_discovered_primals().await;
        
        assert!(primals.is_ok(), "Should get primals list");
        assert!(primals?.is_empty(), "Should start with no primals");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    }

    #[tokio::test]
    async fn test_adapter_metrics_access() {
        let adapter = UniversalCapabilityAdapter::new().await?;
        let metrics = adapter.get_metrics();
        
        assert!(format!("{:?}", metrics).len() > 0, "Should have metrics");
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_health_check_empty_connections() {
        let adapter = UniversalCapabilityAdapter::new().await?;
        let health = adapter.health_check_all_connections().await;
        
        assert!(health.is_ok(), "Should check health");
        assert!(health?.is_empty(), "Should have no connections initially");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    }

    #[tokio::test]
    async fn test_adapter_shutdown_graceful() {
        let adapter = UniversalCapabilityAdapter::new().await?;
        let result = adapter.shutdown().await;
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: adapters
         // TEST_PRIORITY: normal
        
        assert!(result.is_ok(), "Should shutdown gracefully");
    }

    #[tokio::test]
    async fn test_adapter_config_access() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let adapter = UniversalCapabilityAdapter::new().await?;
        let config = adapter.config();
        
        assert!(config.max_providers_per_capability > 0);
    }
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: adapters
 // TEST_PRIORITY: normal

    #[tokio::test]
    async fn test_adapter_full_lifecycle() {
        let adapter = UniversalCapabilityAdapter::new().await?;
        
        let capabilities = adapter.get_available_capabilities().await?;
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        assert!(capabilities.is_empty());
        
        let shutdown = adapter.shutdown().await;
        assert!(shutdown.is_ok());
    }
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: adapters
 // TEST_PRIORITY: normal

    #[tokio::test]
    async fn test_multiple_adapters_coexist() {
        let adapter1 = UniversalCapabilityAdapter::new().await;
        let adapter2 = UniversalCapabilityAdapter::new().await;
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: adapters
         // TEST_PRIORITY: normal
        
        assert!(adapter1.is_ok() && adapter2.is_ok(), "Should create multiple adapters");
    }

    #[tokio::test]
    async fn test_adapter_with_custom_max_providers() {
        let config = AdapterConfig {
            max_providers_per_capability: 10,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: adapters
            // TEST_PRIORITY: normal
            ..Default::default()
        };
        
        let adapter = UniversalCapabilityAdapter::with_config(config).await?;
        assert_eq!(adapter.config().max_providers_per_capability, 10);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    }

    #[tokio::test]
    async fn test_adapter_with_custom_health_interval() {
        let config = AdapterConfig {
            health_check_interval_secs: 60,
            ..Default::default()
        };
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: adapters
         // TEST_PRIORITY: normal
        
        let adapter = UniversalCapabilityAdapter::with_config(config).await?;
        assert_eq!(adapter.config().health_check_interval_secs, 60);
    }

    #[tokio::test]
    async fn test_adapter_with_custom_timeout() {
        let config = AdapterConfig {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: adapters
            // TEST_PRIORITY: normal
            connection_timeout_ms: 5000,
            ..Default::default()
        };
        
        let adapter = UniversalCapabilityAdapter::with_config(config).await?;
        assert_eq!(adapter.config().connection_timeout_ms, 5000);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_adapter_config_persistence() {
        let config = AdapterConfig {
            max_providers_per_capability: 42,
            health_check_interval_secs: 99,
            connection_timeout_ms: 1234,
            ..Default::default()
        };
        
        let adapter = UniversalCapabilityAdapter::with_config(config).await?;
        let stored = adapter.config();
        
        assert_eq!(stored.max_providers_per_capability, 42);
        assert_eq!(stored.health_check_interval_secs, 99);
        assert_eq!(stored.connection_timeout_ms, 1234);
    }
} 