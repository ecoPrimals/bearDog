// SPDX-License-Identifier: AGPL-3.0-or-later

// Comprehensive tests for RustCrypto provider
//
// This module provides comprehensive test coverage for all cryptographic operations
// implemented by the RustCrypto provider, including encryption, signatures, hashing, and KDF.

use super::rustcrypto::RustCryptoProvider;
use crate::tunnel::hsm::crypto::algorithms::*;
use crate::tunnel::hsm::crypto::provider::{NonceGenerator, UniversalCryptoProvider};

// ============================================================================
// PROVIDER BASICS TESTS
// ============================================================================

#[cfg(test)]
mod provider_basics_tests {
    use super::*;

    #[test]
    fn test_provider_new() {
        let provider = RustCryptoProvider::new();
        assert_eq!(provider.provider_name(), "RustCrypto");
    }

    #[test]
    fn test_provider_default() {
        let provider = RustCryptoProvider::default();
        assert_eq!(provider.provider_name(), "RustCrypto");
    }

    #[test]
    fn test_provider_name() {
        let provider = RustCryptoProvider::new();
        assert_eq!(provider.provider_name(), "RustCrypto");
    }

    #[test]
    fn test_provider_version() {
        let provider = RustCryptoProvider::new();
        let version = provider.provider_version();
        assert!(!version.is_empty());
        // Should be semantic version format
        assert!(version.contains('.'));
    }

    #[tokio::test]
    async fn test_discover_capabilities() {
        let provider = RustCryptoProvider::new();
        let caps = provider.discover_capabilities().await.unwrap();

        assert_eq!(caps.provider_name, "RustCrypto");
        assert!(!caps.provider_version.is_empty());

        // Should have symmetric algorithms
        assert!(!caps.symmetric_algorithms.is_empty());
        assert!(
            caps.symmetric_algorithms
                .contains(&SymmetricAlgorithm::Aes256Gcm)
        );
        assert!(
            caps.symmetric_algorithms
                .contains(&SymmetricAlgorithm::ChaCha20Poly1305)
        );

        // Should have signature algorithms
        assert!(!caps.signature_algorithms.is_empty());
        assert!(
            caps.signature_algorithms
                .contains(&SignatureAlgorithm::Ed25519)
        );

        // Should have hash algorithms
        assert!(!caps.hash_algorithms.is_empty());
        assert!(caps.hash_algorithms.contains(&HashAlgorithm::Sha256));

        // Should have KDF algorithms
        assert!(!caps.kdf_algorithms.is_empty());
        assert!(caps.kdf_algorithms.contains(&KdfAlgorithm::HkdfSha256));
    }

    #[tokio::test]
    async fn test_supports_algorithm_symmetric() {
        let provider = RustCryptoProvider::new();

        // Supported
        let alg = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes256Gcm);
        assert!(provider.supports_algorithm(&alg).await);

        let alg = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::ChaCha20Poly1305);
        assert!(provider.supports_algorithm(&alg).await);
    }

    #[tokio::test]
    async fn test_supports_algorithm_signature() {
        let provider = RustCryptoProvider::new();

        let alg = CryptoAlgorithm::Signature(SignatureAlgorithm::Ed25519);
        assert!(provider.supports_algorithm(&alg).await);

        let alg = CryptoAlgorithm::Signature(SignatureAlgorithm::EcdsaP256 {
            hash: HashAlgorithm::Sha256,
        });
        assert!(provider.supports_algorithm(&alg).await);
    }

    #[tokio::test]
    async fn test_supports_algorithm_hash() {
        let provider = RustCryptoProvider::new();

        let alg = CryptoAlgorithm::Hash(HashAlgorithm::Sha256);
        assert!(provider.supports_algorithm(&alg).await);

        let alg = CryptoAlgorithm::Hash(HashAlgorithm::Blake3);
        assert!(provider.supports_algorithm(&alg).await);
    }

    #[tokio::test]
    async fn test_supports_algorithm_asymmetric_not_supported() {
        let provider = RustCryptoProvider::new();

        let alg = CryptoAlgorithm::Asymmetric(AsymmetricAlgorithm::EciesP256);
        assert!(!provider.supports_algorithm(&alg).await);
    }

    #[test]
    fn test_provider_clone() {
        let provider1 = RustCryptoProvider::new();
        let provider2 = provider1.clone();
        assert_eq!(provider1.provider_name(), provider2.provider_name());
    }
}

// ============================================================================
// SYMMETRIC ENCRYPTION TESTS (AES-256-GCM)
// ============================================================================

#[cfg(test)]
mod aes_256_gcm_tests {
    use super::*;

    fn create_test_key_256() -> Vec<u8> {
        vec![0x42; 32] // 256-bit key
    }

    #[tokio::test]
    async fn test_aes_256_gcm_encrypt_decrypt_round_trip() {
        let provider = RustCryptoProvider::new();
        let key = create_test_key_256();
        let plaintext = b"Hello, World! This is a test message.";

        let alg = SymmetricAlgorithm::Aes256Gcm;
        let opts = EncryptionOptions::default();

        // Encrypt
        let encrypted = provider
            .encrypt_symmetric(alg.clone(), &key, plaintext, &opts)
            .await
            .unwrap();
        assert!(!encrypted.ciphertext.is_empty());
        assert!(encrypted.nonce.is_some());
        assert_eq!(encrypted.algorithm, "AES-256-GCM");

        // Decrypt
        let decrypt_opts = DecryptionOptions::default();
        let decrypted = provider
            .decrypt_symmetric(alg, &key, &encrypted, &decrypt_opts)
            .await
            .unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[tokio::test]
    async fn test_aes_256_gcm_with_custom_nonce() {
        let provider = RustCryptoProvider::new();
        let key = create_test_key_256();
        let plaintext = b"Custom nonce test";
        let custom_nonce = vec![0x11; 12]; // 96-bit nonce

        let alg = SymmetricAlgorithm::Aes256Gcm;
        let opts = EncryptionOptions {
            nonce: Some(custom_nonce.clone()),
            associated_data: None,
        };

        let encrypted = provider
            .encrypt_symmetric(alg, &key, plaintext, &opts)
            .await
            .unwrap();
        assert_eq!(encrypted.nonce, Some(custom_nonce));
    }

    #[tokio::test]
    async fn test_aes_256_gcm_invalid_key_length() {
        let provider = RustCryptoProvider::new();
        let bad_key = vec![0x42; 16]; // Wrong size (should be 32)
        let plaintext = b"Test";

        let alg = SymmetricAlgorithm::Aes256Gcm;
        let opts = EncryptionOptions::default();

        let result = provider
            .encrypt_symmetric(alg, &bad_key, plaintext, &opts)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_aes_256_gcm_decrypt_without_nonce() {
        let provider = RustCryptoProvider::new();
        let key = create_test_key_256();

        let encrypted = EncryptedData {
            algorithm: "AES-256-GCM".to_string(),
            ciphertext: vec![1, 2, 3],
            nonce: None, // Missing nonce
            tag: None,
        };

        let alg = SymmetricAlgorithm::Aes256Gcm;
        let opts = DecryptionOptions::default();
        let result = provider
            .decrypt_symmetric(alg, &key, &encrypted, &opts)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_aes_256_gcm_tampered_ciphertext() {
        let provider = RustCryptoProvider::new();
        let key = create_test_key_256();
        let plaintext = b"Original message";

        let alg = SymmetricAlgorithm::Aes256Gcm;
        let opts = EncryptionOptions::default();

        // Encrypt
        let mut encrypted = provider
            .encrypt_symmetric(alg.clone(), &key, plaintext, &opts)
            .await
            .unwrap();

        // Tamper with ciphertext
        if !encrypted.ciphertext.is_empty() {
            encrypted.ciphertext[0] ^= 0xFF;
        }

        // Decrypt should fail (authentication failure)
        let decrypt_opts = DecryptionOptions::default();
        let result = provider
            .decrypt_symmetric(alg, &key, &encrypted, &decrypt_opts)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_aes_256_gcm_empty_plaintext() {
        let provider = RustCryptoProvider::new();
        let key = create_test_key_256();
        let plaintext = b"";

        let alg = SymmetricAlgorithm::Aes256Gcm;
        let opts = EncryptionOptions::default();

        let encrypted = provider
            .encrypt_symmetric(alg.clone(), &key, plaintext, &opts)
            .await
            .unwrap();

        let decrypt_opts = DecryptionOptions::default();
        let decrypted = provider
            .decrypt_symmetric(alg, &key, &encrypted, &decrypt_opts)
            .await
            .unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[tokio::test]
    async fn test_aes_256_gcm_large_plaintext() {
        let provider = RustCryptoProvider::new();
        let key = create_test_key_256();
        let plaintext = vec![0x42; 10000]; // 10KB

        let alg = SymmetricAlgorithm::Aes256Gcm;
        let opts = EncryptionOptions::default();

        let encrypted = provider
            .encrypt_symmetric(alg.clone(), &key, &plaintext, &opts)
            .await
            .unwrap();

        let decrypt_opts = DecryptionOptions::default();
        let decrypted = provider
            .decrypt_symmetric(alg, &key, &encrypted, &decrypt_opts)
            .await
            .unwrap();
        assert_eq!(decrypted, plaintext);
    }
}

// ============================================================================
// SYMMETRIC ENCRYPTION TESTS (AES-128-GCM)
// ============================================================================

#[cfg(test)]
mod aes_128_gcm_tests {
    use super::*;

    fn create_test_key_128() -> Vec<u8> {
        vec![0x42; 16] // 128-bit key
    }

    #[tokio::test]
    async fn test_aes_128_gcm_encrypt_decrypt_round_trip() {
        let provider = RustCryptoProvider::new();
        let key = create_test_key_128();
        let plaintext = b"AES-128 test message";

        let alg = SymmetricAlgorithm::Aes128Gcm;
        let opts = EncryptionOptions::default();

        let encrypted = provider
            .encrypt_symmetric(alg.clone(), &key, plaintext, &opts)
            .await
            .unwrap();
        assert_eq!(encrypted.algorithm, "AES-128-GCM");

        let decrypt_opts = DecryptionOptions::default();
        let decrypted = provider
            .decrypt_symmetric(alg, &key, &encrypted, &decrypt_opts)
            .await
            .unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[tokio::test]
    async fn test_aes_128_gcm_invalid_key_length() {
        let provider = RustCryptoProvider::new();
        let bad_key = vec![0x42; 32]; // Wrong size (should be 16)
        let plaintext = b"Test";

        let alg = SymmetricAlgorithm::Aes128Gcm;
        let opts = EncryptionOptions::default();

        let result = provider
            .encrypt_symmetric(alg, &bad_key, plaintext, &opts)
            .await;
        assert!(result.is_err());
    }
}

// ============================================================================
// SYMMETRIC ENCRYPTION TESTS (ChaCha20-Poly1305)
// ============================================================================

#[cfg(test)]
mod chacha20_poly1305_tests {
    use super::*;

    fn create_test_key_chacha() -> Vec<u8> {
        vec![0x42; 32] // 256-bit key
    }

    #[tokio::test]
    async fn test_chacha20_encrypt_decrypt_round_trip() {
        let provider = RustCryptoProvider::new();
        let key = create_test_key_chacha();
        let plaintext = b"ChaCha20-Poly1305 test";

        let alg = SymmetricAlgorithm::ChaCha20Poly1305;
        let opts = EncryptionOptions::default();

        let encrypted = provider
            .encrypt_symmetric(alg.clone(), &key, plaintext, &opts)
            .await
            .unwrap();
        assert_eq!(encrypted.algorithm, "ChaCha20-Poly1305");

        let decrypt_opts = DecryptionOptions::default();
        let decrypted = provider
            .decrypt_symmetric(alg, &key, &encrypted, &decrypt_opts)
            .await
            .unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[tokio::test]
    async fn test_chacha20_invalid_key_length() {
        let provider = RustCryptoProvider::new();
        let bad_key = vec![0x42; 16]; // Wrong size (should be 32)
        let plaintext = b"Test";

        let alg = SymmetricAlgorithm::ChaCha20Poly1305;
        let opts = EncryptionOptions::default();

        let result = provider
            .encrypt_symmetric(alg, &bad_key, plaintext, &opts)
            .await;
        assert!(result.is_err());
    }
}

// ============================================================================
// ASYMMETRIC ENCRYPTION TESTS (Not Implemented)
// ============================================================================

#[cfg(test)]
mod asymmetric_tests {
    use super::*;

    #[tokio::test]
    async fn test_encrypt_asymmetric_not_implemented() {
        let provider = RustCryptoProvider::new();
        let alg = AsymmetricAlgorithm::EciesP256;
        let opts = EncryptionOptions::default();

        let result = provider.encrypt_asymmetric(alg, &[], b"test", &opts).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_decrypt_asymmetric_not_implemented() {
        let provider = RustCryptoProvider::new();
        let alg = AsymmetricAlgorithm::EciesP256;
        let encrypted = EncryptedData {
            algorithm: "ECIES-P256".to_string(),
            ciphertext: vec![],
            nonce: None,
            tag: None,
        };
        let opts = DecryptionOptions::default();

        let result = provider
            .decrypt_asymmetric(alg, &[], &encrypted, &opts)
            .await;
        assert!(result.is_err());
    }
}

// ============================================================================
// SIGNATURE TESTS (Ed25519)
// ============================================================================

#[cfg(test)]
mod ed25519_signature_tests {
    use super::*;

    fn create_test_keypair_ed25519() -> (Vec<u8>, Vec<u8>) {
        use ed25519_dalek::SigningKey;
        // Create a signing key from seed bytes
        let mut seed = [0u8; 32];
        seed[0] = 0x42;
        seed[31] = 0x24; // Add some variation to make valid key
        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();
        (
            signing_key.to_bytes().to_vec(),
            verifying_key.to_bytes().to_vec(),
        )
    }

    #[tokio::test]
    async fn test_ed25519_sign_verify_round_trip() {
        let provider = RustCryptoProvider::new();
        let (private_key, _public_key) = create_test_keypair_ed25519();
        let message = b"Test message for Ed25519";

        let alg = SignatureAlgorithm::Ed25519;
        let sign_opts = SigningOptions::default();

        // Sign
        let signature = provider
            .sign(alg.clone(), &private_key, message, &sign_opts)
            .await
            .unwrap();
        assert_eq!(signature.algorithm, "Ed25519");
        assert_eq!(signature.signature.len(), 64);

        // Verify with private key (should derive public key automatically)
        let verify_opts = VerificationOptions::default();
        let valid = provider
            .verify(alg, &private_key, message, &signature, &verify_opts)
            .await
            .unwrap();
        assert!(valid);
    }

    #[tokio::test]
    async fn test_ed25519_verify_invalid_signature() {
        let provider = RustCryptoProvider::new();
        let (_, public_key) = create_test_keypair_ed25519();
        let message = b"Test message";

        let invalid_signature = Signature {
            algorithm: "Ed25519".to_string(),
            signature: vec![0x00; 64], // Invalid signature
        };

        let alg = SignatureAlgorithm::Ed25519;
        let opts = VerificationOptions::default();
        let valid = provider
            .verify(alg, &public_key, message, &invalid_signature, &opts)
            .await
            .unwrap();
        assert!(!valid);
    }

    #[tokio::test]
    async fn test_ed25519_verify_wrong_message() {
        let provider = RustCryptoProvider::new();
        let (private_key, public_key) = create_test_keypair_ed25519();
        let message1 = b"Original message";
        let message2 = b"Different message";

        let alg = SignatureAlgorithm::Ed25519;
        let sign_opts = SigningOptions::default();
        let signature = provider
            .sign(alg.clone(), &private_key, message1, &sign_opts)
            .await
            .unwrap();

        let verify_opts = VerificationOptions::default();
        let valid = provider
            .verify(alg, &public_key, message2, &signature, &verify_opts)
            .await
            .unwrap();
        assert!(!valid);
    }

    #[tokio::test]
    async fn test_ed25519_sign_invalid_key_length() {
        let provider = RustCryptoProvider::new();
        let bad_key = vec![0x42; 16]; // Wrong size (should be 32)
        let message = b"Test";

        let alg = SignatureAlgorithm::Ed25519;
        let opts = SigningOptions::default();
        let result = provider.sign(alg, &bad_key, message, &opts).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// SIGNATURE TESTS (ECDSA P-256)
// ============================================================================

#[cfg(test)]
mod ecdsa_p256_signature_tests {
    use super::*;

    fn create_test_keypair_p256() -> (Vec<u8>, Vec<u8>) {
        use p256::SecretKey;
        use p256::ecdsa::SigningKey;

        let secret_key = SecretKey::from_bytes((&[0x42; 32]).into()).unwrap();
        let signing_key = SigningKey::from(secret_key);
        let verifying_key = signing_key.verifying_key();

        (
            signing_key.to_bytes().to_vec(),
            verifying_key.to_sec1_bytes().to_vec(),
        )
    }

    #[tokio::test]
    async fn test_ecdsa_p256_sign_verify_round_trip() {
        let provider = RustCryptoProvider::new();
        let (private_key, public_key) = create_test_keypair_p256();
        let message = b"ECDSA P-256 test message";

        let alg = SignatureAlgorithm::EcdsaP256 {
            hash: HashAlgorithm::Sha256,
        };
        let sign_opts = SigningOptions::default();

        // Sign
        let signature = provider
            .sign(alg.clone(), &private_key, message, &sign_opts)
            .await
            .unwrap();
        assert_eq!(signature.algorithm, "ECDSA-P256-SHA256");

        // Verify
        let verify_opts = VerificationOptions::default();
        let valid = provider
            .verify(alg, &public_key, message, &signature, &verify_opts)
            .await
            .unwrap();
        assert!(valid);
    }

    #[tokio::test]
    async fn test_ecdsa_p256_verify_invalid_signature() {
        let provider = RustCryptoProvider::new();
        let (_, public_key) = create_test_keypair_p256();
        let message = b"Test";

        let invalid_signature = Signature {
            algorithm: "ECDSA-P256-SHA256".to_string(),
            signature: vec![0x00; 64],
        };

        let alg = SignatureAlgorithm::EcdsaP256 {
            hash: HashAlgorithm::Sha256,
        };
        let opts = VerificationOptions::default();
        // Invalid signature format causes an error, not Ok(false)
        let result = provider
            .verify(alg, &public_key, message, &invalid_signature, &opts)
            .await;
        assert!(result.is_err());
    }
}

// ============================================================================
// HASH TESTS
// ============================================================================

#[cfg(test)]
mod hash_tests {
    use super::*;

    #[tokio::test]
    async fn test_hash_sha256() {
        let provider = RustCryptoProvider::new();
        let data = b"Hello, World!";

        let hash = provider.hash(HashAlgorithm::Sha256, data).await.unwrap();
        assert_eq!(hash.len(), 32); // SHA-256 produces 32 bytes

        // Same input should produce same hash
        let hash2 = provider.hash(HashAlgorithm::Sha256, data).await.unwrap();
        assert_eq!(hash, hash2);
    }

    #[tokio::test]
    async fn test_hash_sha384() {
        let provider = RustCryptoProvider::new();
        let data = b"SHA-384 test";

        let hash = provider.hash(HashAlgorithm::Sha384, data).await.unwrap();
        assert_eq!(hash.len(), 48); // SHA-384 produces 48 bytes
    }

    #[tokio::test]
    async fn test_hash_sha512() {
        let provider = RustCryptoProvider::new();
        let data = b"SHA-512 test";

        let hash = provider.hash(HashAlgorithm::Sha512, data).await.unwrap();
        assert_eq!(hash.len(), 64); // SHA-512 produces 64 bytes
    }

    #[tokio::test]
    async fn test_hash_blake3() {
        let provider = RustCryptoProvider::new();
        let data = b"BLAKE3 test";

        let hash = provider.hash(HashAlgorithm::Blake3, data).await.unwrap();
        assert_eq!(hash.len(), 32); // BLAKE3 produces 32 bytes by default
    }

    #[tokio::test]
    async fn test_hash_unsupported_algorithm() {
        let provider = RustCryptoProvider::new();
        let data = b"Test";

        let result = provider.hash(HashAlgorithm::Sha3_256, data).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_hash_empty_data() {
        let provider = RustCryptoProvider::new();
        let data = b"";

        let hash = provider.hash(HashAlgorithm::Sha256, data).await.unwrap();
        assert_eq!(hash.len(), 32);
    }

    #[tokio::test]
    async fn test_hash_large_data() {
        let provider = RustCryptoProvider::new();
        let data = vec![0x42; 100000]; // 100KB

        let hash = provider.hash(HashAlgorithm::Sha256, &data).await.unwrap();
        assert_eq!(hash.len(), 32);
    }

    #[tokio::test]
    async fn test_hash_deterministic() {
        let provider = RustCryptoProvider::new();
        let data = b"Determinism test";

        let hash1 = provider.hash(HashAlgorithm::Sha256, data).await.unwrap();
        let hash2 = provider.hash(HashAlgorithm::Sha256, data).await.unwrap();
        let hash3 = provider.hash(HashAlgorithm::Sha256, data).await.unwrap();

        assert_eq!(hash1, hash2);
        assert_eq!(hash2, hash3);
    }
}

// ============================================================================
// KDF TESTS (HKDF)
// ============================================================================

#[cfg(test)]
mod kdf_tests {
    use super::*;

    #[tokio::test]
    async fn test_hkdf_sha256() {
        let provider = RustCryptoProvider::new();
        let input_key = b"input key material";
        let salt = b"salt value";
        let info = b"context info";
        let output_length = 32;

        let derived = provider
            .derive_key(
                KdfAlgorithm::HkdfSha256,
                input_key,
                salt,
                info,
                output_length,
            )
            .await
            .unwrap();

        assert_eq!(derived.len(), output_length);
    }

    #[tokio::test]
    async fn test_hkdf_sha384() {
        let provider = RustCryptoProvider::new();
        let input_key = b"input key material";
        let salt = b"salt value";
        let info = b"context info";
        let output_length = 48;

        let derived = provider
            .derive_key(
                KdfAlgorithm::HkdfSha384,
                input_key,
                salt,
                info,
                output_length,
            )
            .await
            .unwrap();

        assert_eq!(derived.len(), output_length);
    }

    #[tokio::test]
    async fn test_hkdf_sha512() {
        let provider = RustCryptoProvider::new();
        let input_key = b"input key material";
        let salt = b"salt value";
        let info = b"context info";
        let output_length = 64;

        let derived = provider
            .derive_key(
                KdfAlgorithm::HkdfSha512,
                input_key,
                salt,
                info,
                output_length,
            )
            .await
            .unwrap();

        assert_eq!(derived.len(), output_length);
    }

    #[tokio::test]
    async fn test_hkdf_deterministic() {
        let provider = RustCryptoProvider::new();
        let input_key = b"input key material";
        let salt = b"salt value";
        let info = b"context info";
        let output_length = 32;

        let derived1 = provider
            .derive_key(
                KdfAlgorithm::HkdfSha256,
                input_key,
                salt,
                info,
                output_length,
            )
            .await
            .unwrap();

        let derived2 = provider
            .derive_key(
                KdfAlgorithm::HkdfSha256,
                input_key,
                salt,
                info,
                output_length,
            )
            .await
            .unwrap();

        assert_eq!(derived1, derived2);
    }

    #[tokio::test]
    async fn test_hkdf_different_info_produces_different_keys() {
        let provider = RustCryptoProvider::new();
        let input_key = b"input key material";
        let salt = b"salt value";
        let info1 = b"context info 1";
        let info2 = b"context info 2";
        let output_length = 32;

        let derived1 = provider
            .derive_key(
                KdfAlgorithm::HkdfSha256,
                input_key,
                salt,
                info1,
                output_length,
            )
            .await
            .unwrap();

        let derived2 = provider
            .derive_key(
                KdfAlgorithm::HkdfSha256,
                input_key,
                salt,
                info2,
                output_length,
            )
            .await
            .unwrap();

        assert_ne!(derived1, derived2);
    }

    #[tokio::test]
    async fn test_hkdf_empty_salt() {
        let provider = RustCryptoProvider::new();
        let input_key = b"input key material";
        let salt = b"";
        let info = b"context info";
        let output_length = 32;

        let derived = provider
            .derive_key(
                KdfAlgorithm::HkdfSha256,
                input_key,
                salt,
                info,
                output_length,
            )
            .await
            .unwrap();

        assert_eq!(derived.len(), output_length);
    }

    #[tokio::test]
    async fn test_kdf_unsupported_algorithm() {
        let provider = RustCryptoProvider::new();
        let input_key = b"test";
        let salt = b"salt";
        let info = b"info";

        let result = provider
            .derive_key(
                KdfAlgorithm::Pbkdf2 {
                    hash: HashAlgorithm::Sha256,
                    iterations: 10000,
                },
                input_key,
                salt,
                info,
                32,
            )
            .await;

        assert!(result.is_err());
    }
}

// ============================================================================
// NONCE GENERATION TESTS
// ============================================================================

#[cfg(test)]
mod nonce_tests {
    use super::*;

    #[test]
    fn test_generate_nonce() {
        let provider = RustCryptoProvider::new();
        let nonce = provider.generate_nonce(12);
        assert_eq!(nonce.len(), 12);
    }

    #[test]
    fn test_generate_nonce_different_sizes() {
        let provider = RustCryptoProvider::new();
        let sizes = vec![8, 12, 16, 24, 32];

        for size in sizes {
            let nonce = provider.generate_nonce(size);
            assert_eq!(nonce.len(), size);
        }
    }

    #[test]
    fn test_generate_nonce_uniqueness() {
        let provider = RustCryptoProvider::new();
        let nonce1 = provider.generate_nonce(12);
        let nonce2 = provider.generate_nonce(12);

        // Nonces should be different (with very high probability)
        assert_ne!(nonce1, nonce2);
    }

    #[test]
    fn test_generate_nonce_not_all_zeros() {
        let provider = RustCryptoProvider::new();
        let nonce = provider.generate_nonce(12);

        // At least one byte should be non-zero
        assert!(nonce.iter().any(|&b| b != 0));
    }
}
