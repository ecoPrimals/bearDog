//! Security Registry Module for BearDog
//!
//! This module provides functionality for managing security relationships,
//! trust levels, and registry operations within the BearDog ecosystem.

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
mod tests {
    use super::*;

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
}
