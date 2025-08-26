

use super::*;
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

pub struct EcosystemDiscovery<T> {

    core: Arc<T>,

    discovered_services: Arc<RwLock<HashMap<String, EcosystemService>>>,

    service_health: Arc<RwLock<HashMap<String, EcosystemServiceHealth>>>,

    config: EcosystemDiscoveryConfig,
}
impl<T> EcosystemDiscovery<T> {

    pub async fn new(core: Arc<T>) -> BearDogResult<Self> {
        info!("🔍 Initializing Ecosystem Discovery Service");
        let discovery = Self {
            core,
            discovered_services: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            service_health: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config: EcosystemDiscoveryConfig::default(),
        };
        Ok(discovery)
    }

    pub async fn with_config(
        core: Arc<T>,
        config: EcosystemDiscoveryConfig,
    ) -> BearDogResult<Self> {
        info!("🔍 Initializing Ecosystem Discovery Service with custom config");
            config,

    pub async fn discover_services(&self) -> BearDogResult<Vec<EcosystemService>> {
        debug!("🔍 Starting ecosystem service discovery");
        let mut discovered = Vec::new();
        for ecosystem_id in &self.config.enabled_ecosystems {
            match self.discover_ecosystem_services(ecosystem_id).await {
                Ok(mut services) => {
                    discovered.append(&mut services);
                }
                Err(e) => {
                    warn!(
                        "Failed to discover services for ecosystem {}: {}",
                        ecosystem_id, e
                    );
            }
        }

        {
            let mut services = self.discovered_services.write().await;
            for service in &discovered {
                services.insert(service.service_id.clone(), service.clone());
        info!("✅ Discovered {} ecosystem services", discovered.len());
        Ok(discovered)

    pub async fn get_discovered_services(&self) -> Vec<EcosystemService> {
        let services = self.discovered_services.read().await;
        services.values().cloned().collect()

    pub async fn get_ecosystem_services(&self, ecosystem_id: &str) -> Vec<EcosystemService> {
        services
            .values()
            .filter(|service| service.ecosystem_id == ecosystem_id)
            .cloned()
            .collect()

    async fn discover_ecosystem_services(
        &self,
        ecosystem_id: &str,
    ) -> BearDogResult<Vec<EcosystemService>> {

        debug!("🔍 Discovering services for ecosystem: {}", ecosystem_id);
        Ok(Vec::new())
} 
