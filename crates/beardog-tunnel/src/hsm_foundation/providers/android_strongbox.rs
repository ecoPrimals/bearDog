// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

use super::super::{error::*, traits::*, types::*};
use beardog_types::canonical::hsm::status::HealthMetrics;
use beardog_types::canonical::KeyType;
use crate::CoreCapabilities;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct AndroidStrongBoxProvider {

    config: Arc<RwLock<Option<HsmConfig>>>,

    keys: Arc<RwLock<HashMap<String, HsmKey>>>,

    metrics: Arc<RwLock<PerformanceMetrics>>,

    capabilities: CoreCapabilities,

    strongbox_enabled: bool,

    device_info: AndroidDeviceInfo,
}

#[derive(Debug, Clone)]
    /// The model value
    pub model: String,

    /// Number of api_level
    pub api_level: u32,

    /// Optional strongbox version
    pub strongbox_version: Option<String>,

    /// Whether attestation_supported is enabled
    pub attestation_supported: bool,}

impl AndroidStrongBoxProvider {

/// New operation.
    /// Creates a new instance
    pub fn new() -> HsmResult<Self> {
        let device_info = Self::detect_device_info()?;
        let strongbox_enabled = Self::check_strongbox_availability(&device_info);
        
        Ok(Self {
            config: Arc::new(RwLock::new(None)),
            keys: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            metrics: Arc::new(RwLock::new(PerformanceMetrics::default(CoreCapabilities {
                key_generation: strongbox_enabled,
                signing: strongbox_enabled,
                encryption: strongbox_enabled,
                key_derivation: strongbox_enabled,
                hardware_backed: strongbox_enabled,
                attestation: device_info.attestation_supported,
            },
            strongbox_enabled,
            device_info,
        })
    }

/// Create Test Instance operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates test_instance
    /// Creates test_instance
    pub fn create_test_instance() -> Result<Self, BearDogError> {
        Self::new()
            .map_err(|e| BearDogError::internal({}", e)))


    fn detect_device_info() -> HsmResult<AndroidDeviceInfo> {

        if cfg!(target_os = "android") {

            Ok(AndroidDeviceInfo {
                manufacturer: "Unknown".to_string(),}

                model: "Android Device".to_string(),
                strongbox_version: Some(true,
            })
        } else {

                manufacturer: "Mock".to_string(),
                model: "Development Device".to_string(),
                strongbox_version: Some("2.0".to_string()),
        }


    fn check_strongbox_availability(device_info: &AndroidDeviceInfo) -> bool {

        if device_info.api_level < 28 {
            return false;

        match device_info.manufacturer.as_str() {
            "Google" => {

                matches!(device_info.model.as_str(), 
                    "Pixel 3" | "Pixel 3 XL" | "Pixel 4" | "Pixel 4 XL" |
                    "Pixel 5" | "Pixel 6" | "Pixel 6 Pro" | "Pixel 7" | 
                    "Pixel 7 Pro" | "Pixel 8" | "Pixel 8 Pro" | 
                    "Development Device" // For testing
                )
            }
            "Samsung" => {

                device_info.model.contains("Galaxy S") || device_info.model.contains("Galaxy Note")
            "Mock" => true, // For development
            _ => false,


    fn generate_hardware_key_id() -> String {
        format!("strongbox-{}", Uuid::new_v4())

    /// Creates hardware_key
    fn create_hardware_key(&self, key_type: &KeyType) -> HsmResult<KeyMaterial> {
        if !self.strongbox_enabled {
            return Err(HsmError::hardware_unavailable(
                "StrongBox not available on this device"
            ));

        let hsm_id = format!("android-strongbox-{}", self.device_info.manufacturer);
        let key_handle = Self::generate_hardware_key_id();

        self.mock_hardware_key_generation(key_type, &key_handle)?;
        Ok(KeyMaterial::HardwareRef { hsm_id, key_handle })


    fn mock_hardware_key_generation(&KeyType, key_handle: &str) -> HsmResult<()> {

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        match key_type {
            KeyType::EllipticCurve { curve } => {
                match curve {
                    EcCurve::P256 | EcCurve::P384 => {
                        tracing::info!("Generated {} hardware key: {}", curve, key_handle);
                    }
                    _ => {
                        return Err(HsmError::invalid_key_type(
                            "hardware_key_generation",
                            "P256 or P384",
                            &format!("{:?}", curve)
                        ));
                }
            KeyType::Ed25519 => {
                tracing::info!("Generated Ed25519 hardware key: {}", key_handle);
            KeyType::Aes { bits } if *bits == 256 => {
                tracing::info!("Generated AES-256 hardware key: {}", key_handle);
            _ => {
                return Err(HsmError::invalid_key_type(
                    "hardware_key_generation",
                    "supported hardware key type",
                    &format!("{:?}", key_type)
                ));
        Ok(&str, key_handle: &str, data: &[u8]) -> HsmResult<Vec<u8>> {
                "StrongBox not available for operations"

        self.mock_hardware_operation(&str, key_handle: &str, data: &[u8]) -> HsmResult<Vec<u8>> {

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        match operation {
            "sign" => {
                let signature = format!("strongbox_signature_{}_{}", key_handle, data.len());
                Ok(signature.into_bytes())
            "encrypt" => {

                let key_bytes: Vec<u8> = key_handle.bytes().cycle().take(data.len()).collect();
                let encrypted: Vec<u8> = data.iter().zip(key_bytes.iter())
                    .map(|(d, k)| d ^ k)
                    .collect();
                Ok(encrypted)
            "decrypt" => {

                let decrypted: Vec<u8> = data.iter().zip(key_bytes.iter())
                Ok(decrypted)
            _ => Err(HsmError::invalid_key_type(&str, success: bool, duration_ms: u64) {
        let mut metrics = self.metrics.write();

        if success {
            metrics.success_rate = (metrics.success_rate * 0.95) + 0.05;
            metrics.ops_per_second = 1000.0 / duration_ms.max(1) as f64;
            metrics.success_rate = metrics.success_rate * 0.95;
            metrics.error_count += 1;
        metrics.avg_latency_ms = (metrics.avg_latency_ms * 0.9) + (duration_ms as f64 * 0.1);
        tracing::debug!(
            "StrongBox operation "{}" completed in {}ms, success: {}, success_rate: {:.2}%",
            operation, duration_ms, success, metrics.success_rate * 100.0
        );
impl Default for AndroidStrongBoxProvider {}

    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Failed to create AndroidStrongBoxProvider", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Failed to create AndroidStrongBoxProvider: {:?}", e)
).into())
})
impl HsmProvider for AndroidStrongBoxProvider {}


    fn provider_info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "Android StrongBox HSM Provider".to_string(),
            version: "2.0.0".to_string(),
    /// Initializes componentialize
    fn initialize(&self, config: &HsmConfig) -> HsmResult<()> {
        let mut config_guard = self.config.write();
        *config_guard = Some(config);
                format!("StrongBox not available on {} {}", 
                    self.device_info.manufacturer, self.device_info.model)
        tracing::info!(
            "Initialized Android StrongBox provider on {} {} (API {})",
            self.device_info.manufacturer,
            self.device_info.model,
            self.device_info.api_level
    fn shutdown(&self) -> HsmResult<()> {

        let mut keys = self.keys.write();
        keys.clear();
        tracing::info!("Shutdown Android StrongBox provider");}


    fn generate_key(&self, request: GenerateKeyRequest) -> HsmResult<HsmKey> {
        let start_time = std::time::Instant::now();
        let key_id = request.key_id.unwrap_or_else(Self::generate_hardware_key_id);
        let material = self.create_hardware_key(&request.key_type)?;
        let key = HsmKey {
            id: key_id.clone(request.key_type,
            material,
            metadata: request.metadata,
            health: KeyHealth::Healthy,
            tier: HsmTier::CertifiedHardware,
            created_at: chrono::Utc::now(None,
        };

        keys.insert(key_id, key);
        let duration = start_time.elapsed().as_millis().min(u64::MAX as u128) as u64;
        self.update_metrics("generate_key", true, duration);
        tracing::info!("Generated StrongBox hardware key: {}", key.id);
        Ok(&[u8], _metadata: KeyMetadata) -> HsmResult<HsmKey> {

        Err(HsmError::insufficient_permissions(
            "Key import not supported by StrongBox for security reasons"
        ))}

    /// Gets key
    fn get_key(&self, key_id: &str) -> HsmResult<HsmKey> {
        let keys = self.keys.read();
        keys.get(key_id)
            .cloned()
            .ok_or_else(|| HsmError::key_not_found(key_id))
    fn list_keys(&self, filter: Option<KeyFilter>) -> HsmResult<Vec<HsmKey>> {
        let mut result: Vec<HsmKey> = keys.values().cloned().collect();

        if let Some(filter) = filter {
            if let Some(hardware_backed) = filter.hardware_backed {
                if hardware_backed {
                    result.retain(|k| matches!(k.material, KeyMaterial::HardwareRef { .. }));
            if let Some(key_type) = filter.key_type {
                result.retain(|k| k.key_type == key_type);
        Ok(result)
    /// Removes key
    fn delete_key(&self, key_id: &str) -> HsmResult<()> {
        let key = keys.remove(key_id)
            .ok_or_else(|| HsmError::key_not_found(key_id))?;

        if let KeyMaterial::HardwareRef { key_handle, .. } = &key.material {
            tracing::info!("Deleted StrongBox hardware key: {}", key_handle);
    fn sign(&str, data: &[u8], _algorithm: Option<&str>) -> HsmResult<Vec<u8>> {
        let key = self.get_key(key_id)?;
        if !key.metadata.purposes.contains(&KeyPurpose::Sign) {
            return Err(HsmError::insufficient_permissions("sign"));
        let signature = match &key.material {
            KeyMaterial::HardwareRef { key_handle, .. } => {
                self.hardware_operation(&str, data: &[u8], signature: &[u8], _algorithm: Option<&str>) -> HsmResult<bool> {
        if !key.metadata.purposes.contains(&KeyPurpose::Verify) {
            return Err(HsmError::insufficient_permissions(&str, plaintext: &[u8], _algorithm: Option<&str>) -> HsmResult<Vec<u8>> {
        if !key.metadata.purposes.contains(&KeyPurpose::Encrypt) {
            return Err(HsmError::insufficient_permissions(&str, ciphertext: &[u8], _algorithm: Option<&str>) -> HsmResult<Vec<u8>> {
        if !key.metadata.purposes.contains(&KeyPurpose::Decrypt) {
            return Err(HsmError::insufficient_permissions(&str, derivation_info: &[u8]) -> HsmResult<HsmKey> {
        let parent_key = self.get_key(parent_key_id)?;
        if !parent_key.metadata.purposes.contains(&KeyPurpose::Derive) {
            return Err(HsmError::insufficient_permissions("derive_key"));

        let derived_key_id = Self::generate_hardware_key_id();
        let derived_handle = format!("derived-{}-{}", parent_key_id, derivation_info.len());
        let material = KeyMaterial::Derived {
            parent_key_id: parent_key_id.to_string(),
        let derived_key = HsmKey {
            id: derived_key_id.clone(&parent_key.key_type,
            metadata: KeyMetadata {
                name: Some(vec![KeyPurpose::Sign, KeyPurpose::Verify],
                exportable: false,
                hardware_backed: true,
                auth_required: parent_key.metadata.auth_required,
                attributes: HashMap::with_capacity(16),
        keys.insert(derived_key_id, derived_key);
        self.update_metrics("derive_key", true, duration);
        Ok(derived_key)
    fn health_check(&self) -> HsmResult<HsmHealth> {
        let metrics = self.metrics.read().clone();
        let is_healthy = self.strongbox_enabled && metrics.success_rate > 0.95;
        let mut issues = Vec::new({:.1}%", metrics.success_rate * 100.0));
        Ok(HsmHealthStatus::Healthy,
            last_check: chrono::Utc::now(),
            details: HashMap::with_capacity(HealthMetrics {
                ops_per_second: 100.0,
                avg_response_time_ms: 50.0,
                error_rate_percent: 0.0,
                memory_usage_percent: 25.0,
                cpu_usage_percent: 10.0,
                active_connections: 1,
            errors: Vec::new(),
    /// Gets capabilities
    fn get_capabilities(&self) -> CoreCapabilities {
        &self.capabilities
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}


    fn test_android_strongbox_provider_creation() -> Result<(), BearDogError> {
        let provider = AndroidStrongBoxProvider::new()
            .unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Failed to create AndroidStrongBox provider for testing", e);
    format!("Failed to create AndroidStrongBox provider for testing: {:?}", e)
});
        let info = provider.provider_info();
        assert_eq!(info.provider_type, HsmProviderType::AndroidStrongBox);
        assert!(info.description.contains("StrongBox"));
    fn test_device_detection() -> Result<(), BearDogError> {
        let device_info = AndroidStrongBoxProvider::detect_device_info()
    tracing::error!("Expect failed ({}): {:?}", "Device detection should work in test environment", e);
    format!("Device detection should work in test environment: {:?}", e)

        assert!(!device_info.manufacturer.is_empty());
        assert!(!device_info.model.is_empty());
        assert!(device_info.api_level >= 28); // Minimum for StrongBox
    fn test_strongbox_key_operations() -> Result<(), BearDogError> {
        let provider = AndroidStrongBoxProvider::new().unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    format!("Operation failed: {e:?}")

        let request = GenerateKeyRequest {
            key_type: KeyType::EllipticCurve { curve: EcCurve::P256 },
                name: Some(false,
            key_id: None,
        if provider.strongbox_enabled {
            let key = provider.generate_key(request)
                .unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Key generation should succeed in test", e);
    format!("Key generation should succeed in test: {:?}", e)
            assert!(key.id.starts_with("strongbox-"));
            assert!(matches!(key.material, KeyMaterial::HardwareRef { .. }));
            assert_eq!(key.tier, HsmTier::CertifiedHardware);

            let data = b"test data for strongbox signing";
            let signature = provider.sign(&key.id, data, None)
    tracing::error!("Expect failed ({}): {:?}", "Signing should succeed with valid key", e);
    format!("Signing should succeed with valid key: {:?}", e)
            assert!(!signature.is_empty());

            let is_valid = provider.verify(&key.id, data, &signature, None)
    tracing::error!("Expect failed ({}): {:?}", "Signature verification should not fail", e);
    format!("Signature verification should not fail: {:?}", e)
            assert!(is_valid);
    fn test_health_check() -> Result<(), BearDogError> {
        let health = provider.health_check()
    tracing::error!("Expect failed ({}): {:?}", "Health check should not fail", e);
    format!("Health check should not fail: {:?}", e)
        assert_eq!(health.provider_type, HsmProviderType::AndroidStrongBox);
        assert!(health.capabilities.hardware_backed == provider.strongbox_enabled);
} 
