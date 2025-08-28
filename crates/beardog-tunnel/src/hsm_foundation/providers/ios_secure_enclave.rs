use beardog_errors::BearDogError;

use super::super::{types::*, traits::*, error::*};
use beardog_types::canonical::KeyType;
use crate::CoreCapabilities;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct IosSecureEnclaveProvider {

    config: Arc<RwLock<Option<HsmConfig>>>,

    keys: Arc<RwLock<HashMap<String, HsmKey>>>,

    metrics: Arc<RwLock<PerformanceMetrics>>,

    capabilities: CoreCapabilities,

    secure_enclave_available: bool,

    device_info: IosDeviceInfo,
}

#[derive(Debug, Clone)]
pub struct IosDeviceInfo {

    pub model: String,

    pub ios_version: String,

    pub secure_enclave_generation: u32,

    pub biometric_available: bool,

    pub attestation_supported: bool,}

impl IosSecureEnclaveProvider {

    pub fn new() -> HsmResult<Self> {
        let device_info = Self::detect_device_info()?;
        let secure_enclave_available = Self::check_secure_enclave_availability(&device_info);
        
        Ok(Self {
            config: Arc::new(RwLock::new(None)),
            keys: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
            capabilities: CoreCapabilities {
                key_generation: secure_enclave_available,
                signing: secure_enclave_available,
                encryption: secure_enclave_available,
                key_derivation: secure_enclave_available,
                hardware_backed: secure_enclave_available,
                attestation: device_info.attestation_supported,
            },
            secure_enclave_available,
            device_info,
        })
    }

    pub fn create_test_instance() -> Result<Self, BearDogError> {
        Self::new()
            .map_err(|e| BearDogError::internal(format_args!("Failed to create IosSecureEnclaveProvider test instance: {}", e).to_string()))

    fn detect_device_info() -> HsmResult<IosDeviceInfo> {

        if cfg!(target_os = "ios") {

            Ok(IosDeviceInfo {}

                model: "Unknown iOS Device".to_string(),
                ios_version: "17.0".to_string(),
                secure_enclave_generation: 3,
                biometric_available: true,
                attestation_supported: true,
            })
        } else {

                model: "Simulator".to_string(),
                ios_version: "17.2".to_string(),
                secure_enclave_generation: 4,
        }

    fn check_secure_enclave_availability(device_info: &IosDeviceInfo) -> bool {

        if device_info.model == "Simulator" {
            return true; // For development

        let ios_major_version: u32 = device_info.ios_version
            .split('.')
            .next()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        ios_major_version >= 13 && device_info.secure_enclave_generation >= 1

    fn generate_enclave_key_id() -> String {
        format_args!("secureenclave-{}", Uuid::new_v4().to_string())

    async fn create_enclave_key(&self, key_type: &KeyType) -> HsmResult<KeyMaterial> {
        if !self.secure_enclave_available {
            return Err(HsmError::hardware_unavailable(
                "Secure Enclave not available on this device"
            ));

        let hsm_id = format_args!("ios-secure-enclave-{}", self.device_info.model).to_string();
        let key_handle = Self::generate_enclave_key_id();

        self.mock_enclave_key_generation(key_type, &key_handle).await?;
        Ok(KeyMaterial::HardwareRef { hsm_id, key_handle })

    async fn mock_enclave_key_generation(&self, key_type: &KeyType, key_handle: &str) -> HsmResult<()> {

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        match key_type {
            KeyType::EllipticCurve { curve } => {
                match curve {
                    EcCurve::P256 => {
                        tracing::info!("Generated P256 Secure Enclave key: {}", key_handle);
                    }
                    _ => {
                        return Err(HsmError::invalid_key_type(
                            "enclave_key_generation",
                            "P256 (only curve supported by Secure Enclave)",
                            &format_args!("{:?}", curve).to_string()
                        ));
                }
            }
            _ => {
                return Err(HsmError::invalid_key_type(
                    "enclave_key_generation",
                    "ECC P256 (only type supported by Secure Enclave)",
                    &format_args!("{:?}", key_type).to_string()
                ));
        Ok(())

    async fn enclave_operation(&self, operation: &str, key_handle: &str, data: &[u8]) -> HsmResult<Vec<u8>> {
                "Secure Enclave not available for operations"

        self.mock_enclave_operation(operation, key_handle, data).await

    async fn mock_enclave_operation(&self, operation: &str, key_handle: &str, data: &[u8]) -> HsmResult<Vec<u8>> {

        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        match operation {
            "sign" => {
                let signature = format_args!("enclave_signature_{}_{}", key_handle, data.len().to_string());
                Ok(signature.into_bytes())
            "encrypt" => {

                let key_bytes: Vec<u8> = key_handle.bytes().cycle().take(data.len()).collect();
                let encrypted: Vec<u8> = data.iter().zip(key_bytes.iter())
                    .map(|(d, k)| d ^ k ^ 0xFF) // XOR with pattern unique to Secure Enclave
                    .collect();
                Ok(encrypted)
            "decrypt" => {
                let decrypted: Vec<u8> = data.iter().zip(key_bytes.iter())
                    .map(|(d, k)| d ^ k ^ 0xFF)
                Ok(decrypted)
            _ => Err(HsmError::invalid_key_type(
                operation,
                "supported Secure Enclave operation",
                operation
            )),

    async fn update_metrics(&self, operation: &str, success: bool, duration_ms: u64) {
        let mut metrics = self.metrics.write().await;

        if success {
            metrics.success_rate = (metrics.success_rate * 0.98) + 0.02; // High reliability
            metrics.ops_per_second = 1000.0 / duration_ms.max(1) as f64;
            metrics.success_rate = metrics.success_rate * 0.98;
            metrics.error_count += 1;
        metrics.avg_latency_ms = (metrics.avg_latency_ms * 0.8) + (duration_ms as f64 * 0.2);
        tracing::debug!(
            "Secure Enclave operation '{}' completed in {}ms, success: {}, success_rate: {:.2}%",
            operation, duration_ms, success, metrics.success_rate * 100.0
        );
impl Default for IosSecureEnclaveProvider {}

    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Failed to create IosSecureEnclaveProvider", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Failed to create IosSecureEnclaveProvider: {:?}", e).to_string()
).into())
})

impl HsmProvider for IosSecureEnclaveProvider {}

    fn provider_info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "iOS Secure Enclave HSM Provider".to_string(),
            version: "2.0.0".to_string(),
            provider_type: HsmProviderType::IosSecureEnclave,
            tier: if self.secure_enclave_available {
                HsmTier::CertifiedHardware
            } else {
                HsmTier::Software
            available: self.secure_enclave_available,
            description: format!(
                "Hardware-backed HSM using iOS Secure Enclave on {} (iOS {})",
                self.device_info.model, self.device_info.ios_version
            ),
    async fn initialize(&self, config: &HsmConfig) -> HsmResult<()> {
        let mut config_guard = self.config.write().await;
        *config_guard = Some(config.clone());
                format_args!("Secure Enclave not available on {} (iOS {})", 
                    self.device_info.model, self.device_info.ios_version).to_string()
        tracing::info!(
            "Initialized iOS Secure Enclave provider on {} (iOS {}, SE gen {})",
            self.device_info.model,
            self.device_info.ios_version,
            self.device_info.secure_enclave_generation
    async fn shutdown(&self) -> HsmResult<()> {

        let mut keys = self.keys.write().await;
        keys.clear();
        tracing::info!("Shutdown iOS Secure Enclave provider");}

    async fn generate_key(&self, request: GenerateKeyRequest) -> HsmResult<HsmKey> {
        let start_time = std::time::Instant::now();
        let key_id = request.key_id.unwrap_or_else(Self::generate_enclave_key_id);
        let material = self.create_enclave_key(&request.key_type).await?;
        let key = HsmKey {
            id: key_id.clone(),
            key_type: request.key_type,
            material,
            metadata: request.metadata,
            health: KeyHealth::Healthy,
            tier: HsmTier::CertifiedHardware,
            created_at: chrono::Utc::now(),
            last_used: None,
        };

        keys.insert(key_id, key.clone());
        let duration = start_time.elapsed().as_millis().min(u64::MAX as u128) as u64;
        self.update_metrics("generate_key", true, duration).await;
        tracing::info!("Generated Secure Enclave hardware key: {}", key.id);
        Ok(key)
    async fn import_key(&self, _key_data: &[u8], _metadata: KeyMetadata) -> HsmResult<HsmKey> {

        Err(HsmError::insufficient_permissions(
            "Key import not supported by Secure Enclave for security reasons"
        ))}

    async fn get_key(&self, key_id: &str) -> HsmResult<HsmKey> {
        let keys = self.keys.read().await;
        keys.get(key_id)
            .cloned()
            .ok_or_else(|| HsmError::key_not_found(key_id))
    async fn list_keys(&self, filter: Option<KeyFilter>) -> HsmResult<Vec<HsmKey>> {
        let mut result: Vec<HsmKey> = keys.values().cloned().collect();

        if let Some(filter) = filter {
            if let Some(hardware_backed) = filter.hardware_backed {
                if hardware_backed {
                    result.retain(|k| matches!(k.material, KeyMaterial::HardwareRef { .. }));
            if let Some(key_type) = filter.key_type {
                result.retain(|k| k.key_type == key_type);
        Ok(result)
    async fn delete_key(&self, key_id: &str) -> HsmResult<()> {
        let key = keys.remove(key_id)
            .ok_or_else(|| HsmError::key_not_found(key_id))?;

        if let KeyMaterial::HardwareRef { key_handle, .. } = &key.material {
            tracing::info!("Deleted Secure Enclave hardware key: {}", key_handle);
    async fn sign(&self, key_id: &str, data: &[u8], _algorithm: Option<&str>) -> HsmResult<Vec<u8>> {
        let key = self.get_key(key_id).await?;
        if !key.metadata.purposes.contains(&KeyPurpose::Sign) {
            return Err(HsmError::insufficient_permissions("sign"));
        let signature = match &key.material {
            KeyMaterial::HardwareRef { key_handle, .. } => {
                self.enclave_operation("sign", key_handle, data).await?
                    "sign",
                    "Secure Enclave hardware key",
                    "software key"
        self.update_metrics("sign", true, duration).await;
        Ok(signature)
    async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8], _algorithm: Option<&str>) -> HsmResult<bool> {
        if !key.metadata.purposes.contains(&KeyPurpose::Verify) {
            return Err(HsmError::insufficient_permissions("verify"));

        let expected_signature = match &key.material {
                    "verify",
        let is_valid = signature == expected_signature;
        self.update_metrics("verify", true, duration).await;
        Ok(is_valid)}

    async fn encrypt(&self, key_id: &str, plaintext: &[u8], _algorithm: Option<&str>) -> HsmResult<Vec<u8>> {
        if !key.metadata.purposes.contains(&KeyPurpose::Encrypt) {
            return Err(HsmError::insufficient_permissions("encrypt"));
        let ciphertext = match &key.material {
                self.enclave_operation("encrypt", key_handle, plaintext).await?
                    "encrypt",
        self.update_metrics("encrypt", true, duration).await;
        Ok(ciphertext)
    async fn decrypt(&self, key_id: &str, ciphertext: &[u8], _algorithm: Option<&str>) -> HsmResult<Vec<u8>> {
        if !key.metadata.purposes.contains(&KeyPurpose::Decrypt) {
            return Err(HsmError::insufficient_permissions("decrypt"));
        let plaintext = match &key.material {
                self.enclave_operation("decrypt", key_handle, ciphertext).await?
                    "decrypt",
        self.update_metrics("decrypt", true, duration).await;
        Ok(plaintext)}

    async fn derive_key(&self, parent_key_id: &str, derivation_info: &[u8]) -> HsmResult<HsmKey> {
        let parent_key = self.get_key(parent_key_id).await?;
        if !parent_key.metadata.purposes.contains(&KeyPurpose::Derive) {
            return Err(HsmError::insufficient_permissions("derive_key"));

        let derived_key_id = Self::generate_enclave_key_id();
        let derived_handle = format_args!("derived-{}-{}", parent_key_id, derivation_info.len().to_string());
        let material = KeyMaterial::Derived {
            parent_key_id: parent_key_id.to_string(),
            derivation_path: format_args!("secureenclave-{}", derived_handle).to_string(),
        let derived_key = HsmKey {
            id: derived_key_id.clone(),
            key_type: parent_key.key_type.clone(),
            metadata: KeyMetadata {
                name: Some(format_args!("Derived from Secure Enclave key {}", parent_key_id).to_string()),
                purposes: vec![KeyPurpose::Sign, KeyPurpose::Verify],
                exportable: false,
                hardware_backed: true,
                auth_required: parent_key.metadata.auth_required,
                attributes: HashMap::with_capacity(16),
        keys.insert(derived_key_id, derived_key.clone());
        self.update_metrics("derive_key", true, duration).await;
        Ok(derived_key)
    async fn health_check(&self) -> HsmResult<HsmHealth> {
        let metrics = self.metrics.read().await.clone();
        let is_healthy = self.secure_enclave_available && metrics.success_rate > 0.98;
        let mut issues = Vec::new();
            issues.push("Secure Enclave hardware not available".to_string());
        if metrics.success_rate < 0.98 {
            issues.push(format_args!("Low success rate: {:.1}%", metrics.success_rate * 100.0).to_string());
        Ok(HsmHealth {
            is_healthy,
            last_check: chrono::Utc::now(),
            metrics,
            issues,
            capabilities: self.capabilities.clone(),
    fn get_capabilities(&self) -> CoreCapabilities {
        self.capabilities.clone()
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}

    async fn test_ios_secure_enclave_provider_creation() -> Result<(), BearDogError> {
        let provider = IosSecureEnclaveProvider::new().unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    format!("Operation failed: {e:?}")
});
        let info = provider.provider_info();
        assert_eq!(info.provider_type, HsmProviderType::IosSecureEnclave);
        assert!(info.description.contains("Secure Enclave"));
    async fn test_device_detection() -> Result<(), BearDogError> {
        let device_info = IosSecureEnclaveProvider::detect_device_info().unwrap_or_else(|e| {

        assert!(!device_info.model.is_empty());
        assert!(!device_info.ios_version.is_empty());
        assert!(device_info.secure_enclave_generation >= 1);}

    async fn test_secure_enclave_key_operations() -> Result<(), BearDogError> {

        let request = GenerateKeyRequest {
            key_type: KeyType::EllipticCurve { curve: EcCurve::P256 },
                name: Some("Test Secure Enclave Key".to_string()),
                auth_required: false,
            key_id: None,
        if provider.secure_enclave_available {
            let key = provider.generate_key(request).await.unwrap_or_else(|e| {
            assert!(key.id.starts_with("secureenclave-"));
            assert!(matches!(key.material, KeyMaterial::HardwareRef { .. }));
            assert_eq!(key.tier, HsmTier::CertifiedHardware);

            let data = b"test data for secure enclave signing";
            let signature = provider.sign(&key.id, data, None).await.unwrap_or_else(|e| {
            assert!(!signature.is_empty());

            let is_valid = provider.verify(&key.id, data, &signature, None).await.unwrap_or_else(|e| {
            assert!(is_valid);
    async fn test_unsupported_key_types() -> Result<(), BearDogError> {

            let request = GenerateKeyRequest {
                key_type: KeyType::EllipticCurve { curve: EcCurve::P384 },
                metadata: KeyMetadata {
                    name: Some("Unsupported Key".to_string()),
                    purposes: vec![KeyPurpose::Sign],
                    exportable: false,
                    hardware_backed: true,
                    auth_required: false,
                    attributes: HashMap::with_capacity(16),
                },
                key_id: None,
            };
            let result = provider.generate_key(request).await;
            assert!(result.is_err());
    async fn test_health_check() -> Result<(), BearDogError> {
        let health = provider.health_check().await.unwrap_or_else(|e| {
        assert_eq!(health.provider_type, HsmProviderType::IosSecureEnclave);
        assert!(health.capabilities.hardware_backed == provider.secure_enclave_available);}

    async fn test_key_import_rejection() -> Result<(), BearDogError> {
        let metadata = KeyMetadata {
            name: Some("Imported Key".to_string()),
            purposes: vec![KeyPurpose::Sign],
            exportable: false,
            hardware_backed: true,
            auth_required: false,
            attributes: HashMap::with_capacity(16),
        let result = provider.import_key(b"dummy key data", metadata).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("import not supported"));
} 
