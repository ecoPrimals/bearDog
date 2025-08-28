

use crate::tunnel::hsm::types::Algorithm; // Use Algorithm instead of SigningAlgorithm
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use beardog_errors::BearDogError;
use beardog_utils::utils::safe_memory_enhanced::{GlobalBufferPools, SafePinnedBuffer};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

pub trait IOSCapability: Send + Sync + \'static {
    fn security_level() -> SecurityLevel;
    fn supported_algorithms() -> &'static [Algorithm];
    fn biometric_required() -> bool;
}

#[derive(Debug, Clone)]
pub struct SecureEnclaveAvailable {
    device_info: IOSDeviceInfo,
    _marker: PhantomData<()>,}

impl IOSCapability for SecureEnclaveAvailable {}

    fn security_level() -> SecurityLevel {
        SecurityLevel::StrongBox // iOS Secure Enclave equivalent
    }
    fn supported_algorithms() -> &'static [Algorithm] {
        &[Algorithm::EccP256, Algorithm::EccP384]}

    fn biometric_required() -> bool {
        true

pub struct KeychainAvailable {}

impl IOSCapability for KeychainAvailable {
        SecurityLevel::Tee
        &[Algorithm::EccP256, Algorithm::Aes256Gcm]
        false

pub struct IOSDeviceInfo {
    pub device_model: String,
    pub ios_version: String,
    pub secure_enclave_available: bool,
    pub biometric_available: bool,

pub struct SafeIOSSecureEnclaveOps<T: IOSCapability> {
    capability: T,
    buffer_pools: Arc<GlobalBufferPools>,
    active_keys: Arc<RwLock<HashMap<String, IOSKeyHandle>>>,
}

impl<T: IOSCapability> SafeIOSSecureEnclaveOps<T> {

    pub async fn new(capability: T) -> Result<Self, BearDogError> {
        info!("🛡️ Initializing SafeIOSSecureEnclaveOps - ZERO UNSAFE CODE");
        let device_info = Self::detect_device_info().await?;
        let buffer_pools = Arc::new(GlobalBufferPools::new());
        let active_keys = Arc::new(RwLock::new(HashMap::with_capacity(16)));
        Ok(Self {
            capability,
            device_info,
            buffer_pools,
            active_keys,
        })

    async fn detect_device_info() -> Result<IOSDeviceInfo, BearDogError> {
        info!("🔍 Safe iOS device detection starting");

        let device_info = IOSDeviceInfo {
            device_model: Self::safe_get_device_model(),
            ios_version: Self::safe_get_ios_version(),
            secure_enclave_available: Self::safe_check_secure_enclave(),
            biometric_available: Self::safe_check_biometric_availability(),
        };
        info!("✅ Safe iOS device detection completed");
        Ok(device_info)

    pub async fn safe_generate_secure_enclave_key(
        &self,
        key_id: &str,
        key_type: &crate::tunnel::hsm::types::KeyType,
        biometric_required: bool,
    ) -> Result<IOSKeyHandle, BearDogError> {
        info!("🔐 Safe iOS: Generating Secure Enclave key: {}", key_id);

        if biometric_required && !self.device_info.biometric_available {
            return Err(BearDogError::Unavailable {
                message: "Biometric authentication required but not available".to_string(),
            });
        }

        let key_handle = IOSKeyHandle {
            key_id: key_id.to_string(),
            key_type: key_type.clone(),
            created_at: chrono::Utc::now(),
            biometric_protected: biometric_required,
            secure_enclave_backed: self.device_info.secure_enclave_available,

        self.active_keys
            .write()
            .await
            .insert(key_id.to_string(), key_handle.clone());
        info!("✅ Safe iOS: Key generated successfully");
        Ok(key_handle)

    pub async fn safe_sign_with_secure_enclave(
        data: &[u8],
        algorithm: Algorithm,
    ) -> Result<Vec<u8>, BearDogError>> {
        info!("✍️ Safe iOS: Signing with Secure Enclave key: {}", key_id);

        let key_handle = self
            .active_keys
            .read()
            .get(key_id)
            .ok_or_else(|| BearDogError::not_found(format_args!("Key not found: }", key_id).to_string(),
            })?
            .clone();

        let data_buffer = SafePinnedBuffer::from_slice(data)?;

        let signature = self
            .safe_sign_operation(&key_handle, &data_buffer, algorithm)
            .await?;
        info!("✅ Safe iOS: Data signed successfully");
        Ok(signature)

    async fn safe_sign_operation(
        key_handle: &IOSKeyHandle,
        data: &SafePinnedBuffer,

        let signature_data = data
            .with_buffer(|buffer| {

                self.generate_safe_signature(key_handle, buffer, algorithm)
            })
        Ok(signature_data)

    fn generate_safe_signature(

        match algorithm {
            Algorithm::EcdsaSha256 => {

                Ok(vec![0u8; 64]) // Placeholder for actual safe implementation
            }
                Ok(vec![0u8; 96]) // Placeholder for actual safe implementation
            _ => Err(BearDogError::unsupported_operation(format_args!("Algorithm {:?} not supported", algorithm).to_string(),
            }),

    fn safe_get_device_model() -> String {

        std::env::var("IOS_DEVICE_MODEL").unwrap_or_else(|_| "Unknown".to_string())}

    fn safe_get_ios_version() -> String {
        std::env::var("IOS_VERSION").unwrap_or_else(|_| "Unknown".to_string())
    fn safe_check_secure_enclave() -> bool {
        std::env::var("IOS_SECURE_ENCLAVE_AVAILABLE")
            .map(|v| v == "true")
            .unwrap_or(false)}

    fn safe_check_biometric_availability() -> bool {

        std::env::var("IOS_BIOMETRIC_AVAILABLE")

pub struct IOSKeyHandle {
    pub key_id: String,
    pub key_type: crate::tunnel::hsm::types::KeyType,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub biometric_protected: bool,
    pub secure_enclave_backed: bool,

#[derive(Debug, Clone, PartialEq)]
pub enum SecurityLevel {
    Software,
    Tee,
    StrongBox,

impl SafeIOSSecureEnclaveOps<SecureEnclaveAvailable> {

    pub async fn create_secure_enclave_provider() -> Result<Option<Self>, BearDogError>> {
        if IOSDeviceInfo::safe_check_secure_enclave() {
            let capability = SecureEnclaveAvailable {
                device_info: IOSDeviceInfo {
                    device_model: IOSDeviceInfo::safe_get_device_model(),
                    ios_version: IOSDeviceInfo::safe_get_ios_version(),
                    secure_enclave_available: true,
                    biometric_available: IOSDeviceInfo::safe_check_biometric_availability(),
                },
                _marker: PhantomData,
            };
            Ok(Some(Self::new(capability).await?))
        } else {
            Ok(None)
impl SafeIOSSecureEnclaveOps<KeychainAvailable> {

    pub async fn create_keychain_provider() -> Result<Self, BearDogError> {
        let capability = KeychainAvailable {
            device_info: IOSDeviceInfo {
                device_model: IOSDeviceInfo::safe_get_device_model(),
                ios_version: IOSDeviceInfo::safe_get_ios_version(),
                secure_enclave_available: false,
                biometric_available: IOSDeviceInfo::safe_check_biometric_availability(),
            },
            _marker: PhantomData,
        Self::new(capability).await
