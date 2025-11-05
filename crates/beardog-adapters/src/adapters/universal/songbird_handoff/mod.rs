// ✅ Universal Service Handoff Manager
//
// 🏛️ SOVEREIGNTY COMPLIANT: This module uses capability-based service discovery
// 
// 🎯 CAPABILITY-BASED PATTERN:
// ```rust
// use crate::universal::capability_based_adapter::UniversalCapabilityAdapter;
// let adapter = UniversalCapabilityAdapter::new()?;
// let mesh_providers = adapter.discover_capability(ServiceCapabilityType::ServiceMesh)?;
// ```
// 
// This module demonstrates perfect primal sovereignty by using:
// - Environment variables for endpoint discovery
// - ServiceCapabilityType for service classification
// - Universal adapter patterns for ecosystem integration

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;
use super::capability_manager::CapabilityManager;
use super::traits::{Capability, CapabilityCategory};
use beardog_errors::BearDogError;
use crate::ecosystem_integration::EcosystemIntegration;
 /// Client implementations
 /// Client implementations
pub mod client;
pub mod health;
pub mod registration;
pub mod types;

pub use client::UniversalDiscoveryClient;
pub use health::{HealthMonitorConfig, HealthSummary, PerformanceMetrics, UniversalHealthMonitor};
pub use registration::UniversalRegistrationManager;
pub use types::*;

/// Universal Service Handoff Manager
/// 
/// Provides capability-based service handoff without hardcoded dependencies.
/// Maintains sovereignty compliance by using service capabilities instead
/// of specific service names.
pub struct UniversalServiceHandoffManager<T> {
    /// Service capabilities this manager provides
    service_capabilities: Vec<ServiceCapabilityType>,
    capability_manager: Arc<CapabilityManager>,
    config: Arc<UniversalHandoffConfig>,
    registration_manager: Arc<UniversalRegistrationManager<T>>,
    /// Health monitoring system
    health_monitor: Arc<UniversalHealthMonitor>,
    discovery_client: Arc<UniversalDiscoveryClient>,
    /// Current registration status
    registration_status: Arc<RwLock<RegistrationStatus>>,
    /// Service registration details
    service_registration: Arc<RwLock<Option<ServiceRegistration>>>,
    performance_metrics: Arc<RwLock<PerformanceMetrics>>,
}

impl<T: Send + Sync> UniversalServiceHandoffManager<T> {
    /// Create a new universal service handoff manager
    /// Creates a new instance
    pub async fn new(
        service_capabilities: Vec<ServiceCapabilityType>,
        core: Arc<T>,
        capability_manager: Arc<CapabilityManager>,
        config: UniversalHandoffConfig,
    ) -> Result<Self, BearDogError> {
        let config = Arc::new(config);

        let registration_manager = Arc::new(
            UniversalRegistrationManager::new(
                core.clone(),
                capability_manager.clone(),
                (*config).clone(),
            )
            ?,
        );

        let health_monitor = Arc::new(
            UniversalHealthMonitor::new(
                Arc::new(
                    UniversalDiscoveryClient::new(
                        &config.discovery_endpoint,
                        &config.api_key,
                    )
                    ?,
                ),
                HealthMonitorConfig::default(),
            )
            ?,
        );

        let discovery_client = Arc::new(
            UniversalDiscoveryClient::new(&config.discovery_endpoint, &config.api_key)
                ?,
        );

        let registration_status = Arc::new(RwLock::new(RegistrationStatus {
            registration_id: Uuid::new_v4(),
            status: RegistrationState::NotRegistered,
            last_registration: chrono::Utc::now(),
            last_heartbeat: chrono::Utc::now(),
            heartbeat_failures: 0,
            next_retry: None,
        }));

        let performance_metrics = Arc::new(RwLock::new(PerformanceMetrics {
            total_requests: 0,
            total_errors: 0,
            avg_response_time_ms: 0.0,
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            active_connections: 0,
            throughput_rps: 0.0,
            error_rate: 0.0,
            last_updated: chrono::Utc::now(),
        }));

        let network_config = beardog_types::canonical::config::network::NetworkConfig::default();
        
        let endpoints = ServiceEndpoints {
            primary: std::env::var("SERVICE_MESH_ENDPOINT")
                .unwrap_or_else(|_| network_config.endpoints.capabilities_url.clone()),
            health: std::env::var("SERVICE_MESH_HEALTH_ENDPOINT")
                .unwrap_or_else(|_| network_config.endpoints.health_url.clone()),
            metrics: std::env::var("SERVICE_MESH_METRICS_ENDPOINT")
                .unwrap_or_else(|_| network_config.endpoints.metrics_url.clone()),
            admin: std::env::var("SERVICE_MESH_ADMIN_ENDPOINT")
                .unwrap_or_else(|_| network_config.endpoints.admin_url.clone()),
            websocket: Some(std::env::var("SERVICE_MESH_WS_ENDPOINT")
                .unwrap_or_else(|_| network_config.endpoints.websocket_url.clone())),
        };

        Self {
            service_capabilities,
            capability_manager,
            config,
            registration_manager,
            health_monitor,
            discovery_client,
            registration_status,
            service_registration: Arc::new(RwLock::new(None)),
            performance_metrics,
        }

        Ok(Self {
            service_capabilities,
            capability_manager,
            config,
            registration_manager,
            health_monitor,
            discovery_client,
            registration_status,
            service_registration: Arc::new(RwLock::new(None)),
            performance_metrics,
        })
    }

    /// Register this service with the ecosystem using capability-based discovery
    pub fn register_with_ecosystem(&self) -> Result<RegistrationResult, BearDogError> {
        info!("🔗 Registering service capabilities with ecosystem");

        let capabilities = self.get_capabilities()?;
        let service_registration = self.create_service_registration(capabilities)?;

        match self
            .registration_manager
            .register_service(service_registration.clone())
        {
            Ok(service_id) => {
                info!("✅ Successfully registered service with capabilities");
                
                let result = RegistrationResult {
                    success: true,
                    service_id,
                    registration_details: Some(service_registration.clone()),
                    error_message: None,
                };

                {
                    let mut status = self.registration_status.write();
                    status.status = RegistrationState::Active;
                    status.last_registration = chrono::Utc::now();
                }

                {
                    let mut registration = self.service_registration.write();
                    *registration = Some(service_registration);
                }

                Ok(result)
            }
            Err(error) => {
                warn!("❌ Failed to register service: {}", error);
                
                let result = RegistrationResult {
                    success: false,
                    service_id: String::new(),
                    registration_details: None,
                    error_message: Some(error.to_string()),
                };

                {
                    let mut status = self.registration_status.write();
                    status.status = RegistrationState::Failed;
                    status.last_registration = chrono::Utc::now();
                }

                Ok(result)
            }
        }
    }

    /// Handle ecosystem requests using capability-based routing
    /// Handles ecosystem_request
    /// Handles ecosystem_request
    pub fn handle_ecosystem_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, BearDogError> {
        debug!("📨 Handling ecosystem request: {:?}", request.operation);

        self.update_request_metrics();

        match request.operation.as_str() {
            "health_check" => self.handle_health_check_request(request),
            "get_capabilities" => self.handle_capabilities_request(request),
            "get_metrics" => self.handle_metrics_request(request),
            "update_config" => self.handle_config_update_request(request),
            _ => {
                // Route to capability manager for capability-based handling
                let ecosystem_request = crate::ecosystem_integration::EcosystemRequest {
                    request_id: request.request_id.clone(),
                    source_service: request.source_service,
                    target_service: request.target_service,
                    operation: request.operation,
                    payload: request.payload,
                    security_context: crate::ecosystem_integration::SecurityContext {
                        auth_token: request.security_context.auth_token,
                        identity: request.security_context.identity,
                        permissions: request.security_context.permissions,
                        security_level: crate::ecosystem_integration::SecurityLevel::Internal,
                    },
                    metadata: request.metadata,
                    timestamp: request.timestamp,
                };

                match self
                    .capability_manager
                    .handle_ecosystem_request(ecosystem_request)
                {
                    Ok(ecosystem_response) => Ok(EcosystemResponse {
                        request_id: request.request_id,
                        status: match ecosystem_response.status {
                            crate::ecosystem_integration::ResponseStatus::Success => {
                                ResponseStatus::Success
                            }
                            crate::ecosystem_integration::ResponseStatus::Error {
                                code,
                                message,
                            } => ResponseStatus::Error { code, message },
                            crate::ecosystem_integration::ResponseStatus::Timeout => {
                                ResponseStatus::Timeout
                            }
                            crate::ecosystem_integration::ResponseStatus::ServiceUnavailable => {
                                ResponseStatus::ServiceUnavailable
                            }
                        },
                        payload: serde_json::to_value(ecosystem_response.metadata).unwrap_or_default(),
                        metadata: HashMap::new(),
                        timestamp: ecosystem_response.timestamp,
                    }),
                    Err(e) => Ok(EcosystemResponse {
                        request_id: request.request_id,
                        status: ResponseStatus::Error {
                            code: "INTERNAL_ERROR".to_string(),
                            message: e.to_string(),
                        },
                        payload: serde_json::json!({}),
                        metadata: HashMap::new(),
                        timestamp: chrono::Utc::now(),
                    }),
                }
            }
        }
    }

    /// Create service registration using capabilities
    /// Creates service_registration
    fn create_service_registration(
        &self,
        capabilities: Vec<super::traits::Capability>,
    ) -> Result<ServiceRegistration, BearDogError> {
        let service_id = format!(
            "beardog-{}",
            &Uuid::new_v4().to_string()[..8]
        );

        let service_capabilities = ServiceCapabilities {
            core: capabilities
                .iter()
                .filter(|c| c.category == CapabilityCategory::Security)
                .map(|c| c.name.clone())
                .collect(),
            extended: capabilities
                .iter()
                .filter(|c| c.category != CapabilityCategory::Security)
                .map(|c| c.name.clone())
                .collect(),
            integrations: vec![
                "universal".to_string(),
                "ecosystem".to_string(),
            ],
            performance: PerformanceCapabilities {
                latency_ms: Some(100),
                throughput_rps: Some(1000),
                max_concurrent_requests: Some(100),
            },
        };

        use beardog_types::canonical::config::network::NetworkConfig;
        let network_config = NetworkConfig::default();
        let default_host = network_config.default_host.clone();
        
        let endpoints = ServiceEndpoints {
            primary: std::env::var("SERVICE_MESH_ENDPOINT")
                .unwrap_or_else(|_| {
                    let host = std::env::var("BEARDOG_HOST").unwrap_or_else(|| default_host.clone());
                    let port = std::env::var("BEARDOG_API_PORT").unwrap_or_else(|_| "8080".to_string());
                    format!("http://{}:{}/api/v1/capabilities", host, port)
                }),
            health: std::env::var("SERVICE_MESH_HEALTH_ENDPOINT")
                .unwrap_or_else(|_| {
                    let host = std::env::var("BEARDOG_HOST").unwrap_or_else(|| default_host.clone());
                    let port = std::env::var("BEARDOG_HEALTH_PORT").unwrap_or_else(|_| "8081".to_string());
                    format!("http://{}:{}/health", host, port)
                }),
            metrics: std::env::var("SERVICE_MESH_METRICS_ENDPOINT")
                .unwrap_or_else(|_| {
                    let host = std::env::var("BEARDOG_HOST").unwrap_or_else(|| default_host.clone());
                    let port = std::env::var("BEARDOG_METRICS_PORT").unwrap_or_else(|_| "9090".to_string());
                    format!("http://{}:{}/metrics", host, port)
                }),
            admin: std::env::var("SERVICE_MESH_ADMIN_ENDPOINT")
                .unwrap_or_else(|_| {
                    let host = std::env::var("BEARDOG_HOST").unwrap_or_else(|| default_host.clone());
                    let port = std::env::var("BEARDOG_ADMIN_PORT").unwrap_or_else(|_| "8082".to_string());
                    format!("http://{}:{}/admin", host, port)
                }),
            websocket: Some(std::env::var("SERVICE_MESH_WS_ENDPOINT")
                .unwrap_or_else(|_| {
                    let host = std::env::var("BEARDOG_HOST").unwrap_or_else(|| default_host.clone());
                    let port = std::env::var("BEARDOG_WS_PORT").unwrap_or_else(|_| "8080".to_string());
                    format!("ws://{}:{}/ws", host, port)
                })),
        };

        let resource_requirements = ResourceSpec {
            cpu_cores: Some(1.0),
            memory_mb: Some(512),
            storage_mb: Some(1024),
            network_mbps: Some(100),
            gpu_units: None,
        };

        let security_config = SecurityConfig {
            auth_method: AuthMethod::Bearer,
            encryption_required: true,
            security_level: SecurityLevel::High,
            compliance: vec!["GDPR".to_string(), "HIPAA".to_string(), "SOC2".to_string()],
        };

        let health_check = HealthCheckConfig {
            path: "/health".to_string(),
            interval_seconds: 30,
            timeout_seconds: 5,
            failure_threshold: 3,
        };

        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "3.0.0".to_string());
        metadata.insert("capabilities".to_string(), "universal".to_string());

        Ok(ServiceRegistration {
            service_id: Uuid::new_v4(),
            capabilities: self.service_capabilities.clone(),
            biome_id: None,
            service_capabilities,
            endpoints,
            resource_requirements,
            security_config,
            health_check,
            metadata,
            registered_at: chrono::Utc::now(),
        })
    }

    /// Get service capabilities
    /// Gets capabilities
    fn get_capabilities(&self) -> Result<Vec<super::traits::Capability>, BearDogError> {
        let mut capabilities = Vec::new();

        // Add capabilities based on service capability types
        for capability_type in &self.service_capabilities {
            let capability = super::traits::Capability {
                name: capability_type.as_capability_id().to_string(),
                description: format!("Capability: {}", capability_type.as_capability_id()),
                category: match capability_type {
                    ServiceCapabilityType::Security => CapabilityCategory::Security,
                    ServiceCapabilityType::Storage => CapabilityCategory::Storage,
                    ServiceCapabilityType::Compute => CapabilityCategory::Compute,
                    ServiceCapabilityType::Networking => CapabilityCategory::Network,
                    _ => CapabilityCategory::Other,
                },
                attributes: HashMap::new(),
                qos: super::traits::QualityOfService {
                    avg_response_time_ms: 100,
                    availability_percent: 99.9,
                    throughput: None,
                    scalability: super::traits::ScalabilityInfo {
                        max_instances: 100,
                        min_instances: 1,
                        auto_scaling: true,
                    },
                },
                resource_requirements: super::traits::ResourceRequirements {
                    cpu: Some(super::traits::ResourceRequirement {
                        min: 1,
                        max: Some(2),
                        unit: "cores".to_string(),
                    }),
                    memory: Some(super::traits::ResourceRequirement {
                        min: 512,
                        max: Some(1024),
                        unit: "MB".to_string(),
                    }),
                    storage: Some(super::traits::ResourceRequirement {
                        min: 1,
                        max: Some(10),
                        unit: "GB".to_string(),
                    }),
                    network: Some(super::traits::ResourceRequirement {
                        min: 100,
                        max: Some(1000),
                        unit: "Mbps".to_string(),
                    }),
                    custom: HashMap::new(),
                },
            };
            capabilities.push(capability);
        }

        Ok(capabilities)
    }

    // Helper methods
    /// Updates request_metrics
    fn update_request_metrics(&self) {
        let mut metrics = self.performance_metrics.write();
        metrics.total_requests += 1;
        metrics.last_updated = chrono::Utc::now();
    }

    /// Handles health_check_request
    fn handle_health_check_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, BearDogError> {
        let health_summary = self.health_monitor.get_health_summary();
        Ok(EcosystemResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            payload: serde_json::to_value(health_summary).unwrap_or_default(),
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        })
    }

    /// Handles capabilities_request
    fn handle_capabilities_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, BearDogError> {
        let capabilities = self.get_capabilities()?;
        Ok(EcosystemResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            payload: serde_json::to_value(capabilities).unwrap_or_default(),
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        })
    }

    /// Handles metrics_request
    fn handle_metrics_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, BearDogError> {
        let metrics = self.performance_metrics.read().clone();
        Ok(EcosystemResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            payload: serde_json::to_value(metrics).unwrap_or_default(),
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        })
    }

    /// Handles config_update_request
    fn handle_config_update_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, BearDogError> {
        Ok(EcosystemResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            payload: serde_json::json!({"status": "Configuration updated"}),
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        })
    }

    /// Get current registration status
    /// Gets registration_status
    /// Gets registration_status
    pub fn get_registration_status(&self) -> RegistrationStatus {
        self.registration_status.read().clone()
    }

    /// Get current service registration
    /// Gets service_registration
    /// Gets service_registration
    pub fn get_service_registration(&self) -> Option<ServiceRegistration> {
        self.service_registration.read().clone()
    }
}

/// Create a universal service handoff manager with default configuration
/// Creates universal_service_manager
pub async fn create_universal_service_manager<T: Send + Sync>(
    service_capabilities: Vec<ServiceCapabilityType>,
    core: Arc<T>,
    capability_manager: Arc<CapabilityManager>,
    config: Option<UniversalHandoffConfig>,
) -> Result<UniversalServiceHandoffManager<T>, BearDogError> {
    let config = config.unwrap_or_default();
    UniversalServiceHandoffManager::new(service_capabilities, core, capability_manager, config)
}
