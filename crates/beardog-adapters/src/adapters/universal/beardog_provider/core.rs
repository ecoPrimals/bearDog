

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use super::super::songbird_handoff::health::UniversalHealthMonitor;
use super::super::songbird_handoff::registration::SongBirdRegistrationManager;
use beardog_types::canonical::BearDogConfig;

#[derive(Debug, Clone)]
pub struct ProviderMetadata {

    pub name: String,

    pub version: String,

    pub description: String,

    pub capabilities: Vec<String>,
}

pub struct BearDogPrimalProvider<T: Send + Sync> {

    pub(crate) core: Arc<RwLock<Option<T>>>,

    pub(crate) instance_id: String,

    pub(crate) metadata: ProviderMetadata,

    pub(crate) config: Arc<BearDogConfig>,

    pub(crate) registration_manager: Arc<SongBirdRegistrationManager<T>>,

    pub(crate) health_monitor: Arc<UniversalHealthMonitor>,

    pub(crate) background_tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
impl<T: Send + Sync> BearDogPrimalProvider<T> {

    pub fn new(
        core: Arc<RwLock<Option<T>>>,
        instance_id: &str,
        config: Arc<BearDogConfig>,
        registration_manager: Arc<SongBirdRegistrationManager<T>>,
        health_monitor: Arc<UniversalHealthMonitor>,
    ) -> Self {

        let runtime_config = beardog_types::config::get_config();
        let _endpoints = ServiceEndpoints {
            primary: runtime_config.endpoints.external_api_base_url.clone(),
            health: format_args!("{}/health", runtime_config.endpoints.external_api_base_url).to_string(),
            metrics: format_args!("{}/metrics", runtime_config.endpoints.external_api_base_url).to_string(),
            admin: format_args!("{}/admin", runtime_config.endpoints.external_api_base_url).to_string(),
            websocket: Some(format_args!("wss://{}/ws", 
                runtime_config.endpoints.external_api_base_url.replace("https://", "").to_string().replace("http://", ""))),
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

    pub fn metadata(&self) -> &ProviderMetadata {
        &self.metadata

    pub fn instance_id(&self) -> &str {
        &self.instance_id

    pub fn config(&self) -> &Arc<BearDogConfig> {
        &self.config
use super::super::songbird_handoff::types::ServiceEndpoints;
