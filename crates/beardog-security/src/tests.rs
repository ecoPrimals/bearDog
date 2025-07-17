//! Security provider tests
//!
//! Comprehensive tests for security provider functionality

#[cfg(test)]
mod tests {
    
    
    use crate::types::*;
    use tokio;

    #[tokio::test]
    async fn test_security_provider_creation() {
        let config = SecurityProviderConfig::default();
        let provider = BearDogSecurityProvider::new(config).await.unwrap();
        assert!(provider.config.rate_limiting_enabled);
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let mut config = SecurityProviderConfig::default();
        config.rate_limit.max_requests_per_minute = 2;
        config.rate_limit.window_seconds = 60;

        let mut provider = BearDogSecurityProvider::new(config).await.unwrap();

        // First request should pass
        assert!(provider.check_rate_limit("user1").await.unwrap());

        // Second request should pass
        assert!(provider.check_rate_limit("user1").await.unwrap());

        // Third request should fail
        assert!(!provider.check_rate_limit("user1").await.unwrap());
    }

    #[tokio::test]
    async fn test_authentication() {
        let config = SecurityProviderConfig::default();
        let provider = BearDogSecurityProvider::new(config).await.unwrap();

        // Test valid credentials
        let result = provider.authenticate("admin", "admin123").await.unwrap();
        assert!(result.success);
        assert!(result.session_id.is_some());

        // Test invalid credentials
        let result = provider
            .authenticate("admin", "wrong_password")
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.session_id.is_none());
    }

    #[tokio::test]
    async fn test_session_creation() {
        let config = SecurityProviderConfig::default();
        let mut provider = BearDogSecurityProvider::new(config).await.unwrap();

        let session = provider
            .create_session("user1", Some("127.0.0.1".to_string()))
            .await
            .unwrap();
        assert_eq!(session.user_id, "user1");
        assert!(session.is_active);
        assert_eq!(session.ip_address, Some("127.0.0.1".to_string()));

        // Test session validation
        assert!(provider.validate_session(&session.id).await.unwrap());
    }

    #[tokio::test]
    async fn test_authorization() {
        let config = SecurityProviderConfig::default();
        let provider = BearDogSecurityProvider::new(config).await.unwrap();

        let subject = Subject {
            id: "user1".to_string(),
            subject_type: SubjectType::User,
            roles: vec!["user".to_string()],
            clearance_level: Some(1),
            attributes: std::collections::HashMap::new(),
        };

        let resource = Resource {
            id: "document1".to_string(),
            resource_type: "document".to_string(),
            owner: Some("user1".to_string()),
            classification: ResourceClassification::Internal,
            attributes: std::collections::HashMap::new(),
        };

        let action = Action {
            action_type: ActionType::Read,
            timestamp: chrono::Utc::now(),
            context: std::collections::HashMap::new(),
            source_ip: Some("127.0.0.1".to_string()),
        };

        let result = provider
            .authorize(&subject, &resource, &action)
            .await
            .unwrap();
        assert!(result.permitted);
        assert_eq!(result.risk_level, RiskLevel::Low);
    }

    #[tokio::test]
    async fn test_account_lockout() {
        let mut config = SecurityProviderConfig::default();
        config.max_failed_attempts = 2;
        config.lockout_duration_minutes = 5;

        let provider = BearDogSecurityProvider::new(config).await.unwrap();

        // Test failed attempts
        for _ in 0..3 {
            let result = provider
                .authenticate("user1", "wrong_password")
                .await
                .unwrap();
            assert!(!result.success);
        }

        // Account should be locked after max attempts
        let result = provider
            .authenticate("user1", "correct_password")
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.reason.contains("locked"));
    }

    #[tokio::test]
    async fn test_memory_key_manager() {
        let mut config = SecurityProviderConfig::default();
        config.enable_memory_key_manager = true;

        let provider = BearDogSecurityProvider::new(config).await.unwrap();
        assert!(provider.is_standalone_mode());

        // Test key generation
        let key_id = provider
            .generate_key("AES-256", "encryption", "user1")
            .await
            .unwrap();
        assert!(!key_id.is_empty());
    }
}
