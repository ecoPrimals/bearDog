// E2E Test Suite - Top Level
// Created October 7, 2025

//! BearDog End-to-End Test Suite
//!
//! This test suite provides comprehensive end-to-end testing for BearDog,
//! validating complete production scenarios.
//!
//! # Test Scenarios
//!
//! - **Production Deployment**: Complete deployment workflow validation
//! - **Full-Stack Integration**: Multi-layer integration testing
//! - **Security Flow**: Authentication, authorization, encryption end-to-end
//! - **Disaster Recovery**: Failure and recovery scenario validation
//!
//! # Running Tests
//!
//! ```bash
//! # Run all E2E tests
//! cargo test --test e2e_test_suite
//!
//! # Run specific E2E test
//! cargo test --test e2e_test_suite test_production_deployment_e2e
//!
//! # Run with verbose output
//! cargo test --test e2e_test_suite -- --nocapture
//! ```

mod e2e;

use e2e::{print_e2e_report, E2EScenario, E2ETestFramework};

#[tokio::test]
async fn test_production_deployment_e2e() {
    let framework = E2ETestFramework::new()
        .await
        .expect("Failed to create E2E framework");

    let result = framework
        .run_scenario(E2EScenario::ProductionDeployment)
        .await;
    assert!(result.is_ok(), "Production deployment test failed");

    let test_result = result.unwrap();
    assert!(
        test_result.success,
        "Production deployment scenario failed: {:?}",
        test_result.error_message
    );
    assert!(
        test_result.metrics.successful_requests > 0,
        "No successful requests"
    );
    assert_eq!(
        test_result.metrics.failed_requests, 0,
        "Had failed requests"
    );
}

#[tokio::test]
async fn test_full_stack_integration_e2e() {
    let framework = E2ETestFramework::new()
        .await
        .expect("Failed to create E2E framework");

    let result = framework
        .run_scenario(E2EScenario::FullStackIntegration)
        .await;
    assert!(result.is_ok(), "Full-stack integration test failed");

    let test_result = result.unwrap();
    assert!(
        test_result.success,
        "Full-stack integration scenario failed: {:?}",
        test_result.error_message
    );
    assert!(
        test_result.metrics.data_verified,
        "Data integrity not verified"
    );
}

#[tokio::test]
async fn test_security_flow_e2e() {
    let framework = E2ETestFramework::new()
        .await
        .expect("Failed to create E2E framework");

    let result = framework.run_scenario(E2EScenario::SecurityFlow).await;
    assert!(result.is_ok(), "Security flow test failed");

    let test_result = result.unwrap();
    assert!(
        test_result.success,
        "Security flow scenario failed: {:?}",
        test_result.error_message
    );
    assert!(
        test_result.metrics.successful_requests > 0,
        "No successful security operations"
    );
}

#[tokio::test]
async fn test_disaster_recovery_e2e() {
    let framework = E2ETestFramework::new()
        .await
        .expect("Failed to create E2E framework");

    let result = framework.run_scenario(E2EScenario::DisasterRecovery).await;
    assert!(result.is_ok(), "Disaster recovery test failed");

    let test_result = result.unwrap();
    assert!(
        test_result.success,
        "Disaster recovery scenario failed: {:?}",
        test_result.error_message
    );
    assert!(
        test_result.metrics.data_verified,
        "Data integrity not verified after recovery"
    );
}

#[tokio::test]
async fn test_all_e2e_scenarios() {
    let framework = E2ETestFramework::new()
        .await
        .expect("Failed to create E2E framework");

    let results = framework
        .run_all_scenarios()
        .await
        .expect("Failed to run all scenarios");

    // Print comprehensive report
    print_e2e_report(&results);

    // Verify all scenarios passed
    let all_passed = results.iter().all(|r| r.success);
    assert!(all_passed, "Some E2E scenarios failed");

    // Verify we ran expected number of scenarios
    assert!(
        results.len() >= 4,
        "Expected at least 4 E2E scenarios to run"
    );
}

#[tokio::test]
async fn test_e2e_framework_initialization() {
    let result = E2ETestFramework::new().await;
    assert!(result.is_ok(), "E2E framework initialization failed");
}

#[test]
fn test_e2e_scenario_types() {
    // Verify scenario enum variants
    let scenarios = [
        E2EScenario::ProductionDeployment,
        E2EScenario::FullStackIntegration,
        E2EScenario::SecurityFlow,
        E2EScenario::DisasterRecovery,
    ];

    assert_eq!(scenarios.len(), 4);
}
