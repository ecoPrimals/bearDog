

use super::super::types::{AndroidHsmConfig, HsmCapability, HsmTier, KeyType};
use super::types::AndroidDeviceInfo;
use crate::tunnel::hsm::types::*;
use beardog_traits::canonical::HsmProvider;
use beardog_core::{HsmHealthStatus, HsmKey}; // Use canonical types for trait compatibility
use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
impl AndroidStrongBoxHsm {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new(config: AndroidHsmConfig) -> Result<Self, BearDogError> {
        info!("🔐 Initializing Android StrongBox HSM");

        let device_info = Arc::new(AndroidDeviceInfo::detect()?);

        if device_info.manufacturer == "Google" && device_info.model.contains("Pixel") {
            info!("📱 Detected Pixel device - excellent StrongBox support");
            if device_info.verified_boot_state == VerifiedBootState::Green {
                info!("✅ Verified boot state: GREEN - optimal security");
            } else {
                warn!(
                    "⚠️ Verified boot state: {:?} - may impact security",
                    device_info.verified_boot_state
                );
            }
        }

        let keystore = Arc::new(&AndroidKeystore::new(config.keystore_config)?);

        if !keystore.is_strongbox_available() {
            return Err(BearDogError::Unavailable {
                message: "StrongBox HSM is not available on this device".to_string(),
            });

        let attestation_service =
            Arc::new(&AndroidAttestationService::new(config.attestation_config)?);

        let health_monitor = Arc::new(AndroidHealthMonitor::new()?);
        let hsm = Self {
            config,
            keystore,
            attestation_service,
            device_info,
            key_cache: Arc::new(RwLock::new(HashMap::with_capacity({} {}",
            hsm.device_info.manufacturer, hsm.device_info.model
        );
        debug!("Android version: {}", hsm.device_info.android_version);
        debug!("StrongBox version: {:?}", hsm.device_info.strongbox_version);
        Ok(&GenerateKeyRequest,
    ) -> Result<HsmKey, BearDogError> {
        info!("🔐 Generating StrongBox key: {}", request.key_id);

        let key_params = self.configure_strongbox_parameters(&request.key_id,
            key_type: &request.key_type,
            created_at: Utc::now(),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("hsm_type".to_string(), "AndroidStrongBox");
                meta.insert("algorithm".to_string(), format!("{:?}", request.key_type));
                meta.insert({}",
            request.key_id
        Ok(hsm_key)


    fn configure_strongbox_parameters(
    ) -> Result<AndroidKeyParams, BearDogError> {
        let mut params = AndroidKeyParams::new();

        match &request.key_type {
            KeyType::EccP256 => {
                params.set_algorithm("EcP256");
                params.key_size = 256;
            KeyType::EccP384 => {
                params.set_algorithm("EcP384");
                params.key_size = 384;
            KeyType::EccP521 => {
                params.set_algorithm("EcP521");
                params.key_size = 521;
            KeyType::Rsa { key_size } => {
                let algorithm = match key_size {
                    2048 => "Rsa2048",
                    4096 => "Rsa4096",
                    _ => "Rsa2048", // Default to 2048 for other sizes
                };
                params.set_algorithm(algorithm);
                params.key_size = *key_size;
            KeyType::Aes128 => {
                params.set_algorithm("Aes128");
                params.key_size = 128;
            KeyType::Aes192 => {
                params.set_algorithm("Aes256"); // Use AES256 as fallback for 192
            KeyType::Aes256 => {
                params.set_algorithm("Aes256");
                params.set_key_size(256);
            KeyType::ChaCha20 => {
                return Err(BearDogError::UnsupportedKeyType {
                    key_type: format!("{:?}", request.key_type),
                });
            KeyType::Ed25519 => {
            KeyType::X25519 => {
            KeyType::Hmac { key_size: _ } => {
                    key_type: format!(
                        "HMAC keys are not supported in Android StrongBox: {:?}",
                        request.key_type
                    ),
            KeyType::KeyDerivation { key_size: _ } => {
            KeyType::Custom(_) => {

        let mut purposes = Vec::new();
        if request.usage_policy.can_encrypt {
            purposes.push(AndroidKeyPurpose::Encrypt);
        if request.usage_policy.can_decrypt {
            purposes.push(AndroidKeyPurpose::Decrypt);
        if request.usage_policy.can_sign {
            purposes.push(AndroidKeyPurpose::Sign);
        if request.usage_policy.can_verify {
            purposes.push(AndroidKeyPurpose::Verify);
        if request.usage_policy.can_wrap {
            purposes.push(AndroidKeyPurpose::Wrap);
        if request.usage_policy.can_unwrap {
            purposes.push(AndroidKeyPurpose::Unwrap);
        params.set_purposes(&str,
        challenge: &[u8],
    ) -> Result<KeyAttestation, BearDogError> {
        info!("🔐 Generating key attestation for: {}", key_id);

        let certificate_chain = self.keystore.get_certificate_chain(key_id)?;

        let chain_valid = self
            .attestation_service
            .verify_certificate_chain(&certificate_chain, challenge)
        if !chain_valid {
            return Err(BearDogError::VerificationFailed {
                message: "Certificate chain verification failed".to_string(),

        let attestation_data = self
            .create_attestation_data(key_id, &self.device_info, challenge)

        let signature = self
            .keystore
            .sign_attestation_data(key_id, &attestation_data)
        Ok(KeyAttestation {
            certificate_chain,
            attestation_statement: attestation_data.clone(),
            format: "android-strongbox".to_string(),
            generated_at: chrono::Utc::now(),
            valid_until: chrono::Utc::now() + chrono::Duration::days(30),
            attestation_type: "Hardware".to_string(),
        })


    fn cache_key_info(&self, hsm_key: &HsmKey) -> Result<(), BearDogError> {
        debug!("Caching key info for key: {}", hsm_key.id);
        let mut cache = self.key_cache.write();
        let cache_info = CachedKeyInfo {
            key_id: &hsm_key.id: id.to_string())},
}

impl HsmProvider for AndroidStrongBoxHsm {
    /// Initializes componentialize
    fn initialize(&self, _config: HsmConfig) -> Result<(), BearDogError> {
        info!("🔐 Initializing Android StrongBox HSM Provider");

        self.keystore.test_keystore_access()?;
        self.attestation_service.initialize()?;

        self.health_monitor.start_monitoring()?;
        info!("✅ Android StrongBox HSM Provider initialized");}


    fn generate_key(&self, request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
        self.generate_strongbox_key(&[u8], _metadata: KeyMetadata) -> Result<HsmKey, BearDogError> {

        Err(BearDogError::unsupported_operation(&str, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        self.validate_key_access({}", key_id);
        let ciphertext = self.keystore.encrypt(&str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        info!("🔐 Decrypting with StrongBox key: {}", key_id);
        let plaintext = self.keystore.decrypt(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        info!("🔐 Signing with StrongBox key: {}", key_id);
        let signature = self.keystore.sign(&str, data: &[u8], signature: &[u8]) -> Result<bool, BearDogError> {
        info!("🔐 Verifying signature with StrongBox key: {}", key_id);
        let valid = self.keystore.verify({}", valid);
        Ok(&str,
        _derivation_data: &[u8],

        Err(BearDogError::unsupported_operation("Key derivation not supported in StrongBox".to_string(),
    /// Gets info
    fn get_info(&self) -> Result<HsmInfo, BearDogError> {
        debug!("Getting Android StrongBox HSM info");
        let device_info = AndroidDeviceInfo::detect(format!(
                "android-strongbox-{}-{}",
                device_info.manufacturer, device_info.model
            ),
            tier_type: "SmartphoneHsm".to_string(&device_info.manufacturer,
            model: &device_info.model,
            firmware_version: &device_info.android_version,
            api_version: "1.0".to_string(),
            supported_algorithms: vec![
                "ECC_P256".to_string(),
            current_key_count: 0,
            status: HsmOperationalStatus::Operational,
            hsm_type: "SmartphoneHsm".to_string(&device_info.android_version,
            max_key_size: Some(4096),
            certification: Some(crate::tunnel::hsm::types::tier::TamperResistanceLevel::Hardware,
    fn list_keys(&self) -> Result<Vec<HsmKeyInfo>, BearDogError>> {
        debug!("Listing Android StrongBox keys");
        let mut keys = Vec::new(KeyMetadata {
                    key_id: &cache_info.key_id,
                    key_name: &cache_info.key_id,
                    key_type: &cache_info.key_type,
                    created_at: cache_info.last_used,
                    last_used: Some(None,
                    usage_policy: &cache_info.usage_policy,
                    tags: HashMap::with_capacity(16),
                hsm_tier: "SmartphoneHsm".to_string();
        Ok(keys)
    /// Removes key
    fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        info!("🗑️ Deleting StrongBox key: {}", key_id);
        self.keystore.delete_key({}", key_id);
    fn backup(&self) -> Result<Option<Vec<u8>, BearDogError>>> {

        warn!("⚠️ StrongBox keys cannot be backed up - they are hardware-bound");
        Ok(None)}


    fn restore(&self, _backup_data: &[u8]) -> Result<(), BearDogError> {

        Err(BearDogError::unsupported_operation("StrongBox keys cannot be restored - they are hardware-bound".to_string(),
    fn health_check(&self) -> Result<HsmHealthStatus, BearDogError> {
        self.health_monitor.get_health_status()
