//! Security provider tests
//!
//! Comprehensive tests for security provider functionality

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {

    use crate::types::*;
    use std::collections::HashMap;
    use tokio;

    #[tokio::test]
    async fn test_security_provider_creation() {
        let config = SecurityProviderConfig::default();
        let provider = BearDogSecurityProvider::new_with_config(config)
            .await
            .unwrap();
        assert!(provider.config.rate_limiting_enabled);
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let mut config = SecurityProviderConfig::default();
        config.rate_limit_config.max_operations = 2;
        config.rate_limit_config.window_seconds = 60;

        let mut provider = BearDogSecurityProvider::new_with_config(config)
            .await
            .unwrap();

        // First request should pass
        assert!(provider.check_rate_limit("user1").await.unwrap());

        // Second request should pass
        assert!(provider.check_rate_limit("user1").await.unwrap());

        // Third request should fail (rate limited)
        assert!(!provider.check_rate_limit("user1").await.unwrap());
    }

    #[tokio::test]
    async fn test_authentication() {
        let config = SecurityProviderConfig::default();
        let provider = BearDogSecurityProvider::new_with_config(config)
            .await
            .unwrap();

        // Test valid credentials (hardcoded admin user)
        let mut valid_credentials = HashMap::new();
        valid_credentials.insert("username".to_string(), "admin".to_string());
        valid_credentials.insert("password".to_string(), "admin123".to_string());

        let result = provider.authenticate(&valid_credentials).await.unwrap();
        assert!(result.success);
        assert!(result.user_id.is_some());

        // Test invalid credentials
        let mut invalid_credentials = HashMap::new();
        invalid_credentials.insert("username".to_string(), "admin".to_string());
        invalid_credentials.insert("password".to_string(), "wrongpassword".to_string());

        let result = provider.authenticate(&invalid_credentials).await.unwrap();
        assert!(!result.success);
        assert!(result.reason.contains("Invalid"));
    }

    #[tokio::test]
    async fn test_session_creation() {
        let config = SecurityProviderConfig::default();
        let provider = BearDogSecurityProvider::new_with_config(config)
            .await
            .unwrap();

        let user_info = UserInfo {
            id: "test_user".to_string(),
            user_id: "test_user".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            full_name: "Test User".to_string(),
            roles: vec!["user".to_string()],
            permissions: vec!["read".to_string()],
            status: AccountStatus::Active,
            last_login: None,
        };

        let _session_config = SessionConfig {
            max_age_seconds: 3600,
            require_mfa: false,
            ip_binding: false,
            concurrent_sessions: 5,
        };

        let session_token = provider
            .create_session(
                &user_info,
                "127.0.0.1".to_string(),
                "test-agent".to_string(),
            )
            .await
            .unwrap();

        assert!(!session_token.id.is_empty());
        assert!(session_token.expires_at > chrono::Utc::now());
    }

    #[tokio::test]
    async fn test_authorization() {
        let config = SecurityProviderConfig::default();
        let provider = BearDogSecurityProvider::new_with_config(config)
            .await
            .unwrap();

        let subject = Subject {
            id: "user1".to_string(),
            name: "user1".to_string(),
            subject_type: SubjectType::User,
            roles: vec!["user".to_string()],
            clearance_level: Some(1),
            metadata: HashMap::new(),
        };

        let resource = Resource {
            id: "document1".to_string(),
            name: "Test Document".to_string(),
            classification: ResourceClassification::Internal,
            metadata: HashMap::new(),
        };

        let action = Action {
            action_type: ActionType::Read,
            description: "Read access to document".to_string(),
            risk_level: RiskLevel::Low,
            timestamp: chrono::Utc::now(),
        };

        let result = provider
            .authorize(&subject, &action, &resource)
            .await
            .unwrap();
        assert!(result.permitted);
        assert_eq!(result.risk_level, RiskLevel::Low);
    }

    #[tokio::test]
    async fn test_account_lockout() {
        let config = SecurityProviderConfig {
            max_failed_attempts: 3, // Lock after 3 failed attempts
            lockout_duration_minutes: 5,
            ..Default::default()
        };

        let provider = BearDogSecurityProvider::new_with_config(config)
            .await
            .unwrap();

        // Test failed attempts - need exactly max_failed_attempts (3) to trigger lockout
        for i in 0..3 {
            let mut invalid_credentials = HashMap::new();
            invalid_credentials.insert("username".to_string(), "user1".to_string());
            invalid_credentials.insert("password".to_string(), format!("wrong_password_{i}"));

            let result = provider.authenticate(&invalid_credentials).await.unwrap();
            assert!(!result.success);
            println!("Failed attempt {}: {}", i + 1, result.reason);
        }

        // Account should now be locked after max attempts
        let mut credentials = HashMap::new();
        credentials.insert("username".to_string(), "user1".to_string());
        credentials.insert("password".to_string(), "correct_password".to_string());

        let result = provider.authenticate(&credentials).await.unwrap();
        println!(
            "Final result: success={}, reason='{}'",
            result.success, result.reason
        );
        assert!(!result.success);
        assert!(result.reason.contains("locked"));
    }

    #[tokio::test]
    async fn test_memory_key_manager() {
        let config = SecurityProviderConfig {
            memory_key_manager: None, // Simplified for testing
            ..Default::default()
        };

        let provider = BearDogSecurityProvider::new_with_config(config)
            .await
            .unwrap();

        // Test basic security provider functionality
        assert!(provider.config.rate_limiting_enabled);

        // Test key generation would require actual implementation
        // This is a placeholder until the full integration is complete
    }
}
