// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! CTAP2 `hmac-secret` Extension
//!
//! Implements the `hmac-secret` CTAP2 extension (FIDO v2.0 §9.1) for
//! hardware-backed entropy generation. The authenticator computes
//! `HMAC-SHA-256(credRandom, salt)` internally — the result never touches
//! the host except in encrypted form, providing a true hardware RNG path.
//!
//! # Protocol (pinProtocol 1)
//!
//! 1. Obtain the authenticator's `keyAgreement` P-256 public key
//! 2. Generate ephemeral P-256 key pair
//! 3. Derive `sharedSecret = SHA-256(ECDH(ephemeral, authenticatorKey).x)`
//! 4. Encrypt `salt1` (32 bytes) with AES-256-CBC(sharedSecret, IV=0)
//! 5. Compute `saltAuth = left(HMAC-SHA-256(sharedSecret, saltEnc), 16)`
//! 6. Build `GetAssertion` with extensions: `{ "hmac-secret": { 1: keyAgreement, 2: saltEnc, 3: saltAuth } }`
//! 7. Parse encrypted `hmac-secret` output from response, decrypt with sharedSecret

use beardog_errors::BearDogError;
use beardog_hid::HidDevice;
use ciborium::Value as CborValue;
use hmac::{Hmac, Mac};
use p256::PublicKey;
use p256::ecdh::EphemeralSecret;
use p256::elliptic_curve::rand_core::OsRng;
use p256::elliptic_curve::sec1::ToEncodedPoint;
use sha2::{Digest, Sha256};
use tracing::{debug, info};

use super::client_pin::get_pin_token;
use super::transport::{ctaphid_init, send_ctap2_command};
use super::types::Ctap2Command;

/// Generate hardware-backed entropy via `hmac-secret`.
///
/// Performs a `GetAssertion` with the `hmac-secret` extension against an existing
/// credential. The authenticator returns HMAC-SHA-256 of a random salt keyed
/// by the credential's internal secret — true hardware entropy.
///
/// # Arguments
///
/// * `device` — open HID device handle
/// * `rp_id` — relying party ID that the credential was created with
/// * `credential_id` — credential ID bytes from a prior `MakeCredential`
/// * `pin` — device PIN (required for pinProtocol 1 hmac-secret)
/// * `size` — desired entropy length in bytes (multiples of 32 recommended)
///
/// # Errors
///
/// Returns `BearDogError` on transport failure, missing credential, or
/// if the authenticator does not support `hmac-secret`.
pub async fn hmac_secret_entropy<D: HidDevice + ?Sized>(
    device: &mut D,
    rp_id: &str,
    credential_id: &[u8],
    pin: &str,
    size: usize,
) -> Result<Vec<u8>, BearDogError> {
    info!("Generating {} bytes of hmac-secret entropy", size);

    let cid = ctaphid_init(device).await?;

    // Get pinUvAuthToken for the assertion
    let pin_token = get_pin_token(device, pin).await?;

    // Generate a random salt (32 bytes)
    let salt: [u8; 32] = rand::random();

    // Get authenticator's key agreement key and set up ECDH
    let auth_pubkey = get_key_agreement_for_hmac(device, cid).await?;
    let ephemeral_secret = EphemeralSecret::random(&mut OsRng);
    let ephemeral_public = PublicKey::from(&ephemeral_secret);

    let shared_point = ephemeral_secret.diffie_hellman(&auth_pubkey);
    let shared_secret: [u8; 32] = Sha256::digest(shared_point.raw_secret_bytes()).into();

    // Encrypt salt with AES-256-CBC(sharedSecret, IV=0)
    let salt_enc = aes256_cbc_encrypt_zero_iv(&shared_secret, &salt)?;

    // saltAuth = left(HMAC-SHA-256(sharedSecret, saltEnc), 16)
    let salt_auth = left_hmac_sha256(&shared_secret, &salt_enc)?;

    // Build hmac-secret extension input CBOR
    let platform_key_cbor = encode_public_key_cose(&ephemeral_public)?;
    let hmac_secret_input = CborValue::Map(vec![
        (CborValue::Integer(1.into()), platform_key_cbor),
        (
            CborValue::Integer(2.into()),
            CborValue::Bytes(salt_enc.clone()),
        ),
        (
            CborValue::Integer(3.into()),
            CborValue::Bytes(salt_auth),
        ),
    ]);

    // Build GetAssertion CBOR
    let client_data_hash: [u8; 32] = rand::random();
    let pin_uv_auth_param = pin_token.authenticate(&client_data_hash);

    let assertion_map = CborValue::Map(vec![
        // 1: rpId
        (
            CborValue::Integer(1.into()),
            CborValue::Text(rp_id.to_string()),
        ),
        // 2: clientDataHash
        (
            CborValue::Integer(2.into()),
            CborValue::Bytes(client_data_hash.to_vec()),
        ),
        // 3: allowList
        (
            CborValue::Integer(3.into()),
            CborValue::Array(vec![CborValue::Map(vec![
                (
                    CborValue::Text("type".to_string()),
                    CborValue::Text("public-key".to_string()),
                ),
                (
                    CborValue::Text("id".to_string()),
                    CborValue::Bytes(credential_id.to_vec()),
                ),
            ])]),
        ),
        // 4: extensions
        (
            CborValue::Integer(4.into()),
            CborValue::Map(vec![(
                CborValue::Text("hmac-secret".to_string()),
                hmac_secret_input,
            )]),
        ),
        // 6: pinUvAuthParam
        (
            CborValue::Integer(6.into()),
            CborValue::Bytes(pin_uv_auth_param),
        ),
        // 7: pinUvAuthProtocol
        (CborValue::Integer(7.into()), CborValue::Integer(1.into())),
    ]);

    let mut cbor_payload = Vec::new();
    ciborium::into_writer(&assertion_map, &mut cbor_payload)
        .map_err(|e| BearDogError::system(format!("hmac-secret CBOR encode: {e}")))?;

    debug!(
        "GetAssertion+hmac-secret payload: {} bytes",
        cbor_payload.len()
    );

    let response =
        send_ctap2_command(device, cid, Ctap2Command::GetAssertion, &cbor_payload).await?;

    // Parse GetAssertion response CBOR
    let cbor: CborValue = ciborium::from_reader(response.as_slice())
        .map_err(|e| BearDogError::system(format!("GetAssertion CBOR decode: {e}")))?;

    // Extract extensions output (key 8 in authenticatorGetAssertion response map)
    let encrypted_output = extract_hmac_secret_output(&cbor)?;

    // Decrypt with AES-256-CBC(sharedSecret, IV=0)
    let hmac_output = aes256_cbc_decrypt_zero_iv(&shared_secret, &encrypted_output)?;

    info!(
        "Got {} bytes of hmac-secret output from authenticator",
        hmac_output.len()
    );

    // Expand to requested size via HKDF-like construction
    let mut entropy = Vec::with_capacity(size);
    let mut counter = 0u32;
    while entropy.len() < size {
        let block = Sha256::digest(
            [
                &hmac_output[..],
                &salt[..],
                &counter.to_le_bytes()[..],
            ]
            .concat()
            .as_slice(),
        );
        let take = block.len().min(size - entropy.len());
        entropy.extend_from_slice(&block[..take]);
        counter += 1;
    }

    info!("Generated {} bytes of hardware-backed entropy", size);
    Ok(entropy)
}

/// Extract hmac-secret encrypted output from `GetAssertion` response.
///
/// The hmac-secret output lives in the response map under key 8 (extensions).
fn extract_hmac_secret_output(cbor: &CborValue) -> Result<Vec<u8>, BearDogError> {
    let CborValue::Map(map) = cbor else {
        return Err(BearDogError::system(
            "Expected CBOR map in GetAssertion response".to_string(),
        ));
    };

    // Key 8 = extensions output (per CTAP2 spec for authenticatorGetAssertion)
    // For FIDO 2.0, hmac-secret output is in the authData extensions.
    // For FIDO 2.1+, it may be in the response extensions map (key 8).
    // Check key 8 first (FIDO 2.1), then try parsing from authData (FIDO 2.0).
    if let Some(extensions) = map.iter().find_map(|(k, v)| match k {
        CborValue::Integer(i) if i128::from(*i) == 8 => Some(v),
        _ => None,
    })
        && let CborValue::Map(ext_map) = extensions {
            for (k, v) in ext_map {
                if let CborValue::Text(name) = k
                    && name == "hmac-secret"
                        && let CborValue::Bytes(b) = v {
                            return Ok(b.clone());
                        }
            }
        }

    // Fallback: parse authData (key 2) for CBOR extensions
    if let Some(auth_data_val) = map.iter().find_map(|(k, v)| match k {
        CborValue::Integer(i) if i128::from(*i) == 2 => Some(v),
        _ => None,
    })
        && let CborValue::Bytes(auth_data) = auth_data_val {
            return extract_hmac_secret_from_auth_data(auth_data);
        }

    Err(BearDogError::system(
        "No hmac-secret output found in GetAssertion response".to_string(),
    ))
}

/// Parse hmac-secret output from authenticator data byte string.
///
/// `AuthData` format: rpIdHash(32) | flags(1) | signCount(4) | [extensions CBOR if ED flag set]
fn extract_hmac_secret_from_auth_data(auth_data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    if auth_data.len() < 37 {
        return Err(BearDogError::system(
            "AuthData too short for hmac-secret extraction".to_string(),
        ));
    }

    let flags = auth_data[32];
    let has_extensions = (flags & 0x80) != 0;

    if !has_extensions {
        return Err(BearDogError::system(
            "AuthData has no extensions (ED flag not set)".to_string(),
        ));
    }

    // Extensions start after rpIdHash(32) + flags(1) + signCount(4) = 37
    let ext_offset = 37;
    if ext_offset >= auth_data.len() {
        return Err(BearDogError::system(
            "AuthData too short to contain extensions".to_string(),
        ));
    }

    let ext_cbor: CborValue = ciborium::from_reader(&auth_data[ext_offset..])
        .map_err(|e| BearDogError::system(format!("AuthData extensions CBOR decode: {e}")))?;

    if let CborValue::Map(ext_map) = ext_cbor {
        for (k, v) in &ext_map {
            if let CborValue::Text(name) = k
                && name == "hmac-secret"
                    && let CborValue::Bytes(b) = v {
                        return Ok(b.clone());
                    }
        }
    }

    Err(BearDogError::system(
        "hmac-secret not found in AuthData extensions".to_string(),
    ))
}

// --- Internal helpers (duplicated from client_pin to avoid coupling) ---

fn aes256_cbc_encrypt_zero_iv(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::aes::Aes256;
    use aes_gcm::aes::cipher::{BlockEncrypt, KeyInit};

    if !plaintext.len().is_multiple_of(16) {
        return Err(BearDogError::system(
            "AES-CBC plaintext must be a multiple of 16 bytes".to_string(),
        ));
    }

    let cipher = Aes256::new(key.into());
    let mut ciphertext = Vec::with_capacity(plaintext.len());
    let mut prev_block = [0u8; 16];

    for chunk in plaintext.chunks(16) {
        let mut block = [0u8; 16];
        for (i, &b) in chunk.iter().enumerate() {
            block[i] = b ^ prev_block[i];
        }
        let block_ref: &mut aes_gcm::aes::Block = block.as_mut().into();
        cipher.encrypt_block(block_ref);
        prev_block = block;
        ciphertext.extend_from_slice(&block);
    }

    Ok(ciphertext)
}

fn aes256_cbc_decrypt_zero_iv(key: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::aes::Aes256;
    use aes_gcm::aes::cipher::{BlockDecrypt, KeyInit};

    if !ciphertext.len().is_multiple_of(16) {
        return Err(BearDogError::system(
            "AES-CBC ciphertext must be a multiple of 16 bytes".to_string(),
        ));
    }

    let cipher = Aes256::new(key.into());
    let mut plaintext = Vec::with_capacity(ciphertext.len());
    let mut prev_block = [0u8; 16];

    for chunk in ciphertext.chunks(16) {
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);
        let block_ref: &mut aes_gcm::aes::Block = block.as_mut().into();
        cipher.decrypt_block(block_ref);
        for (i, &p) in prev_block.iter().enumerate() {
            block[i] ^= p;
        }
        plaintext.extend_from_slice(&block);
        prev_block.copy_from_slice(chunk);
    }

    Ok(plaintext)
}

fn left_hmac_sha256(key: &[u8], message: &[u8]) -> Result<Vec<u8>, BearDogError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)
        .map_err(|e| BearDogError::hsm(format!("HMAC initialization failed: {e}")))?;
    mac.update(message);
    let result = mac.finalize().into_bytes();
    Ok(result[..16].to_vec())
}

fn encode_public_key_cose(key: &PublicKey) -> Result<CborValue, BearDogError> {
    let point = key.to_encoded_point(false);
    let x = point
        .x()
        .ok_or_else(|| BearDogError::hsm("P-256 point missing x coordinate".to_string()))?;
    let y = point
        .y()
        .ok_or_else(|| BearDogError::hsm("P-256 point missing y coordinate".to_string()))?;

    Ok(CborValue::Map(vec![
        (CborValue::Integer(1.into()), CborValue::Integer(2.into())),
        (
            CborValue::Integer(3.into()),
            CborValue::Integer((-25_i64).into()),
        ),
        (
            CborValue::Integer((-1_i64).into()),
            CborValue::Integer(1.into()),
        ),
        (
            CborValue::Integer((-2_i64).into()),
            CborValue::Bytes(x.to_vec()),
        ),
        (
            CborValue::Integer((-3_i64).into()),
            CborValue::Bytes(y.to_vec()),
        ),
    ]))
}

/// Get key agreement public key from authenticator for hmac-secret ECDH.
async fn get_key_agreement_for_hmac<D: HidDevice + ?Sized>(
    device: &mut D,
    cid: u32,
) -> Result<PublicKey, BearDogError> {
    let body = encode_get_key_agreement()?;
    let response =
        send_ctap2_command(device, cid, Ctap2Command::ClientPin, &body).await?;

    let cbor: CborValue = ciborium::from_reader(response.as_slice())
        .map_err(|e| BearDogError::system(format!("getKeyAgreement CBOR decode: {e}")))?;

    parse_cose_p256_pubkey(&cbor)
}

fn encode_get_key_agreement() -> Result<Vec<u8>, BearDogError> {
    let cmd_map = CborValue::Map(vec![
        (CborValue::Integer(1.into()), CborValue::Integer(1.into())), // pinProtocol 1
        (CborValue::Integer(2.into()), CborValue::Integer(2.into())), // getKeyAgreement
    ]);
    let mut body = Vec::new();
    ciborium::into_writer(&cmd_map, &mut body)
        .map_err(|e| BearDogError::system(format!("CBOR encode: {e}")))?;
    Ok(body)
}

fn parse_cose_p256_pubkey(cbor: &CborValue) -> Result<PublicKey, BearDogError> {
    let CborValue::Map(map) = cbor else {
        return Err(BearDogError::system(
            "Expected CBOR map in getKeyAgreement response".to_string(),
        ));
    };

    let cose_key = map
        .iter()
        .find_map(|(k, v)| match k {
            CborValue::Integer(i) if i128::from(*i) == 1 => Some(v),
            _ => None,
        })
        .ok_or_else(|| {
            BearDogError::system("Missing keyAgreement (key 1) in response".to_string())
        })?;

    let CborValue::Map(cose_map) = cose_key else {
        return Err(BearDogError::system(
            "keyAgreement is not a CBOR map".to_string(),
        ));
    };

    let x = extract_cose_bytes(cose_map, -2)?;
    let y = extract_cose_bytes(cose_map, -3)?;

    if x.len() != 32 || y.len() != 32 {
        return Err(BearDogError::system(format!(
            "Invalid P-256 coordinate lengths: x={}, y={}",
            x.len(),
            y.len()
        )));
    }

    let mut sec1 = Vec::with_capacity(65);
    sec1.push(0x04);
    sec1.extend_from_slice(&x);
    sec1.extend_from_slice(&y);

    PublicKey::from_sec1_bytes(&sec1)
        .map_err(|e| BearDogError::system(format!("Invalid P-256 public key: {e}")))
}

fn extract_cose_bytes(
    map: &[(CborValue, CborValue)],
    key: i128,
) -> Result<Vec<u8>, BearDogError> {
    map.iter()
        .find_map(|(k, v)| match k {
            CborValue::Integer(i) if i128::from(*i) == key => match v {
                CborValue::Bytes(b) => Some(b.clone()),
                _ => None,
            },
            _ => None,
        })
        .ok_or_else(|| BearDogError::system(format!("Missing COSE key {key}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_cbc_roundtrip() {
        let key = [0x42u8; 32];
        let plaintext = [0xABu8; 32]; // 2 blocks

        let encrypted = aes256_cbc_encrypt_zero_iv(&key, &plaintext).unwrap();
        assert_eq!(encrypted.len(), 32);
        assert_ne!(&encrypted[..], &plaintext[..]);

        let decrypted = aes256_cbc_decrypt_zero_iv(&key, &encrypted).unwrap();
        assert_eq!(&decrypted[..], &plaintext[..]);
    }

    #[test]
    fn test_aes_cbc_rejects_non_block_aligned() {
        let key = [0u8; 32];
        assert!(aes256_cbc_encrypt_zero_iv(&key, &[0u8; 15]).is_err());
        assert!(aes256_cbc_decrypt_zero_iv(&key, &[0u8; 17]).is_err());
    }

    #[test]
    fn test_hmac_sha256_truncation() {
        let key = [0x01u8; 32];
        let msg = b"test message";
        let result = left_hmac_sha256(&key, msg).expect("HMAC in test");
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn test_extract_hmac_secret_from_auth_data_no_extensions() {
        let mut auth_data = vec![0u8; 37]; // rpIdHash + flags + signCount
        auth_data[32] = 0x01; // UP flag, no ED
        assert!(extract_hmac_secret_from_auth_data(&auth_data).is_err());
    }

    #[test]
    fn test_extract_hmac_secret_from_auth_data_too_short() {
        let auth_data = vec![0u8; 10];
        assert!(extract_hmac_secret_from_auth_data(&auth_data).is_err());
    }
}
