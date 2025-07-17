//! Universal SongBird Registration
//!
//! **Universal service registration and discovery logic**
//!
//! This module provides universal service registration patterns that work with
//! any ecosystem component implementing the PrimalProvider trait. It handles
//! the registration of capabilities, endpoints, and metadata with SongBird.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::super::capability_manager::CapabilityManager;
use super::super::traits::{
    Capability, CapabilityCategory, MonitoringConfig, QualityOfService, ResourceRequirements,
};
use super::client::SongBirdDiscoveryClient;
use super::types::{
    AdvertisedService, AuthMethod, CircuitBreakerConfig, EcosystemServiceRegistration,
    EndpointType, HealthCheckConfig, LoadBalancerConfig, LoadBalancingAlgorithm,
    OrchestrationMetadata, PerformanceCapabilities, PerformanceMetrics, PrimalType,
    RegistrationState, RegistrationStatus, ResourceSpec, RoutingRule, ScalingAction, ScalingPolicy,
    SecurityConfig, SecurityLevel, ServiceCapabilities, ServiceEndpoint, ServiceEndpoints,
    ServiceHealth, SongBirdHandoffConfig,
};
use beardog_errors::BearDogResult;

/// Universal SongBird Registration Manager
///
/// Handles the registration of any ecosystem component with SongBird's
/// discovery and orchestration platform using universal patterns.
pub struct SongBirdRegistrationManager<T> {
    /// Universal ecosystem component core
    core: Arc<T>,

    /// Capability manager for dynamic capability advertisement
    capability_manager: Arc<CapabilityManager>,

    /// SongBird client for discovery registration
    client: Arc<SongBirdDiscoveryClient>,

    /// Registration status tracking
    registration_status: Arc<RwLock<RegistrationStatus>>,

    /// Advertised services tracking
    advertised_services: Arc<RwLock<HashMap<String, AdvertisedService>>>,

    /// Registration configuration
    config: SongBirdHandoffConfig,
}

impl<T: Send + Sync> SongBirdRegistrationManager<T> {
    /// Create a new universal registration manager
    pub async fn new(
        core: Arc<T>,
        capability_manager: Arc<CapabilityManager>,
        config: SongBirdHandoffConfig,
    ) -> BearDogResult<Self> {
        info!("🔗 Initializing Universal SongBird Registration Manager");

        // Create SongBird client
        let client = Arc::new(
            SongBirdDiscoveryClient::new(config.songbird_endpoint.clone(), config.api_key.clone())
                .await?,
        );

        // Initialize registration status
        let registration_status = Arc::new(RwLock::new(RegistrationStatus {
            registration_id: uuid::Uuid::new_v4().to_string(),
            status: RegistrationState::NotRegistered,
            last_registration: chrono::Utc::now(),
            last_heartbeat: chrono::Utc::now(),
            consecutive_failures: 0,
            next_retry: None,
        }));

        Ok(Self {
            core,
            capability_manager,
            client,
            registration_status,
            advertised_services: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }

    /// Register this ecosystem component with SongBird
    pub async fn register_with_songbird(&self) -> BearDogResult<()> {
        info!("📡 Registering ecosystem component with SongBird for universal discovery");

        // Update registration status
        {
            let mut status = self.registration_status.write().await;
            status.status = RegistrationState::Registering;
        }

        // Get current capabilities from capability manager
        let genetic_capabilities = self.capability_manager.get_genetic_capabilities().await?;
        let emergent_capabilities = self.capability_manager.get_emergent_capabilities().await?;
        let monitoring_status = self.capability_manager.get_monitoring_status().await?;

        // Create universal service advertisement
        let advertised_service = self
            .create_universal_service_advertisement(
                &genetic_capabilities,
                &emergent_capabilities,
                &monitoring_status,
            )
            .await?;

        // Register with SongBird
        let registration_result = self.client.register_service(&advertised_service).await?;

        // Update registration status
        {
            let mut status = self.registration_status.write().await;
            status.registration_id = registration_result.service_id.clone();
            status.status = RegistrationState::Active;
            status.last_registration = chrono::Utc::now();
            status.consecutive_failures = 0;
        }

        // Store advertised service
        {
            let mut services = self.advertised_services.write().await;
            services.insert(
                advertised_service.registration.service_id.clone(),
                advertised_service,
            );
        }

        // Start background tasks
        self.start_heartbeat_task().await?;
        self.start_health_monitoring_task().await?;

        info!("✅ Successfully registered ecosystem component with SongBird");
        info!("🔍 SongBird can now discover and route requests to this component");
        info!("⚖️ Load balancing, failover, and scaling managed by SongBird");

        Ok(())
    }

    /// Create universal service advertisement
    async fn create_universal_service_advertisement(
        &self,
        genetic_capabilities: &HashMap<
            String,
            super::super::capability_manager::GeneticCapabilityProfile,
        >,
        emergent_capabilities: &HashMap<
            String,
            super::super::capability_manager::EmergentCapability,
        >,
        _monitoring_status: &HashMap<String, super::super::capability_manager::CapabilityMonitor>,
    ) -> BearDogResult<AdvertisedService> {
        // Create universal service ID (ecosystem-agnostic)
        let service_id = format!("universal-component-{}", uuid::Uuid::new_v4());

        // Collect all capabilities for advertisement
        let mut all_capabilities = Vec::new();

        // Add universal core capabilities
        all_capabilities.extend(self.get_universal_core_capabilities());

        // Add genetic capabilities (converted to universal format)
        for genetic_profile in genetic_capabilities.values() {
            all_capabilities
                .extend(self.convert_genetic_capabilities_to_universal(genetic_profile));
        }

        // Add emergent capabilities (converted to universal format)
        for emergent_capability in emergent_capabilities.values() {
            all_capabilities
                .push(self.convert_emergent_capability_to_universal(emergent_capability));
        }

        // Create universal service endpoints
        let endpoints = self.create_universal_service_endpoints().await?;

        // Create health check URL
        let health_check_url = format!(
            "{}://{}/health",
            endpoints
                .first()
                .map(|e| e.protocol.as_str())
                .unwrap_or("http"),
            endpoints
                .first()
                .map(|e| e.url.clone())
                .unwrap_or_else(|| "localhost:8080".to_string())
        );

        // Create discovery tags (universal ecosystem tags)
        let mut discovery_tags = self.config.discovery_tags.clone();
        discovery_tags.extend(vec![
            "ecosystem".to_string(),
            "universal".to_string(),
            "primal-provider".to_string(),
        ]);

        // Create load balancer configuration
        let _load_balancer_config = LoadBalancerConfig {
            algorithm: self.config.load_balancer_algorithm.clone(),
            weight: 100,
            max_requests: 1000,
            circuit_breaker: CircuitBreakerConfig {
                failure_threshold: 5,
                timeout_seconds: 60,
                success_threshold: 3,
            },
        };

        // Create orchestration metadata
        let orchestration_metadata = self
            .create_universal_orchestration_metadata(&all_capabilities)
            .await?;

        Ok(AdvertisedService {
            registration: EcosystemServiceRegistration {
                service_id,
                primal_type: PrimalType::BearDog,
                biome_id: None,
                capabilities: ServiceCapabilities {
                    core: all_capabilities
                        .iter()
                        .filter(|c| c.category == CapabilityCategory::Security)
                        .map(|c| c.name.clone())
                        .collect(),
                    extended: all_capabilities
                        .iter()
                        .filter(|c| c.category != CapabilityCategory::Security)
                        .map(|c| c.name.clone())
                        .collect(),
                    integrations: vec![
                        "songbird".to_string(),
                        "ecosystem".to_string(),
                        "universal".to_string(),
                    ],
                    performance: PerformanceCapabilities {
                        latency_ms: Some(100),
                        throughput_rps: Some(1000),
                        max_concurrent_requests: Some(100),
                    },
                },
                endpoints: ServiceEndpoints {
                    health: health_check_url.clone(),
                    metrics: "http://0.0.0.0:9090/metrics".to_string(),
                    admin: "http://0.0.0.0:8080/admin".to_string(),
                    websocket: None,
                    primary: "http://0.0.0.0:8080/api/v1".to_string(),
                },
                resource_requirements: ResourceSpec {
                    cpu_cores: Some(1.0),
                    memory_mb: Some(512),
                    storage_mb: Some(1024),
                    network_mbps: Some(100),
                    gpu_units: None,
                },
                security_config: SecurityConfig {
                    auth_method: AuthMethod::ApiKey,
                    encryption_required: true,
                    security_level: SecurityLevel::High,
                    compliance: vec!["GDPR".to_string(), "HIPAA".to_string()],
                },
                health_check: HealthCheckConfig {
                    path: health_check_url,
                    interval_seconds: 30,
                    timeout_seconds: 5,
                    failure_threshold: 3,
                },
                metadata: HashMap::new(),
                registered_at: chrono::Utc::now(),
            },
            health: ServiceHealth {
                status: super::types::HealthStatus::Healthy,
                last_check: chrono::Utc::now(),
                metrics: PerformanceMetrics {
                    cpu_percent: 0.0,
                    memory_percent: 0.0,
                    latency_ms: 0,
                    requests_per_second: 0.0,
                    error_rate_percent: 0.0,
                },
                error_details: None,
            },
            load_balancer_config: LoadBalancerConfig {
                algorithm: LoadBalancingAlgorithm::RoundRobin,
                weight: 100,
                max_requests: 1000,
                circuit_breaker: CircuitBreakerConfig {
                    failure_threshold: 5,
                    timeout_seconds: 30,
                    success_threshold: 2,
                },
            },
            orchestration: orchestration_metadata,
        })
    }

    /// Get universal core capabilities
    fn get_universal_core_capabilities(&self) -> Vec<Capability> {
        vec![
            Capability {
                id: "universal.health-check".to_string(),
                name: "Universal Health Check".to_string(),
                description: "Universal health check capability for any ecosystem component"
                    .to_string(),
                category: CapabilityCategory::Monitoring,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            },
            Capability {
                id: "universal.metrics".to_string(),
                name: "Universal Metrics".to_string(),
                description: "Universal metrics collection capability".to_string(),
                category: CapabilityCategory::Monitoring,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            },
            Capability {
                id: "universal.service-discovery".to_string(),
                name: "Universal Service Discovery".to_string(),
                description: "Universal service discovery integration".to_string(),
                category: CapabilityCategory::Integration,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            },
        ]
    }

    /// Convert genetic capabilities to universal format
    fn convert_genetic_capabilities_to_universal(
        &self,
        genetic_profile: &super::super::capability_manager::GeneticCapabilityProfile,
    ) -> Vec<Capability> {
        let mut capabilities = Vec::new();

        // Convert inherited traits to universal capabilities
        for trait_obj in &genetic_profile.inherited_traits {
            capabilities.push(Capability {
                id: format!("universal.genetic.{}", trait_obj.trait_id),
                name: format!("Universal Genetic {:?}", trait_obj.trait_type),
                description: format!(
                    "Universal genetic trait: {:?} (expression: {:.2})",
                    trait_obj.trait_type, trait_obj.expression_level
                ),
                category: CapabilityCategory::Compute,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            });
        }

        // Convert evolved capabilities to universal format (these are capability IDs)
        for capability_id in &genetic_profile.evolved_capabilities {
            capabilities.push(Capability {
                id: format!("universal.genetic.{capability_id}"),
                name: format!("Universal Evolved {capability_id}"),
                description: format!("Universal evolved genetic capability: {capability_id}"),
                category: CapabilityCategory::Compute,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            });
        }

        capabilities
    }

    /// Convert emergent capability to universal format
    fn convert_emergent_capability_to_universal(
        &self,
        emergent: &super::super::capability_manager::EmergentCapability,
    ) -> Capability {
        Capability {
            id: format!("universal.emergent.{}", emergent.capability_id),
            name: format!("Universal Emergent {}", emergent.name),
            description: format!("Universal emergent capability: {}", emergent.description),
            category: CapabilityCategory::Compute,
            attributes: HashMap::new(),
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }

    /// Create universal service endpoints
    async fn create_universal_service_endpoints(&self) -> BearDogResult<Vec<ServiceEndpoint>> {
        let mut endpoints = Vec::new();

        // HTTP/HTTPS endpoint for universal communication
        endpoints.push(ServiceEndpoint {
            url: "https://0.0.0.0:8443/api/v1".to_string(),
            endpoint_type: EndpointType::Primary,
            protocol: "https".to_string(),
            port: 8443,
        });

        // gRPC endpoint for high-performance communication
        endpoints.push(ServiceEndpoint {
            url: "grpc://0.0.0.0:9443".to_string(),
            endpoint_type: EndpointType::Custom("grpc".to_string()),
            protocol: "grpc".to_string(),
            port: 9443,
        });

        // Metrics endpoint for monitoring
        endpoints.push(ServiceEndpoint {
            url: "http://0.0.0.0:9090/metrics".to_string(),
            endpoint_type: EndpointType::Metrics,
            protocol: "http".to_string(),
            port: 9090,
        });

        Ok(endpoints)
    }

    /// Create universal orchestration metadata
    async fn create_universal_orchestration_metadata(
        &self,
        _capabilities: &[Capability],
    ) -> BearDogResult<OrchestrationMetadata> {
        // Create universal routing rules
        let routing_rules = vec![
            RoutingRule {
                condition: "path_prefix == '/api/v1'".to_string(),
                target: "https-endpoint".to_string(),
                weight: 100,
            },
            RoutingRule {
                condition: "content-type == 'application/grpc'".to_string(),
                target: "grpc-endpoint".to_string(),
                weight: 90,
            },
        ];

        // Create universal scaling policies
        let scaling_policies = vec![
            ScalingPolicy {
                metric: "cpu_utilization".to_string(),
                threshold: 80.0,
                action: ScalingAction::ScaleUp(10),
            },
            ScalingPolicy {
                metric: "request_rate".to_string(),
                threshold: 1000.0,
                action: ScalingAction::ScaleUp(5),
            },
        ];

        // Create universal monitoring configuration
        let _monitoring_config = MonitoringConfig {
            metrics_enabled: true,
            log_level: "info".to_string(),
            health_check_interval_seconds: 60,
        };

        Ok(OrchestrationMetadata {
            routing_rules,
            scaling_policies,
            affinity_rules: Vec::new(),
        })
    }

    /// Start heartbeat task for maintaining registration
    async fn start_heartbeat_task(&self) -> BearDogResult<()> {
        debug!("💓 Starting universal heartbeat task");

        let client = Arc::clone(&self.client);
        let primal_id = self.registration_status.read().await.registration_id.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                match client.send_heartbeat().await {
                    Ok(_) => {
                        debug!("💓 Heartbeat sent successfully for primal: {}", primal_id);
                    }
                    Err(e) => {
                        warn!("❌ Heartbeat failed for primal {}: {}", primal_id, e);
                        // In production, might want to implement exponential backoff
                    }
                }
            }
        });

        Ok(())
    }

    /// Start health monitoring task
    async fn start_health_monitoring_task(&self) -> BearDogResult<()> {
        debug!("🏥 Starting universal health monitoring task");

        let client = Arc::clone(&self.client);
        let primal_id = self.registration_status.read().await.registration_id.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                // Perform health check
                match Self::perform_health_check(&primal_id).await {
                    Ok(health_status) => {
                        // Report health status to SongBird
                        if let Err(e) = client.update_health_status(&primal_id, &health_status).await {
                            warn!("Failed to update health status: {}", e);
                        }
                    }
                    Err(e) => {
                        warn!("Health check failed: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// Update capability advertisement
    pub async fn update_capability_advertisement(&self) -> BearDogResult<()> {
        debug!("🔄 Updating universal capability advertisement");

        // Get current capabilities from the provider
        let capabilities = self.get_current_capabilities().await?;
        
        // Update SongBird with new capabilities
        match self.client.update_capabilities(&self.registration_status.read().await.registration_id, &capabilities).await {
            Ok(_) => {
                info!("✅ Capability advertisement updated successfully");
            }
            Err(e) => {
                warn!("❌ Failed to update capability advertisement: {}", e);
                return Err(e);
            }
        }

        Ok(())
    }

    /// Get registration status
    pub async fn get_registration_status(&self) -> RegistrationStatus {
        self.registration_status.read().await.clone()
    }

    /// Get advertised services
    pub async fn get_advertised_services(&self) -> HashMap<String, AdvertisedService> {
        self.advertised_services.read().await.clone()
    }

    /// Send heartbeat to SongBird to maintain registration
    pub async fn send_heartbeat(&self) -> BearDogResult<()> {
        debug!("💓 Sending heartbeat to SongBird");

        match self.client.send_heartbeat(&self.registration_status.read().await.registration_id).await {
            Ok(_) => {
                debug!("💓 Heartbeat sent successfully");
                Ok(())
            }
            Err(e) => {
                warn!("❌ Heartbeat failed: {}", e);
                Err(e)
            }
        }
    }

    /// Deregister from SongBird
    pub async fn deregister(&self) -> BearDogResult<()> {
        info!("🔄 Deregistering from SongBird");

        match self.client.deregister(&self.registration_status.read().await.registration_id).await {
            Ok(_) => {
                info!("✅ Successfully deregistered from SongBird");
                Ok(())
            }
            Err(e) => {
                warn!("❌ Failed to deregister from SongBird: {}", e);
                Err(e)
            }
        }
    }

    /// Perform health check for the primal
    async fn perform_health_check(primal_id: &String) -> BearDogResult<super::types::HealthStatus> {
        let mut health_status = super::types::HealthStatus::Healthy;
        
        // Check component health
        if let Err(e) = Self::check_component_health().await {
            health_status.mark_unhealthy(&format!("Component health check failed: {}", e));
        }
        
        // Check resource utilization
        if let Err(e) = Self::check_resource_utilization().await {
            health_status.mark_degraded(&format!("Resource utilization check failed: {}", e));
        }
        
        debug!("🏥 Health check completed for primal: {}, status: {:?}", primal_id, health_status);
        Ok(health_status)
    }

    /// Check component health
    async fn check_component_health() -> BearDogResult<()> {
        // Check if core components are responding
        // This would check BearDog core, HSM, database connections, etc.
        debug!("🔍 Checking component health");
        Ok(())
    }

    /// Check resource utilization
    async fn check_resource_utilization() -> BearDogResult<()> {
        // Check memory usage, CPU usage, disk space, etc.
        debug!("📊 Checking resource utilization");
        Ok(())
    }

    /// Get current capabilities of the primal
    async fn get_current_capabilities(&self) -> BearDogResult<Vec<Capability>> {
        debug!("🔍 Getting current capabilities");
        
        // This would query the actual provider for its current capabilities
        // For now, return a basic set of security capabilities
        Ok(Self::get_default_capabilities())
    }

    /// Get default BearDog capabilities
    fn get_default_capabilities() -> Vec<Capability> {
        vec![
            Capability::new(
                "security.authentication".to_string(),
                "Authentication".to_string(),
                "User authentication and authorization".to_string(),
                CapabilityCategory::Security,
            ),
            Capability::new(
                "security.encryption".to_string(),
                "Encryption".to_string(),
                "Data encryption and decryption".to_string(),
                CapabilityCategory::Security,
            ),
            Capability::new(
                "security.key_management".to_string(),
                "Key Management".to_string(),
                "Cryptographic key management".to_string(),
                CapabilityCategory::Security,
            ),
            Capability::new(
                "security.audit".to_string(),
                "Audit".to_string(),
                "Security audit and compliance".to_string(),
                CapabilityCategory::Security,
            ),
            Capability::new(
                "security.threat_detection".to_string(),
                "Threat Detection".to_string(),
                "Threat detection and analysis".to_string(),
                CapabilityCategory::Security,
            ),
        ]
    }
}
