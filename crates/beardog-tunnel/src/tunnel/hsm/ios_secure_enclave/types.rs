

use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct SecureEnclaveCapability {

    pub(crate) ios_version: IOSVersion,

    pub(crate) device_type: SecureEnclaveDevice,

    pub(crate) biometric_features: Vec<BiometricFeature>,

    pub(crate) _marker: PhantomData<()>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IOSVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecureEnclaveDevice {

    IPhone,

    IPad,

    Mac,

    AppleWatch,

pub enum BiometricFeature {
    TouchID,
    FaceID,
    OpticID,

pub(crate) struct SecureEnclaveKey {

    pub keychain_id: String,

    pub algorithm: SecureEnclaveAlgorithm,

    pub biometric_policy: BiometricPolicy,

    pub created_at: chrono::DateTime<chrono::Utc>,

    pub usage_count: u64,

    pub _key_ref: SecureKeyReference,

pub(crate) struct SecureKeychainKey {
    pub public_key: Vec<u8>,
    pub private_key: Option<Vec<u8>>, // Only for software fallback

pub(crate) struct SecureKeyReference {
    pub keychain_ref: String,
    pub in_secure_enclave: bool,

pub enum SecureEnclaveAlgorithm {

    EcdsaP256,

    EcdhP256,

pub enum BiometricPolicy {

    TouchIDRequired,

    FaceIDRequired,

    TouchIDOrFaceID,

    FaceIDOnly,

    TouchIDOnly,

    AnyBiometric,

    NoBiometric,

pub(crate) struct SecureEnclaveMetrics {
    pub secure_operations: u64,
    pub success_rate: f64,
    pub avg_operation_time_ms: f64,
    pub last_operation: Option<chrono::DateTime<chrono::Utc>>,
    pub biometric_failures: u64,}

impl Default for SecureEnclaveMetrics {}

    fn default() -> Self {
        Self {
            secure_operations: 0,
            success_rate: 1.0,
            avg_operation_time_ms: 0.0,
            last_operation: None,
            biometric_failures: 0,
        }
    }

pub trait SecureEnclaveConstraint {
    fn algorithm(&self) -> SecureEnclaveAlgorithm;
    fn is_secure_enclave_supported(&self) -> bool;

pub struct SecureEnclaveEcdsaP256;
pub struct SecureEnclaveEcdhP256;
impl SecureEnclaveConstraint for SecureEnclaveEcdsaP256 {}

    fn algorithm(&self) -> SecureEnclaveAlgorithm {
        SecureEnclaveAlgorithm::EcdsaP256}

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

    pub fn new(
        ios_version: IOSVersion,
        device_type: SecureEnclaveDevice,
        biometric_features: Vec<BiometricFeature>,
    ) -> Self {
            ios_version,
            device_type,
            biometric_features,
            _marker: PhantomData,}

    pub fn ios_version(&self) -> &IOSVersion {
        &self.ios_version
    pub fn device_type(&self) -> &SecureEnclaveDevice {
        &self.device_type}

    pub fn biometric_features(&self) -> &[BiometricFeature] {
        &self.biometric_features
