

// MODERNIZATION NOTE: This file contains primal-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use super::super::service_mesh_handoff::health::UniversalHealthMonitor;
use super::super::service_mesh_handoff::registration::ServiceMeshRegistrationManager;
use beardog_types::canonical::BearDogConfig;

#[derive(Debug, Clone)]
    /// The version value
    pub version: String,

    /// The description value
    pub description: String,

    /// Collection of capabilities
    pub capabilities: Vec<String>,
}

pub struct BearDogPrimalProvider<T: Send + Sync> {

    pub(Arc<RwLock<Option<T>>>,

    pub(String,

    pub(ProviderMetadata,

    pub(Arc<BearDogConfig>,

    pub(Arc<ServiceMeshRegistrationManager<T>>,

    pub(Arc<UniversalHealthMonitor>,

    pub(crate) background_tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
impl<T: Send + Sync> BearDogPrimalProvider<T> {

/// New operation.
    /// Creates a new instance
    pub fn new(Arc<RwLock<Option<T>>>,
        instance_id: &str,
        config: Arc<BearDogConfig>,
        registration_manager: Arc<ServiceMeshRegistrationManager<T>>,
        health_monitor: Arc<UniversalHealthMonitor>,
    ) -> Self {

        let runtime_config = beardog_types::config::get_config();
        let _endpoints = ServiceEndpoints {
            primary: runtime_config.&endpoints.external_api_base_url: base_url.to_string(),
            metrics: format!("{}/metrics", runtime_config.endpoints.external_api_base_url),
            admin: format!("{}/admin", runtime_config.endpoints.external_api_base_url),
            websocket: Some(format!("wss://{}/ws", 
                runtime_config.endpoints.external_api_base_url.replace("https://", "").replace("http://", ""))),
        };
        let metadata = ProviderMetadata {
            name: "BearDog Security Provider".to_string(),
            version: "1.0.0".to_string(),
            description: "Universal security provider for ecosystem integration".to_string(),
            capabilities: vec![
                "encryption ".to_string(),
                "decryption".to_string(),
                "authentication".to_string(),
                "authorization".to_string(),
                "key_management".to_string(),
                "audit_logging".to_string(),
                "threat_detection".to_string(),
            ],
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

/// Metadata operation.
    pub fn metadata(&self) -> &ProviderMetadata {
        &self.metadata

/// Instance Id operation.
    pub fn instance_id(&self) -> &str {
        &self.instance_id

/// Config operation.
    pub fn config(&self) -> &Arc<BearDogConfig> {
        &self.config
use super::super::service_mesh_handoff::types::ServiceEndpoints;
