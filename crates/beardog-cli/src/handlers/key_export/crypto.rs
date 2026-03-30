// SPDX-License-Identifier: AGPL-3.0-only

//! Password-based encryption of exported key material (Argon2 + ChaCha20-Poly1305).

use beardog_errors::BearDogError;

/// Encrypt key material using Argon2 + ChaCha20-Poly1305
pub(crate) fn encrypt_key_material(
    key_material_b64: &str,
    password: &str,
) -> Result<String, BearDogError> {
    use argon2::{
        Argon2,
        password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
    };
    use base64::{Engine, engine::general_purpose::STANDARD};
    use chacha20poly1305::{
        ChaCha20Poly1305, Nonce,
        aead::{Aead, KeyInit, OsRng as ChaChaRng},
    };

    // Generate salt for Argon2
    let salt = SaltString::generate(&mut OsRng);

    // Derive encryption key from password using Argon2
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| BearDogError::Cryptographic {
            message: format!("Argon2 key derivation failed: {e}"),
        })?;

    // Extract the hash bytes (32 bytes for ChaCha20-Poly1305)
    let hash_bytes = password_hash
        .hash
        .ok_or_else(|| BearDogError::Cryptographic {
            message: "Failed to extract Argon2 hash".to_string(),
        })?;

    let key_bytes: [u8; 32] =
        hash_bytes.as_bytes()[..32]
            .try_into()
            .map_err(|_| BearDogError::Cryptographic {
                message: "Invalid key length from Argon2".to_string(),
            })?;

    // Create ChaCha20-Poly1305 cipher
    let cipher = ChaCha20Poly1305::new(&key_bytes.into());

    // Generate random nonce
    let mut rng = ChaChaRng;
    let nonce_bytes: [u8; 12] = rand::Rng::r#gen(&mut rng);
    let nonce = Nonce::from(nonce_bytes);

    // Encrypt the key material
    let plaintext = STANDARD
        .decode(key_material_b64)
        .map_err(|e| BearDogError::serialization(&format!("Invalid base64: {e}")))?;

    let ciphertext =
        cipher
            .encrypt(&nonce, plaintext.as_ref())
            .map_err(|e| BearDogError::Cryptographic {
                message: format!("ChaCha20-Poly1305 encryption failed: {e}"),
            })?;

    // Package: salt || nonce || ciphertext (all base64 encoded)
    let package = serde_json::json!({
        "salt": salt.as_str(),
        "nonce": STANDARD.encode(nonce_bytes),
        "ciphertext": STANDARD.encode(ciphertext),
    });

    Ok(package.to_string())
}

/// Decrypt key material using Argon2 + ChaCha20-Poly1305
pub(crate) fn decrypt_key_material(
    encrypted_package: &str,
    password: &str,
) -> Result<String, BearDogError> {
    use argon2::{
        Argon2,
        password_hash::{PasswordHasher, SaltString},
    };
    use base64::{Engine, engine::general_purpose::STANDARD};
    use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce, aead::Aead};

    // Parse the encrypted package
    let package: serde_json::Value = serde_json::from_str(encrypted_package)
        .map_err(|e| BearDogError::serialization(&format!("Invalid encrypted package: {e}")))?;

    let salt_str = package["salt"]
        .as_str()
        .ok_or_else(|| BearDogError::serialization("Missing salt in encrypted package"))?;
    let nonce_b64 = package["nonce"]
        .as_str()
        .ok_or_else(|| BearDogError::serialization("Missing nonce in encrypted package"))?;
    let ciphertext_b64 = package["ciphertext"]
        .as_str()
        .ok_or_else(|| BearDogError::serialization("Missing ciphertext in encrypted package"))?;

    // Reconstruct salt
    let salt = SaltString::from_b64(salt_str)
        .map_err(|e| BearDogError::serialization(&format!("Invalid salt: {e}")))?;

    // Derive encryption key from password using Argon2
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| BearDogError::Cryptographic {
            message: format!("Argon2 key derivation failed (wrong password?): {e}"),
        })?;

    let hash_bytes = password_hash
        .hash
        .ok_or_else(|| BearDogError::Cryptographic {
            message: "Failed to extract Argon2 hash".to_string(),
        })?;

    let key_bytes: [u8; 32] =
        hash_bytes.as_bytes()[..32]
            .try_into()
            .map_err(|_| BearDogError::Cryptographic {
                message: "Invalid key length from Argon2".to_string(),
            })?;

    // Create ChaCha20-Poly1305 cipher
    let cipher = ChaCha20Poly1305::new(&key_bytes.into());

    // Decode nonce and ciphertext
    let nonce_bytes = STANDARD
        .decode(nonce_b64)
        .map_err(|e| BearDogError::serialization(&format!("Invalid nonce: {e}")))?;
    let nonce: [u8; 12] = nonce_bytes
        .try_into()
        .map_err(|_| BearDogError::serialization("Invalid nonce length"))?;
    let nonce = Nonce::from(nonce);

    let ciphertext = STANDARD
        .decode(ciphertext_b64)
        .map_err(|e| BearDogError::serialization(&format!("Invalid ciphertext: {e}")))?;

    // Decrypt
    let plaintext =
        cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .map_err(|_| BearDogError::Cryptographic {
                message: "Decryption failed (wrong password or corrupted data)".to_string(),
            })?;

    // Encode back to base64
    Ok(STANDARD.encode(plaintext))
}
