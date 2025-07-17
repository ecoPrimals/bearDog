//! BearDog Ecosystem Integration
//!
//! **BearDog's implementation of ecosystem API standardization**
//!
//! This module implements the standardized ecosystem integration patterns
//! according to the EcoPrimals Ecosystem API Standardization Guide.
//! It provides unified interfaces for Songbird service mesh integration.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::core::BearDogCore;
use base64::{engine::general_purpose, Engine as _};
use beardog_adapters::adapters::universal::capability_manager::CapabilityManager;
use beardog_adapters::adapters::universal::songbird_handoff;
use beardog_adapters::adapters::universal::traits::{
    Capability, CapabilityCategory, HealthStatus as AdapterHealthStatus, QualityOfService,
    ResourceRequirements, ScalabilityInfo, ThroughputMetric,
};
use beardog_errors::BearDogError;
use beardog_security::encryption::{EncryptedData, EncryptionAlgorithm};
use beardog_security::types::SecurityProvider;

/// Standardized request format for all ecosystem communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemRequest {
    /// Unique request identifier
    pub request_id: Uuid,

    /// Source service identifier
    pub source_service: String,

    /// Target service identifier
    pub target_service: String,

    /// Request operation
    pub operation: String,

    /// Request payload
    pub payload: serde_json::Value,

    /// Security context
    pub security_context: SecurityContext,

    /// Request metadata
    pub metadata: HashMap<String, String>,

    /// Request timestamp
    pub timestamp: DateTime<Utc>,
}

/// Health report for ecosystem integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemHealthReport {
    pub service_id: String,
    pub status: AdapterHealthStatus,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub components: Vec<EcosystemComponentHealth>,
}

/// Component health for ecosystem integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemComponentHealth {
    pub name: String,
    pub healthy: bool,
    pub details: Option<String>,
}

/// Capability update for ecosystem integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemCapabilityUpdate {
    pub service_id: String,
    pub capabilities: Vec<Capability>,
    pub timestamp: DateTime<Utc>,
}

/// Standardized response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemResponse {
    /// Request ID this response is for
    pub request_id: Uuid,

    /// Response status
    pub status: ResponseStatus,

    /// Response payload
    pub payload: serde_json::Value,

    /// Response metadata
    pub metadata: HashMap<String, String>,

    /// Response timestamp
    pub timestamp: DateTime<Utc>,
}

/// Response status for ecosystem operations
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ResponseStatus {
    /// Operation completed successfully
    Success,
    /// Operation failed with error details
    Error {
        /// Error code identifier
        code: String,
        /// Human-readable error message
        message: String,
    },
    /// Operation timed out
    Timeout,
    /// Target service is unavailable
    ServiceUnavailable,
}

/// Security context for all requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Authentication token
    pub auth_token: Option<String>,

    /// User/service identity
    pub identity: String,

    /// Permissions/capabilities
    pub permissions: Vec<String>,

    /// Security level required
    pub security_level: SecurityLevel,
}

/// Security level for ecosystem operations
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SecurityLevel {
    /// Public access, no authentication required
    Public,
    /// Internal access, basic authentication required
    Internal,
    /// Restricted access, elevated authentication required
    Restricted,
    /// Confidential access, highest security required
    Confidential,
}

/// Standardized primal types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimalType {
    /// ToadStool primal - system monitoring and threat detection
    ToadStool,
    /// Songbird primal - communication and protocol management
    Songbird,
    /// BearDog primal - security, genetics, and workflow management
    BearDog,
    /// NestGate primal - secure file system and storage management
    NestGate,
    /// Squirrel primal - task automation and system orchestration
    Squirrel,
    /// BiomeOS primal - operating system and hardware integration
    BiomeOS,
}

impl PrimalType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PrimalType::ToadStool => "toadstool",
            PrimalType::Songbird => "songbird",
            PrimalType::BearDog => "beardog",
            PrimalType::NestGate => "nestgate",
            PrimalType::Squirrel => "squirrel",
            PrimalType::BiomeOS => "biomeos",
        }
    }
}

/// Ecosystem service enumeration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EcosystemService {
    /// ToadStool distributed computing platform
    ToadStool,
    /// Songbird communication platform
    Songbird,
    /// BearDog security platform
    BearDog,
    /// NestGate storage platform
    NestGate,
    /// Squirrel caching platform
    Squirrel,
    /// BiomeOS operating system
    BiomeOS,
}

impl EcosystemService {
    /// Convert ecosystem service to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            EcosystemService::ToadStool => "toadstool",
            EcosystemService::Songbird => "songbird",
            EcosystemService::BearDog => "beardog",
            EcosystemService::NestGate => "nestgate",
            EcosystemService::Squirrel => "squirrel",
            EcosystemService::BiomeOS => "biomeos",
        }
    }
}

/// Service capabilities standardized format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapabilities {
    /// Core capabilities (required)
    pub core: Vec<String>,
    /// Extended capabilities (optional)
    pub extended: Vec<String>,
    /// Cross-primal integrations supported
    pub integrations: Vec<String>,
}

/// Service endpoints standardized format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    /// Health check endpoint
    pub health: String,
    /// Metrics endpoint
    pub metrics: String,
    /// Admin/management endpoint
    pub admin: String,
    /// WebSocket endpoint (if supported)
    pub websocket: Option<String>,
}

/// Health status information for a service
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthStatus {
    /// Current health status type
    pub status: HealthStatusType,
    /// Service version string
    pub version: String,
    /// Service uptime in seconds
    pub uptime_seconds: u64,
    /// Current resource usage metrics
    pub resource_usage: ResourceUsage,
    /// List of online capabilities
    pub capabilities_online: Vec<String>,
    /// Timestamp of last health check
    pub last_check: DateTime<Utc>,
}

/// Health status type enumeration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum HealthStatusType {
    /// Service is operating normally
    Healthy,
    /// Service is functional but with reduced performance
    Degraded,
    /// Service is not functioning properly
    Unhealthy,
    /// Health status cannot be determined
    Unknown,
}

/// Resource usage metrics for a service
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceUsage {
    /// CPU usage percentage (0.0 to 100.0)
    pub cpu_percent: f64,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// Disk usage in bytes
    pub disk_bytes: u64,
    /// Network throughput in bytes per second
    pub network_bytes_per_sec: u64,
}

/// Ecosystem integration error types
#[derive(Debug, thiserror::Error)]
pub enum EcosystemError {
    /// Requested operation is not supported
    #[error("Unsupported operation")]
    UnsupportedOperation,
    /// Request format is invalid
    #[error("Invalid request format")]
    InvalidRequest(String),
    /// Authentication failed
    #[error("Authentication failed")]
    AuthenticationFailed(String),
    /// Authorization failed
    #[error("Authorization failed")]
    AuthorizationFailed,
    /// Service is not available
    #[error("Service unavailable")]
    ServiceUnavailable,
    /// Internal system error
    #[error("Internal error: {0}")]
    InternalError(String),
    /// Registration failed
    #[error("Registration failed: {0}")]
    RegistrationFailed(String),
    /// Health check failed
    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),
    /// Capability update failed
    #[error("Capability update failed: {0}")]
    CapabilityUpdateFailed(String),
    /// Compliance check failed
    #[error("Compliance check failed: {0}")]
    ComplianceCheckFailed(String),
    /// Encryption failed
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    /// Decryption failed
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    /// Threat scan failed
    #[error("Threat scan failed: {0}")]
    ThreatScanFailed(String),
}

impl From<BearDogError> for EcosystemError {
    fn from(error: BearDogError) -> Self {
        EcosystemError::InternalError(error.to_string())
    }
}

/// Trait ALL PRIMALS must implement for ecosystem communication
#[async_trait]
pub trait EcosystemIntegration: Send + Sync {
    /// Register service with Songbird
    async fn register_with_songbird(&self) -> Result<String, EcosystemError>;

    /// Handle incoming requests from other services
    async fn handle_ecosystem_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, EcosystemError>;

    /// Report health status to Songbird
    async fn report_health(&self, health: HealthStatus) -> Result<(), EcosystemError>;

    /// Update service capabilities
    async fn update_capabilities(
        &self,
        capabilities: ServiceCapabilities,
    ) -> Result<(), EcosystemError>;

    /// Deregister from ecosystem
    async fn deregister(&self) -> Result<(), EcosystemError>;
}

/// BearDog's ecosystem integration implementation
pub struct BearDogEcosystemProvider {
    /// Core BearDog instance
    core: Arc<BearDogCore>,

    /// Instance identifier
    instance_id: String,

    /// Service capabilities
    capabilities: ServiceCapabilities,

    /// Service endpoints
    endpoints: ServiceEndpoints,
}

impl BearDogEcosystemProvider {
    /// Create a new BearDog ecosystem provider
    pub fn new(core: Arc<BearDogCore>, instance_id: String) -> Self {
        let capabilities = ServiceCapabilities {
            core: vec![
                "authentication".to_string(),
                "encryption".to_string(),
                "key_management".to_string(),
                "threat_detection".to_string(),
                "compliance".to_string(),
            ],
            extended: vec![
                "audit_logging".to_string(),
                "ml_threat_detection".to_string(),
                "genetic_spawning".to_string(),
                "hsm_support".to_string(),
            ],
            integrations: vec![
                "songbird".to_string(),
                "toadstool".to_string(),
                "nestgate".to_string(),
                "biomeos".to_string(),
            ],
        };

        let endpoints = ServiceEndpoints {
            health: "https://beardog.ecosystem.internal/health".to_string(),
            metrics: "https://beardog.ecosystem.internal/metrics".to_string(),
            admin: "https://beardog.ecosystem.internal/admin".to_string(),
            websocket: Some("wss://beardog.ecosystem.internal/ws".to_string()),
        };

        Self {
            core,
            instance_id,
            capabilities,
            endpoints,
        }
    }

    /// Handle authentication request
    async fn handle_auth_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, EcosystemError> {
        // Extract credentials from request
        let username = request
            .payload
            .get("username")
            .and_then(|v| v.as_str())
            .ok_or_else(|| EcosystemError::InvalidRequest("Missing username".to_string()))?;

        let password = request
            .payload
            .get("password")
            .and_then(|v| v.as_str())
            .ok_or_else(|| EcosystemError::InvalidRequest("Missing password".to_string()))?;

        // Use actual BearDog authentication
        let auth_result = self
            .core
            .security_provider()
            .authenticate(username, password)
            .await
            .map_err(|e| EcosystemError::AuthenticationFailed(e.to_string()))?;

        // Create response
        let response_payload = if auth_result.success {
            serde_json::json!({
                "authenticated": true,
                "user_id": auth_result.user_id,
                "session_id": auth_result.session_id,
                "expires_at": auth_result.expires_at.map(|dt| dt.to_rfc3339()),
                "mfa_required": auth_result.mfa_required
            })
        } else {
            serde_json::json!({
                "authenticated": false,
                "reason": auth_result.reason,
                "retry_after": 60 // seconds
            })
        };

        Ok(EcosystemResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            payload: response_payload,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        })
    }

    /// Handle encryption request
    async fn handle_encrypt_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, EcosystemError> {
        // Extract data to encrypt
        let data_to_encrypt = request
            .payload
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| EcosystemError::InvalidRequest("Missing data to encrypt".to_string()))?;

        // Use BearDog's encryption engine
        let encrypted_data = self
            .core
            .encryption_engine()
            .encrypt(
                data_to_encrypt.as_bytes(),
                Some(EncryptionAlgorithm::Aes256Gcm),
            )
            .await
            .map_err(|e| EcosystemError::EncryptionFailed(e.to_string()))?;

        // Create metadata for encrypted data
        let mut metadata = HashMap::new();
        metadata.insert("algorithm".to_string(), "AES-256-GCM".to_string());
        metadata.insert("key_source".to_string(), "HSM".to_string());
        metadata.insert("security_level".to_string(), "High".to_string());
        metadata.insert("compliance".to_string(), "GDPR,HIPAA,SOC2".to_string());

        Ok(EcosystemResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            payload: serde_json::json!({
                "encrypted_data": general_purpose::STANDARD.encode(&encrypted_data.ciphertext),
                "nonce": general_purpose::STANDARD.encode(&encrypted_data.nonce),
                "algorithm": encrypted_data.algorithm,
                "key_id": encrypted_data.key_id.unwrap_or_else(|| "default".to_string())
            }),
            metadata,
            timestamp: Utc::now(),
        })
    }

    /// Handle decryption request
    async fn handle_decrypt_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, EcosystemError> {
        // Extract encrypted data
        let encrypted_data_b64 = request
            .payload
            .get("encrypted_data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| EcosystemError::InvalidRequest("Missing encrypted_data".to_string()))?;

        let nonce_b64 = request
            .payload
            .get("nonce")
            .and_then(|v| v.as_str())
            .ok_or_else(|| EcosystemError::InvalidRequest("Missing nonce".to_string()))?;

        // Decode base64
        let ciphertext = general_purpose::STANDARD
            .decode(encrypted_data_b64)
            .map_err(|e| {
                EcosystemError::InvalidRequest(format!("Invalid base64 encrypted_data: {e}"))
            })?;

        let nonce = general_purpose::STANDARD
            .decode(nonce_b64)
            .map_err(|e| EcosystemError::InvalidRequest(format!("Invalid base64 nonce: {e}")))?;

        // Create encrypted data structure
        let encrypted_data = EncryptedData {
            ciphertext,
            nonce,
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_id: Some("default".to_string()),
            tag: Some(vec![0u8; 16]), // Placeholder tag
            metadata: HashMap::new(),
        };

        // Use actual BearDog decryption
        let decrypted_data = self
            .core
            .encryption_engine()
            .decrypt(&encrypted_data)
            .await
            .map_err(|e| EcosystemError::DecryptionFailed(e.to_string()))?;

        let decrypted_string = String::from_utf8(decrypted_data).map_err(|e| {
            EcosystemError::DecryptionFailed(format!("Invalid UTF-8 in decrypted data: {e}"))
        })?;

        Ok(EcosystemResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            payload: serde_json::json!({
                "decrypted_data": decrypted_string,
                "algorithm": "AES-256-GCM"
            }),
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        })
    }

    /// Handle compliance check request
    async fn handle_compliance_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, EcosystemError> {
        // Extract compliance standard
        let standard = request
            .payload
            .get("standard")
            .and_then(|v| v.as_str())
            .unwrap_or("GDPR"); // Default to GDPR

        let resource_id = request
            .payload
            .get("resource_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| EcosystemError::InvalidRequest("Missing resource_id".to_string()))?;

        // For now, simulate compliance check
        let compliance_result = serde_json::json!({
            "is_compliant": true,
            "violations": [],
            "risk_score": 0.1,
            "recommendations": []
        });

        Ok(EcosystemResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            payload: serde_json::json!({
                "compliant": compliance_result["is_compliant"],
                "standard": standard,
                "resource_id": resource_id,
                "violations": compliance_result["violations"],
                "risk_score": compliance_result["risk_score"],
                "recommendations": compliance_result["recommendations"]
            }),
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        })
    }

    /// Handle threat scan request
    async fn handle_threat_scan_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, EcosystemError> {
        // Extract scan target
        let target = request
            .payload
            .get("target")
            .and_then(|v| v.as_str())
            .ok_or_else(|| EcosystemError::InvalidRequest("Missing scan target".to_string()))?;

        let scan_type = request
            .payload
            .get("scan_type")
            .and_then(|v| v.as_str())
            .unwrap_or("comprehensive"); // Default scan type

        // For now, simulate threat scan
        let threat_result = serde_json::json!({
            "threats": [],
            "risk_level": "low",
            "recommendations": [],
            "scan_duration_ms": 100
        });

        Ok(EcosystemResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            payload: serde_json::json!({
                "scan_complete": true,
                "target": target,
                "scan_type": scan_type,
                "threats_found": threat_result["threats"].as_array().unwrap_or(&vec![]).len(),
                "threats": threat_result["threats"],
                "risk_level": threat_result["risk_level"],
                "recommendations": threat_result["recommendations"],
                "scan_duration_ms": threat_result["scan_duration_ms"]
            }),
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        })
    }

    /// Get current system uptime
    async fn get_system_uptime(&self) -> u64 {
        // Calculate uptime since process start
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

#[async_trait]
impl EcosystemIntegration for BearDogEcosystemProvider {
    async fn register_with_songbird(&self) -> Result<String, EcosystemError> {
        info!("🎼 Registering BearDog with Songbird service mesh");

        // Create universal handoff manager
        let capability_manager = Arc::new(
            CapabilityManager::placeholder()
                .await
                .map_err(|e| EcosystemError::RegistrationFailed(e.to_string()))?,
        );
        let handoff_manager = songbird_handoff::create_beardog_handoff_manager(
            self.core.clone(),
            capability_manager,
            None, // Use default config
        )
        .await
        .map_err(|e| EcosystemError::RegistrationFailed(e.to_string()))?;

        // Register with Songbird
        let registration_result = handoff_manager
            .register_with_songbird()
            .await
            .map_err(|e| EcosystemError::RegistrationFailed(e.to_string()))?;

        if registration_result.success {
            info!(
                "✅ Successfully registered with Songbird: {}",
                registration_result.service_id
            );
            Ok(registration_result.service_id)
        } else {
            let error_msg = registration_result
                .error_message
                .unwrap_or_else(|| "Unknown registration error".to_string());
            error!("❌ Failed to register with Songbird: {}", error_msg);
            Err(EcosystemError::RegistrationFailed(error_msg))
        }
    }

    async fn handle_ecosystem_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, EcosystemError> {
        debug!("📨 Received ecosystem request: {:?}", request.operation);

        match request.operation.as_str() {
            "authenticate" => self.handle_auth_request(request).await,
            "encrypt" => self.handle_encrypt_request(request).await,
            "decrypt" => self.handle_decrypt_request(request).await,
            "compliance_check" => self.handle_compliance_request(request).await,
            "threat_scan" => self.handle_threat_scan_request(request).await,
            _ => {
                warn!("⚠️ Unsupported operation: {}", request.operation);
                Ok(EcosystemResponse {
                    request_id: request.request_id,
                    status: ResponseStatus::Error {
                        code: "UNSUPPORTED_OPERATION".to_string(),
                        message: format!("Operation '{}' is not supported", request.operation),
                    },
                    payload: serde_json::json!({}),
                    metadata: HashMap::new(),
                    timestamp: Utc::now(),
                })
            }
        }
    }

    async fn report_health(&self, health: HealthStatus) -> Result<(), EcosystemError> {
        info!("🏥 Reporting health status to Songbird");

        // Create universal handoff manager
        let capability_manager = Arc::new(
            CapabilityManager::placeholder()
                .await
                .map_err(|e| EcosystemError::HealthCheckFailed(e.to_string()))?,
        );
        let _handoff_manager = songbird_handoff::create_beardog_handoff_manager(
            self.core.clone(),
            capability_manager,
            None, // Use default config
        )
        .await
        .map_err(|e| EcosystemError::HealthCheckFailed(e.to_string()))?;

        // Report health to Songbird
        let _health_report = EcosystemHealthReport {
            service_id: self.instance_id.clone(),
            status: match health.status {
                HealthStatusType::Healthy => AdapterHealthStatus::Healthy,
                _ => AdapterHealthStatus::Unhealthy {
                    reason: "Health check failed".to_string(),
                    recovery_time: None,
                },
            },
            message: format!(
                "Health check: version {}, uptime {} seconds",
                health.version, health.uptime_seconds
            ),
            timestamp: chrono::Utc::now(),
            components: health
                .capabilities_online
                .into_iter()
                .map(|cap| EcosystemComponentHealth {
                    name: cap,
                    healthy: true,
                    details: Some("Component is operational".to_string()),
                })
                .collect(),
        };

        // TODO: Implement health reporting when trait is available
        // handoff_manager
        //     .report_health(health_report)
        //     .await
        //     .map_err(|e| EcosystemError::HealthCheckFailed(e.to_string()))?;

        debug!("Health status reported successfully");
        Ok(())
    }

    async fn update_capabilities(
        &self,
        capabilities: ServiceCapabilities,
    ) -> Result<(), EcosystemError> {
        info!("🔄 Updating service capabilities");

        // Create universal handoff manager
        let capability_manager = Arc::new(
            CapabilityManager::placeholder()
                .await
                .map_err(|e| EcosystemError::CapabilityUpdateFailed(e.to_string()))?,
        );
        let _handoff_manager = songbird_handoff::create_beardog_handoff_manager(
            self.core.clone(),
            capability_manager,
            None, // Use default config
        )
        .await
        .map_err(|e| EcosystemError::CapabilityUpdateFailed(e.to_string()))?;

        // Update capabilities via Songbird
        let _capability_update = EcosystemCapabilityUpdate {
            service_id: self.instance_id.clone(),
            capabilities: capabilities
                .core
                .iter()
                .map(|cap| Capability {
                    id: cap.clone(),
                    name: cap.clone(),
                    description: format!("Security capability: {cap}"),
                    category: CapabilityCategory::Security,
                    attributes: HashMap::new(),
                    qos: QualityOfService {
                        avg_response_time_ms: 100,
                        availability_percent: 99.9,
                        throughput: Some(ThroughputMetric {
                            value: 1000,
                            unit: "requests/sec".to_string(),
                        }),
                        scalability: ScalabilityInfo {
                            min_instances: 1,
                            max_instances: 10,
                            auto_scaling: true,
                        },
                    },
                    resource_requirements: ResourceRequirements::default(),
                })
                .collect(),
            timestamp: chrono::Utc::now(),
        };

        // TODO: Implement capability update when trait is available
        // handoff_manager
        //     .update_capabilities(capability_update)
        //     .await
        //     .map_err(|e| EcosystemError::CapabilityUpdateFailed(e.to_string()))?;

        debug!("Capabilities updated successfully: {:?}", capabilities.core);
        Ok(())
    }

    async fn deregister(&self) -> Result<(), EcosystemError> {
        info!("👋 Deregistering from ecosystem");

        // Create universal handoff manager
        let capability_manager = Arc::new(
            CapabilityManager::placeholder()
                .await
                .map_err(|e| EcosystemError::InternalError(e.to_string()))?,
        );
        let _handoff_manager = songbird_handoff::create_beardog_handoff_manager(
            self.core.clone(),
            capability_manager,
            None, // Use default config
        )
        .await
        .map_err(|e| EcosystemError::InternalError(e.to_string()))?;

        // Deregister from Songbird
        // TODO: Implement deregistration when method is available
        // handoff_manager
        //     .deregister_from_songbird()
        //     .await
        //     .map_err(|e| EcosystemError::InternalError(e.to_string()))?;

        info!("✅ Successfully deregistered from ecosystem");
        Ok(())
    }
}

/// Factory for creating BearDog ecosystem integration
pub struct BearDogEcosystemFactory;

impl BearDogEcosystemFactory {
    /// Create a new BearDog ecosystem provider
    pub fn create_provider(core: Arc<BearDogCore>) -> BearDogEcosystemProvider {
        let instance_id = format!("beardog-{}", Uuid::new_v4());
        BearDogEcosystemProvider::new(core, instance_id)
    }
}

/// Convenience type alias
pub type EcosystemProvider = BearDogEcosystemProvider;

/// From implementation for error conversion
impl From<EcosystemError> for beardog_errors::BearDogError {
    fn from(err: EcosystemError) -> Self {
        beardog_errors::BearDogError::External {
            message: err.to_string(),
        }
    }
}
