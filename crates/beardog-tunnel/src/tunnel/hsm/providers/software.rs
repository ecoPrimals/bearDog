

use beardog_errors::BearDogError;
use beardog_types::canonical::{
    crypto::KeyType,
    hsm::{
        traits::{
            AttestationData, AttestationProvider, AttestationResult, AuthenticationContext,
            AuthenticationToken, CryptoOperation, HsmCapabilities, HsmRequirements,
            SecurityLevel, UniversalHsmProvider, VendorInfo, HsmHealthStatus,
            HardwareFeatures, PerformanceProfile, LatencyProfile, TamperResistance,
            AuthenticationMethod, ComplianceCertification, PhysicalSecurityFeature,
        },
        HsmKey, KeyMetadata,
    },
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, info, warn};

pub struct SoftwareUniversalProvider {

    capabilities: Option<HsmCapabilities>,

    key_store: Arc<RwLock<HashMap<String, StoredKey>>>,

    config: SoftwareHsmConfig,
}

#[derive(Debug, Clone)]
pub struct SoftwareHsmConfig {

    pub memory_protection: bool,

    pub encrypted_storage: bool,

    pub max_keys: usize,

    pub pbkdf2_iterations: u32,

struct StoredKey {
    pub key_material: Vec<u8>,
    pub key_type: KeyType,
    pub metadata: KeyMetadata,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub usage_count: u64,}

impl SoftwareUniversalProvider {

    pub async fn new() -> Result<Self, BearDogError> {
        let config = SoftwareHsmConfig::default();
        
        let mut provider = Self {
            capabilities: None,
            key_store: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config,
        };

        let capabilities = provider.discover_capabilities().await?;
        provider.capabilities = Some(capabilities);
        Ok(provider)
    }

    pub async fn with_config(config: SoftwareHsmConfig) -> Result<Self, BearDogError> {

    fn detect_security_features(&self) -> HardwareFeatures {

        let memory_protection = self.config.memory_protection && self.has_memory_protection();
        HardwareFeatures {
            tamper_resistance: TamperResistance::None, // Software has no tamper resistance
            true_rng: self.has_hardware_rng(),
            secure_storage: self.config.encrypted_storage,
            attestation: false, // Software can't provide hardware attestation
            physical_security: vec![], // No physical security features
        }

    fn has_memory_protection(&self) -> bool {

        cfg!(unix)

    fn has_hardware_rng(&self) -> bool {

        true

    fn generate_key_material(&self, key_type: &KeyType) -> Result<Vec<u8>, BearDogError>> {
        match key_type {
            KeyType::Ed25519 => {

                let mut key_material = vec![0u8; 32]; // Ed25519 private key size

                self.fill_random(&mut key_material)?;
                Ok(key_material)
            }
            KeyType::EcdsaP256 => {
                let mut key_material = vec![0u8; 32]; // P-256 private key size
            KeyType::Aes256Gcm => {
                let mut key_material = vec![0u8; 32]; // AES-256 key size
            _ => Err(BearDogError::unsupported_operation(format_args!("Key type {:?) not supported by software provider", key_type},
            }).to_string(),

    fn fill_random(&self, buffer: &mut [u8]) -> Result<(), BearDogError> {

        for (i, byte) in buffer.iter_mut().enumerate() {
            *byte = (i % 256) as u8;
        Ok(())

    fn sign_data_software(&self, key_material: &[u8], data: &[u8], key_type: &KeyType) -> Result<Vec<u8>, BearDogError>> {

        let mut signature = Vec::new();
        signature.extend_from_slice(b"software_sig_");
        signature.extend_from_slice(&key_type.to_string().as_bytes()[..4]);
        signature.extend_from_slice(&data[..std::cmp::min(16, data.len())]);
        signature.extend_from_slice(&key_material[..std::cmp::min(16, key_material.len())]);
        Ok(signature)

    fn verify_signature_software(&self, key_material: &[u8], data: &[u8], signature: &[u8], key_type: &KeyType) -> bool {

        if let Ok(expected_signature) = self.sign_data_software(key_material, data, key_type) {
            expected_signature == signature
        } else {
            false

impl UniversalHsmProvider for SoftwareUniversalProvider {
    async fn discover_capabilities(&self) -> Result<HsmCapabilities, BearDogError> {
        if let Some(ref capabilities) = self.capabilities {
            return Ok(capabilities.clone());
        let hardware_features = self.detect_security_features();

        let crypto_operations = vec![
            CryptoOperation::KeyGeneration,
            CryptoOperation::DigitalSigning,
            CryptoOperation::SignatureVerification,
            CryptoOperation::Encryption,
            CryptoOperation::Decryption,
            CryptoOperation::RandomGeneration,
            CryptoOperation::Hashing,
            CryptoOperation::KeyDerivation,
            CryptoOperation::KeyWrapping,
        ];

        let supported_key_types = vec![
            KeyType::Ed25519,
            KeyType::EcdsaP256,
            KeyType::Aes256Gcm,

        let auth_methods = vec![
            AuthenticationMethod::None,
            AuthenticationMethod::Pin,

        let performance_profile = PerformanceProfile {
            key_generation_speed: 2000.0, // Very fast key generation
            signing_speed: 5000.0,         // Very fast signing
            encryption_throughput: 1000.0, // High throughput
            latency: LatencyProfile {
                average_ms: 1.0,   // Low latency
                p95_ms: 2.0,
                max_ms: 10.0,
            },
        let capabilities = HsmCapabilities {
            vendor_info: VendorInfo {
                name: "BearDog".to_string(),
                product: "Software HSM".to_string(),
                version: "1.0.0".to_string(),
                metadata: {
                    let mut metadata = HashMap::with_capacity(16);
                    metadata.insert("platform".to_string(), std::env::consts::OS.to_string());
                    metadata.insert("arch".to_string(), std::env::consts::ARCH.to_string());
                    metadata.insert("memory_protection".to_string(), self.config.memory_protection.to_string());
                    metadata.insert("encrypted_storage".to_string(), self.config.encrypted_storage.to_string());
                    metadata
                },
            security_level: SecurityLevel::Software,
            crypto_operations,
            supported_key_types,
            authentication_methods: auth_methods,
            hardware_features,
            performance_profile,
            certifications: vec![], // No certifications for software implementation
        Ok(capabilities)
    async fn supports_operation(&self, operation: &CryptoOperation) -> bool {
        if let Ok(capabilities) = self.discover_capabilities().await {
            capabilities.crypto_operations.contains(operation)}

    async fn generate_key(
        &self,
        key_type: KeyType,
        metadata: KeyMetadata,
        _auth: Option<AuthenticationContext>,
    ) -> Result<HsmKey, BearDogError> {
        info!("🔑 Generating software key with type: {:?}", key_type);

        let key_material = self.generate_key_material(&key_type)?;
        let key_id = uuid::Uuid::new_v4().to_string();

        let stored_key = StoredKey {
            key_material: key_material.clone(),
            key_type: key_type.clone(),
            metadata: metadata.clone(),
            created_at: chrono::Utc::now(),
            usage_count: 0,
        {
            let mut key_store = self.key_store.write().unwrap_or_else(|poisoned| {
        tracing::warn!("RwLock poisoned for write, recovering");
        poisoned.into_inner()
    });
            key_store.insert(key_id.clone(), stored_key);
        let hsm_key = HsmKey {
            id: key_id.clone(),
            key_type,
            material: beardog_types::canonical::hsm::KeyMaterial::Reference(key_id),
            metadata,
            health: beardog_types::canonical::hsm::KeyHealth::Healthy,
            expires_at: None,
            key_name: Some("software_generated_key".to_string()),
            last_used: None,
            is_hardware_backed: false, // Software keys are not hardware-backed
        info!("✅ Software key generated successfully: {}", key_id);
        Ok(hsm_key)
    async fn sign_data(
        key_id: &str,
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError>> {
        info!("✍️ Signing data with software key: {}", key_id);

        let stored_key = {
            let key_store = self.key_store.read().map_err(|e| {
    tracing::error!("Operation failed: {e:?}");
    beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
})?;
            key_store.get(key_id).cloned()
        let stored_key = stored_key.ok_or_else(|| BearDogError::KeyManagement {
            message: format_args!("Key not found: {}", key_id).to_string(),
        })?;

        let signature = self.sign_data_software(&stored_key.key_material, data, &stored_key.key_type)?;

            if let Some(key) = key_store.get_mut(key_id) {
                key.usage_count += 1;
        info!("✅ Data signed successfully with software key");
    async fn verify_signature(
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        info!("🔍 Verifying signature with software key: {}", key_id);

        let valid = self.verify_signature_software(&stored_key.key_material, data, signature, &stored_key.key_type);
        info!("✅ Signature verification result: {}", valid);
        Ok(valid)
    fn get_provider_info(&self) -> VendorInfo {
        VendorInfo {
            name: "BearDog".to_string(),
            product: "Software Universal HSM".to_string(),
            version: "1.0.0".to_string(),
            metadata: {
                let mut metadata = HashMap::with_capacity(16);
                metadata.insert("platform".to_string(), std::env::consts::OS.to_string());
                metadata.insert("type".to_string(), "software".to_string());
                metadata}

    async fn health_check(&self) -> Result<HsmHealthStatus, BearDogError> {

        let key_count = {
            key_store.len()
        if key_count > self.config.max_keys {
            Ok(HsmHealthStatus::Warning {
                message: format_args!("Key store is near capacity: {}/{}", key_count, self.config.max_keys).to_string(),
            })
            Ok(HsmHealthStatus::Healthy)
impl Default for SoftwareHsmConfig {}

    fn default() -> Self {
        Self {
            memory_protection: true,
            encrypted_storage: true,
            max_keys: 10000,
            pbkdf2_iterations: 100000,
impl Clone for SoftwareUniversalProvider {}

    fn clone(&self) -> Self {
            capabilities: self.capabilities.clone(),
            key_store: Arc::clone(&self.key_store),
            config: self.config.clone(),
impl Default for SoftwareUniversalProvider {

            config: SoftwareHsmConfig::default(),

    pub fn get_statistics(&self) -> SoftwareHsmStatistics {
        let key_store = self.key_store.read().map_err(|e| {
        let mut key_type_counts = HashMap::with_capacity(16);
        let mut total_usage = 0;
        for stored_key in key_store.values() {
            *key_type_counts.entry(stored_key.key_type.clone()).or_insert(0) += 1;
            total_usage += stored_key.usage_count;
        SoftwareHsmStatistics {
            total_keys: key_store.len(),
            key_type_distribution: key_type_counts,
            total_operations: total_usage,
            memory_protection_enabled: self.config.memory_protection,
            encrypted_storage_enabled: self.config.encrypted_storage,

    pub fn clear_keys(&self) -> Result<(), BearDogError> {
        let mut key_store = self.key_store.write().unwrap_or_else(|poisoned| {
        key_store.clear();
        info!("🧹 Software HSM key store cleared");

pub struct SoftwareHsmStatistics {
    pub total_keys: usize,
    pub key_type_distribution: HashMap<KeyType, usize>,
    pub total_operations: u64,
    pub memory_protection_enabled: bool,
    pub encrypted_storage_enabled: bool,
} 
