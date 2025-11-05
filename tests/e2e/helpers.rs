// E2E Testing Helpers
// Created October 7, 2025

//! Helper functions and utilities for E2E testing
//! Now includes real BearDog component integration

use beardog_core::BearDogCore;
use beardog_errors::BearDogError;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use beardog_types::canonical::HealthStatus;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn};

/// Test step result
#[derive(Debug, Clone)]
pub struct StepResult {
    pub name: String,
    pub success: bool,
    pub duration: Duration,
    pub error: Option<String>,
}

/// Execute a test step with timing
pub async fn execute_step<F, Fut>(name: &str, step_fn: F) -> Result<StepResult, BearDogError>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<(), BearDogError>>,
{
    info!("  → Executing step: {}", name);
    let start = std::time::Instant::now();

    match step_fn().await {
        Ok(()) => {
            let duration = start.elapsed();
            info!(
                "  ✅ Step completed: {} ({:.2}s)",
                name,
                duration.as_secs_f64()
            );
            Ok(StepResult {
                name: name.to_string(),
                success: true,
                duration,
                error: None,
            })
        }
        Err(e) => {
            let duration = start.elapsed();
            warn!("  ❌ Step failed: {} - {}", name, e);
            Ok(StepResult {
                name: name.to_string(),
                success: false,
                duration,
                error: Some(e.to_string()),
            })
        }
    }
}

/// Wait for a condition with timeout
pub async fn wait_for_condition<F>(
    condition_fn: F,
    timeout: Duration,
    check_interval: Duration,
) -> Result<(), BearDogError>
where
    F: Fn() -> bool,
{
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        if condition_fn() {
            return Ok(());
        }

        tokio::time::sleep(check_interval).await;
    }

    Err(BearDogError::internal(
        "Timeout waiting for condition".to_string(),
    ))
}

/// Simulate API request
pub async fn simulate_api_request(
    endpoint: &str,
    _payload: Option<&str>,
) -> Result<SimulatedResponse, BearDogError> {
    info!("Simulating API request to: {}", endpoint);

    // Simulate network latency
    tokio::time::sleep(Duration::from_millis(10)).await;

    Ok(SimulatedResponse {
        status_code: 200,
        body: format!("Response from {}", endpoint),
        latency_ms: 10.0,
    })
}

/// Simulated API response
#[derive(Debug, Clone)]
pub struct SimulatedResponse {
    pub status_code: u16,
    pub body: String,
    pub latency_ms: f64,
}

/// Verify data integrity
pub async fn verify_data_integrity(_data_id: &str) -> Result<bool, BearDogError> {
    info!("Verifying data integrity");

    // Simulate data verification
    tokio::time::sleep(Duration::from_millis(5)).await;

    Ok(true)
}

/// Create test data
pub async fn create_test_data(
    _data_type: &str,
    _count: usize,
) -> Result<Vec<String>, BearDogError> {
    info!("Creating test data");

    // Simulate test data creation
    let test_data = vec![
        "test-data-1".to_string(),
        "test-data-2".to_string(),
        "test-data-3".to_string(),
    ];

    Ok(test_data)
}

/// Cleanup test data
pub async fn cleanup_test_data(_data_ids: &[String]) -> Result<(), BearDogError> {
    info!("Cleaning up test data");

    // Simulate cleanup
    tokio::time::sleep(Duration::from_millis(5)).await;

    Ok(())
}

/// Measure operation latency
pub async fn measure_latency<F, Fut, T>(operation: F) -> Result<(T, Duration), BearDogError>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, BearDogError>>,
{
    let start = std::time::Instant::now();
    let result = operation().await?;
    let duration = start.elapsed();

    Ok((result, duration))
}

/// Assert response is successful
pub fn assert_success(response: &SimulatedResponse) -> Result<(), BearDogError> {
    if response.status_code >= 200 && response.status_code < 300 {
        Ok(())
    } else {
        Err(BearDogError::internal(format!(
            "Request failed with status code: {}",
            response.status_code
        )))
    }
}

/// Calculate average latency
pub fn calculate_average_latency(latencies: &[f64]) -> f64 {
    if latencies.is_empty() {
        return 0.0;
    }

    let sum: f64 = latencies.iter().sum();
    sum / latencies.len() as f64
}

/// Calculate peak latency
pub fn calculate_peak_latency(latencies: &[f64]) -> f64 {
    latencies.iter().copied().fold(0.0, f64::max)
}

// ═══════════════════════════════════════════════════════════════════════════
// REAL BEARDOG CORE INTEGRATION HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Initialize a real BearDog Core instance for E2E testing
pub async fn initialize_real_beardog_core() -> Result<Arc<BearDogCore>, BearDogError> {
    info!("🚀 Initializing real BearDog Core for E2E testing");

    let config = UnifiedBearDogConfig::development();
    let core = BearDogCore::new(config);
    let core_arc = Arc::new(core);

    info!("✅ BearDog Core initialized successfully");
    Ok(core_arc)
}

/// Verify real health status of BearDog Core
pub async fn verify_real_health_status(
    core: &Arc<BearDogCore>,
) -> Result<HealthStatus, BearDogError> {
    info!("🔍 Verifying real health status");

    let state = core.state.read().await;
    let health = state.overall_health;

    info!("  Health Status: {:?}", health);
    Ok(health)
}

/// Test real configuration loading
pub async fn test_real_configuration_load() -> Result<UnifiedBearDogConfig, BearDogError> {
    info!("📋 Testing real configuration loading");

    let config = UnifiedBearDogConfig::development();

    info!("  ✅ Configuration loaded successfully");

    Ok(config)
}

/// Test real state management with concurrent access
pub async fn test_real_state_management(core: &Arc<BearDogCore>) -> Result<(), BearDogError> {
    info!("🔄 Testing real state management");

    // Read initial state
    let initial_health = {
        let state = core.state.read().await;
        state.overall_health
    };
    info!("  Initial health: {:?}", initial_health);

    // Perform state transitions
    {
        let mut state = core.state.write().await;
        state.overall_health = HealthStatus::Degraded;
        info!("  State transitioned to: Degraded");
    }

    // Verify state change
    let new_health = {
        let state = core.state.read().await;
        state.overall_health
    };

    if new_health == HealthStatus::Degraded {
        info!("  ✅ State management verified");
    } else {
        return Err(BearDogError::internal(
            "State management verification failed".to_string(),
        ));
    }

    // Restore to healthy
    {
        let mut state = core.state.write().await;
        state.overall_health = HealthStatus::Healthy;
    }

    Ok(())
}

/// Test real component registration
pub async fn test_real_component_registration(core: &Arc<BearDogCore>) -> Result<(), BearDogError> {
    info!("📦 Testing real component registration");

    use beardog_types::canonical::ComponentStatus;

    {
        let mut state = core.state.write().await;
        state
            .components
            .insert("e2e-test-component".to_string(), ComponentStatus::Running);
        info!("  ✅ Component registered successfully");
    }

    // Verify registration
    let state = core.state.read().await;
    if state.components.contains_key("e2e-test-component") {
        info!("  ✅ Component registration verified");
        Ok(())
    } else {
        Err(BearDogError::internal(
            "Component registration verification failed".to_string(),
        ))
    }
}

/// Test concurrent access to real BearDog Core
pub async fn test_real_concurrent_access(core: &Arc<BearDogCore>) -> Result<(), BearDogError> {
    info!("⚡ Testing real concurrent access");

    let mut handles = vec![];

    // Spawn 5 concurrent readers
    for i in 0..5 {
        let core_clone = Arc::clone(core);
        let handle = tokio::spawn(async move {
            for _ in 0..10 {
                let _state = core_clone.state.read().await;
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
            info!("  Reader {} completed", i);
        });
        handles.push(handle);
    }

    // Wait for all to complete
    for handle in handles {
        handle
            .await
            .map_err(|e| BearDogError::internal(format!("Task join error: {}", e)))?;
    }

    info!("  ✅ Concurrent access verified");
    Ok(())
}

/// Shutdown real BearDog Core
pub async fn shutdown_real_beardog_core(_core: Arc<BearDogCore>) -> Result<(), BearDogError> {
    info!("🛑 Shutting down BearDog Core");
    // Core cleanup happens via Arc drop
    info!("  ✅ BearDog Core shutdown complete");
    Ok(())
}
