// SPDX-License-Identifier: AGPL-3.0-or-later
// Comprehensive Tests for License Manager
//
// Tests the context-aware licensing system that manages capability access,
// validation, and restrictions.

use super::license_manager::*;
use crate::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;

// ========================================================================
// License Validation Tests
// ========================================================================

#[tokio::test]
async fn test_get_license_status_basic() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let status = core.get_license_status();

    assert!(status.is_valid, "License should be valid");
    assert!(
        status.expiry_warning.is_none(),
        "Should have no expiry warning"
    );
    assert!(
        !status.capabilities_granted.is_empty(),
        "Should have capabilities"
    );
    assert!(
        status.restrictions_active.is_empty(),
        "Should have no restrictions"
    );
}

#[tokio::test]
async fn test_get_license_status_capabilities() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let status = core.get_license_status();

    // Verify core capabilities are granted
    assert!(
        status
            .capabilities_granted
            .contains(&"security".to_string())
    );
    assert!(status.capabilities_granted.contains(&"hsm".to_string()));
    assert!(status.capabilities_granted.contains(&"crypto".to_string()));
    assert!(
        status
            .capabilities_granted
            .contains(&"monitoring".to_string())
    );
    assert!(
        status
            .capabilities_granted
            .contains(&"workflows".to_string())
    );
    assert!(status.capabilities_granted.contains(&"ai".to_string()));
}

#[tokio::test]
async fn test_get_license_status_count() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let status = core.get_license_status();

    assert_eq!(
        status.capabilities_granted.len(),
        6,
        "Should have 6 capabilities"
    );
}

// ========================================================================
// Capability License Check Tests
// ========================================================================

#[tokio::test]
async fn test_check_capability_license_security() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let result = core.check_capability_license("security");

    assert!(result.is_ok(), "Check should succeed");
    assert!(result.unwrap(), "Security capability should be licensed");
}

#[tokio::test]
async fn test_check_capability_license_hsm() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let result = core.check_capability_license("hsm");

    assert!(result.is_ok(), "Check should succeed");
    assert!(result.unwrap(), "HSM capability should be licensed");
}

#[tokio::test]
async fn test_check_capability_license_crypto() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let result = core.check_capability_license("crypto");

    assert!(result.is_ok(), "Check should succeed");
    assert!(result.unwrap(), "Crypto capability should be licensed");
}

#[tokio::test]
async fn test_check_capability_license_monitoring() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let result = core.check_capability_license("monitoring");

    assert!(result.is_ok(), "Check should succeed");
    assert!(result.unwrap(), "Monitoring capability should be licensed");
}

#[tokio::test]
async fn test_check_capability_license_workflows() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let result = core.check_capability_license("workflows");

    assert!(result.is_ok(), "Check should succeed");
    assert!(result.unwrap(), "Workflows capability should be licensed");
}

#[tokio::test]
async fn test_check_capability_license_ai() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let result = core.check_capability_license("ai");

    assert!(result.is_ok(), "Check should succeed");
    assert!(result.unwrap(), "AI capability should be licensed");
}

#[tokio::test]
async fn test_check_capability_license_unlicensed() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let result = core.check_capability_license("unknown_capability");

    assert!(result.is_ok(), "Check should succeed");
    assert!(
        !result.unwrap(),
        "Unknown capability should not be licensed"
    );
}

#[tokio::test]
async fn test_check_capability_license_case_sensitive() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    // Capabilities are case-sensitive
    let result_lowercase = core.check_capability_license("security");
    let result_uppercase = core.check_capability_license("SECURITY");

    assert!(result_lowercase.unwrap(), "Lowercase should match");
    assert!(
        !result_uppercase.unwrap(),
        "Uppercase should not match (case-sensitive)"
    );
}

// ========================================================================
// LicenseInfo Structure Tests
// ========================================================================

#[tokio::test]
async fn test_license_info_creation() {
    let license = LicenseInfo {
        license_id: "test-license-123".to_string(),
        license_type: "MIT".to_string(),
        issued_at: chrono::Utc::now(),
        expires_at: None,
        capabilities: vec!["security".to_string(), "crypto".to_string()],
        restrictions: std::collections::HashMap::new(),
    };

    assert_eq!(license.license_id, "test-license-123");
    assert_eq!(license.license_type, "MIT");
    assert!(license.expires_at.is_none());
    assert_eq!(license.capabilities.len(), 2);
    assert!(license.restrictions.is_empty());
}

#[tokio::test]
async fn test_license_info_with_expiry() {
    let now = chrono::Utc::now();
    let future = now + chrono::Duration::days(30);

    let license = LicenseInfo {
        license_id: "test-license-456".to_string(),
        license_type: "Commercial".to_string(),
        issued_at: now,
        expires_at: Some(future),
        capabilities: vec!["security".to_string()],
        restrictions: std::collections::HashMap::new(),
    };

    assert!(license.expires_at.is_some());
    assert!(license.expires_at.unwrap() > now);
}

#[tokio::test]
async fn test_license_info_with_restrictions() {
    let mut restrictions = std::collections::HashMap::new();
    restrictions.insert("rate_limit".to_string(), "1000/hour".to_string());
    restrictions.insert("max_users".to_string(), "100".to_string());

    let license = LicenseInfo {
        license_id: "test-license-789".to_string(),
        license_type: "Enterprise".to_string(),
        issued_at: chrono::Utc::now(),
        expires_at: None,
        capabilities: vec!["security".to_string(), "hsm".to_string()],
        restrictions,
    };

    assert_eq!(license.restrictions.len(), 2);
    assert_eq!(license.restrictions.get("rate_limit").unwrap(), "1000/hour");
}

#[tokio::test]
async fn test_license_info_serialization() {
    let license = LicenseInfo {
        license_id: "test-license-serial".to_string(),
        license_type: "Apache-2.0".to_string(),
        issued_at: chrono::Utc::now(),
        expires_at: None,
        capabilities: vec!["security".to_string()],
        restrictions: std::collections::HashMap::new(),
    };

    // Test serialization
    let serialized = serde_json::to_string(&license);
    assert!(serialized.is_ok(), "Should serialize successfully");

    // Test deserialization
    let deserialized: Result<LicenseInfo, _> = serde_json::from_str(&serialized.unwrap());
    assert!(deserialized.is_ok(), "Should deserialize successfully");
}

// ========================================================================
// LicenseValidation Structure Tests
// ========================================================================

#[tokio::test]
async fn test_license_validation_creation() {
    let validation = LicenseValidation {
        is_valid: true,
        expiry_warning: None,
        capabilities_granted: vec!["security".to_string()],
        restrictions_active: vec![],
    };

    assert!(validation.is_valid);
    assert!(validation.expiry_warning.is_none());
    assert_eq!(validation.capabilities_granted.len(), 1);
    assert!(validation.restrictions_active.is_empty());
}

#[tokio::test]
async fn test_license_validation_with_warning() {
    let validation = LicenseValidation {
        is_valid: true,
        expiry_warning: Some("License expires in 7 days".to_string()),
        capabilities_granted: vec!["security".to_string()],
        restrictions_active: vec![],
    };

    assert!(validation.expiry_warning.is_some());
    assert_eq!(
        validation.expiry_warning.unwrap(),
        "License expires in 7 days"
    );
}

#[tokio::test]
async fn test_license_validation_invalid() {
    let validation = LicenseValidation {
        is_valid: false,
        expiry_warning: Some("License expired".to_string()),
        capabilities_granted: vec![],
        restrictions_active: vec!["all_features_disabled".to_string()],
    };

    assert!(!validation.is_valid);
    assert!(validation.capabilities_granted.is_empty());
    assert!(!validation.restrictions_active.is_empty());
}

#[tokio::test]
async fn test_license_validation_serialization() {
    let validation = LicenseValidation {
        is_valid: true,
        expiry_warning: None,
        capabilities_granted: vec!["security".to_string(), "crypto".to_string()],
        restrictions_active: vec![],
    };

    // Test serialization
    let serialized = serde_json::to_string(&validation);
    assert!(serialized.is_ok(), "Should serialize successfully");

    // Test deserialization
    let deserialized: Result<LicenseValidation, _> = serde_json::from_str(&serialized.unwrap());
    assert!(deserialized.is_ok(), "Should deserialize successfully");
}

// ========================================================================
// Multiple BearDogCore Instance Tests
// ========================================================================

#[tokio::test]
async fn test_multiple_cores_same_license() {
    let config = UnifiedBearDogConfig::default();
    let core1 = BearDogCore::new(config.clone());
    let core2 = BearDogCore::new(config);

    let status1 = core1.get_license_status();
    let status2 = core2.get_license_status();

    assert_eq!(status1.is_valid, status2.is_valid);
    assert_eq!(status1.capabilities_granted, status2.capabilities_granted);
}

#[tokio::test]
async fn test_capability_check_consistency() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    // Multiple checks should be consistent
    let check1 = core.check_capability_license("security").unwrap();
    let check2 = core.check_capability_license("security").unwrap();
    let check3 = core.check_capability_license("security").unwrap();

    assert_eq!(check1, check2);
    assert_eq!(check2, check3);
}

// ========================================================================
// Edge Case Tests
// ========================================================================

#[tokio::test]
async fn test_empty_capability_string() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let result = core.check_capability_license("");

    assert!(result.is_ok(), "Should handle empty string");
    assert!(!result.unwrap(), "Empty capability should not be licensed");
}

#[tokio::test]
async fn test_whitespace_capability() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let result = core.check_capability_license("   ");

    assert!(result.is_ok(), "Should handle whitespace");
    assert!(
        !result.unwrap(),
        "Whitespace capability should not be licensed"
    );
}

#[tokio::test]
async fn test_special_characters_capability() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let result = core.check_capability_license("security@#$%");

    assert!(result.is_ok(), "Should handle special characters");
    assert!(
        !result.unwrap(),
        "Invalid capability should not be licensed"
    );
}

// ========================================================================
// Integration Tests
// ========================================================================

#[tokio::test]
async fn test_all_capabilities_checkable() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let status = core.get_license_status();

    // All granted capabilities should be checkable
    for capability in &status.capabilities_granted {
        let result = core.check_capability_license(capability);
        assert!(
            result.is_ok(),
            "Should be able to check capability: {}",
            capability
        );
        assert!(
            result.unwrap(),
            "Capability should be licensed: {}",
            capability
        );
    }
}

#[tokio::test]
async fn test_license_status_completeness() {
    let config = UnifiedBearDogConfig::default();
    let core = BearDogCore::new(config);

    let status = core.get_license_status();

    // Verify status is complete
    assert!(status.is_valid, "Valid license should be valid");
    assert!(
        status.expiry_warning.is_none(),
        "No expiry for permanent license"
    );
    assert!(
        !status.capabilities_granted.is_empty(),
        "Should have capabilities"
    );
    assert!(
        status.restrictions_active.is_empty(),
        "Should have no restrictions"
    );

    // Verify all expected capabilities are present
    let expected_capabilities = vec!["security", "hsm", "crypto", "monitoring", "workflows", "ai"];
    for capability in expected_capabilities {
        assert!(
            status
                .capabilities_granted
                .contains(&capability.to_string()),
            "Should contain capability: {}",
            capability
        );
    }
}
