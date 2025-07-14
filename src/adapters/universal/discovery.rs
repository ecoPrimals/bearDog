//! Ecosystem Discovery Service
//!
//! **Cross-ecosystem service discovery and integration**

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::traits::*;
use crate::{BearDogCore, BearDogResult};

/// Ecosystem Discovery Service
pub struct EcosystemDiscovery {
    /// Core BearDog instance
    core: Arc<BearDogCore>,

    /// Discovered ecosystem services
    discovered_services: Arc<RwLock<HashMap<String, EcosystemService>>>,

    /// Service health status
    service_health: Arc<RwLock<HashMap<String, EcosystemServiceHealth>>>,

    /// Discovery configuration
    config: EcosystemDiscoveryConfig,
}

/// Ecosystem service information
#[derive(Debug, Clone)]
pub struct EcosystemService {
    pub service_id: String,
    pub ecosystem_id: String,
    pub instance_id: String,
    pub service_type: EcosystemServiceType,
    pub endpoints: ServiceEndpoints,
    pub capabilities: Vec<Capability>,
    pub discovery_time: chrono::DateTime<chrono::Utc>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

/// Types of ecosystem services
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EcosystemServiceType {
    Compute,       // ToadStool
    Storage,       // NestGate
    Communication, // SongBird
    AI,            // Squirrel
    BioMe,         // biomeOS
    Security,      // BearDog
    Custom(String),
}

/// Ecosystem service health information
#[derive(Debug, Clone)]
pub struct EcosystemServiceHealth {
    pub service_id: String,
    pub health_status: HealthStatus,
    pub last_health_check: chrono::DateTime<chrono::Utc>,
    pub response_time_ms: u64,
    pub availability_percentage: f64,
    pub error_rate_percentage: f64,
}

/// Discovery configuration
#[derive(Debug, Clone)]
pub struct EcosystemDiscoveryConfig {
    pub discovery_interval_seconds: u64,
    pub health_check_interval_seconds: u64,
    pub service_timeout_seconds: u64,
    pub max_discovery_attempts: u32,
    pub enabled_ecosystems: Vec<String>,
}

impl Default for EcosystemDiscoveryConfig {
    fn default() -> Self {
        Self {
            discovery_interval_seconds: 30,
            health_check_interval_seconds: 60,
            service_timeout_seconds: 10,
            max_discovery_attempts: 3,
            enabled_ecosystems: vec![
                "toadstool".to_string(),
                "songbird".to_string(),
                "nestgate".to_string(),
                "squirrel".to_string(),
                "biomeos".to_string(),
            ],
        }
    }
}

impl EcosystemDiscovery {
    /// Create a new ecosystem discovery service
    pub async fn new(core: Arc<BearDogCore>) -> BearDogResult<Self> {
        info!("🔍 Initializing Ecosystem Discovery Service");

        let discovery = Self {
            core,
            discovered_services: Arc::new(RwLock::new(HashMap::new())),
            service_health: Arc::new(RwLock::new(HashMap::new())),
            config: EcosystemDiscoveryConfig::default(),
        };

        Ok(discovery)
    }

    /// Create with custom configuration
    pub async fn with_config(
        core: Arc<BearDogCore>,
        config: EcosystemDiscoveryConfig,
    ) -> BearDogResult<Self> {
        info!("🔍 Initializing Ecosystem Discovery Service with custom config");

        let discovery = Self {
            core,
            discovered_services: Arc::new(RwLock::new(HashMap::new())),
            service_health: Arc::new(RwLock::new(HashMap::new())),
            config,
        };

        Ok(discovery)
    }

    /// Discover ecosystem services
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
        }

        // Update discovered services
        {
            let mut services = self.discovered_services.write().await;
            for service in &discovered {
                services.insert(service.service_id.clone(), service.clone());
            }
        }

        info!("✅ Discovered {} ecosystem services", discovered.len());
        Ok(discovered)
    }

    /// Get all discovered services
    pub async fn get_discovered_services(&self) -> Vec<EcosystemService> {
        let services = self.discovered_services.read().await;
        services.values().cloned().collect()
    }

    /// Get services for a specific ecosystem
    pub async fn get_ecosystem_services(&self, ecosystem_id: &str) -> Vec<EcosystemService> {
        let services = self.discovered_services.read().await;
        services
            .values()
            .filter(|service| service.ecosystem_id == ecosystem_id)
            .cloned()
            .collect()
    }

    /// Get services by type
    pub async fn get_services_by_type(
        &self,
        service_type: EcosystemServiceType,
    ) -> Vec<EcosystemService> {
        let services = self.discovered_services.read().await;
        services
            .values()
            .filter(|service| service.service_type == service_type)
            .cloned()
            .collect()
    }

    /// Get service health status
    pub async fn get_service_health(&self, service_id: &str) -> Option<EcosystemServiceHealth> {
        let health = self.service_health.read().await;
        health.get(service_id).cloned()
    }

    /// Perform health check on all discovered services
    pub async fn perform_health_checks(&self) -> BearDogResult<Vec<EcosystemServiceHealth>> {
        debug!("🏥 Performing health checks on all discovered services");

        let services = self.get_discovered_services().await;
        let mut health_results = Vec::new();

        for service in services {
            match self.check_service_health(&service).await {
                Ok(health) => {
                    health_results.push(health);
                }
                Err(e) => {
                    warn!(
                        "Health check failed for service {}: {}",
                        service.service_id, e
                    );

                    // Create a failed health status
                    let failed_health = EcosystemServiceHealth {
                        service_id: service.service_id.clone(),
                        health_status: HealthStatus::Unhealthy {
                            reason: format!("Health check failed: {}", e),
                            recovery_time: None,
                        },
                        last_health_check: chrono::Utc::now(),
                        response_time_ms: 0,
                        availability_percentage: 0.0,
                        error_rate_percentage: 100.0,
                    };
                    health_results.push(failed_health);
                }
            }
        }

        // Update health status
        {
            let mut health = self.service_health.write().await;
            for health_status in &health_results {
                health.insert(health_status.service_id.clone(), health_status.clone());
            }
        }

        info!(
            "✅ Completed health checks for {} services",
            health_results.len()
        );
        Ok(health_results)
    }

    /// Register BearDog as a security provider in the ecosystem
    pub async fn register_beardog_as_security_provider(
        &self,
    ) -> BearDogResult<EcosystemRegistration> {
        info!("🔒 Registering BearDog as universal security provider");

        let registration = EcosystemRegistration {
            registration_id: uuid::Uuid::new_v4(),
            ecosystem_id: "beardog".to_string(),
            instance_id: "beardog-universal-001".to_string(),
            endpoints: ServiceEndpoints::default(),
            capabilities: self.get_beardog_universal_capabilities(),
            registration_time: chrono::Utc::now(),
            status: RegistrationStatus::Active,
        };

        info!("✅ BearDog registered as universal security provider");
        Ok(registration)
    }

    /// Get BearDog's universal security capabilities
    fn get_beardog_universal_capabilities(&self) -> Vec<Capability> {
        use super::capability_ids;

        vec![
            Capability {
                id: capability_ids::SECURITY_ENCRYPT.to_string(),
                name: "Universal Encryption".to_string(),
                description: "Provide encryption services to all ecosystem components".to_string(),
                category: CapabilityCategory::Security,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            },
            Capability {
                id: capability_ids::SECURITY_AUTHENTICATE.to_string(),
                name: "Universal Authentication".to_string(),
                description: "Provide authentication services to all ecosystem components"
                    .to_string(),
                category: CapabilityCategory::Security,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            },
            Capability {
                id: capability_ids::SECURITY_AUTHORIZE.to_string(),
                name: "Universal Authorization".to_string(),
                description: "Provide authorization services to all ecosystem components"
                    .to_string(),
                category: CapabilityCategory::Security,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            },
            Capability {
                id: capability_ids::SECURITY_AUDIT.to_string(),
                name: "Universal Audit".to_string(),
                description: "Provide audit services to all ecosystem components".to_string(),
                category: CapabilityCategory::Security,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            },
        ]
    }

    /// Find services based on search criteria
    pub async fn find_services(
        &self,
        criteria: ServiceSearchCriteria,
    ) -> BearDogResult<Vec<EcosystemService>> {
        let services = self.get_discovered_services().await;

        let matching_services: Vec<EcosystemService> = services
            .into_iter()
            .filter(|service| self.service_matches_criteria(service, &criteria))
            .collect();

        Ok(matching_services)
    }

    /// Start background discovery and health checking (placeholder)
    async fn start_background_discovery(&self) -> BearDogResult<()> {
        debug!("🔄 Starting background discovery service");
        // TODO: Implement background tasks
        Ok(())
    }

    /// Discover services for a specific ecosystem
    async fn discover_ecosystem_services(
        &self,
        ecosystem_id: &str,
    ) -> BearDogResult<Vec<EcosystemService>> {
        match ecosystem_id {
            "toadstool" => self.discover_toadstool_services().await,
            "songbird" => self.discover_songbird_services().await,
            "nestgate" => self.discover_nestgate_services().await,
            "squirrel" => self.discover_squirrel_services().await,
            "biomeos" => self.discover_biomeos_services().await,
            _ => Ok(Vec::new()),
        }
    }

    /// Discover ToadStool compute services
    async fn discover_toadstool_services(&self) -> BearDogResult<Vec<EcosystemService>> {
        debug!("🔍 Discovering ToadStool compute services");

        // Mock discovery for now
        let service = EcosystemService {
            service_id: "toadstool-compute-001".to_string(),
            ecosystem_id: "toadstool".to_string(),
            instance_id: "compute-001".to_string(),
            service_type: EcosystemServiceType::Compute,
            endpoints: ServiceEndpoints::default(),
            capabilities: vec![Capability {
                id: "compute.execute".to_string(),
                name: "Code Execution".to_string(),
                description: "Execute code on various platforms".to_string(),
                category: CapabilityCategory::Compute,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            }],
            discovery_time: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        Ok(vec![service])
    }

    /// Discover SongBird communication services
    async fn discover_songbird_services(&self) -> BearDogResult<Vec<EcosystemService>> {
        debug!("🔍 Discovering SongBird communication services");

        // Mock discovery for now
        let service = EcosystemService {
            service_id: "songbird-discovery-001".to_string(),
            ecosystem_id: "songbird".to_string(),
            instance_id: "discovery-001".to_string(),
            service_type: EcosystemServiceType::Communication,
            endpoints: ServiceEndpoints::default(),
            capabilities: vec![Capability {
                id: "communication.discovery".to_string(),
                name: "Service Discovery".to_string(),
                description: "Discover and route to ecosystem services".to_string(),
                category: CapabilityCategory::Communication,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            }],
            discovery_time: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        Ok(vec![service])
    }

    /// Discover NestGate storage services
    async fn discover_nestgate_services(&self) -> BearDogResult<Vec<EcosystemService>> {
        debug!("🔍 Discovering NestGate storage services");

        // Mock discovery for now
        let service = EcosystemService {
            service_id: "nestgate-storage-001".to_string(),
            ecosystem_id: "nestgate".to_string(),
            instance_id: "storage-001".to_string(),
            service_type: EcosystemServiceType::Storage,
            endpoints: ServiceEndpoints::default(),
            capabilities: vec![Capability {
                id: "storage.persist".to_string(),
                name: "Data Persistence".to_string(),
                description: "Secure data storage and retrieval".to_string(),
                category: CapabilityCategory::Storage,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            }],
            discovery_time: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        Ok(vec![service])
    }

    /// Discover Squirrel AI services
    async fn discover_squirrel_services(&self) -> BearDogResult<Vec<EcosystemService>> {
        debug!("🔍 Discovering Squirrel AI services");

        // Mock discovery for now
        let service = EcosystemService {
            service_id: "squirrel-ai-001".to_string(),
            ecosystem_id: "squirrel".to_string(),
            instance_id: "ai-001".to_string(),
            service_type: EcosystemServiceType::AI,
            endpoints: ServiceEndpoints::default(),
            capabilities: vec![Capability {
                id: "ai.inference".to_string(),
                name: "AI Inference".to_string(),
                description: "AI model inference and analysis".to_string(),
                category: CapabilityCategory::AI,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            }],
            discovery_time: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        Ok(vec![service])
    }

    /// Discover biomeOS services
    async fn discover_biomeos_services(&self) -> BearDogResult<Vec<EcosystemService>> {
        debug!("🔍 Discovering biomeOS services");

        // Mock discovery for now
        let service = EcosystemService {
            service_id: "biomeos-integration-001".to_string(),
            ecosystem_id: "biomeos".to_string(),
            instance_id: "integration-001".to_string(),
            service_type: EcosystemServiceType::Custom("Integration".to_string()),
            endpoints: ServiceEndpoints::default(),
            capabilities: vec![Capability {
                id: "integration.adapt".to_string(),
                name: "System Integration".to_string(),
                description: "Integrate with biometric and environmental systems".to_string(),
                category: CapabilityCategory::Integration,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            }],
            discovery_time: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        Ok(vec![service])
    }

    /// Check the health of a specific service
    async fn check_service_health(
        &self,
        service: &EcosystemService,
    ) -> BearDogResult<EcosystemServiceHealth> {
        debug!("🏥 Checking health for service: {}", service.service_id);

        // Mock health check for now
        let health = EcosystemServiceHealth {
            service_id: service.service_id.clone(),
            health_status: HealthStatus::Healthy,
            last_health_check: chrono::Utc::now(),
            response_time_ms: 50,
            availability_percentage: 99.5,
            error_rate_percentage: 0.1,
        };

        Ok(health)
    }

    /// Register with an ecosystem (placeholder)
    async fn register_with_ecosystem(
        &self,
        _ecosystem_id: &str,
        _registration: &EcosystemRegistration,
    ) -> BearDogResult<()> {
        // TODO: Implement registration with specific ecosystems
        Ok(())
    }

    /// Check if service matches search criteria
    fn service_matches_criteria(
        &self,
        service: &EcosystemService,
        criteria: &ServiceSearchCriteria,
    ) -> bool {
        // Check ecosystem ID
        if let Some(ref ecosystem_id) = criteria.ecosystem_id {
            if service.ecosystem_id != *ecosystem_id {
                return false;
            }
        }

        // Check service type
        if let Some(ref service_type) = criteria.service_type {
            if service.service_type != *service_type {
                return false;
            }
        }

        true
    }
}

/// Service search criteria
#[derive(Debug, Clone, Default)]
pub struct ServiceSearchCriteria {
    pub ecosystem_id: Option<String>,
    pub service_type: Option<EcosystemServiceType>,
}
