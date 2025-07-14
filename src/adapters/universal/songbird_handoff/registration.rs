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
use tracing::{debug, info};

use super::super::capability_manager::CapabilityManager;
use super::super::traits::{
    Capability, CapabilityCategory, QualityOfService, ResourceRequirements,
};
use super::client::SongBirdDiscoveryClient;
use super::types::{
    RegistrationStatus, RegistrationState, AdvertisedService, ServiceEndpoint, LoadBalancerConfig,
    OrchestrationMetadata, RoutingRule, RoutingCondition, RoutingAction, ScalingPolicy,
    ScalingMetric, ScalingAction, MonitoringConfig, CustomMetric, MetricType, AlertRule,
    AlertSeverity, SongBirdHandoffConfig,
};
use crate::{BearDogCore, BearDogResult};

/// Universal SongBird Registration Manager
///
/// Handles the registration of any ecosystem component with SongBird's
/// discovery and orchestration platform using universal patterns.
pub struct SongBirdRegistrationManager {
    /// Universal ecosystem component core
    core: Arc<BearDogCore>,

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

impl SongBirdRegistrationManager {
    /// Create a new universal registration manager
    pub async fn new(
        core: Arc<BearDogCore>,
        capability_manager: Arc<CapabilityManager>,
        config: SongBirdHandoffConfig,
    ) -> BearDogResult<Self> {
        info!("🔗 Initializing Universal SongBird Registration Manager");

        // Create SongBird client
        let client = Arc::new(
            SongBirdDiscoveryClient::new(
                config.songbird_endpoint.clone(),
                config.api_key.clone(),
            )
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
            status.registration_id = registration_result.registration_id;
            status.status = RegistrationState::Registered;
            status.last_registration = chrono::Utc::now();
            status.consecutive_failures = 0;
        }

        // Store advertised service
        {
            let mut services = self.advertised_services.write().await;
            services.insert(advertised_service.service_id.clone(), advertised_service);
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
        genetic_capabilities: &HashMap<String, super::super::capability_manager::GeneticCapabilityProfile>,
        emergent_capabilities: &HashMap<String, super::super::capability_manager::EmergentCapability>,
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
            all_capabilities.extend(self.convert_genetic_capabilities_to_universal(genetic_profile));
        }

        // Add emergent capabilities (converted to universal format)
        for emergent_capability in emergent_capabilities.values() {
            all_capabilities.push(self.convert_emergent_capability_to_universal(emergent_capability));
        }

        // Create universal service endpoints
        let endpoints = self.create_universal_service_endpoints().await?;

        // Create health check URL
        let health_check_url = format!("{}://{}/health", 
            endpoints.first().map(|e| e.protocol.as_str()).unwrap_or("http"),
            endpoints.first().map(|e| format!("{}:{}", e.address, e.port)).unwrap_or_else(|| "localhost:8080".to_string())
        );

        // Create discovery tags (universal ecosystem tags)
        let mut discovery_tags = self.config.discovery_tags.clone();
        discovery_tags.extend(vec![
            "ecosystem".to_string(),
            "universal".to_string(),
            "primal-provider".to_string(),
        ]);

        // Create load balancer configuration
        let load_balancer_config = LoadBalancerConfig {
            algorithm: self.config.load_balancer_algorithm.clone(),
            health_check_interval_seconds: self.config.health_check_interval_seconds,
            max_retries: self.config.max_registration_retries,
            timeout_seconds: self.config.registration_timeout_seconds,
            circuit_breaker_enabled: self.config.enable_circuit_breaker,
        };

        // Create orchestration metadata
        let orchestration_metadata = self.create_universal_orchestration_metadata(&all_capabilities).await?;

        Ok(AdvertisedService {
            service_id,
            service_name: "Universal Ecosystem Component".to_string(),
            capabilities: all_capabilities,
            endpoints,
            health_check_url,
            discovery_tags,
            load_balancer_config,
            orchestration_metadata,
        })
    }

    /// Get universal core capabilities
    fn get_universal_core_capabilities(&self) -> Vec<Capability> {
        vec![
            Capability {
                id: "universal.health-check".to_string(),
                name: "Universal Health Check".to_string(),
                description: "Universal health check capability for any ecosystem component".to_string(),
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
                description: format!("Universal genetic trait: {:?} (expression: {:.2})", 
                    trait_obj.trait_type, trait_obj.expression_level),
                category: CapabilityCategory::Compute,
                attributes: HashMap::new(),
                qos: QualityOfService::default(),
                resource_requirements: ResourceRequirements::default(),
            });
        }

        // Convert evolved capabilities to universal format (these are capability IDs)
        for capability_id in &genetic_profile.evolved_capabilities {
            capabilities.push(Capability {
                id: format!("universal.genetic.{}", capability_id),
                name: format!("Universal Evolved {}", capability_id),
                description: format!("Universal evolved genetic capability: {}", capability_id),
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
            protocol: "https".to_string(),
            address: "0.0.0.0".to_string(),
            port: 8443,
            path: Some("/api/v1".to_string()),
            weight: 100,
            health_check: true,
        });

        // gRPC endpoint for high-performance communication
        endpoints.push(ServiceEndpoint {
            protocol: "grpc".to_string(),
            address: "0.0.0.0".to_string(),
            port: 9443,
            path: None,
            weight: 80,
            health_check: true,
        });

        // Metrics endpoint for monitoring
        endpoints.push(ServiceEndpoint {
            protocol: "http".to_string(),
            address: "0.0.0.0".to_string(),
            port: 9090,
            path: Some("/metrics".to_string()),
            weight: 10,
            health_check: false,
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
                rule_id: "universal-api-route".to_string(),
                condition: RoutingCondition::PathPrefix("/api/v1".to_string()),
                action: RoutingAction::RouteToEndpoint("https-endpoint".to_string()),
                priority: 100,
            },
            RoutingRule {
                rule_id: "universal-grpc-route".to_string(),
                condition: RoutingCondition::Header {
                    name: "content-type".to_string(),
                    value: "application/grpc".to_string(),
                },
                action: RoutingAction::RouteToEndpoint("grpc-endpoint".to_string()),
                priority: 90,
            },
        ];

        // Create universal scaling policies
        let scaling_policies = vec![
            ScalingPolicy {
                policy_id: "universal-cpu-scaling".to_string(),
                metric: ScalingMetric::CpuUtilization,
                threshold: 80.0,
                action: ScalingAction::AutoScale { min: 1, max: 10 },
                cooldown_seconds: 300,
            },
            ScalingPolicy {
                policy_id: "universal-request-scaling".to_string(),
                metric: ScalingMetric::RequestRate,
                threshold: 1000.0,
                action: ScalingAction::AutoScale { min: 1, max: 5 },
                cooldown_seconds: 180,
            },
        ];

        // Create universal monitoring configuration
        let monitoring_config = MonitoringConfig {
            metrics_enabled: true,
            tracing_enabled: true,
            logging_level: "info".to_string(),
            custom_metrics: vec![
                CustomMetric {
                    name: "universal_requests_total".to_string(),
                    metric_type: MetricType::Counter,
                    description: "Total number of requests processed".to_string(),
                    tags: HashMap::new(),
                },
                CustomMetric {
                    name: "universal_capability_usage".to_string(),
                    metric_type: MetricType::Gauge,
                    description: "Current capability usage percentage".to_string(),
                    tags: HashMap::new(),
                },
            ],
            alert_rules: vec![
                AlertRule {
                    rule_id: "universal-high-error-rate".to_string(),
                    condition: "error_rate > 0.05".to_string(),
                    threshold: 0.05,
                    severity: AlertSeverity::Warning,
                    notification_channels: vec!["ecosystem-alerts".to_string()],
                },
            ],
        };

        Ok(OrchestrationMetadata {
            routing_rules,
            scaling_policies,
            affinity_rules: Vec::new(),
            security_policies: vec!["universal-security".to_string()],
            monitoring_config,
        })
    }

    /// Start heartbeat task for maintaining registration
    async fn start_heartbeat_task(&self) -> BearDogResult<()> {
        debug!("💓 Starting universal heartbeat task");
        
        // TODO: Implement heartbeat task using tokio::spawn
        // This would periodically send heartbeat to SongBird to maintain registration
        
        Ok(())
    }

    /// Start health monitoring task
    async fn start_health_monitoring_task(&self) -> BearDogResult<()> {
        debug!("🏥 Starting universal health monitoring task");
        
        // TODO: Implement health monitoring task using tokio::spawn
        // This would periodically check component health and update SongBird
        
        Ok(())
    }

    /// Update capability advertisement
    pub async fn update_capability_advertisement(&self) -> BearDogResult<()> {
        debug!("🔄 Updating universal capability advertisement");
        
        // TODO: Implement capability advertisement update
        // This would detect changes in capabilities and update SongBird registration
        
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
} 