use beardog_errors::BearDogError;


use beardog::production::*;

pub async fn test_deployment_validation(prod_manager: &mut ProductionManager) {
    println!("🚀 Testing production deployment validation...");

    let readiness_check = prod_manager
        .check_deployment_readiness()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Deployment readiness check should succeed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Deployment readiness check should succeed", e).to_string())
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

    let environment_validation = prod_manager
        .validate_production_environment()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Environment validation should succeed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Environment validation should succeed", e).to_string())
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

    let dependency_check = prod_manager
        .validate_dependencies()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Dependency validation should succeed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Dependency validation should succeed", e).to_string())
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

    let config_validation = prod_manager
        .validate_production_configuration()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Configuration validation should succeed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Configuration validation should succeed", e).to_string())
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

    let safety_checks = prod_manager
        .run_pre_deployment_safety_checks()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Safety checks should succeed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Safety checks should succeed", e).to_string())
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
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Core initialization failed", e).to_string())
})?,
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Production manager creation failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Production manager creation failed", e).to_string())
})?;
        
    test_deployment_validation(&mut production_manager).await;
} 