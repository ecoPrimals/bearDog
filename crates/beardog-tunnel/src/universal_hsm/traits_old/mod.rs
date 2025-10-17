//! Universal HSM Traits
//!
//! Common trait definitions for universal HSM providers

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Sub-modules
pub mod attestation;
pub mod entropy;
pub mod provider;

// Re-exports
pub use attestation::{AttestationData, AttestationLevel};
pub use entropy::{EphemeralSeed, HumanEntropyCapabilities, HumanEntropyData, HumanEntropyMethod};
pub use provider::UniversalHsmProvider;

/// Provider health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealth {
    /// Whether provider is healthy
    pub is_healthy: bool,
    /// Optional error message
    pub error_message: Option<String>,
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    /// Response time in milliseconds
    pub response_time_ms: Option<f64>,
    /// Whether capabilities have been verified
    pub capabilities_verified: bool,
}

/// Provider type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderType {
    /// Mobile hardware security
    MobileHardware,
    /// Desktop hardware security
    DesktopHardware,
    /// Software HSM
    Software,
    /// PKCS#11 HSM
    Pkcs11,
    /// TPM 2.0+
    Tpm,
    /// Cloud HSM
    Cloud,
    /// USB security token
    UsbToken,
    /// Network HSM
    NetworkHsm,
    /// Custom provider type
    Custom,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::MobileHardware => write!(f, "Mobile Hardware Security"),
            ProviderType::DesktopHardware => write!(f, "Desktop Hardware Security"),
            ProviderType::Software => write!(f, "Software HSM"),
            ProviderType::Pkcs11 => write!(f, "PKCS#11 HSM"),
            ProviderType::Tpm => write!(f, "TPM 2.0+"),
            ProviderType::Cloud => write!(f, "Cloud HSM"),
            ProviderType::UsbToken => write!(f, "USB Security Token"),
            ProviderType::NetworkHsm => write!(f, "Network HSM"),
            ProviderType::Custom => write!(f, "Custom HSM"),
        }
    }
}

/// Platform enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    /// Mobile platform
    Mobile,
    /// Desktop platform
    Desktop,
    /// Server platform
    Server,
    /// Embedded platform
    Embedded,
    /// WebAssembly platform
    Wasm,
    /// Universal (cross-platform)
    Universal,
}

/// Provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Unique provider identifier
    pub provider_id: String,
    /// Provider name
    pub name: String,
    /// Provider version
    pub version: String,
    /// Provider description
    pub description: String,
    /// Provider vendor
    pub vendor: String,
    /// Provider type
    pub provider_type: ProviderType,
    /// Supported platforms
    pub platforms: Vec<Platform>,
}

/// Authentication method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    /// No authentication required
    None,
    /// PIN/Password authentication
    Pin,
    /// Biometric authentication
    Biometric,
    /// Multi-factor authentication
    MultiFactor,
    /// Custom authentication method
    Custom,
}

/// Authentication context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationContext {
    /// Optional user identifier
    pub user_id: Option<String>,
    /// Authentication method used
    pub auth_method: AuthenticationMethod,
    /// Authentication timestamp
    pub authenticated_at: DateTime<Utc>,
    /// Additional authentication data
    pub additional_data: HashMap<String, String>,
}

/// Operation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationMetadata {
    /// Unique operation identifier
    pub operation_id: String,
    /// Operation type
    pub operation_type: String,
    /// Operation start timestamp
    pub started_at: DateTime<Utc>,
    /// Operation completion timestamp
    pub completed_at: DateTime<Utc>,
    /// Operation duration in milliseconds
    pub duration_ms: f64,
    /// Whether operation succeeded
    pub success: bool,
    /// Optional error message
    pub error_message: Option<String>,
    /// Additional operation data
    pub additional_data: HashMap<String, String>,
}

/// Operation result wrapper
#[derive(Debug, Clone)]
pub struct OperationResult<T> {
    /// Operation result data
    pub data: T,
    /// Operation metadata
    pub metadata: OperationMetadata,
}

impl<T> OperationResult<T> {
    /// Create a successful operation result
    pub fn success(
        data: T,
        provider_id: &str,
        operation_type: &str,
        started_at: DateTime<Utc>,
    ) -> Self {
        let completed_at = Utc::now();
        let duration_ms = (completed_at - started_at).num_milliseconds() as f64;
        
        Self {
            data,
            metadata: OperationMetadata {
                operation_id: uuid::Uuid::new_v4().to_string(),
                operation_type: operation_type.to_string(),
                started_at,
                completed_at,
                duration_ms,
                success: true,
                error_message: None,
                additional_data: HashMap::new(),
            },
        }
    }

    /// Create a failed operation result
    pub fn failure(
        error: BearDogError,
        provider_id: &str,
        operation_type: &str,
        started_at: DateTime<Utc>,
    ) -> Result<Self, BearDogError> {
        Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_type_display() {
        assert_eq!(
            ProviderType::MobileHardware.to_string(),
            "Mobile Hardware Security"
        );
        assert_eq!(
            ProviderType::DesktopHardware.to_string(),
            "Desktop Hardware Security"
        );
        assert_eq!(ProviderType::Software.to_string(), "Software HSM");
    }

    #[test]
    fn test_operation_result_success() {
        let started_at = Utc::now();
        let result = OperationResult::success(
            "test_data".to_string(),
            "test_provider",
            "test_operation",
            started_at,
        );
        
        assert_eq!(result.data, "test_data");
        assert!(result.metadata.success);
        assert!(result.metadata.duration_ms >= 0.0);
    }
}
