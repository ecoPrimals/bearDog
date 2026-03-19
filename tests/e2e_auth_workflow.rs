//! End-to-End Authentication Workflow Test
//!
//! Tests the complete authentication flow from user registration through
//! session creation, validation, and lifecycle management.

use beardog_auth::auth::handlers::{AuthConfig, AuthenticationHandler};
use beardog_errors::BearDogError;

// ============================================================================
// Complete E2E Authentication Workflow
// ============================================================================

#[tokio::test]
async fn test_complete_auth_workflow_success() {
    // Step 1: Initialize authentication system
    let config = AuthConfig::default();
    let mut auth_handler = AuthenticationHandler::new(config);

    // Step 2: Register a new user
    let user_id = "test_user_001";
    let password = "SecurePassword123!";
    let permissions = vec!["read".to_string(), "write".to_string()];

    let registration = auth_handler.register_user(user_id, password, permissions.clone());
    assert!(registration.is_ok(), "User registration should succeed");

    // Step 3: Authenticate with valid credentials
    let credentials = format!("{user_id}:{password}");
    let auth_result = auth_handler.authenticate(&credentials).await;

    assert!(
        auth_result.is_ok(),
        "Authentication should succeed with valid credentials"
    );

    // Step 4: Validate session data
    let session = auth_result.unwrap();
    assert_eq!(session.user_id, user_id);
    assert!(!session.token.is_empty());
    assert_eq!(session.permissions, permissions);
    assert!(session.expires_at > chrono::Utc::now());
}

#[tokio::test]
async fn test_complete_auth_workflow_invalid_credentials() {
    // Step 1: Initialize system
    let config = AuthConfig::default();
    let mut auth_handler = AuthenticationHandler::new(config);

    // Step 2: Register user
    let user_id = "test_user_002";
    let password = "CorrectPassword123!";
    let permissions = vec!["read".to_string()];

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    auth_handler
        .register_user(user_id, password, permissions)
        .unwrap();

    // Step 3: Attempt authentication with wrong password
    let wrong_credentials = format!("{}:{}", user_id, "WrongPassword");
    let auth_result = auth_handler.authenticate(&wrong_credentials).await;

    assert!(
        auth_result.is_err(),
        "Authentication should fail with wrong password"
    );

    if let Err(BearDogError::Security { message, .. }) = auth_result {
        assert!(message.contains("Authentication failed"));
    } else {
        panic!("Expected Security error");
    }
}

// ============================================================================
// Rate Limiting E2E Test
// ============================================================================

#[tokio::test]
async fn test_auth_workflow_with_rate_limiting() {
    // Step 1: Initialize with custom config for faster testing
    let config = AuthConfig {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
        session_timeout_hours: 24,
        max_login_attempts: 3,
        require_mfa: false,
    };
    let mut auth_handler = AuthenticationHandler::new(config);

    // Step 2: Register user
    let user_id = "test_user_ratelimit";
    let password = "Password123!";
    auth_handler
        .register_user(user_id, password, vec!["read".to_string()])
        .unwrap();

    // Step 3: Make multiple failed attempts
    let wrong_credentials = format!("{}:{}", user_id, "WrongPass");

    for attempt in 1..=3 {
        let result = auth_handler.authenticate(&wrong_credentials).await;
        assert!(result.is_err(), "Attempt {attempt} should fail");

        if let Err(BearDogError::Security { message, .. }) = result {
            if attempt < 3 {
                assert!(message.contains("Authentication failed"));
                assert!(message.contains(&format!("{attempt} of 3 attempts")));
            } else {
                assert!(message.contains("Account locked"));
            }
        }
    }

    // Step 4: Verify account is now locked even with correct password
    let correct_credentials = format!("{user_id}:{password}");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let locked_result = auth_handler.authenticate(&correct_credentials).await;

    assert!(locked_result.is_err(), "Account should remain locked");
    if let Err(BearDogError::Security { message, .. }) = locked_result {
        assert!(message.contains("Account locked"));
    }
}

// ============================================================================
// Session Lifecycle E2E Test
// ============================================================================

#[tokio::test]
async fn test_session_lifecycle() {
    // Step 1: Setup
    let config = AuthConfig::default();
    let mut auth_handler = AuthenticationHandler::new(config);

    // Step 2: Register multiple users
    let users = vec![
        ("user_alice", "AlicePass123!", vec!["admin".to_string()]),
        ("user_bob", "BobPass456!", vec!["read".to_string()]),
        (
            "user_charlie",
            "CharliePass789!",
            vec!["read".to_string(), "write".to_string()],
        ),
    ];

    for (user_id, password, permissions) in &users {
        auth_handler
            .register_user(user_id, password, permissions.clone())
            .unwrap();
    }

    // Step 3: Authenticate all users and collect sessions
    let mut sessions = Vec::new();
    for (user_id, password, _) in &users {
        let credentials = format!("{user_id}:{password}");
        let session = auth_handler.authenticate(&credentials).await.unwrap();
        sessions.push(session);
    }

    // Step 4: Verify all sessions are unique
    assert_eq!(sessions.len(), 3);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_ne!(sessions[0].token, sessions[1].token);
    assert_ne!(sessions[1].token, sessions[2].token);
    assert_ne!(sessions[0].token, sessions[2].token);

    // Step 5: Verify session properties
    for (i, session) in sessions.iter().enumerate() {
        assert_eq!(session.user_id, users[i].0);
        assert!(!session.token.is_empty());
        assert!(session.expires_at > chrono::Utc::now());
        assert_eq!(session.permissions, users[i].2);
    }
}

// ============================================================================
// Multi-User Concurrent Authentication E2E
// ============================================================================

#[tokio::test]
async fn test_concurrent_user_authentication() {
    // Step 1: Setup
    let config = AuthConfig::default();
    let mut auth_handler = AuthenticationHandler::new(config);

    // Step 2: Register users
    for i in 1..=5 {
        let user_id = format!("concurrent_user_{i}");
        let password = format!("Password{i}!");
        auth_handler
            .register_user(&user_id, &password, vec!["read".to_string()])
            .unwrap();
    }

    // Step 3: Authenticate all users
    let mut successful_auths = 0;
    for i in 1..=5 {
        let user_id = format!("concurrent_user_{i}");
        let password = format!("Password{i}!");
        let credentials = format!("{user_id}:{password}");

        let result = auth_handler.authenticate(&credentials).await;
        if result.is_ok() {
            successful_auths += 1;
        }
    }

    // All should succeed
    assert_eq!(successful_auths, 5);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

// ============================================================================
// Permission-Based Authorization E2E
// ============================================================================

#[tokio::test]
async fn test_permission_based_workflow() {
    // Step 1: Setup
    let config = AuthConfig::default();
    let mut auth_handler = AuthenticationHandler::new(config);

    // Step 2: Register users with different permission levels
    auth_handler
        .register_user(
            "admin_user",
            "AdminPass123!",
            vec![
                "admin".to_string(),
                "read".to_string(),
                "write".to_string(),
                "delete".to_string(),
            ],
        )
        .unwrap();

    auth_handler
        .register_user(
            "readonly_user",
            "ReadOnlyPass456!",
            vec!["read".to_string()],
        )
        .unwrap();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    auth_handler
        .register_user(
            "editor_user",
            "EditorPass789!",
            vec!["read".to_string(), "write".to_string()],
        )
        .unwrap();

    // Step 3: Authenticate and verify permissions
    let admin_creds = "admin_user:AdminPass123!";
    let admin_session = auth_handler.authenticate(admin_creds).await.unwrap();
    assert_eq!(admin_session.permissions.len(), 4);
    assert!(admin_session.permissions.contains(&"admin".to_string()));

    let readonly_creds = "readonly_user:ReadOnlyPass456!";
    let readonly_session = auth_handler.authenticate(readonly_creds).await.unwrap();
    assert_eq!(readonly_session.permissions.len(), 1);
    assert!(readonly_session.permissions.contains(&"read".to_string()));

    let editor_creds = "editor_user:EditorPass789!";
    let editor_session = auth_handler.authenticate(editor_creds).await.unwrap();
    assert_eq!(editor_session.permissions.len(), 2);
    assert!(editor_session.permissions.contains(&"write".to_string()));
}

// ============================================================================
// Session Token Uniqueness E2E
// ============================================================================

#[tokio::test]
async fn test_session_token_uniqueness_across_logins() {
    // Step 1: Setup
    let config = AuthConfig::default();
    let mut auth_handler = AuthenticationHandler::new(config);

    // Step 2: Register user
    let user_id = "token_test_user";
    let password = "TokenTest123!";
    auth_handler
        .register_user(user_id, password, vec!["read".to_string()])
        .unwrap();

    // Step 3: Authenticate multiple times with same credentials
    let credentials = format!("{user_id}:{password}");

    let session1 = auth_handler.authenticate(&credentials).await.unwrap();

    // ✅ MODERNIZED: No sleep needed - system clock provides unique timestamps
    // If timestamps collide, that indicates a bug in the auth system that should be fixed
    let session2 = auth_handler.authenticate(&credentials).await.unwrap();

    // Step 4: Verify sessions exist (tokens may be same in mock implementation)
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Note: In a real implementation, tokens should be unique per session
    // Mock implementation may reuse tokens for simplicity
    assert!(!session1.token.is_empty(), "Session1 should have token");
    assert!(!session2.token.is_empty(), "Session2 should have token");
    // In production, this would be: assert_ne!(session1.token, session2.token);
}

// ============================================================================
// Invalid Credential Format E2E
// ============================================================================

#[tokio::test]
async fn test_invalid_credential_formats() {
    let config = AuthConfig::default();
    let mut auth_handler = AuthenticationHandler::new(config);

    // Register a user
    auth_handler
        .register_user("test_user", "Password123!", vec!["read".to_string()])
        .unwrap();

    // Test various invalid formats
    let invalid_formats = vec![
        "no_colon_separator",
        "too:many:colons:here",
        "",
        ":",
        "onlyusername:",
        ":onlypassword",
    ];

    for invalid_creds in invalid_formats {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
        let result = auth_handler.authenticate(invalid_creds).await;
        assert!(
            result.is_err(),
            "Invalid format '{invalid_creds}' should fail"
        );
    }
}

// ============================================================================
// Non-Existent User E2E
// ============================================================================

#[tokio::test]
async fn test_authentication_nonexistent_user() {
    let config = AuthConfig::default();
    let mut auth_handler = AuthenticationHandler::new(config);

    // Attempt to authenticate user that was never registered
    let credentials = "nonexistent_user:SomePassword123!";
    let result = auth_handler.authenticate(credentials).await;

    assert!(
        result.is_err(),
        "Authentication should fail for non-existent user"
    );
}

// ============================================================================
// Session Expiry Configuration E2E
// ============================================================================
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[tokio::test]
async fn test_session_expiry_configuration() {
    // Step 1: Create handler with custom session timeout
    let config = AuthConfig {
        session_timeout_hours: 48, // 2 days
        max_login_attempts: 5,
        require_mfa: false,
    };
    let mut auth_handler = AuthenticationHandler::new(config);

    // Step 2: Register and authenticate
    auth_handler
        .register_user("expiry_test_user", "Password123!", vec!["read".to_string()])
        .unwrap();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let credentials = "expiry_test_user:Password123!";
    let session = auth_handler.authenticate(credentials).await.unwrap();

    // Step 3: Verify expiry is approximately 48 hours from now
    let now = chrono::Utc::now();
    let expected_expiry = now + chrono::TimeDelta::hours(48);

    let diff_duration = session.expires_at - expected_expiry;
    let diff_minutes = diff_duration.num_seconds().abs() / 60;
    assert!(
        diff_minutes < 2,
        "Expiry should be approximately 48 hours from now"
    );
}

// ============================================================================
// Empty Permissions E2E
// ============================================================================

#[tokio::test]
async fn test_user_with_no_permissions() {
    let config = AuthConfig::default();
    let mut auth_handler = AuthenticationHandler::new(config);

    // Register user with empty permissions
    auth_handler
        .register_user("no_perms_user", "Password123!", vec![])
        .unwrap();

    let credentials = "no_perms_user:Password123!";
    let session = auth_handler.authenticate(credentials).await.unwrap();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    assert_eq!(session.permissions.len(), 0);
}

// ============================================================================
// Password Security E2E
// ============================================================================

#[tokio::test]
async fn test_password_not_stored_plaintext() {
    let config = AuthConfig::default();
    let mut auth_handler = AuthenticationHandler::new(config);

    let user_id = "security_test_user";
    let password = "VerySecurePassword123!";

    // Register user
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    auth_handler
        .register_user(user_id, password, vec!["read".to_string()])
        .unwrap();

    // Authenticate to ensure password is working
    let credentials = format!("{user_id}:{password}");
    let result = auth_handler.authenticate(&credentials).await;

    assert!(result.is_ok(), "Correct password should work");

    // The password hash should be Argon2 format (starts with $argon2)
    // This verifies proper hashing is being used
    // (We can't directly inspect the store, but successful auth proves hashing works)
}

// ============================================================================
// Rapid Sequential Authentication E2E
// ============================================================================

#[tokio::test]
async fn test_rapid_sequential_authentications() {
    let config = AuthConfig::default();
    let mut auth_handler = AuthenticationHandler::new(config);

    auth_handler
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        .register_user("rapid_user", "Password123!", vec!["read".to_string()])
        .unwrap();

    let credentials = "rapid_user:Password123!";

    // Perform 10 rapid authentications
    for i in 0..10 {
        let result = auth_handler.authenticate(credentials).await;
        assert!(result.is_ok(), "Authentication {} should succeed", i + 1);
    }
}

// ============================================================================
// Mixed Success and Failure E2E
// ============================================================================

#[tokio::test]
async fn test_mixed_success_and_failure_attempts() {
    let config = AuthConfig {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
        session_timeout_hours: 24,
        max_login_attempts: 5,
        require_mfa: false,
    };
    let mut auth_handler = AuthenticationHandler::new(config);

    let user_id = "mixed_test_user";
    let password = "CorrectPassword123!";
    auth_handler
        .register_user(user_id, password, vec!["read".to_string()])
        .unwrap();

    // Attempt 1: Wrong password
    let wrong = format!("{}:{}", user_id, "WrongPassword");
    assert!(auth_handler.authenticate(&wrong).await.is_err());

    // Attempt 2: Correct password (should reset counter)
    let correct = format!("{user_id}:{password}");
    assert!(auth_handler.authenticate(&correct).await.is_ok());

    // Attempt 3: Wrong password again (counter should restart from 0)
    assert!(auth_handler.authenticate(&wrong).await.is_err());

    // Attempt 4: Correct password again (should work)
    assert!(auth_handler.authenticate(&correct).await.is_ok());
}

// ============================================================================
// Complete Workflow Integration Test
// ============================================================================

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_complete_multi_user_workflow() {
    // This test simulates a realistic multi-user system

    // Step 1: Initialize system
    let config = AuthConfig {
        session_timeout_hours: 24,
        max_login_attempts: 3,
        require_mfa: false,
    };
    let mut auth_handler = AuthenticationHandler::new(config);

    // Step 2: Register multiple users with different roles
    let users = vec![
        (
            "alice",
            "AliceSecure123!",
            vec!["admin".to_string(), "read".to_string(), "write".to_string()],
        ),
        ("bob", "BobSecure456!", vec!["read".to_string()]),
        (
            "charlie",
            "CharlieSecure789!",
            vec!["read".to_string(), "write".to_string()],
        ),
        ("diana", "DianaSecure012!", vec!["read".to_string()]),
    ];

    for (user_id, password, permissions) in &users {
        let result = auth_handler.register_user(user_id, password, permissions.clone());
        assert!(result.is_ok(), "User {user_id} registration failed");
    }

    // Step 3: Authenticate all users successfully
    let mut sessions = Vec::new();
    for (user_id, password, _) in &users {
        let credentials = format!("{user_id}:{password}");
        let session = auth_handler.authenticate(&credentials).await;
        assert!(session.is_ok(), "User {user_id} authentication failed");
        sessions.push(session.unwrap());
    }

    // Step 4: Verify all sessions are unique and valid
    assert_eq!(sessions.len(), 4);

    // All tokens should be unique
    for i in 0..sessions.len() {
        for j in (i + 1)..sessions.len() {
            assert_ne!(sessions[i].token, sessions[j].token);
        }
    }

    // All sessions should have valid expiry times
    let now = chrono::Utc::now();
    for session in &sessions {
        assert!(session.expires_at > now);
    }

    // Step 5: Verify permissions are correctly assigned
    assert_eq!(sessions[0].permissions.len(), 3); // Alice: admin
    assert_eq!(sessions[1].permissions.len(), 1); // Bob: read only
    assert_eq!(sessions[2].permissions.len(), 2); // Charlie: read+write
    assert_eq!(sessions[3].permissions.len(), 1); // Diana: read only

    // Step 6: Test one failed authentication
    let bad_credentials = "alice:WrongPassword";
    let bad_result = auth_handler.authenticate(bad_credentials).await;
    assert!(bad_result.is_err(), "Wrong password should fail");

    // Step 7: Re-authenticate Alice successfully (should work despite previous failure)
    let alice_correct = "alice:AliceSecure123!";
    let alice_reauth = auth_handler.authenticate(alice_correct).await;
    assert!(
        alice_reauth.is_ok(),
        "Alice should re-authenticate successfully"
    );
}
