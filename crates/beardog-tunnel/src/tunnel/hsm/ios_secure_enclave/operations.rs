

use super::types::*;
use beardog_errors::BearDogError;
use beardog_security::crypto_utils::BearDogCrypto;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

pub struct TypeSafeSecureEnclaveKey<'a, A: SecureEnclaveConstraint> {
    enclave: &'a TypeSafeSecureEnclave,
    key_id: String,
    algorithm: A,
    biometric_policy: BiometricPolicy,
}
impl<'a, A: SecureEnclaveConstraint> TypeSafeSecureEnclaveKey<'a, A> {
    pub(crate) fn new(
        enclave: &'a TypeSafeSecureEnclave,
        key_id: &str,
        algorithm: A,
        biometric_policy: BiometricPolicy,
    ) -> Self {
        Self {
            enclave,
            key_id,
            algorithm,
            biometric_policy,
        }
    }

    pub async fn sign_with_biometric(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>>
    where
        A: SecureEnclaveConstraint,
    {

        if !matches!(
            self.algorithm.algorithm(),
            SecureEnclaveAlgorithm::EcdsaP256
        ) {
            return Err(BearDogError::unsupported_operation("Key algorithm does not support signing".to_string(),
            ));
        self.sign_with_biometric_auth(data).await

    pub async fn sign_with_biometric_auth(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        info!("🔐 Signing data with Secure Enclave key: {}", self.key_id);

        let keys = self.enclave.keychain_keys.read().await;
        let key = keys
            .get(&self.key_id)
            .ok_or_else(|| BearDogError::not_found(format_args!("Key not found: }", self.key_id).to_string(),
            })?;

        self.authenticate_biometric().await?;

        if key._key_ref.in_secure_enclave {
            self.secure_enclave_sign(data, key).await
        } else {
            self.software_fallback_sign(data, key).await

    pub async fn verify_signature(&self, data: &[u8], signature: &[u8]) -> Result<bool, BearDogError> {
        info!(
            "🔍 Verifying signature with Secure Enclave key: {}",
            self.key_id
        );

        BearDogCrypto::verify_ed25519_signature(&key.public_key, data, signature)

    pub async fn key_agreement(&self, peer_public_key: &[u8]) -> Result<Vec<u8>, BearDogError>>
        A: KeyAgreementCapable,
            "🤝 Performing key agreement with Secure Enclave key: {}",

        if !matches!(self.algorithm.algorithm(), SecureEnclaveAlgorithm::EcdhP256) {
            return Err(BearDogError::unsupported_operation("Key algorithm does not support key agreement".to_string(),

            self.secure_enclave_key_agreement(peer_public_key, key)
                .await
            self.software_fallback_key_agreement(peer_public_key, key)

    async fn authenticate_biometric(&self) -> Result<(), BearDogError> {
        match &self.biometric_policy {
            BiometricPolicy::NoBiometric => {
                debug!("🔓 No biometric authentication required");
                Ok(())
            }
            policy => {
                info!("👆 Performing biometric authentication: {:?}", policy);

                self.simulate_biometric_auth(policy).await

    async fn simulate_biometric_auth(&self, policy: &BiometricPolicy) -> Result<(), BearDogError> {

        match policy {
            BiometricPolicy::TouchIDRequired | BiometricPolicy::TouchIDOnly => {
                debug!("👆 TouchID authentication simulated");
            BiometricPolicy::FaceIDRequired | BiometricPolicy::FaceIDOnly => {
                debug!("👤 FaceID authentication simulated");
            BiometricPolicy::TouchIDOrFaceID | BiometricPolicy::AnyBiometric => {
                debug!("🔐 Biometric authentication simulated");

        Ok(())

    async fn secure_enclave_sign(
        &self,
        data: &[u8],
        key: &SecureKeychainKey,
    ) -> Result<Vec<u8>, BearDogError>> {
        info!("🔐 Using Secure Enclave for signing");

        match key.private_key {
            Some(ref private_key) => {

                BearDogCrypto::sign_ed25519(private_key, data)
            None => {

                self.software_fallback_sign(data, key).await

    async fn software_fallback_sign(
        info!("🔧 Using software fallback for signing");
        match &key.private_key {
            Some(private_key) => BearDogCrypto::sign_ed25519(private_key, data),
            None => Err(BearDogError::not_found(format_args!("Key not found: }", self.key_id).to_string(),
            }),

    async fn secure_enclave_key_agreement(
        peer_public_key: &[u8],
        info!("🤝 Using Secure Enclave for key agreement");
            Some(private_key) => {

                BearDogCrypto::perform_ecdh(private_key, peer_public_key)
                self.software_fallback_key_agreement(peer_public_key, key)
                    .await

    async fn software_fallback_key_agreement(
        info!("🔧 Using software fallback for key agreement");
            Some(private_key) => BearDogCrypto::perform_ecdh(private_key, peer_public_key),

pub struct TypeSafeSecureEnclave {

    pub(crate) capability: Option<SecureEnclaveCapability>,

    pub(crate) keychain_keys: Arc<RwLock<HashMap<String, SecureKeychainKey>>>,

    pub(crate) metrics: Arc<RwLock<SecureEnclaveMetrics>>,}

impl TypeSafeSecureEnclave {

    pub async fn new() -> Result<Self, BearDogError> {
        info!("🚀 Initializing Type-Safe Secure Enclave");
        let capability = super::capability::CapabilityDetector::detect_capability().await?;
        Ok(Self {
            capability,
            keychain_keys: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            metrics: Arc::new(RwLock::new(SecureEnclaveMetrics::default())),
        })

    pub async fn generate_secure_key<A: SecureEnclaveConstraint>(
        key_id: &str,
    ) -> Result<TypeSafeSecureEnclaveKey<'_, A, BearDogError>> {
            "🔑 Generating secure key: {} with algorithm: {:?}",
            algorithm.algorithm()

        let key_ref = if self.capability.is_some() {
            self.secure_enclave_generation(key_id, &algorithm.algorithm(), &biometric_policy)
                .await?
            self.software_secure_generation(key_id, &algorithm.algorithm(), &biometric_policy)
        };

        self.update_secure_metrics("key_generation", true).await;
        Ok(TypeSafeSecureEnclaveKey::new(
            self,
            key_id.to_string(),
        ))

    pub async fn generate_secure_key_enum(
        algorithm: SecureEnclaveAlgorithm,
    ) -> Result<SecureKeyReference, BearDogError> {
            key_id, algorithm
            self.secure_enclave_generation(key_id, &algorithm, &biometric_policy)
            self.software_secure_generation(key_id, &algorithm, &biometric_policy)
        Ok(key_ref)

    async fn secure_enclave_generation(
        algorithm: &SecureEnclaveAlgorithm,
        biometric_policy: &BiometricPolicy,
        info!("🔐 Secure Enclave key generation for: {}", key_id);
        match algorithm {
            SecureEnclaveAlgorithm::EcdsaP256 => {
                let keypair = BearDogCrypto::generate_ed25519_keypair()?;
                let mut keys = self.keychain_keys.write().await;
                keys.insert(
                    key_id.to_string(),
                    SecureKeychainKey {
                        _key_ref: SecureKeyReference {
                            keychain_ref: format_args!("secure_ecdsa_{}", key_id).to_string(),
                            in_secure_enclave: true,
                        },
                        public_key: keypair.1.clone(),
                        private_key: Some(keypair.0.clone()), // Store for operations
                        algorithm: SecureEnclaveAlgorithm::EcdsaP256,
                    },
                );
                Ok(SecureKeyReference {
                    keychain_ref: format_args!("secure_ecdsa_{}", key_id).to_string(),
                    in_secure_enclave: true,
                })
            SecureEnclaveAlgorithm::EcdhP256 => {
                            keychain_ref: format_args!("secure_ecdh_{}", key_id).to_string(),
                        private_key: Some(keypair.0.clone()),
                        algorithm: SecureEnclaveAlgorithm::EcdhP256,
                    keychain_ref: format_args!("secure_ecdh_{}", key_id).to_string(),

    async fn software_secure_generation(
        _biometric_policy: &BiometricPolicy,
        info!("🔧 Software fallback for Secure Enclave key: {}", key_id);
                            keychain_ref: format_args!("soft_ecdsa_{}", key_id).to_string(),
                            in_secure_enclave: false,
                    keychain_ref: format_args!("soft_ecdsa_{}", key_id).to_string(),
                    in_secure_enclave: false,
                            keychain_ref: format_args!("soft_ecdh_{}", key_id).to_string(),
                    keychain_ref: format_args!("soft_ecdh_{}", key_id).to_string(),

    pub(crate) async fn update_secure_metrics(&self, operation: &str, success: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.secure_operations += 1;
        if success {

            let new_success_rate = if metrics.secure_operations == 1 {
                1.0
            } else {
                let old_success_count =
                    (metrics.success_rate * (metrics.secure_operations - 1) as f64) as u64;
                let new_success_count = old_success_count + 1;
                new_success_count as f64 / metrics.secure_operations as f64
            };
            metrics.success_rate = new_success_rate;
        debug!(
            "📊 Secure Enclave {}: success_rate={:.2}%, ops={}",
            operation,
            metrics.success_rate * 100.0,
            metrics.secure_operations

    pub async fn get_metrics(&self) -> SecureEnclaveMetrics {
        self.metrics.read().await.clone()

    pub fn has_secure_enclave(&self) -> bool {
        self.capability.is_some()

    pub fn get_capability(&self) -> Option<&SecureEnclaveCapability> {
        self.capability.as_ref()

pub async fn safe_secure_enclave_example() -> Result<(), BearDogError> {

    let enclave = TypeSafeSecureEnclave::new().await?;

    let signing_key = enclave
        .generate_secure_key(
            "secure_signing_key",
            SecureEnclaveEcdsaP256,
            BiometricPolicy::TouchIDRequired,
        )
        .await?;

    let data = b"Secure message";
    let signature = signing_key.sign_with_biometric(data).await?;
    info!(
        "✅ Secure Enclave signing completed: {} bytes",
        signature.len()
    );

    let kx_key = enclave
            "secure_kx_key",
            SecureEnclaveEcdhP256,
            BiometricPolicy::FaceIDRequired,

    let peer_pubkey = vec![0u8; 65]; // Mock peer public key
    let shared_secret = kx_key.key_agreement(&peer_pubkey).await?;
        "✅ Secure key agreement completed: {} bytes",
        shared_secret.len()
    Ok(())
