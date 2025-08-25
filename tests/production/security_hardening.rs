// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


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
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Security validation should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Security validation should succeed", e))
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

    // Test security compliance
    let compliance_check = prod_manager
        .run_security_compliance_check()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Compliance check should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Compliance check should succeed", e))
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
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Core initialization failed", e))
})?,
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Production manager creation failed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Production manager creation failed", e))
})?;
        
    test_security_hardening(&mut production_manager).await;
} 