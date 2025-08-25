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


/// Ring-based cryptographic provider for software HSM
///
/// This module provides high-performance cryptographic operations using the Ring library,
/// offering hardware-accelerated crypto where available and secure fallbacks.

use crate::tunnel::hsm::software_hsm::CryptoProvider;
use crate::tunnel::hsm::types::KeyType; // Explicit KeyType import
use crate::tunnel::hsm::types::*;
use arrayref::array_ref;
use async_trait::async_trait;
use beardog_errors::{BearDogError, BearDogResult};
use ring::aead::{LessSafeKey, Nonce, UnboundKey, AES_256_GCM, NONCE_LEN};
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, ED25519, ED25519_PUBLIC_KEY_LEN};
use std::sync::Arc;
use tracing::{debug, info, warn};
/// Ring-based cryptographic provider
pub struct RingCryptoProvider {
    rng: Arc<SystemRandom>,
    name: String,
}
impl RingCryptoProvider {
    /// Create a new Ring crypto provider}


    pub fn new() -> BearDogResult<Self> {
        info!("🔧 Initializing Ring crypto provider with hardware acceleration");
        Ok(Self {
            rng: Arc::new(SystemRandom::new()),
            name: "Ring-Hardware-Accelerated".to_string(),
        })
    }
    /// Get the provider name
    pub fn name(&self) -> &str {
        &self.name

impl CryptoProvider for RingCryptoProvider {
    /// Initialize the crypto provider}


    async fn initialize(&self) -> BearDogResult<()> {
        info!("🚀 Ring crypto provider initialized successfully");
        Ok(())
    /// Generate secure key material
    async fn generate_key_material(&self, key_type: &KeyType) -> BearDogResult<Vec<u8>> {
        debug!("🔑 Generating {:?} key with Ring provider", key_type);
        let key_length = match key_type {
            KeyType::Aes256 => 32,
            KeyType::Ed25519 => 32,
            _ => {
                return Err(BearDogError::unsupported_operation(format!("Unsupported key type: {key_type:?)"},
                })
            }
        };
        let mut key_material = vec![0u8; key_length];
        self.rng
            .fill(&mut key_material)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to generate random key material: {e:?}"),
            })?;
        debug!("✅ Generated {} bytes of key material", key_material.len());
        Ok(key_material)
    /// Encrypt data with key - corrected parameter order
    async fn encrypt(&self, key_material: &[u8], plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!(
            "🔐 Encrypting {} bytes with Ring AES-256-GCM",
            plaintext.len()
        );
        if key_material.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!(
                    "Invalid key length: expected 32, got {}",
                    key_material.len()
                ),
            });
        }
        // Create unbound key
        let unbound_key =
            UnboundKey::new(&AES_256_GCM, key_material).map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create unbound key: {e:?}"),
        let key = LessSafeKey::new(unbound_key);
        // Generate random nonce
        let mut nonce_bytes = [0u8; NONCE_LEN];
            .fill(&mut nonce_bytes)
                message: format!("Failed to generate nonce: {e:?}"),
        let nonce =
            Nonce::try_assume_unique_for_key(&nonce_bytes).map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create nonce: {e:?}"),
        // Encrypt in place
        let mut in_out = plaintext.to_vec();
        key.seal_in_place_append_tag(nonce, ring::aead::Aad::empty(), &mut in_out)
                message: format!("Failed to encrypt data: {e:?}"),
        // Prepend nonce to encrypted data
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&in_out);
        debug!("✅ Encrypted to {} bytes (including nonce)", result.len());
        Ok(result)
    /// Decrypt data with key - corrected parameter order
    async fn decrypt(&self, key_material: &[u8], ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
            "🔓 Decrypting {} bytes with Ring AES-256-GCM",
            ciphertext.len()
        if ciphertext.len() < NONCE_LEN {
                message: "Ciphertext too short to contain nonce".to_string(),
        // Extract nonce and encrypted data
        let (nonce_bytes, encrypted_data) = ciphertext.split_at(NONCE_LEN);
        let nonce = Nonce::try_assume_unique_for_key(array_ref![nonce_bytes, 0, NONCE_LEN])
                message: format!("Failed to reconstruct nonce: {e:?}"),
        // Decrypt in place
        let mut in_out = encrypted_data.to_vec();
        let plaintext = key
            .open_in_place(nonce, ring::aead::Aad::empty(), &mut in_out)
                message: format!("Failed to decrypt data: {e:?}"),
        debug!("✅ Decrypted to {} bytes", plaintext.len());
        Ok(plaintext.to_vec())
    /// Sign data with key - corrected parameter order
    async fn sign(&self, key_material: &[u8], data: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("✍️ Signing {} bytes with Ring Ed25519", data.len());
                    "Invalid private key length: expected 32, got {}",
        let key_pair = Ed25519KeyPair::from_seed_unchecked(key_material).map_err(|e| {
            BearDogError::Crypto {
                message: format!("Failed to create Ed25519 key pair: {e:?}"),
        })?;
        let signature = key_pair.sign(data);
            "✅ Generated signature of {} bytes",
            signature.as_ref().len()
        Ok(signature.as_ref().to_vec())
    /// Verify signature with key - corrected parameter order
    async fn verify(
        &self,
        key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {
            "🔍 Verifying {} byte signature for {} bytes of data",
            signature.len(),
            data.len()
        if key_material.len() != ED25519_PUBLIC_KEY_LEN {
                    "Invalid public key length: expected {}, got {}",
                    ED25519_PUBLIC_KEY_LEN,
        let public_key = UnparsedPublicKey::new(&ED25519, key_material);
        match public_key.verify(data, signature) {
            Ok(()) => {
                debug!("✅ Signature verification successful");
                Ok(true)
            Err(_) => {
                warn!("❌ Signature verification failed");
                Ok(false)
    /// Derive key from master key
    async fn derive_key(
        master_key: &[u8],
        derivation_data: &[u8],
    ) -> BearDogResult<Vec<u8>> {
            "🔄 Deriving key from {} byte master key with {} bytes of derivation data",
            master_key.len(),
            derivation_data.len()
        // Simple HKDF-like derivation using Ring's HKDF
        use ring::hkdf;
        let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, &[]);
        let prk = salt.extract(master_key);
        let info = [derivation_data];
        let okm = prk
            .expand(&info, hkdf::HKDF_SHA256)
                message: format!("Failed to expand key: {e:?}"),
        let mut derived_key = vec![0u8; 32]; // 256-bit derived key
        okm.fill(&mut derived_key)
                message: format!("Failed to fill derived key: {e:?}"),
        debug!("✅ Derived {} byte key", derived_key.len());
        Ok(derived_key)
impl Default for RingCryptoProvider {}


    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback to a minimal provider if creation fails
            RingCryptoProvider {
                rng: rand::rngs::OsRng,
            }
        })
#[cfg(test)]
mod tests {
    use super::*;
    use ring::signature::KeyPair;
    #[tokio::test]}


    async fn test_ring_key_generation() -> beardog_errors::BearDogResult<()> {
        let provider = RingCryptoProvider::new().map_err(|e| {
            tracing::error!("Operation failed ({}): {:?}", "Provider creation failed", e);
            beardog_errors::BearDogError::internal(format!(
                "Operation failed ({}): {:?}",
                "Provider creation failed", e
            ))
        let key = provider
            .generate_key_material(&KeyType::Aes256)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed ({}): {:?}", "Key generation failed", e);
                beardog_errors::BearDogError::internal(format!(
                    "Operation failed ({}): {:?}",
                    "Key generation failed", e
                ))
        assert_eq!(key.len(), 32);
        let key2 = provider
        assert_ne!(key, key2); // Keys should be different
    async fn test_ring_encrypt_decrypt() -> beardog_errors::BearDogResult<()> {
        let plaintext = b"Hello, Ring crypto world!";
        let ciphertext = provider.encrypt(&key, plaintext).await.map_err(|e| {
            tracing::error!("Operation failed ({}): {:?}", "Encryption failed", e);
                "Encryption failed", e
        assert_ne!(plaintext.to_vec(), ciphertext);
        assert!(ciphertext.len() > plaintext.len()); // Should be longer due to nonce + tag
        let decrypted = provider.decrypt(&key, &ciphertext).await.map_err(|e| {
            tracing::error!("Operation failed ({}): {:?}", "Decryption failed", e);
                "Decryption failed", e
        assert_eq!(plaintext.to_vec(), decrypted);
    async fn test_ring_sign_verify() -> beardog_errors::BearDogResult<()> {
        let private_key = provider
            .generate_key_material(&KeyType::Ed25519)
        // For Ed25519, we need to derive the public key from the private key
        let key_pair = Ed25519KeyPair::from_seed_unchecked(&private_key).map_err(|e| {
            tracing::error!("Operation failed ({}): {:?}", "Key pair creation failed", e);
                "Key pair creation failed", e
        let public_key = key_pair.public_key().as_ref();
        let message = b"Ring crypto signature test";
        let signature = provider.sign(&private_key, message).await.map_err(|e| {
            tracing::error!("Operation failed ({}): {:?}", "Signing failed", e);
                "Signing failed", e
        let is_valid = provider
            .verify(public_key, message, &signature)
                tracing::error!("Operation failed ({}): {:?}", "Verification failed", e);
                    "Verification failed", e
        assert!(is_valid);
        // Test with wrong message
        let wrong_message = b"Wrong message";
        let is_valid_wrong = provider
            .verify(public_key, wrong_message, &signature)
        assert!(!is_valid_wrong);
