//! SongBird Discovery and Orchestration Handoff
//!
//! **BearDog → SongBird Integration for Universal Service Discovery**
//!
//! This module implements the handoff from BearDog's capability advertisement system
//! to SongBird's discovery and orchestration platform. BearDog focuses on capability
//! generation and management, while SongBird handles ecosystem-wide discovery,
//! routing, load balancing, and service orchestration.
//!
//! ## Integration Flow
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    BearDog Security Provider                │
//! │  ┌─────────────────┐  ┌──────────────────────────────────┐  │
//! │  │   Capability    │  │     Comprehensive Capability     │  │
//! │  │ Advertisement   │→ │       Management System          │  │
//! │  │    System       │  │                                  │  │
//! │  └─────────────────┘  └──────────────────────────────────┘  │
//! └─────────────────────────┬───────────────────────────────────┘
//!                           │ Handoff via PrimalProvider
//!                           ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │                 SongBird Orchestrator                       │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
//! │  │   Service   │  │   Request   │  │    Load Balancer    │  │
//! │  │ Discovery   │  │   Routing   │  │   & Orchestration   │  │
//! │  └─────────────┘  └─────────────┘  └─────────────────────┘  │
//! └─────────────────────────────────────────────────────────────┘
//!                           │
//!                           ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │            Ecosystem-Wide Service Mesh                     │
//! │    ToadStool  │  NestGate  │  Squirrel  │  biomeOS        │
//! └─────────────────────────────────────────────────────────────┘
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use super::capability_manager::CapabilityManager;
use super::traits::*;
use crate::{BearDogCore, BearDogError, BearDogResult};

/// SongBird Discovery and Orchestration Handoff Manager
///
/// This manager handles the integration between BearDog's capability advertisement
/// system and SongBird's discovery and orchestration platform.
pub struct SongBirdHandoffManager {
    /// BearDog core reference
    core: Arc<BearDogCore>,

    /// Capability manager reference
    capability_manager: Arc<CapabilityManager>,

    /// SongBird client for discovery registration
    songbird_client: Arc<SongBirdDiscoveryClient>,

    /// Registration status tracking
    registration_status: Arc<RwLock<RegistrationStatus>>,

    /// Advertised services tracking
    advertised_services: Arc<RwLock<HashMap<String, AdvertisedService>>>,

    /// Handoff configuration
    config: SongBirdHandoffConfig,
}

/// SongBird discovery client for capability registration
pub struct SongBirdDiscoveryClient {
    /// SongBird endpoint
    endpoint: String,

    /// API credentials
    api_key: String,

    /// HTTP client
    client: reqwest::Client,

    /// Registration health status
    health_status: Arc<RwLock<ServiceHealth>>,
}

/// Registration status with SongBird
#[derive(Debug, Clone)]
pub struct RegistrationStatus {
    pub registration_id: String,
    pub status: RegistrationState,
    pub last_registration: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
    pub consecutive_failures: u32,
    pub next_retry: Option<DateTime<Utc>>,
}

/// Registration state enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum RegistrationState {
    NotRegistered,
    Registering,
    Registered,
    Failed,
    Expired,
}

/// Advertised service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvertisedService {
    pub service_id: String,
    pub service_name: String,
    pub capabilities: Vec<Capability>,
    pub endpoints: Vec<ServiceEndpoint>,
    pub health_check_url: String,
    pub discovery_tags: Vec<String>,
    pub load_balancer_config: LoadBalancerConfig,
    pub orchestration_metadata: OrchestrationMetadata,
}

/// Service endpoint for SongBird routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub protocol: String,
    pub address: String,
    pub port: u16,
    pub path: Option<String>,
    pub weight: u32,
    pub health_check: bool,
}

/// Load balancer configuration for SongBird
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    pub algorithm: LoadBalancingAlgorithm,
    pub health_check_interval_seconds: u64,
    pub max_retries: u32,
    pub timeout_seconds: u64,
    pub circuit_breaker_enabled: bool,
}

/// Load balancing algorithms supported by SongBird
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    IpHash,
    Random,
    PerformanceBased,
}

/// Orchestration metadata for SongBird routing decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationMetadata {
    pub routing_rules: Vec<RoutingRule>,
    pub scaling_policies: Vec<ScalingPolicy>,
    pub affinity_rules: Vec<AffinityRule>,
    pub security_policies: Vec<String>,
    pub monitoring_config: MonitoringConfig,
}

/// Routing rule for request distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    pub rule_id: String,
    pub condition: RoutingCondition,
    pub action: RoutingAction,
    pub priority: u32,
}

/// Routing condition for rule matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingCondition {
    PathPrefix(String),
    Header { name: String, value: String },
    QueryParam { name: String, value: String },
    ClientIp(String),
    RequestSize { min: Option<u64>, max: Option<u64> },
    Capability(String),
}

/// Routing action to take when condition matches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingAction {
    RouteToEndpoint(String),
    RouteToCapability(String),
    LoadBalance(Vec<String>),
    Reject { code: u16, message: String },
    RateLimitApply { requests_per_second: u64 },
}

/// Scaling policy for dynamic service scaling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPolicy {
    pub policy_id: String,
    pub metric: ScalingMetric,
    pub threshold: f64,
    pub action: ScalingAction,
    pub cooldown_seconds: u64,
}

/// Metrics for scaling decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingMetric {
    CpuUtilization,
    MemoryUtilization,
    RequestRate,
    ResponseTime,
    ErrorRate,
    QueueLength,
}

/// Scaling actions SongBird can take
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingAction {
    ScaleUp { instances: u32 },
    ScaleDown { instances: u32 },
    AutoScale { min: u32, max: u32 },
}

/// Affinity rules for service placement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffinityRule {
    pub rule_id: String,
    pub affinity_type: AffinityType,
    pub target: String,
    pub weight: f64,
}

/// Types of affinity rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AffinityType {
    NodeAffinity,
    ServiceAffinity,
    AntiAffinity,
    ZoneAffinity,
    RegionAffinity,
}

/// Monitoring configuration for SongBird observability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_enabled: bool,
    pub tracing_enabled: bool,
    pub logging_level: String,
    pub custom_metrics: Vec<CustomMetric>,
    pub alert_rules: Vec<AlertRule>,
}

/// Custom metric definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetric {
    pub name: String,
    pub metric_type: MetricType,
    pub description: String,
    pub tags: HashMap<String, String>,
}

/// Metric types for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

/// Alert rule for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub rule_id: String,
    pub condition: String,
    pub threshold: f64,
    pub severity: AlertSeverity,
    pub notification_channels: Vec<String>,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Service health status
#[derive(Debug, Clone)]
pub struct ServiceHealth {
    pub status: HealthStatus,
    pub last_check: DateTime<Utc>,
    pub response_time_ms: u64,
    pub error_count: u64,
    pub uptime_percentage: f64,
}

/// Configuration for SongBird handoff
#[derive(Debug, Clone)]
pub struct SongBirdHandoffConfig {
    pub songbird_endpoint: String,
    pub api_key: String,
    pub registration_timeout_seconds: u64,
    pub heartbeat_interval_seconds: u64,
    pub max_registration_retries: u32,
    pub enable_auto_reregistration: bool,
    pub discovery_tags: Vec<String>,
    pub load_balancer_algorithm: LoadBalancingAlgorithm,
    pub enable_circuit_breaker: bool,
    pub health_check_interval_seconds: u64,
}

impl Default for SongBirdHandoffConfig {
    fn default() -> Self {
        Self {
            songbird_endpoint: std::env::var("SONGBIRD_ENDPOINT")
                .unwrap_or_else(|_| "https://songbird.orchestrator.internal".to_string()),
            api_key: std::env::var("SONGBIRD_API_KEY")
                .unwrap_or_else(|_| "beardog-default-key".to_string()),
            registration_timeout_seconds: 30,
            heartbeat_interval_seconds: 60,
            max_registration_retries: 5,
            enable_auto_reregistration: true,
            discovery_tags: vec![
                "security".to_string(),
                "beardog".to_string(),
                "primal-provider".to_string(),
            ],
            load_balancer_algorithm: LoadBalancingAlgorithm::PerformanceBased,
            enable_circuit_breaker: true,
            health_check_interval_seconds: 30,
        }
    }
}

impl SongBirdHandoffManager {
    /// Create a new SongBird handoff manager
    pub async fn new(
        core: Arc<BearDogCore>,
        capability_manager: Arc<CapabilityManager>,
        config: SongBirdHandoffConfig,
    ) -> BearDogResult<Self> {
        info!("🎼 Initializing SongBird Discovery and Orchestration Handoff");

        // Create SongBird discovery client
        let songbird_client = Arc::new(
            SongBirdDiscoveryClient::new(config.songbird_endpoint.clone(), config.api_key.clone())
                .await?,
        );

        let manager = Self {
            core,
            capability_manager,
            songbird_client,
            registration_status: Arc::new(RwLock::new(RegistrationStatus {
                registration_id: Uuid::new_v4().to_string(),
                status: RegistrationState::NotRegistered,
                last_registration: Utc::now(),
                last_heartbeat: Utc::now(),
                consecutive_failures: 0,
                next_retry: None,
            })),
            advertised_services: Arc::new(RwLock::new(HashMap::new())),
            config,
        };

        info!("✅ SongBird handoff manager initialized successfully");
        Ok(manager)
    }

    /// Register BearDog with SongBird for discovery and orchestration
    pub async fn register_with_songbird(&self) -> BearDogResult<()> {
        info!("📡 Registering BearDog with SongBird for ecosystem discovery");

        // Update registration status
        {
            let mut status = self.registration_status.write().await;
            status.status = RegistrationState::Registering;
        }

        // Get current capabilities from capability manager
        let genetic_capabilities = self.capability_manager.get_genetic_capabilities().await?;
        let emergent_capabilities = self.capability_manager.get_emergent_capabilities().await?;
        let monitoring_status = self.capability_manager.get_monitoring_status().await?;

        // Create comprehensive service advertisement
        let advertised_service = self
            .create_service_advertisement(
                &genetic_capabilities,
                &emergent_capabilities,
                &monitoring_status,
            )
            .await?;

        // Register with SongBird
        let registration_result = self
            .songbird_client
            .register_service(&advertised_service)
            .await?;

        // Update registration status
        {
            let mut status = self.registration_status.write().await;
            status.registration_id = registration_result.registration_id;
            status.status = RegistrationState::Registered;
            status.last_registration = Utc::now();
            status.consecutive_failures = 0;
        }

        // Store advertised service
        {
            let mut services = self.advertised_services.write().await;
            services.insert(advertised_service.service_id.clone(), advertised_service);
        }

        // Start heartbeat and health monitoring
        self.start_heartbeat_task().await?;
        self.start_health_monitoring_task().await?;

        info!("✅ Successfully registered BearDog with SongBird for discovery and orchestration");
        info!("🔍 SongBird can now discover and route requests to BearDog security capabilities");
        info!("⚖️ Load balancing, failover, and scaling will be managed by SongBird");

        Ok(())
    }

    /// Create comprehensive service advertisement for SongBird
    async fn create_service_advertisement(
        &self,
        genetic_capabilities: &HashMap<String, super::capability_manager::GeneticCapabilityProfile>,
        emergent_capabilities: &HashMap<String, super::capability_manager::EmergentCapability>,
        _monitoring_status: &HashMap<String, super::capability_manager::CapabilityMonitor>,
    ) -> BearDogResult<AdvertisedService> {
        let service_id = "beardog-security-provider".to_string();

        // Collect all capabilities for advertisement
        let mut all_capabilities = Vec::new();

        // Add core security capabilities
        all_capabilities.extend(self.get_core_security_capabilities());

        // Add genetic capabilities
        for genetic_profile in genetic_capabilities.values() {
            all_capabilities.extend(self.convert_genetic_capabilities_to_standard(genetic_profile));
        }

        // Add emergent capabilities
        for emergent_capability in emergent_capabilities.values() {
            all_capabilities
                .push(self.convert_emergent_capability_to_standard(emergent_capability));
        }

        // Create service endpoints
        let endpoints = vec![
            ServiceEndpoint {
                protocol: "HTTPS".to_string(),
                address: "beardog.security.internal".to_string(),
                port: 8443,
                path: Some("/api/v1".to_string()),
                weight: 100,
                health_check: true,
            },
            ServiceEndpoint {
                protocol: "gRPC".to_string(),
                address: "beardog.security.internal".to_string(),
                port: 9443,
                path: None,
                weight: 80,
                health_check: true,
            },
            ServiceEndpoint {
                protocol: "BSTP".to_string(), // BearDog Secure Tunnel Protocol
                address: "beardog.security.internal".to_string(),
                port: 10443,
                path: None,
                weight: 90,
                health_check: true,
            },
        ];

        // Create load balancer configuration
        let load_balancer_config = LoadBalancerConfig {
            algorithm: self.config.load_balancer_algorithm.clone(),
            health_check_interval_seconds: self.config.health_check_interval_seconds,
            max_retries: 3,
            timeout_seconds: 30,
            circuit_breaker_enabled: self.config.enable_circuit_breaker,
        };

        // Create orchestration metadata with comprehensive routing rules
        let orchestration_metadata = OrchestrationMetadata {
            routing_rules: vec![
                // Route encryption requests to high-performance endpoints
                RoutingRule {
                    rule_id: "encryption-routing".to_string(),
                    condition: RoutingCondition::Capability("security.encrypt".to_string()),
                    action: RoutingAction::RouteToEndpoint("beardog-crypto-optimized".to_string()),
                    priority: 100,
                },
                // Route authentication requests to BSTP for low latency
                RoutingRule {
                    rule_id: "auth-routing".to_string(),
                    condition: RoutingCondition::Capability("security.authenticate".to_string()),
                    action: RoutingAction::RouteToEndpoint("beardog-bstp".to_string()),
                    priority: 90,
                },
                // Route audit requests to standard HTTPS
                RoutingRule {
                    rule_id: "audit-routing".to_string(),
                    condition: RoutingCondition::PathPrefix("/api/v1/audit".to_string()),
                    action: RoutingAction::RouteToEndpoint("beardog-https".to_string()),
                    priority: 70,
                },
            ],
            scaling_policies: vec![
                // Auto-scale based on security request load
                ScalingPolicy {
                    policy_id: "security-load-scaling".to_string(),
                    metric: ScalingMetric::RequestRate,
                    threshold: 1000.0, // Requests per second
                    action: ScalingAction::AutoScale { min: 2, max: 10 },
                    cooldown_seconds: 300,
                },
                // Scale based on CPU utilization
                ScalingPolicy {
                    policy_id: "cpu-scaling".to_string(),
                    metric: ScalingMetric::CpuUtilization,
                    threshold: 70.0, // 70% CPU
                    action: ScalingAction::ScaleUp { instances: 2 },
                    cooldown_seconds: 180,
                },
            ],
            affinity_rules: vec![
                // Prefer nodes with security hardware
                AffinityRule {
                    rule_id: "security-hardware-affinity".to_string(),
                    affinity_type: AffinityType::NodeAffinity,
                    target: "security-hardware=true".to_string(),
                    weight: 0.8,
                },
                // Anti-affinity with compute-heavy services
                AffinityRule {
                    rule_id: "compute-anti-affinity".to_string(),
                    affinity_type: AffinityType::AntiAffinity,
                    target: "service-type=compute".to_string(),
                    weight: 0.6,
                },
            ],
            security_policies: vec![
                "require-mutual-tls".to_string(),
                "encrypt-at-rest".to_string(),
                "audit-all-requests".to_string(),
                "rate-limit-by-client".to_string(),
            ],
            monitoring_config: MonitoringConfig {
                metrics_enabled: true,
                tracing_enabled: true,
                logging_level: "info".to_string(),
                custom_metrics: vec![
                    CustomMetric {
                        name: "security_operations_per_second".to_string(),
                        metric_type: MetricType::Counter,
                        description: "Number of security operations processed per second"
                            .to_string(),
                        tags: HashMap::from([
                            ("service".to_string(), "beardog".to_string()),
                            ("type".to_string(), "security".to_string()),
                        ]),
                    },
                    CustomMetric {
                        name: "threat_detection_latency".to_string(),
                        metric_type: MetricType::Histogram,
                        description: "Latency of threat detection operations".to_string(),
                        tags: HashMap::from([
                            ("service".to_string(), "beardog".to_string()),
                            ("operation".to_string(), "threat_detection".to_string()),
                        ]),
                    },
                ],
                alert_rules: vec![
                    AlertRule {
                        rule_id: "high-error-rate".to_string(),
                        condition: "error_rate > 0.05".to_string(),
                        threshold: 0.05,
                        severity: AlertSeverity::Warning,
                        notification_channels: vec!["security-team".to_string()],
                    },
                    AlertRule {
                        rule_id: "security-breach-attempt".to_string(),
                        condition: "failed_auth_rate > 10".to_string(),
                        threshold: 10.0,
                        severity: AlertSeverity::Critical,
                        notification_channels: vec![
                            "security-team".to_string(),
                            "incident-response".to_string(),
                        ],
                    },
                ],
            },
        };

        let advertised_service = AdvertisedService {
            service_id,
            service_name: "BearDog Universal Security Provider".to_string(),
            capabilities: all_capabilities,
            endpoints,
            health_check_url: "https://beardog.security.internal:8443/health".to_string(),
            discovery_tags: self.config.discovery_tags.clone(),
            load_balancer_config,
            orchestration_metadata,
        };

        Ok(advertised_service)
    }

    /// Get core security capabilities for advertisement
    fn get_core_security_capabilities(&self) -> Vec<Capability> {
        use super::capability_ids;

        vec![
            Capability {
                id: capability_ids::SECURITY_ENCRYPT.to_string(),
                name: "Universal Encryption".to_string(),
                description: "Post-quantum encryption for all ecosystem components".to_string(),
                category: CapabilityCategory::Security,
                attributes: HashMap::from([
                    (
                        "algorithms".to_string(),
                        CapabilityAttribute {
                            value: "AES256,ChaCha20,Quantum-Resistant".to_string(),
                            data_type: AttributeDataType::Array,
                            required: true,
                            description: Some("Supported encryption algorithms".to_string()),
                        },
                    ),
                    (
                        "performance".to_string(),
                        CapabilityAttribute {
                            value: "1000".to_string(),
                            data_type: AttributeDataType::Integer,
                            required: false,
                            description: Some("Encryptions per second".to_string()),
                        },
                    ),
                ]),
                qos: QualityOfService {
                    avg_response_time_ms: 5,
                    availability_percent: 99.95,
                    throughput: Some(ThroughputMetric {
                        value: 1000,
                        unit: "encryptions/sec".to_string(),
                    }),
                    scalability: ScalabilityInfo {
                        min_instances: 1,
                        max_instances: 100,
                        auto_scaling: true,
                    },
                },
                resource_requirements: ResourceRequirements::default(),
            },
            Capability {
                id: capability_ids::SECURITY_AUTHENTICATE.to_string(),
                name: "Multi-Modal Authentication".to_string(),
                description: "Biometric, cryptographic, and human-entropy authentication"
                    .to_string(),
                category: CapabilityCategory::Security,
                attributes: HashMap::from([(
                    "auth_methods".to_string(),
                    CapabilityAttribute {
                        value: "biometric,crypto,human_entropy,mfa".to_string(),
                        data_type: AttributeDataType::Array,
                        required: true,
                        description: Some("Supported authentication methods".to_string()),
                    },
                )]),
                qos: QualityOfService {
                    avg_response_time_ms: 15,
                    availability_percent: 99.9,
                    throughput: Some(ThroughputMetric {
                        value: 500,
                        unit: "authentications/sec".to_string(),
                    }),
                    scalability: ScalabilityInfo {
                        min_instances: 2,
                        max_instances: 50,
                        auto_scaling: true,
                    },
                },
                resource_requirements: ResourceRequirements::default(),
            },
            Capability {
                id: capability_ids::SECURITY_AUTHORIZE.to_string(),
                name: "Zero-Trust Authorization".to_string(),
                description: "Policy-based authorization with genetic spawning support".to_string(),
                category: CapabilityCategory::Security,
                attributes: HashMap::from([(
                    "policies".to_string(),
                    CapabilityAttribute {
                        value: "rbac,abac,genetic,contextual".to_string(),
                        data_type: AttributeDataType::Array,
                        required: true,
                        description: Some("Supported authorization policies".to_string()),
                    },
                )]),
                qos: QualityOfService {
                    avg_response_time_ms: 10,
                    availability_percent: 99.95,
                    throughput: Some(ThroughputMetric {
                        value: 2000,
                        unit: "authorizations/sec".to_string(),
                    }),
                    scalability: ScalabilityInfo {
                        min_instances: 2,
                        max_instances: 100,
                        auto_scaling: true,
                    },
                },
                resource_requirements: ResourceRequirements::default(),
            },
        ]
    }

    /// Convert genetic capabilities to standard format
    fn convert_genetic_capabilities_to_standard(
        &self,
        genetic_profile: &super::capability_manager::GeneticCapabilityProfile,
    ) -> Vec<Capability> {
        genetic_profile
            .evolved_capabilities
            .iter()
            .map(|cap_id| Capability {
                id: cap_id.clone(),
                name: format!("Genetic Capability (Gen {})", genetic_profile.generation),
                description: format!(
                    "Evolved security capability with fitness {:.2}",
                    genetic_profile.fitness_score
                ),
                category: CapabilityCategory::Custom("genetic_security".to_string()),
                attributes: HashMap::from([
                    (
                        "generation".to_string(),
                        CapabilityAttribute {
                            value: genetic_profile.generation.to_string(),
                            data_type: AttributeDataType::Integer,
                            required: true,
                            description: Some("Genetic generation number".to_string()),
                        },
                    ),
                    (
                        "fitness_score".to_string(),
                        CapabilityAttribute {
                            value: genetic_profile.fitness_score.to_string(),
                            data_type: AttributeDataType::Float,
                            required: true,
                            description: Some("Genetic fitness score".to_string()),
                        },
                    ),
                ]),
                qos: QualityOfService {
                    avg_response_time_ms: (50.0 * (1.0 - genetic_profile.fitness_score)) as u64,
                    availability_percent: 95.0 + (genetic_profile.fitness_score * 4.9),
                    throughput: Some(ThroughputMetric {
                        value: (genetic_profile.fitness_score * 1000.0) as u64,
                        unit: "genetic_ops/sec".to_string(),
                    }),
                    scalability: ScalabilityInfo {
                        min_instances: 1,
                        max_instances: (genetic_profile.fitness_score * 20.0) as u32,
                        auto_scaling: true,
                    },
                },
                resource_requirements: ResourceRequirements::default(),
            })
            .collect()
    }

    /// Convert emergent capability to standard format
    fn convert_emergent_capability_to_standard(
        &self,
        emergent: &super::capability_manager::EmergentCapability,
    ) -> Capability {
        Capability {
            id: emergent.capability_id.clone(),
            name: emergent.name.clone(),
            description: emergent.description.clone(),
            category: CapabilityCategory::Custom("emergent_security".to_string()),
            attributes: HashMap::from([
                (
                    "stability_score".to_string(),
                    CapabilityAttribute {
                        value: emergent.stability_score.to_string(),
                        data_type: AttributeDataType::Float,
                        required: true,
                        description: Some("Emergent capability stability score".to_string()),
                    },
                ),
                (
                    "uniqueness_score".to_string(),
                    CapabilityAttribute {
                        value: emergent.uniqueness_score.to_string(),
                        data_type: AttributeDataType::Float,
                        required: true,
                        description: Some("Emergent capability uniqueness score".to_string()),
                    },
                ),
                (
                    "parent_capabilities".to_string(),
                    CapabilityAttribute {
                        value: emergent.parent_capabilities.join(","),
                        data_type: AttributeDataType::Array,
                        required: true,
                        description: Some(
                            "Parent capabilities that created this emergent capability".to_string(),
                        ),
                    },
                ),
            ]),
            qos: QualityOfService {
                avg_response_time_ms: (100.0 * (1.0 - emergent.stability_score)) as u64,
                availability_percent: 90.0 + (emergent.stability_score * 9.9),
                throughput: Some(ThroughputMetric {
                    value: (emergent.uniqueness_score * 500.0) as u64,
                    unit: "emergent_ops/sec".to_string(),
                }),
                scalability: ScalabilityInfo {
                    min_instances: 1,
                    max_instances: if emergent.stability_score > 0.8 {
                        20
                    } else {
                        5
                    },
                    auto_scaling: emergent.stability_score > 0.7,
                },
            },
            resource_requirements: ResourceRequirements::default(),
        }
    }

    /// Start heartbeat task to maintain registration with SongBird
    async fn start_heartbeat_task(&self) -> BearDogResult<()> {
        let client = self.songbird_client.clone();
        let status = self.registration_status.clone();
        let interval_seconds = self.config.heartbeat_interval_seconds;

        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_secs(interval_seconds));

            loop {
                interval.tick().await;

                // Send heartbeat to SongBird
                if let Err(e) = client.send_heartbeat().await {
                    error!("❌ Failed to send heartbeat to SongBird: {}", e);

                    // Update failure count
                    let mut reg_status = status.write().await;
                    reg_status.consecutive_failures += 1;

                    if reg_status.consecutive_failures >= 3 {
                        reg_status.status = RegistrationState::Failed;
                        warn!("⚠️ Registration with SongBird marked as failed after {} consecutive failures", 
                              reg_status.consecutive_failures);
                    }
                } else {
                    // Reset failure count on success
                    let mut reg_status = status.write().await;
                    reg_status.last_heartbeat = Utc::now();
                    reg_status.consecutive_failures = 0;

                    if reg_status.status == RegistrationState::Failed {
                        reg_status.status = RegistrationState::Registered;
                        info!("✅ Registration with SongBird restored");
                    }
                }
            }
        });

        Ok(())
    }

    /// Start health monitoring task
    async fn start_health_monitoring_task(&self) -> BearDogResult<()> {
        let client = self.songbird_client.clone();
        let services = self.advertised_services.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

            loop {
                interval.tick().await;

                // Update health status for all advertised services
                let service_list = services.read().await;
                for service in service_list.values() {
                    if let Err(e) = client.update_service_health(&service.service_id).await {
                        warn!(
                            "⚠️ Failed to update health for service {}: {}",
                            service.service_id, e
                        );
                    }
                }
            }
        });

        Ok(())
    }

    /// Get current registration status
    pub async fn get_registration_status(&self) -> RegistrationStatus {
        self.registration_status.read().await.clone()
    }

    /// Get all advertised services
    pub async fn get_advertised_services(&self) -> HashMap<String, AdvertisedService> {
        self.advertised_services.read().await.clone()
    }

    /// Update capability advertisement with SongBird
    pub async fn update_capability_advertisement(&self) -> BearDogResult<()> {
        info!("🔄 Updating capability advertisement with SongBird");

        // Re-register with updated capabilities
        self.register_with_songbird().await?;

        info!("✅ Capability advertisement updated successfully");
        Ok(())
    }
}

impl SongBirdDiscoveryClient {
    /// Create a new SongBird discovery client
    pub async fn new(endpoint: String, api_key: String) -> BearDogResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(tokio::time::Duration::from_secs(30))
            .build()
            .map_err(|e| BearDogError::Network {
                message: format!("Failed to create HTTP client: {}", e),
            })?;

        Ok(Self {
            endpoint,
            api_key,
            client,
            health_status: Arc::new(RwLock::new(ServiceHealth {
                status: HealthStatus::Unknown,
                last_check: Utc::now(),
                response_time_ms: 0,
                error_count: 0,
                uptime_percentage: 0.0,
            })),
        })
    }

    /// Register service with SongBird
    pub async fn register_service(
        &self,
        service: &AdvertisedService,
    ) -> BearDogResult<ServiceRegistrationResult> {
        let url = format!("{}/api/v1/services/register", self.endpoint);

        debug!("📡 Registering service with SongBird: {}", url);

        let registration_payload = serde_json::json!({
            "service": service,
            "registration_metadata": {
                "client": "beardog-handoff-manager",
                "version": "1.0.0",
                "timestamp": Utc::now().to_rfc3339(),
            }
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&registration_payload)
            .send()
            .await
            .map_err(|e| BearDogError::Network {
                message: format!("Failed to send registration request: {}", e),
            })?;

        if response.status().is_success() {
            let result: ServiceRegistrationResult =
                response
                    .json()
                    .await
                    .map_err(|e| BearDogError::Serialization {
                        message: format!("Failed to parse registration response: {}", e),
                    })?;

            info!(
                "✅ Successfully registered with SongBird: {}",
                result.registration_id
            );
            Ok(result)
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(BearDogError::Network {
                message: format!("Registration failed with status {}: {}", status, error_text),
            })
        }
    }

    /// Send heartbeat to SongBird
    pub async fn send_heartbeat(&self) -> BearDogResult<()> {
        let url = format!("{}/api/v1/services/heartbeat", self.endpoint);

        let heartbeat_payload = serde_json::json!({
            "service_id": "beardog-security-provider",
            "timestamp": Utc::now().to_rfc3339(),
            "health_status": "healthy",
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&heartbeat_payload)
            .send()
            .await
            .map_err(|e| BearDogError::Network {
                message: format!("Failed to send heartbeat: {}", e),
            })?;

        if response.status().is_success() {
            debug!("💓 Heartbeat sent to SongBird successfully");
            Ok(())
        } else {
            Err(BearDogError::Network {
                message: format!("Heartbeat failed with status {}", response.status()),
            })
        }
    }

    /// Update service health status
    pub async fn update_service_health(&self, service_id: &str) -> BearDogResult<()> {
        let url = format!("{}/api/v1/services/{}/health", self.endpoint, service_id);

        let health_payload = serde_json::json!({
            "service_id": service_id,
            "health_status": "healthy",
            "timestamp": Utc::now().to_rfc3339(),
            "metrics": {
                "response_time_ms": 10,
                "error_rate": 0.001,
                "throughput": 1000,
            }
        });

        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&health_payload)
            .send()
            .await
            .map_err(|e| BearDogError::Network {
                message: format!("Failed to update health: {}", e),
            })?;

        if response.status().is_success() {
            debug!("🏥 Health status updated for service: {}", service_id);
            Ok(())
        } else {
            Err(BearDogError::Network {
                message: format!("Health update failed with status {}", response.status()),
            })
        }
    }
}

/// Result of service registration with SongBird
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationResult {
    pub registration_id: String,
    pub service_discovery_url: String,
    pub load_balancer_endpoints: Vec<String>,
    pub monitoring_dashboard_url: String,
    pub orchestration_policies_applied: Vec<String>,
}

impl Default for LoadBalancerConfig {
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::PerformanceBased,
            health_check_interval_seconds: 30,
            max_retries: 3,
            timeout_seconds: 30,
            circuit_breaker_enabled: true,
        }
    }
}
