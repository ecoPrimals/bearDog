//! Production Security Hardening Tests
//!
//! Tests for security hardening validation, security policies,
//! encryption validation, and security compliance checks.

use beardog::production::*;

/// Test production security hardening
pub async fn test_security_hardening(prod_manager: &mut ProductionManager) {
    println!("🔒 Testing security hardening validation...");

    // Test security configuration validation
    let security_validation = prod_manager
        .validate_security_hardening()
        .await
        .expect("Security validation should succeed");

    assert!(
        security_validation.encryption_standards_met,
        "Encryption standards should be met"
    );
    assert!(
        security_validation.access_controls_configured,
        "Access controls should be configured"
    );
    assert!(
        security_validation.audit_logging_enabled,
        "Audit logging should be enabled"
    );

    // Test security compliance
    let compliance_check = prod_manager
        .run_security_compliance_check()
        .await
        .expect("Compliance check should succeed");

    assert!(
        compliance_check.gdpr_compliant,
        "Should be GDPR compliant"
    );
    assert!(
        compliance_check.security_policies_enforced,
        "Security policies should be enforced"
    );

    println!("✅ Security hardening tests completed");
}

#[tokio::test]
async fn test_security_hardening_standalone() {
    use beardog::core::*;
    use std::sync::Arc;
    
    let config = BearDogConfig::production();
    let core = Arc::new(
        BearDogCore::new(config)
            .await
            .expect("Core initialization failed"),
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .expect("Production manager creation failed");
        
    test_security_hardening(&mut production_manager).await;
} 