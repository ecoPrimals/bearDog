// SPDX-License-Identifier: AGPL-3.0-only

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
    pub const fn new(provider: P) -> Self {
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
///
/// # Migration (v0.10.0)
///
/// Superseded by [`beardog_traits::hsm::HsmKeyProvider`] which supports
/// both compile-time and dynamic dispatch via `Arc<dyn HsmKeyProvider>`.
/// This trait will be removed in a future release.
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
///
/// # Migration (v0.10.0)
///
/// Superseded by [`beardog_types::hsm::HsmCapabilitySet`].
/// This trait will be removed in a future release.
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
pub const fn migrate_to_zero_cost<P>(provider: P) -> ZeroCostHsmProvider<P>
where
    P: HsmProviderTrait,
{
    ZeroCostHsmProvider::new(provider)
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::HsmKey;

    struct TestSoftwareProvider;

    impl HsmProviderTrait for TestSoftwareProvider {
        type Capabilities = SoftwareHsmCapabilities;
        const CAPABILITIES: &'static Self::Capabilities = &SoftwareHsmCapabilities;

        fn execute_operation(&self, operation: HsmOperation) -> Result<HsmKey, BearDogError> {
            match operation {
                HsmOperation::KeyDeletion { .. } => Err(BearDogError::not_found(
                    "key already removed in test provider".to_string(),
                )),
                _ => Ok(HsmKey::default()),
            }
        }

        fn generate_key_typed<A>(&self, _algorithm: A) -> Result<HsmKey, BearDogError>
        where
            A: KeyAlgorithm,
            Self: SupportsAlgorithm<A>,
        {
            Ok(HsmKey::default())
        }
    }

    impl SupportsAlgorithm<Aes256> for TestSoftwareProvider {}

    #[test]
    fn zero_cost_provider_new_and_capabilities_pointer() {
        let p = ZeroCostHsmProvider::new(TestSoftwareProvider);
        let _caps = ZeroCostHsmProvider::<TestSoftwareProvider>::capabilities();
        assert!(<SoftwareHsmCapabilities as HsmCapabilities>::ALGORITHMS.contains(&"AES-256"));
        assert_eq!(
            <SoftwareHsmCapabilities as HsmCapabilities>::SECURITY_LEVEL,
            SecurityLevel::Software
        );
        let _ = format!("{:?}", SecurityLevel::TrustedExecutionEnvironment);
        let _ = format!("{:?}", SecurityLevel::SecureEnclave);
        let _ = format!("{:?}", SecurityLevel::HardwareSecurityModule);
        let _ = p;
    }

    #[test]
    fn execute_operation_happy_path_and_key_deletion_error() {
        let p = ZeroCostHsmProvider::new(TestSoftwareProvider);
        let key = p
            .execute_operation(HsmOperation::KeyGeneration {
                key_id: "k".to_string(),
                key_size: 256,
            })
            .expect("key generation op");
        assert!(!key.key_id.is_empty());

        let err = p
            .execute_operation(HsmOperation::KeyDeletion {
                key_id: "missing".to_string(),
            })
            .expect_err("test provider rejects deletion");
        assert!(
            format!("{err}").contains("removed") || format!("{err}").contains("not found"),
            "{err}"
        );
    }

    #[test]
    fn generate_key_typed_aes256() {
        let p = ZeroCostHsmProvider::new(TestSoftwareProvider);
        let k = p.generate_key(Aes256).expect("typed generate");
        assert_eq!(k.algorithm, "AES");
    }

    #[test]
    fn migrate_to_zero_cost_is_identity_shape() {
        let z = migrate_to_zero_cost(TestSoftwareProvider);
        z.execute_operation(HsmOperation::Signing {
            key_id: "s".to_string(),
            algorithm: "ed25519".to_string(),
        })
        .expect("signing operation through migrated wrapper");
    }

    #[test]
    fn key_algorithm_constants() {
        assert_eq!(Aes256::NAME, "AES-256");
        assert_eq!(Rsa2048::KEY_SIZE, 2048);
        assert_eq!(EcdsaP256::NAME, "ECDSA-P256");
    }

    #[test]
    fn android_and_ios_capability_statics() {
        assert_eq!(
            <AndroidStrongboxCapabilities as HsmCapabilities>::SECURITY_LEVEL,
            SecurityLevel::HardwareSecurityModule
        );
        assert_eq!(
            <IOSSecureEnclaveCapabilities as HsmCapabilities>::MAX_KEY_SIZE,
            256
        );
    }
}
