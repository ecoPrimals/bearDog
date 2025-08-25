// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Universal `HSM` Traits - Canonical Definitions
///
/// **SINGLE SOURCE OF TRUTH FOR `HSM` TRAITS**
/// This module defines the canonical `HSM` provider traits that replace all
/// fragmented trait definitions throughout the codebase. All `HSM` providers
/// must implement these unified traits.
/// ## Replaced Trait Definitions
/// - ❌ `hsm_foundation/traits.rs` (ELIMINATED)
/// - ❌ `tunnel/hsm/types/providers.rs` (ELIMINATED)
/// - ❌ `beardog-types/providers.rs` (SUPERSEDED)
/// - ✅ This unified trait system (CANONICAL)

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::crypto::KeyType;
use beardog_types::canonical::hsm::keys::KeyMetadata;
// Use canonical provider info from beardog-types instead of duplicate definition
use beardog_types::canonical::providers::ProviderInfo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod attestation;
pub mod entropy;
pub mod provider;

// Re-exports
pub use attestation::{AttestationData, AttestationLevel};
pub use entropy::{EphemeralSeed, HumanEntropyCapabilities, HumanEntropyData, HumanEntropyMethod};
// Re-export canonical HSM provider from beardog-traits
pub use beardog_traits::HsmProvider;

// Re-export canonical ProviderInfo instead of defining duplicate
pub use beardog_types::canonical::providers::ProviderInfo;
/// **Provider Health Status - Universal Standard**
/// Standardized health information that all `HSM` providers must provide.
pub struct ProviderHealth {
    /// Whether the provider is currently healthy
    pub is_healthy: bool,
    /// Optional error message if unhealthy
    pub error_message: Option<String>,
    /// Timestamp of last health check
    pub last_check: DateTime<Utc>,
    /// Response time for health check (if available)
    pub response_time_ms: Option<f64>,
    /// Whether all capabilities have been verified
    pub capabilities_verified: bool,
/// Standardized classification of HSM provider types - vendor agnostic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderType {
    /// Mobile hardware security (any smartphone with secure hardware)
    MobileHardware,
    /// Desktop/laptop hardware security (any device with secure hardware)
    DesktopHardware,
    /// Software-based HSM implementation
    Software,
    /// PKCS#11 compliant hardware security module (any vendor)
    Pkcs11,
    /// Trusted Platform Module (any TPM 2.0+ device)
    Tpm,
    /// Cloud-based HSM service (any cloud provider)
    Cloud,
    /// USB security token (any FIDO2/PKCS#11 compatible device)
    UsbToken,
    /// Network-attached HSM (any network HSM appliance)
    NetworkHsm,
    /// Custom/proprietary HSM implementation
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
/// **Platform Compatibility** - Universal capability-based classification
/// Platforms where the HSM provider can operate, classified by capabilities
/// rather than specific vendor implementations.
pub enum Platform {
    /// Mobile platforms with hardware security (any smartphone/tablet)
    Mobile,
    /// Desktop/laptop platforms (any OS with hardware security support)
    Desktop,
    /// Server platforms (any server-class hardware)
    Server,
    /// Embedded platforms (any embedded device with security hardware)
    Embedded,
    /// Web assembly environments
    Wasm,
    /// Cloud/virtualized environments
    /// Universal (works on any platform)
    Universal,
/// **Key Generation Request - Universal Standard**
/// Standardized key generation request structure.}


#[derive(Debug, Clone)]
pub struct KeyGenerationRequest {
    /// Type of key to generate
    pub key_type: KeyType,
    /// Key metadata
    pub metadata: KeyMetadata,
    /// Whether to use human entropy (if available)
    pub use_human_entropy: bool,
    /// Required security level
    pub min_security_level: crate::SecurityLevel,
    /// Whether hardware attestation is required
    pub require_attestation: bool,
    /// Whether biometric authentication is required
    pub require_biometric: bool,
/// **Signing Request - Universal Standard**
/// Standardized signing request structure.
pub struct SigningRequest {
    /// Key identifier
    pub key_id: String,
    /// Data to sign
    pub data: Vec<u8>,
    /// Signing algorithm (optional, use key's default if None)
    pub algorithm: Option<String>,
    /// Whether user presence is required
    pub require_user_presence: bool,
    /// Authentication context (if required)
    pub auth_context: Option<AuthenticationContext>,
/// **Signature Verification Request - Universal Standard**
/// Standardized signature verification request structure.
pub struct VerificationRequest {
    /// Original data that was signed
    /// Signature to verify
    pub signature: Vec<u8>,
    /// Verification algorithm (optional, use key's default if None)
/// **Authentication Context**
/// Context information for authenticated operations.
pub struct AuthenticationContext {
    /// User identifier
    pub user_id: Option<String>,
    /// Authentication method used
    pub auth_method: AuthenticationMethod,
    /// Timestamp of authentication
    pub authenticated_at: DateTime<Utc>,
    /// Additional authentication data
    pub additional_data: HashMap<String, String>,
/// **Authentication Methods**
/// Supported authentication methods for `HSM` operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthenticationMethod {
    /// No authentication required
    None,
    /// PIN/password authentication
    Pin,
    /// Biometric authentication (fingerprint, face, etc.)
    Biometric,
    /// Hardware token authentication
    Token,
    /// Multi-factor authentication
    MultiFactor,
/// **Operation Result - Universal Standard**
/// Standardized result structure for `HSM` operations.}


pub struct OperationResult<T> {
    /// Operation result data
    pub data: T,
    /// Operation metadata
    pub metadata: OperationMetadata,
/// **Operation Metadata**
/// Metadata about `HSM` operations for auditing and monitoring.
pub struct OperationMetadata {
    /// Unique operation identifier
    pub operation_id: String,
    /// Provider that performed the operation
    /// Operation type
    pub operation_type: String,
    /// Timestamp when operation started
    pub started_at: DateTime<Utc>,
    /// Timestamp when operation completed
    pub completed_at: DateTime<Utc>,
    /// Operation duration in milliseconds
    pub duration_ms: f64,
    /// Whether operation was successful
    pub success: bool,
    /// Error message (if operation failed)
    /// Additional operation-specific data
impl<T> OperationResult<T> {
    /// Create a successful operation result}


    pub fn success(
        data: T,
        provider_id: String,
        operation_type: String,
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
                additional_data: HashMap::new(),
            },
    /// Create a failed operation result
    pub fn failure(
        error: String,
        _provider_id: String,
        _operation_type: String,
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
