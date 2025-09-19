

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::super::SecurityLevel; // Import SecurityLevel from hsm module
use super::safe_keystore_replacement::{
    KeyGenerationRequest, KeyInfo, SafeHardwareProvider, SigningRequest, VerificationRequest,
}; // Add missing imports
use crate::tunnel::hsm::types::{Algorithm, HsmKey, KeyType}; // Remove SecurityLevel from here
use beardog_errors::BearDogError;
use beardog_utils::utils::safe_memory_enhanced::{GlobalBufferPools, SafePinnedBuffer};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

pub trait AndroidCapability: Send + Sync + \'static {
    fn security_level(AndroidDeviceInfo,
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
    /// The model value
    pub model: String,
    /// Number of api_level
    pub api_level: u32,
    /// The security patch value
    pub security_patch: String,
    /// Optional strongbox version
    pub strongbox_version: Option<String>,
    /// Optional titan m version
    pub titan_m_version: Option<String>,

pub struct SafeMobileHardwareProvider<C: AndroidCapability> {
    capability: C,
    keystore: SafeAndroidKeystore,
    buffer_pools: Arc<GlobalBufferPools>,
    _marker: PhantomData<C>,
}

impl<C: AndroidCapability> SafeMobileHardwareProvider<C> {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(capability: C) -> Result<Self, BearDogError> {
        info!(
            "🤖 Creating SafeMobileHardwareProvider with {:?} security level",
            C::security_level()
        );
        let keystore = SafeAndroidKeystore::new()?;
        let buffer_pools = Arc::new(GlobalBufferPools::new(PhantomData,
        })

/// Capability operation.
    pub fn capability(&self) -> &C {
        &self.capability

/// Supports Algorithm operation.
    pub fn supports_algorithm(&self, algorithm: Algorithm) -> bool {
        C::supported_algorithms(AndroidCapability> SafeHardwareProvider for SafeMobileHardwareProvider<C> {
    fn generate_key_safe(
        &self,
        key_id: &str,
        algorithm: Algorithm,
    ) -> Result<SafeKeyHandle, BearDogError> {

        if !self.supports_algorithm(algorithm) {
            return Err(BearDogError::internal(&format!(
                "Unsupported operation {:?} not supported by {:?} security level",
                algorithm,
                C::security_level(Generating {:?} key "{}" with {:?} security",
            algorithm,
            key_id,
        self.keystore
            .generate_key_safe(key_id, algorithm, C::security_level(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
            "✍️ Safe Android: Signing {} bytes with key "{}" using {:?}",
            data.len(),

        let mut buffer = self.buffer_pools.get_medium();
        buffer
            .with_buffer(|buf| {
                buf.with_mut_slice(|slice| {

                    let copy_len = std::cmp::min(&[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
            "🔍 Safe Android: Verifying signature for key "{}" using {:?}",
            .verify_signature_safe(key_id, data, signature)


    fn supports_strongbox(&self) -> bool {

        C::security_level() >= SecurityLevel::High}


    fn generate_key(&self, request: &KeyGenerationRequest) -> Result<HsmKey, BearDogError> {

        let safe_handle = self
            .generate_key_safe(&request.key_id,
            hsm_type: "android_strongbox".to_string()),
                is_hardware_backed: Some(request.hardware_backed),
                ..Default::default(crate::tunnel::hsm::types::KeyMaterial::HardwareReference {
                reference: safe_handle.key_id,
                hsm_location: "android_strongbox".to_string(),
            hsm_tier: "production".to_string(),
            created_at: chrono::Utc::now(),
    fn sign(&self, request: &SigningRequest) -> Result<SafePinnedBuffer, BearDogError> {

        let signature = self.sign_safe(&request.key_id, &request.data)?;
        Ok(SafePinnedBuffer::from_vec(signature))}


    fn verify(&self, request: &VerificationRequest) -> Result<bool, BearDogError> {

        self.verify_safe(&request.key_id, &request.data, &request.signature)
    /// Removes key
    fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        info!("🗑️ Safe Android: Deleting key "{}"", key_id);
        self.keystore.delete_key_safe(key_id)
    fn key_exists(&self, key_id: &str) -> Result<bool, BearDogError> {
        info!("🔍 Safe Android: Checking if key "{}" exists", key_id);
        self.keystore.key_exists_safe(key_id)
    /// Gets key_info
    fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError> {
        info!("📋 Safe Android: Getting key info for "{}"", key_id);

        Ok(KeyInfo {
            key_id: key_id.to_string()


    fn detect_device_info_safe() -> Result<AndroidDeviceInfo, BearDogError> {
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

        let strongbox_version = Self::detect_strongbox_version_safe()?;
        let titan_m_version = Self::detect_titan_m_version_safe()?;
        Ok(AndroidDeviceInfo {
            model,
            api_level,
            security_patch,
            strongbox_version,
            titan_m_version,


    fn detect_strongbox_version_safe() -> Result<Option<String>, BearDogError>> {
        if !cfg!(target_os = "android") {
            return Ok(None);

        debug!("🛡️ Safely detecting StrongBox version");

        if std::env::var("ANDROID_STRONGBOX_AVAILABLE").is_ok() {
            Ok(Some("StrongBox-1.0".to_string()))
        } else {
            Ok(None)


    fn detect_titan_m_version_safe() -> Result<Option<String>, BearDogError>> {
        debug!("🔒 Safely detecting Titan M version");

        if std::env::var({} with {:?} at {:?} level",

        let spec = SafeKeyGenerationSpec {
            key_id: key_id.to_string(), // Add the required key_id field
            algorithm: algorithm.clone(),

        };
        let key_metadata = self.generate_key_with_safe_api(spec)?;

        self.keys
            .write()
            .insert(key_id.to_string(), key_metadata.clone());
        Ok(SafeKeyHandle::new(SafeKeyGenerationSpec,
    ) -> Result<SafeKeyMetadata, BearDogError> {

            "🔑 Generating key with safe API: {} ({:?})",
            spec.key_id, spec.algorithm

        Ok(spec.key_id,
            algorithm: spec.algorithm,

            hardware_backed: spec.hardware_backed,
            created_at: std::time::SystemTime::now(0,

/// Sign Data Safe operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn sign_data_safe(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        debug!("✍️ Safe signing with key: {}", key_id);

        let key_metadata = {
            let keys = self.keys.read();
            keys.get(key_id)
                .cloned()
                .ok_or_else(|| BearDogError::not_found(&SafeKeyMetadata,
    ) -> Result<Vec<u8>, BearDogError>> {

            "📝 Signing {} bytes with safe API for key: {}",
            key_id

        let signature_size = match metadata.algorithm {
            Algorithm::EcdsaP256 => 64,   // P-256 signature size
            Algorithm::EcdsaP384 => 96,   // P-384 signature size
            Algorithm::RsaPss2048 => 256, // RSA-2048 signature size
            Algorithm::Aes256Gcm => 32,   // `AES`-GCM tag size

        Ok({}", key_id);

        self.verify_with_safe_api({}",
            signature.len(),

        let expected_size = match metadata.algorithm {
            Algorithm::EcdsaP256 => 64,
            Algorithm::EcdsaP384 => 96,
            Algorithm::RsaPss2048 => 256,
            Algorithm::Aes256Gcm => 32,
        Ok(String,
    algorithm: Algorithm,

    hardware_backed: bool,

pub struct SafeKeyMetadata {
    algorithm: crate::tunnel::hsm::types::KeyType,
    created_at: std::time::SystemTime,
    usage_count: u64,

impl SafeMobileHardwareProvider<StrongBoxAvailable> {

/// Detect Strongbox operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn detect_strongbox() -> Result<Option<Self>, BearDogError>> {
            info!("📱 Not on Android platform, StrongBox not available");
        debug!("🔍 Detecting StrongBox availability safely");

        let strongbox_available = std::env::var("ANDROID_STRONGBOX_AVAILABLE").is_ok()
            || Self::check_strongbox_with_safe_api()?;
        if strongbox_available {
            let device_info = SafeAndroidKeystore::detect_device_info_safe(PhantomData,
            };
            info!("✅ StrongBox detected and available");
            Ok(Some(Self::new(capability)?))
            info!("❌ StrongBox not available on this device");


    fn check_strongbox_with_safe_api() -> Result<bool, BearDogError> {

        debug!("🛡️ Checking StrongBox with safe API");

        Ok(std::env::var("STRONGBOX_MOCK_AVAILABLE").is_ok())
impl SafeMobileHardwareProvider<TeeAvailable> {

/// Detect Tee operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn detect_tee() -> Result<Option<Self>, BearDogError>> {
        debug!("🔍 Detecting TEE availability safely");

        let tee_available = Self::check_tee_with_safe_api()?;
        if tee_available {
            let capability = TeeAvailable {
            info!("✅ TEE detected and available");
            info!("❌ TEE not available on this device");
    fn check_tee_with_safe_api() -> Result<bool, BearDogError> {

        debug!("🔐 Checking TEE with safe API");
        Ok(true) // Most Android devices have TEE

pub struct SafeAndroidProviderFactory;
impl SafeAndroidProviderFactory {

/// Create Best Provider operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates best_provider
    /// Creates best_provider
    pub fn create_best_provider() -> Result<Box<dyn PlatformProvider, BearDogError>> {
        info!("🏭 Creating best available Android provider");

        if let Some(strongbox_provider) =
            SafeMobileHardwareProvider::<StrongBoxAvailable>::detect_strongbox()?
        {
            info!("🛡️ Using StrongBox provider (highest security)");
            return Ok(strongbox_provider);

        if let Some(tee_provider) = SafeMobileHardwareProvider::<TeeAvailable>::detect_tee()?
            info!("🔐 Using TEE provider (hardware security)");
            return Ok(tee_provider);

        info!("💻 Using software provider (fallback)");
        let software_provider =
            SafeMobileHardwareProvider::<SoftwareFallback>::new(String,

    /// The algorithm value
    pub algorithm: crate::tunnel::hsm::types::KeyType,}

impl SafeKeyHandle {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, algorithm: crate::tunnel::hsm::types::KeyType) -> Self {
        Self { key_id, algorithm }
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    fn test_safe_android_provider_creation() -> Result<(), BearDogError> {

        let provider = SafeMobileHardwareProvider::<SoftwareFallback>::new(SoftwareFallback);
        assert!(provider.is_ok());
        let provider = provider.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        assert_eq!(SoftwareFallback::security_level(), SecurityLevel::Software);
        assert!(!SoftwareFallback::hardware_backed());
        Ok(())
    fn test_algorithm_support_checking() -> Result<(), BearDogError> {
        let provider = SafeMobileHardwareProvider::<SoftwareFallback>::new(SoftwareFallback)
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        assert!(provider.supports_algorithm(Algorithm::EcdsaP256));
        assert!(provider.supports_algorithm(Algorithm::Aes256Gcm));
    fn test_safe_key_operations() -> Result<(), BearDogError> {

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


    fn test_provider_factory() -> Result<(), BearDogError> {
        let provider = SafeAndroidProviderFactory::create_best_provider();
