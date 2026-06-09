// SPDX-License-Identifier: AGPL-3.0-or-later

// Comprehensive tests for crypto algorithms
//
// This module provides comprehensive test coverage for all algorithm enums, display traits, and conversions.

use super::algorithms::*;
use std::collections::HashSet;

// ============================================================================
// CRYPTO ALGORITHM TESTS
// ============================================================================

#[cfg(test)]
mod crypto_algorithm_tests {
    use super::*;

    #[test]
    fn test_crypto_algorithm_symmetric() {
        let alg = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes256Gcm);
        assert!(matches!(alg, CryptoAlgorithm::Symmetric(_)));
    }

    #[test]
    fn test_crypto_algorithm_asymmetric() {
        let alg = CryptoAlgorithm::Asymmetric(AsymmetricAlgorithm::EciesP256);
        assert!(matches!(alg, CryptoAlgorithm::Asymmetric(_)));
    }

    #[test]
    fn test_crypto_algorithm_signature() {
        let alg = CryptoAlgorithm::Signature(SignatureAlgorithm::Ed25519);
        assert!(matches!(alg, CryptoAlgorithm::Signature(_)));
    }

    #[test]
    fn test_crypto_algorithm_hash() {
        let alg = CryptoAlgorithm::Hash(HashAlgorithm::Sha256);
        assert!(matches!(alg, CryptoAlgorithm::Hash(_)));
    }

    #[test]
    fn test_crypto_algorithm_kdf() {
        let alg = CryptoAlgorithm::Kdf(KdfAlgorithm::HkdfSha256);
        assert!(matches!(alg, CryptoAlgorithm::Kdf(_)));
    }

    #[test]
    fn test_as_symmetric_success() {
        let sym = SymmetricAlgorithm::Aes256Gcm;
        let alg = CryptoAlgorithm::Symmetric(sym.clone());
        let result = alg.as_symmetric();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), sym);
    }

    #[test]
    fn test_as_symmetric_failure() {
        let alg = CryptoAlgorithm::Hash(HashAlgorithm::Sha256);
        let result = alg.as_symmetric();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected symmetric algorithm")
        );
    }

    #[test]
    fn test_as_signature_success() {
        let sig = SignatureAlgorithm::Ed25519;
        let alg = CryptoAlgorithm::Signature(sig.clone());
        let result = alg.as_signature();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), sig);
    }

    #[test]
    fn test_as_signature_failure() {
        let alg = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes256Gcm);
        let result = alg.as_signature();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected signature algorithm")
        );
    }

    #[test]
    fn test_crypto_algorithm_display_symmetric() {
        let alg = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes256Gcm);
        assert_eq!(alg.to_string(), "AES-256-GCM");
    }

    #[test]
    fn test_crypto_algorithm_display_asymmetric() {
        let alg = CryptoAlgorithm::Asymmetric(AsymmetricAlgorithm::EciesP256);
        assert_eq!(alg.to_string(), "ECIES-P256");
    }

    #[test]
    fn test_crypto_algorithm_display_signature() {
        let alg = CryptoAlgorithm::Signature(SignatureAlgorithm::Ed25519);
        assert_eq!(alg.to_string(), "Ed25519");
    }

    #[test]
    fn test_crypto_algorithm_display_hash() {
        let alg = CryptoAlgorithm::Hash(HashAlgorithm::Sha256);
        assert_eq!(alg.to_string(), "SHA-256");
    }

    #[test]
    fn test_crypto_algorithm_display_kdf() {
        let alg = CryptoAlgorithm::Kdf(KdfAlgorithm::HkdfSha256);
        assert_eq!(alg.to_string(), "HKDF-SHA256");
    }

    #[test]
    fn test_crypto_algorithm_clone() {
        let alg1 = CryptoAlgorithm::Hash(HashAlgorithm::Sha256);
        let alg2 = alg1.clone();
        assert_eq!(alg1, alg2);
    }

    #[test]
    fn test_crypto_algorithm_equality() {
        let alg1 = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes256Gcm);
        let alg2 = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes256Gcm);
        assert_eq!(alg1, alg2);
    }

    #[test]
    fn test_crypto_algorithm_inequality() {
        let alg1 = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes256Gcm);
        let alg2 = CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes128Gcm);
        assert_ne!(alg1, alg2);
    }

    #[test]
    fn test_crypto_algorithm_hashable() {
        let mut set = HashSet::new();
        set.insert(CryptoAlgorithm::Hash(HashAlgorithm::Sha256));
        set.insert(CryptoAlgorithm::Hash(HashAlgorithm::Sha384));
        set.insert(CryptoAlgorithm::Hash(HashAlgorithm::Sha256)); // Duplicate
        assert_eq!(set.len(), 2);
    }
}

// ============================================================================
// SYMMETRIC ALGORITHM TESTS
// ============================================================================

#[cfg(test)]
mod symmetric_algorithm_tests {
    use super::*;

    #[test]
    fn test_aes_with_mode() {
        let alg = SymmetricAlgorithm::Aes {
            mode: AesMode::Gcm,
            key_size: 256,
        };
        assert_eq!(alg.to_string(), "AES-256-GCM");
    }

    #[test]
    fn test_aes_different_modes() {
        let modes = vec![
            (AesMode::Gcm, "AES-256-GCM"),
            (AesMode::Ctr, "AES-256-CTR"),
            (AesMode::Cbc, "AES-256-CBC"),
            (AesMode::Cfb, "AES-256-CFB"),
        ];
        for (mode, expected) in modes {
            let alg = SymmetricAlgorithm::Aes {
                mode,
                key_size: 256,
            };
            assert_eq!(alg.to_string(), expected);
        }
    }

    #[test]
    fn test_aes_different_key_sizes() {
        let key_sizes = vec![128, 192, 256];
        for key_size in key_sizes {
            let alg = SymmetricAlgorithm::Aes {
                mode: AesMode::Gcm,
                key_size,
            };
            assert_eq!(alg.to_string(), format!("AES-{}-GCM", key_size));
        }
    }

    #[test]
    fn test_chacha20_poly1305() {
        let alg = SymmetricAlgorithm::ChaCha20Poly1305;
        assert_eq!(alg.to_string(), "ChaCha20-Poly1305");
    }

    #[test]
    fn test_chacha20_with_key_size() {
        let alg = SymmetricAlgorithm::ChaCha20 { key_size: 256 };
        assert_eq!(alg.to_string(), "ChaCha20-256");
    }

    #[test]
    fn test_aes256_gcm() {
        let alg = SymmetricAlgorithm::Aes256Gcm;
        assert_eq!(alg.to_string(), "AES-256-GCM");
    }

    #[test]
    fn test_aes128_gcm() {
        let alg = SymmetricAlgorithm::Aes128Gcm;
        assert_eq!(alg.to_string(), "AES-128-GCM");
    }

    #[test]
    fn test_symmetric_custom() {
        let spec = AlgorithmSpec {
            name: "TestCipher".to_string(),
            category: AlgorithmCategory::SymmetricEncryption,
            key_sizes: vec![256],
            block_sizes: vec![16],
            parameters: vec![],
        };
        let alg = SymmetricAlgorithm::Custom {
            name: "MyCustomAlg".to_string(),
            spec,
        };
        assert_eq!(alg.to_string(), "Custom(MyCustomAlg)");
    }

    #[test]
    fn test_symmetric_clone() {
        let alg1 = SymmetricAlgorithm::Aes256Gcm;
        let alg2 = alg1.clone();
        assert_eq!(alg1, alg2);
    }

    #[test]
    fn test_symmetric_equality() {
        let alg1 = SymmetricAlgorithm::ChaCha20Poly1305;
        let alg2 = SymmetricAlgorithm::ChaCha20Poly1305;
        assert_eq!(alg1, alg2);
    }

    #[test]
    fn test_symmetric_hash() {
        let mut set = HashSet::new();
        set.insert(SymmetricAlgorithm::Aes256Gcm);
        set.insert(SymmetricAlgorithm::Aes128Gcm);
        set.insert(SymmetricAlgorithm::Aes256Gcm); // Duplicate
        assert_eq!(set.len(), 2);
    }
}

// ============================================================================
// AES MODE TESTS
// ============================================================================

#[cfg(test)]
mod aes_mode_tests {
    use super::*;

    #[test]
    fn test_aes_mode_gcm() {
        assert_eq!(AesMode::Gcm.to_string(), "GCM");
    }

    #[test]
    fn test_aes_mode_ctr() {
        assert_eq!(AesMode::Ctr.to_string(), "CTR");
    }

    #[test]
    fn test_aes_mode_cbc() {
        assert_eq!(AesMode::Cbc.to_string(), "CBC");
    }

    #[test]
    fn test_aes_mode_cfb() {
        assert_eq!(AesMode::Cfb.to_string(), "CFB");
    }

    #[test]
    fn test_aes_mode_clone() {
        let mode1 = AesMode::Gcm;
        let mode2 = mode1.clone();
        assert_eq!(mode1, mode2);
    }

    #[test]
    fn test_aes_mode_equality() {
        assert_eq!(AesMode::Gcm, AesMode::Gcm);
        assert_ne!(AesMode::Gcm, AesMode::Ctr);
    }
}

// ============================================================================
// ASYMMETRIC ALGORITHM TESTS
// ============================================================================

#[cfg(test)]
mod asymmetric_algorithm_tests {
    use super::*;

    #[test]
    fn test_rsa_oaep() {
        let alg = AsymmetricAlgorithm::RsaOaep {
            key_size: 2048,
            hash: HashAlgorithm::Sha256,
        };
        assert_eq!(alg.to_string(), "RSA-OAEP-2048-SHA-256");
    }

    #[test]
    fn test_rsa_pkcs1v15() {
        let alg = AsymmetricAlgorithm::RsaPkcs1v15 { key_size: 2048 };
        assert_eq!(alg.to_string(), "RSA-PKCS1v15-2048");
    }

    #[test]
    fn test_ecies_p256() {
        let alg = AsymmetricAlgorithm::EciesP256;
        assert_eq!(alg.to_string(), "ECIES-P256");
    }

    #[test]
    fn test_ecies_p384() {
        let alg = AsymmetricAlgorithm::EciesP384;
        assert_eq!(alg.to_string(), "ECIES-P384");
    }

    #[test]
    fn test_asymmetric_custom() {
        let spec = AlgorithmSpec {
            name: "CustomAsym".to_string(),
            category: AlgorithmCategory::AsymmetricEncryption,
            key_sizes: vec![4096],
            block_sizes: vec![],
            parameters: vec![],
        };
        let alg = AsymmetricAlgorithm::Custom {
            name: "MyAsym".to_string(),
            spec,
        };
        assert_eq!(alg.to_string(), "Custom(MyAsym)");
    }

    #[test]
    fn test_rsa_oaep_different_key_sizes() {
        let key_sizes = vec![2048, 3072, 4096];
        for key_size in key_sizes {
            let alg = AsymmetricAlgorithm::RsaOaep {
                key_size,
                hash: HashAlgorithm::Sha256,
            };
            assert_eq!(alg.to_string(), format!("RSA-OAEP-{}-SHA-256", key_size));
        }
    }

    #[test]
    fn test_asymmetric_clone() {
        let alg1 = AsymmetricAlgorithm::EciesP256;
        let alg2 = alg1.clone();
        assert_eq!(alg1, alg2);
    }

    #[test]
    fn test_asymmetric_equality() {
        let alg1 = AsymmetricAlgorithm::EciesP256;
        let alg2 = AsymmetricAlgorithm::EciesP256;
        assert_eq!(alg1, alg2);
    }
}

// ============================================================================
// SIGNATURE ALGORITHM TESTS
// ============================================================================

#[cfg(test)]
mod signature_algorithm_tests {
    use super::*;

    #[test]
    fn test_ed25519() {
        let alg = SignatureAlgorithm::Ed25519;
        assert_eq!(alg.to_string(), "Ed25519");
    }

    #[test]
    fn test_ecdsa_p256() {
        let alg = SignatureAlgorithm::EcdsaP256 {
            hash: HashAlgorithm::Sha256,
        };
        assert_eq!(alg.to_string(), "ECDSA-P256-SHA-256");
    }

    #[test]
    fn test_ecdsa_p384() {
        let alg = SignatureAlgorithm::EcdsaP384 {
            hash: HashAlgorithm::Sha384,
        };
        assert_eq!(alg.to_string(), "ECDSA-P384-SHA-384");
    }

    #[test]
    fn test_rsa_pss() {
        let alg = SignatureAlgorithm::RsaPss {
            key_size: 2048,
            hash: HashAlgorithm::Sha256,
        };
        assert_eq!(alg.to_string(), "RSA-PSS-2048-SHA-256");
    }

    #[test]
    fn test_rsa_pkcs1v15_signature() {
        let alg = SignatureAlgorithm::RsaPkcs1v15 {
            key_size: 2048,
            hash: HashAlgorithm::Sha256,
        };
        assert_eq!(alg.to_string(), "RSA-PKCS1v15-2048-SHA-256");
    }

    #[test]
    fn test_signature_custom() {
        let spec = AlgorithmSpec {
            name: "CustomSig".to_string(),
            category: AlgorithmCategory::DigitalSignature,
            key_sizes: vec![],
            block_sizes: vec![],
            parameters: vec![],
        };
        let alg = SignatureAlgorithm::Custom {
            name: "MySig".to_string(),
            spec,
        };
        assert_eq!(alg.to_string(), "Custom(MySig)");
    }

    #[test]
    fn test_signature_clone() {
        let alg1 = SignatureAlgorithm::Ed25519;
        let alg2 = alg1.clone();
        assert_eq!(alg1, alg2);
    }

    #[test]
    fn test_signature_equality() {
        let alg1 = SignatureAlgorithm::Ed25519;
        let alg2 = SignatureAlgorithm::Ed25519;
        assert_eq!(alg1, alg2);
    }
}

// ============================================================================
// HASH ALGORITHM TESTS
// ============================================================================

#[cfg(test)]
mod hash_algorithm_tests {
    use super::*;

    #[test]
    fn test_sha256() {
        assert_eq!(HashAlgorithm::Sha256.to_string(), "SHA-256");
    }

    #[test]
    fn test_sha384() {
        assert_eq!(HashAlgorithm::Sha384.to_string(), "SHA-384");
    }

    #[test]
    fn test_sha512() {
        assert_eq!(HashAlgorithm::Sha512.to_string(), "SHA-512");
    }

    #[test]
    fn test_sha3_256() {
        assert_eq!(HashAlgorithm::Sha3_256.to_string(), "SHA3-256");
    }

    #[test]
    fn test_sha3_384() {
        assert_eq!(HashAlgorithm::Sha3_384.to_string(), "SHA3-384");
    }

    #[test]
    fn test_sha3_512() {
        assert_eq!(HashAlgorithm::Sha3_512.to_string(), "SHA3-512");
    }

    #[test]
    fn test_blake2b() {
        let alg = HashAlgorithm::Blake2b { output_size: 64 };
        assert_eq!(alg.to_string(), "BLAKE2b-64");
    }

    #[test]
    fn test_blake2s() {
        let alg = HashAlgorithm::Blake2s { output_size: 32 };
        assert_eq!(alg.to_string(), "BLAKE2s-32");
    }

    #[test]
    fn test_blake3() {
        assert_eq!(HashAlgorithm::Blake3.to_string(), "BLAKE3");
    }

    #[test]
    fn test_hash_custom() {
        let spec = AlgorithmSpec {
            name: "CustomHash".to_string(),
            category: AlgorithmCategory::Hash,
            key_sizes: vec![],
            block_sizes: vec![],
            parameters: vec![],
        };
        let alg = HashAlgorithm::Custom {
            name: "MyHash".to_string(),
            spec,
        };
        assert_eq!(alg.to_string(), "Custom(MyHash)");
    }

    #[test]
    fn test_hash_clone() {
        let alg1 = HashAlgorithm::Sha256;
        let alg2 = alg1.clone();
        assert_eq!(alg1, alg2);
    }

    #[test]
    fn test_hash_equality() {
        assert_eq!(HashAlgorithm::Sha256, HashAlgorithm::Sha256);
        assert_ne!(HashAlgorithm::Sha256, HashAlgorithm::Sha384);
    }

    #[test]
    fn test_blake2_different_sizes() {
        let sizes = vec![32, 48, 64];
        for size in sizes {
            let alg = HashAlgorithm::Blake2b { output_size: size };
            assert_eq!(alg.to_string(), format!("BLAKE2b-{}", size));
        }
    }
}

// ============================================================================
// KDF ALGORITHM TESTS
// ============================================================================

#[cfg(test)]
mod kdf_algorithm_tests {
    use super::*;

    #[test]
    fn test_hkdf_sha256() {
        assert_eq!(KdfAlgorithm::HkdfSha256.to_string(), "HKDF-SHA256");
    }

    #[test]
    fn test_hkdf_sha384() {
        assert_eq!(KdfAlgorithm::HkdfSha384.to_string(), "HKDF-SHA384");
    }

    #[test]
    fn test_hkdf_sha512() {
        assert_eq!(KdfAlgorithm::HkdfSha512.to_string(), "HKDF-SHA512");
    }

    #[test]
    fn test_pbkdf2() {
        let alg = KdfAlgorithm::Pbkdf2 {
            hash: HashAlgorithm::Sha256,
            iterations: 100000,
        };
        assert_eq!(alg.to_string(), "PBKDF2-SHA-256-100000");
    }

    #[test]
    fn test_scrypt() {
        let alg = KdfAlgorithm::Scrypt {
            n: 16384,
            r: 8,
            p: 1,
        };
        assert_eq!(alg.to_string(), "Scrypt-16384-8-1");
    }

    #[test]
    fn test_argon2() {
        let alg = KdfAlgorithm::Argon2 {
            variant: Argon2Variant::Argon2id,
        };
        assert_eq!(alg.to_string(), "Argon2-Argon2id");
    }

    #[test]
    fn test_kdf_custom() {
        let spec = AlgorithmSpec {
            name: "CustomKdf".to_string(),
            category: AlgorithmCategory::KeyDerivation,
            key_sizes: vec![],
            block_sizes: vec![],
            parameters: vec![],
        };
        let alg = KdfAlgorithm::Custom {
            name: "MyKdf".to_string(),
            spec,
        };
        assert_eq!(alg.to_string(), "Custom(MyKdf)");
    }

    #[test]
    fn test_argon2_variants() {
        let variants = vec![
            Argon2Variant::Argon2d,
            Argon2Variant::Argon2i,
            Argon2Variant::Argon2id,
        ];
        for variant in variants {
            let alg = KdfAlgorithm::Argon2 {
                variant: variant.clone(),
            };
            assert!(alg.to_string().starts_with("Argon2-"));
        }
    }

    #[test]
    fn test_kdf_clone() {
        let alg1 = KdfAlgorithm::HkdfSha256;
        let alg2 = alg1.clone();
        assert_eq!(alg1, alg2);
    }

    #[test]
    fn test_kdf_equality() {
        assert_eq!(KdfAlgorithm::HkdfSha256, KdfAlgorithm::HkdfSha256);
        assert_ne!(KdfAlgorithm::HkdfSha256, KdfAlgorithm::HkdfSha384);
    }
}

// ============================================================================
// STRUCT TESTS
// ============================================================================

#[cfg(test)]
mod struct_tests {
    use super::*;

    #[test]
    fn test_algorithm_spec_creation() {
        let spec = AlgorithmSpec {
            name: "TestAlg".to_string(),
            category: AlgorithmCategory::SymmetricEncryption,
            key_sizes: vec![128, 256],
            block_sizes: vec![16],
            parameters: vec![("param1".to_string(), "value1".to_string())],
        };
        assert_eq!(spec.name, "TestAlg");
        assert_eq!(spec.key_sizes.len(), 2);
    }

    #[test]
    fn test_encrypted_data_creation() {
        let data = EncryptedData {
            algorithm: "AES-256-GCM".to_string(),
            ciphertext: vec![1, 2, 3, 4],
            nonce: Some(vec![5, 6, 7]),
            tag: Some(vec![8, 9]),
        };
        assert_eq!(data.algorithm, "AES-256-GCM");
        assert_eq!(data.ciphertext.len(), 4);
        assert!(data.nonce.is_some());
        assert!(data.tag.is_some());
    }

    #[test]
    fn test_signature_creation() {
        let sig = Signature {
            algorithm: "Ed25519".to_string(),
            signature: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(sig.algorithm, "Ed25519");
        assert_eq!(sig.signature.len(), 5);
    }

    #[test]
    fn test_encryption_options_default() {
        let opts = EncryptionOptions::default();
        assert!(opts.nonce.is_none());
        assert!(opts.associated_data.is_none());
    }

    #[test]
    fn test_encryption_options_with_data() {
        let opts = EncryptionOptions {
            nonce: Some(vec![1, 2, 3]),
            associated_data: Some(vec![4, 5, 6]),
        };
        assert!(opts.nonce.is_some());
        assert_eq!(opts.nonce.unwrap().len(), 3);
    }

    #[test]
    fn test_decryption_options_default() {
        let opts = DecryptionOptions::default();
        assert!(opts.associated_data.is_none());
    }

    #[test]
    fn test_signing_options_default() {
        let opts = SigningOptions::default();
        assert!(!opts.deterministic);
    }

    #[test]
    fn test_signing_options_deterministic() {
        let opts = SigningOptions {
            deterministic: true,
        };
        assert!(opts.deterministic);
    }

    #[test]
    fn test_verification_options_default() {
        let _opts = VerificationOptions::default();
        // No fields to test, just ensure it compiles
    }

    #[test]
    fn test_algorithm_category_variants() {
        let categories = [
            AlgorithmCategory::SymmetricEncryption,
            AlgorithmCategory::AsymmetricEncryption,
            AlgorithmCategory::DigitalSignature,
            AlgorithmCategory::Hash,
            AlgorithmCategory::KeyDerivation,
        ];
        assert_eq!(categories.len(), 5);
    }

    #[test]
    fn test_crypto_operation_variants() {
        let operations = [
            CryptoOperation::SymmetricEncryption,
            CryptoOperation::SymmetricDecryption,
            CryptoOperation::AsymmetricEncryption,
            CryptoOperation::AsymmetricDecryption,
            CryptoOperation::Signing,
            CryptoOperation::Verification,
            CryptoOperation::Hashing,
            CryptoOperation::KeyDerivation,
        ];
        assert_eq!(operations.len(), 8);
    }

    #[test]
    fn test_encrypted_data_clone() {
        let data1 = EncryptedData {
            algorithm: "AES".to_string(),
            ciphertext: vec![1, 2, 3],
            nonce: None,
            tag: None,
        };
        let data2 = data1.clone();
        assert_eq!(data1.algorithm, data2.algorithm);
    }

    #[test]
    fn test_signature_clone() {
        let sig1 = Signature {
            algorithm: "Ed25519".to_string(),
            signature: vec![1, 2, 3],
        };
        let sig2 = sig1.clone();
        assert_eq!(sig1.algorithm, sig2.algorithm);
    }
}
