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


//! Production Deployment Validation Tests
//!
//! Tests for production deployment readiness, environment validation,
//! dependency checks, and pre-deployment safety procedures.

use beardog::production::*;

/// Test production deployment validation
pub async fn test_deployment_validation(prod_manager: &mut ProductionManager) {
    println!("🚀 Testing production deployment validation...");

    // Test deployment readiness check
    let readiness_check = prod_manager
        .check_deployment_readiness()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Deployment readiness check should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Deployment readiness check should succeed", e))
})?;

    assert!(
        readiness_check.configuration_valid,
        "Production configuration should be valid"
    );
    assert!(
        readiness_check.security_requirements_met,
        "Security requirements should be met"
    );
    assert!(
        readiness_check.performance_requirements_met,
        "Performance requirements should be met"
    );
    assert!(
        readiness_check.monitoring_configured,
        "Monitoring should be configured"
    );
    assert!(
        readiness_check.backup_systems_ready,
        "Backup systems should be ready"
    );

    // Test environment validation
    let environment_validation = prod_manager
        .validate_production_environment()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Environment validation should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Environment validation should succeed", e))
})?;

    assert!(
        environment_validation.os_compatibility,
        "OS should be compatible"
    );
    assert!(
        environment_validation.hardware_requirements_met,
        "Hardware requirements should be met"
    );
    assert!(
        environment_validation.network_configuration_valid,
        "Network should be configured"
    );
    assert!(
        environment_validation.storage_requirements_met,
        "Storage should be adequate"
    );
    assert!(
        environment_validation.security_policies_applied,
        "Security policies should be applied"
    );

    // Test dependency validation
    let dependency_check = prod_manager
        .validate_dependencies()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Dependency validation should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Dependency validation should succeed", e))
})?;

    assert!(
        dependency_check.system_libraries_present,
        "System libraries should be present"
    );
    assert!(
        dependency_check.crypto_libraries_verified,
        "Crypto libraries should be verified"
    );
    assert!(
        dependency_check.network_libraries_available,
        "Network libraries should be available"
    );
    assert!(
        dependency_check.version_compatibility_verified,
        "Version compatibility should be verified"
    );

    // Test configuration validation
    let config_validation = prod_manager
        .validate_production_configuration()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Configuration validation should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Configuration validation should succeed", e))
})?;

    assert!(
        config_validation.security_settings_optimal,
        "Security settings should be optimal"
    );
    assert!(
        config_validation.performance_settings_tuned,
        "Performance should be tuned"
    );
    assert!(
        config_validation.logging_configured_properly,
        "Logging should be configured"
    );
    assert!(
        config_validation.monitoring_endpoints_active,
        "Monitoring endpoints should be active"
    );

    // Test pre-deployment safety checks
    let safety_checks = prod_manager
        .run_pre_deployment_safety_checks()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Safety checks should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Safety checks should succeed", e))
})?;

    assert!(
        safety_checks.data_integrity_verified,
        "Data integrity should be verified"
    );
    assert!(
        safety_checks.backup_procedures_tested,
        "Backup procedures should be tested"
    );
    assert!(
        safety_checks.rollback_plan_ready,
        "Rollback plan should be ready"
    );
    assert!(
        safety_checks.emergency_procedures_documented,
        "Emergency procedures should be documented"
    );
}

/// Individual deployment validation test functions can be added here
/// for more granular testing if needed

#[tokio::test]
async fn test_deployment_readiness_standalone() {
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
        
    test_deployment_validation(&mut production_manager).await;
} 