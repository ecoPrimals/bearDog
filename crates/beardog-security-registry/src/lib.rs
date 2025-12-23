//! Security Registry Module for `BearDog`
//!
//! This module provides functionality for managing security relationships,
//! trust levels, and registry operations within the `BearDog` ecosystem.

// Note: Production code lints are enforced at workspace level
// Test code is allowed to use unwrap/expect via test module attributes

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

mod security;
mod trust;

pub use trust::{TrustConfig, TrustLevel, TrustStore};

/// Security registry configuration
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct SecurityRegistryConfig {
    /// Unique identifier for this registry instance
    pub registry_id: String,
    /// Public API endpoint
    pub public_endpoint: String,
    /// Orchestration endpoint
    pub orchestration_endpoint: String,
    /// Trust configuration
    pub trust_config: TrustConfig,
}

impl Default for SecurityRegistryConfig {
    #[inline]
    fn default() -> Self {
        use beardog_types::constants::domains::network::defaults::default_api_port;

        Self {
            registry_id: format!("beardog-{}", uuid::Uuid::new_v4()),
            public_endpoint: env::var("BEARDOG_PUBLIC_ENDPOINT")
                .unwrap_or_else(|_| format!("https://localhost:{}", default_api_port())),
            orchestration_endpoint: env::var("ORCHESTRATION_ENDPOINT")
                .unwrap_or_else(|_| format!("http://localhost:{}/security", default_api_port())),
            trust_config: TrustConfig::default(),
        }
    }
}

/// Security registry for managing trust relationships
pub struct SecurityRegistry {
    config: SecurityRegistryConfig,
    trust_relationships: Arc<RwLock<TrustStore>>,
}

impl SecurityRegistry {
    /// Creates a new security registry instance
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn new(config: SecurityRegistryConfig) -> Result<Self, BearDogError> {
        info!(
            "\u{1f510} Initializing BearDog Security Registry: {}",
            config.registry_id
        );

        let trust_store = TrustStore::new();

        Ok(Self {
            config,
            trust_relationships: Arc::new(RwLock::new(trust_store)),
        })
    }

    /// Establishes trust with a node
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub async fn establish_trust(
        &self,
        node_id: &str,
        trust_level: TrustLevel,
    ) -> Result<(), BearDogError> {
        info!(
            "\u{1f91d} Establishing trust with BearDog instance: {} (level: {:?})",
            node_id, trust_level
        );

        let mut trust_store = self.trust_relationships.write().await;
        trust_store.set_trust_level(node_id.to_string(), trust_level);

        Ok(())
    }

    /// Gets trust level for a node
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub async fn get_trust_level(&self, node_id: &str) -> Result<Option<TrustLevel>, BearDogError> {
        let trust_store = self.trust_relationships.read().await;
        Ok(trust_store.get_trust_level(node_id).cloned())
    }

    /// Verifies trust for a node
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub async fn verify_trust(&self, node_id: &str) -> Result<Option<TrustLevel>, BearDogError> {
        self.get_trust_level(node_id).await
    }

    /// Gets health status of the registry
    pub async fn health(&self) -> SecurityRegistryHealth {
        let trust_store = self.trust_relationships.read().await;
        SecurityRegistryHealth {
            instance_id: self.config.registry_id.clone(),
            last_check: Utc::now(),
            operational: true,
            trust_relationships_count: trust_store.count(),
        }
    }
}

/// Health status of the security registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRegistryHealth {
    /// Registry instance ID
    pub instance_id: String,
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    /// Whether the registry is operational
    pub operational: bool,
    /// Number of trust relationships
    pub trust_relationships_count: usize,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
#[allow(clippy::field_reassign_with_default, clippy::default_trait_access)]
#[allow(clippy::useless_vec, clippy::used_underscore_binding)]
mod tests {
    use super::*;
    use trust::{TrustConfig, TrustManager, TrustPropagation, TrustStore, TrustVerifier};

    #[test]
    fn security_registry_module_loads() {
        let _health = SecurityRegistryHealth {
            instance_id: "test_node".to_owned(),
            last_check: Utc::now(),
            operational: true,
            trust_relationships_count: 0,
        };
    }

    #[tokio::test]
    async fn registry_initialization() -> Result<(), BearDogError> {
        let config = SecurityRegistryConfig::default();
        let _registry = SecurityRegistry::new(config)?;
        println!("✅ Security registry initialization test passed");
        Ok(())
    }

    #[tokio::test]
    async fn registry_node_registration() -> Result<(), BearDogError> {
        let config = SecurityRegistryConfig::default();
        let registry = SecurityRegistry::new(config)?;

        registry
            .establish_trust("test-node-1", TrustLevel::Basic)
            .await?;
        let trust_level = registry.get_trust_level("test-node-1").await?;

        assert_eq!(trust_level, Some(TrustLevel::Basic));
        println!("\u{2705} Security registry node registration test passed");
        Ok(())
    }

    #[tokio::test]
    async fn registry_security_validation() -> Result<(), BearDogError> {
        let config = SecurityRegistryConfig::default();
        let registry = SecurityRegistry::new(config)?;

        let health = registry.health().await;
        assert!(health.operational);
        assert_eq!(health.trust_relationships_count, 0);

        println!("\u{2705} Security registry validation test passed");
        Ok(())
    }

    // === Trust Config Tests ===

    #[test]
    fn test_trust_config_default() {
        let config = TrustConfig::default();
        assert_eq!(config.verification_timeout_secs, 30);
        assert!(config.enable_propagation);
    }

    #[test]
    fn test_trust_config_custom() {
        let config = TrustConfig {
            verification_timeout_secs: 60,
            enable_propagation: false,
        };
        assert_eq!(config.verification_timeout_secs, 60);
        assert!(!config.enable_propagation);
    }

    // === Trust Level Tests ===

    #[test]
    fn test_trust_level_variants() {
        assert_ne!(TrustLevel::Untrusted, TrustLevel::Basic);
        assert_ne!(TrustLevel::Basic, TrustLevel::Verified);
        assert_ne!(TrustLevel::Verified, TrustLevel::Full);
    }

    #[test]
    fn test_trust_level_clone() {
        let level = TrustLevel::Verified;
        let cloned = level.clone();
        assert_eq!(level, cloned);
    }

    #[test]
    fn test_trust_level_serialization() {
        let level = TrustLevel::Full;
        let serialized = serde_json::to_string(&level).expect("serialize");
        let deserialized: TrustLevel = serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(level, deserialized);
    }

    // === Trust Store Tests ===

    #[test]
    fn test_trust_store_new() {
        let store = TrustStore::new();
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_trust_store_default() {
        let store = TrustStore::default();
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_trust_store_set_and_get() {
        let mut store = TrustStore::new();
        store.set_trust_level("entity1".to_string(), TrustLevel::Basic);
        assert_eq!(store.get_trust_level("entity1"), Some(&TrustLevel::Basic));
        assert_eq!(store.count(), 1);
    }

    #[test]
    fn test_trust_store_overwrite() {
        let mut store = TrustStore::new();
        store.set_trust_level("entity1".to_string(), TrustLevel::Basic);
        store.set_trust_level("entity1".to_string(), TrustLevel::Verified);
        assert_eq!(
            store.get_trust_level("entity1"),
            Some(&TrustLevel::Verified)
        );
        assert_eq!(store.count(), 1);
    }

    #[test]
    fn test_trust_store_multiple_entities() {
        let mut store = TrustStore::new();
        store.set_trust_level("entity1".to_string(), TrustLevel::Basic);
        store.set_trust_level("entity2".to_string(), TrustLevel::Verified);
        store.set_trust_level("entity3".to_string(), TrustLevel::Full);
        assert_eq!(store.count(), 3);
        assert_eq!(store.get_trust_level("entity1"), Some(&TrustLevel::Basic));
        assert_eq!(
            store.get_trust_level("entity2"),
            Some(&TrustLevel::Verified)
        );
        assert_eq!(store.get_trust_level("entity3"), Some(&TrustLevel::Full));
    }

    #[test]
    fn test_trust_store_nonexistent_entity() {
        let store = TrustStore::new();
        assert_eq!(store.get_trust_level("nonexistent"), None);
    }

    // === Trust Verifier Tests ===

    #[tokio::test]
    async fn test_trust_verifier_creation() {
        let config = TrustConfig::default();
        let verifier = TrustVerifier::new(config);
        // Verifier should be created successfully
        assert!(format!("{:?}", verifier).contains("TrustVerifier"));
    }

    #[tokio::test]
    async fn test_trust_verifier_verify() {
        let config = TrustConfig::default();
        let verifier = TrustVerifier::new(config);
        let result = verifier.verify("test_entity").await;
        assert!(result.is_ok());
        // Default implementation returns Basic
        assert_eq!(result.expect("should verify"), TrustLevel::Basic);
    }

    // === Trust Propagation Tests ===

    #[tokio::test]
    async fn test_trust_propagation_creation() {
        let config = TrustConfig::default();
        let propagation = TrustPropagation::new(config);
        assert!(format!("{:?}", propagation).contains("TrustPropagation"));
    }

    #[tokio::test]
    async fn test_trust_propagation_enabled() {
        let config = TrustConfig {
            verification_timeout_secs: 30,
            enable_propagation: true,
        };
        let propagation = TrustPropagation::new(config);
        let result = propagation
            .propagate("test_entity", &TrustLevel::Verified)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_trust_propagation_disabled() {
        let config = TrustConfig {
            verification_timeout_secs: 30,
            enable_propagation: false,
        };
        let propagation = TrustPropagation::new(config);
        let result = propagation
            .propagate("test_entity", &TrustLevel::Full)
            .await;
        assert!(result.is_ok());
    }

    // === Trust Manager Tests ===

    #[tokio::test]
    async fn test_trust_manager_creation() {
        let config = TrustConfig::default();
        let manager = TrustManager::new(config);
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_trust_manager_verify_new_entity() {
        let config = TrustConfig::default();
        let mut manager = TrustManager::new(config).expect("should create manager");
        let result = manager.verify_trust("new_entity").await;
        assert!(result.is_ok());
        // Should get Basic trust level from verifier
        assert_eq!(result.expect("should verify"), TrustLevel::Basic);
    }

    #[tokio::test]
    async fn test_trust_manager_verify_cached_entity() {
        let config = TrustConfig::default();
        let mut manager = TrustManager::new(config).expect("should create manager");

        // First verification
        let result1 = manager.verify_trust("cached_entity").await;
        assert!(result1.is_ok());

        // Second verification should return cached value
        let result2 = manager.verify_trust("cached_entity").await;
        assert!(result2.is_ok());
        assert_eq!(result1.expect("first"), result2.expect("second"));
    }

    #[tokio::test]
    async fn test_trust_manager_propagate_enabled() {
        let config = TrustConfig {
            verification_timeout_secs: 30,
            enable_propagation: true,
        };
        let mut manager = TrustManager::new(config).expect("should create manager");
        let result = manager.propagate_trust("entity", TrustLevel::Full).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_trust_manager_propagate_disabled() {
        let config = TrustConfig {
            verification_timeout_secs: 30,
            enable_propagation: false,
        };
        let mut manager = TrustManager::new(config).expect("should create manager");
        let result = manager
            .propagate_trust("entity", TrustLevel::Verified)
            .await;
        assert!(result.is_ok());
    }

    // === Security Registry Config Tests ===

    #[test]
    fn test_security_registry_config_default() {
        let config = SecurityRegistryConfig::default();
        assert!(!config.registry_id.is_empty());
        assert!(config.registry_id.starts_with("beardog-"));
        assert!(!config.public_endpoint.is_empty());
        assert!(!config.orchestration_endpoint.is_empty());
    }

    #[test]
    fn test_security_registry_config_clone() {
        let config = SecurityRegistryConfig::default();
        let cloned = config.clone();
        assert_eq!(config.registry_id, cloned.registry_id);
    }

    // === Security Registry Tests ===

    #[tokio::test]
    async fn test_registry_multiple_trust_levels() -> Result<(), BearDogError> {
        let config = SecurityRegistryConfig::default();
        let registry = SecurityRegistry::new(config)?;

        registry
            .establish_trust("node-untrusted", TrustLevel::Untrusted)
            .await?;
        registry
            .establish_trust("node-basic", TrustLevel::Basic)
            .await?;
        registry
            .establish_trust("node-verified", TrustLevel::Verified)
            .await?;
        registry
            .establish_trust("node-full", TrustLevel::Full)
            .await?;

        assert_eq!(
            registry.get_trust_level("node-untrusted").await?,
            Some(TrustLevel::Untrusted)
        );
        assert_eq!(
            registry.get_trust_level("node-basic").await?,
            Some(TrustLevel::Basic)
        );
        assert_eq!(
            registry.get_trust_level("node-verified").await?,
            Some(TrustLevel::Verified)
        );
        assert_eq!(
            registry.get_trust_level("node-full").await?,
            Some(TrustLevel::Full)
        );

        let health = registry.health().await;
        assert_eq!(health.trust_relationships_count, 4);

        Ok(())
    }

    #[tokio::test]
    async fn test_registry_update_trust_level() -> Result<(), BearDogError> {
        let config = SecurityRegistryConfig::default();
        let registry = SecurityRegistry::new(config)?;

        registry
            .establish_trust("upgrading-node", TrustLevel::Basic)
            .await?;
        assert_eq!(
            registry.get_trust_level("upgrading-node").await?,
            Some(TrustLevel::Basic)
        );

        registry
            .establish_trust("upgrading-node", TrustLevel::Full)
            .await?;
        assert_eq!(
            registry.get_trust_level("upgrading-node").await?,
            Some(TrustLevel::Full)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_registry_verify_trust() -> Result<(), BearDogError> {
        let config = SecurityRegistryConfig::default();
        let registry = SecurityRegistry::new(config)?;

        // Nonexistent node
        assert_eq!(registry.verify_trust("unknown-node").await?, None);

        // Existing node
        registry
            .establish_trust("known-node", TrustLevel::Verified)
            .await?;
        assert_eq!(
            registry.verify_trust("known-node").await?,
            Some(TrustLevel::Verified)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_registry_health_with_relationships() -> Result<(), BearDogError> {
        let config = SecurityRegistryConfig::default();
        let registry = SecurityRegistry::new(config)?;

        let health_empty = registry.health().await;
        assert!(health_empty.operational);
        assert_eq!(health_empty.trust_relationships_count, 0);

        registry
            .establish_trust("node-1", TrustLevel::Basic)
            .await?;
        registry.establish_trust("node-2", TrustLevel::Full).await?;

        let health_populated = registry.health().await;
        assert!(health_populated.operational);
        assert_eq!(health_populated.trust_relationships_count, 2);

        Ok(())
    }

    // === SecurityRegistryHealth Tests ===

    #[test]
    fn test_security_registry_health_clone() {
        let health = SecurityRegistryHealth {
            instance_id: "test-instance".to_owned(),
            last_check: Utc::now(),
            operational: true,
            trust_relationships_count: 5,
        };
        let cloned = health.clone();
        assert_eq!(health.instance_id, cloned.instance_id);
        assert_eq!(health.operational, cloned.operational);
        assert_eq!(
            health.trust_relationships_count,
            cloned.trust_relationships_count
        );
    }

    #[test]
    fn test_security_registry_health_serialization() {
        let health = SecurityRegistryHealth {
            instance_id: "test-instance".to_owned(),
            last_check: Utc::now(),
            operational: true,
            trust_relationships_count: 10,
        };
        let serialized = serde_json::to_string(&health).expect("serialize");
        assert!(serialized.contains("test-instance"));
        assert!(serialized.contains("10"));

        let deserialized: SecurityRegistryHealth =
            serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(health.instance_id, deserialized.instance_id);
        assert_eq!(health.operational, deserialized.operational);
    }
}
