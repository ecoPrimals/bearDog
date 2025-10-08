// Chaos Framework Integration Tests
// Created October 7, 2025 - Phase 1 Completion

//! Integration tests for the chaos testing framework
//!
//! Tests the integration of all chaos framework components:
//! - Framework initialization
//! - Fault injector registration
//! - Recovery validator setup
//! - Scenario execution
//! - Reporting generation

use super::*;
use tracing::info;

/// Test framework initialization
pub async fn test_framework_initialization() -> Result<(), beardog_errors::BearDogError> {
    info!("🧪 Testing Framework Initialization");
    
    let config = ChaosTestConfig::default();
    let framework = ChaosTestFramework::new(config)?;
    
    assert!(framework.fault_injectors.len() > 0);
    assert!(framework.recovery_validators.len() > 0);
    assert!(framework.scenarios.len() > 0);
    
    info!("  ✅ Framework initialized successfully");
    info!("     - {} fault injectors", framework.fault_injectors.len());
    info!("     - {} recovery validators", framework.recovery_validators.len());
    info!("     - {} scenarios", framework.scenarios.len());
    
    Ok(())
}

/// Test fault injector registration
pub async fn test_fault_injector_registration() -> Result<(), beardog_errors::BearDogError> {
    info!("🧪 Testing Fault Injector Registration");
    
    let config = ChaosTestConfig::default();
    let framework = ChaosTestFramework::new(config)?;
    
    // Verify all required injectors are registered
    let required_injectors = vec!["network", "security", "database", "resource"];
    
    for injector_name in required_injectors {
        assert!(
            framework.fault_injectors.contains_key(injector_name),
            "Missing injector: {}",
            injector_name
        );
        info!("  ✅ Injector '{}' registered", injector_name);
    }
    
    Ok(())
}

/// Test recovery validator setup
pub async fn test_recovery_validator_setup() -> Result<(), beardog_errors::BearDogError> {
    info!("🧪 Testing Recovery Validator Setup");
    
    let config = ChaosTestConfig::default();
    let framework = ChaosTestFramework::new(config)?;
    
    assert!(framework.recovery_validators.len() >= 2);
    
    // Verify validator components
    let validator_components: Vec<String> = framework
        .recovery_validators
        .iter()
        .map(|v| v.target_component())
        .collect();
    
    info!("  ✅ Recovery validators: {:?}", validator_components);
    
    Ok(())
}

/// Test scenario execution pipeline
pub async fn test_scenario_execution() -> Result<(), beardog_errors::BearDogError> {
    info!("🧪 Testing Scenario Execution Pipeline");
    
    let config = ChaosTestConfig {
        scenario_timeout_ms: 5000,  // Short timeout for testing
        ..Default::default()
    };
    
    let mut framework = ChaosTestFramework::new(config)?;
    
    // Execute a simple scenario
    if let Some(scenario) = framework.scenarios.first().cloned() {
        info!("  Executing scenario: {}", scenario.name);
        
        let result = framework.run_chaos_scenario(&scenario).await?;
        
        assert_eq!(result.scenario_name, scenario.name);
        assert!(result.duration_ms > 0);
        
        info!("  ✅ Scenario executed successfully");
        info!("     - Duration: {}ms", result.duration_ms);
        info!("     - Success: {}", result.success);
    }
    
    Ok(())
}

/// Test metrics collection
pub async fn test_metrics_collection() -> Result<(), beardog_errors::BearDogError> {
    info!("🧪 Testing Metrics Collection");
    
    let metrics_collector = ChaosMetricsCollector::new(1000);
    
    // Start collection
    metrics_collector.start_collection()?;
    
    // Collect baseline
    let baseline = ChaosMetricsCollector::collect_baseline_metrics().await?;
    assert_eq!(baseline.response_time_increase, 1.0);
    
    // Collect current
    let current = ChaosMetricsCollector::collect_current_metrics().await?;
    assert!(current.response_time_increase >= 1.0);
    
    // Measure impact
    let impact = ChaosMetricsCollector::measure_fault_impact().await?;
    assert!(impact.response_time_increase >= 1.0);
    
    metrics_collector.stop_collection()?;
    
    info!("  ✅ Metrics collection working");
    
    Ok(())
}

/// Test report generation
pub async fn test_report_generation() -> Result<(), beardog_errors::BearDogError> {
    info!("🧪 Testing Report Generation");
    
    let config = ChaosTestConfig {
        scenario_timeout_ms: 5000,
        ..Default::default()
    };
    
    let mut framework = ChaosTestFramework::new(config)?;
    
    // Run chaos testing
    let report = framework.run_chaos_testing().await?;
    
    assert!(report.overall_resilience_score >= 0.0);
    assert!(report.scenario_results.len() > 0);
    assert!(report.recommendations.len() > 0);
    
    info!("  ✅ Report generated successfully");
    info!("     - Resilience Score: {:.2}%", report.overall_resilience_score);
    info!("     - Scenarios Executed: {}", report.scenario_results.len());
    info!("     - Recommendations: {}", report.recommendations.len());
    
    Ok(())
}

/// Test controller lifecycle
pub async fn test_controller_lifecycle() -> Result<(), beardog_errors::BearDogError> {
    info!("🧪 Testing Controller Lifecycle");
    
    let config = ChaosTestConfig::default();
    let controller = ChaosController::new(config);
    
    // Test start
    controller.start();
    assert!(controller.is_running());
    info!("  ✅ Controller started");
    
    // Test stop
    controller.stop();
    assert!(!controller.is_running());
    info!("  ✅ Controller stopped");
    
    Ok(())
}

/// Test fault tracking
pub async fn test_fault_tracking() -> Result<(), beardog_errors::BearDogError> {
    info!("🧪 Testing Fault Tracking");
    
    let config = ChaosTestConfig::default();
    let controller = ChaosController::new(config);
    controller.start();
    
    // Create and track a fault
    let fault = ActiveFault {
        id: "test-fault-1".to_string(),
        fault_type: FaultType::NetworkPartition { duration_ms: 1000 },
        start_time: std::time::Instant::now(),
        duration: std::time::Duration::from_millis(1000),
        target_component: "network".to_string(),
        severity: FaultSeverity::High,
    };
    
    controller.track_active_fault(fault.clone()).await;
    
    let active_faults = controller.get_active_faults().await;
    assert_eq!(active_faults.len(), 1);
    assert_eq!(active_faults[0].id, "test-fault-1");
    
    info!("  ✅ Fault tracking working");
    
    // Clean up
    controller.remove_active_fault(&fault.id).await;
    let active_faults_after = controller.get_active_faults().await;
    assert_eq!(active_faults_after.len(), 0);
    
    controller.stop();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_framework_init() {
        let result = test_framework_initialization().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_injector_registration() {
        let result = test_fault_injector_registration().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validator_setup() {
        let result = test_recovery_validator_setup().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_scenario_exec() {
        let result = test_scenario_execution().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_metrics() {
        let result = test_metrics_collection().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_reporting() {
        let result = test_report_generation().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_controller() {
        let result = test_controller_lifecycle().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_tracking() {
        let result = test_fault_tracking().await;
        assert!(result.is_ok());
    }
}

