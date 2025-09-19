

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod access_control;
pub mod compliance_sovereignty;
pub mod crypto_sovereignty;
pub mod trust_management;

pub use access_control::AccessController;
pub use compliance_sovereignty::ComplianceSovereignty;
pub use crypto_sovereignty::CryptoSovereignty;
pub use trust_management::TrustManager;

pub trait SecuritySovereigntyOps: Send + Sync {


    fn establish_crypto_sovereignty(&str,
    ) -> impl std::future::Future<
        Output = Result<CryptoSovereigntyResult, crate::BearDogSecurityError>,
    ;


    fn verify_trust(&str,
    ) -> impl std::future::Future<
        Output = Result<TrustVerificationResult, crate::BearDogSecurityError>,
    ;
}

#[derive(Debug, Clone)]
    /// The crypto keys value
    pub crypto_keys: CryptoKeySet,
    /// The established at value
    pub established_at: chrono::DateTime<chrono::Utc>,
    /// The sovereignty level value
    pub sovereignty_level: SovereigntyLevel,
}

#[derive(Debug, Clone)]
    /// Collection of signing key
    pub signing_key: Vec<u8>,
    /// Collection of encryption key
    pub encryption_key: Vec<u8>,
    /// Collection of key derivation salt
    pub key_derivation_salt: Vec<u8>,
}

#[derive(Debug, Clone)]
    /// The trust level value
    pub trust_level: TrustLevel,
    /// Collection of verification proof
    pub verification_proof: Vec<u8>,
    /// The verified at value
    pub verified_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq)]

pub enum SovereigntyLevel {
    /// Represents basic variant
    Basic,
    /// State indicating enhanced
    Enhanced,
    /// Represents maximum variant
    Maximum,
    /// Represents absolute variant
    Absolute,
}

#[derive(Debug, Clone, PartialEq)]

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
