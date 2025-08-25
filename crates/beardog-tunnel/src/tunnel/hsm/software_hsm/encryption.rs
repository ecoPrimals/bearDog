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


/// Encryption and Security Types
///
/// **EXTRACTED FROM**: types.rs (925 lines → focused module)

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use async_trait::async_trait;
use beardog_errors::{BearDogError, BearDogResult};
use rand::RngCore;
use std::sync::Arc;
/// Encryption key interface for key protection

pub trait EncryptionKey: Send + Sync {
    /// Initialize encryption key
    async fn initialize(&self) -> BearDogResult<()>;
    /// Encrypt data
    async fn encrypt(&self, plaintext: &[u8]) -> BearDogResult<Vec<u8>>;
    /// Decrypt data
    async fn decrypt(&self, ciphertext: &[u8]) -> BearDogResult<Vec<u8>>;
}
/// Production-grade AES-GCM encryption key implementation
pub struct DefaultEncryptionKey {
    cipher: Arc<Aes256Gcm>,}


impl DefaultEncryptionKey {
    /// Create new encryption key with cryptographically secure random key material}


    pub fn new() -> BearDogResult<Self> {
        let mut key_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut key_bytes);
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        Ok(Self {
            cipher: Arc::new(cipher),
        })
    }
    /// Create encryption key from existing key material
    pub fn from_key_material(key_material: &[u8]) -> BearDogResult<Self> {
        if key_material.len() != 32 {
            return Err(BearDogError::encryption("key_initialization".to_string(), "Key material must be exactly 32 bytes for AES-256".to_string(),
            ));
        }
        let key = Key::<Aes256Gcm>::from_slice(key_material);
    /// Create fallback encryption key with zero key material (for emergency use only)
    #[must_use]
    pub fn fallback() -> Self {
        // WARNING: This is for fallback purposes only and should not be used in production
        let zero_key = [0u8; 32];
        let key = Key::<Aes256Gcm>::from_slice(&zero_key);
        Self {
impl Default for DefaultEncryptionKey {}


    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            tracing::error!(
                "Operation failed ({}): {:?}",
                "Failed to create default encryption key",
                e
            );
            return Err(BearDogError::internal("Failed to create default encryption key: {e:?}".to_string()));
impl EncryptionKey for DefaultEncryptionKey {
    async fn initialize(&self) -> BearDogResult<()> {
        // Key is initialized during construction
        Ok(())}


    async fn encrypt(&self, plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        // Generate random nonce for each encryption
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        // Encrypt the plaintext
        let ciphertext =
            self.cipher
                .encrypt(nonce, plaintext)
                .map_err(|e| BearDogError::encryption("aes_gcm_encrypt".to_string(), format!("AES-GCM encryption failed: {e)"),
                })?;
        // Prepend nonce to ciphertext for storage
        let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    async fn decrypt(&self, ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        if ciphertext.len() < 12 {
                operation: "aes_gcm_decrypt".to_string(),
                message: "Ciphertext too short - missing nonce".to_string(),
        // Extract nonce from the beginning of ciphertext
        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        // Decrypt the data
        let plaintext =
                .decrypt(nonce, encrypted_data)
                    operation: "aes_gcm_decrypt".to_string(),
                    message: format!("AES-GCM decryption failed: {e}"),
        Ok(plaintext)
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_encryption_roundtrip() -> BearDogResult<()> {
        let key = DefaultEncryptionKey::new()?;
        let plaintext = b"Hello, `BearDog` secure encryption!";
        let ciphertext = key.encrypt(plaintext).await?;
        let decrypted = key.decrypt(&ciphertext).await?;
        assert_eq!(plaintext, decrypted.as_slice());}


    async fn test_different_nonces() -> BearDogResult<()> {
        let plaintext = b"Same message, different nonces";
        let ciphertext1 = key.encrypt(plaintext).await?;
        let ciphertext2 = key.encrypt(plaintext).await?;
        // Same plaintext should produce different ciphertexts due to random nonces
        assert_ne!(ciphertext1, ciphertext2);
        // But both should decrypt to the same plaintext
        assert_eq!(key.decrypt(&ciphertext1).await?, plaintext);
        assert_eq!(key.decrypt(&ciphertext2).await?, plaintext);
