// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::auth::handlers::{AuthConfig, AuthenticationHandler};

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_auth_handler_creation() {
    let handler = AuthenticationHandler::new(AuthConfig::default());
    assert_eq!(handler.get_login_attempts("any"), 0);
    assert!(!handler.is_user_locked("any"));
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_session_validation_by_token() {
    let mut handler = AuthenticationHandler::new(AuthConfig::default());
    handler
        .register_user(
            "test-user-001",
            "correct-horse-battery-staple",
            vec!["read".to_string(), "write".to_string()],
        )
        .expect("register_user");

    let session = handler
        .authenticate("test-user-001:correct-horse-battery-staple")
        .await
        .expect("authenticate");

    let validated = handler
        .validate_session(&session.token)
        .expect("validate_session");
    assert_eq!(validated.user_id, "test-user-001");
    assert_eq!(
        validated.permissions,
        vec!["read".to_string(), "write".to_string()]
    );
}
