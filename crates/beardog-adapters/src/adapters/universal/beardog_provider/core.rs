//! Core BearDog PrimalProvider implementation
//!
//! This module contains the main BearDogPrimalProvider struct and its constructor.

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use super::super::songbird_handoff::health::UniversalHealthMonitor;
use super::super::songbird_handoff::registration::SongBirdRegistrationManager;

use beardog_config::BearDogConfig;

/// Metadata for the BearDog provider
#[derive(Debug, Clone)]
pub struct ProviderMetadata {
    /// Provider name
    pub name: String,
    /// Provider version
    pub version: String,
    /// Provider description
    pub description: String,
    /// Provider capabilities
    pub capabilities: Vec<String>,
}

/// BearDog PrimalProvider core structure
pub struct BearDogPrimalProvider<T: Send + Sync> {
    /// Core instance
    pub(crate) core: Arc<RwLock<Option<T>>>,

    /// Instance ID
    pub(crate) instance_id: String,

    /// Provider metadata
    pub(crate) metadata: ProviderMetadata,

    /// Configuration
    pub(crate) config: Arc<BearDogConfig>,

    /// Registration manager for SongBird integration
    pub(crate) registration_manager: Arc<SongBirdRegistrationManager<T>>,

    /// Health monitor
    pub(crate) health_monitor: Arc<UniversalHealthMonitor>,

    /// Background tasks
    pub(crate) background_tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
}

impl<T: Send + Sync> BearDogPrimalProvider<T> {
    /// Create a new BearDog PrimalProvider
    pub fn new(
        core: Arc<RwLock<Option<T>>>,
        instance_id: String,
        config: Arc<BearDogConfig>,
        registration_manager: Arc<SongBirdRegistrationManager<T>>,
        health_monitor: Arc<UniversalHealthMonitor>,
    ) -> Self {
        let _endpoints = ServiceEndpoints {
            primary: "http://localhost:8443".to_string(),
            health: "http://localhost:8443/health".to_string(),
            metrics: "http://localhost:8443/metrics".to_string(),
            admin: "http://localhost:8443/admin".to_string(),
            websocket: Some("ws://localhost:8443/ws".to_string()),
        };

        let metadata = ProviderMetadata {
            name: "BearDog Security Provider".to_string(),
            version: "1.0.0".to_string(),
            description: "Universal security provider for ecosystem integration".to_string(),
            capabilities: vec![
                "encryption".to_string(),
                "decryption".to_string(),
                "authentication".to_string(),
                "authorization".to_string(),
                "key_management".to_string(),
                "audit_logging".to_string(),
                "threat_detection".to_string(),
            ],
        };

        Self {
            core,
            instance_id,
            metadata,
            config,
            registration_manager,
            health_monitor,
            background_tasks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get the provider metadata
    pub fn metadata(&self) -> &ProviderMetadata {
        &self.metadata
    }

    /// Get the instance ID
    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }

    /// Get the configuration
    pub fn config(&self) -> &Arc<BearDogConfig> {
        &self.config
    }
}

use super::super::songbird_handoff::types::ServiceEndpoints;
