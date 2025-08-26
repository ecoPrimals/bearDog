

use beardog_errors::BearDogResult;
use beardog_types::canonical::{HsmKey, HsmOperation};
use std::marker::PhantomData;

pub struct ZeroCostHsmProvider<P> {
    provider: P,
    _phantom: PhantomData<P>,
}

impl<P> ZeroCostHsmProvider<P>
where
    P: HsmProviderTrait,
{

    pub fn new(provider: P) -> Self {
        Self {
            provider,
            _phantom: PhantomData,
        }
    }

    pub const fn capabilities() -> &'static P::Capabilities {

        P::CAPABILITIES
    }

    pub async fn execute_operation(&self, operation: HsmOperation) -> BearDogResult<HsmKey> {

        self.provider.execute_operation(operation).await
    }

    pub async fn generate_key<A>(&self, algorithm: A) -> BearDogResult<HsmKey> 
    where
        A: KeyAlgorithm,
        P: SupportsAlgorithm<A>,
    {

        self.provider.generate_key_typed(algorithm).await
    }
}

pub trait HsmProviderTrait: Send + Sync + \'static {

    type Capabilities: HsmCapabilities;
    const CAPABILITIES: &'static Self::Capabilities;

    async fn execute_operation(&self, operation: HsmOperation) -> BearDogResult<HsmKey>;

    async fn generate_key_typed<A>(&self, algorithm: A) -> BearDogResult<HsmKey>
    where
        A: KeyAlgorithm,
        Self: SupportsAlgorithm<A>;
}

pub trait HsmCapabilities: Send + Sync + \'static {

    const ALGORITHMS: &'static [&'static str];

    const MAX_KEY_SIZE: usize;

    const SECURITY_LEVEL: SecurityLevel;
}

pub trait KeyAlgorithm: Send + Sync + \'static {
    const NAME: &'static str;
    const KEY_SIZE: usize;
}

pub trait SupportsAlgorithm<A: KeyAlgorithm>: HsmProviderTrait {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    Software,
    Hardware,
    SecureEnclave,
    HardwareSecurityModule,
}

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

pub fn migrate_to_zero_cost<P>(provider: P) -> ZeroCostHsmProvider<P>
where
    P: HsmProviderTrait,
{
    ZeroCostHsmProvider::new(provider)
}
