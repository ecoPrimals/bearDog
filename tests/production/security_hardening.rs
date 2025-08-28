use beardog_errors::BearDogError;


use beardog::production::*;

pub async fn test_security_hardening(prod_manager: &mut ProductionManager) {
    println!("🔒 Testing security hardening validation...");

    let security_validation = prod_manager
        .validate_security_hardening()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Security validation should succeed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Security validation should succeed", e).to_string())
})?;

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

    let compliance_check = prod_manager
        .run_security_compliance_check()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Compliance check should succeed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Compliance check should succeed", e).to_string())
})?;

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
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Core initialization failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Core initialization failed", e).to_string())
})?,
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Production manager creation failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Production manager creation failed", e).to_string())
})?;
        
    test_security_hardening(&mut production_manager).await;
} 