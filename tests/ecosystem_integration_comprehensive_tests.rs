//! Comprehensive Ecosystem Integration Tests
//!
//! Tests for BearDog's ecosystem integration following the standardization guide

use beardog::{
    ecosystem_integration::{
        BearDogEcosystemFactory, BearDogEcosystemProvider, EcosystemIntegration, EcosystemRequest,
        EcosystemResponse, PrimalType, ResponseStatus, SecurityContext, SecurityLevel,
    },
    BearDogConfig, BearDogCore,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio;
use uuid::Uuid;

#[tokio::test]
async fn test_ecosystem_provider_creation() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.unwrap());

    let provider = BearDogEcosystemFactory::create_provider(core);

    // Test basic provider properties
    assert!(!provider.instance_id.is_empty());
    assert!(provider
        .capabilities
        .core
        .contains(&"authentication".to_string()));
    assert!(provider
        .capabilities
        .core
        .contains(&"encryption".to_string()));
    assert!(provider
        .capabilities
        .extended
        .contains(&"ml_threat_detection".to_string()));
}

#[tokio::test]
async fn test_songbird_registration() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.unwrap());
    let provider = BearDogEcosystemFactory::create_provider(core);

    // Test Songbird registration
    let registration_result = provider.register_with_songbird().await;
    assert!(registration_result.is_ok());

    let registration_id = registration_result.unwrap();
    assert!(!registration_id.is_empty());

    // Should be a valid UUID format
    assert!(Uuid::parse_str(&registration_id).is_ok());
}

#[tokio::test]
async fn test_authentication_request_handling() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.unwrap());
    let provider = BearDogEcosystemFactory::create_provider(core);

    let request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service: "toadstool-compute-01".to_string(),
        target_service: "beardog-security-01".to_string(),
        operation: "authenticate".to_string(),
        payload: serde_json::json!({
            "username": "test_user",
            "password": "test_password"
        }),
        security_context: SecurityContext {
            auth_token: Some("test_token".to_string()),
            identity: "toadstool-service".to_string(),
            permissions: vec!["security.auth".to_string()],
            security_level: SecurityLevel::Internal,
        },
        metadata: HashMap::new(),
        timestamp: chrono::Utc::now(),
    };

    let response = provider
        .handle_ecosystem_request(request.clone())
        .await
        .unwrap();

    // Verify response structure
    assert_eq!(response.request_id, request.request_id);
    assert!(matches!(response.status, ResponseStatus::Success));
    assert!(response.payload.is_object());

    // Verify authentication response payload
    let payload = response.payload.as_object().unwrap();
    assert!(payload.contains_key("authenticated"));
    assert!(payload.contains_key("user_id"));
    assert!(payload.contains_key("permissions"));
}

#[tokio::test]
async fn test_encryption_request_handling() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.unwrap());
    let provider = BearDogEcosystemFactory::create_provider(core);

    let request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service: "nestgate-storage-01".to_string(),
        target_service: "beardog-security-01".to_string(),
        operation: "encrypt".to_string(),
        payload: serde_json::json!({
            "data": "sensitive_data_to_encrypt",
            "algorithm": "aes-256-gcm"
        }),
        security_context: SecurityContext {
            auth_token: Some("test_token".to_string()),
            identity: "nestgate-service".to_string(),
            permissions: vec!["security.encrypt".to_string()],
            security_level: SecurityLevel::Confidential,
        },
        metadata: HashMap::new(),
        timestamp: chrono::Utc::now(),
    };

    let response = provider
        .handle_ecosystem_request(request.clone())
        .await
        .unwrap();

    // Verify response structure
    assert_eq!(response.request_id, request.request_id);
    assert!(matches!(response.status, ResponseStatus::Success));

    // Verify encryption response payload
    let payload = response.payload.as_object().unwrap();
    assert!(payload.contains_key("encrypted"));
    assert!(payload.contains_key("algorithm"));
    assert!(payload.contains_key("data"));
}

#[tokio::test]
async fn test_compliance_request_handling() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.unwrap());
    let provider = BearDogEcosystemFactory::create_provider(core);

    let request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service: "biomeos-orchestrator-01".to_string(),
        target_service: "beardog-security-01".to_string(),
        operation: "compliance_check".to_string(),
        payload: serde_json::json!({
            "framework": "gdpr",
            "data_type": "personal_data"
        }),
        security_context: SecurityContext {
            auth_token: Some("test_token".to_string()),
            identity: "biomeos-service".to_string(),
            permissions: vec!["security.compliance".to_string()],
            security_level: SecurityLevel::Restricted,
        },
        metadata: HashMap::new(),
        timestamp: chrono::Utc::now(),
    };

    let response = provider
        .handle_ecosystem_request(request.clone())
        .await
        .unwrap();

    // Verify response structure
    assert_eq!(response.request_id, request.request_id);
    assert!(matches!(response.status, ResponseStatus::Success));

    // Verify compliance response payload
    let payload = response.payload.as_object().unwrap();
    assert!(payload.contains_key("compliant"));
    assert!(payload.contains_key("frameworks"));
    assert!(payload.contains_key("checks_passed"));
    assert!(payload.contains_key("checks_failed"));
}

#[tokio::test]
async fn test_unsupported_operation_handling() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.unwrap());
    let provider = BearDogEcosystemFactory::create_provider(core);

    let request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service: "unknown-service".to_string(),
        target_service: "beardog-security-01".to_string(),
        operation: "unsupported_operation".to_string(),
        payload: serde_json::json!({}),
        security_context: SecurityContext {
            auth_token: None,
            identity: "unknown-service".to_string(),
            permissions: vec![],
            security_level: SecurityLevel::Public,
        },
        metadata: HashMap::new(),
        timestamp: chrono::Utc::now(),
    };

    let response = provider
        .handle_ecosystem_request(request.clone())
        .await
        .unwrap();

    // Verify error response
    assert_eq!(response.request_id, request.request_id);
    assert!(matches!(response.status, ResponseStatus::Error { .. }));

    if let ResponseStatus::Error { code, message } = response.status {
        assert_eq!(code, "UNSUPPORTED_OPERATION");
        assert!(message.contains("unsupported_operation"));
    }
}

#[tokio::test]
async fn test_health_status_reporting() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.unwrap());
    let provider = BearDogEcosystemFactory::create_provider(core);

    // Test health status retrieval
    let health_status = provider.get_health_status().await.unwrap();
    assert!(!health_status.version.is_empty());
    assert!(health_status.uptime_seconds > 0);
    assert!(!health_status.capabilities_online.is_empty());

    // Test health reporting
    let report_result = provider.report_health(health_status).await;
    assert!(report_result.is_ok());
}

#[tokio::test]
async fn test_capability_updates() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.unwrap());
    let provider = BearDogEcosystemFactory::create_provider(core);

    let capabilities = beardog::ecosystem_integration::ServiceCapabilities {
        core: vec!["authentication".to_string(), "encryption".to_string()],
        extended: vec!["ml_threat_detection".to_string()],
        integrations: vec!["songbird".to_string(), "toadstool".to_string()],
    };

    let result = provider.update_capabilities(capabilities).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_ecosystem_deregistration() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.unwrap());
    let provider = BearDogEcosystemFactory::create_provider(core);

    // Register first
    let registration_result = provider.register_with_songbird().await;
    assert!(registration_result.is_ok());

    // Then deregister
    let deregister_result = provider.deregister().await;
    assert!(deregister_result.is_ok());
}

#[tokio::test]
async fn test_primal_type_conversion() {
    assert_eq!(PrimalType::BearDog.as_str(), "beardog");
    assert_eq!(PrimalType::ToadStool.as_str(), "toadstool");
    assert_eq!(PrimalType::Songbird.as_str(), "songbird");
    assert_eq!(PrimalType::NestGate.as_str(), "nestgate");
    assert_eq!(PrimalType::Squirrel.as_str(), "squirrel");
    assert_eq!(PrimalType::BiomeOS.as_str(), "biomeos");
}

#[tokio::test]
async fn test_security_context_validation() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.unwrap());
    let provider = BearDogEcosystemFactory::create_provider(core);

    // Test request with different security levels
    let security_levels = vec![
        SecurityLevel::Public,
        SecurityLevel::Internal,
        SecurityLevel::Restricted,
        SecurityLevel::Confidential,
    ];

    for security_level in security_levels {
        let request = EcosystemRequest {
            request_id: Uuid::new_v4(),
            source_service: "test-service".to_string(),
            target_service: "beardog-security-01".to_string(),
            operation: "authenticate".to_string(),
            payload: serde_json::json!({}),
            security_context: SecurityContext {
                auth_token: Some("test_token".to_string()),
                identity: "test-service".to_string(),
                permissions: vec!["security.auth".to_string()],
                security_level: security_level.clone(),
            },
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        };

        let response = provider.handle_ecosystem_request(request).await.unwrap();
        assert!(matches!(response.status, ResponseStatus::Success));
    }
}
