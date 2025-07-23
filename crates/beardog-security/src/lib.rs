//! Security and cryptography for BearDog Security Manager
//!
//! This crate provides comprehensive security functionality including
//! encryption, decryption, key management, and security policies.

pub mod address_management;
pub mod crypto_utils;
pub mod decentralized_auth;
pub mod encryption;
pub mod handlers;
pub mod memory_key_manager;
pub mod recovery;
// pub mod recovery_tests; // Temporarily disabled due to API changes
pub mod tests;
pub mod types;
pub mod zero_copy; // New modular zero-copy implementation
pub mod zero_copy_crypto; // Add zero-copy crypto optimizations (legacy compat)

// Re-export main types for convenience
pub use address_management::{AddressFormat, AddressInfo, AddressManager, DerivationPath};
pub use crypto_utils::BearDogCrypto;
pub use memory_key_manager::MemoryKeyManager;
pub use types::*;
pub use zero_copy_crypto::{BufferPool, EncryptionContext, ZeroCopyConfig, ZeroCopyCrypto};

#[cfg(test)]
mod comprehensive_tests {
    use super::*;
    use crate::types::audit_types::{
        Action, ActionType, Resource, ResourceClassification, RiskLevel,
    };
    use crate::types::auth_types::{Subject, SubjectType};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_crypto_utils_functionality() {
        // Test key generation
        let key = crypto_utils::BearDogCrypto::generate_secure_random(32)
            .expect("Failed to generate key");
        assert_eq!(key.len(), 32);

        let key2 = crypto_utils::BearDogCrypto::generate_secure_random(32)
            .expect("Failed to generate key");
        assert_ne!(key, key2); // Should be different

        // Test encryption/decryption
        let plaintext = b"test data for encryption";
        let nonce = crypto_utils::BearDogCrypto::generate_secure_nonce(12)
            .expect("Failed to generate nonce");
        let encrypted = crypto_utils::BearDogCrypto::encrypt_aes_gcm(&key, plaintext, Some(&nonce))
            .expect("Failed to encrypt");
        assert_ne!(encrypted.0, plaintext.to_vec());

        let decrypted =
            crypto_utils::BearDogCrypto::decrypt_aes_gcm(&key, &encrypted.0, &encrypted.1)
                .expect("Failed to decrypt");
        assert_eq!(decrypted, plaintext.to_vec());
    }

    #[tokio::test]
    async fn test_decentralized_auth_manager() {
        let auth_manager = decentralized_auth::DecentralizedAuthManager::new(24)
            .expect("Failed to create auth manager");

        // Test token creation
        let permissions = vec!["read".to_string(), "write".to_string()];
        let mut metadata = HashMap::new();
        metadata.insert("role".to_string(), "admin".to_string());

        let token = auth_manager
            .create_auth_token(
                "test_user",
                "test_service",
                permissions.clone(),
                metadata.clone(),
            )
            .expect("Failed to create token");

        assert_eq!(token.claims.subject, "test_user");
        assert_eq!(token.claims.audience, "test_service");
        assert_eq!(token.claims.permissions, permissions);
        assert!(!token.signature.is_empty());

        // Test token verification
        let is_valid = auth_manager
            .verify_auth_token(&token)
            .expect("Failed to verify token");
        assert!(is_valid);

        // Token refresh functionality not yet implemented
        // TODO: Implement token refresh in DecentralizedAuthManager
        // let refreshed_token = auth_manager
        //     .refresh_token(&token)
        //     .expect("Failed to refresh token");
        // assert_eq!(refreshed_token.claims.subject, token.claims.subject);
        // assert_eq!(refreshed_token.claims.audience, token.claims.audience);
    }

    #[tokio::test]
    async fn test_memory_key_manager() {
        let config = memory_key_manager::MemoryKeyConfig::default();
        let key_manager = memory_key_manager::MemoryKeyManager::new(config)
            .await
            .expect("Failed to create key manager");

        // Test key storage and retrieval
        let key_data = b"super_secret_key_data";
        let metadata = memory_key_manager::KeyMetadata {
            key_type: "AES-256".to_string(),
            purpose: "testing".to_string(),
            algorithm: "AES-GCM".to_string(),
            key_size: 256,
            owner_id: "test_user".to_string(),
            tags: vec!["test".to_string()],
            attributes: std::collections::HashMap::new(),
        };

        let key_id = key_manager
            .store_key(key_data, metadata)
            .await
            .expect("Failed to store key");

        let retrieved = key_manager
            .get_key(&key_id)
            .await
            .expect("Failed to retrieve key");
        assert_eq!(retrieved, key_data.to_vec());

        // Test key deletion
        key_manager
            .delete_key(&key_id)
            .await
            .expect("Failed to delete key");
        assert!(key_manager.get_key(&key_id).await.is_err());

        // Test key listing
        let metadata1 = memory_key_manager::KeyMetadata {
            key_type: "AES-256".to_string(),
            purpose: "test1".to_string(),
            algorithm: "AES-GCM".to_string(),
            key_size: 256,
            owner_id: "test_user".to_string(),
            tags: vec!["test".to_string()],
            attributes: std::collections::HashMap::new(),
        };
        let metadata2 = memory_key_manager::KeyMetadata {
            key_type: "AES-256".to_string(),
            purpose: "test2".to_string(),
            algorithm: "AES-GCM".to_string(),
            key_size: 256,
            owner_id: "test_user".to_string(),
            tags: vec!["test".to_string()],
            attributes: std::collections::HashMap::new(),
        };

        key_manager
            .store_key(b"data1", metadata1)
            .await
            .expect("Failed to store key1");
        key_manager
            .store_key(b"data2", metadata2)
            .await
            .expect("Failed to store key2");

        let keys = key_manager.list_keys().await.expect("Failed to list keys");
        assert_eq!(keys.len(), 2);
        // Check that we have keys with the expected purposes
        assert!(keys.iter().any(|k| k.purpose == "test1"));
        assert!(keys.iter().any(|k| k.purpose == "test2"));
    }

    #[tokio::test]
    async fn test_security_types() {
        // Test Subject creation
        let subject = Subject {
            id: "user123".to_string(),
            name: "Test User".to_string(),
            subject_type: SubjectType::User,
            roles: vec!["admin".to_string()],
            clearance_level: Some(5),
            metadata: {
                let mut attrs = HashMap::new();
                attrs.insert("department".to_string(), "security".to_string());
                attrs.insert("clearance".to_string(), "high".to_string());
                attrs
            },
        };

        assert_eq!(subject.id, "user123");
        assert_eq!(subject.subject_type, SubjectType::User);
        assert_eq!(
            subject.metadata.get("department"),
            Some(&"security".to_string())
        );

        // Test Resource creation
        let resource = Resource {
            id: "resource456".to_string(),
            name: "database".to_string(),
            classification: ResourceClassification::Confidential,
            metadata: HashMap::new(),
        };

        assert_eq!(resource.id, "resource456");
        assert_eq!(
            resource.classification,
            ResourceClassification::Confidential
        );

        // Test Action creation
        let action = Action {
            action_type: ActionType::Read,
            description: "Reading user data".to_string(),
            risk_level: RiskLevel::Low,
            timestamp: chrono::Utc::now(),
        };

        assert_eq!(action.action_type, ActionType::Read);
        assert_eq!(action.risk_level, RiskLevel::Low);
    }

    #[tokio::test]
    async fn test_authorization_result() {
        let result = AuthorizationResult {
            permitted: true,
            authorized: true,
            reason: "User has required permissions".to_string(),
            additional_requirements: vec!["mfa".to_string()],
            risk_level: RiskLevel::Medium,
            audit_id: "audit123".to_string(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
        };

        assert!(result.permitted);
        assert!(result.authorized);
        assert_eq!(result.reason, "User has required permissions");
        assert_eq!(result.additional_requirements, vec!["mfa".to_string()]);
        assert_eq!(result.risk_level, RiskLevel::Medium);
        assert!(result.expires_at.is_some());
    }

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Low < RiskLevel::Medium);
        assert!(RiskLevel::Medium < RiskLevel::High);
        assert!(RiskLevel::High < RiskLevel::Critical);

        let levels = vec![
            RiskLevel::Critical,
            RiskLevel::Low,
            RiskLevel::High,
            RiskLevel::Medium,
        ];
        let mut sorted_levels = levels.clone();
        sorted_levels.sort();

        assert_eq!(
            sorted_levels,
            vec![
                RiskLevel::Low,
                RiskLevel::Medium,
                RiskLevel::High,
                RiskLevel::Critical
            ]
        );
    }

    #[test]
    fn test_resource_classification_ordering() {
        assert!(ResourceClassification::Public < ResourceClassification::Internal);
        assert!(ResourceClassification::Internal < ResourceClassification::Confidential);
        assert!(ResourceClassification::Confidential < ResourceClassification::Secret);
        assert!(ResourceClassification::Secret < ResourceClassification::TopSecret);
    }

    #[tokio::test]
    async fn test_error_handling() {
        // Test invalid key sizes
        let result = crypto_utils::BearDogCrypto::generate_secure_random(0);
        assert!(result.is_err());

        // Test decryption with wrong key
        let key1 = crypto_utils::BearDogCrypto::generate_secure_random(32)
            .expect("Failed to generate key1");
        let key2 = crypto_utils::BearDogCrypto::generate_secure_random(32)
            .expect("Failed to generate key2");
        let nonce = crypto_utils::BearDogCrypto::generate_secure_nonce(12)
            .expect("Failed to generate nonce");

        let plaintext = b"test data";
        let encrypted =
            crypto_utils::BearDogCrypto::encrypt_aes_gcm(&key1, plaintext, Some(&nonce))
                .expect("Failed to encrypt");
        let decrypt_result =
            crypto_utils::BearDogCrypto::decrypt_aes_gcm(&key2, &encrypted.0, &encrypted.1);
        assert!(decrypt_result.is_err());
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let auth_manager = std::sync::Arc::new(
            decentralized_auth::DecentralizedAuthManager::new(24)
                .expect("Failed to create auth manager"),
        );

        let mut handles = vec![];

        // Create multiple tokens concurrently
        for i in 0..10 {
            let manager_clone = std::sync::Arc::clone(&auth_manager);
            let handle = tokio::spawn(async move {
                let token = manager_clone
                    .create_auth_token(
                        &format!("user{i}"),
                        "test_service",
                        vec!["read".to_string()],
                        HashMap::new(),
                    )
                    .expect("Failed to create token");

                let is_valid = manager_clone
                    .verify_auth_token(&token)
                    .expect("Failed to verify");
                assert!(is_valid);
                i
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            let result = handle.await.expect("Task failed");
            assert!(result < 10);
        }
    }

    #[test]
    fn test_serialization() {
        let subject = Subject {
            id: "test_user".to_string(),
            name: "Test User".to_string(),
            subject_type: SubjectType::User,
            roles: vec![],
            clearance_level: None,
            metadata: HashMap::new(),
        };

        // Test JSON serialization
        let json_str = serde_json::to_string(&subject).expect("Failed to serialize");
        assert!(json_str.contains("test_user"));

        let deserialized: Subject = serde_json::from_str(&json_str).expect("Failed to deserialize");
        assert_eq!(deserialized.id, subject.id);
        assert_eq!(deserialized.subject_type, subject.subject_type);
    }

    #[tokio::test]
    async fn test_performance_characteristics() {
        let start = std::time::Instant::now();

        // Test encryption performance
        let key = crypto_utils::BearDogCrypto::generate_secure_random(32)
            .expect("Failed to generate key");
        let nonce = crypto_utils::BearDogCrypto::generate_secure_nonce(12)
            .expect("Failed to generate nonce");
        let plaintext = vec![0u8; 1024]; // 1KB of data

        for _ in 0..100 {
            let _encrypted =
                crypto_utils::BearDogCrypto::encrypt_aes_gcm(&key, &plaintext, Some(&nonce))
                    .expect("Failed to encrypt");
        }

        let duration = start.elapsed();

        // Should complete reasonably quickly (less than 1 second for 100 encryptions of 1KB)
        assert!(duration.as_secs() < 1);
    }

    #[tokio::test]
    async fn test_edge_cases() {
        // Test empty data encryption
        let key = crypto_utils::BearDogCrypto::generate_secure_random(32)
            .expect("Failed to generate key");
        let nonce = crypto_utils::BearDogCrypto::generate_secure_nonce(12)
            .expect("Failed to generate nonce");
        let empty_data = b"";

        let encrypted =
            crypto_utils::BearDogCrypto::encrypt_aes_gcm(&key, empty_data, Some(&nonce))
                .expect("Failed to encrypt empty data");
        let decrypted =
            crypto_utils::BearDogCrypto::decrypt_aes_gcm(&key, &encrypted.0, &encrypted.1)
                .expect("Failed to decrypt empty data");
        assert_eq!(decrypted, empty_data.to_vec());

        // Test very large data
        let large_data = vec![42u8; 10_000]; // 10KB
        let encrypted =
            crypto_utils::BearDogCrypto::encrypt_aes_gcm(&key, &large_data, Some(&nonce))
                .expect("Failed to encrypt large data");
        let decrypted =
            crypto_utils::BearDogCrypto::decrypt_aes_gcm(&key, &encrypted.0, &encrypted.1)
                .expect("Failed to decrypt large data");
        assert_eq!(decrypted, large_data);
    }
}
