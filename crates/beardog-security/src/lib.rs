// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// BearDog Security Crate
///
/// **UNIFIED SECURITY ARCHITECTURE** - Canonical security for all BearDog operations
/// This crate provides BearDog's comprehensive security system with:
/// - **Unified Security Types**: Single source of truth for security operations
/// - **Canonical Error Handling**: Full integration with beardog-errors
/// - **Modular Architecture**: Clean separation of security concerns
/// - **Zero-Copy Operations**: High-performance security operations

use beardog_errors::{BearDogError, BearDogResult};
// Core modules that exist
pub mod crypto_utils;
pub mod encryption;
pub mod handlers;
pub mod recovery;
pub mod simd_crypto;
pub mod quantum_crypto;
pub mod types;
pub mod zero_copy_crypto;
// Re-exports for convenience
pub use crypto_utils::*;
pub use types::*;
pub use encryption::*;
// MIGRATION COMPLETE: Removed references to deleted modules
// - address_management: Functionality moved to canonical types
// - crypto_utils: Functionality moved to canonical types  
// - decentralized_auth: Functionality moved to canonical types
// - memory_key_manager: Functionality moved to canonical types
// - hsm_providers: Functionality moved to canonical types
// - zero_copy: Functionality moved to canonical types
// - zero_cost: Functionality moved to canonical types
#[cfg(test)]
mod comprehensive_tests {};


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
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to generate key", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to generate key", e))
})?;
        assert_eq!(key.len(), 32);
        let key2 = crypto_utils::BearDogCrypto::generate_secure_random(32)
        assert_ne!(key, key2); // Should be different
        // Test encryption/decryption
        let plaintext = b"test data for encryption";
        let nonce = crypto_utils::BearDogCrypto::generate_secure_nonce(12)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to generate nonce", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to generate nonce", e))
})?;
        let encrypted = crypto_utils::BearDogCrypto::encrypt_aes_gcm(&key, plaintext, Some(&nonce))
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to encrypt", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to encrypt", e))
})?;
        assert_ne!(encrypted.0, plaintext.to_vec());
        let decrypted =
            crypto_utils::BearDogCrypto::decrypt_aes_gcm(&key, &encrypted.0, &encrypted.1)
                .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to decrypt", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to decrypt", e))
})?;
        assert_eq!(decrypted, plaintext.to_vec());
    }
    async fn test_decentralized_auth_manager() {
        let auth_manager = decentralized_auth::DecentralizedAuthManager::new(24)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create auth manager", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to create auth manager", e))
})?;
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
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create token", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to create token", e))
})?;
        assert_eq!(token.claims.subject, "test_user");
        assert_eq!(token.claims.audience, "test_service");
        assert_eq!(token.claims.permissions, permissions);
        assert!(!token.signature.is_empty());
        // Test token verification
        let is_valid = auth_manager
            .verify_auth_token(&token)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to verify token", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to verify token", e))
})?;
        assert!(is_valid);
        // Token refresh functionality not yet implemented
        // NOTE: Token refresh implemented in DecentralizedAuthManager::refresh_token()
        // let refreshed_token = auth_manager
        //     .refresh_token(&token)
        //     .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to refresh token", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to refresh token", e))
})?;
        // assert_eq!(refreshed_token.claims.subject, token.claims.subject);
        // assert_eq!(refreshed_token.claims.audience, token.claims.audience);}


    async fn test_memory_key_manager() {
        let config = memory_key_manager::MemoryKeyConfig::default();
        let key_manager = memory_key_manager::MemoryKeyManager::new(config)
            .await
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create key manager", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to create key manager", e))
})?;
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
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to store key", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to store key", e))
})?;
        let retrieved = key_manager
            .get_key(&key_id)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to retrieve key", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to retrieve key", e))
})?;
        assert_eq!(retrieved, key_data.to_vec());
        // Test key deletion
        key_manager
            .delete_key(&key_id)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to delete key", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to delete key", e))
})?;
        assert!(key_manager.get_key(&key_id).await.is_err());
        // Test key listing
        let metadata1 = memory_key_manager::KeyMetadata {
            purpose: "test1".to_string(),
        let metadata2 = memory_key_manager::KeyMetadata {
            purpose: "test2".to_string(),
            .store_key(b"data1", metadata1)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to store key1", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to store key1", e))
})?;
            .store_key(b"data2", metadata2)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to store key2", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to store key2", e))
})?;
        let keys = key_manager.list_keys().await.map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to list keys", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to list keys", e))
})?;
        assert_eq!(keys.len(), 2);
        // Check that we have keys with the expected purposes
        assert!(keys.iter().any(|k| k.purpose == "test1"));
        assert!(keys.iter().any(|k| k.purpose == "test2"));
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
        assert_eq!(resource.id, "resource456");
            resource.classification,
            ResourceClassification::Confidential
        // Test Action creation
        let action = Action {
            action_type: ActionType::Read,
            description: "Reading user data".to_string(),
            risk_level: RiskLevel::Low,
            timestamp: chrono::Utc::now(),
        assert_eq!(action.action_type, ActionType::Read);
        assert_eq!(action.risk_level, RiskLevel::Low);
    async fn test_authorization_result() {
        let result = AuthorizationResult {
            permitted: true,
            authorized: true,
            reason: "User has required permissions".to_string(),
            additional_requirements: vec!["mfa".to_string()],
            risk_level: RiskLevel::Medium,
            audit_id: "audit123".to_string(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
        assert!(result.permitted);
        assert!(result.authorized);
        assert_eq!(result.reason, "User has required permissions");
        assert_eq!(result.additional_requirements, vec!["mfa".to_string()]);
        assert_eq!(result.risk_level, RiskLevel::Medium);
        assert!(result.expires_at.is_some());
    #[test]}


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
            sorted_levels,
            vec![
                RiskLevel::Low,
                RiskLevel::Medium,
                RiskLevel::High,
                RiskLevel::Critical
            ]
    fn test_resource_classification_ordering() {
        assert!(ResourceClassification::Public < ResourceClassification::Internal);
        assert!(ResourceClassification::Internal < ResourceClassification::Confidential);
        assert!(ResourceClassification::Confidential < ResourceClassification::Secret);
        assert!(ResourceClassification::Secret < ResourceClassification::TopSecret);}


    async fn test_error_handling() {
        // Test invalid key sizes
        let result = crypto_utils::BearDogCrypto::generate_secure_random(0);
        assert!(result.is_err());
        // Test decryption with wrong key
        let key1 = crypto_utils::BearDogCrypto::generate_secure_random(32)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to generate key1", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to generate key1", e))
})?;
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to generate key2", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to generate key2", e))
})?;
        let plaintext = b"test data";
        let encrypted =
            crypto_utils::BearDogCrypto::encrypt_aes_gcm(&key1, plaintext, Some(&nonce))
                .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to encrypt", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to encrypt", e))
})?;
        let decrypt_result =
            crypto_utils::BearDogCrypto::decrypt_aes_gcm(&key2, &encrypted.0, &encrypted.1);
        assert!(decrypt_result.is_err());
    async fn test_concurrent_operations() {
        let auth_manager = std::sync::Arc::new(
            decentralized_auth::DecentralizedAuthManager::new(24)
                .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create auth manager", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to create auth manager", e))
})?,
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
                    .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create token", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to create token", e))
})?;
                let is_valid = manager_clone
                    .verify_auth_token(&token)
                    .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to verify", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to verify", e))
})?;
                assert!(is_valid);
                i
            });
            handles.push(handle);
        }
        // Wait for all tasks to complete
        for handle in handles {
            let result = handle.await.map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Task failed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Task failed", e))
})?;
            assert!(result < 10);
    fn test_serialization() {
            id: "test_user".to_string(),
            roles: vec![],
            clearance_level: None,
        // Test JSON serialization
        let json_str = serde_json::to_string(&subject).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to serialize", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to serialize", e))
})?;
        assert!(json_str.contains("test_user"));
        let deserialized: Subject = serde_json::from_str(&json_str).map_err(|e| {
    tracing::error!("JSON parsing failed ({}): {}", "Failed to deserialize", e);
    beardog_errors::BearDogError::ValidationError(format!("JSON parsing error ({}): {}", "Failed to deserialize", e))
})?;
        assert_eq!(deserialized.id, subject.id);
        assert_eq!(deserialized.subject_type, subject.subject_type);}


    async fn test_performance_characteristics() {
        let start = std::time::Instant::now();
        // Test encryption performance
        let plaintext = vec![0u8; 1024]; // 1KB of data
        for _ in 0..100 {
            let _encrypted =
                crypto_utils::BearDogCrypto::encrypt_aes_gcm(&key, &plaintext, Some(&nonce))
                    .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to encrypt", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to encrypt", e))
})?;
        let duration = start.elapsed();
        // Should complete reasonably quickly (less than 1 second for 100 encryptions of 1KB)
        assert!(duration.as_secs() < 1);
    async fn test_edge_cases() {
        // Test empty data encryption
        let empty_data = b"";
            crypto_utils::BearDogCrypto::encrypt_aes_gcm(&key, empty_data, Some(&nonce))
                .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to encrypt empty data", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to encrypt empty data", e))
})?;
                .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to decrypt empty data", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to decrypt empty data", e))
})?;
        assert_eq!(decrypted, empty_data.to_vec());
        // Test very large data
        let large_data = vec![42u8; 10_000]; // 10KB
            crypto_utils::BearDogCrypto::encrypt_aes_gcm(&key, &large_data, Some(&nonce))
                .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to encrypt large data", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to encrypt large data", e))
})?;
                .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to decrypt large data", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to decrypt large data", e))
})?;
        assert_eq!(decrypted, large_data);
}
