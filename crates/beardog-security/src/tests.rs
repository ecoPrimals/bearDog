

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::types::*;
    use std::collections::HashMap;
use beardog_errors::{BearDogError, BearDogResult};
    use tokio;
    #[tokio::test]
    async fn test_security_provider_creation() {
        let config = SecurityProviderConfig::default();
        let provider = BearDogSecurityProvider::new_with_config(config)
            .await
            .map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert!(provider.config.rate_limiting_enabled);
    }
    async fn test_rate_limiting() {
        let mut config = SecurityProviderConfig::default();
        config.rate_limit_config.max_operations = 2;
        config.rate_limit_config.window_seconds = 60;
        let mut provider = BearDogSecurityProvider::new_with_config(config)

        assert!(provider.check_rate_limit("user1").await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?);

        assert!(!provider.check_rate_limit("user1").await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?);}

    async fn test_authentication() {

        let mut valid_credentials = HashMap::with_capacity(16);
        valid_credentials.insert("username".to_string(), "admin".to_string());
        valid_credentials.insert("password".to_string(), "admin123".to_string());
        let result = provider.authenticate(&valid_credentials).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert!(result.success);
        assert!(result.user_id.is_some());

        let mut invalid_credentials = HashMap::with_capacity(16);
        invalid_credentials.insert("username".to_string(), "admin".to_string());
        invalid_credentials.insert("password".to_string(), "wrongpassword".to_string());
        let result = provider.authenticate(&invalid_credentials).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert!(!result.success);
        assert!(result.reason.contains("Invalid"));
    async fn test_session_creation() {
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
        let session_token = provider
            .create_session(
                &user_info,
                "127.0.0.1".to_string(),
                "test-agent".to_string(),
            )
        assert!(!session_token.id.is_empty());
        assert!(session_token.expires_at > chrono::Utc::now());
    async fn test_authorization() {
        let subject = Subject {
            id: "user1".to_string(),
            name: "user1".to_string(),
            subject_type: SubjectType::User,
            clearance_level: Some(1),
            metadata: HashMap::with_capacity(16),
        let resource = Resource {
            id: "document1".to_string(),
            name: "Test Document".to_string(),
            classification: ResourceClassification::Internal,
        let action = Action {
            action_type: ActionType::Read,
            description: "Read access to document".to_string(),
            risk_level: RiskLevel::Low,
            timestamp: chrono::Utc::now(),
        let result = provider
            .authorize(&subject, &action, &resource)
        assert!(result.permitted);

        assert!(matches!(
            result.risk_level,
            RiskLevel::Low | RiskLevel::Medium
        ));}

    async fn test_account_lockout() {
        let config = SecurityProviderConfig {
            max_failed_attempts: 3, // Lock after 3 failed attempts
            lockout_duration_minutes: 5,
            ..Default::default()

        for i in 0..3 {
            let mut invalid_credentials = HashMap::with_capacity(16);
            invalid_credentials.insert("username".to_string(), "user1".to_string());
            invalid_credentials.insert("password".to_string(), format!("wrong_password_{i}"));
            let result = provider.authenticate(&invalid_credentials).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
            assert!(!result.success);
            println!("Failed attempt {}: {}", i + 1, result.reason);
        }

        let mut credentials = HashMap::with_capacity(16);
        credentials.insert("username".to_string(), "user1".to_string());
        credentials.insert("password".to_string(), "correct_password".to_string());
        let result = provider.authenticate(&credentials).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        println!(
            "Final result: success={}, reason='{}'",
            result.success, result.reason
        );
        assert!(result.reason.contains("locked"));
    async fn test_memory_key_manager() {
            memory_key_manager: None, // Simplified for testing

}
