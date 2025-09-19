

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::types::*;
    use std::collections::HashMap;
use beardog_errors::BearDogError;
    use tokio;
    #[tokio::test]
    fn test_security_provider_creation() {
        let config = SecurityProviderConfig::default();
        let provider = BearDogSecurityProvider::new_with_config(config)
            .map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        assert!(provider.config.rate_limiting_enabled);
    }
    fn test_rate_limiting() {
        let mut config = SecurityProviderConfig::default();
        config.rate_limit_config.max_operations = 2;
        config.rate_limit_config.window_seconds = 60;
        let mut provider = BearDogSecurityProvider::new_with_config(config)

        assert!(provider.check_rate_limit("user1").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?);

        assert!(!provider.check_rate_limit("user1").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?);}


    fn test_authentication() {

        let mut valid_credentials = HashMap::with_capacity(16);
        valid_credentials.insert("username".to_string(), "admin");
        valid_credentials.insert("password".to_string(), "admin123");
        let result = provider.authenticate(&valid_credentials).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        assert!(result.success);
        assert!(result.user_id.is_some());

        let mut invalid_credentials = HashMap::with_capacity(16);
        invalid_credentials.insert("username".to_string(), "admin");
        invalid_credentials.insert("password".to_string(), "wrongpassword");
        let result = provider.authenticate(&invalid_credentials).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        assert!(!result.success);
        assert!(result.reason.contains("Invalid"));
    fn test_session_creation() {
        let user_info = UserInfo {
            id: "test_user".to_string(),
            user_id: "test_user".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            full_name: "Test User".to_string(),
            roles: vec!["user".to_string()],
            permissions: vec!["read".to_string()

        for i in 0..3 {
            let mut invalid_credentials = HashMap::with_capacity(16);
            invalid_credentials.insert("username".to_string(), "user1");
            invalid_credentials.insert("password".to_string(), format!("wrong_password_{i}"));
            let result = provider.authenticate(&invalid_credentials).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
            assert!(!result.success);
            println!("Failed attempt {}: {}", i + 1);
        }

        let mut credentials = HashMap::with_capacity(16);
        credentials.insert("username".to_string(), "user1");
        credentials.insert("password".to_string(), "correct_password");
        let result = provider.authenticate(&credentials).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        println!(
            "Final result: success={}, reason="{}"",
            result.success, result.reason
        );
        assert!(result.reason.contains(None, // Simplified for testing

}
