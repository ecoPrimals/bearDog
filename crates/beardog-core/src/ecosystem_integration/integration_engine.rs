

use super::{UniversalHsmProvider, SongbirdServiceDiscovery, SongbirdServiceDiscoveryFactory};
use beardog_types::canonical::configuration::consolidated::IntegrationConfig;
use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

#[derive(Debug)]
pub struct IntegrationEngine {
    config: IntegrationConfig,

    universal_hsm: Option<Arc<UniversalHsmProvider>>,

    songbird_discovery: Option<Arc<RwLock<SongbirdServiceDiscovery>>>,

    ecosystem_integrated: bool,
}

impl Default for IntegrationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegrationEngine {

    #[must_use]
    pub fn new() -> Self {
        Self {
            config: IntegrationConfig::default(),
            universal_hsm: None,
            songbird_discovery: None,
            ecosystem_integrated: false,
        }
    }

    pub fn from_config(config: IntegrationConfig) -> Self {
        Self {
            config,
            universal_hsm: None,
            songbird_discovery: None,
            ecosystem_integrated: false,
        }
    }

    pub async fn initialize_universal_hsm(&mut self) -> Result<(), BearDogError> {
        info!("🔐 Initializing Universal HSM Architecture");

        let songbird_discovery = SongbirdServiceDiscoveryFactory::create_default();
        let songbird_arc = Arc::new(RwLock::new(songbird_discovery));

        let universal_hsm = UniversalHsmProvider::with_service_discovery(
            songbird_arc.clone() as ZeroCostsuper<impl super>
        );

        self.universal_hsm = Some(Arc::new(universal_hsm));
        self.songbird_discovery = Some(songbird_arc);

        info!("✅ Universal HSM Architecture initialized");
        Ok(())
    }

    pub async fn integrate_with_ecosystem(&mut self) -> Result<(), BearDogError> {
        info!("🌐 Starting Phase 4: Ecosystem Integration");

        if self.universal_hsm.is_none() {
            self.initialize_universal_hsm().await?;
        }

        if let Some(songbird_discovery) = &self.songbird_discovery {
            let mut discovery = songbird_discovery.write().await;
            match discovery.register_beardog_service().await {
                Ok(()) => {
                    info!("🎼 Successfully registered with Songbird service mesh");
                }
                Err(e) => {
                    warn!("Failed to register with Songbird (will retry later): {}", e);

                }
            }
        }

        if let Some(universal_hsm) = &self.universal_hsm {
            match universal_hsm.discover_ecosystem_providers().await {
                Ok(providers) => {
                    info!("🌐 Discovered {} ecosystem HSM providers", providers.len());
                    for provider in &providers {
                        info!("  - {} ({}) - {}", provider.name, provider.vendor, provider.endpoint);
                    }
                }
                Err(e) => {
                    warn!("Failed to discover ecosystem providers: {}", e);
                }
            }
        }

        self.ecosystem_integrated = true;
        info!("✅ Phase 4: Ecosystem Integration completed");
        Ok(())
    }

    pub fn get_universal_hsm(&self) -> Option<Arc<UniversalHsmProvider>> {
        self.universal_hsm.clone()
    }

    pub fn get_songbird_discovery(&self) -> Option<Arc<RwLock<SongbirdServiceDiscovery>>> {
        self.songbird_discovery.clone()
    }

    pub fn is_ecosystem_integrated(&self) -> bool {
        self.ecosystem_integrated
    }

    pub async fn get_service_health(&self) -> Result<serde_json::Value, BearDogError> {
        let mut health_status = serde_json::Map::new();

        health_status.insert("integration_engine".to_string(), serde_json::json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "config_loaded": true,
            "ecosystem_integrated": self.ecosystem_integrated
        }));

        if let Some(universal_hsm) = &self.universal_hsm {
            match universal_hsm.get_ecosystem_status().await {
                Ok(hsm_status) => {
                    health_status.insert("universal_hsm".to_string(), hsm_status);
                }
                Err(e) => {
                    health_status.insert("universal_hsm".to_string(), serde_json::json!({
                        "status": "error",
                        "error": e.to_string()
                    }));
                }
            }
        } else {
            health_status.insert("universal_hsm".to_string(), serde_json::json!({
                "status": "not_initialized"
            }));
        }

        if let Some(songbird_discovery) = &self.songbird_discovery {
            let discovery = songbird_discovery.read().await;
            match discovery.health_check().await {
                Ok(is_healthy) => {
                    health_status.insert("songbird_connectivity".to_string(), serde_json::json!({
                        "status": if is_healthy { "healthy" } else { "degraded" },
                        "connected": is_healthy,
                        "last_check": chrono::Utc::now().to_rfc3339()
                    }));
                }
                Err(e) => {
                    health_status.insert("songbird_connectivity".to_string(), serde_json::json!({
                        "status": "error",
                        "connected": false,
                        "error": e.to_string(),
                        "last_check": chrono::Utc::now().to_rfc3339()
                    }));
                }
            }
        } else {
            health_status.insert("songbird_connectivity".to_string(), serde_json::json!({
                "status": "not_configured",
                "connected": false
            }));
        }

        let ecosystem_status = if self.ecosystem_integrated {
            "operational"
        } else {
            "initializing"
        };

        health_status.insert("ecosystem_connectivity".to_string(), serde_json::json!({
            "status": ecosystem_status,
            "phase": "Phase 4: Universal HSM Architecture",
            "services_discovered": self.get_discovered_services_count().await.unwrap_or(0),
            "last_check": chrono::Utc::now().to_rfc3339()
        }));
        
        Ok(serde_json::Value::Object(health_status))
    }

    pub async fn refresh_ecosystem_services(&self) -> Result<(), BearDogError> {
        info!("🔄 Refreshing ecosystem services");

        if let Some(universal_hsm) = &self.universal_hsm {
            match universal_hsm.discover_ecosystem_providers().await {
                Ok(providers) => {
                    info!("🌐 Refreshed {} ecosystem HSM providers", providers.len());
                }
                Err(e) => {
                    warn!("Failed to refresh ecosystem providers: {}", e);
                }
            }
        }

        Ok(())
    }

    async fn get_discovered_services_count(&self) -> Result<u32, BearDogError> {
        if let Some(universal_hsm) = &self.universal_hsm {
            match universal_hsm.discover_ecosystem_providers().await {
                Ok(providers) => Ok(providers.len() as u32),
                Err(_) => Ok(0)
            }
        } else {
            Ok(0)
        }
    }
}
