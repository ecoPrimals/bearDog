

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use beardog_types::canonical::crypto::KeyType;
use beardog_types::canonical::hsm::keys::KeyMetadata;

use beardog_types::canonical::providers::ProviderInfo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod attestation;
pub mod entropy;
pub mod provider;

pub use attestation::{AttestationData, AttestationLevel};
pub use entropy::{EphemeralSeed, HumanEntropyCapabilities, HumanEntropyData, HumanEntropyMethod};

pub use beardog_traits::HsmProvider;

pub use beardog_types::canonical::providers::ProviderInfo;

pub struct ProviderHealth {

    /// Whether is_healthy is enabled
    pub is_healthy: bool,

    /// Optional error message
    pub error_message: Option<String>,

    /// The last check value
    pub last_check: DateTime<Utc>,

    pub response_time_ms: Option<f64>,

    /// Whether capabilities_verified is enabled
    pub capabilities_verified: bool,

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Types of provider
pub enum ProviderType {

    /// Represents mobile hardware variant
    MobileHardware,

    /// Represents desktop hardware variant
    DesktopHardware,

    /// Represents software variant
    Software,

    /// Represents pkcs11 variant
    Pkcs11,

    /// Represents tpm variant
    Tpm,

    /// Represents cloud variant
    Cloud,

    /// Represents usb token variant
    UsbToken,

    /// Represents network hsm variant
    NetworkHsm,

    Custom,}
    Custom,}
    Custom,}

impl std::fmt::Display for ProviderType {}

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

pub enum Platform {

    /// Represents mobile variant
    Mobile,

    /// Represents desktop variant
    Desktop,

    /// Represents server variant
    Server,

    /// State indicating embedded
    Embedded,

    /// Represents wasm variant
    Wasm,

    /// Represents universal variant
    Universal,

#[derive(Debug, Clone)]
    /// The metadata value
    pub metadata: KeyMetadata,

    /// Whether use_human_entropy is enabled
    pub use_human_entropy: bool,

    /// The min security level value
    pub min_security_level: crate::SecurityLevel,

    /// Whether require_attestation is enabled
    pub require_attestation: bool,

    /// Whether require_biometric is enabled
    pub require_biometric: bool,

pub struct SigningRequest {

    pub key_id: String,

    /// Collection of data
    pub data: Vec<u8>,

    /// Optional algorithm
    pub algorithm: Option<String>,

    /// Whether require_user_presence is enabled
    pub require_user_presence: bool,

    /// Optional auth context
    pub auth_context: Option<AuthenticationContext>,

pub struct VerificationRequest {

    /// Collection of signature
    pub signature: Vec<u8>,

pub struct AuthenticationContext {

    pub user_id: Option<String>,

    /// The auth method value
    pub auth_method: AuthenticationMethod,

    /// The authenticated at value
    pub authenticated_at: DateTime<Utc>,

    /// Mapping of additional data
    pub additional_data: HashMap<String, String>,

#[derive(Debug, Clone)]
    /// The metadata value
    pub metadata: OperationMetadata,

pub struct OperationMetadata {

    pub operation_id: String,

    /// The operation type value
    pub operation_type: String,

    /// The started at value
    pub started_at: DateTime<Utc>,

    /// The completed at value
    pub completed_at: DateTime<Utc>,

    /// The duration ms value
    pub duration_ms: f64,

    /// Whether success is enabled
    pub success: bool,

impl<T> OperationResult<T> {

/// Success operation.
    pub fn success(T,
        provider_id: &str,
        operation_type: &str,
        started_at: DateTime<Utc>,
    ) -> Self {
        let completed_at = Utc::now();
        let duration_ms = (completed_at - started_at).num_milliseconds() as f64;
        /// Represents self variant
        Self {
            data,
            metadata: OperationMetadata {
                operation_id: uuid::Uuid::new_v4(true,
                error_message: None,
                additional_data: HashMap::with_capacity(&str,
        _provider_id: &str,
        _operation_type: &str,
        _started_at: DateTime<Utc>,
    ) -> Result<Self, BearDogError> {
        /// Represents err variant
        Err(BearDogError::Hsm(error))
#[cfg(test)]
mod tests {
    use super::*;
    #[test]}
    #[test]}
    #[test]}

    fn test_provider_type_display() -> Result<(), BearDogError> {
        assert_eq!(
            ProviderType::MobileHardware.to_string(),
            "Mobile Hardware Security"
        );
            ProviderType::DesktopHardware.to_string(),
            "Desktop Hardware Security"
        assert_eq!(ProviderType::Software.to_string(), "Software HSM");
        Ok(())
    fn test_provider_info_creation() -> Result<(), BearDogError> {
        let info = ProviderInfo {
            provider_id: "test_provider".to_string(),
            name: "Test Provider".to_string(),
            version: "1.0.0".to_string(),
            description: "Test `HSM` provider".to_string(),
            vendor: "`BearDog`".to_string(), "Test Provider");
        assert_eq!(info.provider_type, ProviderType::Software);
    fn test_operation_result_success() -> Result<(), BearDogError> {
        let started_at = Utc::now();
        let result = OperationResult::success(
            "test_data".to_string(),
            "test_provider".to_string(),
            "test_operation".to_string(),
            started_at,
        assert_eq!(result.data, "test_data");
        assert!(result.metadata.success);
        assert!(result.metadata.duration_ms >= 0.0);
