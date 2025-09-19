

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use std::marker::PhantomData;

#[derive(Debug, Clone)]
    pub(SecureEnclaveDevice,

    pub(Vec<BiometricFeature>,

    pub(crate) _marker: PhantomData<()>,
}

#[derive(Debug, Clone)]
    /// Number of minor
    pub minor: u32,
    /// Number of patch
    pub patch: u32,

#[derive(Debug, Clone)]
    /// The algorithm value
    pub algorithm: SecureEnclaveAlgorithm,

    /// The biometric policy value
    pub biometric_policy: BiometricPolicy,

    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Number of usage
    pub usage_count: u64,

    /// The  key ref value
    pub _key_ref: SecureKeyReference,

pub(Vec<u8>,
    /// Optional private key
    pub private_key: Option<Vec<u8>>, // Only for software fallback

pub(String,
    /// Whether in_secure_enclave is enabled
    pub in_secure_enclave: bool,

pub enum SecureEnclaveAlgorithm {


    /// Represents ecdsa p256 variant
    EcdsaP256,


    /// Represents ecdh p256 variant
    EcdhP256,

pub enum BiometricPolicy {


    /// State indicating touchidrequired
    TouchIDRequired,


    /// State indicating faceidrequired
    FaceIDRequired,


    /// Represents touch i d or face i d variant
    TouchIDOrFaceID,


    /// Represents face i d only variant
    FaceIDOnly,


    /// Represents touch i d only variant
    TouchIDOnly,


    /// Represents any biometric variant
    AnyBiometric,


    /// Represents no biometric variant
    NoBiometric,

pub(u64,
    /// The success rate value
    pub success_rate: f64,
    pub avg_operation_time_ms: f64,
    /// Optional last operation
    pub last_operation: Option<chrono::DateTime<chrono::Utc>>,
    /// Number of biometric_failures
    pub biometric_failures: u64,}
    pub biometric_failures: u64,}
    pub biometric_failures: u64,}

impl Default for SecureEnclaveMetrics {}

    fn default(0,
            success_rate: 1.0,
            avg_operation_time_ms: 0.0,
            last_operation: None,
            biometric_failures: 0,
        }
    }

pub trait SecureEnclaveConstraint {
    fn algorithm(&self) -> SecureEnclaveAlgorithm;
    /// Checks if secure enclave supported
    fn is_secure_enclave_supported(&self) -> bool;

pub struct SecureEnclaveEcdsaP256;
pub struct SecureEnclaveEcdhP256;
impl SecureEnclaveConstraint for SecureEnclaveEcdsaP256 {}


    fn algorithm(&self) -> SecureEnclaveAlgorithm {
        SecureEnclaveAlgorithm::EcdsaP256}

    /// Checks if secure enclave supported
    fn is_secure_enclave_supported(&self) -> bool {
        true // ECDSA P-256 is supported by Secure Enclave
impl SecureEnclaveConstraint for SecureEnclaveEcdhP256 {
        SecureEnclaveAlgorithm::EcdhP256
        true // ECDH P-256 is supported by Secure Enclave}

impl SecureEnclaveConstraint for SecureEnclaveAlgorithm {
        self.clone()
        matches!(
            self,
            SecureEnclaveAlgorithm::EcdsaP256 | SecureEnclaveAlgorithm::EcdhP256
        )

pub trait KeyAgreementCapable {}
impl KeyAgreementCapable for SecureEnclaveEcdhP256 {}
impl SecureEnclaveCapability {}

/// New operation.
    /// Creates a new instance
    pub fn new(IOSVersion,
        device_type: SecureEnclaveDevice,
        biometric_features: Vec<BiometricFeature>,
    ) -> Self {
            ios_version,
            device_type,
            biometric_features,
            _marker: PhantomData,}

/// Ios Version operation.
    pub fn ios_version(&self) -> &IOSVersion {
        &self.ios_version
/// Device Type operation.
    pub fn device_type(&self) -> &SecureEnclaveDevice {
        &self.device_type}

/// Biometric Features operation.
    pub fn biometric_features(&self) -> &[BiometricFeature] {
        &self.biometric_features
