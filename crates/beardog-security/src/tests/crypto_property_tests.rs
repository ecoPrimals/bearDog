// SPDX-License-Identifier: AGPL-3.0-or-later

//! Property-based tests for cryptographic invariants (proptest).

#![cfg(test)]

use crate::crypto_utils::BearDogCrypto;
use beardog_errors::BearDogError;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};
use proptest::prelude::*;

fn chacha_poly1305_roundtrip(
    key: &[u8],
    nonce: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| BearDogError::security(format!("ChaCha20-Poly1305 encrypt: {e}")))?;
    cipher
        .decrypt(nonce, ciphertext.as_slice())
        .map_err(|e| BearDogError::security(format!("ChaCha20-Poly1305 decrypt: {e}")))
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 64,
        ..ProptestConfig::default()
    })]

    /// Any message signed with a fresh Ed25519 key verifies.
    #[test]
    fn ed25519_sign_verify_roundtrip(msg in prop::collection::vec(any::<u8>(), 1..4097)) {
        let (sk, pk) = BearDogCrypto::generate_ed25519_keypair();
        let sig = BearDogCrypto::sign_ed25519(&sk, &msg).expect("sign");
        prop_assert!(BearDogCrypto::verify_ed25519(&pk, &msg, &sig).expect("verify op"));
    }

    /// BLAKE3 is deterministic for the same input (crate uses `blake3` for hashing).
    #[test]
    fn blake3_hash_deterministic(data in prop::collection::vec(any::<u8>(), 0..4097)) {
        let a = blake3::hash(&data).as_bytes().to_vec();
        let b = blake3::hash(&data).as_bytes().to_vec();
        prop_assert_eq!(a, b);
    }

    /// ChaCha20-Poly1305 encrypt/decrypt roundtrip for random plaintext, 32-byte key, 12-byte nonce.
    #[test]
    fn chacha20_poly1305_roundtrip(
        plaintext in prop::collection::vec(any::<u8>(), 0..4097),
        key in prop::array::uniform32(any::<u8>()),
        nonce in prop::array::uniform12(any::<u8>()),
    ) {
        let out = chacha_poly1305_roundtrip(&key, &nonce, &plaintext).expect("chacha roundtrip");
        prop_assert_eq!(out, plaintext);
    }

    /// AES-256-GCM roundtrip via `BearDogCrypto` with explicit nonce.
    #[test]
    fn aes_256_gcm_roundtrip(
        plaintext in prop::collection::vec(any::<u8>(), 0..4097),
        key in prop::array::uniform32(any::<u8>()),
        nonce in prop::array::uniform12(any::<u8>()),
    ) {
        let (ct, n) = BearDogCrypto::encrypt_aes_gcm(&key, &plaintext, Some(&nonce))
            .expect("encrypt");
        prop_assert_eq!(n, nonce);
        let pt = BearDogCrypto::decrypt_aes_gcm(&key, &ct, &nonce).expect("decrypt");
        prop_assert_eq!(pt, plaintext);
    }

    /// HMAC-SHA256: compute then verify succeeds for any message and non-empty key.
    #[test]
    fn hmac_sha256_verify_roundtrip(
        msg in prop::collection::vec(any::<u8>(), 0..4097),
        key in prop::collection::vec(any::<u8>(), 1..256),
    ) {
        let tag = BearDogCrypto::hmac_sha256(&key, &msg).expect("hmac");
        prop_assert!(BearDogCrypto::verify_hmac_sha256(&key, &msg, &tag).expect("verify"));
    }

    /// PBKDF2-HMAC-SHA256: same inputs always yield the same derived key.
    #[test]
    fn pbkdf2_key_derivation_deterministic(
        password in prop::collection::vec(any::<u8>(), 0..128),
        salt in prop::collection::vec(any::<u8>(), 8..64),
        iterations in 1u32..128u32,
        key_len in 16usize..65usize,
    ) {
        let k1 = BearDogCrypto::derive_pbkdf2_key(&password, &salt, iterations, key_len).expect("d1");
        let k2 = BearDogCrypto::derive_pbkdf2_key(&password, &salt, iterations, key_len).expect("d2");
        prop_assert_eq!(k2.len(), key_len);
        prop_assert_eq!(k1, k2);
    }
}
