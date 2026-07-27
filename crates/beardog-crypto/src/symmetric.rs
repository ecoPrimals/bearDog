// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symmetric encryption algorithms
//!
//! Provides implementations of symmetric encryption algorithms including:
//! - AES-256-GCM (256-bit key, authenticated encryption)
//! - AES-128-GCM (128-bit key, faster, still secure)
//! - ChaCha20-Poly1305 (stream cipher, constant-time)
//!
//! All implementations use safe Rust with established cryptography libraries.

use beardog_errors::BearDogError;

type Result<T> = std::result::Result<T, BearDogError>;

/// AES-256-GCM encryption
///
/// # Arguments
///
/// * `data` - Plaintext to encrypt
/// * `key` - 256-bit (32-byte) encryption key
/// * `aad` - Optional additional authenticated data
///
/// # Returns
///
/// Tuple of (ciphertext, nonce, authentication tag)
///
/// # Security
///
/// - Uses 96-bit random nonce (secure for 2^32 messages per key)
/// - 128-bit authentication tag
/// - Constant-time implementation
///
/// # Errors
///
/// Returns an error if the cipher cannot be initialized or encryption fails.
pub fn encrypt_aes_256_gcm(
    data: &[u8],
    key: &[u8; 32],
    aad: Option<&[u8]>,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    use aes_gcm::{
        Aes256Gcm, Nonce,
        aead::{Aead, KeyInit, Payload},
    };

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| BearDogError::hsm(format!("Failed to create AES-256-GCM cipher: {e}")))?;

    // Generate cryptographically secure random nonce
    let nonce_bytes = generate_random_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Prepare payload with optional AAD
    let payload = Payload {
        msg: data,
        aad: aad.unwrap_or(&[]),
    };

    // Encrypt (includes authentication)
    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|e| BearDogError::hsm(format!("AES-256-GCM encryption failed: {e}")))?;

    // AES-GCM appends the tag to ciphertext
    // Split into ciphertext and tag (last 16 bytes)
    let tag_start = ciphertext.len().saturating_sub(16);
    let (encrypted_data, tag) = ciphertext.split_at(tag_start);

    Ok((encrypted_data.to_vec(), nonce_bytes.to_vec(), tag.to_vec()))
}

/// AES-256-GCM decryption
///
/// # Arguments
///
/// * `ciphertext` - Encrypted data (without tag)
/// * `nonce` - 96-bit nonce used during encryption
/// * `tag` - 128-bit authentication tag
/// * `key` - 256-bit (32-byte) decryption key (must match encryption key)
/// * `aad` - Optional additional authenticated data (must match encryption AAD)
///
/// # Returns
///
/// Original plaintext data
///
/// # Errors
///
/// Returns error if:
/// - Authentication fails (wrong key or tampered data)
/// - Invalid nonce or tag format
pub fn decrypt_aes_256_gcm(
    ciphertext: &[u8],
    nonce: &[u8],
    tag: &[u8],
    key: &[u8; 32],
    aad: Option<&[u8]>,
) -> Result<Vec<u8>> {
    use aes_gcm::{
        Aes256Gcm, Nonce,
        aead::{Aead, KeyInit, Payload},
    };

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| BearDogError::hsm(format!("Failed to create AES-256-GCM cipher: {e}")))?;

    // Reconstruct full ciphertext (data + tag)
    let mut full_ciphertext = ciphertext.to_vec();
    full_ciphertext.extend_from_slice(tag);

    let nonce = Nonce::from_slice(nonce);

    // Prepare payload
    let payload = Payload {
        msg: &full_ciphertext,
        aad: aad.unwrap_or(&[]),
    };

    // Decrypt and verify authentication
    let plaintext = cipher.decrypt(nonce, payload).map_err(|e| {
        BearDogError::hsm(format!("AES-256-GCM decryption/authentication failed: {e}"))
    })?;

    Ok(plaintext)
}

/// AES-128-GCM encryption (faster alternative to AES-256-GCM)
///
/// Provides faster performance while maintaining strong security.
/// Still considered secure for most applications.
///
/// # Errors
///
/// Returns an error if the cipher cannot be initialized or encryption fails.
pub fn encrypt_aes_128_gcm(
    data: &[u8],
    key: &[u8; 16],
    aad: Option<&[u8]>,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    use aes_gcm::{
        Aes128Gcm, Nonce,
        aead::{Aead, KeyInit, Payload},
    };

    let cipher = Aes128Gcm::new_from_slice(key)
        .map_err(|e| BearDogError::hsm(format!("Failed to create AES-128-GCM cipher: {e}")))?;

    let nonce_bytes = generate_random_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let payload = Payload {
        msg: data,
        aad: aad.unwrap_or(&[]),
    };

    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|e| BearDogError::hsm(format!("AES-128-GCM encryption failed: {e}")))?;

    let tag_start = ciphertext.len().saturating_sub(16);
    let (encrypted_data, tag) = ciphertext.split_at(tag_start);

    Ok((encrypted_data.to_vec(), nonce_bytes.to_vec(), tag.to_vec()))
}

/// AES-128-GCM decryption
///
/// # Errors
///
/// Returns an error if authentication fails or the ciphertext was tampered with.
pub fn decrypt_aes_128_gcm(
    ciphertext: &[u8],
    nonce: &[u8],
    tag: &[u8],
    key: &[u8; 16],
    aad: Option<&[u8]>,
) -> Result<Vec<u8>> {
    use aes_gcm::{
        Aes128Gcm, Nonce,
        aead::{Aead, KeyInit, Payload},
    };

    let cipher = Aes128Gcm::new_from_slice(key)
        .map_err(|e| BearDogError::hsm(format!("Failed to create AES-128-GCM cipher: {e}")))?;

    let mut full_ciphertext = ciphertext.to_vec();
    full_ciphertext.extend_from_slice(tag);

    let nonce = Nonce::from_slice(nonce);

    let payload = Payload {
        msg: &full_ciphertext,
        aad: aad.unwrap_or(&[]),
    };

    let plaintext = cipher.decrypt(nonce, payload).map_err(|e| {
        BearDogError::hsm(format!("AES-128-GCM decryption/authentication failed: {e}"))
    })?;

    Ok(plaintext)
}

/// ChaCha20-Poly1305 encryption
///
/// Stream cipher with Poly1305 authentication.
/// Advantages:
/// - Constant-time (no cache-timing attacks)
/// - Fast on platforms without AES-NI
/// - Modern IETF standard
///
/// # Errors
///
/// Returns an error if the cipher cannot be initialized or encryption fails.
pub fn encrypt_chacha20_poly1305(
    data: &[u8],
    key: &[u8; 32],
    aad: Option<&[u8]>,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    use chacha20poly1305::{
        ChaCha20Poly1305, Nonce,
        aead::{Aead, KeyInit, Payload},
    };

    let cipher = ChaCha20Poly1305::new_from_slice(key).map_err(|e| {
        BearDogError::hsm(format!("Failed to create ChaCha20-Poly1305 cipher: {e}"))
    })?;

    let nonce_bytes = generate_random_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let payload = Payload {
        msg: data,
        aad: aad.unwrap_or(&[]),
    };

    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|e| BearDogError::hsm(format!("ChaCha20-Poly1305 encryption failed: {e}")))?;

    // Extract tag (last 16 bytes)
    let tag_start = ciphertext.len().saturating_sub(16);
    let (encrypted_data, tag) = ciphertext.split_at(tag_start);

    Ok((encrypted_data.to_vec(), nonce_bytes.to_vec(), tag.to_vec()))
}

/// ChaCha20-Poly1305 decryption
///
/// # Errors
///
/// Returns an error if authentication fails or the ciphertext was tampered with.
pub fn decrypt_chacha20_poly1305(
    ciphertext: &[u8],
    nonce: &[u8],
    tag: &[u8],
    key: &[u8; 32],
    aad: Option<&[u8]>,
) -> Result<Vec<u8>> {
    use chacha20poly1305::{
        ChaCha20Poly1305, Nonce,
        aead::{Aead, KeyInit, Payload},
    };

    let cipher = ChaCha20Poly1305::new_from_slice(key).map_err(|e| {
        BearDogError::hsm(format!("Failed to create ChaCha20-Poly1305 cipher: {e}"))
    })?;

    let mut full_ciphertext = ciphertext.to_vec();
    full_ciphertext.extend_from_slice(tag);

    let nonce = Nonce::from_slice(nonce);

    let payload = Payload {
        msg: &full_ciphertext,
        aad: aad.unwrap_or(&[]),
    };

    let plaintext = cipher.decrypt(nonce, payload).map_err(|e| {
        BearDogError::hsm(format!(
            "ChaCha20-Poly1305 decryption/authentication failed: {e}"
        ))
    })?;

    Ok(plaintext)
}

/// Generate cryptographically secure random nonce (96 bits / 12 bytes)
///
/// Uses the operating system's CSPRNG via `getrandom`.
fn generate_random_nonce() -> [u8; 12] {
    use rand::RngCore;
    let mut nonce = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce);
    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_256_gcm_round_trip() {
        let key = [42u8; 32];
        let data = b"Hello, BearDog!";

        let (ciphertext, nonce, tag) =
            encrypt_aes_256_gcm(data, &key, None).expect("encrypt_aes_256_gcm in test");
        let plaintext =
            decrypt_aes_256_gcm(&ciphertext, &nonce, &tag, &key, None).expect("decrypt in test");

        assert_eq!(data, plaintext.as_slice());
    }

    #[test]
    fn test_aes_256_gcm_with_aad() {
        let key = [42u8; 32];
        let data = b"Secret message";
        let aad = b"Additional context";

        let (ciphertext, nonce, tag) =
            encrypt_aes_256_gcm(data, &key, Some(aad)).expect("encrypt_aes_256_gcm in test");
        let plaintext = decrypt_aes_256_gcm(&ciphertext, &nonce, &tag, &key, Some(aad))
            .expect("decrypt in test");

        assert_eq!(data, plaintext.as_slice());
    }

    #[test]
    fn test_chacha20_round_trip() {
        let key = [99u8; 32];
        let data = b"ChaCha test data";

        let (ciphertext, nonce, tag) =
            encrypt_chacha20_poly1305(data, &key, None).expect("encrypt_chacha20_poly1305 in test");
        let plaintext = decrypt_chacha20_poly1305(&ciphertext, &nonce, &tag, &key, None)
            .expect("decrypt_chacha20_poly1305 in test");

        assert_eq!(data, plaintext.as_slice());
    }
}
