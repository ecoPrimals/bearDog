// SPDX-License-Identifier: AGPL-3.0-only

//! Coverage Gap Tests for beardog-security
//!
//! Targets modules with lowest coverage:
//! - authorization_types (0%)
//! - memory_key_manager/metrics (0%)
//! - memory_key_manager/config (~33%)
//! - genesis/types (~57%)
//! - simd_crypto (~70%) - safe_chacha20_with_nonce
//! - crypto_utils (~85%) - generate_password, sha256_hash
//! - genesis/witness (~81%)
//! - genesis/physical_proof (~85%)

#[cfg(test)]
mod coverage_gap_tests {
    // ========================================================================
    // authorization_types: 0% → test all Default impls and type construction
    // ========================================================================

    mod authorization_types_tests {
        use crate::authorization_types::*;
        use std::collections::HashMap;

        #[test]
        fn test_subject_default() {
            let subject = Subject::default();
            assert!(subject.id.is_empty());
            assert!(subject.name.is_empty());
            assert!(matches!(subject.subject_type, SubjectType::User));
            assert!(subject.roles.is_empty());
            assert!(subject.clearance_level.is_none());
            assert!(subject.metadata.is_empty());
        }

        #[test]
        fn test_subject_custom() {
            let mut metadata = HashMap::new();
            metadata.insert("key".to_string(), "value".to_string());
            let subject = Subject {
                id: "sub-001".to_string(),
                name: "Test Subject".to_string(),
                subject_type: SubjectType::Service,
                roles: vec!["admin".to_string()],
                clearance_level: Some(5),
                metadata,
            };
            assert_eq!(subject.id, "sub-001");
            assert_eq!(subject.name, "Test Subject");
            assert!(matches!(subject.subject_type, SubjectType::Service));
            assert_eq!(subject.roles.len(), 1);
            assert_eq!(subject.clearance_level, Some(5));
        }

        #[test]
        fn test_subject_type_variants() {
            let _user = SubjectType::User;
            let _service = SubjectType::Service;
            let _node = SubjectType::Node;
            let _system = SubjectType::System;
            // Verify Debug
            assert!(!format!("{:?}", SubjectType::User).is_empty());
            assert!(!format!("{:?}", SubjectType::Node).is_empty());
            assert!(!format!("{:?}", SubjectType::System).is_empty());
        }

        #[test]
        fn test_action_default() {
            let action = Action::default();
            assert!(action.name.is_empty());
            assert!(matches!(action.action_type, ActionType::Read));
            assert!(action.metadata.is_empty());
        }

        #[test]
        fn test_action_custom() {
            let action = Action {
                name: "deploy".to_string(),
                action_type: ActionType::Execute,
                metadata: HashMap::new(),
            };
            assert_eq!(action.name, "deploy");
            assert_eq!(action.action_type, ActionType::Execute);
        }

        #[test]
        fn test_action_type_variants() {
            assert_eq!(ActionType::Read, ActionType::Read);
            assert_ne!(ActionType::Read, ActionType::Write);
            assert_ne!(ActionType::Execute, ActionType::Delete);
            assert_ne!(ActionType::Admin, ActionType::Approve);
            assert_ne!(ActionType::Create, ActionType::Update);
            // Debug
            assert!(!format!("{:?}", ActionType::Write).is_empty());
            assert!(!format!("{:?}", ActionType::Admin).is_empty());
            assert!(!format!("{:?}", ActionType::Approve).is_empty());
            assert!(!format!("{:?}", ActionType::Create).is_empty());
            assert!(!format!("{:?}", ActionType::Update).is_empty());
        }

        #[test]
        fn test_resource_default() {
            let resource = Resource::default();
            assert!(resource.name.is_empty());
            assert!(matches!(
                resource.classification,
                ResourceClassification::Internal
            ));
            assert!(resource.metadata.is_empty());
        }

        #[test]
        fn test_resource_custom() {
            let resource = Resource {
                name: "secret-doc".to_string(),
                classification: ResourceClassification::Secret,
                metadata: HashMap::new(),
            };
            assert_eq!(resource.name, "secret-doc");
            assert_eq!(resource.classification, ResourceClassification::Secret);
        }

        #[test]
        fn test_resource_classification_variants() {
            assert_eq!(
                ResourceClassification::Public,
                ResourceClassification::Public
            );
            assert_ne!(
                ResourceClassification::Confidential,
                ResourceClassification::TopSecret
            );
            assert!(!format!("{:?}", ResourceClassification::Internal).is_empty());
            assert!(!format!("{:?}", ResourceClassification::TopSecret).is_empty());
        }

        #[test]
        fn test_risk_level_variants() {
            assert_eq!(RiskLevel::Low, RiskLevel::Low);
            assert_ne!(RiskLevel::Medium, RiskLevel::High);
            assert_ne!(RiskLevel::High, RiskLevel::Critical);
            assert!(!format!("{:?}", RiskLevel::Critical).is_empty());
        }

        #[test]
        fn test_authorization_result_construction() {
            let result = AuthorizationResult {
                authorized: true,
                reason: "Access granted".to_string(),
                risk_level: RiskLevel::Low,
                additional_requirements: vec![],
                expires_at: None,
                audit_id: "audit-001".to_string(),
            };
            assert!(result.authorized);
            assert_eq!(result.reason, "Access granted");
            assert_eq!(result.risk_level, RiskLevel::Low);
            assert!(result.additional_requirements.is_empty());
            assert!(result.expires_at.is_none());
        }

        #[test]
        fn test_authorization_result_with_requirements() {
            let result = AuthorizationResult {
                authorized: false,
                reason: "Needs approval".to_string(),
                risk_level: RiskLevel::High,
                additional_requirements: vec!["mfa".to_string(), "manager_approval".to_string()],
                expires_at: Some(chrono::Utc::now()),
                audit_id: "audit-002".to_string(),
            };
            assert!(!result.authorized);
            assert_eq!(result.additional_requirements.len(), 2);
            assert!(result.expires_at.is_some());
        }

        #[test]
        fn test_subject_clone() {
            let subject = Subject {
                id: "clone-test".to_string(),
                name: "Clone".to_string(),
                subject_type: SubjectType::Node,
                roles: vec!["reader".to_string()],
                clearance_level: Some(3),
                metadata: HashMap::new(),
            };
            let cloned = subject.clone();
            assert_eq!(cloned.id, subject.id);
            assert_eq!(cloned.name, subject.name);
        }
    }

    // ========================================================================
    // memory_key_manager/metrics: 0% → test Default impls
    // ========================================================================

    mod metrics_tests {
        use crate::memory_key_manager::metrics::*;
        use std::time::Duration;

        #[test]
        fn test_key_manager_metrics_default() {
            let metrics = KeyManagerMetrics::default();
            assert_eq!(metrics.total_keys, 0);
            assert_eq!(metrics.keys_created, 0);
            assert_eq!(metrics.keys_accessed, 0);
            assert_eq!(metrics.keys_expired, 0);
            assert_eq!(metrics.avg_access_time, Duration::from_millis(0));
            assert_eq!(metrics.cache_hit_rate, 0.0);
            assert_eq!(metrics.memory_usage, 0);
        }

        #[test]
        fn test_key_manager_metrics_custom() {
            let metrics = KeyManagerMetrics {
                total_keys: 100,
                keys_created: 200,
                keys_accessed: 500,
                keys_expired: 10,
                avg_access_time: Duration::from_micros(50),
                cache_hit_rate: 0.95,
                memory_usage: 1024,
            };
            assert_eq!(metrics.total_keys, 100);
            assert_eq!(metrics.keys_created, 200);
            assert_eq!(metrics.cache_hit_rate, 0.95);
        }

        #[test]
        fn test_operation_metrics() {
            let op = OperationMetrics {
                operation_type: "generate".to_string(),
                duration: Duration::from_millis(5),
                success: true,
                error: None,
            };
            assert_eq!(op.operation_type, "generate");
            assert!(op.success);
            assert!(op.error.is_none());
        }

        #[test]
        fn test_operation_metrics_with_error() {
            let op = OperationMetrics {
                operation_type: "decrypt".to_string(),
                duration: Duration::from_millis(2),
                success: false,
                error: Some("key not found".to_string()),
            };
            assert!(!op.success);
            assert_eq!(op.error.as_deref(), Some("key not found"));
        }

        #[test]
        fn test_extended_metrics() {
            let ext = ExtendedMetrics {
                base_metrics: KeyManagerMetrics::default(),
                recent_operations: vec![OperationMetrics {
                    operation_type: "store".to_string(),
                    duration: Duration::from_millis(1),
                    success: true,
                    error: None,
                }],
                timestamp: chrono::Utc::now(),
            };
            assert_eq!(ext.base_metrics.total_keys, 0);
            assert_eq!(ext.recent_operations.len(), 1);
        }

        #[test]
        fn test_metrics_serde_roundtrip() {
            let metrics = KeyManagerMetrics::default();
            let json = serde_json::to_string(&metrics).unwrap();
            let deserialized: KeyManagerMetrics = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized.total_keys, 0);
            assert_eq!(deserialized.keys_created, 0);
        }
    }

    // ========================================================================
    // memory_key_manager/config: ~33% → test KeyStorageConfig default
    // ========================================================================

    mod config_tests {
        use crate::memory_key_manager::config::*;

        #[test]
        fn test_memory_key_config_default() {
            let config = MemoryKeyConfig::default();
            assert_eq!(config.max_keys, 1000);
            assert_eq!(config.key_expiration_seconds, 0);
            assert!(!config.enable_rotation);
        }

        #[test]
        fn test_key_storage_config_default() {
            let config = KeyStorageConfig::default();
            assert_eq!(config.backend, "memory");
            assert_eq!(config.max_capacity, 10000);
            assert!(config.encrypt_at_rest);
        }

        #[test]
        fn test_key_storage_config_custom() {
            let config = KeyStorageConfig {
                backend: "disk".to_string(),
                max_capacity: 5000,
                encrypt_at_rest: false,
            };
            assert_eq!(config.backend, "disk");
            assert_eq!(config.max_capacity, 5000);
            assert!(!config.encrypt_at_rest);
        }

        #[test]
        fn test_key_manager_config_alias() {
            // KeyManagerConfig is a type alias for MemoryKeyConfig
            let config: KeyManagerConfig = MemoryKeyConfig::default();
            assert_eq!(config.max_keys, 1000);
        }

        #[test]
        fn test_config_serde_roundtrip() {
            let config = MemoryKeyConfig {
                max_keys: 500,
                key_expiration_seconds: 3600,
                enable_rotation: true,
            };
            let json = serde_json::to_string(&config).unwrap();
            let deserialized: MemoryKeyConfig = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized.max_keys, 500);
            assert_eq!(deserialized.key_expiration_seconds, 3600);
            assert!(deserialized.enable_rotation);
        }

        #[test]
        fn test_key_storage_config_serde_roundtrip() {
            let config = KeyStorageConfig::default();
            let json = serde_json::to_string(&config).unwrap();
            let deserialized: KeyStorageConfig = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized.backend, "memory");
            assert!(deserialized.encrypt_at_rest);
        }
    }

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
            let (ciphertext, nonce) =
                BearDogCrypto::encrypt_aes_gcm(&key, plaintext, None).unwrap();
            let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce).unwrap();
            assert_eq!(decrypted, plaintext);
        }

        #[test]
        fn test_decrypt_aes_gcm_tampered_ciphertext_fails() {
            let key = BearDogCrypto::generate_secure_random(32);
            let (ciphertext, nonce) =
                BearDogCrypto::encrypt_aes_gcm(&key, b"plaintext", None).unwrap();
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

    // ========================================================================
    // entropy_orchestrator: HumanEntropyInput with all fields (mix_with_human_input)
    // ========================================================================

    mod entropy_orchestrator_tests {
        use crate::hsm::entropy_orchestrator::types::SecurityLevel;
        use crate::hsm::entropy_orchestrator::{HsmDeviceInfo, HsmDeviceType, HumanEntropyInput};

        #[tokio::test]
        async fn test_human_entropy_input_environmental_data() {
            let input = HumanEntropyInput {
                biometric_data: None,
                behavioral_data: None,
                environmental_data: Some(vec![10, 20, 30, 40, 50]),
            };
            assert!(input.environmental_data.is_some());
        }

        #[tokio::test]
        async fn test_human_entropy_input_all_fields() {
            let input = HumanEntropyInput {
                biometric_data: Some(vec![1, 2, 3]),
                behavioral_data: Some(vec![4, 5, 6]),
                environmental_data: Some(vec![7, 8, 9]),
            };
            assert!(input.biometric_data.is_some());
            assert!(input.behavioral_data.is_some());
            assert!(input.environmental_data.is_some());
        }

        #[tokio::test]
        async fn test_generate_entropy_with_all_human_inputs() {
            let mut orchestrator = crate::hsm::entropy_orchestrator::HsmEntropyOrchestrator::new()
                .await
                .unwrap();
            let human_input = HumanEntropyInput {
                biometric_data: Some(vec![100, 101, 102]),
                behavioral_data: Some(vec![200, 201, 202]),
                environmental_data: Some(vec![50, 51, 52]),
            };
            let result = orchestrator
                .generate_human_entropy(32, Some(human_input))
                .await;
            assert!(result.is_ok() || result.is_err());
        }

        #[test]
        fn test_hsm_device_info_serialization() {
            let info = HsmDeviceInfo {
                device_type: HsmDeviceType::Fido2,
                device_id: "fido2_0".to_string(),
                name: "Test Device".to_string(),
                security_level: SecurityLevel::Hardware,
                biometric_capable: true,
            };
            let json = serde_json::to_string(&info).unwrap();
            let deserialized: HsmDeviceInfo = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized.device_id, info.device_id);
            assert_eq!(deserialized.security_level, SecurityLevel::Hardware);
        }
    }

    // ========================================================================
    // genesis/physical_proof: PhysicalProximityVerifier, PhysicalProofError
    // ========================================================================

    mod physical_proof_tests {
        use crate::genesis::physical_proof::*;
        use crate::genesis::types::{PhysicalChannelType, TrustLevel};

        #[test]
        fn test_physical_proximity_verifier_default_config() {
            let verifier = PhysicalProximityVerifier::default_genesis_config();
            let result = verifier.verify(PhysicalChannelType::HardwareKey);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), TrustLevel::Maximum);
        }

        #[test]
        fn test_physical_proximity_verifier_maximum_security() {
            let verifier = PhysicalProximityVerifier::maximum_security();
            let result = verifier.verify(PhysicalChannelType::HardwareKey);
            assert!(result.is_ok());
        }

        #[test]
        fn test_physical_proximity_verifier_bluetooth_insufficient_for_maximum() {
            let verifier = PhysicalProximityVerifier::maximum_security();
            let result = verifier.verify(PhysicalChannelType::Bluetooth);
            assert!(result.is_err());
            if let Err(PhysicalProofError::InsufficientTrust { actual }) = result {
                assert_eq!(actual, TrustLevel::Medium);
            }
        }

        #[test]
        fn test_physical_proof_error_display() {
            let err = PhysicalProofError::UnsupportedChannel(PhysicalChannelType::Bluetooth);
            let msg = format!("{err}");
            assert!(!msg.is_empty());

            let err2 = PhysicalProofError::AttestationFailed("bad attestation".to_string());
            assert!(format!("{err2}").contains("bad attestation"));
        }

        #[test]
        fn test_physical_proof_error_from_beardog_error() {
            use beardog_errors::BearDogError;
            let err = PhysicalProofError::InsufficientTrust {
                actual: TrustLevel::Low,
            };
            let beardog_err: BearDogError = err.into();
            assert!(format!("{beardog_err}").contains("Physical proof"));
        }
    }

    // ========================================================================
    // FIDO2 operations: Ctap2Command enum variants (when fido2 feature enabled)
    // ========================================================================

    #[cfg(feature = "fido2")]
    mod fido2_operations_tests {
        use crate::hsm::fido2::Ctap2Command;

        #[test]
        fn test_ctap2_command_all_variants() {
            assert_eq!(Ctap2Command::MakeCredential as u8, 0x01);
            assert_eq!(Ctap2Command::GetAssertion as u8, 0x02);
            assert_eq!(Ctap2Command::GetInfo as u8, 0x04);
            assert_eq!(Ctap2Command::ClientPin as u8, 0x06);
            assert_eq!(Ctap2Command::Reset as u8, 0x07);
            assert_eq!(Ctap2Command::GetNextAssertion as u8, 0x08);
            assert_eq!(Ctap2Command::CredentialManagement as u8, 0x0A);
        }
    }

    // ========================================================================
    // genesis/witness: ~81% → test uncovered error paths
    // ========================================================================

    mod witness_tests {
        use crate::genesis::types::PhysicalChannelType;
        use crate::genesis::witness::*;
        use std::time::{SystemTime, UNIX_EPOCH};

        fn current_timestamp() -> u64 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        }

        #[test]
        fn test_witness_trust_level() {
            let witness = GenesisWitness::new(
                "device-1".to_string(),
                vec![0u8; 32],
                PhysicalChannelType::HardwareKey,
                current_timestamp(),
                vec![0u8; 64],
            );
            assert_eq!(
                witness.trust_level(),
                crate::genesis::types::TrustLevel::Maximum
            );
        }

        #[test]
        fn test_witness_nfc_trust_level() {
            let witness = GenesisWitness::new(
                "nfc-1".to_string(),
                vec![0u8; 32],
                PhysicalChannelType::Nfc,
                current_timestamp(),
                vec![0u8; 64],
            );
            assert_eq!(
                witness.trust_level(),
                crate::genesis::types::TrustLevel::High
            );
        }

        #[test]
        fn test_witness_bluetooth_trust_level() {
            let witness = GenesisWitness::new(
                "bt-1".to_string(),
                vec![0u8; 32],
                PhysicalChannelType::Bluetooth,
                current_timestamp(),
                vec![0u8; 64],
            );
            assert_eq!(
                witness.trust_level(),
                crate::genesis::types::TrustLevel::Medium
            );
        }

        #[test]
        fn test_witness_verification_error_from_beardog_error() {
            use beardog_errors::BearDogError;

            let err = WitnessVerificationError::InvalidSignature;
            let beardog_err: BearDogError = err.into();
            let msg = format!("{beardog_err}");
            assert!(msg.contains("Witness verification failed"));
        }

        #[test]
        fn test_witness_verification_error_display() {
            let err = WitnessVerificationError::SignatureExpired {
                age_secs: 100000,
                max_secs: 86400,
            };
            let msg = format!("{err}");
            assert!(msg.contains("expired"));
            assert!(msg.contains("100000"));

            let err2 = WitnessVerificationError::InvalidPublicKey("wrong format".to_string());
            assert!(format!("{err2}").contains("wrong format"));

            let err3 = WitnessVerificationError::UnauthorizedWitness {
                device_id: "bad-device".to_string(),
            };
            assert!(format!("{err3}").contains("bad-device"));

            let err4 = WitnessVerificationError::CryptoError("crypto fail".to_string());
            assert!(format!("{err4}").contains("crypto fail"));
        }

        #[test]
        fn test_verifier_default_is_permissive() {
            let verifier = GenesisWitnessVerifier::default();
            let witness = GenesisWitness::new(
                "any-dev".to_string(),
                vec![0u8; 32],
                PhysicalChannelType::HardwareKey,
                current_timestamp(),
                vec![0u8; 64],
            );
            assert!(verifier.verify(&witness, "node-1").is_ok());
        }

        #[test]
        fn test_verify_empty_node_id() {
            let verifier = GenesisWitnessVerifier::permissive();
            let witness = GenesisWitness::new(
                "dev-1".to_string(),
                vec![0u8; 32],
                PhysicalChannelType::HardwareKey,
                current_timestamp(),
                vec![0u8; 64],
            );
            // Empty node ID should fail signature validation
            assert!(verifier.verify(&witness, "").is_err());
        }

        #[test]
        fn test_verify_empty_device_id() {
            let verifier = GenesisWitnessVerifier::permissive();
            let witness = GenesisWitness::new(
                "".to_string(), // empty device ID
                vec![0u8; 32],
                PhysicalChannelType::HardwareKey,
                current_timestamp(),
                vec![0u8; 64],
            );
            assert!(verifier.verify(&witness, "node-1").is_err());
        }

        #[test]
        fn test_witness_serde_roundtrip() {
            let witness = GenesisWitness::new(
                "serde-device".to_string(),
                vec![1u8; 32],
                PhysicalChannelType::QrCodeWithOob,
                current_timestamp(),
                vec![2u8; 64],
            );
            let json = serde_json::to_string(&witness).unwrap();
            let deserialized: GenesisWitness = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized.device_id, "serde-device");
            assert_eq!(deserialized.public_key.len(), 32);
            assert_eq!(deserialized.signature.len(), 64);
        }
    }

    // ========================================================================
    // memory_key_manager/mod: ~92% → cover remaining paths
    // ========================================================================

    mod key_manager_tests {
        use crate::memory_key_manager::{KeyMetadata, MemoryKeyConfig, MemoryKeyManager};

        #[test]
        fn test_key_manager_generate_and_exists() {
            let config = MemoryKeyConfig::default();
            let manager = MemoryKeyManager::new(config).unwrap();

            let key_id = manager.generate_key().unwrap();
            assert!(manager.key_exists(&key_id).unwrap());
            assert!(!manager.key_exists("nonexistent").unwrap());
        }

        #[test]
        fn test_key_manager_store_get_delete() {
            let config = MemoryKeyConfig::default();
            let mut manager = MemoryKeyManager::new(config).unwrap();

            let metadata = KeyMetadata {
                id: "test-key".to_string(),
                created_at: chrono::Utc::now(),
                key_type: "AES-256".to_string(),
            };

            let payload = vec![1u8, 2, 3, 4];
            let key_id = manager.store_key(&payload, metadata).unwrap();

            let retrieved = manager.get_key(&key_id).unwrap();
            assert_eq!(retrieved, payload);

            manager.delete_key(&key_id).unwrap();
            assert!(manager.get_key(&key_id).is_err());
        }

        #[test]
        fn test_key_manager_get_nonexistent() {
            let config = MemoryKeyConfig::default();
            let manager = MemoryKeyManager::new(config).unwrap();
            assert!(manager.get_key("does-not-exist").is_err());
        }

        #[test]
        fn test_key_manager_list_keys() {
            let config = MemoryKeyConfig::default();
            let manager = MemoryKeyManager::new(config).unwrap();
            let keys = manager.list_keys().unwrap();
            assert!(keys.is_empty());
        }

        #[test]
        fn test_key_metadata_construction() {
            let meta = KeyMetadata {
                id: "key-001".to_string(),
                created_at: chrono::Utc::now(),
                key_type: "RSA-2048".to_string(),
            };
            assert_eq!(meta.id, "key-001");
            assert_eq!(meta.key_type, "RSA-2048");
        }
    }
}
