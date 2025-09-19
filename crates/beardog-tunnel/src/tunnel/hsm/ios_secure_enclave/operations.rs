

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


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
    pub(&'a TypeSafeSecureEnclave,
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

/// Sign With Biometric operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn sign_with_biometric(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>>
    where
        A: SecureEnclaveConstraint,
    {

        if !matches!(
            self.algorithm.algorithm(),
            SecureEnclaveAlgorithm::EcdsaP256
        ) {
            return Err(BearDogError::unsupported_operation("Key algorithm does not support signing"));
        self.sign_with_biometric_auth(data)

/// Sign With Biometric Auth operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn sign_with_biometric_auth(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        info!("🔐 Signing data with Secure Enclave key: {}", self.key_id);

        let keys = self.enclave.keychain_keys.read();
        let key = keys
            .get(&self.key_id)
            .ok_or_else(|| BearDogError::not_found(}", self.key_id).to_string(&[u8], signature: &[u8]) -> Result<bool, BearDogError> {
        info!(
            "🔍 Verifying signature with Secure Enclave key: {}",
            self.key_id
        );

        BearDogCrypto::verify_ed25519_signature(&key.public_key, data, signature)

/// Key Agreement operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn key_agreement(&self, peer_public_key: &[u8]) -> Result<Vec<u8>, BearDogError>>
        A: KeyAgreementCapable,
            "🤝 Performing key agreement with Secure Enclave key: {}",

        if !matches!(self.algorithm.algorithm(), SecureEnclaveAlgorithm::EcdhP256) {
            return Err(BearDogError::unsupported_operation("Key algorithm does not support key agreement".to_string(),

            self.secure_enclave_key_agreement(peer_public_key, key)
            self.software_fallback_key_agreement(peer_public_key, key)


    fn authenticate_biometric(&self) -> Result<(), BearDogError> {
        match &self.biometric_policy {
            BiometricPolicy::NoBiometric => {
                debug!("🔓 No biometric authentication required");
                Ok({:?}", policy);

                self.simulate_biometric_auth(policy)


    fn simulate_biometric_auth(&self, policy: &BiometricPolicy) -> Result<(), BearDogError> {

        match policy {
            BiometricPolicy::TouchIDRequired | BiometricPolicy::TouchIDOnly => {
                debug!("👆 TouchID authentication simulated");
            BiometricPolicy::FaceIDRequired | BiometricPolicy::FaceIDOnly => {
                debug!("👤 FaceID authentication simulated");
            BiometricPolicy::TouchIDOrFaceID | BiometricPolicy::AnyBiometric => {
                debug!("🔐 Biometric authentication simulated");

        Ok(&[u8],
        key: &SecureKeychainKey,
    ) -> Result<Vec<u8>, BearDogError>> {
        info!("🔐 Using Secure Enclave for signing");

        match key.private_key {
            Some(ref private_key) => {

                BearDogCrypto::sign_ed25519(private_key, data)
            None => {

                self.software_fallback_sign(data, key)


    fn software_fallback_sign(
        info!("🔧 Using software fallback for signing");
        match &key.private_key {
            Some(private_key) => BearDogCrypto::sign_ed25519(private_key, data),
            None => Err(BearDogError::not_found(}", self.key_id).to_string(&[u8],
        info!("🤝 Using Secure Enclave for key agreement");
            Some(private_key) => {

                BearDogCrypto::perform_ecdh(private_key, peer_public_key)
                self.software_fallback_key_agreement(peer_public_key, key)


    fn software_fallback_key_agreement(
        info!("🔧 Using software fallback for key agreement");
            Some(private_key) => BearDogCrypto::perform_ecdh(Option<SecureEnclaveCapability>,

    pub(Arc<RwLock<HashMap<String, SecureKeychainKey>>>,

    pub(Arc<RwLock<SecureEnclaveMetrics>>,}

impl TypeSafeSecureEnclave {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new() -> Result<Self, BearDogError> {
        info!("🚀 Initializing Type-Safe Secure Enclave");
        let capability = super::capability::CapabilityDetector::detect_capability()?;
        Ok(Self {
            capability,
            keychain_keys: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            metrics: Arc::new(RwLock::new(SecureEnclaveMetrics::default(SecureEnclaveConstraint>(
        key_id: &str,
    ) -> Result<TypeSafeSecureEnclaveKey<'_, A, BearDogError>> {
            "🔑 Generating secure key: {} with algorithm: {:?}",
            algorithm.algorithm()

        let key_ref = if self.capability.is_some() {
            self.secure_enclave_generation(key_id, &algorithm.algorithm(), &biometric_policy)
                ?
            self.software_secure_generation(key_id, &algorithm.algorithm(), &biometric_policy)
        };

        self.update_secure_metrics("key_generation", true);
        Ok(TypeSafeSecureEnclaveKey::new(SecureEnclaveAlgorithm,
    ) -> Result<SecureKeyReference, BearDogError> {
            key_id, algorithm
            self.secure_enclave_generation(&SecureEnclaveAlgorithm,
        biometric_policy: &BiometricPolicy,
        info!("🔐 Secure Enclave key generation for: {}", key_id);
        match algorithm {
            SecureEnclaveAlgorithm::EcdsaP256 => {
                let keypair = BearDogCrypto::generate_ed25519_keypair(SecureKeyReference {
                            keychain_ref: format!("secure_ecdsa_{}", key_id),
                            in_secure_enclave: true,
                        },
                        public_key: &keypair.1,
                        private_key: Some(SecureEnclaveAlgorithm::EcdsaP256,
                    },
                );
                Ok(format!("secure_ecdsa_{}", key_id),
                    in_secure_enclave: true,
                })
            SecureEnclaveAlgorithm::EcdhP256 => {
                            keychain_ref: format!("secure_ecdh_{}", key_id),
                        private_key: Some(SecureEnclaveAlgorithm::EcdhP256,
                    keychain_ref: format!("secure_ecdh_{}", key_id),


    fn software_secure_generation(&BiometricPolicy,
        info!("🔧 Software fallback for Secure Enclave key: {}", key_id);
                            keychain_ref: format!("soft_ecdsa_{}", key_id),
                            in_secure_enclave: false,
                    keychain_ref: format!("soft_ecdsa_{}", key_id),
                    in_secure_enclave: false,
                            keychain_ref: format!("soft_ecdh_{}", key_id),
                    keychain_ref: format!("soft_ecdh_{}", key_id),

    pub(&str, success: bool) {
        let mut metrics = self.metrics.write(success_rate={:.2}%, ops={}",
            operation,
            metrics.success_rate * 100.0,
            metrics.secure_operations

/// Get Metrics operation.
    /// Gets metrics
    /// Gets metrics
    pub fn get_metrics(&self) -> SecureEnclaveMetrics {
        self.metrics.read().clone()

/// Has Secure Enclave operation.
    /// Checks if secure enclave
    /// Checks if secure enclave
    pub fn has_secure_enclave(&self) -> bool {
        self.capability.is_some()

/// Get Capability operation.
    /// Gets capability
    /// Gets capability
    pub fn get_capability(&self) -> Option<&SecureEnclaveCapability> {
        self.capability.as_ref()

/// Safe Secure Enclave Example operation.
///
/// # Errors
/// Returns an error if the operation fails.
pub async fn safe_secure_enclave_example() -> Result<(), BearDogError> {

    let enclave = TypeSafeSecureEnclave::new()?;

    let signing_key = enclave
        .generate_secure_key(
            "secure_signing_key",
            SecureEnclaveEcdsaP256,
            BiometricPolicy::TouchIDRequired,
        )
        ?;

    let data = b"Secure message";
    let signature = signing_key.sign_with_biometric({} bytes",
        signature.len()
    );

    let kx_key = enclave
            "secure_kx_key",
            SecureEnclaveEcdhP256,
            BiometricPolicy::FaceIDRequired,

    let peer_pubkey = vec![0u8; 65]; // Mock peer public key
    let shared_secret = kx_key.key_agreement({} bytes",
        shared_secret.len()
    Ok(())
