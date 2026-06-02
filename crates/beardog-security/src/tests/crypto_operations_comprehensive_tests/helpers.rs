// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_errors::BearDogError;

pub(crate) fn generate_ed25519_keypair() -> Result<Ed25519Keypair, BearDogError> {
    use ed25519_dalek::SigningKey;
    use rand::RngCore;

    let mut seed = [0u8; 32];
    rand::rng().fill_bytes(&mut seed);
    let signing_key = SigningKey::from_bytes(&seed);
    let verifying_key = signing_key.verifying_key();

    Ok(Ed25519Keypair {
        public: verifying_key.to_bytes().to_vec(),
        secret: signing_key.to_bytes().to_vec(),
    })
}

pub(crate) fn sign_ed25519(secret: &[u8], message: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use ed25519_dalek::{Signer, SigningKey};

    if secret.len() != 32 {
        return Err(BearDogError::validation(
            "Ed25519 secret key must be 32 bytes",
        ));
    }

    let secret_array: [u8; 32] = secret
        .try_into()
        .map_err(|_| BearDogError::validation("Invalid secret key length"))?;

    let signing_key = SigningKey::from_bytes(&secret_array);
    let signature = signing_key.sign(message);

    Ok(signature.to_bytes().to_vec())
}

pub(crate) fn verify_ed25519_signature(
    public: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<bool, BearDogError> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    if public.len() != 32 {
        return Err(BearDogError::validation(
            "Ed25519 public key must be 32 bytes",
        ));
    }

    if signature.len() != 64 {
        return Err(BearDogError::validation(
            "Ed25519 signature must be 64 bytes",
        ));
    }

    let public_array: [u8; 32] = public
        .try_into()
        .map_err(|_| BearDogError::validation("Invalid public key length"))?;

    let sig_array: [u8; 64] = signature
        .try_into()
        .map_err(|_| BearDogError::validation("Invalid signature length"))?;

    let verifying_key = VerifyingKey::from_bytes(&public_array)
        .map_err(|e| BearDogError::validation(&format!("Invalid public key: {e}")))?;

    let sig = Signature::from_bytes(&sig_array);

    Ok(verifying_key.verify(message, &sig).is_ok())
}

pub(crate) fn generate_aes_256_key() -> Result<Vec<u8>, BearDogError> {
    use rand::RngCore;
    let mut key = vec![0u8; 32];
    rand::rng().fill_bytes(&mut key);
    Ok(key)
}

pub(crate) fn generate_aes_nonce() -> Result<Vec<u8>, BearDogError> {
    use rand::RngCore;
    let mut nonce = vec![0u8; 12];
    rand::rng().fill_bytes(&mut nonce);
    Ok(nonce)
}

pub(crate) fn encrypt_aes_256_gcm(
    key: &[u8],
    nonce: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| BearDogError::security(format!("AES-GCM encryption failed: {e}")))
}

pub(crate) fn decrypt_aes_256_gcm(
    key: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| BearDogError::security(format!("AES-GCM decryption failed: {e}")))
}

pub(crate) fn generate_chacha20_key() -> Result<Vec<u8>, BearDogError> {
    use rand::RngCore;
    let mut key = vec![0u8; 32];
    rand::rng().fill_bytes(&mut key);
    Ok(key)
}

pub(crate) fn generate_chacha20_nonce() -> Result<Vec<u8>, BearDogError> {
    use rand::RngCore;
    let mut nonce = vec![0u8; 12];
    rand::rng().fill_bytes(&mut nonce);
    Ok(nonce)
}

pub(crate) fn encrypt_chacha20_poly1305(
    key: &[u8],
    nonce: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce, aead::Aead};

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| BearDogError::security(format!("ChaCha20-Poly1305 encryption failed: {e}")))
}

pub(crate) fn decrypt_chacha20_poly1305(
    key: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce, aead::Aead};

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| BearDogError::security(format!("ChaCha20-Poly1305 decryption failed: {e}")))
}

pub(crate) fn encrypt_chacha20_poly1305_with_aad(
    key: &[u8],
    nonce: &[u8],
    plaintext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce, aead::Aead, aead::Payload};

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);
    let payload = Payload {
        msg: plaintext,
        aad,
    };

    cipher.encrypt(nonce, payload).map_err(|e| {
        BearDogError::security(format!("ChaCha20-Poly1305 AAD encryption failed: {e}"))
    })
}

pub(crate) fn decrypt_chacha20_poly1305_with_aad(
    key: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce, aead::Aead, aead::Payload};

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);
    let payload = Payload {
        msg: ciphertext,
        aad,
    };

    cipher.decrypt(nonce, payload).map_err(|e| {
        BearDogError::security(format!("ChaCha20-Poly1305 AAD decryption failed: {e}"))
    })
}

pub(crate) fn blake3_hash(data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    let hash = blake3::hash(data);
    Ok(hash.as_bytes().to_vec())
}

pub(crate) fn blake3_keyed_hash(key: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    if key.len() != 32 {
        return Err(BearDogError::validation("BLAKE3 key must be 32 bytes"));
    }
    let key_array: [u8; 32] = key
        .try_into()
        .map_err(|_| BearDogError::validation("Invalid key length"))?;
    let hash = blake3::keyed_hash(&key_array, data);
    Ok(hash.as_bytes().to_vec())
}

pub(crate) fn derive_key_pbkdf2(
    password: &[u8],
    salt: &[u8],
    iterations: u32,
) -> Result<Vec<u8>, BearDogError> {
    use pbkdf2::pbkdf2_hmac_array;
    use sha2::Sha256;

    let key = pbkdf2_hmac_array::<Sha256, 32>(password, salt, iterations);
    Ok(key.to_vec())
}

pub(crate) fn derive_key_argon2(password: &[u8], salt: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHasher};

    // Convert salt to base64 string format required by argon2
    let salt_str = SaltString::encode_b64(salt)
        .map_err(|e| BearDogError::validation(&format!("Invalid salt: {e}")))?;

    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password, &salt_str)
        .map_err(|e| BearDogError::security(format!("Argon2 failed: {e}")))?;

    // Extract the raw hash bytes
    let hash_bytes = hash
        .hash
        .ok_or_else(|| BearDogError::security("No hash produced".to_string()))?;

    Ok(hash_bytes.as_bytes().to_vec())
}

pub(crate) struct Ed25519Keypair {
    pub public: Vec<u8>,
    pub secret: Vec<u8>,
}
