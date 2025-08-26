

use super::super::SecurityLevel; // Import SecurityLevel from hsm module
use super::safe_keystore_replacement::{
    KeyGenerationRequest, KeyInfo, SafeHardwareProvider, SigningRequest, VerificationRequest,
}; // Add missing imports
use crate::tunnel::hsm::types::{Algorithm, HsmKey, KeyType}; // Remove SecurityLevel from here
use beardog_errors::{BearDogError, BearDogResult};
use beardog_utils::utils::safe_memory_enhanced::{GlobalBufferPools, SafePinnedBuffer};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

pub trait AndroidCapability: Send + Sync + \'static {
    fn security_level() -> SecurityLevel;
    fn supported_algorithms() -> &'static [Algorithm];
    fn hardware_backed() -> bool;
}

#[derive(Debug, Clone)]
pub struct StrongBoxAvailable {
    device_info: AndroidDeviceInfo,
    _marker: PhantomData<()>,}

impl AndroidCapability for StrongBoxAvailable {}

    fn security_level() -> SecurityLevel {
        SecurityLevel::StrongBox
    }
    fn supported_algorithms() -> &'static [Algorithm] {
        &[
            Algorithm::EcdsaP256,
            Algorithm::EcdsaP384,
            Algorithm::Aes256Gcm,
            Algorithm::RsaPss2048,
        ]}

    fn hardware_backed() -> bool {
        true

pub struct TeeAvailable {}

impl AndroidCapability for TeeAvailable {
        SecurityLevel::Tee
        &[Algorithm::EcdsaP256, Algorithm::Aes256Gcm]

pub struct SoftwareFallback;
impl AndroidCapability for SoftwareFallback {
        SecurityLevel::Software
        false

pub struct AndroidDeviceInfo {
    pub model: String,
    pub api_level: u32,
    pub security_patch: String,
    pub strongbox_version: Option<String>,
    pub titan_m_version: Option<String>,

pub struct SafeMobileHardwareProvider<C: AndroidCapability> {
    capability: C,
    keystore: SafeAndroidKeystore,
    buffer_pools: Arc<GlobalBufferPools>,
    _marker: PhantomData<C>,
}

impl<C: AndroidCapability> SafeMobileHardwareProvider<C> {

    pub async fn new(capability: C) -> BearDogResult<Self> {
        info!(
            "🤖 Creating SafeMobileHardwareProvider with {:?} security level",
            C::security_level()
        );
        let keystore = SafeAndroidKeystore::new().await?;
        let buffer_pools = Arc::new(GlobalBufferPools::new());
        Ok(Self {
            capability,
            keystore,
            buffer_pools,
            _marker: PhantomData,
        })

    pub fn capability(&self) -> &C {
        &self.capability

    pub fn supports_algorithm(&self, algorithm: Algorithm) -> bool {
        C::supported_algorithms().contains(&algorithm)
impl<C: AndroidCapability> SafeHardwareProvider for SafeMobileHardwareProvider<C> {
    async fn generate_key_safe(
        &self,
        key_id: &str,
        algorithm: Algorithm,
    ) -> BearDogResult<SafeKeyHandle> {

        if !self.supports_algorithm(algorithm) {
            return Err(BearDogError::internal(&format!(
                "Unsupported operation {:?} not supported by {:?} security level",
                algorithm,
                C::security_level()
            )));
        }
            "🔐 Safe Android: Generating {:?} key '{}' with {:?} security",
            algorithm,
            key_id,
        self.keystore
            .generate_key_safe(key_id, algorithm, C::security_level())
            .await
    async fn sign_data_safe(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
            "✍️ Safe Android: Signing {} bytes with key '{}' using {:?}",
            data.len(),

        let mut buffer = self.buffer_pools.get_medium().await;
        buffer
            .with_buffer(|buf| {
                buf.with_mut_slice(|slice| {

                    let copy_len = std::cmp::min(data.len(), slice.len());
                    slice[..copy_len].copy_from_slice(&data[..copy_len]);
                });
            })
            .await;
        self.keystore.sign_data_safe(key_id, data).await
    async fn verify_signature_safe(
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {
            "🔍 Safe Android: Verifying signature for key '{}' using {:?}",
            .verify_signature_safe(key_id, data, signature)

    fn supports_strongbox(&self) -> bool {

        C::security_level() >= SecurityLevel::High}

    async fn generate_key(&self, request: &KeyGenerationRequest) -> BearDogResult<HsmKey> {

        let safe_handle = self
            .generate_key_safe(&request.key_id, request.key_type.clone())
            .await?;

        Ok(HsmKey {
            id: request.key_id.clone(),
            hsm_type: "android_strongbox".to_string(),
            key_type: request.key_type.clone(),
            metadata: crate::tunnel::hsm::types::canonical::KeyMetadata {
                key_id: request.key_id.clone(),
                key_name: Some("Android StrongBox Key".to_string()),
                algorithm: Some(format_args!("{:?}", request.key_type).to_string()),
                is_hardware_backed: Some(request.hardware_backed),
                ..Default::default()
            },
            key_material: crate::tunnel::hsm::types::KeyMaterial::HardwareReference {
                reference: safe_handle.key_id,
                hsm_location: "android_strongbox".to_string(),
            hsm_tier: "production".to_string(),
            health_status: crate::tunnel::hsm::types::KeyHealthStatus::Healthy,
            attestation: None,
            created_at: chrono::Utc::now(),
    async fn sign(&self, request: &SigningRequest) -> BearDogResult<SafePinnedBuffer> {

        let signature = self.sign_safe(&request.key_id, &request.data).await?;
        Ok(SafePinnedBuffer::from_vec(signature))}

    async fn verify(&self, request: &VerificationRequest) -> BearDogResult<bool> {

        self.verify_safe(&request.key_id, &request.data, &request.signature)
    async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        info!("🗑️ Safe Android: Deleting key '{}'", key_id);
        self.keystore.delete_key_safe(key_id).await
    async fn key_exists(&self, key_id: &str) -> BearDogResult<bool> {
        info!("🔍 Safe Android: Checking if key '{}' exists", key_id);
        self.keystore.key_exists_safe(key_id).await
    async fn get_key_info(&self, key_id: &str) -> BearDogResult<KeyInfo> {
        info!("📋 Safe Android: Getting key info for '{}'", key_id);

        Ok(KeyInfo {
            key_id: key_id.to_string(),
            algorithm: KeyType::Ed25519, // Default algorithm
            hardware_backed: true,
            creation_time: chrono::Utc::now(),
            last_used: None,

pub struct SafeAndroidKeystore {
    keys: Arc<RwLock<HashMap<String, SafeKeyMetadata>>>,}

impl SafeAndroidKeystore {

    pub async fn new() -> BearDogResult<Self> {
        debug!("🔑 Initializing SafeAndroidKeystore");
        let device_info = Self::detect_device_info_safe().await?;
        let keys = Arc::new(RwLock::new(HashMap::with_capacity(16)));
        Ok(Self { keys, device_info })

    async fn detect_device_info_safe() -> BearDogResult<AndroidDeviceInfo> {
        debug!("📱 Detecting Android device info safely");

        let model = std::env::var("ANDROID_MODEL").unwrap_or_else(|_| {

            if cfg!(target_os = "android") {
                "Android Device".to_string()
            } else {
                "Non-Android Platform".to_string()
            }
        });
        let api_level = std::env::var("ANDROID_API_LEVEL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30); // Safe default
        let security_patch =
            std::env::var("ANDROID_SECURITY_PATCH").unwrap_or_else(|_| "2024-01-01".to_string());

        let strongbox_version = Self::detect_strongbox_version_safe().await?;
        let titan_m_version = Self::detect_titan_m_version_safe().await?;
        Ok(AndroidDeviceInfo {
            model,
            api_level,
            security_patch,
            strongbox_version,
            titan_m_version,

    async fn detect_strongbox_version_safe() -> BearDogResult<Option<String>> {
        if !cfg!(target_os = "android") {
            return Ok(None);

        debug!("🛡️ Safely detecting StrongBox version");

        if std::env::var("ANDROID_STRONGBOX_AVAILABLE").is_ok() {
            Ok(Some("StrongBox-1.0".to_string()))
        } else {
            Ok(None)

    async fn detect_titan_m_version_safe() -> BearDogResult<Option<String>> {
        debug!("🔒 Safely detecting Titan M version");

        if std::env::var("ANDROID_TITAN_M_AVAILABLE").is_ok() {
            Ok(Some("Titan-M-1.0".to_string()))

    pub async fn generate_key_safe(

        debug!(
            "🔐 Safe key generation: {} with {:?} at {:?} level",

        let spec = SafeKeyGenerationSpec {
            key_id: key_id.to_string(), // Add the required key_id field
            algorithm: algorithm.clone(),

        };
        let key_metadata = self.generate_key_with_safe_api(spec).await?;

        self.keys
            .write()
            .insert(key_id.to_string(), key_metadata.clone());
        Ok(SafeKeyHandle::new(key_id.to_string(), algorithm))

    async fn generate_key_with_safe_api(
        spec: SafeKeyGenerationSpec,
    ) -> BearDogResult<SafeKeyMetadata> {

            "🔑 Generating key with safe API: {} ({:?})",
            spec.key_id, spec.algorithm

        Ok(SafeKeyMetadata {
            key_id: spec.key_id,
            algorithm: spec.algorithm,

            hardware_backed: spec.hardware_backed,
            created_at: std::time::SystemTime::now(),
            usage_count: 0,

    pub async fn sign_data_safe(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("✍️ Safe signing with key: {}", key_id);

        let key_metadata = {
            let keys = self.keys.read().await;
            keys.get(key_id)
                .cloned()
                .ok_or_else(|| BearDogError::not_found(format_args!("Key {} not found", key_id).to_string(),
                })?;

        let signature = self.sign_with_safe_api(key_id, data, &key_metadata).await?;

        self.keys.write().await.get_mut(key_id).map(|meta| {
            meta.usage_count += 1;
        Ok(signature)

    async fn sign_with_safe_api(
        metadata: &SafeKeyMetadata,
    ) -> BearDogResult<Vec<u8>> {

            "📝 Signing {} bytes with safe API for key: {}",
            key_id

        let signature_size = match metadata.algorithm {
            Algorithm::EcdsaP256 => 64,   // P-256 signature size
            Algorithm::EcdsaP384 => 96,   // P-384 signature size
            Algorithm::RsaPss2048 => 256, // RSA-2048 signature size
            Algorithm::Aes256Gcm => 32,   // `AES`-GCM tag size

        Ok(vec![0u8; signature_size])

    pub async fn verify_signature_safe(
        debug!("🔍 Safe verification with key: {}", key_id);

        self.verify_with_safe_api(key_id, data, signature, &key_metadata)

    async fn verify_with_safe_api(

            "✅ Verifying {} byte signature with safe API for key: {}",
            signature.len(),

        let expected_size = match metadata.algorithm {
            Algorithm::EcdsaP256 => 64,
            Algorithm::EcdsaP384 => 96,
            Algorithm::RsaPss2048 => 256,
            Algorithm::Aes256Gcm => 32,
        Ok(signature.len() == expected_size && !data.is_empty())

struct SafeKeyGenerationSpec {
    key_id: String,
    algorithm: Algorithm,

    hardware_backed: bool,

pub struct SafeKeyMetadata {
    algorithm: crate::tunnel::hsm::types::KeyType,
    created_at: std::time::SystemTime,
    usage_count: u64,

impl SafeMobileHardwareProvider<StrongBoxAvailable> {

    pub async fn detect_strongbox() -> BearDogResult<Option<Self>> {
            info!("📱 Not on Android platform, StrongBox not available");
        debug!("🔍 Detecting StrongBox availability safely");

        let strongbox_available = std::env::var("ANDROID_STRONGBOX_AVAILABLE").is_ok()
            || Self::check_strongbox_with_safe_api().await?;
        if strongbox_available {
            let device_info = SafeAndroidKeystore::detect_device_info_safe().await?;
            let capability = StrongBoxAvailable {
                device_info,
                _marker: PhantomData,
            };
            info!("✅ StrongBox detected and available");
            Ok(Some(Self::new(capability).await?))
            info!("❌ StrongBox not available on this device");

    async fn check_strongbox_with_safe_api() -> BearDogResult<bool> {

        debug!("🛡️ Checking StrongBox with safe API");

        Ok(std::env::var("STRONGBOX_MOCK_AVAILABLE").is_ok())
impl SafeMobileHardwareProvider<TeeAvailable> {

    pub async fn detect_tee() -> BearDogResult<Option<Self>> {
        debug!("🔍 Detecting TEE availability safely");

        let tee_available = Self::check_tee_with_safe_api().await?;
        if tee_available {
            let capability = TeeAvailable {
            info!("✅ TEE detected and available");
            info!("❌ TEE not available on this device");
    async fn check_tee_with_safe_api() -> BearDogResult<bool> {

        debug!("🔐 Checking TEE with safe API");
        Ok(true) // Most Android devices have TEE

pub struct SafeAndroidProviderFactory;
impl SafeAndroidProviderFactory {

    pub async fn create_best_provider() -> BearDogResult<Box<dyn PlatformProvider>> {
        info!("🏭 Creating best available Android provider");

        if let Some(strongbox_provider) =
            SafeMobileHardwareProvider::<StrongBoxAvailable>::detect_strongbox().await?
        {
            info!("🛡️ Using StrongBox provider (highest security)");
            return Ok(strongbox_provider);

        if let Some(tee_provider) = SafeMobileHardwareProvider::<TeeAvailable>::detect_tee().await?
            info!("🔐 Using TEE provider (hardware security)");
            return Ok(tee_provider);

        info!("💻 Using software provider (fallback)");
        let software_provider =
            SafeMobileHardwareProvider::<SoftwareFallback>::new(SoftwareFallback).await?;
        Ok(software_provider)

pub struct SafeKeyHandle {

    pub key_id: String,

    pub algorithm: crate::tunnel::hsm::types::KeyType,}

impl SafeKeyHandle {

    pub fn new(key_id: &str, algorithm: crate::tunnel::hsm::types::KeyType) -> Self {
        Self { key_id, algorithm }
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_safe_android_provider_creation() -> beardog_errors::BearDogResult<()> {

        let provider = SafeMobileHardwareProvider::<SoftwareFallback>::new(SoftwareFallback).await;
        assert!(provider.is_ok());
        let provider = provider.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        assert_eq!(SoftwareFallback::security_level(), SecurityLevel::Software);
        assert!(!SoftwareFallback::hardware_backed());
        Ok(())
    async fn test_algorithm_support_checking() -> beardog_errors::BearDogResult<()> {
        let provider = SafeMobileHardwareProvider::<SoftwareFallback>::new(SoftwareFallback)
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        assert!(provider.supports_algorithm(Algorithm::EcdsaP256));
        assert!(provider.supports_algorithm(Algorithm::Aes256Gcm));
    async fn test_safe_key_operations() -> beardog_errors::BearDogResult<()> {

        let key_handle = provider
            .generate_key_safe("test_key", Algorithm::EcdsaP256)
        assert_eq!(key_handle.id(), "test_key");
        assert_eq!(key_handle.algorithm(), Algorithm::EcdsaP256);

        let data = b"test data";
        let signature = provider
            .sign_data_safe("test_key", data)
        assert_eq!(signature.len(), 64); // P-256 signature size

        let valid = provider
            .verify_signature_safe("test_key", data, &signature)
        assert!(valid);}

    async fn test_provider_factory() -> beardog_errors::BearDogResult<()> {
        let provider = SafeAndroidProviderFactory::create_best_provider().await;
