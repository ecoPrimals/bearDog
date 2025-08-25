// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Integration engine for ecosystem coordination
/// 
/// Enhanced for Phase 4: Universal HSM Architecture with full ecosystem integration

use super::{UniversalHsmProvider, SongbirdServiceDiscovery, SongbirdServiceDiscoveryFactory};
use beardog_types::config::monitoring::IntegrationConfig;
use beardog_errors::BearDogResult;
use beardog_errors::idiomatic::SystemResult;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

/// Enhanced Integration Engine with Universal HSM Architecture
#[derive(Debug)]
pub struct IntegrationEngine {
    config: IntegrationConfig,
    /// Universal HSM Provider for vendor-agnostic HSM operations
    universal_hsm: Option<Arc<UniversalHsmProvider>>,
    /// Songbird service discovery for ecosystem integration
    songbird_discovery: Option<Arc<RwLock<SongbirdServiceDiscovery>>>,
    /// Integration status
    ecosystem_integrated: bool,
}

impl Default for IntegrationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegrationEngine {
    /// Create a new integration engine
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: IntegrationConfig::default(),
            universal_hsm: None,
            songbird_discovery: None,
            ecosystem_integrated: false,
        }
    }

    /// Create integration engine from configuration
    pub fn from_config(config: IntegrationConfig) -> Self {
        Self {
            config,
            universal_hsm: None,
            songbird_discovery: None,
            ecosystem_integrated: false,
        }
    }

    /// Initialize Universal HSM Architecture
    pub async fn initialize_universal_hsm(&mut self) -> BearDogResult<()> {
        info!("🔐 Initializing Universal HSM Architecture");

        // Create Songbird service discovery
        let songbird_discovery = SongbirdServiceDiscoveryFactory::create_default();
        let songbird_arc = Arc::new(RwLock::new(songbird_discovery));

        // Create Universal HSM Provider with Songbird integration
        let universal_hsm = UniversalHsmProvider::with_service_discovery(
            songbird_arc.clone() as Arc<dyn super::EcosystemServiceDiscovery + Send + Sync>
        );

        // Store components
        self.universal_hsm = Some(Arc::new(universal_hsm));
        self.songbird_discovery = Some(songbird_arc);

        info!("✅ Universal HSM Architecture initialized");
        Ok(())
    }

    /// Integrate with the ecosystem through Songbird service mesh
    pub async fn integrate_with_ecosystem(&mut self) -> BearDogResult<()> {
        info!("🌐 Starting Phase 4: Ecosystem Integration");

        // Initialize Universal HSM if not already done
        if self.universal_hsm.is_none() {
            self.initialize_universal_hsm().await?;
        }

        // Register BearDog with Songbird service mesh
        if let Some(songbird_discovery) = &self.songbird_discovery {
            let mut discovery = songbird_discovery.write().await;
            match discovery.register_beardog_service().await {
                Ok(()) => {
                    info!("🎼 Successfully registered with Songbird service mesh");
                }
                Err(e) => {
                    warn!("Failed to register with Songbird (will retry later): {}", e);
                    // Don't fail completely - we can operate without Songbird
                }
            }
        }

        // Discover ecosystem HSM providers
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

    /// Get Universal HSM Provider
    pub fn get_universal_hsm(&self) -> Option<Arc<UniversalHsmProvider>> {
        self.universal_hsm.clone()
    }

    /// Get Songbird service discovery client
    pub fn get_songbird_discovery(&self) -> Option<Arc<RwLock<SongbirdServiceDiscovery>>> {
        self.songbird_discovery.clone()
    }

    /// Check if ecosystem integration is active
    pub fn is_ecosystem_integrated(&self) -> bool {
        self.ecosystem_integrated
    }

    /// Get comprehensive service health status including Universal HSM
    pub async fn get_service_health(&self) -> SystemResult<serde_json::Value> {
        let mut health_status = serde_json::Map::new();
        
        // Basic integration engine status
        health_status.insert("integration_engine".to_string(), serde_json::json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "config_loaded": true,
            "ecosystem_integrated": self.ecosystem_integrated
        }));

        // Universal HSM status
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

        // Songbird connectivity status
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

        // Overall ecosystem connectivity
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

    /// Refresh ecosystem providers and services
    pub async fn refresh_ecosystem_services(&self) -> BearDogResult<()> {
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
    
    /// Get the count of discovered services
    async fn get_discovered_services_count(&self) -> BearDogResult<u32> {
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
