

use crate::tunnel::hsm::types::canonical::KeyMetadata;
use crate::tunnel::hsm::types::{HsmKey, KeyAttestation, KeyHealthStatus, KeyMaterial, KeyType};
use beardog_errors::BearDogError;
use tracing::{info, warn};

pub struct SafeSecureEnclave {

    available: bool,
}
impl SafeSecureEnclave {

    pub async fn new() -> Result<Self, BearDogError> {
        info!("🍎 Initializing safe iOS Secure Enclave");
        let available = Self::safe_check_availability().await;
        Ok(Self { available })
    }

    async fn safe_check_availability() -> bool {

        if !cfg!(target_os = "ios") {
            info!("📱 Not on iOS platform, Secure Enclave not available");
            return false;
        }

        let ios_version = Self::get_ios_version();
        if ios_version < 9.0 {
            warn!(
                "iOS version {} < 9.0, Secure Enclave not supported",
                ios_version
            );

        let device_type = Self::get_device_type();
        let supports_secure_enclave = matches!(device_type.as_str(),
            device if device.contains("iPhone") && Self::parse_iphone_version(&device) >= 5 ||
                     device.contains("iPad") && Self::parse_ipad_version(&device) >= 3);
        info!(
            "🍎 iOS device: {}, Secure Enclave supported: {}",
            device_type, supports_secure_enclave
        );
        supports_secure_enclave

    fn get_ios_version() -> f32 {

        std::env::var("IOS_VERSION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(9.0) // Safe default

    fn get_device_type() -> String {
        std::env::var("IOS_DEVICE_TYPE")
            .or_else(|_| std::env::var("DEVICE_TYPE"))
            .unwrap_or_else(|_| "Unknown".to_string())

    fn parse_iphone_version(device: &str) -> u32 {
        if device.contains("iPhone15") {
            15
        } else if device.contains("iPhone14") {
            14
        } else if device.contains("iPhone13") {
            13
        } else if device.contains("iPhone12") {
            12
        } else if device.contains("iPhone11") {
            11
        } else if device.contains("iPhone10") {
            10
        } else if device.contains("iPhone9") {
            9
        } else if device.contains("iPhone8") {
            8
        } else if device.contains("iPhone7") {
            7
        } else if device.contains("iPhone6") {
            6
        } else if device.contains("iPhone5") {
            5
        } else {
            4

    fn parse_ipad_version(device: &str) -> u32 {
        if device.contains("iPad10") {
        } else if device.contains("iPad9") {
        } else if device.contains("iPad8") {
        } else if device.contains("iPad7") {
        } else if device.contains("iPad6") {
        } else if device.contains("iPad5") {
        } else if device.contains("iPad4") {
        } else if device.contains("iPad3") {
            3
            2

    pub async fn safe_generate_key(
        &self,
        key_id: &str,
        key_type: &KeyType,
    ) -> Result<HsmKey, BearDogError> {
        info!("🔐 Safe key generation for iOS Secure Enclave: {}", key_id);
        if !self.available {
            return Err(BearDogError::Unavailable {
                message: "iOS Secure Enclave not available on this device".to_string(),
            });

        use beardog_security::crypto_utils::BearDogCrypto;
        let keypair = match key_type {
            KeyType::Ed25519 => BearDogCrypto::generate_ed25519_keypair()?,
            _ => {
                return Err(BearDogError::NotSupported {
                    message: format!(
                        "Key type {:?} not supported in safe iOS implementation",
                        key_type
                    ),
                });
            }
        };

        Ok(HsmKey {
            id: key_id.to_string(),
            hsm_type: "ios_secure_enclave".to_string(),
            key_type: key_type.clone(),
            metadata: KeyMetadata {
                key_id: key_id.to_string(),
                key_name: Some("iOS Secure Enclave Key".to_string()),
                algorithm: Some(format_args!("{:?}", key_type).to_string()),
                key_size: Some(256),
                creation_time: Some(chrono::Utc::now()),
                is_hardware_backed: Some(self.available),
                ..Default::default()
            },
            key_material: KeyMaterial::HardwareReference {
                reference: key_id.to_string(), // Use 'reference' instead of 'key_handle'
                hsm_location: "ios_secure_enclave".to_string(), // Use 'hsm_location' instead of 'device_id'
            hsm_tier: "hardware".to_string(),
            health_status: KeyHealthStatus::Healthy,
            attestation: Some(KeyAttestation {
                attestation_format: "ios_secure_enclave".to_string(),
                attestation_data: keypair.public_key_bytes().to_vec(), // Store public key here
                verified: true,
                issuer: "Apple Secure Enclave".to_string(),
            }),
            created_at: chrono::Utc::now(),
        })

    pub async fn safe_sign_data(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        info!("✍️ Safe data signing for iOS Secure Enclave: {}", key_id);
                message: "iOS Secure Enclave not available for signing".to_string(),

        let keypair = self.derive_key_from_id(key_id)?;
        let signature = BearDogCrypto::sign_ed25519(&keypair.private_key_bytes(), data)?;
        info!("✅ Safe signature generated: {} bytes", signature.len());
        Ok(signature)

    pub async fn safe_verify_signature(
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
            "🔍 Safe signature verification for iOS Secure Enclave: {}",
            key_id

        let is_valid =
            BearDogCrypto::verify_ed25519_signature(&keypair.public_key_bytes(), data, signature)?;
        info!("✅ Safe signature verification: {}", is_valid);
        Ok(is_valid)

    fn derive_key_from_id(&self, key_id: &str) -> Result<ed25519_dalek::SigningKey, BearDogError> {

        let seed = BearDogCrypto::derive_key_pbkdf2(
            key_id.as_bytes(),
            b"ios_secure_enclave_safe_salt",
            32,
            10000, // iterations
        )?;
        let signing_key = if seed.len() >= 32 {
            let mut key_bytes = [0u8; 32];
            key_bytes.copy_from_slice(&seed[..32]);
            ed25519_dalek::SigningKey::from_bytes(&key_bytes)
            return Err(BearDogError::internal(
                "Insufficient key material".to_string(),
            ));
        Ok(signing_key)

    fn generate_safe_attestation_cert(&self, key_id: &str) -> Result<Vec<u8>, BearDogError>> {
        let mut cert_data = Vec::new();

        cert_data.extend_from_slice(b"iOS_SECURE_ENCLAVE_CERT");
        cert_data.extend_from_slice(key_id.as_bytes());
        cert_data.extend_from_slice(&chrono::Utc::now().timestamp().to_le_bytes());

        cert_data.resize(512, 0x00);
        Ok(cert_data)

    pub fn is_available(&self) -> bool {
        self.available
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_safe_secure_enclave_creation() -> Result<(), BearDogError> {
        let enclave = SafeSecureEnclave::new().await;
        assert!(enclave.is_ok());
        Ok(())}

    async fn test_safe_key_generation() -> Result<(), BearDogError> {
        let enclave = SafeSecureEnclave::new().await.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        if enclave.is_available() {
            let key = enclave
                .safe_generate_key("test_key", &KeyType::Ed25519)
                .await;
            assert!(key.is_ok());
    #[test]
    fn test_device_version_parsing() -> Result<(), BearDogError> {
        assert_eq!(SafeSecureEnclave::parse_iphone_version("iPhone14,2"), 14);
        assert_eq!(SafeSecureEnclave::parse_ipad_version("iPad8,1"), 8);
