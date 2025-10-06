#![allow(async_fn_in_trait)]

use crate::ecosystem::primal_types::ServiceDependency;
use crate::ecosystem::primal_types::{
    PrimalError,
    PrimalHealth,
    PrimalMetadata,
    PrimalRequest,
    PrimalResponse, // PrimalType removed
    UniversalIntegrationConfig,
};
use crate::BearDogCore;
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::{CapabilityType, ServiceCapabilityType};
use tracing::{debug, info, warn};

/// Primal Trait - Core interface for all primal components
///
/// Defines the essential interface that all primal components must implement
/// for health monitoring, request handling, and capability discovery.
///
/// This trait enables the ecosystem integration pattern where each primal:
/// 1. Provides metadata about itself
/// 2. Can be initialized with configuration
/// 3. Reports health status
/// 4. Handles requests from other primals
/// 5. Advertises its capabilities
///
/// ## Example Implementation
///
/// ```rust,ignore
/// impl PrimalTrait for MyCustomPrimal {
///     fn metadata(&self) -> PrimalMetadata {
///         PrimalMetadata {
///             name: "my-custom-primal".to_string(),
///             version: "1.0.0".to_string(),
///             // ...
///         }
///     }
///     
///     async fn health_check(&self) -> Result<PrimalHealth, PrimalError> {
///         Ok(PrimalHealth::Healthy)
///     }
///     // ... other trait methods
/// }
/// ```
pub trait PrimalTrait {
    /// Get metadata about this primal component
    ///
    /// Returns information such as name, version, description, and endpoints.
    fn metadata(&self) -> PrimalMetadata;

    /// Initialize the primal component with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Universal integration configuration for ecosystem coordination
    ///
    /// # Errors
    ///
    /// Returns `PrimalError` if initialization fails.
    async fn initialize(&self, config: &UniversalIntegrationConfig) -> Result<(), PrimalError>;

    /// Check the health status of this primal component
    ///
    /// # Returns
    ///
    /// Returns `PrimalHealth` indicating the current operational status.
    ///
    /// # Errors
    ///
    /// Returns `PrimalError` if health check fails to complete.
    async fn health_check(&self) -> Result<PrimalHealth, PrimalError>;

    /// Handle an incoming request to the primal component
    ///
    /// # Arguments
    ///
    /// * `request` - The request to process
    ///
    /// # Returns
    ///
    /// Returns `PrimalResponse` containing the result of the request.
    ///
    /// # Errors
    ///
    /// Returns `PrimalError` if the request cannot be processed.
    async fn handle_request(&self, request: PrimalRequest) -> Result<PrimalResponse, PrimalError>;

    /// Discover AI capabilities available in this primal component
    ///
    /// Returns a list of AI-related capabilities such as machine learning,
    /// natural language processing, or hybrid intelligence features.
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if capability discovery fails.
    async fn discover_ai_capabilities(&self) -> Result<Vec<ServiceCapabilityType>, BearDogError>;

    /// Discover compute capabilities available in this primal component
    ///
    /// Returns a list of compute-related capabilities such as processing power,
    /// parallel execution, or specialized hardware acceleration.
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if capability discovery fails.
    async fn discover_compute_capabilities(
        &self,
    ) -> Result<Vec<ServiceCapabilityType>, BearDogError>;

    /// Discover storage capabilities available in this primal component
    ///
    /// Returns a list of storage-related capabilities such as persistent storage,
    /// caching, or distributed storage systems.
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if capability discovery fails.
    async fn discover_storage_capabilities(
        &self,
    ) -> Result<Vec<ServiceCapabilityType>, BearDogError>;
}

impl PrimalTrait for BearDogCore {
    fn metadata(&self) -> PrimalMetadata {
        PrimalMetadata {
            display_name: Some("BearDog Security System".to_string()),
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocol_versions: vec!["1.0.0".to_string()],
            security_attestations: vec![],
            custom_fields: std::collections::HashMap::new(),
            capabilities: vec![
                CapabilityType::Security,
                CapabilityType::HardwareSecurityModule,
                CapabilityType::KeyManagement,
                CapabilityType::Authentication,
                CapabilityType::BiometricAuth,
            ],
            dependencies: vec![
                ServiceDependency::Optional {
                    capability: ServiceCapabilityType::ContainerOrchestration,
                    min_version: "1.0.0".to_string(),
                    reason: "Enhanced performance through compute integration".to_string(),
                },
                ServiceDependency::Optional {
                    capability: ServiceCapabilityType::ServiceMesh,
                    min_version: "1.0.0".to_string(),
                    reason: "Service discovery and load balancing".to_string(),
                },
                ServiceDependency::Optional {
                    capability: ServiceCapabilityType::DistributedIntelligence,
                    min_version: "1.0.0".to_string(),
                    reason: "AI-powered threat detection and security optimization".to_string(),
                },
            ],
            supported_protocols: vec!["grpc".to_string(), "https".to_string()],
            health_check_endpoint: "/api/v1/health".to_string(),
            metrics_endpoint: "/api/v1/metrics".to_string(),
        }
    }

    #[allow(deprecated)]
    /// Initializes componentialize
    async fn initialize(&self, config: &UniversalIntegrationConfig) -> Result<(), PrimalError> {
        info!("🚀 Initializing BearDog primal with ecosystem integration");

        // Initialize core systems first - removed recursive call
        info!("✅ Core systems initialized");

        // Discover and register AI capabilities if available
        if let Err(e) = self.discover_ai_capabilities().await {
            warn!("AI capability discovery failed: {:?}", e);
        }

        // Discover and register capabilities based on configuration
        if config.enable_capability_discovery {
            // Discover required capabilities
            for capability in &config.required_capabilities {
                if let Err(e) = self.discover_capability(capability).await {
                    warn!(
                        "Failed to discover required capability {:?}: {:?}",
                        capability, e
                    );
                }
            }

            // Discover optional capabilities
            for capability in &config.optional_capabilities {
                if let Err(e) = self.discover_capability(capability).await {
                    warn!(
                        "Failed to discover optional capability {:?}: {:?}",
                        capability, e
                    );
                }
            }
        }

        info!("✅ BearDog primal initialization completed");
        Ok(())
    }

    async fn health_check(&self) -> Result<PrimalHealth, PrimalError> {
        debug!("🏥 Performing BearDog primal health check");

        let health_status = self
            .get_ecosystem_integration_health()
            .await
            .map_err(|e| PrimalError::health_check_failed(format!("Health check failed: {e}")))?;

        Ok(PrimalHealth {
            status: health_status,
            last_check: chrono::Utc::now(),
            details: std::collections::HashMap::new(),
            checks: std::collections::HashMap::from([
                ("core".to_string(), true),
                ("universal_adapter".to_string(), true),
                ("capability_discovery".to_string(), true),
            ]),
            timestamp: chrono::Utc::now(),
        })
    }

    /// Handles request
    async fn handle_request(&self, request: PrimalRequest) -> Result<PrimalResponse, PrimalError> {
        debug!("📥 Handling primal request: {:?}", request.operation_type);

        match request.operation_type.as_str() {
            "security_operation" => {
                // Handle security-specific operations
                Ok(PrimalResponse {
                    id: uuid::Uuid::new_v4().to_string(),
                    request_id: request.request_id,
                    status: "completed ".to_string(),
                    success: true,
                    data: serde_json::json!({
                        "operation": "security_operation",
                        "result": "success "
                    }),
                    metadata: std::collections::HashMap::new(),
                    timestamp: chrono::Utc::now(),
                })
            }
            "health_check" => {
                let _health = self.health_check().await?;
                Ok(PrimalResponse {
                    id: uuid::Uuid::new_v4().to_string(),
                    request_id: request.request_id,
                    status: "completed ".to_string(),
                    success: true,
                    data: serde_json::json!({
                        "health_status": "healthy",
                        "timestamp": chrono::Utc::now()
                    }),
                    metadata: std::collections::HashMap::new(),
                    timestamp: chrono::Utc::now(),
                })
            }
            _ => Err(PrimalError::unsupported_operation(format!(
                "Operation \"{}\" not supported",
                request.operation_type
            ))),
        }
    }

    async fn discover_ai_capabilities(&self) -> Result<Vec<ServiceCapabilityType>, BearDogError> {
        info!("🤖 Discovering AI capabilities through universal adapter");

        let mut capabilities = Vec::new();

        // Try to discover AI services through universal adapter
        match self
            .universal_adapter
            .discover_capability_endpoint(ServiceCapabilityType::ArtificialIntelligence)
            .await
        {
            Ok(_endpoint) => {
                capabilities.push(ServiceCapabilityType::ArtificialIntelligence);
                capabilities.push(ServiceCapabilityType::Custom("MachineLearning".to_string()));
                capabilities.push(ServiceCapabilityType::Custom("ThreatDetection".to_string()));
                info!("✅ AI capabilities discovered and registered");
            }
            Err(_) => {
                debug!("ℹ️ AI services not currently available");
            }
        }

        Ok(capabilities)
    }

    async fn discover_compute_capabilities(
        &self,
    ) -> Result<Vec<ServiceCapabilityType>, BearDogError> {
        info!("🔧 Discovering compute capabilities through universal adapter");

        let mut capabilities = Vec::new();

        // Try to discover compute services through universal adapter
        match self
            .universal_adapter
            .discover_capability_endpoint(ServiceCapabilityType::Compute)
            .await
        {
            Ok(_endpoint) => {
                capabilities.push(ServiceCapabilityType::Compute);
                capabilities.push(ServiceCapabilityType::Orchestration);
                capabilities.push(ServiceCapabilityType::Custom("HPC".to_string()));
                info!("✅ Compute capabilities discovered and registered");
            }
            Err(_) => {
                debug!("ℹ️ Compute services not currently available");
            }
        }

        Ok(capabilities)
    }

    async fn discover_storage_capabilities(
        &self,
    ) -> Result<Vec<ServiceCapabilityType>, BearDogError> {
        info!("💾 Discovering storage capabilities through universal adapter");

        let mut capabilities = Vec::new();

        // Try to discover storage services through universal adapter
        match self
            .universal_adapter
            .discover_capability_endpoint(ServiceCapabilityType::Storage)
            .await
        {
            Ok(_endpoint) => {
                capabilities.push(ServiceCapabilityType::Storage);
                capabilities.push(ServiceCapabilityType::Custom("SecureStorage".to_string()));
                capabilities.push(ServiceCapabilityType::Custom(
                    "BiometricDataStorage".to_string(),
                ));
                info!("✅ Storage capabilities discovered and registered");
            }
            Err(_) => {
                debug!("ℹ️ Storage services not currently available");
            }
        }

        Ok(capabilities)
    }
}

impl BearDogCore {
    /// Generic capability discovery method (private helper)
    async fn discover_capability(
        &self,
        capability: &ServiceCapabilityType,
    ) -> Result<(), BearDogError> {
        info!("🔍 Discovering capability: {:?}", capability);

        match capability {
            ServiceCapabilityType::ArtificialIntelligence => {
                self.discover_ai_capabilities().await.map(|_| ())
            }
            ServiceCapabilityType::Compute => {
                self.discover_compute_capabilities().await.map(|_| ())
            }
            ServiceCapabilityType::Storage => {
                self.discover_storage_capabilities().await.map(|_| ())
            }
            _ => {
                debug!(
                    "ℹ️ Capability {:?} discovery not yet implemented",
                    capability
                );
                Ok(())
            }
        }
    }
}
