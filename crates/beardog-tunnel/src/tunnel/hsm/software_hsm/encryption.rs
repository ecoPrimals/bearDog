

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use beardog_errors::BearDogError;
use rand::RngCore;
use std::sync::Arc;

pub trait EncryptionKey: Send + Sync {

    /// Initializes componentialize
    fn initialize(&self) -> Result<(), BearDogError>;


    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>>;


    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>>;
}

pub struct DefaultEncryptionKey {
    cipher: Arc<Aes256Gcm>,}

impl DefaultEncryptionKey {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        let mut key_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut key_bytes);
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        Ok(Self {
            cipher: Arc::new(cipher),
        })
    }

/// From Key Material operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates instance from key material
    pub fn from_key_material(key_material: &[u8]) -> Result<Self, BearDogError> {
        if key_material.len() != 32 {
            return Err(BearDogError::encryption("key_initialization".to_string(), "Key material must be exactly 32 bytes for AES-256"));
        }
        let key = Key::<Aes256Gcm>::from_slice(key_material);

/// Fallback operation.
    pub fn fallback() -> Self {

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
    /// Initializes componentialize
    fn initialize(&self) -> Result<(), BearDogError> {

        Ok(())}


    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>> {

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext =
            self.cipher
                .encrypt(nonce, plaintext)
                .map_err(|e| BearDogError::encryption("aes_gcm_encrypt".to_string(), format!("AES-GCM encryption failed: {}e"),
                })?;

        let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        if ciphertext.len() < 12 {
                operation: "aes_gcm_decrypt".to_string(),
                message: "Ciphertext too short - missing nonce".to_string(),

        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext =
                .decrypt(nonce, encrypted_data)
                    operation: "aes_gcm_decrypt".to_string(),
                    message: format!("AES-GCM decryption failed: {e}"),
        Ok(plaintext)
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    fn test_encryption_roundtrip() -> Result<(), BearDogError> {
        let key = DefaultEncryptionKey::new()?;
        let plaintext = b"Hello, `BearDog` secure encryption!";
        let ciphertext = key.encrypt(plaintext)?;
        let decrypted = key.decrypt(&ciphertext)?;
        assert_eq!(plaintext, decrypted.as_slice());}


    fn test_different_nonces() -> Result<(), BearDogError> {
        let plaintext = b"Same message, different nonces";
        let ciphertext1 = key.encrypt(plaintext)?;
        let ciphertext2 = key.encrypt(plaintext)?;

        assert_ne!(ciphertext1, ciphertext2);

        assert_eq!(key.decrypt(&ciphertext1)?, plaintext);
        assert_eq!(key.decrypt(&ciphertext2)?, plaintext);
