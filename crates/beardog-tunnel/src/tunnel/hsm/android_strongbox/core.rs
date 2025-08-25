// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Android StrongBox HSM Core Implementation
///
/// This module contains the main AndroidStrongBoxHsm implementation and the
/// HsmProvider trait implementation for Android StrongBox functionality.

use super::super::types::{AndroidHsmConfig, HsmCapability, HsmTier, KeyType};
use super::types::AndroidDeviceInfo;
use crate::tunnel::hsm::types::*;
use beardog_traits::canonical::HsmProvider;
use async_trait::async_trait;
use beardog_core::{HsmHealthStatus, HsmKey}; // Use canonical types for trait compatibility
use beardog_errors::{BearDogError, BearDogResult};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
impl AndroidStrongBoxHsm {
    /// Create a new Android StrongBox HSM instance
    ///
    /// This method initializes all components of the Android StrongBox HSM,
    /// including device detection, keystore validation, and health monitoring.
    /// # Arguments
    /// * `config` - Android HSM configuration
    /// # Returns
    /// * `Ok(AndroidStrongBoxHsm)` - Successfully initialized HSM
    /// * `Err(BearDogError)` - Initialization failure
    pub async fn new(config: AndroidHsmConfig) -> BearDogResult<Self> {
        info!("🔐 Initializing Android StrongBox HSM");
        // Initialize device information
        let device_info = Arc::new(AndroidDeviceInfo::detect().await?);
        // Verify GrapheneOS compatibility
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
        // Initialize keystore
        let keystore = Arc::new(AndroidKeystore::new(config.keystore_config.clone()).await?);
        // Verify StrongBox availability
        if !keystore.is_strongbox_available() {
            return Err(BearDogError::Unavailable {
                message: "StrongBox HSM is not available on this device".to_string(),
            });
        // Initialize attestation service
        let attestation_service =
            Arc::new(AndroidAttestationService::new(config.attestation_config.clone()).await?);
        // Initialize health monitor
        let health_monitor = Arc::new(AndroidHealthMonitor::new().await?);
        let hsm = Self {
            config,
            keystore,
            attestation_service,
            device_info,
            key_cache: Arc::new(RwLock::new(HashMap::new())),
            health_monitor,
        };
        info!("✅ Android StrongBox HSM initialized successfully");
        debug!(
            "Device: {} {}",
            hsm.device_info.manufacturer, hsm.device_info.model
        );
        debug!("Android version: {}", hsm.device_info.android_version);
        debug!("StrongBox version: {:?}", hsm.device_info.strongbox_version);
        Ok(hsm)
    }
    /// Generate a new key in Android StrongBox
    /// This method creates a new hardware-backed key in the Android StrongBox,
    /// with optional user presence validation and key attestation.
    /// * `request` - Key generation request with specifications
    /// * `Ok(HsmKey)` - Successfully generated key
    /// * `Err(BearDogError)` - Key generation failure
    pub async fn generate_strongbox_key(
        &self,
        request: &GenerateKeyRequest,
    ) -> BearDogResult<HsmKey> {
        info!("🔐 Generating StrongBox key: {}", request.key_id);
        // Configure StrongBox parameters
        let key_params = self.configure_strongbox_parameters(request)?;
        // Generate key in Android Keystore/StrongBox
        self.keystore
            .generate_key(&request.key_id, &key_params)
            .await?;
        // Generate attestation if requested
        let attestation = if let Some(challenge) = request.attestation_challenge.as_ref() {
            Some(
                self.generate_key_attestation(&request.key_id, challenge)
                    .await?,
            )
        } else {
            None
        // Create HsmKey structure using canonical beardog_core::HsmKey
        let hsm_key = HsmKey {
            id: request.key_id.clone(),
            key_type: request.key_type.clone(),
            created_at: Utc::now(),
            metadata: {
                let mut meta = std::collections::HashMap::new();
                meta.insert("hsm_type".to_string(), "AndroidStrongBox".to_string());
                meta.insert("algorithm".to_string(), format!("{:?}", request.key_type));
                meta.insert("hardware_backed".to_string(), "true".to_string());
                meta.insert("tier".to_string(), "SmartphoneHsm".to_string());
                if let Some(_attestation) = &attestation {
                    meta.insert("attestation".to_string(), "hardware".to_string());
                }
                // Store original metadata as well
                for (key, value) in &request.metadata.tags {
                    meta.insert(format!("original_{}", key), value.clone());
                meta
            },
        // Cache key information
        self.cache_key_info(&hsm_key).await?;
        info!(
            "✅ StrongBox key generated successfully: {}",
            request.key_id
        Ok(hsm_key)
    /// Configure StrongBox parameters for key generation
    /// Converts a BearDog key generation request into Android-specific parameters.
    fn configure_strongbox_parameters(
    ) -> BearDogResult<AndroidKeyParams> {
        let mut params = AndroidKeyParams::new();
        // Set algorithm and key size using correct enum variants
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
        // Set key purposes
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
        params.set_purposes(purposes);
        // Set StrongBox requirement
        params.set_strongbox_required(true);
        // Set user authentication requirement based on security level
        params.set_user_authentication_required(request.require_user_presence);
        // Set key validity
        if let Some(expires_at) = request.metadata.expires_at {
            params.set_key_validity_end(expires_at);
        // Set attestation challenge if needed
        if request.attestation_challenge.is_some() {
            let challenge = self.generate_attestation_challenge()?;
            params.set_attestation_challenge(challenge);
        Ok(params)
    /// Generate an attestation challenge
    fn generate_attestation_challenge(&self) -> BearDogResult<Vec<u8>> {
        self.attestation_service
            .challenge_generator
            .generate_challenge(32)
    /// Generate key attestation
    /// Creates a hardware-backed attestation for the specified key.}


    async fn generate_key_attestation(
        key_id: &str,
        challenge: &[u8],
    ) -> BearDogResult<KeyAttestation> {
        info!("🔐 Generating key attestation for: {}", key_id);
        // Get certificate chain from keystore
        let certificate_chain = self.keystore.get_certificate_chain(key_id).await?;
        // Verify certificate chain
        let chain_valid = self
            .attestation_service
            .verify_certificate_chain(&certificate_chain, challenge)
        if !chain_valid {
            return Err(BearDogError::VerificationFailed {
                message: "Certificate chain verification failed".to_string(),
        // Create attestation data
        let attestation_data = self
            .create_attestation_data(key_id, &self.device_info, challenge)
        // Sign attestation data
        let signature = self
            .keystore
            .sign_attestation_data(key_id, &attestation_data)
        Ok(KeyAttestation {
            certificate_chain,
            attestation_statement: attestation_data.clone(),
            format: "android-strongbox".to_string(),
            generated_at: chrono::Utc::now(),
            valid_until: chrono::Utc::now() + chrono::Duration::days(30),
            attestation_type: "Hardware".to_string(), // Convert AttestationLevel to String
            attestation_data,
            attestation_signature: signature,
            verified: true,
        })
    /// Cache key information for performance optimization
    async fn cache_key_info(&self, hsm_key: &HsmKey) -> BearDogResult<()> {
        debug!("Caching key info for key: {}", hsm_key.id);
        let mut cache = self.key_cache.write().await;
        let cache_info = CachedKeyInfo {
            key_id: hsm_key.id.clone(),
            key_type: hsm_key.key_type.clone(),
            hsm_type: HsmTier::SmartphoneHsm {
                device_type: SmartphoneType::Android {
                    manufacturer: self.device_info.manufacturer.clone(),
                    model: self.device_info.model.clone(),
                    android_version: self.device_info.android_version.clone(),
                    strongbox_version: self.device_info.strongbox_version.clone(),
                },
                secure_enclave: SecureEnclaveType::AndroidStrongBox {
                    implementation: self.keystore.strongbox_implementation.clone(),
                    hardware_backed: true,
                    key_attestation: true,
                attestation_level: AttestationLevel::Hardware,
                user_presence_required: false,
            strongbox_backed: true,
            attestation_verified: hsm_key
                .attestation
                .as_ref()
                .map(|a| a.verified)
                .unwrap_or(false),
            last_used: Utc::now(),
            usage_count: 0,
            created_at: hsm_key.created_at,
            usage_policy: hsm_key.metadata.usage_policy.clone(),
            health_status: KeyHealthStatus::Healthy,
        cache.insert(hsm_key.id.clone(), cache_info);
        Ok(())
    /// Update key usage statistics
    async fn update_key_usage(&self, key_id: &str) -> BearDogResult<()> {
        if let Some(cache_info) = cache.get_mut(key_id) {
            cache_info.last_used = Utc::now();
            cache_info.usage_count += 1;
    /// Validate key exists and is accessible}


    async fn validate_key_access(&self, key_id: &str) -> BearDogResult<()> {
        // Check if key exists in cache
        let cache = self.key_cache.read().await;
        if !cache.contains_key(key_id) {
            return Err(BearDogError::not_found(format!("Key not found: {key_id}"))},
}

impl HsmProvider for AndroidStrongBoxHsm {
    async fn initialize(&self, _config: HsmConfig) -> BearDogResult<()> {
        info!("🔐 Initializing Android StrongBox HSM Provider");
        // Test keystore access
        self.keystore.test_keystore_access().await?;
        self.attestation_service.initialize().await?;
        // Start health monitoring
        self.health_monitor.start_monitoring().await?;
        info!("✅ Android StrongBox HSM Provider initialized");}


    async fn generate_key(&self, request: GenerateKeyRequest) -> BearDogResult<HsmKey> {
        self.generate_strongbox_key(&request).await
    async fn import_key(&self, _key_data: &[u8], _metadata: KeyMetadata) -> BearDogResult<HsmKey> {
        // StrongBox keys cannot be imported - they must be generated in hardware
        Err(BearDogError::unsupported_operation("StrongBox keys must be generated in hardware".to_string(),
        ))}


    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        self.validate_key_access(key_id).await?;
        self.update_key_usage(key_id).await?;
        info!("🔐 Encrypting with StrongBox key: {}", key_id);
        let ciphertext = self.keystore.encrypt(key_id, plaintext).await?;
        info!("✅ Encryption completed successfully");
        Ok(ciphertext)
    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        info!("🔐 Decrypting with StrongBox key: {}", key_id);
        let plaintext = self.keystore.decrypt(key_id, ciphertext).await?;
        info!("✅ Decryption completed successfully");
        Ok(plaintext)
    async fn sign(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        info!("🔐 Signing with StrongBox key: {}", key_id);
        let signature = self.keystore.sign(key_id, data).await?;
        info!("✅ Signing completed successfully");
        Ok(signature)
    async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool> {
        info!("🔐 Verifying signature with StrongBox key: {}", key_id);
        let valid = self.keystore.verify(key_id, data, signature).await?;
        info!("✅ Signature verification completed: {}", valid);
        Ok(valid)
    async fn derive_key(
        _master_key_id: &str,
        _derivation_data: &[u8],
        // Key derivation not directly supported in Android StrongBox
        Err(BearDogError::unsupported_operation("Key derivation not supported in StrongBox".to_string(),
    async fn get_info(&self) -> BearDogResult<HsmInfo> {
        debug!("Getting Android StrongBox HSM info");
        let device_info = AndroidDeviceInfo::detect().await?;
        Ok(HsmInfo {
            instance_id: format!(
                "android-strongbox-{}-{}",
                device_info.manufacturer, device_info.model
            ),
            tier_type: "SmartphoneHsm".to_string(),
            vendor: device_info.manufacturer.clone(),
            model: device_info.model.clone(),
            firmware_version: device_info.android_version.clone(),
            api_version: "1.0".to_string(),
            capabilities: vec![
                HsmCapability::KeyGeneration,
                HsmCapability::Signing,
                HsmCapability::Encryption,
                HsmCapability::Decryption,
                HsmCapability::KeyAttestation,
                HsmCapability::UserPresenceValidation,
            ],
            supported_algorithms: vec![
                "ECC_P256".to_string(),
                "RSA_2048".to_string(),
                "AES_256".to_string(),
            max_key_count: 100,
            current_key_count: 0,
            status: HsmOperationalStatus::Operational,
            hsm_type: "SmartphoneHsm".to_string(),
            version: device_info.android_version.clone(),
            max_key_size: Some(4096),
            certification: Some("Android StrongBox".to_string()),
            tamper_resistance: crate::tunnel::hsm::types::tier::TamperResistanceLevel::Hardware,
    async fn list_keys(&self) -> BearDogResult<Vec<HsmKeyInfo>> {
        debug!("Listing Android StrongBox keys");
        let mut keys = Vec::new();
        // Get keys from cache
        for (_key_id, cache_info) in cache.iter() {
            let key_info = HsmKeyInfo {
                metadata: KeyMetadata {
                    key_id: cache_info.key_id.clone(),
                    key_name: cache_info.key_id.clone(),
                    key_type: cache_info.key_type.clone(),
                    created_at: cache_info.last_used,
                    last_used: Some(cache_info.last_used),
                    expires_at: None,
                    usage_policy: cache_info.usage_policy.clone(),
                    tags: HashMap::new(),
                hsm_tier: "SmartphoneHsm".to_string(),
                health_status: cache_info.health_status.clone(),
                performance_metrics: KeyPerformanceMetrics {
                    avg_latency_ms: 0.0,
                    ops_per_second: 0.0,
                    error_rate: 0.0,
                    total_operations: 0,
                last_accessed: Some(cache_info.last_used),
                access_count: 0,
                key_id: cache_info.key_id.clone(),
                key_type: cache_info.key_type.clone(),
                hsm_type: "SmartphoneHsm".to_string(),
                created_at: cache_info.last_used,
                usage_policy: cache_info.usage_policy.clone(),
            };
            keys.push(key_info);
        Ok(keys)
    async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        info!("🗑️ Deleting StrongBox key: {}", key_id);
        self.keystore.delete_key(key_id).await?;
        // Remove from cache
        cache.remove(key_id);
        info!("✅ StrongBox key deleted successfully: {}", key_id);
    async fn backup(&self) -> BearDogResult<Option<Vec<u8>>> {
        // StrongBox keys cannot be backed up - they are hardware-bound
        warn!("⚠️ StrongBox keys cannot be backed up - they are hardware-bound");
        Ok(None)}


    async fn restore(&self, _backup_data: &[u8]) -> BearDogResult<()> {
        // StrongBox keys cannot be restored - they are hardware-bound
        Err(BearDogError::unsupported_operation("StrongBox keys cannot be restored - they are hardware-bound".to_string(),
    async fn health_check(&self) -> BearDogResult<HsmHealthStatus> {
        self.health_monitor.get_health_status().await
