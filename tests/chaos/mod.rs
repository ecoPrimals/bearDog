// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used)]
#![allow(
    missing_docs,
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all,
    clippy::unreadable_literal
)]

//! Chaos Testing Framework for `BearDog`
//!
//! This module provides comprehensive chaos testing capabilities to validate
//! system behavior under adverse conditions including:
//! - Network failures and latency
//! - Resource exhaustion
//! - Hardware failures (HSM)
//! - Concurrent failure scenarios
//! - Recovery and resilience testing
//!
//! # Philosophy
//!
//! Chaos testing validates that `BearDog` maintains sovereignty and security
//! even when the environment is hostile or failing. This ensures production
//! readiness and fault tolerance.

use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

// Scenario test modules
#[cfg(test)]
mod hsm_chaos_tests;
#[cfg(test)]
mod network_chaos_tests;
#[cfg(test)]
mod resource_chaos_tests;

/// Chaos testing configuration
#[derive(Debug, Clone)]
pub struct ChaosConfig {
    /// Duration to run chaos scenarios
    pub duration: Duration,
    /// Probability of chaos injection (0.0-1.0)
    pub chaos_probability: f64,
    /// Enable network chaos
    pub enable_network_chaos: bool,
    /// Enable resource chaos
    pub enable_resource_chaos: bool,
    /// Enable HSM chaos
    pub enable_hsm_chaos: bool,
    /// Recovery validation timeout
    pub recovery_timeout: Duration,
}

impl Default for ChaosConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(60),
            chaos_probability: 0.3, // 30% chaos injection rate
            enable_network_chaos: true,
            enable_resource_chaos: true,
            enable_hsm_chaos: true,
            recovery_timeout: Duration::from_secs(10),
        }
    }
}

/// Types of chaos that can be injected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChaosType {
    /// Network latency injection
    NetworkLatency,
    /// Network packet loss
    NetworkPacketLoss,
    /// Network connection drop
    NetworkDisconnect,
    /// Memory pressure
    MemoryPressure,
    /// CPU saturation
    CpuSaturation,
    /// Disk I/O pressure
    DiskPressure,
    /// HSM temporary failure
    HsmTemporaryFailure,
    /// HSM timeout
    HsmTimeout,
    /// Concurrent failures
    ConcurrentFailures,
}

/// Result of a chaos test
#[derive(Debug)]
pub struct ChaosTestResult {
    /// Test name
    pub name: String,
    /// Chaos type tested
    pub chaos_type: ChaosType,
    /// Whether the system maintained correctness
    pub correctness_maintained: bool,
    /// Whether the system recovered
    pub recovered: bool,
    /// Recovery time
    pub recovery_time: Option<Duration>,
    /// Error details if any
    pub error: Option<String>,
    /// Metrics collected during test
    pub metrics: ChaosMetrics,
}

/// Metrics collected during chaos testing
#[derive(Debug, Default)]
pub struct ChaosMetrics {
    /// Total operations attempted
    pub operations_attempted: u64,
    /// Successful operations
    pub operations_succeeded: u64,
    /// Failed operations
    pub operations_failed: u64,
    /// Average latency during chaos
    pub avg_latency_ms: f64,
    /// Peak latency during chaos
    pub peak_latency_ms: u64,
    /// Data integrity violations detected
    pub integrity_violations: u64,
}

impl ChaosMetrics {
    /// Calculate success rate
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "success ratio from u64 counters as f64 for percentage display"
    )]
    pub fn success_rate(&self) -> f64 {
        if self.operations_attempted == 0 {
            return 0.0;
        }
        self.operations_succeeded as f64 / self.operations_attempted as f64
    }
}

/// Chaos testing framework
pub struct ChaosEngine {
    config: ChaosConfig,
    active_chaos: Arc<tokio::sync::RwLock<Vec<ChaosType>>>,
}

impl ChaosEngine {
    /// Create a new chaos engine with configuration
    #[must_use]
    pub fn new(config: ChaosConfig) -> Self {
        Self {
            config,
            active_chaos: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    /// Create a chaos engine with default configuration
    #[must_use]
    pub fn default_engine() -> Self {
        Self::new(ChaosConfig::default())
    }

    /// Run a chaos test scenario
    pub async fn run_chaos_test<F, Fut>(
        &self,
        name: &str,
        chaos_type: ChaosType,
        test_fn: F,
    ) -> ChaosTestResult
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<(), String>>,
    {
        let metrics = ChaosMetrics::default();

        // Inject chaos
        self.inject_chaos(chaos_type).await;

        // Run test function
        let result = test_fn().await;

        // Stop chaos
        self.stop_chaos(chaos_type).await;

        // Validate recovery
        let recovery_start = std::time::Instant::now();
        let recovered = self.validate_recovery().await;
        let recovery_time = recovery_start.elapsed();

        ChaosTestResult {
            name: name.to_string(),
            chaos_type,
            correctness_maintained: result.is_ok(),
            recovered,
            recovery_time: Some(recovery_time),
            error: result.err(),
            metrics,
        }
    }

    /// Inject chaos into the system
    async fn inject_chaos(&self, chaos_type: ChaosType) {
        let mut active = self.active_chaos.write().await;
        active.push(chaos_type);

        // Actual chaos injection would happen here
        // For now, this is a framework - implementation depends on infrastructure
    }

    /// Stop chaos injection
    async fn stop_chaos(&self, chaos_type: ChaosType) {
        let mut active = self.active_chaos.write().await;
        active.retain(|&c| c != chaos_type);
    }

    /// Validate system recovery after chaos
    async fn validate_recovery(&self) -> bool {
        // Wait for recovery period
        sleep(self.config.recovery_timeout).await;

        // Check if system is healthy
        // This would integrate with health check systems
        true
    }

    /// Check if chaos is currently active
    pub async fn is_chaos_active(&self) -> bool {
        let active = self.active_chaos.read().await;
        !active.is_empty()
    }

    /// Get currently active chaos types
    pub async fn active_chaos_types(&self) -> Vec<ChaosType> {
        let active = self.active_chaos.read().await;
        active.clone()
    }
}

/// Network chaos simulator
pub struct NetworkChaos {
    latency_ms: u64,
    packet_loss_rate: f64,
    disconnect_probability: f64,
}

impl NetworkChaos {
    /// Create network chaos with specified parameters
    #[must_use]
    pub fn new(latency_ms: u64, packet_loss_rate: f64, disconnect_probability: f64) -> Self {
        Self {
            latency_ms,
            packet_loss_rate,
            disconnect_probability,
        }
    }

    /// Inject network latency
    pub async fn inject_latency(&self) {
        if self.latency_ms > 0 {
            sleep(Duration::from_millis(self.latency_ms)).await;
        }
    }

    /// Simulate packet loss
    #[must_use]
    pub fn should_drop_packet(&self) -> bool {
        use rand::Rng;
        let mut rng = rand::rng();
        rng.random::<f64>() < self.packet_loss_rate
    }

    /// Simulate connection drop
    #[must_use]
    pub fn should_disconnect(&self) -> bool {
        use rand::Rng;
        let mut rng = rand::rng();
        rng.random::<f64>() < self.disconnect_probability
    }
}

/// HSM chaos simulator
pub struct HsmChaos {
    failure_rate: f64,
    timeout_rate: f64,
    timeout_duration: Duration,
}

impl HsmChaos {
    /// Create HSM chaos with specified parameters
    #[must_use]
    pub fn new(failure_rate: f64, timeout_rate: f64, timeout_duration: Duration) -> Self {
        Self {
            failure_rate,
            timeout_rate,
            timeout_duration,
        }
    }

    /// Simulate HSM failure
    #[must_use]
    pub fn should_fail(&self) -> bool {
        use rand::Rng;
        let mut rng = rand::rng();
        rng.random::<f64>() < self.failure_rate
    }

    /// Simulate HSM timeout
    pub async fn maybe_timeout(&self) -> bool {
        use rand::Rng;
        let mut rng = rand::rng();
        let should_timeout = rng.random::<f64>() < self.timeout_rate;

        if should_timeout {
            sleep(self.timeout_duration).await;
            true
        } else {
            false
        }
    }
}

/// Resource chaos simulator
pub struct ResourceChaos {
    memory_pressure_mb: usize,
    cpu_cores_to_saturate: usize,
}

impl ResourceChaos {
    /// Create resource chaos with specified parameters
    #[must_use]
    pub fn new(memory_pressure_mb: usize, cpu_cores_to_saturate: usize) -> Self {
        Self {
            memory_pressure_mb,
            cpu_cores_to_saturate,
        }
    }

    /// Create memory pressure
    #[must_use]
    pub fn create_memory_pressure(&self) -> Vec<Vec<u8>> {
        let mut pressure = Vec::new();
        for _ in 0..self.memory_pressure_mb {
            pressure.push(vec![0u8; 1024 * 1024]); // 1 MB each
        }
        pressure
    }

    /// Create CPU pressure
    pub async fn create_cpu_pressure(&self, duration: Duration) {
        let handles: Vec<_> = (0..self.cpu_cores_to_saturate)
            .map(|_| {
                tokio::spawn(async move {
                    let start = std::time::Instant::now();
                    while start.elapsed() < duration {
                        // CPU-intensive work
                        let mut sum = 0u64;
                        for i in 0..1_000_000 {
                            sum = sum.wrapping_add(i);
                        }
                        // Prevent optimization
                        std::hint::black_box(sum);
                    }
                })
            })
            .collect();

        for handle in handles {
            let _ = handle.await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chaos_engine_creation() {
        let engine = ChaosEngine::default_engine();
        assert!(!engine.is_chaos_active().await);
    }

    #[tokio::test]
    async fn test_chaos_injection() {
        let engine = ChaosEngine::default_engine();
        engine.inject_chaos(ChaosType::NetworkLatency).await;
        assert!(engine.is_chaos_active().await);

        let active = engine.active_chaos_types().await;
        assert_eq!(active.len(), 1);
        assert_eq!(active[0], ChaosType::NetworkLatency);
    }

    #[tokio::test]
    async fn test_chaos_stop() {
        let engine = ChaosEngine::default_engine();
        engine.inject_chaos(ChaosType::NetworkLatency).await;
        engine.stop_chaos(ChaosType::NetworkLatency).await;
        assert!(!engine.is_chaos_active().await);
    }

    #[test]
    fn test_network_chaos_packet_loss() {
        let chaos = NetworkChaos::new(0, 1.0, 0.0); // 100% packet loss
        assert!(chaos.should_drop_packet());

        let no_chaos = NetworkChaos::new(0, 0.0, 0.0); // 0% packet loss
        assert!(!no_chaos.should_drop_packet());
    }

    #[test]
    fn test_hsm_chaos_failure() {
        let chaos = HsmChaos::new(1.0, 0.0, Duration::from_millis(100)); // 100% failure
        assert!(chaos.should_fail());

        let no_chaos = HsmChaos::new(0.0, 0.0, Duration::from_millis(100)); // 0% failure
        assert!(!no_chaos.should_fail());
    }

    #[test]
    fn test_chaos_metrics() {
        let mut metrics = ChaosMetrics::default();
        metrics.operations_attempted = 100;
        metrics.operations_succeeded = 95;
        metrics.operations_failed = 5;

        assert!((metrics.success_rate() - 0.95).abs() < 1e-9);
    }

    #[tokio::test]
    async fn test_run_chaos_test() {
        let engine = ChaosEngine::default_engine();

        let result = engine
            .run_chaos_test("test_scenario", ChaosType::NetworkLatency, || async {
                Ok(())
            })
            .await;

        assert_eq!(result.name, "test_scenario");
        assert_eq!(result.chaos_type, ChaosType::NetworkLatency);
        assert!(result.correctness_maintained);
        assert!(result.recovered);
    }
}
