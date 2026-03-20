// SPDX-License-Identifier: AGPL-3.0-only
//! Session management and security tests for BearDog authentication
//!
//! These tests verify session lifecycle, rate limiting, and security features.

use beardog_auth::auth::handlers::{AuthConfig, AuthenticationHandler};

#[tokio::test]
async fn test_auth_handler_creation() {
    let config = AuthConfig::default();
    let _handler = AuthenticationHandler::new(config);
    // Handler created successfully
}

#[tokio::test]
async fn test_auth_config_defaults() {
    let config = AuthConfig::default();
    assert_eq!(config.session_timeout_hours, 24);
    assert_eq!(config.max_login_attempts, 5);
    assert!(!config.require_mfa);
}

#[tokio::test]
async fn test_custom_auth_config() {
    let config = AuthConfig {
        session_timeout_hours: 12,
        max_login_attempts: 3,
        require_mfa: true,
    };

    assert_eq!(config.session_timeout_hours, 12);
    assert_eq!(config.max_login_attempts, 3);
    assert!(config.require_mfa);
}

#[tokio::test]
async fn test_successful_authentication() {
    let config = AuthConfig::default();
    let mut handler = AuthenticationHandler::new(config);

    // Register test user with secure credentials
    handler
        .register_user("testuser", "secure_password", vec!["read".to_string()])
        .expect("Registration should succeed");

    let credentials = "testuser:secure_password";
    let result = handler.authenticate(credentials).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_session_data_fields() {
    let config = AuthConfig::default();
    let mut handler = AuthenticationHandler::new(config);

    // Register test user
    handler
        .register_user("testuser", "secure_password", vec!["read".to_string()])
        .expect("Registration should succeed");

    let credentials = "testuser:secure_password";
    let session = handler
        .authenticate(credentials)
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        .await
        .expect("Auth should succeed");

    assert_eq!(session.user_id, "testuser");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(!session.token.is_empty());
    assert!(session.expires_at > chrono::Utc::now());
}

#[tokio::test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
async fn test_failed_authentication() {
    let config = AuthConfig::default();
    let mut handler = AuthenticationHandler::new(config);

    // Register test user
    handler
        .register_user("testuser", "correct_password", vec!["read".to_string()])
        .expect("Registration should succeed");

    // Try with wrong password
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let credentials = "testuser:wrong_password";
    let result = handler.authenticate(credentials).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_rate_limiting_increments() {
    let config = AuthConfig {
        session_timeout_hours: 24,
        max_login_attempts: 3,
        require_mfa: false,
    };
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut handler = AuthenticationHandler::new(config);

    // Register test user
    handler
        .register_user("testuser", "correct_password", vec!["read".to_string()])
        .expect("Registration should succeed");

    let credentials = "testuser:wrong_password";

    // First failed attempt
    let _ = handler.authenticate(credentials).await;
    assert_eq!(handler.get_login_attempts("testuser"), 1);

    // Second failed attempt
    let _ = handler.authenticate(credentials).await;
    assert_eq!(handler.get_login_attempts("testuser"), 2);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: important
#[tokio::test]
async fn test_account_lockout() {
    let config = AuthConfig {
        session_timeout_hours: 24,
        max_login_attempts: 3,
        require_mfa: false,
    };
    let mut handler = AuthenticationHandler::new(config);

    // Register test user
    handler
        .register_user("testuser", "correct_password", vec!["read".to_string()])
        .expect("Registration should succeed");

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let credentials = "testuser:wrong_password";

    // Exhaust login attempts
    for _ in 0..3 {
        let _ = handler.authenticate(credentials).await;
    }

    // Next attempt should be locked out
    let result = handler.authenticate(credentials).await;
    assert!(result.is_err());

    // Verify lockout message
    if let Err(e) = result {
        let error_msg = format!("{:?}", e);
        assert!(error_msg.contains("Account locked") || error_msg.contains("locked"));
    }
}

#[tokio::test]
async fn test_is_user_locked() {
    let config = AuthConfig {
        session_timeout_hours: 24,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        max_login_attempts: 3,
        require_mfa: false,
    };
    let mut handler = AuthenticationHandler::new(config);

    // Register test user
    handler
        .register_user("testuser", "correct_password", vec!["read".to_string()])
        .expect("Registration should succeed");

    let credentials = "testuser:wrong_password";

    assert!(!handler.is_user_locked("testuser"));

    // Exhaust login attempts
    for _ in 0..3 {
        let _ = handler.authenticate(credentials).await;
    }

    assert!(handler.is_user_locked("testuser"));
}

#[tokio::test]
async fn test_reset_login_attempts() {
    let config = AuthConfig {
        session_timeout_hours: 24,
        max_login_attempts: 3,
        require_mfa: false,
    };
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut handler = AuthenticationHandler::new(config);

    // Register test user
    handler
        .register_user("testuser", "correct_password", vec!["read".to_string()])
        .expect("Registration should succeed");

    let credentials = "testuser:wrong_password";

    // Make some failed attempts
    for _ in 0..2 {
        let _ = handler.authenticate(credentials).await;
    }

    assert_eq!(handler.get_login_attempts("testuser"), 2);

    // Reset attempts
    handler
        .reset_login_attempts("testuser")
        .expect("Reset should succeed");
    assert_eq!(handler.get_login_attempts("testuser"), 0);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_successful_auth_resets_attempts() {
    let config = AuthConfig::default();
    let mut handler = AuthenticationHandler::new(config);

    // Register test user
    handler
        .register_user("testuser", "correct_password", vec!["read".to_string()])
        .expect("Registration should succeed");

    let invalid_credentials = "testuser:wrong_password";
    let valid_credentials = "testuser:correct_password";

    // Make some failed attempts
    let _ = handler.authenticate(invalid_credentials).await;
    let _ = handler.authenticate(invalid_credentials).await;
    assert_eq!(handler.get_login_attempts("testuser"), 2);

    // Successful auth should reset
    let result = handler.authenticate(valid_credentials).await;
    assert!(result.is_ok());
    assert_eq!(handler.get_login_attempts("testuser"), 0);
}

#[tokio::test]
async fn test_session_validation() {
    let config = AuthConfig::default();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut handler = AuthenticationHandler::new(config);

    // Register test user
    handler
        .register_user("testuser", "secure_password", vec!["read".to_string()])
        .expect("Registration should succeed");

    let credentials = "testuser:secure_password";
    let session = handler
        .authenticate(credentials)
        .await
        .expect("Auth should succeed");

    // Validate the session token
    let validated = handler.validate_session(&session.token);
    assert!(validated.is_ok());
}

#[tokio::test]
async fn test_invalid_session_validation() {
    let config = AuthConfig::default();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let handler = AuthenticationHandler::new(config);

    let result = handler.validate_session("invalid_token_12345");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_session_logout() {
    let config = AuthConfig::default();
    let mut handler = AuthenticationHandler::new(config);

    // Register test user
    handler
        .register_user("testuser", "secure_password", vec!["read".to_string()])
        .expect("Registration should succeed");

    let credentials = "testuser:secure_password";
    let session = handler
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
        .authenticate(credentials)
        .await
        .expect("Auth should succeed");

    // Logout should succeed
    let result = handler.logout(&session.token);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(result.is_ok());

    // Session should no longer be valid
    let validation = handler.validate_session(&session.token);
    assert!(validation.is_err());
}

#[tokio::test]
async fn test_multiple_sessions() {
    let config = AuthConfig::default();
    let mut handler = AuthenticationHandler::new(config);

    // Register test users
    handler
        .register_user("user1", "password1", vec!["read".to_string()])
        .expect("Registration should succeed");
    handler
        .register_user("user2", "password2", vec!["read".to_string()])
        .expect("Registration should succeed");

    let session1 = handler
        .authenticate("user1:password1")
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        .await
        .expect("Auth1 should succeed");
    let session2 = handler
        .authenticate("user2:password2")
        .await
        .expect("Auth2 should succeed");

    // Both sessions should be valid
    assert!(handler.validate_session(&session1.token).is_ok());
    assert!(handler.validate_session(&session2.token).is_ok());
}

#[tokio::test]
async fn test_session_expiration_cleanup() {
    let config = AuthConfig {
        session_timeout_hours: 24,
        max_login_attempts: 5,
        require_mfa: false,
    };
    let mut handler = AuthenticationHandler::new(config);

    // Register test user
    handler
        .register_user("testuser", "secure_password", vec!["read".to_string()])
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        .expect("Registration should succeed");

    // Create a session
    let credentials = "testuser:secure_password";
    let _session = handler
        .authenticate(credentials)
        .await
        .expect("Auth should succeed");

    // Cleanup shouldn't remove active sessions
    handler.cleanup_expired_sessions();

    // Session should still be valid (not expired yet)
    let _validated = handler.validate_session(&_session.token);
    // This test mainly verifies cleanup doesn't crash
}

#[tokio::test]
async fn test_login_attempts_for_different_users() {
    let config = AuthConfig::default();
    let mut handler = AuthenticationHandler::new(config);

    // Register test users
    handler
        .register_user("user1", "password1", vec!["read".to_string()])
        .expect("Registration should succeed");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    handler
        .register_user("user2", "password2", vec!["read".to_string()])
        .expect("Registration should succeed");

    let user1_creds = "user1:wrong_password";
    let user2_creds = "user2:wrong_password";

    // Make attempts for different users
    let _ = handler.authenticate(user1_creds).await;
    let _ = handler.authenticate(user2_creds).await;
    let _ = handler.authenticate(user1_creds).await;

    assert_eq!(handler.get_login_attempts("user1"), 2);
    assert_eq!(handler.get_login_attempts("user2"), 1);
}

#[tokio::test]
async fn test_session_permissions() {
    let config = AuthConfig::default();
    let mut handler = AuthenticationHandler::new(config);

    // Register test user with permissions
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    handler
        .register_user(
            "testuser",
            "secure_password",
            vec!["read".to_string(), "write".to_string()],
        )
        .expect("Registration should succeed");

    let credentials = "testuser:secure_password";
    let session = handler
        .authenticate(credentials)
        .await
        .expect("Auth should succeed");

    // Session should have permissions
    assert!(!session.permissions.is_empty());
    assert!(session.permissions.contains(&"read".to_string()));
    assert!(session.permissions.contains(&"write".to_string()));
}

#[tokio::test]
async fn test_auth_config_serialization() {
    let config = AuthConfig {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        session_timeout_hours: 12,
        max_login_attempts: 3,
        require_mfa: true,
    };

    let json = serde_json::to_string(&config).expect("Serialization should succeed");
    assert!(json.contains("12"));
    assert!(json.contains("3"));
    assert!(json.contains("true"));
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_auth_config_deserialization() {
    let json = r#"{"session_timeout_hours":12,"max_login_attempts":3,"require_mfa":true}"#;
    let config: AuthConfig = serde_json::from_str(json).expect("Deserialization should succeed");

    assert_eq!(config.session_timeout_hours, 12);
    assert_eq!(config.max_login_attempts, 3);
    assert!(config.require_mfa);
}
