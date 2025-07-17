//! Ecosystem Integration Module
//!
//! Provides types and traits for ecosystem-wide integration and communication.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security levels for ecosystem operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityLevel {
    /// Internal security level
    Internal,
    /// Standard security level
    Standard,
    /// Enhanced security level
    Enhanced,
    /// Classified security level
    Classified,
    /// Top secret security level
    TopSecret,
}

/// Response status for ecosystem operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseStatus {
    /// Operation successful
    Success,
    /// Operation failed with error
    Error { code: String, message: String },
    /// Operation timed out
    Timeout,
    /// Service unavailable
    ServiceUnavailable,
}

/// Health status for ecosystem components
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    /// Component is healthy
    Healthy,
    /// Component is degraded
    Degraded { message: String },
    /// Component is unhealthy
    Unhealthy { message: String },
    /// Component is starting
    Starting,
    /// Component is stopping
    Stopping,
}

/// Security context for ecosystem operations
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

/// Ecosystem request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemRequest {
    /// Request ID
    pub request_id: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Source service
    pub source_service: String,
    /// Target service
    pub target_service: String,
    /// Request operation
    pub operation: String,
    /// Request payload
    pub payload: serde_json::Value,
    /// Security context
    pub security_context: SecurityContext,
    /// Request metadata
    pub metadata: HashMap<String, String>,
}

/// Ecosystem response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemResponse {
    /// Request ID this response is for
    pub request_id: String,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
    /// Response status
    pub status: ResponseStatus,
    /// Response payload
    pub payload: HashMap<String, serde_json::Value>,
    /// Response metadata
    pub metadata: HashMap<String, String>,
}

/// Service capabilities structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapabilities {
    /// Service name
    pub service_name: String,
    /// Service version
    pub version: String,
    /// Supported operations
    pub operations: Vec<String>,
    /// Security requirements
    pub security_requirements: Vec<SecurityLevel>,
    /// Health status
    pub health_status: HealthStatus,
}

/// Ecosystem error types
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum EcosystemError {
    /// Registration failed
    #[error("Registration failed: {message}")]
    RegistrationFailed { message: String },
    /// Authentication failed
    #[error("Authentication failed: {message}")]
    AuthenticationFailed { message: String },
    /// Authorization failed
    #[error("Authorization failed: {message}")]
    AuthorizationFailed { message: String },
    /// Service unavailable
    #[error("Service unavailable: {service}")]
    ServiceUnavailable { service: String },
    /// Communication error
    #[error("Communication error: {message}")]
    CommunicationError { message: String },
    /// Validation error
    #[error("Validation error: {message}")]
    ValidationError { message: String },
}

/// Ecosystem integration trait
#[async_trait]
pub trait EcosystemIntegration {
    /// Register service with Songbird
    async fn register_with_songbird(&self) -> Result<String, EcosystemError>;

    /// Handle ecosystem request
    async fn handle_ecosystem_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, EcosystemError>;

    /// Report health status to Songbird
    async fn report_health(&self, health: HealthStatus) -> Result<(), EcosystemError>;

    /// Update capabilities
    async fn update_capabilities(
        &self,
        capabilities: ServiceCapabilities,
    ) -> Result<(), EcosystemError>;

    /// Deregister from ecosystem
    async fn deregister(&self) -> Result<(), EcosystemError>;
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            auth_token: None,
            identity: "unknown".to_string(),
            permissions: vec![],
            security_level: SecurityLevel::Internal,
        }
    }
}

impl Default for ServiceCapabilities {
    fn default() -> Self {
        Self {
            service_name: "unknown".to_string(),
            version: "0.1.0".to_string(),
            operations: vec![],
            security_requirements: vec![SecurityLevel::Internal],
            health_status: HealthStatus::Starting,
        }
    }
}
