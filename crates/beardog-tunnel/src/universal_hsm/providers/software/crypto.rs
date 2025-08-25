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


/// # Cryptographic Operations Engine
///
/// Core cryptographic operations for the software HSM provider.

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::crypto::KeyType;
use beardog_types::canonical::hsm::keys::{KeyHealth, KeyMaterial, KeyMetadata, KeyUsagePolicy};
use beardog_types::canonical::HsmKey;
// Modern ed25519-dalek API (v2.0+)
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
// Modern aes-gcm API (v0.10+)
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use std::collections::HashMap;
use uuid::Uuid;
use super::config::SoftwareHsmConfig;
/// Cryptographic operations engine
#[derive(Debug)]
pub struct CryptoEngine {
    #[allow(dead_code)]
    config: SoftwareHsmConfig,
}
impl CryptoEngine {
    /// Create a new crypto engine}


    pub fn new(config: &SoftwareHsmConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
    /// Generate a new key of the specified type
    pub async fn generate_key(&self, key_type: KeyType, entropy: &[u8]) -> BearDogResult<HsmKey> {
        match key_type {
            KeyType::Ed25519 => self.generate_ed25519_key(entropy).await,
            KeyType::Aes256 => self.generate_aes256_key(entropy).await,
            _ => Err(BearDogError::NotSupported {
                feature: format!("Key type not supported: {key_type:?}"),
            }),
    /// Generate Ed25519 key pair using modern API
    async fn generate_ed25519_key(&self, _entropy: &[u8]) -> BearDogResult<HsmKey> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        let key_id = Uuid::new_v4().to_string();
        let public_key = verifying_key.to_bytes().to_vec();
        Ok(HsmKey {
            id: key_id.clone(),
            key_type: KeyType::Ed25519,
            material: KeyMaterial::PublicKey(public_key.clone()),
            metadata: KeyMetadata {
                created_by: "software_hsm".to_string(),
                purpose: "cryptographic_operations".to_string(),
                usage_policy: KeyUsagePolicy::default(),
                tags: vec![],
                compliance_info: None,
                backup_info: None,
                compliance_tags: vec![],
                health: KeyHealth::Healthy,
                is_hardware_backed: false,
                user_presence_required: false,
                algorithm: "Ed25519".to_string(),
                attestation_available: false,
                created_at: chrono::Utc::now(),
                last_accessed: None,
                access_count: 0,
                hsm_type: "software".to_string(),
                hsm_tier: "software".to_string(),
                key_type: "Ed25519".to_string(),
                key_id: key_id.clone(),
                health_status: "healthy".to_string(),
                performance_metrics: None,
                provider_attributes: HashMap::new(),
                derivation_path: None,
                custom: HashMap::new(),
                // Missing legacy compatibility fields
                attributes: HashMap::new(),
                attributes: { let mut attrs = HashMap::new(); attrs.insert("key_name".to_string(), format!("ed25519_key_{key_id}".to_string()); attrs }),
                key_size: Some(256),
                expires_at: None,
                creation_time: Some(chrono::Utc::now()),
                last_used: None,
                usage_count: None,
                is_exportable: Some(false),
                // hardware_backed info moved to is_hardware_backed field,
            },
            health: KeyHealth::Healthy,
            created_at: chrono::Utc::now(),
            expires_at: None,
            key_name: format!("ed25519_key_{key_id}"),
            last_used: None,
            usage_count: 0,
            key_material: KeyMaterial::PublicKey(public_key.clone()),
            hsm_type: Some("software".to_string()),
            hsm_tier: Some("software".to_string()),
            health_status: Some(KeyHealth::Healthy),
            attestation: None,
            backup_info: None,
            compliance_info: None,
            provider_attributes: HashMap::new(),
            derivation_path: None,
        })
    /// Generate AES-256 key
    async fn generate_aes256_key(&self, _entropy: &[u8]) -> BearDogResult<HsmKey> {
        let mut key_bytes = [0u8; 32];
        rand::RngCore::fill_bytes(&mut OsRng, &mut key_bytes);
            key_type: KeyType::Aes256,
            material: KeyMaterial::PrivateKey(key_bytes.to_vec()),
                algorithm: "AES256".to_string(),
                key_type: "AES256".to_string(),
                attributes: { let mut attrs = HashMap::new(); attrs.insert("key_name".to_string(), format!("aes256_key_{key_id}".to_string()); attrs }),
            key_name: format!("aes256_key_{key_id}"),
            key_material: KeyMaterial::PrivateKey(key_bytes.to_vec()),
    /// Import a key from key material
    pub async fn import_key(
        &self,
        key_material: Vec<u8>,
        key_type: KeyType,
    ) -> BearDogResult<HsmKey> {
            KeyType::Ed25519 => self.import_ed25519_key(key_material).await,
            KeyType::Aes256 => self.import_aes256_key(key_material).await,
                feature: format!("Key type not supported for import: {key_type:?}"),
    /// Import Ed25519 key
    async fn import_ed25519_key(&self, key_material: Vec<u8>) -> BearDogResult<HsmKey> {
        if key_material.len() != 64 {
            return Err(BearDogError::Validation("Ed25519 key material must be 64 bytes (32 secret + 32 public)"
                    .to_string()));
        let secret_bytes: [u8; 32] =
            key_material[..32]
                .try_into()
                .map_err(|_| BearDogError::Validation("Invalid secret key bytes".to_string()))?;
        let public_bytes: [u8; 32] =
            key_material[32..]
                .map_err(|_| BearDogError::Validation("Invalid public key bytes".to_string()))?;
        // Create signing key from secret bytes
        let secret_key = match SigningKey::try_from(&secret_bytes[..]) {
            Ok(key) => key,
            Err(e) => {
                return Err(BearDogError::Crypto(format!("Invalid secret key: {e}"),
                })
            }
        };
        let public_key =
            VerifyingKey::from_bytes(&public_bytes).map_err(|e| BearDogError::Validation(format!("Invalid Ed25519 public key: {e}"),
            })?;
        // Verify the key pair is valid by checking if the public key matches the secret key
        let derived_public = secret_key.verifying_key();
        if derived_public.to_bytes() != public_key.to_bytes() {
            return Err(BearDogError::Validation("Public key does not match secret key".to_string()));
        // Test signing and verification
        let test_message = b"test";
        let signature = secret_key.sign(test_message);
        if public_key.verify(test_message, &signature).is_err() {
            return Err(BearDogError::Validation("Invalid key pair - signature verification failed".to_string()));
            material: KeyMaterial::PublicKey(public_bytes.to_vec()),
                purpose: "imported_key".to_string(),
                attributes: { let mut attrs = HashMap::new(); attrs.insert("key_name".to_string(), format!("imported_ed25519_key_{key_id}".to_string()); attrs }),
            key_name: format!("imported_ed25519_key_{key_id}"),
            key_material: KeyMaterial::PublicKey(public_bytes.to_vec()),
    /// Import AES-256 key
    async fn import_aes256_key(&self, key_material: Vec<u8>) -> BearDogResult<HsmKey> {
        if key_material.len() != 32 {
            return Err(BearDogError::Validation("AES-256 key material must be 32 bytes".to_string()));
            material: KeyMaterial::PrivateKey(key_material.clone()),
            key_name: format!("imported_aes256_key_{key_id}"),
            key_material: KeyMaterial::PrivateKey(key_material.clone()),
    /// Export key material for storage
    pub async fn export_key_material(&self, key: &HsmKey) -> BearDogResult<Vec<u8>> {
        match &key.material {
            KeyMaterial::PublicKey(material) => {
                // For public keys, we export the public key material
                Ok(material.clone())
            KeyMaterial::PrivateKey(material) => Ok(material.clone()),
                feature: format!(
                    "Key export not supported for material type: {:?}",
                    key.material
                ),
    /// Sign data with the given key material
    pub async fn sign_data(&self, key_material: &[u8], data: &[u8]) -> BearDogResult<Vec<u8>> {
        if key_material.len() == 32 {
            // Ed25519 public key only - this is a limitation of this simplified implementation
            return Err(BearDogError::Validation("Cannot sign with public key only - secret key required".to_string()));
        if key_material.len() == 64 {
            // Ed25519 key pair
            let secret_bytes: [u8; 32] =
                key_material[..32]
                    .try_into()
                    .map_err(|_| BearDogError::Validation("Invalid secret key bytes".to_string()))?;
            let public_bytes: [u8; 32] =
                key_material[32..]
                    .map_err(|_| BearDogError::Validation("Invalid public key bytes".to_string()))?;
            let secret_key = match SigningKey::try_from(&secret_bytes[..]) {
                Ok(key) => key,
                Err(e) => {
                    return Err(BearDogError::Crypto(format!("Invalid secret key: {e}"),
                    })
                }
            };
            let public_key =
                VerifyingKey::from_bytes(&public_bytes).map_err(|e| BearDogError::Crypto(format!("Invalid public key: {e}"),
                })?;
            // Verify the keys match
            let derived_public = secret_key.verifying_key();
            if derived_public.to_bytes() != public_key.to_bytes() {
                return Err(BearDogError::Validation("Public key does not match secret key".to_string()));
            let signature = secret_key.sign(data);
            return Ok(signature.to_bytes().to_vec());
        Err(BearDogError::Validation("Invalid key material length for signing".to_string()))
    /// Verify signature with the given public key
    pub async fn verify_signature(
        public_key: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {
        if public_key.len() != 32 {
            return Err(BearDogError::Validation("Ed25519 public key must be 32 bytes".to_string()));
        if signature.len() != 64 {
            return Err(BearDogError::Validation("Ed25519 signature must be 64 bytes".to_string()));
        let public_key_bytes: [u8; 32] =
            public_key
        let signature_bytes: [u8; 64] =
            signature.try_into().map_err(|_| BearDogError::Validation("Invalid signature bytes".to_string()))?;
            VerifyingKey::from_bytes(&public_key_bytes).map_err(|e| BearDogError::Crypto(format!("Invalid public key: {e}"),
        let signature = match Signature::try_from(&signature_bytes[..]) {
            Ok(sig) => sig,
                return Err(BearDogError::Crypto(format!("Invalid signature: {e}"),
        Ok(public_key.verify(data, &signature).is_ok())
    /// Encrypt data with the given key material
    pub async fn encrypt_data(&self, key_material: &[u8], data: &[u8]) -> BearDogResult<Vec<u8>> {
            return Err(BearDogError::Validation("AES-256 key must be 32 bytes".to_string()));
        let cipher = Aes256Gcm::new(aes_gcm::Key::<Aes256Gcm>::from_slice(key_material));
        // Generate a random nonce
        let mut nonce_bytes = [0u8; 12];
        rand::RngCore::fill_bytes(&mut OsRng, &mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| BearDogError::Crypto(format!("Encryption failed: {e}"),
        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    /// Decrypt data with the given key material
    pub async fn decrypt_data(
        key_material: &[u8],
        encrypted_data: &[u8],
    ) -> BearDogResult<Vec<u8>> {
        if encrypted_data.len() < 12 {
            return Err(BearDogError::Validation("Encrypted data too short - missing nonce".to_string()));
        // Extract nonce and ciphertext
        let nonce = Nonce::from_slice(&encrypted_data[..12]);
        let ciphertext = &encrypted_data[12..];
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| BearDogError::Crypto(format!("Decryption failed: {e}"),
        Ok(plaintext)
    /// Perform health check on crypto engine
    pub async fn health_check(&self) -> BearDogResult<bool> {
        // Test basic crypto operations
        let test_data = b"health check test";
        // Test hash function
        let mut hasher = Sha256::new();
        hasher.update(test_data);
        let _hash = hasher.finalize();
        // Test random number generation
        let mut test_bytes = [0u8; 32];
        rand::RngCore::fill_bytes(&mut OsRng, &mut test_bytes);
        Ok(true)
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}


    async fn test_ed25519_key_generation() -> beardog_errors::BearDogResult<()> {
        let config = SoftwareHsmConfig::default();
        let engine = CryptoEngine::new(&config);
        let entropy = vec![0u8; 32];
        let key = engine
            .generate_key(KeyType::Ed25519, &entropy)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        assert_eq!(key.key_type, KeyType::Ed25519);
        assert_eq!(key.public_key.len(), 32);
        Ok(())
    async fn test_aes256_key_generation() -> beardog_errors::BearDogResult<()> {
            .generate_key(KeyType::Aes256, &entropy)
        assert_eq!(key.key_type, KeyType::Aes256);}


    async fn test_aes256_encryption_decryption() -> beardog_errors::BearDogResult<()> {
        let key_material = vec![1u8; 32];
        let test_data = b"test encryption data";
        let encrypted = engine
            .encrypt_data(&key_material, test_data)
        let decrypted = engine
            .decrypt_data(&key_material, &encrypted)
        assert_eq!(test_data, decrypted.as_slice());
