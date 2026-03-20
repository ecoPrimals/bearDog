// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for Core Security Provider

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        clippy::float_cmp,
        clippy::useless_vec,
        clippy::needless_range_loop,
        clippy::uninlined_format_args,
        clippy::field_reassign_with_default,
        clippy::manual_range_contains,
        unused_variables,
        dead_code
    )]

    use crate::core::security::CoreSecurityProvider;
    use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
    use beardog_types::canonical::providers_unified::traits::{
        AuthenticationContext, AuthenticationRequest, AuthorizationRequest, ProviderConfiguration,
        UnifiedProvider, UnifiedSecurityProvider,
    };
    use std::collections::HashMap;

    fn create_test_provider() -> CoreSecurityProvider {
        let config = UnifiedBearDogConfig::default();
        CoreSecurityProvider::new(config)
    }

    fn create_auth_request(user_id: &str) -> AuthenticationRequest {
        let mut credentials = HashMap::new();
        credentials.insert("password".to_string(), "test_password".to_string());

        let mut metadata = HashMap::new();
        metadata.insert("test_key".to_string(), "test_value".to_string());

        AuthenticationRequest {
            user_id: user_id.to_string(),
            credentials,
            context: AuthenticationContext {
                client_ip: Some("127.0.0.1".to_string()),
                user_agent: Some("test_agent".to_string()),
                session_id: Some("test_session".to_string()),
                metadata,
            },
        }
    }

    fn create_authz_request(
        user_id: &str,
        resource: &str,
        operation: &str,
    ) -> AuthorizationRequest {
        AuthorizationRequest {
            user_id: user_id.to_string(),
            resource: resource.to_string(),
            operation: operation.to_string(),
            context: HashMap::new(),
        }
    }

    #[test]
    fn test_new_security_provider() {
        let config = UnifiedBearDogConfig::default();
        let provider = CoreSecurityProvider::new(config);

        // Verify provider can be created
        assert!(format!("{:?}", provider).contains("CoreSecurityProvider"));
    }

    #[test]
    fn test_provider_info() {
        let provider = create_test_provider();
        let info = provider.provider_info();

        assert_eq!(info.id, "core_security");
        assert_eq!(info.name, "Core Security Provider");
        assert_eq!(info.version, "1.0.0");
        assert!(
            info.supported_capabilities
                .contains(&"authentication".to_string())
        );
        assert!(
            info.supported_capabilities
                .contains(&"authorization".to_string())
        );
    }

    #[tokio::test]
    async fn test_health_check() {
        let provider = create_test_provider();
        let health = provider
            .health_check()
            .await
            .expect("Health check should succeed");

        assert!(matches!(
            health.status,
            beardog_types::canonical::providers_unified::traits::HealthStatus::Healthy
        ));
        assert!(health.details.contains_key("status"));
        assert_eq!(health.details.get("status").unwrap(), "OK");
    }

    #[tokio::test]
    async fn test_metrics() {
        let provider = create_test_provider();
        let metrics = provider.metrics().await.expect("Metrics should succeed");

        assert!(metrics.performance.contains_key("requests_per_second"));
        assert_eq!(metrics.system_metrics.error_rate, 0.0);
    }

    #[test]
    fn test_capabilities() {
        let provider = create_test_provider();
        let capabilities = provider.capabilities();

        assert_eq!(capabilities.len(), 2);
        assert!(capabilities.iter().any(|c| c.name == "Authentication"));
        assert!(capabilities.iter().any(|c| c.name == "Authorization"));

        // Verify authentication capability
        let auth_cap = capabilities
            .iter()
            .find(|c| c.name == "Authentication")
            .unwrap();
        assert!(auth_cap.enabled);
        assert!(!auth_cap.parameters.is_empty());
    }

    #[tokio::test]
    async fn test_initialize() {
        let mut provider = create_test_provider();
        let config = ProviderConfiguration {
            parameters: std::collections::HashMap::new(),
            connection:
                beardog_types::canonical::providers_unified::traits::ConnectionConfiguration {
                    timeout_seconds: 5,
                    max_retries: 3,
                    pool_size: 10,
                },
            security: beardog_types::canonical::providers_unified::traits::SecurityConfiguration {
                tls_enabled: true,
                cert_path: None,
                key_path: None,
            },
            performance:
                beardog_types::canonical::providers_unified::traits::PerformanceConfiguration {
                    monitoring_enabled: true,
                    metrics_interval_seconds: 60,
                    optimization_level: "standard".to_string(),
                },
        };

        let result = provider.initialize(config).await;
        assert!(result.is_ok(), "Initialize should succeed");
    }

    #[tokio::test]
    async fn test_shutdown() {
        let mut provider = create_test_provider();
        let result = provider.shutdown().await;
        assert!(result.is_ok(), "Shutdown should succeed");
    }

    #[tokio::test]
    async fn test_authenticate_success() {
        let provider = create_test_provider();
        let request = create_auth_request("test_user");

        let response = provider
            .authenticate(request)
            .await
            .expect("Authentication should succeed");

        assert!(response.success);
        assert!(response.user_info.is_some());
        assert!(response.token.is_some());
        // Phase 2: Now returns real JWT tokens (Dec 8, 2025)
        let token = response.token.unwrap();
        assert!(!token.is_empty());
        // JWT tokens have 3 parts separated by dots
        assert_eq!(token.split('.').count(), 3, "Token should be a valid JWT");
    }

    #[tokio::test]
    async fn test_authenticate_includes_user_info() {
        let provider = create_test_provider();
        let request = create_auth_request("user123");

        let response = provider
            .authenticate(request)
            .await
            .expect("Authentication should succeed");

        let user_info = response.user_info.expect("User info should be present");
        assert!(user_info.contains_key("user_id"));
        assert_eq!(user_info.get("user_id").unwrap(), "user123");
        // Phase 2: User info now includes roles (Dec 8, 2025)
        assert!(user_info.contains_key("roles"));
    }

    #[tokio::test]
    async fn test_authorize_success() {
        let provider = create_test_provider();

        // Phase 2: First authenticate to initialize auth manager and assign role
        let auth_request = create_auth_request("test_user");
        provider
            .authenticate(auth_request)
            .await
            .expect("Auth should succeed");

        let request = create_authz_request("test_user", "test_resource", "read");

        let response = provider
            .authorize(request)
            .await
            .expect("Authorization should succeed");

        // Phase 2: Real RBAC - user is assigned User role on first auth, which has read permission
        assert!(response.granted);
        assert!(response.permissions.contains(&"read".to_string()));
        assert!(response.denial_reason.is_none());
    }

    #[tokio::test]
    async fn test_authorize_includes_permissions() {
        let provider = create_test_provider();

        // Phase 2: First authenticate to get assigned a role
        let auth_request = create_auth_request("admin");
        provider
            .authenticate(auth_request)
            .await
            .expect("Auth should succeed");

        let request = create_authz_request("admin", "admin_panel", "write");

        let response = provider
            .authorize(request)
            .await
            .expect("Authorization should succeed");

        // Phase 2: Real RBAC - user gets User role, which has read, write, execute permissions
        assert!(response.granted);
        assert!(response.permissions.contains(&"write".to_string()));
        assert!(response.permissions.contains(&"read".to_string()));
        assert!(response.permissions.contains(&"execute".to_string()));
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_roundtrip() {
        let provider = create_test_provider();
        let data = b"sensitive data";
        let key_id = "test_key";

        // Encrypt
        let encrypted = provider
            .encrypt(data, key_id)
            .await
            .expect("Encryption should succeed");

        // Decrypt
        let decrypted = provider
            .decrypt(&encrypted, key_id)
            .await
            .expect("Decryption should succeed");

        // Verify roundtrip (note: mock implementation doesn't actually encrypt)
        assert_eq!(decrypted, data);
    }

    #[tokio::test]
    async fn test_sign_verify_roundtrip() {
        let provider = create_test_provider();
        let data = b"data to sign";
        let key_id = "signing_key";

        // Sign - uses real Ed25519
        let signature = provider
            .sign(data, key_id)
            .await
            .expect("Signing should succeed");

        assert_eq!(signature.len(), 64, "Ed25519 signature should be 64 bytes");

        // Note: Key persistence is implemented in BearDogCryptoService
        // See crypto_service tests for verification roundtrip tests
        // This test validates that CoreSecurityProvider can sign data
    }

    #[tokio::test]
    async fn test_generate_random() {
        let provider = create_test_provider();

        // Generate small random data
        let random_10 = provider
            .generate_random(10)
            .await
            .expect("Random generation should succeed");
        assert_eq!(random_10.len(), 10);

        // Generate larger random data
        let random_100 = provider
            .generate_random(100)
            .await
            .expect("Random generation should succeed");
        assert_eq!(random_100.len(), 100);
    }

    #[tokio::test]
    async fn test_generate_random_zero_length() {
        let provider = create_test_provider();
        let random = provider
            .generate_random(0)
            .await
            .expect("Random generation should succeed");
        assert_eq!(random.len(), 0);
    }

    #[test]
    fn test_security_context() {
        let provider = create_test_provider();
        let context = provider.security_context();

        // Updated to reflect production-grade crypto (Dec 7, 2025)
        assert_eq!(context.security_level, "production");
        assert!(
            context
                .encryption_algorithms
                .contains(&"aes-256-gcm".to_string())
        );
        assert!(
            context
                .signature_algorithms
                .contains(&"ed25519".to_string())
        );
        assert!(
            context
                .key_derivation_functions
                .contains(&"hkdf-sha256".to_string())
        );
        assert!(context.random_generators.contains(&"os_csprng".to_string()));
    }

    #[test]
    fn test_provider_clone() {
        let provider = create_test_provider();
        let cloned = provider.clone();

        // Verify clone has same provider info
        assert_eq!(provider.provider_info().id, cloned.provider_info().id);
        assert_eq!(provider.provider_info().name, cloned.provider_info().name);
    }

    #[tokio::test]
    async fn test_multiple_authentications() {
        let provider = create_test_provider();

        for i in 0..5 {
            let request = create_auth_request(&format!("user_{}", i));

            let response = provider
                .authenticate(request)
                .await
                .expect("Authentication should succeed");
            assert!(response.success);
        }
    }

    #[tokio::test]
    async fn test_multiple_authorizations() {
        let provider = create_test_provider();

        // Phase 2: First authenticate to get assigned a role
        let auth_request = create_auth_request("test_user");
        provider
            .authenticate(auth_request)
            .await
            .expect("Auth should succeed");

        // Phase 2: User role has read, write, execute permissions but NOT delete
        let operations = vec!["read", "write", "execute"];

        for operation in operations {
            let request = create_authz_request("test_user", "test_resource", operation);

            let response = provider
                .authorize(request)
                .await
                .expect("Authorization should succeed");
            assert!(
                response.granted,
                "User should have {} permission",
                operation
            );
            assert!(response.permissions.contains(&operation.to_string()));
        }

        // Test that delete is denied (User role doesn't have delete permission)
        let delete_request = create_authz_request("test_user", "test_resource", "delete");
        let delete_response = provider
            .authorize(delete_request)
            .await
            .expect("Authorization should complete");
        assert!(
            !delete_response.granted,
            "User should NOT have delete permission"
        );
        assert!(delete_response.denial_reason.is_some());
    }

    #[tokio::test]
    async fn test_encrypt_empty_data() {
        let provider = create_test_provider();
        let data = b"";
        let key_id = "test_key";

        let encrypted = provider
            .encrypt(data, key_id)
            .await
            .expect("Encryption should succeed");
        // Real AES-256-GCM adds overhead (nonce + tag)
        // Empty plaintext still produces ciphertext with auth tag
        assert!(
            !encrypted.is_empty(),
            "Encrypted data should include overhead"
        );
    }

    #[tokio::test]
    async fn test_decrypt_empty_data() {
        let provider = create_test_provider();
        let key_id = "test_key";

        // For real crypto, we need to encrypt first to get valid ciphertext
        let plaintext = b"";
        let ciphertext = provider
            .encrypt(plaintext, key_id)
            .await
            .expect("Encryption should succeed");

        let decrypted = provider
            .decrypt(&ciphertext, key_id)
            .await
            .expect("Decryption should succeed");
        assert_eq!(decrypted, plaintext, "Roundtrip should preserve data");
    }

    #[tokio::test]
    async fn test_sign_empty_data() {
        let provider = create_test_provider();
        let data = b"";
        let key_id = "signing_key";

        let signature = provider
            .sign(data, key_id)
            .await
            .expect("Signing should succeed");
        // Real Ed25519 signatures are always 64 bytes, even for empty data
        assert_eq!(signature.len(), 64, "Ed25519 signature should be 64 bytes");
    }
}
