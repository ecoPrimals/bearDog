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
        .expect("Deployment readiness check should succeed");

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
        .expect("Environment validation should succeed");

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
        .expect("Dependency validation should succeed");

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
        .expect("Configuration validation should succeed");

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
        .expect("Safety checks should succeed");

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
            .expect("Core initialization failed"),
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .expect("Production manager creation failed");
        
    test_deployment_validation(&mut production_manager).await;
} 