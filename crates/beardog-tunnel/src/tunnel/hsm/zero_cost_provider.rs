// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use crate::tunnel::hsm::types::HsmOperation;
use beardog_errors::BearDogError;
use beardog_types::canonical::HsmKey;
use std::marker::PhantomData;

pub struct ZeroCostHsmProvider<P> {
    provider: P,
    _phantom: PhantomData<P>,
}

impl<P> ZeroCostHsmProvider<P>
where
    P: HsmProviderTrait,
{
    /// New operation.
    /// Creates a new instance
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            _phantom: PhantomData,
        }
    }

    pub const fn capabilities() -> &'static P::Capabilities {
        P::CAPABILITIES
    }

    /// Execute Operation operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Executes operation
    /// Executes operation
    pub fn execute_operation(&self, operation: HsmOperation) -> Result<HsmKey, BearDogError> {
        self.provider.execute_operation(operation)
    }

    /// Generate Key operation.
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

/// HSM provider trait
pub trait HsmProviderTrait: Send + Sync + 'static {
    type Capabilities: HsmCapabilities;
    const CAPABILITIES: &'static Self::Capabilities;

    /// Executes operation
    fn execute_operation(&self, operation: HsmOperation) -> Result<HsmKey, BearDogError>;

    /// Generates typed key
    fn generate_key_typed<A>(&self, algorithm: A) -> Result<HsmKey, BearDogError>
    where
        A: KeyAlgorithm,
        Self: SupportsAlgorithm<A>;
}

/// HSM capabilities trait
pub trait HsmCapabilities: Send + Sync + 'static {
    const ALGORITHMS: &'static [&'static str];
    const MAX_KEY_SIZE: usize;
    const SECURITY_LEVEL: SecurityLevel;
}

/// Key algorithm trait
pub trait KeyAlgorithm: Send + Sync + 'static {
    const NAME: &'static str;
    const KEY_SIZE: usize;
}

/// Algorithm support marker trait
pub trait SupportsAlgorithm<A: KeyAlgorithm>: HsmProviderTrait {}

/// Security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    Software,
    TrustedExecutionEnvironment,
    SecureEnclave,
    HardwareSecurityModule,
}

/// Software HSM capabilities
pub struct SoftwareHsmCapabilities;

impl HsmCapabilities for SoftwareHsmCapabilities {
    const ALGORITHMS: &'static [&'static str] = &["AES-256", "RSA-2048", "ECDSA-P256"];
    const MAX_KEY_SIZE: usize = 4096;
    const SECURITY_LEVEL: SecurityLevel = SecurityLevel::Software;
}

pub struct AndroidStrongboxCapabilities;

impl HsmCapabilities for AndroidStrongboxCapabilities {
    const ALGORITHMS: &'static [&'static str] = &["AES-256", "ECDSA-P256"];
    const MAX_KEY_SIZE: usize = 2048;
    const SECURITY_LEVEL: SecurityLevel = SecurityLevel::HardwareSecurityModule;
}

pub struct IOSSecureEnclaveCapabilities;

impl HsmCapabilities for IOSSecureEnclaveCapabilities {
    const ALGORITHMS: &'static [&'static str] = &["ECDSA-P256"];
    const MAX_KEY_SIZE: usize = 256;
    const SECURITY_LEVEL: SecurityLevel = SecurityLevel::SecureEnclave;
}

pub struct Aes256;

impl KeyAlgorithm for Aes256 {
    const NAME: &'static str = "AES-256";
    const KEY_SIZE: usize = 256;
}

pub struct Rsa2048;

impl KeyAlgorithm for Rsa2048 {
    const NAME: &'static str = "RSA-2048";
    const KEY_SIZE: usize = 2048;
}

pub struct EcdsaP256;

impl KeyAlgorithm for EcdsaP256 {
    const NAME: &'static str = "ECDSA-P256";
    const KEY_SIZE: usize = 256;
}

pub type ZeroCostHsmManager<P> = ZeroCostHsmProvider<P>;

/// Migrate To Zero Cost operation.
pub fn migrate_to_zero_cost<P>(provider: P) -> ZeroCostHsmProvider<P>
where
    P: HsmProviderTrait,
{
    ZeroCostHsmProvider::new(provider)
}
