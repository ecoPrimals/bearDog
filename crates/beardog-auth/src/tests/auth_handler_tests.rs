use beardog_auth::auth::handlers::{AuthenticationHandler, SessionData};
use beardog_errors::BearDogError;
use beardog_types::canonical::{
    configuration::consolidated::BearDogCanonicalConfig, providers::ProviderConfig,
};

#[tokio::test]
async fn test_auth_handler_creation() -> Result<(), BearDogError> {
    // Create a real auth handler with default configuration
    let config = BearDogCanonicalConfig::default();
    let provider_config = ProviderConfig::default();

    let handler = AuthenticationHandler::new(&config, &provider_config)?;

    // Verify the handler was created successfully
    assert!(handler.is_initialized());
    Ok(())
}

#[tokio::test]
async fn test_authorization_validation() -> Result<(), BearDogError> {
    // Create a real auth handler and test validation
    let config = BearDogCanonicalConfig::default();
    let provider_config = ProviderConfig::default();

    let handler = AuthenticationHandler::new(&config, &provider_config)?;

    // Create test session data
    let session = SessionData {
        session_id: "test-session-001".to_string(),
        user_id: "test-user-001".to_string(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
        permissions: vec!["read".to_string(), "write".to_string()],
    };

    // Test validation with real session data
    let validation_result = handler.validate_session(&session)?;
    assert!(
        validation_result,
        "Session validation should succeed for valid session"
    );

    Ok(())
}
