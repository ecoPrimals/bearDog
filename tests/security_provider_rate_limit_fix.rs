

use beardog::security::{
    Action, ActionType, BearDogSecurityProvider, Resource, ResourceClassification,
    SecurityProvider, SecurityProviderConfig, Subject, SubjectType,
};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_rate_limiting_fixed() -> Result<(), Box<dyn std::error::Error>> {

    let mut config = SecurityProviderConfig::default();
    config.rate_limit.max_requests_per_minute = 2; // Very low for testing
    config.rate_limiting_enabled = true;

    let security_provider = BearDogSecurityProvider::new(config).await?;

    let subject = Subject {
        id: "test-user-rate-limit".to_string(),
        subject_type: SubjectType::User,
        roles: vec!["user".to_string()],
        attributes: HashMap::with_capacity(16),
        clearance_level: Some(3),
    };

    let resource = Resource {
        id: "test-resource".to_string(),
        resource_type: "file".to_string(),
        owner: Some("test-user-rate-limit".to_string()),
        classification: ResourceClassification::Internal,
        attributes: HashMap::with_capacity(16),
    };

    let action = Action {
        action_type: ActionType::Read,
        context: HashMap::with_capacity(16),
        timestamp: chrono::Utc::now(),
        source_ip: Some("127.0.0.1".to_string()),
    };

    let result1 = security_provider
        .authorize(&subject, &resource, &action)
        .await?;
    assert!(result1.permitted, "First request should be allowed");

    let result2 = security_provider
        .authorize(&subject, &resource, &action)
        .await?;
    assert!(result2.permitted, "Second request should be allowed");

    let result3 = security_provider
        .authorize(&subject, &resource, &action)
        .await?;
    assert!(!result3.permitted, "Third request should be rate limited");
    assert!(
        result3.reason.contains("rate limit") || result3.reason.contains("Rate limit"),
        "Rate limit reason should be mentioned in result: {}",
        result3.reason
    );

    sleep(Duration::from_secs(2)).await;

    let result4 = security_provider
        .authorize(&subject, &resource, &action)
        .await?;

    println!("✅ Rate limiting test completed successfully");
    println!("   - First two requests: allowed");
    println!("   - Third request: rate limited with proper error message");
    println!("   - Rate limit reason: {}", result3.reason);

    Ok(())
}

#[tokio::test]
async fn test_rate_limiting_different_users() -> Result<(), Box<dyn std::error::Error>> {

    let mut config = SecurityProviderConfig::default();
    config.rate_limit.max_requests_per_minute = 1;
    config.rate_limiting_enabled = true;

    let security_provider = BearDogSecurityProvider::new(config).await?;

    let user1 = Subject {
        id: "user1".to_string(),
        subject_type: SubjectType::User,
        roles: vec!["user".to_string()],
        attributes: HashMap::with_capacity(16),
        clearance_level: Some(3),
    };

    let user2 = Subject {
        id: "user2".to_string(),
        subject_type: SubjectType::User,
        roles: vec!["user".to_string()],
        attributes: HashMap::with_capacity(16),
        clearance_level: Some(3),
    };

    let resource = Resource {
        id: "shared-resource".to_string(),
        resource_type: "file".to_string(),
        owner: None,
        classification: ResourceClassification::Internal,
        attributes: HashMap::with_capacity(16),
    };

    let action = Action {
        action_type: ActionType::Read,
        context: HashMap::with_capacity(16),
        timestamp: chrono::Utc::now(),
        source_ip: Some("127.0.0.1".to_string()),
    };

    let result1 = security_provider
        .authorize(&user1, &resource, &action)
        .await?;
    assert!(result1.permitted, "User1 first request should be allowed");

    let result2 = security_provider
        .authorize(&user2, &resource, &action)
        .await?;
    assert!(result2.permitted, "User2 first request should be allowed");

    let result3 = security_provider
        .authorize(&user1, &resource, &action)
        .await?;
    assert!(
        !result3.permitted,
        "User1 second request should be rate limited"
    );

    let result4 = security_provider
        .authorize(&user2, &resource, &action)
        .await?;
    assert!(
        !result4.permitted,
        "User2 second request should be rate limited"
    );

    println!("✅ Per-user rate limiting working correctly");

    Ok(())
}

#[tokio::test]
async fn test_rate_limiting_disabled() -> Result<(), Box<dyn std::error::Error>> {

    let mut config = SecurityProviderConfig::default();
    config.rate_limiting_enabled = false; // Disable rate limiting

    let security_provider = BearDogSecurityProvider::new(config).await?;

    let subject = Subject {
        id: "unlimited-user".to_string(),
        subject_type: SubjectType::User,
        roles: vec!["user".to_string()],
        attributes: HashMap::with_capacity(16),
        clearance_level: Some(3),
    };

    let resource = Resource {
        id: "test-resource".to_string(),
        resource_type: "file".to_string(),
        owner: Some("unlimited-user".to_string()),
        classification: ResourceClassification::Internal,
        attributes: HashMap::with_capacity(16),
    };

    let action = Action {
        action_type: ActionType::Read,
        context: HashMap::with_capacity(16),
        timestamp: chrono::Utc::now(),
        source_ip: Some("127.0.0.1".to_string()),
    };

    for i in 0..5 {
        let result = security_provider
            .authorize(&subject, &resource, &action)
            .await?;
        assert!(
            result.permitted,
            "Request {} should be allowed when rate limiting disabled",
            i + 1
        );
    }

    println!("✅ Disabled rate limiting allows all requests");

    Ok(())
}
