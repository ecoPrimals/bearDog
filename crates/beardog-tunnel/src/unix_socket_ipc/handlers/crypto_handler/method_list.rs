// SPDX-License-Identifier: AGPL-3.0-or-later

//! Registered JSON-RPC method names for [`super::CryptoHandler`].

/// All crypto RPC method names (must stay in sync with routing).
pub fn crypto_method_names() -> Vec<&'static str> {
    vec![
        // Core crypto operations (EdDSA)
        "crypto.ed25519_generate_keypair",
        "crypto.sign_ed25519",
        "crypto.verify_ed25519",
        // ECDSA signature algorithms (TLS 1.3)
        "crypto.sign_ecdsa_secp256r1",
        "crypto.verify_ecdsa_secp256r1",
        "crypto.sign_ecdsa_secp384r1",
        "crypto.verify_ecdsa_secp384r1",
        // RSA signature algorithms (legacy + modern)
        "crypto.sign_rsa_pkcs1_sha256",
        "crypto.verify_rsa_pkcs1_sha256",
        "crypto.sign_rsa_pss_sha256",
        "crypto.verify_rsa_pss_sha256",
        // Key exchange (X25519 + ECDH)
        "crypto.x25519_generate_ephemeral",
        "crypto.x25519_derive_secret",
        "crypto.ecdh_p256_generate",
        "crypto.ecdh_p256_derive",
        "crypto.ecdh_p384_generate",
        "crypto.ecdh_p384_derive",
        // AEAD encryption
        "crypto.chacha20_poly1305_encrypt",
        "crypto.chacha20_poly1305_decrypt",
        "crypto.aes256_gcm_encrypt",
        "crypto.aes256_gcm_decrypt",
        "crypto.aes128_gcm_encrypt",
        "crypto.aes128_gcm_decrypt",
        // Hashing
        "crypto.blake3_hash",
        "crypto.hmac_sha256",
        "crypto.hash_for_cipher",
        "crypto.sha256",
        "crypto.sha384",
        "crypto.sha512",
        "crypto.sha1",
        "crypto.sha3_256",
        "crypto.derive_onion_address",
        "crypto.hmac_sha384",
        "crypto.hmac_sha512",
        "crypto.hmac_blake3",
        // Password hashing
        "crypto.argon2id_hash",
        "crypto.argon2id_verify",
        "crypto.pbkdf2_sha256",
        "crypto.bcrypt_hash",
        "crypto.bcrypt_verify",
        "crypto.scrypt",
        // TLS crypto operations
        "tls.derive_secrets",
        "tls.derive_handshake_secrets",
        "tls.derive_application_secrets",
        "tls.compute_finished_verify_data",
        "tls.sign_handshake",
        "tls.verify_certificate",
        // TLS 1.2 crypto operations
        "crypto.ecdhe.p256.generate",
        "crypto.ecdhe.p256.compute_shared",
        "crypto.ecdhe.p384.generate",
        "crypto.ecdhe.p384.compute_shared",
        "crypto.aead.aes_128_gcm.encrypt",
        "crypto.aead.aes_128_gcm.decrypt",
        "crypto.aead.aes_256_gcm.encrypt",
        "crypto.aead.aes_256_gcm.decrypt",
        "crypto.kdf.tls12_prf",
        // Genetic crypto operations (Phase 5)
        "genetic.derive_lineage_key",
        "genetic.derive_lineage_beacon_key",
        "genetic.mix_entropy",
        "genetic.verify_lineage",
        "genetic.generate_lineage_proof",
        "genetic.generate_challenge",
        "genetic.respond_to_challenge",
        "genetic.verify_challenge_response",
        // Device enrollment
        "genetic.derive_device_seed",
        "genetic.sign_lineage_certificate",
        "genetic.verify_lineage_certificate",
        // Lineage semantic methods (downstream thymic selection)
        "lineage.list",
        "lineage.verify",
        "lineage.get",
        // Semantic aliases
        "crypto.hash",
        "crypto.hmac",
        "crypto.sign",
        "crypto.verify",
        "crypto.public_key",
        "crypto.encrypt",
        "crypto.decrypt",
        "crypto.generate_keypair",
        "crypto.derive_secret",
        "crypto.derive_purpose_key",
        "crypto.derive_public_key",
        "crypto.sign_registration",
        // Dot-separated semantic names (SEMANTIC_METHOD_NAMING_STANDARD v2.0)
        "crypto.ed25519.sign",
        "crypto.ed25519.verify",
        // Cross-Primal namespace (beardog.crypto.*)
        "beardog.crypto.sha3_256",
        "beardog.crypto.ed25519_generate_keypair",
        "beardog.crypto.sign_ed25519",
        "beardog.crypto.verify_ed25519",
        "beardog.crypto.x25519_generate_ephemeral",
        "beardog.crypto.x25519_derive_secret",
        "beardog.crypto.chacha20_poly1305_encrypt",
        "beardog.crypto.chacha20_poly1305_decrypt",
        "beardog.crypto.hmac_sha256",
        "beardog.crypto.blake3_hash",
        "beardog.crypto.derive_onion_address",
        "beardog.crypto.generate_onion_identity",
        // Tor Phase 2
        "beardog.crypto.tor_ntor_client_init",
        "beardog.crypto.tor_ntor_client_finish",
        "beardog.crypto.tor_ntor_server_respond",
        "beardog.crypto.tor_cell_encrypt",
        "beardog.crypto.tor_cell_decrypt",
        "beardog.crypto.tor_kdf",
    ]
}

#[cfg(test)]
mod tests {
    use super::crypto_method_names;

    #[test]
    fn semantic_crypto_methods_registered_before_beardog_aliases() {
        let names = crypto_method_names();
        let i_sem = names
            .iter()
            .position(|&m| m == "crypto.sha3_256")
            .expect("crypto.sha3_256");
        let i_bd = names
            .iter()
            .position(|&m| m == "beardog.crypto.sha3_256")
            .expect("beardog.crypto.sha3_256");
        assert!(
            i_sem < i_bd,
            "semantic crypto.sha3_256 should appear before beardog.crypto.sha3_256 in method list"
        );
        assert!(names.contains(&"crypto.derive_onion_address"));
        assert!(names.contains(&"beardog.crypto.derive_onion_address"));
        assert!(names.contains(&"crypto.hash"));
        assert!(names.contains(&"beardog.crypto.blake3_hash"));
    }

    #[test]
    fn dot_separated_semantic_ed25519_methods_registered() {
        let names = crypto_method_names();
        assert!(
            names.contains(&"crypto.ed25519.sign"),
            "dot-separated crypto.ed25519.sign per SEMANTIC_METHOD_NAMING_STANDARD v2.0"
        );
        assert!(
            names.contains(&"crypto.ed25519.verify"),
            "dot-separated crypto.ed25519.verify per SEMANTIC_METHOD_NAMING_STANDARD v2.0"
        );
    }
}
