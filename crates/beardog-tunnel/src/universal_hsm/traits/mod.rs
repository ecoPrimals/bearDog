

use beardog_errors::{BearDogError, BearDogResult};
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

    pub is_healthy: bool,

    pub error_message: Option<String>,

    pub last_check: DateTime<Utc>,

    pub response_time_ms: Option<f64>,

    pub capabilities_verified: bool,

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderType {

    MobileHardware,

    DesktopHardware,

    Software,

    Pkcs11,

    Tpm,

    Cloud,

    UsbToken,

    NetworkHsm,

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

    Mobile,

    Desktop,

    Server,

    Embedded,

    Wasm,

    Universal,

#[derive(Debug, Clone)]
pub struct KeyGenerationRequest {

    pub key_type: KeyType,

    pub metadata: KeyMetadata,

    pub use_human_entropy: bool,

    pub min_security_level: crate::SecurityLevel,

    pub require_attestation: bool,

    pub require_biometric: bool,

pub struct SigningRequest {

    pub key_id: String,

    pub data: Vec<u8>,

    pub algorithm: Option<String>,

    pub require_user_presence: bool,

    pub auth_context: Option<AuthenticationContext>,

pub struct VerificationRequest {

    pub signature: Vec<u8>,

pub struct AuthenticationContext {

    pub user_id: Option<String>,

    pub auth_method: AuthenticationMethod,

    pub authenticated_at: DateTime<Utc>,

    pub additional_data: HashMap<String, String>,

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthenticationMethod {

    None,

    Pin,

    Biometric,

    Token,

    MultiFactor,

pub struct OperationResult<T> {

    pub data: T,

    pub metadata: OperationMetadata,

pub struct OperationMetadata {

    pub operation_id: String,

    pub operation_type: String,

    pub started_at: DateTime<Utc>,

    pub completed_at: DateTime<Utc>,

    pub duration_ms: f64,

    pub success: bool,

impl<T> OperationResult<T> {

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
                provider_id,
                operation_type,
                started_at,
                completed_at,
                duration_ms,
                success: true,
                error_message: None,
                additional_data: HashMap::with_capacity(16),
            },

    pub fn failure(
        error: &str,
        _provider_id: &str,
        _operation_type: &str,
        _started_at: DateTime<Utc>,
    ) -> BearDogResult<Self> {
        Err(BearDogError::Hsm(error))
#[cfg(test)]
mod tests {
    use super::*;
    #[test]}

    fn test_provider_type_display() -> beardog_errors::BearDogResult<()> {
        assert_eq!(
            ProviderType::MobileHardware.to_string(),
            "Mobile Hardware Security"
        );
            ProviderType::DesktopHardware.to_string(),
            "Desktop Hardware Security"
        assert_eq!(ProviderType::Software.to_string(), "Software HSM");
        Ok(())
    fn test_provider_info_creation() -> beardog_errors::BearDogResult<()> {
        let info = ProviderInfo {
            provider_id: "test_provider".to_string(),
            name: "Test Provider".to_string(),
            version: "1.0.0".to_string(),
            provider_type: ProviderType::Software,
            security_level: crate::SecurityLevel::Software,
            supports_attestation: false,
            supports_biometric: false,
            supports_human_entropy: false,
            supported_key_types: vec![KeyType::Ed25519],
            description: "Test `HSM` provider".to_string(),
            vendor: "`BearDog`".to_string(),
            platforms: vec![Platform::Linux],
        };
        assert_eq!(info.name, "Test Provider");
        assert_eq!(info.provider_type, ProviderType::Software);
    fn test_operation_result_success() -> beardog_errors::BearDogResult<()> {
        let started_at = Utc::now();
        let result = OperationResult::success(
            "test_data".to_string(),
            "test_provider".to_string(),
            "test_operation".to_string(),
            started_at,
        assert_eq!(result.data, "test_data");
        assert!(result.metadata.success);
        assert!(result.metadata.duration_ms >= 0.0);
