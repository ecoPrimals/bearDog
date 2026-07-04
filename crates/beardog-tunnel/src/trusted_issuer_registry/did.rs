// SPDX-License-Identifier: AGPL-3.0-or-later

use ed25519_dalek::VerifyingKey;

/// Derive the `did:key:z6Mk...` DID from an Ed25519 public key.
///
/// This is the canonical derivation: multicodec prefix `0xed01` +
/// 32-byte public key, then base58btc-encode with `z` prefix.
#[must_use]
pub fn did_from_verifying_key(key: &VerifyingKey) -> String {
    let mut multicodec = Vec::with_capacity(34);
    multicodec.push(0xed);
    multicodec.push(0x01);
    multicodec.extend_from_slice(key.as_bytes());
    format!("did:key:z{}", bs58::encode(&multicodec).into_string())
}

/// Check if a DID string matches a verifying key.
#[must_use]
pub fn did_matches_key(did: &str, key: &VerifyingKey) -> bool {
    did_from_verifying_key(key) == did
}
