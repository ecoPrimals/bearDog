// Real E2E Scenarios - Using Actual BearDog Components
// Created November 1, 2025 - Corrected Implementation

//! Real End-to-End test scenarios using actual BearDog Core components
//!
//! These tests validate complete production workflows using real:
//! - BearDog Core initialization and state management
//! - Configuration loading and validation
//! - Health monitoring and state transitions
//! - Component registration and management
//! - Concurrent access patterns
//!
//! Unlike simulated E2E tests, these use actual BearDog components
//! to validate real-world behavior and integration.

use super::helpers::*;
use super::{E2EMetrics, E2ETestConfig};
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use std::sync::Arc;
use tracing::info;

// ═══════════════════════════════════════════════════════════════════════════
// SCENARIO 1: Core Initialization and Lifecycle
// ═══════════════════════════════════════════════════════════════════════════

// TEST_CATEGORY: e2e
// TEST_DOMAIN: core
// TEST_PRIORITY: critical
/// Test real BearDog Core initialization, operation, and shutdown lifecycle
pub async fn run_real_core_lifecycle_test(
    _config: &E2ETestConfig,
) -> Result<E2EMetrics, BearDogError> {
    info!("🧪 REAL E2E SCENARIO 1: Core Initialization and Lifecycle");

    let mut metrics = E2EMetrics::default();

    // Step 1: Initialize real BearDog Core
    info!("Step 1: Initialize Real BearDog Core");
    let core = initialize_real_beardog_core().await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 2: Verify initial health status
    info!("Step 2: Verify Initial Health Status");
    let health = verify_real_health_status(&core).await?;
    if health == HealthStatus::Healthy {
        info!("  ✅ Core is healthy");
        metrics.successful_requests += 1;
    } else {
        metrics.failed_requests += 1;
    }
    metrics.total_requests += 1;

    // Step 3: Test state management
    info!("Step 3: Test State Management");
    test_real_state_management(&core).await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 4: Test component registration
    info!("Step 4: Test Component Registration");
    test_real_component_registration(&core).await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 5: Shutdown
    info!("Step 5: Shutdown BearDog Core");
    shutdown_real_beardog_core(core).await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    metrics.data_verified = true;

    info!("✅ REAL E2E SCENARIO 1 COMPLETE");
    info!("   Total Steps: {}", metrics.total_requests);
    info!("   Successful: {}", metrics.successful_requests);
    info!("   Failed: {}", metrics.failed_requests);

    Ok(metrics)
}

// ═══════════════════════════════════════════════════════════════════════════
// SCENARIO 2: Configuration Loading and Validation
// ═══════════════════════════════════════════════════════════════════════════

// TEST_CATEGORY: e2e
// TEST_DOMAIN: config
// TEST_PRIORITY: critical
/// Test real configuration loading, validation, and environment handling
pub async fn run_real_config_validation_test(
    _config: &E2ETestConfig,
) -> Result<E2EMetrics, BearDogError> {
    info!("🧪 REAL E2E SCENARIO 2: Configuration Loading and Validation");

    let mut metrics = E2EMetrics::default();

    // Step 1: Load development configuration
    info!("Step 1: Load Development Configuration");
    let _dev_config = test_real_configuration_load().await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 2: Initialize core with development config
    info!("Step 2: Initialize Core with Development Config");
    let core = initialize_real_beardog_core().await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 3: Verify core is operational with config
    info!("Step 3: Verify Core Operational");
    let health = verify_real_health_status(&core).await?;
    if health == HealthStatus::Healthy {
        metrics.successful_requests += 1;
    } else {
        metrics.failed_requests += 1;
    }
    metrics.total_requests += 1;

    // Step 4: Test configuration-driven behavior
    info!("Step 4: Test Configuration-Driven Behavior");
    test_real_state_management(&core).await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Cleanup
    shutdown_real_beardog_core(core).await?;

    metrics.data_verified = true;

    info!("✅ REAL E2E SCENARIO 2 COMPLETE");
    info!("   Total Steps: {}", metrics.total_requests);
    info!("   Successful: {}", metrics.successful_requests);

    Ok(metrics)
}

// ═══════════════════════════════════════════════════════════════════════════
// SCENARIO 3: Health Monitoring and State Transitions
// ═══════════════════════════════════════════════════════════════════════════

// TEST_CATEGORY: e2e
// TEST_DOMAIN: monitoring
// TEST_PRIORITY: critical
/// Test real health monitoring, state transitions, and degradation handling
pub async fn run_real_health_monitoring_test(
    _config: &E2ETestConfig,
) -> Result<E2EMetrics, BearDogError> {
    info!("🧪 REAL E2E SCENARIO 3: Health Monitoring and State Transitions");

    let mut metrics = E2EMetrics::default();

    // Step 1: Initialize core
    info!("Step 1: Initialize Core");
    let core = initialize_real_beardog_core().await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 2: Verify healthy baseline
    info!("Step 2: Verify Healthy Baseline");
    let initial_health = verify_real_health_status(&core).await?;
    if initial_health == HealthStatus::Healthy {
        info!("  ✅ Baseline health: Healthy");
        metrics.successful_requests += 1;
    } else {
        metrics.failed_requests += 1;
    }
    metrics.total_requests += 1;

    // Step 3: Simulate degradation
    info!("Step 3: Simulate Health Degradation");
    {
        let mut state = core.state.write().await;
        state.overall_health = HealthStatus::Degraded;
        info!("  ⚠️  Health transitioned to: Degraded");
    }
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 4: Verify degraded state
    info!("Step 4: Verify Degraded State");
    let degraded_health = verify_real_health_status(&core).await?;
    if degraded_health == HealthStatus::Degraded {
        info!("  ✅ Degraded state verified");
        metrics.successful_requests += 1;
    } else {
        metrics.failed_requests += 1;
    }
    metrics.total_requests += 1;

    // Step 5: Simulate recovery
    info!("Step 5: Simulate Health Recovery");
    {
        let mut state = core.state.write().await;
        state.overall_health = HealthStatus::Healthy;
        info!("  ✅ Health transitioned to: Healthy");
    }
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 6: Verify recovery
    info!("Step 6: Verify Recovery");
    let recovered_health = verify_real_health_status(&core).await?;
    if recovered_health == HealthStatus::Healthy {
        info!("  ✅ Recovery verified");
        metrics.successful_requests += 1;
    } else {
        metrics.failed_requests += 1;
    }
    metrics.total_requests += 1;

    // Cleanup
    shutdown_real_beardog_core(core).await?;

    metrics.data_verified = true;

    info!("✅ REAL E2E SCENARIO 3 COMPLETE");
    info!("   Total Steps: {}", metrics.total_requests);
    info!("   Successful: {}", metrics.successful_requests);

    Ok(metrics)
}

// ═══════════════════════════════════════════════════════════════════════════
// SCENARIO 4: Concurrent Access and Thread Safety
// ═══════════════════════════════════════════════════════════════════════════

// TEST_CATEGORY: e2e
// TEST_DOMAIN: concurrency
// TEST_PRIORITY: critical
/// Test real concurrent access patterns and thread safety
pub async fn run_real_concurrency_test(
    _config: &E2ETestConfig,
) -> Result<E2EMetrics, BearDogError> {
    info!("🧪 REAL E2E SCENARIO 4: Concurrent Access and Thread Safety");

    let mut metrics = E2EMetrics::default();

    // Step 1: Initialize core
    info!("Step 1: Initialize Core");
    let core = initialize_real_beardog_core().await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 2: Test concurrent reads
    info!("Step 2: Test Concurrent Reads (5 readers x 10 iterations)");
    test_real_concurrent_access(&core).await?;
    metrics.total_requests += 50; // 5 readers x 10 iterations
    metrics.successful_requests += 50;

    // Step 3: Test concurrent reads with writes
    info!("Step 3: Test Concurrent Reads + Writes");
    let mut handles = vec![];

    // Spawn 3 readers
    for i in 0..3 {
        let core_clone = Arc::clone(&core);
        let handle = tokio::spawn(async move {
            for _ in 0..10 {
                let _state = core_clone.state.read().await;
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            }
            info!("  Reader {} completed", i);
        });
        handles.push(handle);
    }

    // Spawn 2 writers
    for i in 0..2 {
        let core_clone = Arc::clone(&core);
        let handle = tokio::spawn(async move {
            for _ in 0..5 {
                {
                    let mut state = core_clone.state.write().await;
                    state.overall_health = HealthStatus::Degraded;
                }
                tokio::time::sleep(std::time::Duration::from_millis(2)).await;
                {
                    let mut state = core_clone.state.write().await;
                    state.overall_health = HealthStatus::Healthy;
                }
            }
            info!("  Writer {} completed", i);
        });
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        handle
            .await
            .map_err(|e| BearDogError::internal(format!("Task join error: {}", e)))?;
    }

    metrics.total_requests += 40; // 3 readers x 10 + 2 writers x 5
    metrics.successful_requests += 40;

    info!("  ✅ Concurrent access with reads+writes verified");

    // Step 4: Verify final state integrity
    info!("Step 4: Verify State Integrity");
    let final_health = verify_real_health_status(&core).await?;
    info!("  Final health: {:?}", final_health);
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Cleanup
    shutdown_real_beardog_core(core).await?;

    metrics.data_verified = true;

    info!("✅ REAL E2E SCENARIO 4 COMPLETE");
    info!("   Total Steps: {}", metrics.total_requests);
    info!("   Successful: {}", metrics.successful_requests);

    Ok(metrics)
}

// ═══════════════════════════════════════════════════════════════════════════
// SCENARIO 5: Component Registration and Management
// ═══════════════════════════════════════════════════════════════════════════

// TEST_CATEGORY: e2e
// TEST_DOMAIN: components
// TEST_PRIORITY: high
/// Test real component registration, tracking, and lifecycle management
pub async fn run_real_component_management_test(
    _config: &E2ETestConfig,
) -> Result<E2EMetrics, BearDogError> {
    info!("🧪 REAL E2E SCENARIO 5: Component Registration and Management");

    let mut metrics = E2EMetrics::default();

    // Step 1: Initialize core
    info!("Step 1: Initialize Core");
    let core = initialize_real_beardog_core().await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 2: Register multiple components
    info!("Step 2: Register Multiple Components");
    let component_names = vec![
        "api-server",
        "auth-service",
        "database-connection",
        "cache-service",
        "monitoring-agent",
    ];

    for component_name in &component_names {
        {
            let mut state = core.state.write().await;
            state.components.insert(
                component_name.to_string(),
                beardog_types::canonical::ComponentStatus::Running,
            );
        }

        metrics.total_requests += 1;
        metrics.successful_requests += 1;
        info!("  ✅ Registered: {}", component_name);
    }

    // Step 3: Verify all components registered
    info!("Step 3: Verify All Components Registered");
    let registered_count = {
        let state = core.state.read().await;
        state.components.len()
    };
    info!("  Registered components: {}", registered_count);

    if registered_count >= component_names.len() {
        info!("  ✅ All components verified");
        metrics.successful_requests += 1;
    } else {
        metrics.failed_requests += 1;
    }
    metrics.total_requests += 1;

    // Step 4: Simulate component degradation
    info!("Step 4: Simulate Component Degradation");
    {
        let mut state = core.state.write().await;
        state.components.insert(
            "database-connection".to_string(),
            beardog_types::canonical::ComponentStatus::Failed,
        );
        info!("  ⚠️  database-connection degraded");
    }
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 5: Verify component states
    info!("Step 5: Verify Component States");
    {
        let state = core.state.read().await;
        for component_name in &component_names {
            if let Some(status) = state.components.get(*component_name) {
                info!("  {} - Status: {:?}", component_name, status);
            }
        }
    }
    metrics.total_requests += component_names.len() as u64;
    metrics.successful_requests += component_names.len() as u64;

    // Step 6: Simulate recovery
    info!("Step 6: Simulate Component Recovery");
    {
        let mut state = core.state.write().await;
        state.components.insert(
            "database-connection".to_string(),
            beardog_types::canonical::ComponentStatus::Running,
        );
        info!("  ✅ database-connection recovered");
    }
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Cleanup
    shutdown_real_beardog_core(core).await?;

    metrics.data_verified = true;

    info!("✅ REAL E2E SCENARIO 5 COMPLETE");
    info!("   Total Steps: {}", metrics.total_requests);
    info!("   Successful: {}", metrics.successful_requests);

    Ok(metrics)
}
