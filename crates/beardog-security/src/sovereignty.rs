

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod access_control;
pub mod compliance_sovereignty;
pub mod crypto_sovereignty;
pub mod trust_management;

pub use access_control::AccessController;
pub use compliance_sovereignty::{ComplianceSovereignty, ComplianceStatus, SovereigntyLevel, ComplianceFinding, FindingSeverity};
pub use crypto_sovereignty::CryptoSovereignty;
pub use trust_management::TrustManager;

pub trait SecuritySovereigntyOps: Send + Sync {
    /// Establish crypto sovereignty for a given context
    fn establish_crypto_sovereignty(&self, context_id: &str) -> impl std::future::Future<Output = Result<CryptoSovereigntyResult, crate::BearDogSecurityError>> + Send;

    /// Verify trust for a given entity
    fn verify_trust(&self, entity_id: &str) -> impl std::future::Future<Output = Result<TrustVerificationResult, crate::BearDogSecurityError>> + Send;
}

/// Digital sovereignty context for security operations
#[derive(Debug, Clone)]
pub struct SovereigntyContext {
    /// The crypto keys value
    pub crypto_keys: CryptoKeySet,
    /// The established at value
    pub established_at: chrono::DateTime<chrono::Utc>,
    /// The sovereignty level value
    pub sovereignty_level: SovereigntyLevel,
}

/// Cryptographic key set for sovereignty operations
#[derive(Debug, Clone)]
pub struct CryptoKeySet {
    /// Collection of signing key
    pub signing_key: Vec<u8>,
    /// Collection of encryption key
    pub encryption_key: Vec<u8>,
    /// Collection of key derivation salt
    pub key_derivation_salt: Vec<u8>,
}

/// Sovereignty verification context
#[derive(Debug, Clone)]
pub struct SovereigntyVerification {
    /// The trust level value
    pub trust_level: TrustLevel,
    /// Collection of verification proof
    pub verification_proof: Vec<u8>,
    /// The verified at value
    pub verified_at: chrono::DateTime<chrono::Utc>,
}

// SovereigntyLevel is now defined in compliance_sovereignty module

/// Trust levels for verification
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TrustLevel {
    /// State indicating untrusted
    Untrusted,
    /// State indicating limited
    Limited,
    /// State indicating trusted
    Trusted,
    /// State indicating highlytrusted
    HighlyTrusted,
    /// Represents absolute variant
    Absolute,
}

/// Result of crypto sovereignty establishment
#[derive(Debug, Clone)]
pub struct CryptoSovereigntyResult {
    /// Success status
    pub success: bool,
    /// Context identifier
    pub context_id: String,
    /// Timestamp of establishment
    pub established_at: chrono::DateTime<chrono::Utc>,
}

/// Result of trust verification
#[derive(Debug, Clone)]
pub struct TrustVerificationResult {
    /// Verification status
    pub verified: bool,
    /// Trust level assigned
    pub trust_level: TrustLevel,
    /// Timestamp of verification
    pub verified_at: chrono::DateTime<chrono::Utc>,
}
