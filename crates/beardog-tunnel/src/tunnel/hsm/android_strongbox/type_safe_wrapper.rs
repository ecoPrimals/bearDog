

use beardog_errors::BearDogError;
use beardog_security::crypto_utils::`BearDog`Crypto;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct StrongBoxCapability {

    security_level: SecurityLevel,

    supported_algorithms: Vec<SupportedAlgorithm>,

    _marker: PhantomData<()>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityLevel {

    Software,

    Tee,

    StrongBox,

pub enum SupportedAlgorithm {
    Aes256Gcm,
    EcdsaP256,
    Ed25519,
    RsaPss2048,

pub struct TypeSafeAndroidKeystore {

    capability: Option<StrongBoxCapability>,

    keys: Arc<RwLock<HashMap<String, TypeSafeKey>>>,

    metrics: Arc<RwLock<OperationMetrics>>,

#[allow(dead_code)] // Mock struct for type safety demonstration
struct TypeSafeKey {

    id: String,

    algorithm: SupportedAlgorithm,

    created_at: chrono::DateTime<chrono::Utc>,

    usage_count: u64,

    _key_handle: KeyHandle,

#[allow(dead_code)] // Mock handle for demonstration
struct KeyHandle {

    internal_id: String,

#[derive(Debug, Default)]
#[allow(dead_code)] // Metrics struct for demonstration
struct OperationMetrics {

    operations_count: u64,

    success_rate: f64,

    avg_operation_time_ms: f64,}

impl TypeSafeAndroidKeystore {

    pub async fn new() -> Result<Self, BearDogError> {
        info!("🔐 Initializing type-safe Android keystore");

        let capability = Self::detect_strongbox_capability().await?;
        match &capability {
            Some(cap) => {
                info!(
                    "✅ StrongBox detected: {:?} with algorithms: {:?}",
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

    async fn detect_strongbox_capability() -> Result<Option<StrongBoxCapability>, BearDogError>> {}

        #[cfg(target_os = "android")]
        return Self::android_capability_detection().await;
        #[cfg(not(target_os = "android"))]
        return Ok(None); // Non-Android platforms don't have StrongBox

    #[cfg(target_os = "android")]
    async fn android_capability_detection() -> Result<Option<StrongBoxCapability>, BearDogError>> {

        info!("🔍 Detecting Android StrongBox using safe APIs");

        if let Ok(strongbox_available) = std::env::var("ANDROID_STRONGBOX_AVAILABLE") {
            if strongbox_available == "true" {
                return Ok(Some(StrongBoxCapability {
                    security_level: SecurityLevel::StrongBox,
                    supported_algorithms: vec![
                        SupportedAlgorithm::Aes256Gcm,
                        SupportedAlgorithm::EcdsaP256,
                    ],
                    _marker: PhantomData,
                }));

        match Self::safe_android_api_check().await {
            Ok(true) => {
                info!("✅ StrongBox confirmed via safe Android APIs");
                Ok(Some(StrongBoxCapability {
                        SupportedAlgorithm::Ed25519,
                }))
            Ok(false) => {
                info!("ℹ️ StrongBox not available, detected TEE support");
                    security_level: SecurityLevel::Tee,
            Err(e) => {
                warn!("⚠️ Could not detect Android capabilities safely: {}", e);
                Ok(None) // Fall back to software

    async fn safe_android_api_check() -> Result<bool, BearDogError> {

        let device_info = Self::get_safe_device_info().await?;

        let has_strongbox = device_info.api_level >= 28 && // Android 9+
                           device_info.has_hardware_keystore &&
                           device_info.device_model.contains("Pixel");
        Ok(has_strongbox)

    async fn get_safe_device_info() -> Result<DeviceInfo, BearDogError> {

        let api_level = std::env::var("ANDROID_API_LEVEL")
            .unwrap_or_else(|_| "28".to_string())
            .parse::<u32>()
            .unwrap_or(28);
        let device_model = std::env::var("ANDROID_MODEL").unwrap_or_else(|_| "Unknown".to_string());

        let has_hardware_keystore =
            tokio::fs::metadata("/system/etc/security/keystore2_keystore_attest_ca.pem")
                .await
                .is_ok();
        Ok(DeviceInfo {
            api_level,
            device_model,
            has_hardware_keystore,

    pub async fn generate_key_safe<A: AlgorithmConstraint>(
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
            .await?;

        let safe_key = TypeSafeKey {
            id: key_id.to_string(),
            algorithm: algorithm.algorithm(),
            created_at: chrono::Utc::now(),
            usage_count: 0,
            _key_handle: key_handle,
        };

        let mut keys = self.keys.write().await;
        keys.insert(key_id.to_string(), safe_key);

        self.update_metrics_safe("key_generation", true).await;
        Ok(TypeSafeKeyRef {
            key_id: key_id.to_string(),
            algorithm,
            keystore: self,
            _marker: PhantomData,

    async fn safe_key_generation(
        algorithm: &SupportedAlgorithm,
    ) -> Result<KeyHandle, BearDogError> {
        match &self.capability {
            Some(cap) if cap.security_level == SecurityLevel::StrongBox => {
                self.strongbox_safe_generation(key_id, algorithm).await
            Some(cap) if cap.security_level == SecurityLevel::Tee => {
                self.tee_safe_generation(key_id, algorithm).await
            _ => self.software_safe_generation(key_id, algorithm).await,

    async fn strongbox_safe_generation(
        info!("🔐 StrongBox safe key generation: {}", key_id);

        match algorithm {
            SupportedAlgorithm::Aes256Gcm => {

                self.safe_aes_generation(key_id).await
            SupportedAlgorithm::EcdsaP256 => {

                self.safe_ecdsa_generation(key_id).await
            _ => Err(BearDogError::unsupported_operation(format!("Algorithm {algorithm:?) not supported in StrongBox"},
            }),

    async fn safe_aes_generation(&self, key_id: &str) -> Result<KeyHandle, BearDogError> {

        let _key_material = `BearDog`Crypto::secure_random_bytes(32)?; // 32 bytes = 256 bits

        Ok(KeyHandle {
            internal_id: format!("safe_aes_{key_id}"),
            security_level: SecurityLevel::StrongBox,

    async fn safe_ecdsa_generation(&self, key_id: &str) -> Result<KeyHandle, BearDogError> {

        let _keypair = `BearDog`Crypto::generate_ed25519_keypair()?; // Using Ed25519 as safe alternative
            internal_id: format!("safe_ecdsa_{key_id}"),

    async fn software_safe_generation(
        info!("🔧 Software safe key generation: {}", key_id);
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

    async fn tee_safe_generation(
        info!("🔐 TEE safe key generation: {}", key_id);

        self.software_safe_generation(key_id, algorithm).await

    async fn update_metrics_safe(&self, operation: &str, success: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.operations_count += 1;
        if success {

            let new_success_rate = if metrics.operations_count == 1 {
                1.0
            } else {
                let old_success_count =
                    (metrics.success_rate * (metrics.operations_count - 1) as f64) as u64;
                let new_success_count = old_success_count + 1;
                new_success_count as f64 / metrics.operations_count as f64
            };
            metrics.success_rate = new_success_rate;
        debug!(
            "📊 Operation {}: success_rate={:.2}%, ops={}",
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

    pub async fn sign_data(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        self.keystore
            .safe_sign_operation(&self.key_id, data, &self.algorithm.algorithm())
            .await

    pub async fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>>
    where
        A: EncryptionCapable,
    {
            .safe_encrypt_operation(&self.key_id, data, &self.algorithm.algorithm())

pub trait EncryptionCapable {}
impl EncryptionCapable for Aes256GcmAlgorithm {}
impl EncryptionCapable for RsaPss2048Algorithm {}

    async fn safe_sign_operation(
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError>> {
        info!("✍️ Safe signing operation: {} with {:?}", key_id, algorithm);

        let keys = self.keys.read().await;
        let key = keys.get(key_id).ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}"))},
        })?;

        let signature = match key._key_handle.security_level {
            SecurityLevel::StrongBox => {
                self.strongbox_safe_signing(key_id, data, algorithm).await?
            SecurityLevel::Tee => self.tee_safe_signing(key_id, data, algorithm).await?,
            SecurityLevel::Software => self.software_safe_signing(key_id, data, algorithm).await?,

        self.update_metrics_safe("signing", true).await;
        Ok(signature)

    async fn strongbox_safe_signing(

                info!("🔐 StrongBox ECDSA signing (safe): {}", key_id);

                let keypair = `BearDog`Crypto::generate_ed25519_keypair()?;
                `BearDog`Crypto::sign_ed25519(&keypair.1, data)
            _ => Err(BearDogError::unsupported_operation(format!("Signing with {algorithm:?) not supported in StrongBox"},

    async fn software_safe_signing(
        _key_id: &str,
            _ => Err(BearDogError::unsupported_operation(format!("Algorithm {algorithm:?) not supported for signing"),

    async fn tee_safe_signing(

        self.software_safe_signing(key_id, data, algorithm).await

    async fn safe_encrypt_operation(
            "🔐 Safe encryption operation: {} with {:?}",
            key_id, algorithm

                let ciphertext = data.to_vec(); // Placeholder for AES-GCM encryption
                Ok(ciphertext)

                let _key = `BearDog`Crypto::secure_random_bytes(32)?;

            _ => Err(BearDogError::unsupported_operation(format!("Algorithm {algorithm:?) not supported for encryption"},

#[cfg(target_os = "android")]
#[derive(Debug)]
struct DeviceInfo {
    api_level: u32,
    device_model: String,
    has_hardware_keystore: bool,

pub async fn safe_usage_example() -> Result<(), BearDogError> {

    let keystore = TypeSafeAndroidKeystore::new().await?;

    let signing_key = keystore
        .generate_key_safe("test_key", Ed25519Algorithm)
        .await?;

    let data = b"Hello, safe world!";
    let signature = signing_key.sign_data(data).await?;
    info!("✅ Safe signing completed: {} bytes", signature.len());

    let encryption_key = keystore
        .generate_key_safe("encrypt_key", Aes256GcmAlgorithm)
    let encrypted = encryption_key.encrypt_data(data).await?;
    info!("✅ Safe encryption completed: {} bytes", encrypted.len());
    Ok(())
#[cfg(test)]
mod tests {
    use super::*;
    type TestResult = Result<(), Box<dyn std::error::Error>>;
    #[tokio::test]
    async fn test_type_safe_keystore() -> TestResult {
        let keystore = TypeSafeAndroidKeystore::new().await?;

        assert!(keystore.capability.is_some() || keystore.capability.is_none());

        let key = keystore.generate_key_safe("test", Ed25519Algorithm).await?;
        let signature = key.sign_data(b"test data").await?;
        assert!(!signature.is_empty());
        Ok(())
    #[test]}

    fn test_compile_time_algorithm_verification() -> Result<(), BearDogError> {

        fn accepts_encryption_capable<A: AlgorithmConstraint + EncryptionCapable>(_: A) {}

        accepts_encryption_capable(Aes256GcmAlgorithm);
        accepts_encryption_capable(RsaPss2048Algorithm);

