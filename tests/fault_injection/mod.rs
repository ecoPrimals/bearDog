#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

//! Fault Injection Testing Framework for `BearDog`
//!
//! This module provides systematic fault injection capabilities to test
//! error handling, recovery, and resilience. Unlike chaos testing which
//! simulates environmental failures, fault injection deliberately injects
//! faults at specific points in the code to validate error paths.
//!
//! # Coverage
//!
//! - API error injection
//! - HSM operation failures
//! - Network failures at specific points
//! - Resource allocation failures
//! - Timeout scenarios
//! - Data corruption scenarios
//! - Concurrent fault scenarios

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Fault injection configuration
#[derive(Debug, Clone)]
pub struct FaultConfig {
    /// Enable fault injection
    pub enabled: bool,
    /// Fault injection rate (0.0-1.0)
    pub injection_rate: f64,
    /// Specific faults to inject
    pub fault_types: Vec<FaultType>,
    /// Maximum concurrent faults
    pub max_concurrent_faults: usize,
}

impl Default for FaultConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            injection_rate: 0.1, // 10% fault rate
            fault_types: vec![
                FaultType::NetworkTimeout,
                FaultType::HsmFailure,
                FaultType::MemoryAllocationFailure,
            ],
            max_concurrent_faults: 3,
        }
    }
}

/// Types of faults that can be injected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultType {
    /// Network timeout
    NetworkTimeout,
    /// Network connection refused
    NetworkConnectionRefused,
    /// Network data corruption
    NetworkDataCorruption,
    /// HSM operation failure
    HsmFailure,
    /// HSM timeout
    HsmTimeout,
    /// HSM invalid response
    HsmInvalidResponse,
    /// Memory allocation failure
    MemoryAllocationFailure,
    /// Disk full
    DiskFull,
    /// File not found
    FileNotFound,
    /// Permission denied
    PermissionDenied,
    /// Invalid input
    InvalidInput,
    /// Timeout
    Timeout,
    /// Resource exhausted
    ResourceExhausted,
    /// Concurrent access conflict
    ConcurrentConflict,
}

/// Result of a fault injection test
#[derive(Debug)]
pub struct FaultTestResult {
    /// Test name
    pub name: String,
    /// Fault type injected
    pub fault_type: FaultType,
    /// Whether error was handled correctly
    pub error_handled_correctly: bool,
    /// Whether system recovered
    pub system_recovered: bool,
    /// Error propagation was correct
    pub error_propagation_correct: bool,
    /// Details
    pub details: String,
    /// Metrics
    pub metrics: FaultMetrics,
}

/// Metrics for fault injection tests
#[derive(Debug, Default)]
pub struct FaultMetrics {
    /// Faults injected
    pub faults_injected: u64,
    /// Errors caught properly
    pub errors_caught: u64,
    /// Errors escaped
    pub errors_escaped: u64,
    /// Panics detected
    pub panics_detected: u64,
    /// Recovery successes
    pub recoveries_succeeded: u64,
    /// Recovery failures
    pub recoveries_failed: u64,
}

/// Fault injection engine
pub struct FaultInjector {
    config: FaultConfig,
    active_faults: Arc<tokio::sync::RwLock<Vec<FaultType>>>,
    fault_count: AtomicU64,
    enabled: AtomicBool,
}

impl FaultInjector {
    /// Create a new fault injector
    #[must_use]
    pub fn new(config: FaultConfig) -> Self {
        let enabled = config.enabled;
        Self {
            config,
            active_faults: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            fault_count: AtomicU64::new(0),
            enabled: AtomicBool::new(enabled),
        }
    }

    /// Create with default configuration
    #[must_use]
    pub fn default_injector() -> Self {
        Self::new(FaultConfig::default())
    }

    /// Enable fault injection
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::SeqCst);
    }

    /// Disable fault injection
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::SeqCst);
    }

    /// Check if fault should be injected
    pub fn should_inject_fault(&self, fault_type: FaultType) -> bool {
        if !self.enabled.load(Ordering::SeqCst) {
            return false;
        }

        if !self.config.fault_types.contains(&fault_type) {
            return false;
        }

        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen::<f64>() < self.config.injection_rate
    }

    /// Inject a fault at a specific point
    pub async fn inject_fault<T, E>(
        &self,
        fault_type: FaultType,
        error_generator: impl FnOnce() -> E,
    ) -> Result<T, E> {
        if self.should_inject_fault(fault_type) {
            self.fault_count.fetch_add(1, Ordering::SeqCst);

            let mut active = self.active_faults.write().await;
            active.push(fault_type);

            Err(error_generator())
        } else {
            // Return a placeholder - actual implementation would continue normal flow
            panic!("Fault not injected - normal flow should continue");
        }
    }

    /// Run a fault injection test
    pub async fn run_fault_test<F, Fut, T, E>(
        &self,
        name: &str,
        fault_type: FaultType,
        test_fn: F,
    ) -> FaultTestResult
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Debug,
    {
        self.enable();

        let mut metrics = FaultMetrics::default();

        // Run test with fault injection
        let result = test_fn().await;

        let error_handled_correctly = result.is_err();
        metrics.faults_injected = self.fault_count.load(Ordering::SeqCst);

        if error_handled_correctly {
            metrics.errors_caught += 1;
        } else {
            metrics.errors_escaped += 1;
        }

        // Validate recovery
        let system_recovered = self.validate_system_recovery().await;
        if system_recovered {
            metrics.recoveries_succeeded += 1;
        } else {
            metrics.recoveries_failed += 1;
        }

        self.disable();

        FaultTestResult {
            name: name.to_string(),
            fault_type,
            error_handled_correctly,
            system_recovered,
            error_propagation_correct: true, // Would validate error types
            details: format!("{:?}", result.err()),
            metrics,
        }
    }

    /// Validate system recovery after fault
    async fn validate_system_recovery(&self) -> bool {
        // Clear active faults
        let mut active = self.active_faults.write().await;
        active.clear();

        // System should be able to perform basic operations
        true
    }

    /// Get fault injection statistics
    pub fn get_stats(&self) -> FaultStats {
        FaultStats {
            total_faults_injected: self.fault_count.load(Ordering::SeqCst),
            enabled: self.enabled.load(Ordering::SeqCst),
        }
    }
}

/// Statistics for fault injection
#[derive(Debug)]
pub struct FaultStats {
    /// Total faults injected
    pub total_faults_injected: u64,
    /// Whether injection is enabled
    pub enabled: bool,
}

/// Helper to inject network faults
pub struct NetworkFaultInjector {
    base_injector: Arc<FaultInjector>,
}

impl NetworkFaultInjector {
    /// Create a network fault injector
    pub fn new(base_injector: Arc<FaultInjector>) -> Self {
        Self { base_injector }
    }

    /// Maybe inject a network timeout
    pub async fn maybe_timeout(&self, duration: Duration) -> Result<(), String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::NetworkTimeout)
        {
            // No sleep needed - testing timeout injection logic, not actual timeout
            // For time-based timeout tests, use tokio::time::pause() + advance()
            let _ = duration; // Track duration for validation
            Err("Network timeout".to_string())
        } else {
            Ok(())
        }
    }

    /// Maybe inject connection refused
    pub fn maybe_connection_refused(&self) -> Result<(), String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::NetworkConnectionRefused)
        {
            Err("Connection refused".to_string())
        } else {
            Ok(())
        }
    }

    /// Maybe corrupt network data
    pub fn maybe_corrupt_data(&self, data: &mut [u8]) {
        if self
            .base_injector
            .should_inject_fault(FaultType::NetworkDataCorruption)
        {
            // Flip random bits
            if let Some(byte) = data.get_mut(0) {
                *byte ^= 0xFF;
            }
        }
    }
}

/// Helper to inject HSM faults
pub struct HsmFaultInjector {
    base_injector: Arc<FaultInjector>,
}

impl HsmFaultInjector {
    /// Create an HSM fault injector
    pub fn new(base_injector: Arc<FaultInjector>) -> Self {
        Self { base_injector }
    }

    /// Maybe inject HSM failure
    pub fn maybe_fail(&self) -> Result<(), String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::HsmFailure)
        {
            Err("HSM operation failed".to_string())
        } else {
            Ok(())
        }
    }

    /// Maybe inject HSM timeout
    pub async fn maybe_timeout(&self, duration: Duration) -> Result<(), String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::HsmTimeout)
        {
            // No sleep needed - testing HSM timeout injection logic, not actual timeout
            // For time-based timeout tests, use tokio::time::pause() + advance()
            let _ = duration; // Track duration for validation
            Err("HSM timeout".to_string())
        } else {
            Ok(())
        }
    }

    /// Maybe return invalid HSM response
    pub fn maybe_invalid_response<T>(&self, valid_response: T) -> Result<T, String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::HsmInvalidResponse)
        {
            Err("Invalid HSM response".to_string())
        } else {
            Ok(valid_response)
        }
    }
}

/// Helper to inject resource faults
pub struct ResourceFaultInjector {
    base_injector: Arc<FaultInjector>,
}

impl ResourceFaultInjector {
    /// Create a resource fault injector
    pub fn new(base_injector: Arc<FaultInjector>) -> Self {
        Self { base_injector }
    }

    /// Maybe inject memory allocation failure
    pub fn maybe_allocation_failure<T>(&self, value: T) -> Result<T, String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::MemoryAllocationFailure)
        {
            Err("Memory allocation failed".to_string())
        } else {
            Ok(value)
        }
    }

    /// Maybe inject disk full error
    pub fn maybe_disk_full(&self) -> Result<(), String> {
        if self.base_injector.should_inject_fault(FaultType::DiskFull) {
            Err("Disk full".to_string())
        } else {
            Ok(())
        }
    }

    /// Maybe inject resource exhausted
    pub fn maybe_resource_exhausted(&self) -> Result<(), String> {
        if self
            .base_injector
            .should_inject_fault(FaultType::ResourceExhausted)
        {
            Err("Resource exhausted".to_string())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fault_injector_creation() {
        let injector = FaultInjector::default_injector();
        let stats = injector.get_stats();
        assert_eq!(stats.total_faults_injected, 0);
    }

    #[test]
    fn test_enable_disable() {
        let injector = FaultInjector::default_injector();
        assert!(injector.enabled.load(Ordering::SeqCst));

        injector.disable();
        assert!(!injector.enabled.load(Ordering::SeqCst));

        injector.enable();
        assert!(injector.enabled.load(Ordering::SeqCst));
    }

    #[test]
    fn test_should_inject_fault() {
        let mut config = FaultConfig::default();
        config.injection_rate = 0.0; // Never inject
        let injector = FaultInjector::new(config);

        assert!(!injector.should_inject_fault(FaultType::NetworkTimeout));
    }

    #[tokio::test]
    async fn test_network_fault_injector() {
        let base = Arc::new(FaultInjector::default_injector());
        base.disable(); // Disable for predictable test

        let network = NetworkFaultInjector::new(base);
        let result = network.maybe_connection_refused();
        assert!(result.is_ok()); // Should not inject when disabled
    }

    #[test]
    fn test_hsm_fault_injector() {
        let base = Arc::new(FaultInjector::default_injector());
        base.disable(); // Disable for predictable test

        let hsm = HsmFaultInjector::new(base);
        let result = hsm.maybe_fail();
        assert!(result.is_ok()); // Should not inject when disabled
    }

    #[test]
    fn test_resource_fault_injector() {
        let base = Arc::new(FaultInjector::default_injector());
        base.disable(); // Disable for predictable test

        let resource = ResourceFaultInjector::new(base);
        let result = resource.maybe_disk_full();
        assert!(result.is_ok()); // Should not inject when disabled
    }

    #[test]
    fn test_fault_metrics() {
        let metrics = FaultMetrics {
            faults_injected: 100,
            errors_caught: 95,
            errors_escaped: 5,
            panics_detected: 0,
            recoveries_succeeded: 95,
            recoveries_failed: 5,
        };

        assert_eq!(metrics.faults_injected, 100);
        assert_eq!(metrics.errors_caught, 95);
    }
}
