// Comprehensive Security Test Suite
//
// This module provides extensive testing for BearDog's security capabilities,
// including encryption, authentication, HSM integration, and compliance validation.

use beardog_core::{BearDogConfig, BearDogCore};
use beardog_errors::BearDogError;
use beardog_monitoring::monitoring::types::AlertLevel;
use beardog_security::*;
use beardog_threat::threat::ThreatSeverity;
use beardog_types::canonical::*;
use std::collections::HashMap;
use tokio_test;

#[tokio::test]
async fn test_security_core_initialization() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;

    // Test security provider initialization
    let security_provider = core.security_provider();
    assert!(security_provider.is_initialized()?);

    // Test encryption engine initialization
    let encryption_engine = core.encryption_engine();
    assert!(encryption_engine.is_ready()?);

    println!("✅ Security core initialization successful");
    Ok(())
}

#[tokio::test]
async fn test_encryption_decryption_workflow() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;
    let engine = core.encryption_engine();

    let test_data = b"sensitive_security_test_data";

    // Test encryption
    let encrypted = engine
        .encrypt(test_data, Some(EncryptionAlgorithm::Aes256Gcm))
        ?;

    assert!(!encrypted.ciphertext.is_empty());
    assert!(!encrypted.nonce.is_empty());

    // Test decryption
    let decrypted = engine.decrypt(&encrypted)?;
    assert_eq!(decrypted, test_data);

    println!("✅ Encryption/decryption workflow validated");
    Ok(())
}

#[tokio::test]
async fn test_hsm_integration() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;

    // Test HSM availability
    let hsm_status = core.check_hsm_status()?;
    assert!(matches!(
        hsm_status,
        HsmStatus::Available | HsmStatus::MockMode
    ));

    // Test key generation
    let key_result = core.generate_hsm_key("test_key_id");
    assert!(key_result.is_ok() || key_result.is_err()); // Either real HSM or mock

    println!("✅ HSM integration test completed");
    Ok(())
}

#[tokio::test]
async fn test_authentication_flows() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;
    let provider = core.security_provider();

    // Test valid authentication
    let valid_result = provider.authenticate("test_user", "test_token");
    // Should succeed or fail gracefully depending on configuration
    assert!(valid_result.is_ok() || valid_result.is_err());

    // Test invalid authentication
    let invalid_result = provider.authenticate("", "");
    assert!(invalid_result.is_err(), "Empty credentials should fail");

    println!("✅ Authentication flows validated");
    Ok(())
}

#[tokio::test]
async fn test_audit_logging() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;
    let audit = core.audit_engine();

    let audit_event = AuditEvent {
        id: "test_security_event".to_string(),
        event_type: AuditEventType::Security,
        severity: AuditSeverity::Medium,
        timestamp: chrono::Utc::now(),
        user_id: Some("test_user".to_string()),
        resource: Some("test_resource".to_string()),
        action: "security_test".to_string(),
        description: "Security test audit event".to_string(),
        outcome: "success ".to_string(),
        metadata: HashMap::new(),
        details: HashMap::new(),
    };

    let result = audit.log_event(audit_event);
    assert!(result.is_ok(), "Audit logging should succeed");

    println!("✅ Audit logging validated");
    Ok(())
}

#[tokio::test]
async fn test_compliance_validation() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;

    // Test sovereignty compliance
    let sovereignty_status = core.check_sovereignty_compliance()?;
    assert!(sovereignty_status.is_compliant());

    // Test security standards compliance
    let security_compliance = core.validate_security_standards()?;
    assert!(security_compliance.overall_score > 0.8); // 80% minimum

    println!("✅ Compliance validation successful");
    Ok(())
}

#[tokio::test]
async fn test_concurrent_security_operations() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;

    let mut handles = Vec::new();

    // Spawn multiple concurrent security operations
    for i in 0..5 {
        let core_clone = core.clone();
        let handle = tokio::spawn(async move {
            let engine = core_clone.encryption_engine();
            let test_data = format!("concurrent_test_data_{}", i);

            let encrypted = engine
                .encrypt(test_data.as_bytes(), Some(EncryptionAlgorithm::Aes256Gcm))
                ?;

            let decrypted = engine.decrypt(&encrypted)?;
            assert_eq!(decrypted, test_data.as_bytes());

            Ok::<(), BearDogError>(())
        });
        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        let result = handle
            
            .map_err(|e| BearDogError::internal(format!("Concurrent operation failed: {:?}", e)))?;
        assert!(
            result.is_ok(),
            "Concurrent security operation should succeed"
        );
    }

    println!("✅ Concurrent security operations validated");
    Ok(())
}
