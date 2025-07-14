//! Universal NestGate Integration Adapter
//!
//! Universal NestGate adapter providing secure file transfer integration with ZFS key management,
//! secure file operations, audit trail integration, and policy enforcement for any ecosystem component.
//!
//! This adapter has been refactored to use universal patterns instead of being BearDog-specific,
//! allowing any primal (BearDog, SongBird, ToadStool, biomeOS, etc.) to integrate with NestGate.

use std::sync::Arc;
use tracing::info;

pub mod audit;
pub mod core;
pub mod policy;
pub mod types;
pub mod zfs;

// Re-export main types and structs
pub use audit::{AuditManager, AuditReport, AuditReportType, AuditStatistics};
pub use core::{Connection, ConnectionPool, ConnectionStatus, UniversalNestGateAdapter};
pub use policy::{PolicyEngine, PolicyStatistics};
pub use types::*;
pub use zfs::{ZfsDataset, ZfsManager, ZfsOperation};

/// Universal NestGate adapter builder
pub struct NestGateAdapterBuilder {
    /// Provider instance
    provider: Option<Arc<dyn PrimalProvider>>,
    /// Configuration
    config: Option<NestGateConfig>,
    /// Custom components
    custom_components: std::collections::HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}

impl NestGateAdapterBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self {
            provider: None,
            config: None,
            custom_components: std::collections::HashMap::new(),
        }
    }

    /// Set primal provider
    pub fn with_provider(mut self, provider: Arc<dyn PrimalProvider>) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Set configuration
    pub fn with_config(mut self, config: NestGateConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Add custom component
    pub fn with_custom_component<T: std::any::Any + Send + Sync>(
        mut self,
        name: String,
        component: T,
    ) -> Self {
        self.custom_components.insert(name, Box::new(component));
        self
    }

    /// Build the adapter
    pub async fn build(self) -> NestGateResult<UniversalNestGateAdapter> {
        let provider = self.provider.ok_or_else(|| {
            NestGateError::Configuration("Provider is required".to_string())
        })?;

        let config = self.config.unwrap_or_else(|| {
            let mut default_config = NestGateConfig::default();
            default_config.provider_name = provider.name().to_string();
            default_config
        });

        info!(
            "Building universal NestGate adapter for provider: {}",
            provider.name()
        );

        UniversalNestGateAdapter::new(provider, config).await
    }
}

impl Default for NestGateAdapterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Universal NestGate adapter factory
pub struct NestGateAdapterFactory;

impl NestGateAdapterFactory {
    /// Create adapter for BearDog
    pub async fn create_for_beardog(
        beardog_provider: Arc<dyn PrimalProvider>,
        config: Option<NestGateConfig>,
    ) -> NestGateResult<UniversalNestGateAdapter> {
        info!("Creating NestGate adapter for BearDog");

        let config = config.unwrap_or_else(|| {
            let mut config = NestGateConfig::default();
            config.provider_name = "beardog".to_string();
            config.capabilities = vec![
                "file_operations".to_string(),
                "key_management".to_string(),
                "zfs_integration".to_string(),
                "policy_enforcement".to_string(),
                "audit_logging".to_string(),
                "beardog_specific".to_string(),
            ];
            config
        });

        NestGateAdapterBuilder::new()
            .with_provider(beardog_provider)
            .with_config(config)
            .build()
            .await
    }

    /// Create adapter for SongBird
    pub async fn create_for_songbird(
        songbird_provider: Arc<dyn PrimalProvider>,
        config: Option<NestGateConfig>,
    ) -> NestGateResult<UniversalNestGateAdapter> {
        info!("Creating NestGate adapter for SongBird");

        let config = config.unwrap_or_else(|| {
            let mut config = NestGateConfig::default();
            config.provider_name = "songbird".to_string();
            config.capabilities = vec![
                "file_operations".to_string(),
                "key_management".to_string(),
                "zfs_integration".to_string(),
                "policy_enforcement".to_string(),
                "audit_logging".to_string(),
                "service_orchestration".to_string(),
                "universal_patterns".to_string(),
            ];
            config
        });

        NestGateAdapterBuilder::new()
            .with_provider(songbird_provider)
            .with_config(config)
            .build()
            .await
    }

    /// Create adapter for ToadStool
    pub async fn create_for_toadstool(
        toadstool_provider: Arc<dyn PrimalProvider>,
        config: Option<NestGateConfig>,
    ) -> NestGateResult<UniversalNestGateAdapter> {
        info!("Creating NestGate adapter for ToadStool");

        let config = config.unwrap_or_else(|| {
            let mut config = NestGateConfig::default();
            config.provider_name = "toadstool".to_string();
            config.capabilities = vec![
                "file_operations".to_string(),
                "key_management".to_string(),
                "zfs_integration".to_string(),
                "policy_enforcement".to_string(),
                "audit_logging".to_string(),
                "compute_integration".to_string(),
                "performance_optimization".to_string(),
            ];
            config
        });

        NestGateAdapterBuilder::new()
            .with_provider(toadstool_provider)
            .with_config(config)
            .build()
            .await
    }

    /// Create adapter for biomeOS
    pub async fn create_for_biomeos(
        biomeos_provider: Arc<dyn PrimalProvider>,
        config: Option<NestGateConfig>,
    ) -> NestGateResult<UniversalNestGateAdapter> {
        info!("Creating NestGate adapter for biomeOS");

        let config = config.unwrap_or_else(|| {
            let mut config = NestGateConfig::default();
            config.provider_name = "biomeos".to_string();
            config.capabilities = vec![
                "file_operations".to_string(),
                "key_management".to_string(),
                "zfs_integration".to_string(),
                "policy_enforcement".to_string(),
                "audit_logging".to_string(),
                "ecosystem_integration".to_string(),
                "biometric_auth".to_string(),
            ];
            config
        });

        NestGateAdapterBuilder::new()
            .with_provider(biomeos_provider)
            .with_config(config)
            .build()
            .await
    }

    /// Create generic adapter for any primal
    pub async fn create_for_primal(
        primal_name: &str,
        primal_provider: Arc<dyn PrimalProvider>,
        config: Option<NestGateConfig>,
    ) -> NestGateResult<UniversalNestGateAdapter> {
        info!("Creating NestGate adapter for primal: {}", primal_name);

        let config = config.unwrap_or_else(|| {
            let mut config = NestGateConfig::default();
            config.provider_name = primal_name.to_string();
            config.capabilities = vec![
                "file_operations".to_string(),
                "key_management".to_string(),
                "zfs_integration".to_string(),
                "policy_enforcement".to_string(),
                "audit_logging".to_string(),
                "universal_patterns".to_string(),
            ];
            config
        });

        NestGateAdapterBuilder::new()
            .with_provider(primal_provider)
            .with_config(config)
            .build()
            .await
    }
}

/// Universal NestGate service registry
pub struct NestGateServiceRegistry {
    /// Registered adapters
    adapters: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, Arc<UniversalNestGateAdapter>>>>,
}

impl NestGateServiceRegistry {
    /// Create new service registry
    pub fn new() -> Self {
        Self {
            adapters: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Register adapter
    pub async fn register_adapter(
        &self,
        name: String,
        adapter: Arc<UniversalNestGateAdapter>,
    ) -> NestGateResult<()> {
        info!("Registering NestGate adapter: {}", name);
        self.adapters.write().await.insert(name, adapter);
        Ok(())
    }

    /// Get adapter
    pub async fn get_adapter(&self, name: &str) -> NestGateResult<Option<Arc<UniversalNestGateAdapter>>> {
        let adapters = self.adapters.read().await;
        Ok(adapters.get(name).cloned())
    }

    /// List all adapters
    pub async fn list_adapters(&self) -> NestGateResult<Vec<String>> {
        let adapters = self.adapters.read().await;
        Ok(adapters.keys().cloned().collect())
    }

    /// Remove adapter
    pub async fn remove_adapter(&self, name: &str) -> NestGateResult<bool> {
        info!("Removing NestGate adapter: {}", name);
        Ok(self.adapters.write().await.remove(name).is_some())
    }

    /// Health check all adapters
    pub async fn health_check_all(&self) -> NestGateResult<std::collections::HashMap<String, HealthStatus>> {
        let adapters = self.adapters.read().await;
        let mut health_results = std::collections::HashMap::new();

        for (name, adapter) in adapters.iter() {
            match adapter.health_check().await {
                Ok(health) => {
                    health_results.insert(name.clone(), health);
                }
                Err(e) => {
                    health_results.insert(name.clone(), HealthStatus {
                        healthy: false,
                        message: format!("Health check failed: {}", e),
                        components: std::collections::HashMap::new(),
                        last_check: chrono::Utc::now(),
                    });
                }
            }
        }

        Ok(health_results)
    }
}

impl Default for NestGateServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for NestGate integration
pub mod utils {
    use super::*;

    /// Validate NestGate configuration
    pub fn validate_config(config: &NestGateConfig) -> NestGateResult<()> {
        if config.api_endpoint.is_empty() {
            return Err(NestGateError::Configuration(
                "API endpoint cannot be empty".to_string(),
            ));
        }

        if config.provider_name.is_empty() {
            return Err(NestGateError::Configuration(
                "Provider name cannot be empty".to_string(),
            ));
        }

        if config.auth.api_key.is_empty() && config.auth.method == AuthMethod::ApiKey {
            return Err(NestGateError::Configuration(
                "API key cannot be empty for API key authentication".to_string(),
            ));
        }

        Ok(())
    }

    /// Create default configuration for primal
    pub fn create_default_config(primal_name: &str) -> NestGateConfig {
        let mut config = NestGateConfig::default();
        config.provider_name = primal_name.to_string();
        
        // Set primal-specific capabilities
        config.capabilities = match primal_name {
            "beardog" => vec![
                "file_operations".to_string(),
                "key_management".to_string(),
                "zfs_integration".to_string(),
                "policy_enforcement".to_string(),
                "audit_logging".to_string(),
                "beardog_specific".to_string(),
            ],
            "songbird" => vec![
                "file_operations".to_string(),
                "key_management".to_string(),
                "zfs_integration".to_string(),
                "policy_enforcement".to_string(),
                "audit_logging".to_string(),
                "service_orchestration".to_string(),
                "universal_patterns".to_string(),
            ],
            "toadstool" => vec![
                "file_operations".to_string(),
                "key_management".to_string(),
                "zfs_integration".to_string(),
                "policy_enforcement".to_string(),
                "audit_logging".to_string(),
                "compute_integration".to_string(),
                "performance_optimization".to_string(),
            ],
            "biomeos" => vec![
                "file_operations".to_string(),
                "key_management".to_string(),
                "zfs_integration".to_string(),
                "policy_enforcement".to_string(),
                "audit_logging".to_string(),
                "ecosystem_integration".to_string(),
                "biometric_auth".to_string(),
            ],
            _ => vec![
                "file_operations".to_string(),
                "key_management".to_string(),
                "zfs_integration".to_string(),
                "policy_enforcement".to_string(),
                "audit_logging".to_string(),
                "universal_patterns".to_string(),
            ],
        };
        
        config
    }

    /// Get supported capabilities for primal
    pub fn get_supported_capabilities(primal_name: &str) -> Vec<String> {
        create_default_config(primal_name).capabilities
    }

    /// Check if primal supports capability
    pub fn supports_capability(primal_name: &str, capability: &str) -> bool {
        get_supported_capabilities(primal_name).contains(&capability.to_string())
    }
}

/// Re-export core functionality for easy access
pub use core::UniversalNestGateAdapter as NestGateAdapter;
pub use types::{NestGateConfig, NestGateError, NestGateResult};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const UNIVERSAL_ADAPTER_VERSION: &str = "1.0.0";

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Clone)]
    struct MockPrimalProvider {
        name: String,
        capabilities: Vec<String>,
    }

    impl MockPrimalProvider {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                capabilities: vec![
                    "file_operations".to_string(),
                    "key_management".to_string(),
                ],
            }
        }
    }

    impl PrimalProvider for MockPrimalProvider {
        fn name(&self) -> &str {
            &self.name
        }

        fn capabilities(&self) -> Vec<String> {
            self.capabilities.clone()
        }

        async fn health_check(&self) -> NestGateResult<HealthStatus> {
            Ok(HealthStatus {
                healthy: true,
                message: "Mock provider healthy".to_string(),
                components: HashMap::new(),
                last_check: chrono::Utc::now(),
            })
        }

        fn config(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[tokio::test]
    async fn test_adapter_builder() {
        let provider = Arc::new(MockPrimalProvider::new("test"));
        let config = NestGateConfig::default();

        let adapter = NestGateAdapterBuilder::new()
            .with_provider(provider)
            .with_config(config)
            .build()
            .await;

        assert!(adapter.is_ok());
    }

    #[tokio::test]
    async fn test_adapter_factory() {
        let provider = Arc::new(MockPrimalProvider::new("beardog"));
        
        let adapter = NestGateAdapterFactory::create_for_beardog(provider, None).await;
        assert!(adapter.is_ok());
    }

    #[tokio::test]
    async fn test_service_registry() {
        let registry = NestGateServiceRegistry::new();
        let provider = Arc::new(MockPrimalProvider::new("test"));
        
        let adapter = NestGateAdapterBuilder::new()
            .with_provider(provider)
            .build()
            .await
            .unwrap();

        let result = registry.register_adapter("test".to_string(), Arc::new(adapter)).await;
        assert!(result.is_ok());

        let retrieved = registry.get_adapter("test").await.unwrap();
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_utils() {
        let config = utils::create_default_config("beardog");
        assert_eq!(config.provider_name, "beardog");
        assert!(config.capabilities.contains(&"beardog_specific".to_string()));

        assert!(utils::supports_capability("beardog", "file_operations"));
        assert!(!utils::supports_capability("beardog", "non_existent_capability"));
    }
} 