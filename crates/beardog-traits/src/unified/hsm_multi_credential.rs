// SPDX-License-Identifier: AGPL-3.0-only

//! # Universal Multi-Credential HSM Provider Trait
//!
//! This module defines vendor-agnostic traits for hardware security modules that support
//! **multiple cryptographic identities** (credentials/roles) on a single device.
//!
//! ## 🎯 **Design Philosophy**
//!
//! - **Vendor Agnostic**: Works with SoloKeys, YubiKeys, Nitrokeys, TPMs, and future hardware
//! - **Protocol Agnostic**: Abstracts FIDO2, PKCS#11, TPM 2.0, and proprietary protocols
//! - **Role-Based**: Supports multiple identities with different permissions per device
//! - **Hardware-First**: Private keys never leave secure hardware
//! - **Deterministic**: Enables cross-device key derivation from shared entropy
//!
//! ## 🏗️ **Architecture**
//!
//! ```text
//! MultiCredentialHsmProvider
//! ├── Credential Management (create, list, delete)
//! ├── Role Assignment (admin, operator, auditor, etc.)
//! ├── Hierarchical Derivation (parent → child credentials)
//! ├── Hardware Entropy (true random number generation)
//! └── Cross-Device Replication (via deterministic derivation)
//! ```
//!
//! ## 📚 **Usage Example**
//!
//! ```rust,ignore
//! // Example of using multi-credential HSM provider
//! use beardog_traits::unified::hsm_multi_credential::*;
//!
//! async fn setup_security_roles(hsm: impl MultiCredentialHsmProvider) {
//!     // Create admin credential
//!     let admin = hsm.create_credential(CredentialRequest::new("admin")).await;
//!     
//!     // Create operator credential (child of admin)
//!     let operator = hsm.create_credential(
//!         CredentialRequest::new("operator")
//!             .with_parent(admin.credential_id.clone())
//!     ).await;
//!     
//!     // List all credentials on device
//!     let creds = hsm.list_credentials().await;
//!     println!("Device has {} credentials", creds.len());
//! }
//! ```
//!
//! ## 🔐 **Supported Hardware**
//!
//! - **FIDO2 Devices**: SoloKeys, YubiKey 5, Nitrokey FIDO2, Titan Security Key
//! - **PKCS#11 Devices**: YubiKey PIV, OpenSC smart cards, HSM cards
//! - **TPM 2.0**: Discrete and firmware TPM modules
//! - **Mobile Hardware**: Android StrongBox (Pixel Titan M2), iOS Secure Enclave
//! - **Future**: OpenPGP cards, proprietary HSMs

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// CORE TRAIT: MultiCredentialHsmProvider
// ============================================================================

/// Universal multi-credential HSM provider trait
///
/// This trait provides multi-credential operations for hardware security modules,
/// supporting multiple cryptographic identities (credentials) on a single hardware device,
/// with role-based access control.
///
/// Note: This trait is independent of `HsmProvider` to allow simpler implementation.
/// Providers can optionally implement both traits for full HSM + multi-credential support.
pub trait MultiCredentialHsmProvider: Send + Sync + 'static {
    /// Error type for provider operations
    type Error: std::error::Error + Send + Sync + 'static;
    /// **Create a new credential on the device**
    ///
    /// Creates a new cryptographic identity with specified role and permissions.
    /// The private key is generated in hardware and never leaves the device.
    ///
    /// # Arguments
    /// * `request` - Credential creation parameters (role, permissions, parent, etc.)
    ///
    /// # Returns
    /// * `CredentialInfo` - Information about the created credential
    ///
    /// # Hardware Operations
    /// - FIDO2: `authenticatorMakeCredential` CTAP2 command
    /// - PKCS#11: `C_GenerateKeyPair` with metadata
    /// - TPM 2.0: `TPM2_Create` in hierarchy
    fn create_credential(
        &self,
        request: CredentialRequest,
    ) -> impl std::future::Future<Output = Result<CredentialInfo, Self::Error>> + Send;

    /// **List all credentials on the device**
    ///
    /// Returns information about all stored credentials, including their roles,
    /// permissions, and usage statistics.
    ///
    /// # Returns
    /// * `Vec<CredentialInfo>` - List of all credentials
    ///
    /// # Hardware Operations
    /// - FIDO2: `authenticatorCredentialManagement` enumerate
    /// - PKCS#11: `C_FindObjects` with filter
    /// - TPM 2.0: Enumerate persistent handles
    fn list_credentials(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<CredentialInfo>, Self::Error>> + Send;

    /// **Delete a credential from the device**
    ///
    /// Permanently removes a credential and its associated private key from hardware.
    /// This operation is irreversible.
    ///
    /// # Arguments
    /// * `credential_id` - Unique identifier of the credential to delete
    ///
    /// # Hardware Operations
    /// - FIDO2: `authenticatorCredentialManagement` delete
    /// - PKCS#11: `C_DestroyObject`
    /// - TPM 2.0: `TPM2_EvictControl`
    fn delete_credential(
        &self,
        credential_id: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// **Get detailed information about a credential**
    ///
    /// Retrieves metadata about a specific credential without exposing the private key.
    ///
    /// # Arguments
    /// * `credential_id` - Unique identifier of the credential
    ///
    /// # Returns
    /// * `CredentialInfo` - Full credential metadata
    fn get_credential_info(
        &self,
        credential_id: &str,
    ) -> impl std::future::Future<Output = Result<CredentialInfo, Self::Error>> + Send;

    /// **Sign data with a specific credential**
    ///
    /// Performs a cryptographic signing operation using the private key associated
    /// with the specified credential. The private key never leaves hardware.
    ///
    /// # Arguments
    /// * `credential_id` - Credential to use for signing
    /// * `data` - Data to sign
    /// * `require_user_presence` - Whether to require button press/biometric
    ///
    /// # Returns
    /// * `Vec<u8>` - Digital signature
    ///
    /// # Hardware Operations
    /// - FIDO2: `authenticatorGetAssertion` with credential ID
    /// - PKCS#11: `C_Sign` with key handle
    /// - TPM 2.0: `TPM2_Sign` with key handle
    fn sign_with_credential(
        &self,
        credential_id: &str,
        data: &[u8],
        require_user_presence: bool,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// **Generate hardware entropy (true random numbers)**
    ///
    /// Uses the device's hardware random number generator to produce
    /// cryptographically secure random bytes.
    ///
    /// # Arguments
    /// * `size` - Number of random bytes to generate
    ///
    /// # Returns
    /// * `Vec<u8>` - Cryptographically secure random bytes
    ///
    /// # Hardware Operations
    /// - FIDO2: `hmac-secret` extension with random salt
    /// - PKCS#11: `C_GenerateRandom`
    /// - TPM 2.0: `TPM2_GetRandom`
    fn generate_hardware_entropy(
        &self,
        size: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// **Derive a child credential from a parent**
    ///
    /// Creates a hierarchical credential relationship where the child credential
    /// has reduced permissions compared to the parent. This enables role delegation.
    ///
    /// # Arguments
    /// * `parent_credential_id` - Parent credential ID
    /// * `child_request` - Child credential parameters
    ///
    /// # Returns
    /// * `CredentialInfo` - Information about the derived child credential
    ///
    /// # Example Hierarchy
    /// ```text
    /// ROOT (admin)
    ///  ├── OPERATOR (read/write)
    ///  │    └── READONLY (read only)
    ///  └── SECURITY (security ops)
    /// ```
    fn derive_child_credential(
        &self,
        parent_credential_id: &str,
        child_request: CredentialRequest,
    ) -> impl std::future::Future<Output = Result<CredentialInfo, Self::Error>> + Send;

    /// **Get credential hierarchy**
    ///
    /// Returns the parent-child relationships between credentials on this device.
    ///
    /// # Returns
    /// * `CredentialHierarchy` - Tree structure of credential relationships
    fn get_credential_hierarchy(
        &self,
    ) -> impl std::future::Future<Output = Result<CredentialHierarchy, Self::Error>> + Send;

    /// **Check device capabilities**
    ///
    /// Returns information about what multi-credential features are supported
    /// by the hardware device.
    ///
    /// # Returns
    /// * `MultiCredentialCapabilities` - Supported features and limits
    fn get_multi_credential_capabilities(&self) -> MultiCredentialCapabilities;

    /// **Replicate credential to another device**
    ///
    /// Uses deterministic key derivation to recreate the same credential on another
    /// device from shared entropy. The private key is never extracted or transmitted.
    ///
    /// # Arguments
    /// * `credential_id` - Credential to replicate
    /// * `target_request` - Credential request for target device (derived from source)
    /// * `shared_entropy` - Shared entropy seed (for deterministic derivation)
    ///
    /// # Returns
    /// * `CredentialReplicationData` - Data needed to replicate on target device
    ///
    /// # Security Note
    /// Both devices must have access to the same shared entropy seed. This should
    /// be established through a secure out-of-band channel or using hardware attestation.
    /// The replication data contains NO private keys - only public key and metadata.
    fn prepare_credential_replication(
        &self,
        credential_id: &str,
        shared_entropy: &[u8],
    ) -> impl std::future::Future<Output = Result<CredentialReplicationData, Self::Error>> + Send;
}

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Request to create a new credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialRequest {
    /// Role name (e.g., "admin", "operator", "auditor")
    pub role: String,

    /// Human-readable display name
    pub display_name: Option<String>,

    /// List of permissions granted to this credential
    pub permissions: Vec<String>,

    /// Whether to require user presence (button press/biometric) for operations
    pub require_user_presence: bool,

    /// Whether to require user verification (PIN/password/biometric)
    pub require_user_verification: bool,

    /// Parent credential ID (for hierarchical derivation)
    pub parent_credential: Option<String>,

    /// Custom metadata (stored on device if supported)
    pub metadata: HashMap<String, String>,

    /// Key algorithm preference (e.g., "ES256", "Ed25519")
    pub algorithm: Option<String>,
}

/// Information about a stored credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialInfo {
    /// Unique credential identifier
    pub credential_id: String,

    /// Role name
    pub role: String,

    /// Display name
    pub display_name: Option<String>,

    /// Permissions
    pub permissions: Vec<String>,

    /// Public key (for verification)
    pub public_key: Vec<u8>,

    /// Public key algorithm
    pub algorithm: String,

    /// When the credential was created
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Last time this credential was used
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,

    /// Number of times this credential has been used
    pub use_count: u64,

    /// Parent credential ID (if this is a derived credential)
    pub parent_credential_id: Option<String>,

    /// Custom metadata
    pub metadata: HashMap<String, String>,

    /// Whether user presence is required
    pub requires_user_presence: bool,

    /// Whether user verification is required
    pub requires_user_verification: bool,
}

/// Hierarchical structure of credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialHierarchy {
    /// Root credentials (no parent)
    pub roots: Vec<CredentialNode>,
}

/// A node in the credential hierarchy tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialNode {
    /// Credential information
    pub credential: CredentialInfo,

    /// Child credentials
    pub children: Vec<Self>,
}

/// Data for replicating a credential to another device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialReplicationData {
    /// Original credential information
    pub source_credential: CredentialInfo,

    /// Derivation path (for deterministic key derivation)
    pub derivation_path: Vec<u32>,

    /// Shared entropy hash (for verification)
    pub entropy_hash: Vec<u8>,

    /// Credential request for target device
    pub target_request: CredentialRequest,
}

/// Device capabilities for multi-credential operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiCredentialCapabilities {
    /// Maximum number of credentials that can be stored
    pub max_credentials: Option<usize>,

    /// Current number of credentials stored
    pub current_credentials: usize,

    /// Whether hierarchical credentials are supported
    pub supports_hierarchical_credentials: bool,

    /// Whether deterministic key derivation is supported
    pub supports_deterministic_derivation: bool,

    /// Whether hardware entropy generation is supported
    pub supports_hardware_entropy: bool,

    /// Maximum entropy bytes per request
    pub max_entropy_bytes: Option<usize>,

    /// Supported key algorithms
    pub supported_algorithms: Vec<String>,

    /// Whether user presence verification is available
    pub supports_user_presence: bool,

    /// Whether user verification (PIN/biometric) is available
    pub supports_user_verification: bool,

    /// Whether credential metadata storage is supported
    pub supports_metadata: bool,

    /// Protocol used by this device
    pub protocol: HsmProtocol,
}

/// HSM communication protocol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HsmProtocol {
    /// FIDO2/CTAP2 (`SoloKeys`, `YubiKey` FIDO2, Nitrokey FIDO2)
    Fido2,

    /// PKCS#11 (`YubiKey` PIV, smart cards, HSMs)
    Pkcs11,

    /// TPM 2.0 (discrete or firmware TPM)
    Tpm2,

    /// `OpenPGP` Card
    OpenPgp,

    /// Android `StrongBox`
    AndroidStrongBox,

    /// iOS Secure Enclave
    IosSecureEnclave,

    /// Proprietary/vendor-specific
    Proprietary(String),
}

// ============================================================================
// HELPER TRAITS FOR PROTOCOL-SPECIFIC IMPLEMENTATIONS
// ============================================================================

/// Trait for converting protocol-specific credential IDs to universal format
pub trait CredentialIdConverter {
    /// Convert from protocol-specific ID to universal ID
    fn to_universal_id(&self, protocol_id: &[u8]) -> String;

    /// Convert from universal ID to protocol-specific ID
    ///
    /// Note: Takes `&self` to allow stateful converters with device-specific mappings
    ///
    /// # Errors
    ///
    /// Implementations return [`BearDogError`] when `universal_id` cannot be mapped to bytes.
    #[expect(
        clippy::wrong_self_convention,
        reason = "trait method naming differs from Rust convention"
    )]
    fn from_universal_id(&self, universal_id: &str) -> Result<Vec<u8>, BearDogError>;
}

/// Trait for mapping `BearDog` permissions to protocol-specific attributes
pub trait PermissionMapper {
    /// Map `BearDog` permissions to protocol-specific attributes
    fn map_permissions(&self, permissions: &[String]) -> HashMap<String, serde_json::Value>;

    /// Map protocol-specific attributes back to `BearDog` permissions
    fn unmap_permissions(&self, attributes: &HashMap<String, serde_json::Value>) -> Vec<String>;
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_request_serialization() {
        let request = CredentialRequest {
            role: "admin".to_string(),
            display_name: Some("Admin User".to_string()),
            permissions: vec!["read".to_string(), "write".to_string()],
            require_user_presence: true,
            require_user_verification: false,
            parent_credential: None,
            metadata: HashMap::new(),
            algorithm: Some("ES256".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CredentialRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(request.role, deserialized.role);
    }

    #[test]
    fn test_hsm_protocol_equality() {
        assert_eq!(HsmProtocol::Fido2, HsmProtocol::Fido2);
        assert_ne!(HsmProtocol::Fido2, HsmProtocol::Pkcs11);
        assert_eq!(
            HsmProtocol::Proprietary("custom".to_string()),
            HsmProtocol::Proprietary("custom".to_string())
        );
    }

    #[test]
    fn test_multi_credential_capabilities() {
        let caps = MultiCredentialCapabilities {
            max_credentials: Some(50),
            current_credentials: 5,
            supports_hierarchical_credentials: true,
            supports_deterministic_derivation: true,
            supports_hardware_entropy: true,
            max_entropy_bytes: Some(64),
            supported_algorithms: vec!["ES256".to_string(), "Ed25519".to_string()],
            supports_user_presence: true,
            supports_user_verification: true,
            supports_metadata: true,
            protocol: HsmProtocol::Fido2,
        };

        assert_eq!(caps.max_credentials, Some(50));
        assert!(caps.supports_hierarchical_credentials);
        assert_eq!(caps.protocol, HsmProtocol::Fido2);
    }
}
