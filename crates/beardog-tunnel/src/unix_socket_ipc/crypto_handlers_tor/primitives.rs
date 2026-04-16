// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared primitives: HKDF, HMAC, `ChaCha20` counter mode, ntor circuit key derivation.

use super::constants::NTOR_T_EXPAND;
use beardog_errors::BearDogError;
use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Circuit keys derived from ntor handshake
pub(super) struct CircuitKeys {
    /// Forward digest key (Df) - 20 bytes
    pub(super) df: [u8; 20],
    /// Backward digest key (Db) - 20 bytes
    pub(super) db: [u8; 20],
    /// Forward encryption key (Kf) - 16 bytes for AES-128 or 32 for `ChaCha20`
    pub(super) kf: [u8; 16],
    /// Backward encryption key (Kb) - 16 bytes for AES-128 or 32 for `ChaCha20`
    pub(super) kb: [u8; 16],
}

/// Derive circuit keys from `KEY_SEED` using HKDF
pub(super) fn derive_circuit_keys(key_seed: &[u8]) -> Result<CircuitKeys, BearDogError> {
    // Expand to get: Df (20) + Db (20) + Kf (16) + Kb (16) = 72 bytes
    let expanded = hkdf_expand(key_seed, NTOR_T_EXPAND, 72)?;

    if expanded.len() != 72 {
        return Err(BearDogError::crypto_error(format!(
            "HKDF expansion produced {} bytes, expected 72",
            expanded.len()
        )));
    }

    Ok(CircuitKeys {
        df: expanded[0..20]
            .try_into()
            .map_err(|_| BearDogError::crypto_error("Failed to convert Df slice to [u8; 20]"))?,
        db: expanded[20..40]
            .try_into()
            .map_err(|_| BearDogError::crypto_error("Failed to convert Db slice to [u8; 20]"))?,
        kf: expanded[40..56]
            .try_into()
            .map_err(|_| BearDogError::crypto_error("Failed to convert Kf slice to [u8; 16]"))?,
        kb: expanded[56..72]
            .try_into()
            .map_err(|_| BearDogError::crypto_error("Failed to convert Kb slice to [u8; 16]"))?,
    })
}

/// HMAC-SHA256
pub(super) fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<[u8; 32], BearDogError> {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|e| BearDogError::crypto_error(format!("HMAC-SHA256 key error: {e}")))?;
    mac.update(data);
    let result = mac.finalize();
    Ok(result.into_bytes().into())
}

/// HKDF-Expand (simplified - uses HMAC iteratively)
pub(super) fn hkdf_expand(prk: &[u8], info: &[u8], length: usize) -> Result<Vec<u8>, BearDogError> {
    let mut output = Vec::with_capacity(length);
    let mut t = Vec::new();
    let mut counter = 1u8;

    while output.len() < length {
        let mut input = t.clone();
        input.extend_from_slice(info);
        input.push(counter);

        t = hmac_sha256(prk, &input)?.to_vec();
        output.extend_from_slice(&t);
        counter += 1;
    }

    output.truncate(length);
    Ok(output)
}

/// `ChaCha20` counter mode (for cell encryption)
pub(super) fn chacha20_counter_mode(
    key: &[u8],
    counter: u64,
    data: &mut [u8],
) -> Result<(), BearDogError> {
    use chacha20::ChaCha20;
    use chacha20::cipher::{KeyIvInit, StreamCipher};

    // Validate key length
    let key_arr: [u8; 32] = key.try_into().map_err(|_| {
        BearDogError::invalid_input(&format!(
            "ChaCha20 requires exactly 32-byte key, got {}",
            key.len()
        ))
    })?;

    // Construct nonce from counter (12 bytes)
    let mut nonce = [0u8; 12];
    nonce[4..12].copy_from_slice(&counter.to_le_bytes());

    let mut cipher = ChaCha20::new(&key_arr.into(), &nonce.into());
    cipher.apply_keystream(data);
    Ok(())
}

/// Fixed state encryption key (for protecting `client_state`)
pub(super) const STATE_ENCRYPTION_KEY: [u8; 32] = [
    0x62, 0x65, 0x61, 0x72, 0x64, 0x6f, 0x67, 0x2d, // "beardog-"
    0x6e, 0x74, 0x6f, 0x72, 0x2d, 0x73, 0x74, 0x61, // "ntor-sta"
    0x74, 0x65, 0x2d, 0x6b, 0x65, 0x79, 0x2d, 0x76, // "te-key-v"
    0x31, 0x2d, 0x70, 0x72, 0x6f, 0x64, 0x00, 0x01, // "1-prod.."
];

/// Simple XOR encryption (used with derived key for state protection)
pub(super) fn xor_encrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, BearDogError> {
    // Expand key using HKDF to match data length
    let expanded_key = hkdf_expand(key, b"state-encryption", data.len())?;
    Ok(data
        .iter()
        .zip(expanded_key.iter())
        .map(|(d, k)| d ^ k)
        .collect())
}

/// Constant-time comparison to prevent timing attacks
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
