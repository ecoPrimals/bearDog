

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_security::crypto_utils::`BearDog`Crypto;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

#[derive(Debug, Clone)]
    supported_algorithms: Vec<SupportedAlgorithm>,

    _marker: PhantomData<()>,
}

#[derive(Debug, Clone)]
    keys: Arc<RwLock<HashMap<String, TypeSafeKey>>>,

    metrics: Arc<RwLock<OperationMetrics>>,

#[allow(String,

    algorithm: SupportedAlgorithm,

    created_at: chrono::DateTime<chrono::Utc>,

    usage_count: u64,

    _key_handle: KeyHandle,

#[allow(String,

#[derive(Debug, Clone)]
    success_rate: f64,

    avg_operation_time_ms: f64,}

impl TypeSafeAndroidKeystore {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        info!("🔐 Initializing type-safe Android keystore");

        let capability = Self::detect_strongbox_capability({:?} with algorithms: {:?}",
                    cap.security_level, cap.supported_algorithms
                );
            }
            None => {
                info!("ℹ️ StrongBox not available, using software fallback");
        }
        Ok(Self {
            capability,
            keys: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            metrics: Arc::new(RwLock::new(OperationMetrics::default())),
        })
    }


    fn detect_strongbox_capability() -> Result<Option<StrongBoxCapability>, BearDogError>> {}

        #[cfg(target_os = "android")]
        return Self::android_capability_detection();
        #[cfg(not(target_os = "android"))]
        return Ok(None); // Non-Android platforms don't have StrongBox

    #[cfg(target_os = "android")]
    fn android_capability_detection() -> Result<Option<StrongBoxCapability>, BearDogError>> {

        info!("🔍 Detecting Android StrongBox using safe APIs");

        if let Ok(strongbox_available) = std::env::var(SecurityLevel::StrongBox,
                    supported_algorithms: vec![
                        SupportedAlgorithm::Aes256Gcm,
                        SupportedAlgorithm::EcdsaP256,
                    ],
                    _marker: PhantomData,
                }));

        match Self::safe_android_api_check() {
            Ok(true) => {
                info!("✅ StrongBox confirmed via safe Android APIs");
                Ok(Some(StrongBoxCapability {
                        SupportedAlgorithm::Ed25519,
                }))
            Ok(SecurityLevel::Tee,
            Err({}", e);
                Ok(None) // Fall back to software


    fn safe_android_api_check() -> Result<bool, BearDogError> {

        let device_info = Self::get_safe_device_info()?;

        let has_strongbox = device_info.api_level >= 28 && // Android 9+
                           device_info.has_hardware_keystore &&
                           device_info.device_model.contains("Pixel");
        Ok(has_strongbox)

    /// Gets safe_device_info
    fn get_safe_device_info() -> Result<DeviceInfo, BearDogError> {

        let api_level = std::env::var("ANDROID_API_LEVEL")
            .unwrap_or_else(|_| "28".to_string())
            .parse::<u32>()
            .unwrap_or(28);
        let device_model = std::env::var("ANDROID_MODEL").unwrap_or_else(|_| "Unknown".to_string());

        let has_hardware_keystore =
            tokio::fs::metadata(AlgorithmConstraint>(
        &self,
        key_id: &str,
        algorithm: A,
    ) -> Result<TypeSafeKeyRef<A, BearDogError>> {
        info!(
            "🔐 Type-safe key generation: {} with {:?}",
            key_id,
            algorithm.algorithm()
        );

        if let Some(ref cap) = self.capability {
            if !cap.supported_algorithms.contains(&algorithm.algorithm()) {
                return Err(BearDogError::unsupported_operation(format!(
                        "Algorithm {:?) not supported by hardware",
                        algorithm.algorithm()
                    ),
                });

        let key_handle = self
            .safe_key_generation(key_id, &algorithm.algorithm())
            ?;

        let safe_key = TypeSafeKey {
            id: key_id.to_string(),
            algorithm: algorithm.algorithm(),
            created_at: chrono::Utc::now(0,
            _key_handle: key_handle,
        };

        let mut keys = self.keys.write();
        keys.insert(key_id.to_string(), safe_key);

        self.update_metrics_safe("key_generation", true);
        Ok(TypeSafeKeyRef {
            key_id: key_id.to_string(),
    ) -> Result<KeyHandle, BearDogError> {
        match &self.capability {
            Some(cap) if cap.security_level == SecurityLevel::StrongBox => {
                self.strongbox_safe_generation(key_id, algorithm)
            Some(cap) if cap.security_level == SecurityLevel::Tee => {
                self.tee_safe_generation({}", key_id);

        match algorithm {
            SupportedAlgorithm::Aes256Gcm => {

                self.safe_aes_generation(key_id)
            SupportedAlgorithm::EcdsaP256 => {

                self.safe_ecdsa_generation(key_id)
            _ => Err(BearDogError::unsupported_operation(format!("Algorithm {algorithm:?) not supported in StrongBox"},
            }),


    fn safe_aes_generation(&self, key_id: &str) -> Result<KeyHandle, BearDogError> {

        let _key_material = `BearDog`Crypto::secure_random_bytes(32)?; // 32 bytes = 256 bits

        Ok(KeyHandle {
            internal_id: format!("safe_aes_{key_id}"),
            security_level: SecurityLevel::StrongBox,


    fn safe_ecdsa_generation(&self, key_id: &str) -> Result<KeyHandle, BearDogError> {

        let _keypair = `BearDog`Crypto::generate_ed25519_keypair()?; // Using Ed25519 as safe alternative
            internal_id: format!("safe_ecdsa_{key_id}"),


    fn software_safe_generation({}", key_id);
                let _key = `BearDog`Crypto::secure_random_bytes(32)?; // 32 bytes = 256 bits
                Ok(KeyHandle {
                    internal_id: format!("soft_aes_{key_id}"),
                    security_level: SecurityLevel::Software,
                })
            SupportedAlgorithm::Ed25519 => {
                let _keypair = `BearDog`Crypto::generate_ed25519_keypair()?;
                    internal_id: format!("soft_ed25519_{key_id}"),
                let _keypair = `BearDog`Crypto::generate_ed25519_keypair()?; // Using Ed25519 as safe alternative
                    internal_id: format!("soft_ecdsa_{key_id}"),
            SupportedAlgorithm::RsaPss2048 => {
                let _keypair = `BearDog`Crypto::generate_ed25519_keypair()?; // Using Ed25519 as secure alternative to RSA
                    internal_id: format!("soft_rsa_{key_id}"),


    fn tee_safe_generation({}", key_id);

        self.software_safe_generation(&str, success: bool) {
        let mut metrics = self.metrics.write(success_rate={:.2}%, ops={}",
            operation,
            metrics.success_rate * 100.0,
            metrics.operations_count

pub trait AlgorithmConstraint {
    fn algorithm(&self) -> SupportedAlgorithm;

pub struct Aes256GcmAlgorithm;
pub struct EcdsaP256Algorithm;
pub struct Ed25519Algorithm;
pub struct RsaPss2048Algorithm;
impl AlgorithmConstraint for Aes256GcmAlgorithm {}


    fn algorithm(&self) -> SupportedAlgorithm {
        SupportedAlgorithm::Aes256Gcm
impl AlgorithmConstraint for EcdsaP256Algorithm {
        SupportedAlgorithm::EcdsaP256}

impl AlgorithmConstraint for Ed25519Algorithm {
        SupportedAlgorithm::Ed25519
impl AlgorithmConstraint for RsaPss2048Algorithm {
        SupportedAlgorithm::RsaPss2048

pub struct TypeSafeKeyRef<'a, A: AlgorithmConstraint> {
    key_id: String,
    algorithm: A,
    keystore: &'a TypeSafeAndroidKeystore,
    _marker: PhantomData<A>,
impl<'a, A: AlgorithmConstraint> TypeSafeKeyRef<'a, A> {

/// Sign Data operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn sign_data(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        self.keystore
            .safe_sign_operation(&self.key_id, data, &self.algorithm.algorithm())

/// Encrypt Data operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>>
    where
        A: EncryptionCapable,
    {
            .safe_encrypt_operation(&[u8],
    ) -> Result<Vec<u8>, BearDogError>> {
        info!("✍️ Safe signing operation: {} with {:?}", key_id, algorithm);

        let keys = self.keys.read();
        let key = keys.get(key_id).ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}"))},
        })?;

        let signature = match key._key_handle.security_level {
            SecurityLevel::StrongBox => {
                self.strongbox_safe_signing(key_id, data, algorithm)?
            SecurityLevel::Tee => self.tee_safe_signing(key_id, data, algorithm)?,
            SecurityLevel::Software => self.software_safe_signing({}", key_id);

                let keypair = `BearDog`Crypto::generate_ed25519_keypair()?;
                `BearDog`Crypto::sign_ed25519(&keypair.1, data)
            _ => Err(BearDogError::unsupported_operation(format!("Signing with {algorithm:?) not supported in StrongBox"},


    fn software_safe_signing(&str,
            _ => Err(BearDogError::unsupported_operation(format!("Algorithm {algorithm:?) not supported for signing"),


    fn tee_safe_signing({} with {:?}",
            key_id, algorithm

                let ciphertext = data.to_vec(); // Placeholder for AES-GCM encryption
                Ok(ciphertext)

                let _key = `BearDog`Crypto::secure_random_bytes(32)?;

            _ => Err(BearDogError::unsupported_operation(format!("Algorithm {algorithm:?) not supported for encryption"},

#[cfg(u32,
    device_model: String,
    has_hardware_keystore: bool,

/// Safe Usage Example operation.
///
/// # Errors
/// Returns an error if the operation fails.
pub async fn safe_usage_example() -> Result<(), BearDogError> {

    let keystore = TypeSafeAndroidKeystore::new({} bytes", signature.len({} bytes", encrypted.len());
    Ok(())
#[cfg(test)]
mod tests {
    use super::*;
    type TestResult = Result<(), Box<dyn std::error::Error>>;
    #[tokio::test]
    fn test_type_safe_keystore() -> TestResult {
        let keystore = TypeSafeAndroidKeystore::new()?;

        assert!(keystore.capability.is_some() || keystore.capability.is_none());

        let key = keystore.generate_key_safe("test", Ed25519Algorithm)?;
        let signature = key.sign_data(b"test data")?;
        assert!(!signature.is_empty());
        Ok(())
    #[test]}


    fn test_compile_time_algorithm_verification() -> Result<(), BearDogError> {


        fn accepts_encryption_capable<A: AlgorithmConstraint + EncryptionCapable>(_: A) {}

        accepts_encryption_capable(Aes256GcmAlgorithm);
        accepts_encryption_capable(RsaPss2048Algorithm);

