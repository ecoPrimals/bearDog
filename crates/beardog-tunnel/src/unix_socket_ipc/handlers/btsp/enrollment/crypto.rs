// SPDX-License-Identifier: AGPL-3.0-or-later

//! Enrollment HMAC key derivation and message construction.

use beardog_config::env_keys;
use hkdf::Hkdf;
use sha2::Sha256;

/// Derive an enrollment HMAC key from the family seed via HKDF.
///
/// This mirrors `LineageKeyDerivation::derive_key` — same HKDF-SHA256 pattern,
/// generation in the info string, `FAMILY_ID` as salt for family scoping.
///
/// ```text
/// enrollment_key = HKDF-SHA256(
///     ikm  = family_seed,
///     salt = family_id_bytes,
///     info = "enrollment-v{generation}"
/// )
/// ```
pub fn derive_enrollment_key(family_seed: &[u8], generation: u32) -> [u8; 32] {
    let family_id = beardog_errors::process_env::var(env_keys::ENV_FAMILY_ID)
        .unwrap_or_else(|_| "default".to_string());

    let info = format!("enrollment-v{generation}");
    let hk = Hkdf::<Sha256>::new(Some(family_id.as_bytes()), family_seed);
    let mut key = [0u8; 32];
    #[expect(
        clippy::expect_used,
        reason = "HKDF-SHA256 expand to 32 bytes is infallible"
    )]
    hk.expand(info.as_bytes(), &mut key)
        .expect("HKDF expand 32 bytes");
    key
}

/// Build the enrollment HMAC message including seed generation.
pub fn build_enrollment_message(
    node_id: &str,
    public_key: &str,
    timestamp: u64,
    generation: u32,
) -> String {
    format!("{node_id}|{public_key}|{timestamp}|{generation}")
}

/// Compute HMAC-SHA256 over data with the given key.
pub fn compute_hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};

    type HmacSha256 = Hmac<Sha256>;

    #[expect(clippy::expect_used, reason = "HMAC-SHA256 accepts keys of any length")]
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC-SHA256 key init");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}
