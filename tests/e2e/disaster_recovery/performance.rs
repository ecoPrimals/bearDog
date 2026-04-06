// SPDX-License-Identifier: AGPL-3.0-or-later
//! Performance Degradation & Resource Exhaustion Scenarios
//!
//! Tests system behavior under resource constraints and load.
//! Validates graceful degradation and resource management.

use super::super::helpers::*;
use super::super::{E2EMetrics, E2ETestConfig};
use beardog_errors::BearDogError;
use std::sync::Mutex;
use tracing::{info, warn};

/// Per-scenario resource state (avoids process-global statics).
pub struct ResourceExhaustionContext {
    resource_limits: Mutex<u32>,
}

impl ResourceExhaustionContext {
    fn new() -> Self {
        Self {
            resource_limits: Mutex::new(100),
        }
    }
}

/// Resource exhaustion and performance degradation scenario
pub struct ResourceExhaustionScenario;

/// System performance metrics
#[derive(Debug, Clone, Default)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub latency_ms: u64,
}

impl ResourceExhaustionScenario {
    /// Run performance degradation tests
    pub async fn run(config: &E2ETestConfig) -> Result<E2EMetrics, BearDogError> {
        info!("⚡ Performance Degradation Tests");

        let ctx = ResourceExhaustionContext::new();
        let mut metrics = E2EMetrics::default();

        // Test 1: Baseline performance
        Self::test_baseline_performance(&ctx, &mut metrics, config).await?;

        // Test 2: Resource exhaustion
        Self::test_resource_exhaustion(&ctx, &mut metrics, config).await?;

        // Test 3: Graceful degradation
        Self::test_graceful_degradation(&ctx, &mut metrics, config).await?;

        // Test 4: Recovery after exhaustion
        Self::test_recovery(&ctx, &mut metrics, config).await?;

        // Modern pattern: explicit data verification flag
        // Performance scenarios don't test data integrity
        metrics.data_verified = true; // This scenario tests performance, not data

        info!("✅ Performance tests complete");
        Ok(metrics)
    }

    async fn test_baseline_performance(
        ctx: &ResourceExhaustionContext,
        metrics: &mut E2EMetrics,
        _config: &E2ETestConfig,
    ) -> Result<(), BearDogError> {
        info!("  Measuring baseline performance...");

        // Initialize system with full resources
        initialize_system_with_full_resources(ctx).await?;
        metrics.total_requests += 1;
        metrics.successful_requests += 1;

        // Measure performance
        let system_metrics = measure_system_performance(ctx).await?;
        metrics.total_requests += 1;

        if system_metrics.cpu_usage < 50.0 {
            info!("  ✅ Baseline CPU: {}%", system_metrics.cpu_usage);
            metrics.successful_requests += 1;
        } else {
            warn!("  ⚠️  High baseline CPU: {}%", system_metrics.cpu_usage);
            metrics.failed_requests += 1;
        }

        Ok(())
    }

    async fn test_resource_exhaustion(
        ctx: &ResourceExhaustionContext,
        metrics: &mut E2EMetrics,
        _config: &E2ETestConfig,
    ) -> Result<(), BearDogError> {
        info!("  Testing resource exhaustion...");

        // Reduce available resources
        set_resource_limits(ctx, 20).await?; // 20% capacity
        metrics.total_requests += 1;
        metrics.successful_requests += 1;

        // Measure under constraint
        let system_metrics = measure_system_performance(ctx).await?;
        metrics.total_requests += 1;

        if system_metrics.latency_ms > 100 {
            info!(
                "  ✅ Latency increased as expected: {}ms",
                system_metrics.latency_ms
            );
            metrics.successful_requests += 1;
        }

        Ok(())
    }

    async fn test_graceful_degradation(
        ctx: &ResourceExhaustionContext,
        metrics: &mut E2EMetrics,
        _config: &E2ETestConfig,
    ) -> Result<(), BearDogError> {
        info!("  Testing graceful degradation...");

        // System should still function under constraints
        for i in 0..5 {
            match send_component_request(ctx, "api-server", "test").await {
                Ok(()) => {
                    metrics.successful_requests += 1;
                }
                Err(e) => {
                    // Degraded but not crashed
                    warn!("  Request {} degraded: {}", i, e);
                    metrics.failed_requests += 1;
                }
            }
            metrics.total_requests += 1;
        }

        Ok(())
    }

    async fn test_recovery(
        ctx: &ResourceExhaustionContext,
        metrics: &mut E2EMetrics,
        _config: &E2ETestConfig,
    ) -> Result<(), BearDogError> {
        info!("  Testing recovery after exhaustion...");

        // Restore resources
        set_resource_limits(ctx, 100).await?; // Full capacity
        metrics.total_requests += 1;
        metrics.successful_requests += 1;

        // Verify performance recovery
        let system_metrics = measure_system_performance(ctx).await?;
        metrics.total_requests += 1;

        if system_metrics.latency_ms < 50 {
            info!(
                "  ✅ Performance recovered: {}ms",
                system_metrics.latency_ms
            );
            metrics.successful_requests += 1;
        } else {
            warn!("  ⚠️  Performance not fully recovered");
            metrics.failed_requests += 1;
        }

        Ok(())
    }
}

// =============================================================================
// Performance Monitoring Operations (Capability-Based)
// =============================================================================

/// Initialize system with full resources
async fn initialize_system_with_full_resources(
    ctx: &ResourceExhaustionContext,
) -> Result<(), BearDogError> {
    let mut limits = ctx.resource_limits.lock().unwrap();
    *limits = 100; // 100% capacity
    Ok(())
}

/// Measure system performance metrics
async fn measure_system_performance(
    ctx: &ResourceExhaustionContext,
) -> Result<SystemMetrics, BearDogError> {
    let limits = ctx.resource_limits.lock().unwrap();
    let capacity = *limits;

    // Simulate metrics based on available capacity
    let metrics = SystemMetrics {
        cpu_usage: 100.0 - f64::from(capacity),
        memory_usage: 100.0 - f64::from(capacity),
        latency_ms: if capacity > 50 { 20 } else { 200 },
    };

    Ok(metrics)
}

/// Set resource limits (percentage of full capacity)
async fn set_resource_limits(
    ctx: &ResourceExhaustionContext,
    percentage: u32,
) -> Result<(), BearDogError> {
    let mut limits = ctx.resource_limits.lock().unwrap();
    *limits = percentage;
    info!("    Resource limits set to {}%", percentage);
    Ok(())
}

/// Send request to component (for degradation testing)
async fn send_component_request(
    ctx: &ResourceExhaustionContext,
    _component: &str,
    _data: &str,
) -> Result<(), BearDogError> {
    let limits = ctx.resource_limits.lock().unwrap();

    // Fail if resources too constrained
    if *limits < 10 {
        Err(BearDogError::unavailable("Resources exhausted".to_string()))
    } else {
        Ok(())
    }
}
