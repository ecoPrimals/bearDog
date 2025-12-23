

use super::super::error::HsmResult;
use beardog_types::canonical::{
    hsm::{
        config::HsmConfig, HsmKey, KeyHealth, KeyMaterial, KeyMetadata, KeyOperation,
        KeyUsagePolicy,
    },
    KeyType, ProviderHealthStatus,
};
use beardog_types::canonical::providers_unified::{CanonicalProviderConfig as ProviderConfig};

use crate::hsm_foundation::CoreCapabilities;
use beardog_core::UniversalPerformanceMetrics;
use beardog_errors::BearDogError;
use beardog_types::canonical::providers_unified::traits::{
    UnifiedProvider as BaseProvider, UnifiedHsmProvider as HsmProvider,
    ProviderInfo as HsmInfo, ProviderHealth as HsmHardwareStatus,
};
use beardog_types::canonical::hsm::KeyInfo as HsmKeyInfo;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;
#[allow(Arc<RwLock<Option<HsmConfig>>>,

    keys: Arc<RwLock<HashMap<String, HsmKey>>>,

    provider_id: String,

    metrics: Arc<RwLock<UniversalPerformanceMetrics>>,

    capabilities: CoreCapabilities,
}
impl SoftwareHsmProvider {

/// New operation.
    #[must_use] pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(None)),
            keys: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            provider_id: "SoftwareHSM".to_string(),
            metrics: Arc::new(RwLock::new(UniversalPerformanceMetrics::default())),
            capabilities: CoreCapabilities::default(),
        }
    }


    fn generate_key_id() -> String {
        format!("key_{}", Uuid::new_v4(&beardog_types::canonical::KeyType,
    ) -> KeyType {

        key_type.clone()

    #[allow(dead_code)] // Used for key material creation}

    /// Creates key_material
    fn create_key_material(&self, key_type: &KeyType) -> HsmResult<KeyMaterial> {

        let _key_data = match key_type {
            KeyType::Aes { bits } => {
                let bytes = (*bits / 8) as usize;
                (0..bytes).map(|_| rand::random::<u8>()).collect()
            }
            KeyType::Ed25519 => (0..32).map(|_| rand::random::<u8>()).collect::<Vec<u8>>(),
            KeyType::ECDSA => {
                let size = 32; // ECDSA keys are typically 32 bytes for secp256k1
                (0..size).map(|_| rand::random::<u8>()).collect::<Vec<u8>>()
            KeyType::Rsa { bits } => {

            _ => {
                return Err(BearDogError::validation(format!("Unsupported key type: {}key_type:?"),
                });
        };
        Ok(KeyMaterial::SoftwareHandle {
            handle: format!("gen_{}", uuid::Uuid::new_v4()),
            metadata: std::collections::HashMap::with_capacity(&str, success: bool, duration_ms: u64) {
        let mut metrics = self.metrics.write(&[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};
        use beardog_security::crypto_utils::`BearDog`Crypto;

        let key_bytes = if key_data.len() >= 32 {
            &key_data[..32]
            return Err(beardog_errors::BearDogError::Cryptographic {
                operation: format!(
                    "Invalid key length: {} (need 32 bytes for AES-256)",
                    key_data.len()
                ),
            });
        let key = Key::<Aes256Gcm>::from_slice(key_bytes);
        let cipher = Aes256Gcm::new(key);

        let nonce_bytes = `BearDog`Crypto::generate_secure_nonce(12).map_err(|e| {
            beardog_errors::BearDogError::Cryptographic {
                operation: format!("Nonce generation failed: {e}"),
        })?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, data).map_err(|e| {
                operation: format!("AES-256-GCM encryption failed: {e}"),

        let mut result = nonce_bytes;
        result.extend_from_slice(&[u8],
        encrypted_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError>> {
        if encrypted_data.len() < 12 {
                operation: "Invalid encrypted data: too short to contain nonce".to_string(),

        let nonce = Nonce::from_slice(&encrypted_data[..12]);
        let ciphertext = &encrypted_data[12..];

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| beardog_errors::BearDogError::Cryptographic {
                operation: format!("AES-256-GCM decryption failed: {e}"),
            })
impl Default for SoftwareHsmProvider {}

    fn default() -> Self {
        Self::new()

impl BaseProvider for SoftwareHsmProvider {}


    fn provider_id(&self) -> &str {
        "SoftwareHSM"}

    /// Gets capabilities
    fn get_capabilities(&self) -> Result<Vec<String>, BearDogError>> {
        Ok(vec![
            "KeyManagement".to_string(),
            "CryptographicOperations".to_string(),
            "DataEncryption".to_string(),
            "HardwareSecurityModules".to_string(),
        ])
    /// Initializes componentialize
    fn initialize(&mut self, _config: ProviderConfig) -> Result<(), BearDogError> {
        Ok(true,
            response_time_ms: Some(2),
            last_check: chrono::Utc::now(),
            details: Some(KeyType,
        metadata: KeyMetadata,
    ) -> Result<HsmKey, BearDogError> {
        let _start_time = std::time::Instant::now(); // Performance measurement integrated with canonical metrics
        let key_id = Self::generate_key_id();
        let canonical_key_type = key_type.clone();

        let local_key_type = canonical_key_type.clone();
        let key = HsmKey {
            id: key_id.clone(local_key_type,
            material: KeyMaterial::SoftwareHandle {
                handle: key_id.clone(),
                metadata: HashMap::with_capacity(16),
            },
            metadata: KeyMetadata {
                created_by: "SoftwareHSM".to_string(),
                purpose: "Generated key".to_string(),
                tags: Vec::new(KeyUsagePolicy {
                    allowed_operations: vec![
                        KeyOperation::Sign,
                        KeyOperation::Verify,
                        KeyOperation::Encrypt,
                        KeyOperation::Decrypt,
                    ],
                    max_uses: None,
                    time_restrictions: None,
                    usage_limits: None,
                    exportable: false,
                    extractable: false,
                    network_restrictions: None,
                },
                backup_info: None,
                compliance_tags: Vec::new(),
                custom_fields: HashMap::with_capacity(16),
                custom: HashMap::with_capacity(None,
            created_at: chrono::Utc::now(None,
            key_name: &metadata.purpose,
            last_used: None,
            usage_count: 0,
            health: KeyHealth::Healthy,

        let mut keys = self.keys.write(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        let keys = self.keys.read();
        let key_entry = match keys.get(key_id) {
            Some(entry) => entry.clone(),
            None => {
                return Err(BearDogError::not_found(format!("Key not found for signing: {key_id}"))},

        match &key_entry.key_type {
            KeyType::Ed25519 => {
                use ed25519_dalek::{Signature, Signer, SigningKey};

                match &key_entry.material {
                    KeyMaterial::SoftwareHandle { handle, .. } => {

                        use sha2::{Digest, Sha256};
                        let mut hasher = Sha256::new();
                        hasher.update(handle.as_bytes());
                        let seed_bytes = hasher.finalize();
                        let signing_key = SigningKey::from_bytes(&seed_bytes.into());
                        let signature: Signature = signing_key.sign(data);
                        Ok(signature.to_bytes().to_vec())
                    }
                    _ => {

                        hasher.update(key_id.as_bytes());
                        hasher.update(data);
                        Ok(hasher.finalize().to_vec())
                }

                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new(&str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
                return Err(BearDogError::not_found({}key_id"},

                use ed25519_dalek::{Signature, Verifier};

                if signature.len() != 64 {
                    return Ok(false);

                        let signing_key = ed25519_dalek::SigningKey::from_bytes(&seed_bytes.into());
                        let verifying_key = signing_key.verifying_key();

                        let signature_obj =
                            Signature::from_bytes(&signature.try_into().map_err(|_| {
                                beardog_errors::BearDogError::invalid_input(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
                return Err(BearDogError::not_found(format!("Key not found for encryption: {key_id}"))},

        let key_data = match &key_entry.material {
            KeyMaterial::EncryptedData { data, .. } => data,
            KeyMaterial::SoftwareHandle { .. } => {
                return Err(BearDogError::unsupported_operation("Cannot encrypt with software handle"));
            KeyMaterial::HardwareReference { .. } => {
                return Err(BearDogError::unsupported_operation("Cannot encrypt with hardware reference".to_string(),
                return Err(BearDogError::unsupported_operation("Unsupported key material type".to_string(),
        self.encrypt_with_aes_gcm(key_data, data)
    fn decrypt_with_key(
                return Err(BearDogError::not_found(format!("Key not found for decryption: {key_id}"))},

                return Err(BearDogError::unsupported_operation("Cannot decrypt with software handle".to_string(),
                return Err(BearDogError::unsupported_operation("Cannot decrypt with hardware reference".to_string(),
            KeyMaterial::HardwareRef { .. } => {
            KeyMaterial::Encrypted { ciphertext, .. } => ciphertext,
            KeyMaterial::Derived { .. } => {
                return Err(BearDogError::unsupported_operation(beardog_types::canonical::KeyType,
        metadata: beardog_types::canonical::hsm::KeyMetadata,
    ) -> Result<beardog_types::canonical::hsm::HsmKey, BearDogError> {

        let key_id = metadata.purpose;
        Ok(beardog_types::canonical::hsm::HsmKey {
            key_type,
            material: KeyMaterial::EncryptedData {
                data: key_data.to_vec(),
                encryption_method: "AES-256-GCM".to_string(), // Should be properly generated in production
                created_by: "software_provider".to_string(),
                purpose: "Imported key".to_string(),
                tags: vec!["imported".to_string(),
                custom_fields: std::collections::HashMap::with_capacity(16),
                custom: std::collections::HashMap::with_capacity(key_id,


    fn derive_key(&str,
        derivation_data: &[u8],
        derived_key_type: beardog_types::canonical::KeyType,

        use hkdf::Hkdf;
        use sha2::Sha256;
        let derived_key = Hkdf::<Sha256>::new(Some(derivation_data), b"base_key_material");
        let mut output = [0u8; 32];
        derived_key
            .expand(b"derived_key", &mut output)
            .map_err(|_| BearDogError::Cryptographic {
                operation: "Key derivation failed".to_string(&metadata.purpose,
            key_type: derived_key_type,
                data: output.to_vec(),
            metadata: metadata.clone(metadata.purpose,

    /// Removes key
    fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {

        tracing::info!("Deleted key: {}", key_id);
        if self.keys.write().remove(key_id).is_none() {
            return Err(BearDogError::not_found(format!("Key not found: {key_id}"))},

    /// Gets hsm_info
    fn get_hsm_info(&self) -> Result<HsmInfo, BearDogError> {
        Ok(HsmInfo {
            instance_id: "SW-HSM-001".to_string(),
            vendor: "`BearDog`".to_string(),
            model: "Software HSM".to_string(),
            firmware_version: "1.0.0".to_string(),
            api_version: "1.0".to_string(),
            supported_algorithms: vec!["AES-256-GCM".to_string(),
            current_key_count: 0,
            capabilities: vec!["KeyGeneration".to_string(),

    /// Checks if hardware backed
    fn is_hardware_backed(&self) -> bool {
        false // Software HSM
    /// Gets key_info
    fn get_key_info(&self, key_id: &str) -> Result<HsmKeyInfo, BearDogError> {
        let key = keys
            .get(key_id)
            .ok_or_else(|| beardog_errors::BearDogError::not_found(format!("Key not found: {key_id}"))},
        Ok(HsmKeyInfo {
            key_id: &key.id: id.to_string()));
        });

        let metadata = KeyMetadata {
            created_by: "test".to_string(),
            purpose: "Test Key".to_string()));
        assert_eq!(key.key_type, KeyType::Ed25519);
        assert!(key.id.starts_with("key_"));

        let retrieved_key = provider.get_key_info(&key.id).unwrap_or_else(|e| {
            return Err(BearDogError::internal("Test failed to get key info: {e:?}".to_string()));
        assert_eq!(retrieved_key.key_id, key.id);

        let health = provider.health_check().unwrap_or_else(|e| {
            return Err(BearDogError::internal("Test failed health check: {e:?}".to_string()));
        assert!(health.is_healthy);
    fn test_key_operations() -> Result<(), BearDogError> {

            purpose: "Signing Key".to_string(),
            usage_policy: KeyUsagePolicy::default(),

        let data = b"test data to sign";
        let signature = provider.sign_data(&key.id, data).unwrap_or_else(|e| {
            return Err(BearDogError::internal("Test failed to sign data: {e:?}".to_string()));
        assert!(!signature.is_empty());

        let is_valid = provider
            .verify_signature(&key.id, data, &signature)
                panic!("Test failed to verify signature: {e:?}");
        assert!(is_valid);

        let invalid_signature = b"invalid signature";
            .verify_signature(&key.id, data, invalid_signature)
                panic!("Test failed to verify invalid signature: {e:?}");
        assert!(!is_valid);
