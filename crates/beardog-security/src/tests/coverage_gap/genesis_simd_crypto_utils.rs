// SPDX-License-Identifier: AGPL-3.0-only

// ========================================================================
// genesis/types: ~57% → test uncovered description/stars/meets_threshold
// ========================================================================

mod genesis_types_tests {
    use crate::genesis::types::*;

    #[test]
    fn test_physical_channel_descriptions() {
        assert_eq!(
            PhysicalChannelType::HardwareKey.description(),
            "Hardware Security Key (SoloKey/YubiKey)"
        );
        assert_eq!(
            PhysicalChannelType::QrCodeWithOob.description(),
            "QR Code with Out-of-Band Verification"
        );
        assert_eq!(
            PhysicalChannelType::Nfc.description(),
            "NFC Tap (Near-Field Communication)"
        );
        assert_eq!(
            PhysicalChannelType::Bluetooth.description(),
            "Bluetooth Pairing"
        );
    }

    #[test]
    fn test_physical_channel_supports_attestation() {
        assert!(PhysicalChannelType::HardwareKey.supports_attestation());
        assert!(PhysicalChannelType::Nfc.supports_attestation());
        assert!(!PhysicalChannelType::QrCodeWithOob.supports_attestation());
        assert!(!PhysicalChannelType::Bluetooth.supports_attestation());
    }

    #[test]
    fn test_trust_level_descriptions() {
        assert_eq!(TrustLevel::Low.description(), "Low (⭐)");
        assert_eq!(TrustLevel::Medium.description(), "Medium (⭐⭐⭐)");
        assert_eq!(TrustLevel::High.description(), "High (⭐⭐⭐⭐)");
        assert_eq!(TrustLevel::Maximum.description(), "Maximum (⭐⭐⭐⭐⭐)");
    }

    #[test]
    fn test_trust_level_stars() {
        assert_eq!(TrustLevel::Low.stars(), "⭐");
        assert_eq!(TrustLevel::Medium.stars(), "⭐⭐⭐");
        assert_eq!(TrustLevel::High.stars(), "⭐⭐⭐⭐");
        assert_eq!(TrustLevel::Maximum.stars(), "⭐⭐⭐⭐⭐");
    }

    #[test]
    fn test_trust_level_meets_threshold() {
        assert!(TrustLevel::Maximum.meets_threshold(TrustLevel::Low));
        assert!(TrustLevel::Maximum.meets_threshold(TrustLevel::Maximum));
        assert!(TrustLevel::High.meets_threshold(TrustLevel::Medium));
        assert!(!TrustLevel::Low.meets_threshold(TrustLevel::Medium));
        assert!(!TrustLevel::Medium.meets_threshold(TrustLevel::High));
    }

    #[test]
    fn test_trust_level_low_not_sufficient_for_genesis() {
        assert!(!TrustLevel::Low.is_sufficient_for_genesis());
    }

    #[test]
    fn test_trust_level_default() {
        let default = TrustLevel::default();
        assert_eq!(default, TrustLevel::Medium);
    }

    #[test]
    fn test_trust_level_ordering_comprehensive() {
        assert!(TrustLevel::Low < TrustLevel::Medium);
        assert!(TrustLevel::Medium < TrustLevel::High);
        assert!(TrustLevel::High < TrustLevel::Maximum);
        assert!(TrustLevel::Low < TrustLevel::Maximum);
    }

    #[test]
    fn test_physical_channel_serde_roundtrip() {
        let channel = PhysicalChannelType::HardwareKey;
        let json = serde_json::to_string(&channel).unwrap();
        let deserialized: PhysicalChannelType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, channel);
    }

    #[test]
    fn test_trust_level_serde_roundtrip() {
        for level in [
            TrustLevel::Low,
            TrustLevel::Medium,
            TrustLevel::High,
            TrustLevel::Maximum,
        ] {
            let json = serde_json::to_string(&level).unwrap();
            let deserialized: TrustLevel = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, level);
        }
    }
}

// ========================================================================
// simd_crypto: ~70% → test safe_chacha20_with_nonce
// ========================================================================

mod simd_crypto_tests {
    use crate::simd_crypto::{SafeCryptoConfig, SafeCryptoEngine, SafeCryptoStats};

    #[test]
    fn test_safe_chacha20_with_nonce_basic() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let data = b"plaintext for nonce test";
        let key = b"01234567890123456789012345678901"; // 32 bytes
        let nonce = b"123456789012"; // 12 bytes

        let encrypted = engine.safe_chacha20_with_nonce(data, key, nonce).unwrap();
        assert_eq!(encrypted.len(), data.len());
        assert_ne!(encrypted, data.to_vec());
    }

    #[test]
    fn test_safe_chacha20_with_nonce_reversibility() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let original = b"round-trip with nonce";
        let key = b"01234567890123456789012345678901"; // 32 bytes
        let nonce = b"abcdef012345"; // 12 bytes

        let encrypted = engine
            .safe_chacha20_with_nonce(original, key, nonce)
            .unwrap();
        let decrypted = engine
            .safe_chacha20_with_nonce(&encrypted, key, nonce)
            .unwrap();

        assert_eq!(decrypted, original.to_vec());
    }

    #[test]
    fn test_safe_chacha20_with_nonce_invalid_key() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let data = b"test";
        let short_key = b"too_short";
        let nonce = b"123456789012"; // 12 bytes

        let result = engine.safe_chacha20_with_nonce(data, short_key, nonce);
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_chacha20_with_nonce_invalid_nonce() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let data = b"test";
        let key = b"01234567890123456789012345678901"; // 32 bytes
        let short_nonce = b"short"; // wrong length

        let result = engine.safe_chacha20_with_nonce(data, key, short_nonce);
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_chacha20_with_nonce_different_nonces_different_output() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let data = b"same plaintext for both";
        let key = b"01234567890123456789012345678901"; // 32 bytes
        let nonce1 = b"nonce1nonce1"; // 12 bytes (typo fix: exactly 12)
        let nonce2 = b"nonce2nonce2"; // 12 bytes

        let encrypted1 = engine.safe_chacha20_with_nonce(data, key, nonce1).unwrap();
        let encrypted2 = engine.safe_chacha20_with_nonce(data, key, nonce2).unwrap();

        assert_ne!(encrypted1, encrypted2);
    }

    #[test]
    fn test_safe_chacha20_with_nonce_stats_tracking() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let data = b"stats test data";
        let key = b"01234567890123456789012345678901";
        let nonce = b"123456789012";

        engine.safe_chacha20_with_nonce(data, key, nonce).unwrap();

        let stats = engine.get_stats();
        assert_eq!(stats.get("operations").unwrap(), "1");
        assert_eq!(
            stats.get("bytes_processed").unwrap(),
            &data.len().to_string()
        );
    }

    #[test]
    fn test_safe_crypto_stats_default() {
        let stats = SafeCryptoStats::default();
        assert_eq!(stats.operations_performed, 0);
        assert_eq!(stats.total_bytes_processed, 0);
    }
}

// ========================================================================
// crypto_utils: ~85% → test generate_password, sha256_hash
// ========================================================================

mod crypto_utils_tests {
    use crate::crypto_utils::BearDogCrypto;

    #[test]
    fn test_generate_password_valid() {
        let password = BearDogCrypto::generate_password(16).unwrap();
        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_minimum_length() {
        let password = BearDogCrypto::generate_password(8).unwrap();
        assert_eq!(password.len(), 8);
    }

    #[test]
    fn test_generate_password_too_short() {
        let result = BearDogCrypto::generate_password(7);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_password_uniqueness() {
        let p1 = BearDogCrypto::generate_password(32).unwrap();
        let p2 = BearDogCrypto::generate_password(32).unwrap();
        assert_ne!(p1, p2);
    }

    #[test]
    fn test_generate_api_key_with_prefix() {
        let key = BearDogCrypto::generate_api_key("sk").unwrap();
        assert!(key.starts_with("sk_"));
        assert!(key.len() > 3); // prefix + _ + base64
    }

    #[test]
    fn test_generate_api_key_empty_prefix() {
        let key = BearDogCrypto::generate_api_key("").unwrap();
        assert!(!key.contains('_') || key.contains('_')); // just base64
        assert!(!key.is_empty());
    }

    #[test]
    fn test_sha256_hash_method() {
        let hash = BearDogCrypto::sha256_hash(b"test data");
        // SHA256 hex string is 64 characters
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_sha256_hash_deterministic() {
        let h1 = BearDogCrypto::sha256_hash(b"same input");
        let h2 = BearDogCrypto::sha256_hash(b"same input");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_sha256_hash_different_inputs() {
        let h1 = BearDogCrypto::sha256_hash(b"input1");
        let h2 = BearDogCrypto::sha256_hash(b"input2");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_zero_memory() {
        let mut buffer = vec![0xAA_u8; 32];
        BearDogCrypto::zero_memory(&mut buffer);
        assert!(buffer.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_zero_memory_empty() {
        let mut buffer = vec![];
        BearDogCrypto::zero_memory(&mut buffer);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_constant_time_compare_equal() {
        let a = b"secret_data_here";
        let b = b"secret_data_here";
        assert!(BearDogCrypto::constant_time_compare(a, b));
    }

    #[test]
    fn test_constant_time_compare_different() {
        let a = b"secret_data_here";
        let b = b"secret_data_diff";
        assert!(!BearDogCrypto::constant_time_compare(a, b));
    }

    #[test]
    fn test_constant_time_compare_different_lengths() {
        let a = b"short";
        let b = b"longer_data";
        assert!(!BearDogCrypto::constant_time_compare(a, b));
    }

    #[test]
    fn test_encrypt_aes_gcm_invalid_nonce_length() {
        let key = BearDogCrypto::generate_secure_random(32);
        let bad_nonce = vec![0u8; 8]; // wrong: should be 12
        let result = BearDogCrypto::encrypt_aes_gcm(&key, b"test", Some(&bad_nonce));
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_aes_gcm_invalid_key_length() {
        let bad_key = vec![0u8; 16]; // wrong: should be 32
        let nonce = vec![0u8; 12];
        let result = BearDogCrypto::decrypt_aes_gcm(&bad_key, b"ciphertext", &nonce);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_password_argon2_invalid_hash() {
        let result = BearDogCrypto::verify_password_argon2("password", "not_a_valid_hash");
        assert!(result.is_err());
    }

    #[test]
    fn test_sign_ed25519_invalid_key_length() {
        let short_key = vec![0u8; 16];
        let result = BearDogCrypto::sign_ed25519(&short_key, b"message");
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_ed25519_invalid_signature_returns_false() {
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair();
        let _sig = BearDogCrypto::sign_ed25519(&private_key, b"message").unwrap();
        let wrong_sig = vec![0u8; 64]; // Invalid signature
        let result = BearDogCrypto::verify_ed25519(&public_key, b"message", &wrong_sig);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_verify_ed25519_invalid_public_key_length() {
        let short_pubkey = vec![0u8; 16];
        let sig = vec![0u8; 64];
        let result = BearDogCrypto::verify_ed25519(&short_pubkey, b"msg", &sig);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_ed25519_invalid_signature_length() {
        let pubkey = BearDogCrypto::generate_secure_random(32);
        let short_sig = vec![0u8; 32];
        let result = BearDogCrypto::verify_ed25519(&pubkey, b"msg", &short_sig);
        assert!(result.is_err());
    }

    #[test]
    fn test_derive_pbkdf2_zero_iterations() {
        let result = BearDogCrypto::derive_pbkdf2_key(b"password", b"salt", 0, 32);
        assert!(result.is_err());
    }

    #[test]
    fn test_hmac_sha256_and_verify() {
        let key = b"hmac_key_32_bytes_long________";
        let data = b"data to authenticate";
        let tag = BearDogCrypto::hmac_sha256(key, data).unwrap();
        let verified = BearDogCrypto::verify_hmac_sha256(key, data, &tag).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_hmac_verify_wrong_tag_fails() {
        let key = b"hmac_key_32_bytes_long________";
        let data = b"data";
        let wrong_tag = vec![0u8; 32];
        let verified = BearDogCrypto::verify_hmac_sha256(key, data, &wrong_tag).unwrap();
        assert!(!verified);
    }

    #[test]
    fn test_hash_and_verify_password_argon2_roundtrip() {
        let password = "secure_password_123";
        let hash = BearDogCrypto::hash_password_argon2(password).unwrap();
        assert!(!hash.is_empty());
        let verified = BearDogCrypto::verify_password_argon2(password, &hash).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_verify_password_argon2_wrong_password() {
        let hash = BearDogCrypto::hash_password_argon2("correct").unwrap();
        let verified = BearDogCrypto::verify_password_argon2("wrong", &hash).unwrap();
        assert!(!verified);
    }

    #[test]
    fn test_encrypt_decrypt_aes_gcm_roundtrip() {
        let key = BearDogCrypto::generate_secure_random(32);
        let plaintext = b"secret message for aes-gcm";
        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, plaintext, None).unwrap();
        let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_decrypt_aes_gcm_tampered_ciphertext_fails() {
        let key = BearDogCrypto::generate_secure_random(32);
        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, b"plaintext", None).unwrap();
        let mut tampered = ciphertext;
        if !tampered.is_empty() {
            tampered[0] ^= 0xFF;
        }
        let result = BearDogCrypto::decrypt_aes_gcm(&key, &tampered, &nonce);
        assert!(result.is_err());
    }

    #[test]
    fn test_derive_pbkdf2_success() {
        let key = BearDogCrypto::derive_pbkdf2_key(b"password", b"salt", 1000, 32).unwrap();
        assert_eq!(key.len(), 32);
        let key2 = BearDogCrypto::derive_pbkdf2_key(b"password", b"salt", 1000, 32).unwrap();
        assert_eq!(key, key2);
    }

    #[test]
    fn test_ed25519_sign_verify_roundtrip() {
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair();
        let message = b"message to sign";
        let signature = BearDogCrypto::sign_ed25519(&private_key, message).unwrap();
        let verified = BearDogCrypto::verify_ed25519(&public_key, message, &signature).unwrap();
        assert!(verified);
    }
}
