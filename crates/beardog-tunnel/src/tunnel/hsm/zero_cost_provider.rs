//! Zero-cost HSM provider abstraction
//!
//! Provides compile-time verified HSM operations with zero runtime overhead
//! through Rust's const generics and trait system.

use crate::tunnel::hsm::types::HsmOperation;
use beardog_errors::BearDogError;
use beardog_types::canonical::HsmKey;
use std::marker::PhantomData;

/// Zero-cost abstraction over HSM providers using compile-time verification
pub struct ZeroCostHsmProvider<P> {
    provider: P,
    _phantom: PhantomData<P>,
}

impl<P> ZeroCostHsmProvider<P>
where
    P: HsmProviderTrait,
{
    /// Creates a new zero-cost HSM provider wrapper
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            _phantom: PhantomData,
        }
    }

    /// Returns the static capability set of this provider
    pub const fn capabilities() -> &'static P::Capabilities {
        P::CAPABILITIES
    }

    /// Execute an HSM operation through the provider
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn execute_operation(&self, operation: HsmOperation) -> Result<HsmKey, BearDogError> {
        self.provider.execute_operation(operation)
    }

    /// Generate a key with compile-time algorithm verification
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn generate_key<A>(&self, algorithm: A) -> Result<HsmKey, BearDogError>
    where
        A: KeyAlgorithm,
        P: SupportsAlgorithm<A>,
    {
        self.provider.generate_key_typed(algorithm)
    }
}

/// HSM provider trait for zero-cost abstractions
pub trait HsmProviderTrait: Send + Sync + 'static {
    /// Associated capabilities type
    type Capabilities: HsmCapabilities;
    /// Static capabilities instance
    const CAPABILITIES: &'static Self::Capabilities;

    /// Executes an HSM operation
    fn execute_operation(&self, operation: HsmOperation) -> Result<HsmKey, BearDogError>;

    /// Generates a typed key using the specified algorithm
    fn generate_key_typed<A>(&self, algorithm: A) -> Result<HsmKey, BearDogError>
    where
        A: KeyAlgorithm,
        Self: SupportsAlgorithm<A>;
}

/// HSM capabilities trait for compile-time capability queries
pub trait HsmCapabilities: Send + Sync + 'static {
    /// Supported algorithm names
    const ALGORITHMS: &'static [&'static str];
    /// Maximum supported key size in bits
    const MAX_KEY_SIZE: usize;
    /// Security level of this HSM
    const SECURITY_LEVEL: SecurityLevel;
}

/// Key algorithm trait for compile-time algorithm verification
pub trait KeyAlgorithm: Send + Sync + 'static {
    /// Algorithm name identifier
    const NAME: &'static str;
    /// Key size in bits
    const KEY_SIZE: usize;
}

/// Algorithm support marker trait
pub trait SupportsAlgorithm<A: KeyAlgorithm>: HsmProviderTrait {}

/// Security level classification for HSM providers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// Pure software implementation
    Software,
    /// Trusted Execution Environment (TEE)
    TrustedExecutionEnvironment,
    /// Hardware Secure Enclave (e.g., Apple SEP)
    SecureEnclave,
    /// Dedicated Hardware Security Module
    HardwareSecurityModule,
}

/// Software HSM capabilities (highest flexibility, lowest assurance)
pub struct SoftwareHsmCapabilities;

impl HsmCapabilities for SoftwareHsmCapabilities {
    const ALGORITHMS: &'static [&'static str] = &["AES-256", "RSA-2048", "ECDSA-P256"];
    const MAX_KEY_SIZE: usize = 4096;
    const SECURITY_LEVEL: SecurityLevel = SecurityLevel::Software;
}

/// Android StrongBox HSM capabilities (Phase 2)
pub struct AndroidStrongboxCapabilities;

impl HsmCapabilities for AndroidStrongboxCapabilities {
    const ALGORITHMS: &'static [&'static str] = &["AES-256", "ECDSA-P256"];
    const MAX_KEY_SIZE: usize = 2048;
    const SECURITY_LEVEL: SecurityLevel = SecurityLevel::HardwareSecurityModule;
}

/// iOS Secure Enclave capabilities (Phase 2)
pub struct IOSSecureEnclaveCapabilities;

impl HsmCapabilities for IOSSecureEnclaveCapabilities {
    const ALGORITHMS: &'static [&'static str] = &["ECDSA-P256"];
    const MAX_KEY_SIZE: usize = 256;
    const SECURITY_LEVEL: SecurityLevel = SecurityLevel::SecureEnclave;
}

/// AES-256 key algorithm marker type
pub struct Aes256;

impl KeyAlgorithm for Aes256 {
    const NAME: &'static str = "AES-256";
    const KEY_SIZE: usize = 256;
}

/// RSA-2048 key algorithm marker type
pub struct Rsa2048;

impl KeyAlgorithm for Rsa2048 {
    const NAME: &'static str = "RSA-2048";
    const KEY_SIZE: usize = 2048;
}

/// ECDSA P-256 key algorithm marker type
pub struct EcdsaP256;

impl KeyAlgorithm for EcdsaP256 {
    const NAME: &'static str = "ECDSA-P256";
    const KEY_SIZE: usize = 256;
}

/// Type alias for backward compatibility
pub type ZeroCostHsmManager<P> = ZeroCostHsmProvider<P>;

/// Migrate an existing HSM provider to the zero-cost abstraction
pub fn migrate_to_zero_cost<P>(provider: P) -> ZeroCostHsmProvider<P>
where
    P: HsmProviderTrait,
{
    ZeroCostHsmProvider::new(provider)
}
