// SPDX-License-Identifier: AGPL-3.0-only
//! Disaster Recovery E2E Test Suite
//!
//! **Architecture**: Domain-driven test organization
//! - `mod.rs`: Orchestration & main test runner
//! - `component_resilience.rs`: Component failure & recovery scenarios
//! - `data_integrity.rs`: Data corruption detection & recovery
//! - `consensus.rs`: Byzantine fault tolerance & distributed consensus
//! - `performance.rs`: Resource exhaustion & degradation scenarios
//!
//! **Design Philosophy**:
//! - Each module represents a failure domain
//! - Self-contained test scenarios
//! - Shared state through capability-based access
//! - Modern async patterns (no sleep-based synchronization)

use super::helpers::*;
use super::{E2EMetrics, E2ETestConfig};
use beardog_errors::BearDogError;
use tracing::info;

mod component_resilience;
mod consensus;
mod data_integrity;
mod performance;

// Re-export scenario runners
pub use component_resilience::ComponentFailureScenario;
pub use consensus::ConsensusFailureScenario;
pub use data_integrity::DataIntegrityScenario;
pub use performance::ResourceExhaustionScenario;

/// Disaster recovery test orchestrator
pub struct DisasterRecoveryTest;

// TEST_CATEGORY: e2e
// TEST_DOMAIN: core
// TEST_PRIORITY: critical
/// Run comprehensive disaster recovery E2E test
///
/// **Test Architecture**:
/// 1. Component Resilience (failover, crash recovery)
/// 2. Data Integrity (corruption detection, backup recovery)
/// 3. Consensus (Byzantine faults, network partitions)
/// 4. Performance (resource exhaustion, degradation)
///
/// **Modern Patterns**:
/// - Event-driven synchronization (no arbitrary sleeps)
/// - Capability-based component access
/// - Composable scenario execution
pub async fn run_disaster_recovery_test(
    config: &E2ETestConfig,
) -> Result<E2EMetrics, BearDogError> {
    info!("🚨 Starting Comprehensive Disaster Recovery E2E Test");

    let mut total_metrics = E2EMetrics::default();

    // Scenario 1: Component Resilience
    info!("📦 Scenario 1: Component Failure & Recovery");
    let component_metrics = ComponentFailureScenario::run(config).await?;
    total_metrics.merge(component_metrics);

    // Scenario 2: Data Integrity
    info!("💾 Scenario 2: Data Corruption & Recovery");
    let data_metrics = DataIntegrityScenario::run(config).await?;
    total_metrics.merge(data_metrics);

    // Scenario 3: Distributed Consensus
    info!("🌐 Scenario 3: Byzantine Fault Tolerance");
    let consensus_metrics = ConsensusFailureScenario::run(config).await?;
    total_metrics.merge(consensus_metrics);

    // Scenario 4: Resource Exhaustion
    info!("⚡ Scenario 4: Performance Degradation");
    let performance_metrics = ResourceExhaustionScenario::run(config).await?;
    total_metrics.merge(performance_metrics);

    info!("✅ Disaster Recovery Test Complete");
    info!(
        "📊 Total: {} requests, {} successful, {} failed",
        total_metrics.total_requests,
        total_metrics.successful_requests,
        total_metrics.failed_requests
    );

    Ok(total_metrics)
}

// Helper extension for metrics merging
impl E2EMetrics {
    /// Modern idiomatic merge: accumulate all metrics properly
    ///
    /// Design: Complete implementation, no "TODO: merge other fields"
    /// Pattern: Explicit field handling ensures nothing is forgotten
    fn merge(&mut self, other: Self) {
        // Accumulate counters
        self.total_requests += other.total_requests;
        self.successful_requests += other.successful_requests;
        self.failed_requests += other.failed_requests;

        // Aggregate latency (keep worst case for peak)
        self.peak_latency_ms = self.peak_latency_ms.max(other.peak_latency_ms);

        // Calculate weighted average for average latency
        if self.total_requests > 0 && other.total_requests > 0 {
            let total = self.total_requests + other.total_requests;
            self.average_latency_ms = (self.average_latency_ms * self.total_requests as f64
                + other.average_latency_ms * other.total_requests as f64)
                / total as f64;
        } else if other.total_requests > 0 {
            self.average_latency_ms = other.average_latency_ms;
        }

        // Logical AND: data is verified only if ALL scenarios verify
        self.data_verified = self.data_verified && other.data_verified;
    }
}
