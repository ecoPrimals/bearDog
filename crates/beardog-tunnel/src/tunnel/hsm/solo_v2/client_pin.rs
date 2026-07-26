// SPDX-License-Identifier: AGPL-3.0-or-later

//! PIN protocol helpers (pinProtocol 1: P-256 ECDH, AES-256-CBC, HMAC-SHA-256).

use beardog_errors::BearDogError;
use ciborium::Value as CborValue;

/// Parse a P-256 public key from a CTAP2 CBOR response map (key 1 = `keyAgreement`).
///
/// # Errors
///
/// Returns `BearDogError` if the CBOR value is not a map, required keys are missing, or the key bytes are invalid.
pub fn parse_cose_p256_from_map(cbor: &CborValue) -> Result<p256::PublicKey, BearDogError> {
    let CborValue::Map(map) = cbor else {
        return Err(BearDogError::system("Expected CBOR map".to_string()));
    };
    let cose_key = map
        .iter()
        .find_map(|(k, v)| match k {
            CborValue::Integer(i) if i128::from(*i) == 1 => Some(v),
            _ => None,
        })
        .ok_or_else(|| BearDogError::system("Missing keyAgreement (key 1)".to_string()))?;

    let CborValue::Map(cose_map) = cose_key else {
        return Err(BearDogError::system(
            "keyAgreement is not a map".to_string(),
        ));
    };

    let find_bytes = |key: i128| -> Option<&[u8]> {
        cose_map.iter().find_map(|(k, v)| match k {
            CborValue::Integer(i) if i128::from(*i) == key => match v {
                CborValue::Bytes(b) => Some(b.as_slice()),
                _ => None,
            },
            _ => None,
        })
    };

    let x = find_bytes(-2)
        .ok_or_else(|| BearDogError::system("Missing x in COSE key".to_string()))?;
    let y = find_bytes(-3)
        .ok_or_else(|| BearDogError::system("Missing y in COSE key".to_string()))?;

    let mut sec1 = Vec::with_capacity(65);
    sec1.push(0x04);
    sec1.extend_from_slice(x);
    sec1.extend_from_slice(y);
    p256::PublicKey::from_sec1_bytes(&sec1)
        .map_err(|e| BearDogError::system(format!("Invalid P-256 key: {e}")))
}

/// Extract a byte-string value from a CBOR map by integer key.
///
/// # Errors
///
/// Returns `BearDogError` if the CBOR value is not a map or the key is absent.
pub fn extract_bytes_from_cbor_map(
    cbor: &CborValue,
    key: i128,
) -> Result<Vec<u8>, BearDogError> {
    let CborValue::Map(map) = cbor else {
        return Err(BearDogError::system("Expected CBOR map".to_string()));
    };
    map.iter()
        .find_map(|(k, v)| match k {
            CborValue::Integer(i) if i128::from(*i) == key => match v {
                CborValue::Bytes(b) => Some(b.clone()),
                _ => None,
            },
            _ => None,
        })
        .ok_or_else(|| BearDogError::system(format!("Missing bytes at key {key}")))
}

/// AES-256-CBC encrypt with zero IV (pinProtocol 1).
///
/// # Errors
///
/// Returns `BearDogError` if the plaintext length is not a multiple of 16 bytes.
pub fn aes256_cbc_encrypt_zero_iv(
    key: &[u8; 32],
    plaintext: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::aes::Aes256;
    use aes_gcm::aes::cipher::{BlockEncrypt, KeyInit};

    if !plaintext.len().is_multiple_of(16) {
        return Err(BearDogError::system(
            "AES-CBC plaintext must be a multiple of 16".to_string(),
        ));
    }
    let cipher = Aes256::new(key.into());
    let mut out = Vec::with_capacity(plaintext.len());
    let mut prev = [0u8; 16];
    for chunk in plaintext.chunks(16) {
        let mut block = [0u8; 16];
        for (i, &b) in chunk.iter().enumerate() {
            block[i] = b ^ prev[i];
        }
        let block_ref: &mut aes_gcm::aes::Block = block.as_mut().into();
        cipher.encrypt_block(block_ref);
        prev = block;
        out.extend_from_slice(&block);
    }
    Ok(out)
}

/// AES-256-CBC decrypt with zero IV (pinProtocol 1).
///
/// # Errors
///
/// Returns `BearDogError` if the ciphertext length is not a multiple of 16 bytes.
pub fn aes256_cbc_decrypt_zero_iv(
    key: &[u8; 32],
    ciphertext: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::aes::Aes256;
    use aes_gcm::aes::cipher::{BlockDecrypt, KeyInit};

    if !ciphertext.len().is_multiple_of(16) {
        return Err(BearDogError::system(
            "AES-CBC ciphertext must be a multiple of 16".to_string(),
        ));
    }
    let cipher = Aes256::new(key.into());
    let mut out = Vec::with_capacity(ciphertext.len());
    let mut prev = [0u8; 16];
    for chunk in ciphertext.chunks(16) {
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);
        let block_ref: &mut aes_gcm::aes::Block = block.as_mut().into();
        cipher.decrypt_block(block_ref);
        for (i, &p) in prev.iter().enumerate() {
            block[i] ^= p;
        }
        out.extend_from_slice(&block);
        prev.copy_from_slice(chunk);
    }
    Ok(out)
}
