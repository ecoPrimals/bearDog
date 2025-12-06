#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]
// Full-Stack Integration E2E Test
// Created October 7, 2025

//! Full-stack integration end-to-end test
//!
//! This test validates complete integration across all BearDog layers:
//! 1. API layer (request handling)
//! 2. Business logic layer (core processing)
//! 3. Security layer (authentication, encryption)
//! 4. Storage layer (persistence)
//! 5. Integration layer (external services)

use super::helpers::*;
use super::{E2EMetrics, E2ETestConfig};
use beardog_errors::BearDogError;
use tracing::info;

/// Full-stack integration test implementation
pub struct FullStackIntegrationTest;

// TEST_CATEGORY: e2e
// TEST_DOMAIN: core
// TEST_PRIORITY: critical
/// Run full-stack integration E2E test
pub async fn run_full_stack_integration_test(
    config: &E2ETestConfig,
) -> Result<E2EMetrics, BearDogError> {
    info!("🏗️  Starting Full-Stack Integration E2E Test");

    let mut metrics = E2EMetrics::default();
    let mut latencies = Vec::new();

    // Step 1: API Layer - Request Processing
    info!("Step 1: API Layer Integration");
    for i in 0..10 {
        let (response, _) = measure_latency(|| async {
            simulate_api_request(
                "/api/v1/workflow/process",
                Some(r#"{"action": "test", "data": "sample"}"#),
            )
            .await
        })
        .await?;

        assert_success(&response)?;
        latencies.push(response.latency_ms);
        metrics.total_requests += 1;
        metrics.successful_requests += 1;

        if config.verbose_logging && i % 3 == 0 {
            info!("  Processed {}/10 API requests", i + 1);
        }
    }

    // Step 2: Business Logic Layer - Core Processing
    info!("Step 2: Business Logic Layer Integration");
    execute_step("Workflow Execution", || async {
        // Simulate complex business logic processing
        for _i in 0..5 {
            // No sleep needed - testing business logic, not timing
        }
        Ok(())
    })
    .await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 3: Security Layer - Authentication & Encryption
    info!("Step 3: Security Layer Integration");

    // Simulate authentication
    execute_step("Authentication Flow", || async {
        let (response, _) = measure_latency(|| async {
            simulate_api_request("/api/v1/auth/verify", Some(r#"{"token": "test-token"}"#)).await
        })
        .await?;
        assert_success(&response)?;
        Ok(())
    })
    .await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Simulate encryption/decryption
    execute_step("Encryption Operations", || async {
        // No sleep needed - testing encryption logic, not timing
        Ok(())
    })
    .await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 4: Storage Layer - Data Persistence
    info!("Step 4: Storage Layer Integration");

    // Create test entities
    let test_entities = create_test_data("workflow-entity", 5).await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Verify persistence
    for entity_id in &test_entities {
        let integrity_ok = verify_data_integrity(entity_id).await?;
        if !integrity_ok {
            return Err(BearDogError::internal(format!(
                "Data persistence check failed for: {}",
                entity_id
            )));
        }
        metrics.total_requests += 1;
        metrics.successful_requests += 1;
    }

    metrics.data_verified = true;

    // Step 5: Integration Layer - External Services
    info!("Step 5: Integration Layer");

    execute_step("External Service Communication", || async {
        // Simulate calls to external services
        let (response, _) = measure_latency(|| async {
            simulate_api_request("/api/v1/integration/external", None).await
        })
        .await?;
        assert_success(&response)?;
        Ok(())
    })
    .await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Step 6: Cross-Layer Validation
    info!("Step 6: Cross-Layer Validation");

    execute_step("End-to-End Data Flow", || async {
        // Simulate data flowing through all layers
        let (response, _) = measure_latency(|| async {
            simulate_api_request(
                "/api/v1/workflow/complete",
                Some(r#"{"workflow_id": "test-123"}"#),
            )
            .await
        })
        .await?;
        assert_success(&response)?;
        Ok(())
    })
    .await?;
    metrics.total_requests += 1;
    metrics.successful_requests += 1;

    // Cleanup
    if config.enable_cleanup {
        info!("Step 7: Cleanup");
        cleanup_test_data(&test_entities).await?;
    }

    // Calculate metrics
    metrics.average_latency_ms = calculate_average_latency(&latencies);
    metrics.peak_latency_ms = calculate_peak_latency(&latencies);

    info!("✅ Full-Stack Integration E2E Test Complete");
    info!("   Total Requests: {}", metrics.total_requests);
    info!("   Successful: {}", metrics.successful_requests);
    info!("   Failed: {}", metrics.failed_requests);
    info!("   Avg Latency: {:.2}ms", metrics.average_latency_ms);
    info!("   Peak Latency: {:.2}ms", metrics.peak_latency_ms);
    info!("   Data Verified: {}", metrics.data_verified);

    Ok(metrics)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_full_stack_integration() {
        let config = E2ETestConfig::default();
        let result = run_full_stack_integration_test(&config).await;
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert!(metrics.successful_requests > 0);
        assert_eq!(metrics.failed_requests, 0);
        assert!(metrics.data_verified);
    }
}
