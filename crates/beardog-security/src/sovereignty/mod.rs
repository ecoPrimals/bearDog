//! # Sovereignty Module
//!
//! This module provides security sovereignty operations for the BearDog ecosystem.
//! It handles cryptographic sovereignty, access control, compliance, and trust management.

pub mod access_control;
pub mod compliance_sovereignty;
pub mod crypto_sovereignty;
pub mod trust_management;

pub use access_control::AccessController;
pub use compliance_sovereignty::ComplianceSovereignty;
pub use crypto_sovereignty::CryptoSovereignty;
pub use trust_management::TrustManager;

use crate::BearDogSecurityError;

// ============================================================
// Sovereignty Operations Trait
// ============================================================

/// Security sovereignty operations
pub trait SecuritySovereigntyOps: Send + Sync {
    /// Establish cryptographic sovereignty for an identity
    fn establish_crypto_sovereignty(
        &self,
        identity: &str,
    ) -> impl std::future::Future<Output = Result<CryptoSovereigntyResult, BearDogSecurityError>> + Send;

    /// Verify trust for an entity
    fn verify_trust(
        &self,
        entity: &str,
    ) -> impl std::future::Future<Output = Result<TrustVerificationResult, BearDogSecurityError>> + Send;
}

// ============================================================
// Result Types
// ============================================================

/// Result of establishing cryptographic sovereignty
#[derive(Debug, Clone)]
pub struct CryptoSovereigntyResult {
    /// Unique identifier for this sovereignty
    pub sovereignty_id: String,

    /// Cryptographic keys for the sovereignty
    pub crypto_keys: CryptoKeySet,

    /// When the sovereignty was established
    pub established_at: chrono::DateTime<chrono::Utc>,

    /// Level of sovereignty achieved
    pub sovereignty_level: SovereigntyLevel,
}

/// Cryptographic key set for sovereignty
#[derive(Debug, Clone)]
pub struct CryptoKeySet {
    /// Signing key (private)
    pub signing_key: Vec<u8>,

    /// Encryption key (private)
    pub encryption_key: Vec<u8>,

    /// Key derivation salt
    pub key_derivation_salt: Vec<u8>,

    /// Public signing key
    pub public_signing_key: Option<Vec<u8>>,

    /// Public encryption key
    pub public_encryption_key: Option<Vec<u8>>,
}

/// Result of trust verification
#[derive(Debug, Clone)]
pub struct TrustVerificationResult {
    /// Entity that was verified
    pub entity: String,

    /// Trust level determined
    pub trust_level: TrustLevel,

    /// Cryptographic proof of verification
    pub verification_proof: Vec<u8>,

    /// When the verification occurred
    pub verified_at: chrono::DateTime<chrono::Utc>,

    /// Additional verification metadata
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

// ============================================================
// Level Enums
// ============================================================

/// Sovereignty level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SovereigntyLevel {
    /// Basic sovereignty - minimal guarantees
    Basic,

    /// Enhanced sovereignty - standard security
    Enhanced,

    /// Maximum sovereignty - high security
    Maximum,

    /// Absolute sovereignty - highest possible
    Absolute,
}

impl Default for SovereigntyLevel {
    fn default() -> Self {
        Self::Enhanced
    }
}

/// Trust level for entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TrustLevel {
    /// Untrusted entity
    Untrusted,

    /// Limited trust
    Limited,

    /// Trusted entity
    Trusted,

    /// Highly trusted entity
    HighlyTrusted,

    /// Absolute trust (e.g., self)
    Absolute,
}

impl Default for TrustLevel {
    fn default() -> Self {
        Self::Limited
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereignty_level_ordering() {
        assert!(SovereigntyLevel::Absolute > SovereigntyLevel::Maximum);
        assert!(SovereigntyLevel::Maximum > SovereigntyLevel::Enhanced);
        assert!(SovereigntyLevel::Enhanced > SovereigntyLevel::Basic);
    }

    #[test]
    fn test_trust_level_ordering() {
        assert!(TrustLevel::Absolute > TrustLevel::HighlyTrusted);
        assert!(TrustLevel::HighlyTrusted > TrustLevel::Trusted);
        assert!(TrustLevel::Trusted > TrustLevel::Limited);
        assert!(TrustLevel::Limited > TrustLevel::Untrusted);
    }

    #[test]
    fn test_sovereignty_level_default() {
        assert_eq!(SovereigntyLevel::default(), SovereigntyLevel::Enhanced);
    }

    #[test]
    fn test_trust_level_default() {
        assert_eq!(TrustLevel::default(), TrustLevel::Limited);
    }
}
