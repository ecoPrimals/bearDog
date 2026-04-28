// SPDX-License-Identifier: AGPL-3.0-or-later

//! Crypto Handler Tests
//!
//! Tests for the `CryptoHandler` method registration and routing.

use super::*;

#[test]
fn test_crypto_handler_methods() {
    let handler = CryptoHandler;
    let methods = handler.methods();

    // Should have 92 methods (Phase 1-8 + TLS 1.2 + Dark Forest + Device Enrollment + Onion Service + Tor v3 + Tor Phase 2)
    // Breakdown (Feb 2026 - Deep Debt Evolution):
    //   - 3 Ed25519 (generate, sign, verify)
    //   - 4 ECDSA (P-256 + P-384 sign/verify)
    //   - 4 RSA (PKCS1 + PSS sign/verify)
    //   - 6 key exchange (X25519 x2, ECDH x4)
    //   - 6 AEAD (ChaCha20 x2, AES-GCM x4)
    //   - 12 hash (blake3, sha256, sha384, sha512, sha1, sha3_256, derive_onion_address, hash_for_cipher + hmac variants + hmac_blake3)
    //   - 6 password (argon2id x2, pbkdf2, bcrypt x2, scrypt)
    //   - 6 TLS 1.3 (derive_secrets, derive_handshake_secrets, derive_application_secrets, compute_finished_verify_data, sign_handshake, verify_certificate)
    //   - 9 TLS 1.2 semantic (ecdhe x4, aead x4, tls12_prf)
    //   - 10 genetic (derive_lineage_key, mix_entropy, verify_lineage, generate_lineage_proof, challenge x3, device x3)
    //   - 8 semantic aliases (hash, hmac, sign, verify, encrypt, decrypt, generate_keypair, derive_secret)
    //   - 10 beardog.crypto.* (onion service namespace)
    //   - 2 Tor v3 Phase 1 (crypto.derive_onion_address semantic + beardog.crypto.* alias; generate_onion_identity)
    //   - 6 Tor Phase 2 (ntor_client_init, ntor_client_finish, ntor_server_respond, cell_encrypt, cell_decrypt, tor_kdf)
    //   - 2 semantic dot-separated aliases (crypto.ed25519.sign, crypto.ed25519.verify)
    //   - 1 crypto.public_key (standalone key retrieval)
    assert_eq!(methods.len(), 96);

    // Verify all core crypto methods are present
    assert!(methods.contains(&"crypto.sign_ed25519"));
    assert!(methods.contains(&"crypto.verify_ed25519"));
    assert!(methods.contains(&"crypto.x25519_generate_ephemeral"));
    assert!(methods.contains(&"crypto.x25519_derive_secret"));
    assert!(methods.contains(&"crypto.chacha20_poly1305_encrypt"));
    assert!(methods.contains(&"crypto.chacha20_poly1305_decrypt"));
    assert!(methods.contains(&"crypto.blake3_hash"));
    assert!(methods.contains(&"crypto.hmac_sha256"));

    // Verify ECDSA methods
    assert!(methods.contains(&"crypto.sign_ecdsa_secp256r1"));
    assert!(methods.contains(&"crypto.verify_ecdsa_secp256r1"));
    assert!(methods.contains(&"crypto.sign_ecdsa_secp384r1"));
    assert!(methods.contains(&"crypto.verify_ecdsa_secp384r1"));

    // Verify RSA methods
    assert!(methods.contains(&"crypto.sign_rsa_pkcs1_sha256"));
    assert!(methods.contains(&"crypto.verify_rsa_pkcs1_sha256"));
    assert!(methods.contains(&"crypto.sign_rsa_pss_sha256"));
    assert!(methods.contains(&"crypto.verify_rsa_pss_sha256"));

    // Verify TLS methods
    assert!(methods.contains(&"tls.derive_secrets"));
    assert!(methods.contains(&"tls.derive_handshake_secrets"));
    assert!(methods.contains(&"tls.derive_application_secrets"));
    assert!(methods.contains(&"tls.sign_handshake"));
    assert!(methods.contains(&"tls.verify_certificate"));

    // Verify genetic methods (Phase 5 + Dark Forest)
    assert!(methods.contains(&"genetic.derive_lineage_key"));
    assert!(methods.contains(&"genetic.mix_entropy"));
    assert!(methods.contains(&"genetic.verify_lineage"));
    assert!(methods.contains(&"genetic.generate_lineage_proof"));
    assert!(methods.contains(&"genetic.generate_challenge"));
    assert!(methods.contains(&"genetic.respond_to_challenge"));
    assert!(methods.contains(&"genetic.verify_challenge_response"));

    // Verify device enrollment methods (Feb 5, 2026 - Deep Debt: DERIVE, not COPY)
    assert!(methods.contains(&"genetic.derive_device_seed"));
    assert!(methods.contains(&"genetic.sign_lineage_certificate"));
    assert!(methods.contains(&"genetic.verify_lineage_certificate"));

    // Verify semantic aliases (Phase 2 - Jan 27, 2026)
    assert!(methods.contains(&"crypto.hash"));
    assert!(methods.contains(&"crypto.hmac"));
    assert!(methods.contains(&"crypto.sign"));
    assert!(methods.contains(&"crypto.verify"));
    assert!(methods.contains(&"crypto.public_key"));
    assert!(methods.contains(&"crypto.encrypt"));
    assert!(methods.contains(&"crypto.decrypt"));
    assert!(methods.contains(&"crypto.generate_keypair"));
    assert!(methods.contains(&"crypto.derive_secret"));

    // Verify beardog.crypto.* namespace (onion service methods)
    assert!(methods.contains(&"beardog.crypto.sha3_256"));
    assert!(methods.contains(&"beardog.crypto.ed25519_generate_keypair"));
    assert!(methods.contains(&"beardog.crypto.sign_ed25519"));
    assert!(methods.contains(&"beardog.crypto.verify_ed25519"));
    assert!(methods.contains(&"beardog.crypto.x25519_generate_ephemeral"));
    assert!(methods.contains(&"beardog.crypto.x25519_derive_secret"));
    assert!(methods.contains(&"beardog.crypto.chacha20_poly1305_encrypt"));
    assert!(methods.contains(&"beardog.crypto.chacha20_poly1305_decrypt"));
    assert!(methods.contains(&"beardog.crypto.hmac_sha256"));
    assert!(methods.contains(&"beardog.crypto.blake3_hash"));

    // Verify Tor v3 methods (Feb 7, 2026 - Phase 1 Tor Integration)
    assert!(methods.contains(&"crypto.derive_onion_address"));
    assert!(methods.contains(&"beardog.crypto.derive_onion_address"));
    assert!(methods.contains(&"beardog.crypto.generate_onion_identity"));

    // Verify Tor Phase 2 methods (Feb 2026 - Pure Rust Tor Protocol)
    assert!(methods.contains(&"beardog.crypto.tor_ntor_client_init"));
    assert!(methods.contains(&"beardog.crypto.tor_ntor_client_finish"));
    assert!(methods.contains(&"beardog.crypto.tor_ntor_server_respond"));
    assert!(methods.contains(&"beardog.crypto.tor_cell_encrypt"));
    assert!(methods.contains(&"beardog.crypto.tor_cell_decrypt"));
    assert!(methods.contains(&"beardog.crypto.tor_kdf"));

    // Verify dot-separated semantic aliases (BD-01 / Wire Standard L2)
    assert!(methods.contains(&"crypto.ed25519.sign"));
    assert!(methods.contains(&"crypto.ed25519.verify"));
}

#[test]
fn test_handler_method_count() {
    let handler = CryptoHandler;
    assert_eq!(
        handler.methods().len(),
        101,
        "Should have exactly 101 crypto methods (Phase 1-8 + TLS 1.2 + Dark Forest + Device Enrollment + Onion Service + Tor v3 + Tor Phase 2 + derive_lineage_beacon_key + dot-separated ed25519 aliases + crypto.public_key + lineage.list/verify/get + crypto.derive_purpose_key + crypto.sign_registration)"
    );
}
