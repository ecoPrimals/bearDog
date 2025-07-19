//! Ecosystem Discovery Service
//!
//! **Cross-ecosystem service discovery and integration**

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::traits::*;
use beardog_errors::{BearDogError, BearDogResult};

/// Ecosystem Discovery Service
pub struct EcosystemDiscovery<T> {
    /// Core BearDog instance
    core: Arc<T>,

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
    /// Unique identifier for the service
    pub service_id: String,
    /// Identifier of the ecosystem this service belongs to
    pub ecosystem_id: String,
    /// Unique identifier for this service instance
    pub instance_id: String,
    /// Type of service provided
    pub service_type: EcosystemServiceType,
    /// Network endpoints for accessing the service
    pub endpoints: ServiceEndpoints,
    /// List of capabilities provided by this service
    pub capabilities: Vec<Capability>,
    /// When this service was first discovered
    pub discovery_time: chrono::DateTime<chrono::Utc>,
    /// Last time this service was seen as active
    pub last_seen: chrono::DateTime<chrono::Utc>,
    /// Additional metadata about the service
    pub metadata: HashMap<String, String>,
}

/// Types of ecosystem services
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EcosystemServiceType {
    /// Compute services (ToadStool)
    Compute,
    /// Storage services (NestGate)
    Storage,
    /// Communication services (SongBird)
    Communication,
    /// AI services (Squirrel)
    AI,
    /// BioMe services (biomeOS)
    BioMe,
    /// Security services (BearDog)
    Security,
    /// Core services (BearDog Core)
    Core,
    /// Custom service type with name
    Custom(String),
}

/// Ecosystem service health information
#[derive(Debug, Clone)]
pub struct EcosystemServiceHealth {
    /// Unique identifier for the service
    pub service_id: String,
    /// Current health status of the service
    pub health_status: HealthStatus,
    /// When the last health check was performed
    pub last_health_check: chrono::DateTime<chrono::Utc>,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Availability percentage (0-100)
    pub availability_percentage: f64,
    /// Error rate percentage (0-100)
    pub error_rate_percentage: f64,
}

/// Discovery configuration
#[derive(Debug, Clone)]
pub struct EcosystemDiscoveryConfig {
    /// Interval between discovery attempts in seconds
    pub discovery_interval_seconds: u64,
    /// Interval between health checks in seconds
    pub health_check_interval_seconds: u64,
    /// Timeout for service operations in seconds
    pub service_timeout_seconds: u64,
    /// Maximum number of discovery attempts before giving up
    pub max_discovery_attempts: u32,
    /// List of ecosystems to discover services for
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

impl<T> EcosystemDiscovery<T> {
    /// Create a new ecosystem discovery service
    pub async fn new(core: Arc<T>) -> BearDogResult<Self> {
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
        core: Arc<T>,
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
                            reason: format!("Health check failed: {e}"),
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
            registration_id: uuid::Uuid::new_v4().to_string(),
            ecosystem_id: "beardog".to_string(),
            instance_id: "beardog-universal-001".to_string(),
            endpoints: ServiceEndpoints::default(),
            capabilities: self.get_beardog_universal_capabilities(),
            registered_at: chrono::Utc::now(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(24)), // Registration expires in 24 hours
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
    pub async fn start_background_discovery(&self) -> BearDogResult<()> {
        debug!("🔄 Starting background discovery service");

        // Use core instance for discovery operations
        let _core_ref = Arc::clone(&self.core);

        // STEP 1: Try to connect to SongBird discovery service
        if (self.try_connect_to_songbird_discovery().await).is_ok() {
            info!("✅ Connected to SongBird discovery service - delegating to SongBird");
            // Delegate all discovery to SongBird
            return Ok(());
        }

        // STEP 2: SongBird unavailable - use local fallback
        info!("⚠️  SongBird discovery unavailable - using local fallback");

        // Implement background discovery with core integration
        let mut discovered_services = self.discovered_services.write().await;

        // LOCAL FALLBACK ONLY: Basic service registration for when SongBird is unavailable
        let fallback_service = EcosystemService {
            service_id: "local-fallback".to_string(),
            ecosystem_id: "beardog-local".to_string(),
            instance_id: "fallback-instance".to_string(),
            service_type: EcosystemServiceType::Core,
            endpoints: ServiceEndpoints {
                primary: "http://localhost:8080".to_string(),
                health: "http://localhost:8080/health".to_string(),
                metrics: Some("http://localhost:8080/metrics".to_string()),
                admin: None,
                events: None,
                custom: HashMap::new(),
            },
            capabilities: vec![Capability {
                id: "local_fallback".to_string(),
                name: "local_fallback".to_string(),
                category: CapabilityCategory::Security,
                description: "Local fallback when SongBird unavailable".to_string(),
                qos: QualityOfService {
                    avg_response_time_ms: 50,
                    availability_percent: 99.9,
                    throughput: None,
                    scalability: ScalabilityInfo {
                        min_instances: 1,
                        max_instances: 1,
                        auto_scaling: false,
                    },
                },
                attributes: HashMap::new(),
                resource_requirements: ResourceRequirements::default(),
            }],
            discovery_time: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        discovered_services.insert(fallback_service.service_id.clone(), fallback_service);

        info!("✅ Local fallback discovery service started");
        Ok(())
    }

    /// Try to connect to SongBird discovery service
    async fn try_connect_to_songbird_discovery(&self) -> BearDogResult<()> {
        // TODO: Implement actual SongBird discovery service connection
        // This should:
        // 1. Check if SongBird discovery service is available
        // 2. Establish connection to SongBird
        // 3. Register this BearDog instance with SongBird
        // 4. Set up event handlers for SongBird discovery events

        // For now, return error to indicate SongBird is unavailable
        // This forces the use of local fallback
        Err(BearDogError::Network {
            message: "SongBird discovery service not available - using local fallback".to_string(),
        })
    }

    /// Discover services for a specific ecosystem
    pub async fn discover_ecosystem_services(
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

    /// Register with an ecosystem - tries SongBird first, then local fallback
    pub async fn register_with_ecosystem(
        &self,
        ecosystem_id: &str,
        registration: &EcosystemRegistration,
    ) -> BearDogResult<()> {
        info!("📝 Registering with ecosystem: {}", ecosystem_id);

        // STEP 1: Try to register with SongBird discovery service
        if (self.try_connect_to_songbird_discovery().await).is_ok() {
            info!("✅ Registering with SongBird discovery service");
            // TODO: Delegate registration to SongBird
            return Ok(());
        }

        // STEP 2: SongBird unavailable - use local fallback registration
        info!("⚠️  SongBird unavailable - using local fallback registration");

        // Use core instance for registration operations
        let _core_ref = Arc::clone(&self.core);

        // Create a service entry for this registration
        let service = EcosystemService {
            service_id: format!("registered-{ecosystem_id}"),
            ecosystem_id: ecosystem_id.to_string(),
            instance_id: registration.instance_id.clone(),
            service_type: EcosystemServiceType::Custom("registered".to_string()),
            endpoints: registration.endpoints.clone(),
            capabilities: registration.capabilities.clone(),
            discovery_time: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        // Store the registered service
        let mut discovered_services = self.discovered_services.write().await;
        discovered_services.insert(service.service_id.clone(), service);

        info!(
            "✅ Successfully registered with ecosystem: {}",
            ecosystem_id
        );
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
    /// Filter by ecosystem ID
    pub ecosystem_id: Option<String>,
    /// Filter by service type
    pub service_type: Option<EcosystemServiceType>,
}
