// SPDX-License-Identifier: AGPL-3.0-or-later

//! Construction, defaults, and cloning for [`super::EcosystemDiscoveryAdapter`].

use super::EcosystemDiscoveryAdapter;
#[cfg(test)]
use beardog_core::ecosystem::primal_types::DiscoveredPrimal;
use beardog_errors::BearDogError;
use beardog_types::canonical::config::domains::bootstrap::{
    BootstrapDiscoveryConfig, DiscoveryProtocol, UnifiedBootstrapConfig,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

impl EcosystemDiscoveryAdapter {
    /// Create a new discovery adapter with default configuration
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Self::with_config(Self::default_config())
    }

    /// Create adapter with custom configuration
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn with_config(config: UnifiedBootstrapConfig) -> Result<Self, BearDogError> {
        info!("🌐 Creating ecosystem discovery adapter");
        info!("📋 Using capability-based discovery (no hardcoded primals)");
        tracing::debug!(
            "   Enabled protocols: {:?}",
            config.discovery.enabled_protocols
        );

        Ok(Self {
            discovered_primals: Arc::new(RwLock::new(HashMap::new())),
            discovered_capabilities: Arc::new(RwLock::new(HashMap::new())),
            listener: Arc::new(RwLock::new(None)),
            config,
        })
    }

    /// Create default bootstrap configuration for CLI usage
    pub fn default_config() -> UnifiedBootstrapConfig {
        UnifiedBootstrapConfig {
            discovery: BootstrapDiscoveryConfig {
                enabled_protocols: vec![
                    DiscoveryProtocol::MulticastDNS,
                    DiscoveryProtocol::HttpDiscovery,
                    DiscoveryProtocol::EnvironmentDiscovery,
                ],
                ..BootstrapDiscoveryConfig::default()
            },
            ..UnifiedBootstrapConfig::default()
        }
    }

    /// Test-only: seed discovered primals without network (unit tests).
    #[cfg(test)]
    pub async fn insert_primal_for_test(&self, primal: DiscoveredPrimal) {
        self.discovered_primals
            .write()
            .await
            .insert(primal.primal_id.clone(), primal);
    }
}

impl Default for EcosystemDiscoveryAdapter {
    fn default() -> Self {
        // SAFETY: new() -> with_config() only creates HashMaps and Arc wrappers, which cannot fail.
        // If it somehow fails, we create a minimal fallback to avoid panicking.
        Self::new().unwrap_or_else(|e| {
            tracing::error!(
                "Unexpected failure creating EcosystemDiscoveryAdapter: {}. Using minimal fallback.",
                e
            );
            Self {
                discovered_primals: Arc::new(RwLock::new(HashMap::new())),
                discovered_capabilities: Arc::new(RwLock::new(HashMap::new())),
                listener: Arc::new(RwLock::new(None)),
                config: Self::default_config(),
            }
        })
    }
}

// Clone implementation for convenience (Arc-based, cheap clone)
impl Clone for EcosystemDiscoveryAdapter {
    fn clone(&self) -> Self {
        Self {
            discovered_primals: Arc::clone(&self.discovered_primals),
            discovered_capabilities: Arc::clone(&self.discovered_capabilities),
            listener: Arc::clone(&self.listener),
            config: self.config.clone(),
        }
    }
}
