//! Security module tests
//!
//! Comprehensive unit tests for the security provider functionality.

use super::*;
use crate::BearDogError;
use chrono::Utc;
use std::collections::HashMap;

#[tokio::test]
async fn test_security_provider_creation() {
    let config = SecurityProviderConfig::default();
    let provider = BearDogSecurityProvider::new(config).await;
    assert!(provider.is_ok());
}

#[tokio::test]
async fn test_rate_limiting_functionality() {
    let config = SecurityProviderConfig {
        rate_limiting_enabled: true,
        rate_limit: RateLimitConfig {
            max_requests_per_minute: 3,
            per_user_limiting: true,
            window_seconds: 60,
        },
        ..Default::default()
    };

    let mut provider = BearDogSecurityProvider::new(config).await.unwrap();

    // First 3 requests should pass
    for _ in 0..3 {
        let result = provider.check_rate_limit("test_user").await.unwrap();
        assert!(result);
    }

    // 4th request should fail
    let result = provider.check_rate_limit("test_user").await.unwrap();
    assert!(!result);
}

#[tokio::test]
async fn test_account_lockout_functionality() {
    let config = SecurityProviderConfig {
        max_failed_attempts: 3,
        lockout_duration_minutes: 5,
        ..Default::default()
    };

    let mut provider = BearDogSecurityProvider::new(config).await.unwrap();

    // Account should not be locked initially
    assert!(!provider.is_account_locked("test_user"));

    // Record failed attempts
    for _ in 0..3 {
        provider.record_failed_attempt("test_user");
    }

    // Account should now be locked
    assert!(provider.is_account_locked("test_user"));
}

#[tokio::test]
async fn test_session_management() {
    let config = SecurityProviderConfig {
        session: SessionConfig {
            timeout_minutes: 30,
            max_concurrent_sessions: 2,
            encryption_enabled: true,
        },
        ..Default::default()
    };

    let mut provider = BearDogSecurityProvider::new(config).await.unwrap();

    // Create a session
    let session = provider
        .create_session("test_user", Some("127.0.0.1".to_string()))
        .await
        .unwrap();
    assert_eq!(session.user_id, "test_user");
    assert!(session.is_active);

    // Validate the session
    let is_valid = provider
        .validate_and_refresh_session(&session.id)
        .await
        .unwrap();
    assert!(is_valid);
}

#[tokio::test]
async fn test_mfa_token_generation_and_verification() {
    let config = SecurityProviderConfig {
        mfa: MfaConfig {
            enabled: true,
            required_methods: vec![MfaMethod::TOTP],
            token_validity_minutes: 5,
        },
        ..Default::default()
    };

    let mut provider = BearDogSecurityProvider::new(config).await.unwrap();

    // Generate MFA token
    let token = provider
        .generate_mfa_token("test_user", MfaMethod::TOTP)
        .await
        .unwrap();
    assert_eq!(token.len(), 6); // 6-digit token

    // Verify the token
    let is_valid = provider
        .verify_mfa_token("test_user", &token)
        .await
        .unwrap();
    assert!(is_valid);

    // Token should not be valid after use
    let is_valid_again = provider
        .verify_mfa_token("test_user", &token)
        .await
        .unwrap();
    assert!(!is_valid_again);
}

#[tokio::test]
async fn test_threat_analysis() {
    let config = SecurityProviderConfig::default();
    let provider = BearDogSecurityProvider::new(config).await.unwrap();

    let subject = Subject {
        id: "test_user".to_string(),
        subject_type: SubjectType::User,
        attributes: HashMap::new(),
        roles: vec!["user".to_string()],
        clearance_level: Some(3),
    };

    let resource = Resource {
        id: "test_resource".to_string(),
        resource_type: "document".to_string(),
        classification: ResourceClassification::Confidential,
        attributes: HashMap::new(),
        owner: Some("test_user".to_string()),
    };

    let action = Action {
        action_type: ActionType::Read,
        context: HashMap::new(),
        timestamp: Utc::now(),
        source_ip: Some("127.0.0.1".to_string()),
    };

    let risk_level = provider
        .analyze_threat(&subject, &resource, &action)
        .await
        .unwrap();
    assert!(matches!(risk_level, RiskLevel::Low | RiskLevel::Medium));
}

#[tokio::test]
async fn test_authentication_implementation() {
    let config = SecurityProviderConfig::default();
    let provider = BearDogSecurityProvider::new(config).await.unwrap();

    let auth_result = provider
        .authenticate("test_user", "password")
        .await
        .unwrap();
    assert!(auth_result.success);
    assert_eq!(auth_result.user_id, Some("test_user".to_string()));
    assert!(auth_result.mfa_required); // Default config has MFA enabled
}

#[tokio::test]
async fn test_authorization_implementation() {
    let config = SecurityProviderConfig::default();
    let provider = BearDogSecurityProvider::new(config).await.unwrap();

    let subject = Subject {
        id: "admin_user".to_string(),
        subject_type: SubjectType::User,
        attributes: HashMap::new(),
        roles: vec!["admin".to_string()],
        clearance_level: Some(5),
    };

    let resource = Resource {
        id: "secure_resource".to_string(),
        resource_type: "system".to_string(),
        classification: ResourceClassification::Secret,
        attributes: HashMap::new(),
        owner: None,
    };

    let action = Action {
        action_type: ActionType::Admin,
        context: HashMap::new(),
        timestamp: Utc::now(),
        source_ip: Some("127.0.0.1".to_string()),
    };

    let auth_result = provider
        .authorize(&subject, &resource, &action)
        .await
        .unwrap();
    assert!(auth_result.permitted);
}

#[tokio::test]
async fn test_health_check() {
    let config = SecurityProviderConfig::default();
    let provider = BearDogSecurityProvider::new(config).await.unwrap();

    let health = provider.health().await.unwrap();
    assert_eq!(health.overall_status, HealthStatus::Healthy);
    assert!(!health.components.is_empty());
}

#[tokio::test]
async fn test_metrics_collection() {
    let config = SecurityProviderConfig::default();
    let provider = BearDogSecurityProvider::new(config).await.unwrap();

    let metrics = provider.metrics().await.unwrap();
    assert_eq!(metrics.total_authentications, 0); // New provider
    assert_eq!(metrics.active_sessions, 0);
}

#[tokio::test]
async fn test_cleanup_expired_data() {
    let config = SecurityProviderConfig {
        session: SessionConfig {
            timeout_minutes: 1, // Very short timeout for testing
            max_concurrent_sessions: 10,
            encryption_enabled: true,
        },
        mfa: MfaConfig {
            enabled: true,
            required_methods: vec![MfaMethod::TOTP],
            token_validity_minutes: 1,
        },
        ..Default::default()
    };

    let mut provider = BearDogSecurityProvider::new(config).await.unwrap();

    // Create a session and MFA token
    let _session = provider.create_session("test_user", None).await.unwrap();
    let _token = provider
        .generate_mfa_token("test_user", MfaMethod::TOTP)
        .await
        .unwrap();

    // Wait for expiration (in real test, we'd mock time)
    // For now, just test that cleanup runs without error
    let result = provider.cleanup_expired_data().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_audit_event_creation() {
    let config = SecurityProviderConfig {
        audit_logging_enabled: true,
        ..Default::default()
    };

    let mut provider = BearDogSecurityProvider::new(config).await.unwrap();

    let mut details = HashMap::new();
    details.insert("test_key".to_string(), "test_value".to_string());

    let result = provider
        .create_audit_event(
            "test_user",
            "test_resource",
            ActionType::Read,
            true,
            RiskLevel::Low,
            details,
        )
        .await;

    assert!(result.is_ok());
    assert_eq!(provider.audit_events.len(), 1);
    assert_eq!(provider.metrics.audit_events_generated, 1);
}

#[tokio::test]
async fn test_concurrent_session_limit() {
    let config = SecurityProviderConfig {
        session: SessionConfig {
            timeout_minutes: 30,
            max_concurrent_sessions: 1, // Only allow 1 session
            encryption_enabled: true,
        },
        ..Default::default()
    };

    let mut provider = BearDogSecurityProvider::new(config).await.unwrap();

    // First session should succeed
    let session1 = provider.create_session("test_user", None).await;
    assert!(session1.is_ok());

    // Second session should fail
    let session2 = provider.create_session("test_user", None).await;
    assert!(session2.is_err());

    if let Err(BearDogError::SecurityViolation(msg)) = session2 {
        assert!(msg.contains("Maximum concurrent sessions exceeded"));
    } else {
        panic!("Expected SecurityViolation error");
    }
}

#[tokio::test]
async fn test_session_validation() {
    let config = SecurityProviderConfig::default();
    let provider = BearDogSecurityProvider::new(config).await.unwrap();

    // Test validation of non-existent session
    let is_valid = provider
        .validate_session("non_existent_session")
        .await
        .unwrap();
    assert!(!is_valid);
}
