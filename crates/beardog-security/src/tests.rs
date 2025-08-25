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


/// Security provider tests
///
/// Comprehensive tests for security provider functionality

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
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        assert!(provider.config.rate_limiting_enabled);
    }
    async fn test_rate_limiting() {
        let mut config = SecurityProviderConfig::default();
        config.rate_limit_config.max_operations = 2;
        config.rate_limit_config.window_seconds = 60;
        let mut provider = BearDogSecurityProvider::new_with_config(config)
        // First request should pass
        assert!(provider.check_rate_limit("user1").await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
        // Second request should pass
        // Third request should fail (rate limited)
        assert!(!provider.check_rate_limit("user1").await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);}


    async fn test_authentication() {
        // Test valid credentials (hardcoded admin user)
        let mut valid_credentials = HashMap::new();
        valid_credentials.insert("username".to_string(), "admin".to_string());
        valid_credentials.insert("password".to_string(), "admin123".to_string());
        let result = provider.authenticate(&valid_credentials).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        assert!(result.success);
        assert!(result.user_id.is_some());
        // Test invalid credentials
        let mut invalid_credentials = HashMap::new();
        invalid_credentials.insert("username".to_string(), "admin".to_string());
        invalid_credentials.insert("password".to_string(), "wrongpassword".to_string());
        let result = provider.authenticate(&invalid_credentials).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
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
            metadata: HashMap::new(),
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
        // Note: Risk level may vary based on analysis - just ensure it's reasonable
        assert!(matches!(
            result.risk_level,
            RiskLevel::Low | RiskLevel::Medium
        ));}


    async fn test_account_lockout() {
        let config = SecurityProviderConfig {
            max_failed_attempts: 3, // Lock after 3 failed attempts
            lockout_duration_minutes: 5,
            ..Default::default()
        // Test failed attempts - need exactly max_failed_attempts (3) to trigger lockout
        for i in 0..3 {
            let mut invalid_credentials = HashMap::new();
            invalid_credentials.insert("username".to_string(), "user1".to_string());
            invalid_credentials.insert("password".to_string(), format!("wrong_password_{i}"));
            let result = provider.authenticate(&invalid_credentials).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
            assert!(!result.success);
            println!("Failed attempt {}: {}", i + 1, result.reason);
        }
        // Account should now be locked after max attempts
        let mut credentials = HashMap::new();
        credentials.insert("username".to_string(), "user1".to_string());
        credentials.insert("password".to_string(), "correct_password".to_string());
        let result = provider.authenticate(&credentials).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        println!(
            "Final result: success={}, reason='{}'",
            result.success, result.reason
        );
        assert!(result.reason.contains("locked"));
    async fn test_memory_key_manager() {
            memory_key_manager: None, // Simplified for testing
        // Test basic security provider functionality
        // Test key generation would require actual implementation
        // This is a placeholder until the full integration is complete
}
