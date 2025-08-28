

use super::universal_hsm_provider::{EcosystemHsmProvider, EcosystemServiceDiscovery, ProviderHealthStatus};
use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::{HsmCapabilities, HsmHardwareStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongbirdServiceDiscoveryConfig {

    pub songbird_endpoint: String,

    pub discovery_timeout_ms: u64,

    pub health_check_timeout_ms: u64,

    pub registration_retries: u32,

    pub beardog_service_info: BearDogServiceInfo,
}

impl Default for SongbirdServiceDiscoveryConfig {
    fn default() -> Self {
        Self {
            songbird_endpoint: std::env::var("BEARDOG_SONGBIRD_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            discovery_timeout_ms: 30000,
            health_check_timeout_ms: 5000,
            registration_retries: 3,
            beardog_service_info: BearDogServiceInfo::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogServiceInfo {
    pub service_id: String,
    pub service_name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub endpoint: String,
    pub health_check_path: String,
    pub priority: u32,
}

impl Default for BearDogServiceInfo {
    fn default() -> Self {
        Self {
            service_id: format_args!("beardog-{}", Uuid::new_v4().to_string()),
            service_name: "BearDog Universal HSM".to_string(),
            version: "4.0.0".to_string(),
            capabilities: vec![
                "universal_hsm".to_string(),
                "key_management".to_string(),
                "cryptographic_operations".to_string(),
                "vendor_agnostic_hsm".to_string(),
                "ecosystem_hsm_provider".to_string(),
            ],
            endpoint: "http://localhost:3000".to_string(),
            health_check_path: "/health".to_string(),
            priority: 100, // High priority for HSM operations
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistryResponse {
    pub services: Vec<ServiceRegistration>,
    pub total_count: usize,
    pub page: Option<u32>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    pub service_id: String,
    pub service_name: String,
    pub service_type: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub health_status: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub registered_at: String,
    pub last_health_check: Option<String>,
}

#[derive(Debug)]
pub struct SongbirdServiceDiscovery {
    config: SongbirdServiceDiscoveryConfig,
    http_client: reqwest::Client,
    registered: bool,
}

impl SongbirdServiceDiscovery {

    pub fn new(config: SongbirdServiceDiscoveryConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_millis(30000))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
            registered: false,
        }
    }

    pub async fn register_beardog_service(&mut self) -> Result<(), BearDogError> {
        let registration_payload = serde_json::json!({
            "service_id": self.config.beardog_service_info.service_id,
            "service_name": self.config.beardog_service_info.service_name,
            "service_type": "hsm_provider",
            "endpoint": self.config.beardog_service_info.endpoint,
            "capabilities": self.config.beardog_service_info.capabilities,
            "health_check_path": self.config.beardog_service_info.health_check_path,
            "priority": self.config.beardog_service_info.priority,
            "metadata": {
                "version": self.config.beardog_service_info.version,
                "vendor_agnostic": true,
                "universal_hsm": true,
                "ecosystem_integration": "songbird",
                "failover_support": true,
                "ai_first_apis": true
            }
        });

        let registration_url = format_args!("{}/api/v1/services/register", self.config.songbird_endpoint).to_string();
        let timeout_duration = Duration::from_millis(self.config.health_check_timeout_ms);

        for attempt in 1..=self.config.registration_retries {
            match timeout(
                timeout_duration,
                self.http_client
                    .post(&registration_url)
                    .json(&registration_payload)
                    .send(),
            ).await {
                Ok(Ok(response)) => {
                    if response.status().is_success() {
                        self.registered = true;
                        info!("🎼 Successfully registered BearDog with Songbird service mesh (attempt {})", attempt);
                        return Ok(());
                    } else {
                        warn!("Songbird registration failed with status: {} (attempt {})", response.status(), attempt);
                    }
                }
                Ok(Err(e)) => {
                    warn!("HTTP error during Songbird registration (attempt {}): {}", attempt, e);
                }
                Err(_) => {
                    warn!("Timeout during Songbird registration (attempt {})", attempt);
                }
            }

            if attempt < self.config.registration_retries {
                tokio::time::sleep(Duration::from_millis(1000 * attempt as u64)).await;
            }
        }

        Err(BearDogError::network_error(
            "Failed to register with Songbird after all retries".to_string(),
        ))
    }

    async fn query_services_by_capability(&self, capability: &str) -> Result<Vec<ServiceRegistration>, BearDogError> {
        let query_url = format!(
            "{}/api/v1/services/query?capability={}",
            self.config.songbird_endpoint,
            urlencoding::encode(capability)
        );

        let timeout_duration = Duration::from_millis(self.config.discovery_timeout_ms);

        match timeout(
            timeout_duration,
            self.http_client.get(&query_url).send(),
        ).await {
            Ok(Ok(response)) => {
                if response.status().is_success() {
                    match response.json::<ServiceRegistryResponse>().await {
                        Ok(registry_response) => {
                            debug!("Found {} services with capability '{}'", registry_response.services.len(), capability);
                            Ok(registry_response.services)
                        }
                        Err(e) => {
                            warn!("Failed to parse Songbird service registry response: {}", e);
                            Ok(vec![])
                        }
                    }
                } else {
                    warn!("Songbird service query failed with status: {}", response.status());
                    Ok(vec![])
                }
            }
            Ok(Err(e)) => {
                warn!("HTTP error during Songbird service query: {}", e);
                Ok(vec![])
            }
            Err(_) => {
                warn!("Timeout during Songbird service query");
                Ok(vec![])
            }
        }
    }

    fn convert_to_ecosystem_provider(&self, service: &ServiceRegistration) -> Option<EcosystemHsmProvider> {

        if !service.capabilities.iter().any(|cap| 
            cap.contains("hsm") || cap.contains("crypto") || cap.contains("key_management")
        ) {
            return None;
        }

        let health_status = match service.health_status.as_str() {
            "healthy" => ProviderHealthStatus::Healthy,
            "optimal" => ProviderHealthStatus::Optimal,
            "degraded" => ProviderHealthStatus::Degraded,
            "unhealthy" => ProviderHealthStatus::Unhealthy,
            _ => ProviderHealthStatus::Offline,
        };

        let vendor = service.metadata
            .get("vendor")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let capabilities = HsmCapabilities {
            supported_key_types: vec![], // Will be populated by actual provider
            max_keys: 1000,
            hardware_backed: true,
            fips_140_2_level: Some(2),
            common_criteria_level: None,
            supports_key_derivation: true,
            supports_bulk_operations: false,
        };

        Some(EcosystemHsmProvider {
            id: service.service_id.clone(),
            name: service.service_name.clone(),
            vendor,
            capabilities,
            endpoint: service.endpoint.clone(),
            priority: service.metadata
                .get("priority")
                .and_then(|p| p.as_u64())
                .unwrap_or(500) as u32,
            health_status,
            ecosystem_node: service.metadata
                .get("node_id")
                .and_then(|n| n.as_str())
                .unwrap_or("unknown")
                .to_string(),
        })
    }
}

impl EcosystemServiceDiscovery for SongbirdServiceDiscovery {

    async fn discover_hsm_providers(&self) -> Result<Vec<EcosystemHsmProvider>, BearDogError> {
        debug!("🔍 Discovering HSM providers through Songbird service mesh");

        let hsm_capabilities = vec![
            "hsm_provider",
            "key_management", 
            "cryptographic_operations",
            "universal_hsm",
            "hardware_security_module",
        ];

        let mut discovered_providers = Vec::new();
        let mut seen_service_ids = std::collections::HashSet::new();

        for capability in hsm_capabilities {
            match self.query_services_by_capability(capability).await {
                Ok(services) => {
                    for service in services {

                        if seen_service_ids.contains(&service.service_id) {
                            continue;
                        }
                        seen_service_ids.insert(service.service_id.clone());

                        if let Some(provider) = self.convert_to_ecosystem_provider(&service) {
                            discovered_providers.push(provider);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to query services for capability '{}': {}", capability, e);
                }
            }
        }

        info!("🌐 Discovered {} unique HSM providers through Songbird", discovered_providers.len());
        Ok(discovered_providers)
    }

    async fn register_hsm_provider(&self, provider_info: &EcosystemHsmProvider) -> Result<(), BearDogError> {
        let registration_payload = serde_json::json!({
            "service_id": provider_info.id,
            "service_name": provider_info.name,
            "service_type": "hsm_provider",
            "endpoint": provider_info.endpoint,
            "capabilities": [
                "hsm_provider",
                "key_management",
                "cryptographic_operations"
            ],
            "priority": provider_info.priority,
            "metadata": {
                "vendor": provider_info.vendor,
                "ecosystem_node": provider_info.ecosystem_node,
                "health_status": format_args!("{:?}", provider_info.health_status).to_string(),
                "registered_by": "beardog_universal_hsm"
            }
        });

        let registration_url = format_args!("{}/api/v1/services/register", self.config.songbird_endpoint).to_string();
        let timeout_duration = Duration::from_millis(self.config.health_check_timeout_ms);

        match timeout(
            timeout_duration,
            self.http_client
                .post(&registration_url)
                .json(&registration_payload)
                .send(),
        ).await {
            Ok(Ok(response)) => {
                if response.status().is_success() {
                    info!("✅ Successfully registered HSM provider '{}' with Songbird", provider_info.name);
                    Ok(())
                } else {
                    Err(BearDogError::network_error(format!(
                        "Failed to register HSM provider with Songbird: {}",
                        response.status()
                    )))
                }
            }
            Ok(Err(e)) => Err(BearDogError::network_error(format!(
                "HTTP error registering HSM provider: {}",
                e
            ))),
            Err(_) => Err(BearDogError::network_error(
                "Timeout registering HSM provider with Songbird".to_string(),
            )),
        }
    }

    async fn health_check(&self) -> Result<bool, BearDogError> {
        let health_url = format_args!("{}/api/v1/health", self.config.songbird_endpoint).to_string();
        let timeout_duration = Duration::from_millis(self.config.health_check_timeout_ms);

        match timeout(
            timeout_duration,
            self.http_client.get(&health_url).send(),
        ).await {
            Ok(Ok(response)) => {
                let is_healthy = response.status().is_success();
                if is_healthy {
                    debug!("✅ Songbird service mesh health check passed");
                } else {
                    warn!("⚠️ Songbird service mesh health check failed: {}", response.status());
                }
                Ok(is_healthy)
            }
            Ok(Err(e)) => {
                warn!("❌ Songbird health check HTTP error: {}", e);
                Ok(false)
            }
            Err(_) => {
                warn!("❌ Songbird health check timeout");
                Ok(false)
            }
        }
    }
}

pub struct SongbirdServiceDiscoveryFactory;

impl SongbirdServiceDiscoveryFactory {

    pub fn create_default() -> SongbirdServiceDiscovery {
        SongbirdServiceDiscovery::new(SongbirdServiceDiscoveryConfig::default())
    }

    pub fn create_with_config(config: SongbirdServiceDiscoveryConfig) -> SongbirdServiceDiscovery {
        SongbirdServiceDiscovery::new(config)
    }

    pub fn create_for_development() -> SongbirdServiceDiscovery {
        let mut config = SongbirdServiceDiscoveryConfig::default();
        config.songbird_endpoint = "http://localhost:8080".to_string();
        config.beardog_service_info.endpoint = "http://localhost:3000".to_string();
        SongbirdServiceDiscovery::new(config)
    }

    pub fn create_for_production(songbird_endpoint: &str, beardog_endpoint: &str) -> SongbirdServiceDiscovery {
        let mut config = SongbirdServiceDiscoveryConfig::default();
        config.songbird_endpoint = songbird_endpoint;
        config.beardog_service_info.endpoint = beardog_endpoint;
        config.discovery_timeout_ms = 10000; // Shorter timeout for production
        config.health_check_timeout_ms = 3000;
        SongbirdServiceDiscovery::new(config)
    }
} 