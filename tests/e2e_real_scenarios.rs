// Real E2E Scenario Tests - Test File
// Created November 1, 2025

//! Test file for running real E2E scenarios using actual BearDog components

mod e2e;

use e2e::real_scenarios::*;
use e2e::E2ETestConfig;

// TEST_CATEGORY: e2e
// TEST_DOMAIN: core
// TEST_PRIORITY: critical
#[tokio::test]
async fn test_real_scenario_1_core_lifecycle() {
    let config = E2ETestConfig::default();
    let result = run_real_core_lifecycle_test(&config).await;
    assert!(
        result.is_ok(),
        "Core lifecycle test failed: {:?}",
        result.err()
    );

    let metrics = result.unwrap();
    assert!(metrics.successful_requests > 0);
    assert_eq!(metrics.failed_requests, 0);
    assert!(metrics.data_verified);
}

// TEST_CATEGORY: e2e
// TEST_DOMAIN: config
// TEST_PRIORITY: critical
#[tokio::test]
async fn test_real_scenario_2_config_validation() {
    let config = E2ETestConfig::default();
    let result = run_real_config_validation_test(&config).await;
    assert!(
        result.is_ok(),
        "Config validation test failed: {:?}",
        result.err()
    );

    let metrics = result.unwrap();
    assert!(metrics.successful_requests > 0);
    assert!(metrics.data_verified);
}

// TEST_CATEGORY: e2e
// TEST_DOMAIN: monitoring
// TEST_PRIORITY: critical
#[tokio::test]
async fn test_real_scenario_3_health_monitoring() {
    let config = E2ETestConfig::default();
    let result = run_real_health_monitoring_test(&config).await;
    assert!(
        result.is_ok(),
        "Health monitoring test failed: {:?}",
        result.err()
    );

    let metrics = result.unwrap();
    assert!(metrics.successful_requests > 0);
    assert!(metrics.data_verified);
}

// TEST_CATEGORY: e2e
// TEST_DOMAIN: concurrency
// TEST_PRIORITY: critical
#[tokio::test]
async fn test_real_scenario_4_concurrency() {
    let config = E2ETestConfig::default();
    let result = run_real_concurrency_test(&config).await;
    assert!(
        result.is_ok(),
        "Concurrency test failed: {:?}",
        result.err()
    );

    let metrics = result.unwrap();
    assert!(metrics.successful_requests > 0);
    assert!(metrics.data_verified);
}

// TEST_CATEGORY: e2e
// TEST_DOMAIN: components
// TEST_PRIORITY: high
#[tokio::test]
async fn test_real_scenario_5_component_management() {
    let config = E2ETestConfig::default();
    let result = run_real_component_management_test(&config).await;
    assert!(
        result.is_ok(),
        "Component management test failed: {:?}",
        result.err()
    );

    let metrics = result.unwrap();
    assert!(metrics.successful_requests > 0);
    assert!(metrics.data_verified);
}

// TEST_CATEGORY: e2e
// TEST_DOMAIN: integration
// TEST_PRIORITY: high
#[tokio::test]
async fn test_all_real_scenarios_sequential() {
    let config = E2ETestConfig::default();

    println!("\n═══════════════════════════════════════════════════════════");
    println!("🧪 Running All 5 Real E2E Scenarios");
    println!("═══════════════════════════════════════════════════════════\n");

    let mut total_success = 0;
    let mut total_failed = 0;
    let mut total_requests = 0;

    // Scenario 1: Core Lifecycle
    match run_real_core_lifecycle_test(&config).await {
        Ok(metrics) => {
            total_success += metrics.successful_requests;
            total_failed += metrics.failed_requests;
            total_requests += metrics.total_requests;
            println!(
                "✅ 1. Core Lifecycle - Success: {}, Failed: {}",
                metrics.successful_requests, metrics.failed_requests
            );
        }
        Err(e) => panic!("❌ 1. Core Lifecycle failed: {}", e),
    }

    // Scenario 2: Config Validation
    match run_real_config_validation_test(&config).await {
        Ok(metrics) => {
            total_success += metrics.successful_requests;
            total_failed += metrics.failed_requests;
            total_requests += metrics.total_requests;
            println!(
                "✅ 2. Config Validation - Success: {}, Failed: {}",
                metrics.successful_requests, metrics.failed_requests
            );
        }
        Err(e) => panic!("❌ 2. Config Validation failed: {}", e),
    }

    // Scenario 3: Health Monitoring
    match run_real_health_monitoring_test(&config).await {
        Ok(metrics) => {
            total_success += metrics.successful_requests;
            total_failed += metrics.failed_requests;
            total_requests += metrics.total_requests;
            println!(
                "✅ 3. Health Monitoring - Success: {}, Failed: {}",
                metrics.successful_requests, metrics.failed_requests
            );
        }
        Err(e) => panic!("❌ 3. Health Monitoring failed: {}", e),
    }

    // Scenario 4: Concurrency
    match run_real_concurrency_test(&config).await {
        Ok(metrics) => {
            total_success += metrics.successful_requests;
            total_failed += metrics.failed_requests;
            total_requests += metrics.total_requests;
            println!(
                "✅ 4. Concurrency - Success: {}, Failed: {}",
                metrics.successful_requests, metrics.failed_requests
            );
        }
        Err(e) => panic!("❌ 4. Concurrency failed: {}", e),
    }

    // Scenario 5: Component Management
    match run_real_component_management_test(&config).await {
        Ok(metrics) => {
            total_success += metrics.successful_requests;
            total_failed += metrics.failed_requests;
            total_requests += metrics.total_requests;
            println!(
                "✅ 5. Component Management - Success: {}, Failed: {}",
                metrics.successful_requests, metrics.failed_requests
            );
        }
        Err(e) => panic!("❌ 5. Component Management failed: {}", e),
    }

    println!("\n═══════════════════════════════════════════════════════════");
    println!("🎯 All Real E2E Scenarios Complete");
    println!("═══════════════════════════════════════════════════════════");
    println!("   Total Requests:  {}", total_requests);
    println!("   Total Success:   {}", total_success);
    println!("   Total Failed:    {}", total_failed);
    println!(
        "   Success Rate:    {:.1}%",
        (total_success as f64 / total_requests as f64) * 100.0
    );
    println!("═══════════════════════════════════════════════════════════\n");

    assert_eq!(total_failed, 0, "Some scenarios had failures");
    assert!(total_success > 0, "No successful operations");
}
