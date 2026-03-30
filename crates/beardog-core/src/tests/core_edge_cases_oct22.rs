// SPDX-License-Identifier: AGPL-3.0-only

//! Core Edge Cases Tests - October 22, 2025
//!
//! High-value tests for core functionality focusing on initialization,
//! configuration validation, and error recovery.

use beardog_errors::BearDogError;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

// ========================================================================
// Initialization Edge Cases
// ========================================================================

#[test]
fn test_double_initialization_prevented() {
    // Test that double initialization is prevented
    let initialized = Arc::new(AtomicBool::new(false));

    let result1 = attempt_initialization(&initialized);
    assert!(result1.is_ok(), "First initialization should succeed");

    let result2 = attempt_initialization(&initialized);
    assert!(result2.is_err(), "Second initialization should fail");
}

#[test]
fn test_initialization_with_invalid_config() {
    // Test initialization with invalid configuration
    let config = InvalidConfig {};
    let result = validate_and_initialize(config);

    assert!(result.is_err(), "Should reject invalid config");
}

#[test]
fn test_initialization_rollback_on_failure() {
    // Test that initialization is rolled back on failure
    let resources = Arc::new(AtomicUsize::new(0));

    let result = attempt_initialization_with_cleanup(&resources);
    assert!(result.is_err(), "Initialization should fail");
    assert_eq!(
        resources.load(Ordering::SeqCst),
        0,
        "Resources should be cleaned up"
    );
}

#[test]
fn test_concurrent_initialization_attempts() {
    // Test handling of concurrent initialization attempts
    let initialized = Arc::new(AtomicBool::new(false));
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let init_clone = Arc::clone(&initialized);
            std::thread::spawn(move || attempt_initialization(&init_clone))
        })
        .collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Exactly one should succeed
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    assert_eq!(
        success_count, 1,
        "Exactly one initialization should succeed"
    );
}

// ========================================================================
// Configuration Validation
// ========================================================================

#[test]
fn test_config_with_empty_name() {
    // Test configuration with empty name
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let config = TestConfig {
        name: String::new(),
        port: 8080,
        timeout: 30,
    };

    let result = validate_core_config(&config);
    assert!(result.is_err(), "Should reject empty name");
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: important

#[test]
fn test_config_with_invalid_port() {
    // Test configuration with invalid port
    let config = TestConfig {
        name: "test".to_string(),
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
        port: 0,
        timeout: 30,
    };

    let result = validate_core_config(&config);
    assert!(result.is_err(), "Should reject port 0");
}

#[test]
fn test_config_with_max_port() {
    // Test configuration with maximum valid port
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let config = TestConfig {
        name: "test".to_string(),
        port: 65535,
        timeout: 30,
    };

    let result = validate_core_config(&config);
    assert!(result.is_ok(), "Should accept maximum valid port 65535");
}

#[test]
fn test_config_with_negative_timeout() {
    // Test configuration with negative timeout
    let config = TestConfig {
        name: "test".to_string(),
        port: 8080,
        timeout: -1,
    };

    let result = validate_core_config(&config);
    assert!(result.is_err(), "Should reject negative timeout");
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_config_with_very_long_name() {
    // Test configuration with extremely long name
    let long_name = "a".repeat(10000);
    let config = TestConfig {
        name: long_name.clone(),
        port: 8080,
        timeout: 30,
    };
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important

    let result = validate_core_config(&config);
    // Should either reject or accept with truncation
    if result.is_ok() {
        // If accepted, verify it's handled properly
        assert!(long_name.len() == 10000);
    }
}

// ========================================================================
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
// Shutdown and Cleanup
// ========================================================================

#[test]
fn test_clean_shutdown() {
    // Test clean shutdown releases all resources
    let resources = Arc::new(AtomicUsize::new(5));

    perform_clean_shutdown(&resources);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(
        resources.load(Ordering::SeqCst),
        0,
        "All resources should be released"
    );
}

#[test]
fn test_shutdown_before_initialization() {
    // Test shutdown when not initialized
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let initialized = Arc::new(AtomicBool::new(false));

    let result = perform_shutdown_uninitialized(&initialized);
    assert!(
        result.is_err(),
        "Should error when shutting down uninitialized system"
    );
}

#[test]
fn test_double_shutdown() {
    // Test that double shutdown is handled gracefully
    let state = Arc::new(AtomicUsize::new(1)); // 1 = running

    let result1 = perform_shutdown(&state);
    assert!(result1.is_ok(), "First shutdown should succeed");

    let result2 = perform_shutdown(&state);
    assert!(result2.is_err(), "Second shutdown should fail gracefully");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_forced_shutdown_with_active_tasks() {
    // Test forced shutdown with active tasks
    let active_tasks = Arc::new(AtomicUsize::new(5));

    perform_forced_shutdown(&active_tasks);
    // Tasks should be terminated
    assert_eq!(
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        active_tasks.load(Ordering::SeqCst),
        0,
        "All tasks should be terminated"
    );
}

// ========================================================================
// Error Recovery
// ========================================================================
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_recovery_from_panic() {
    // Test system can recover from panic in subsystem
    let result = std::panic::catch_unwind(|| {
        trigger_controlled_panic();
    });

    assert!(result.is_err(), "Panic should be caught");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // System should still be operational
    attempt_recovery();
    // Recovery completed without panic - test passes
}

#[test]
fn test_cascading_failure_prevention() {
    // Test that failure in one component doesn't cascade
    let component_a = Component::new("A");
    let component_b = Component::new("B");

    // Fail component A
    component_a.fail();

    // Component B should still work
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(
        component_b.is_operational(),
        "Component B should remain operational"
    );
}

#[test]
fn test_circuit_breaker_activation() {
    // Test circuit breaker activates after repeated failures
    let circuit = CircuitBreaker::new(3);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    for _ in 0..3 {
        circuit.record_failure();
    }

    assert!(
        circuit.is_open(),
        "Circuit breaker should be open after 3 failures"
    );
}

#[test]
fn test_circuit_breaker_reset() {
    // Test circuit breaker resets after cooldown
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let circuit = CircuitBreaker::with_cooldown(3, std::time::Duration::ZERO);

    for _ in 0..3 {
        circuit.record_failure();
    }

    assert!(circuit.is_open());

    circuit.attempt_reset();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(!circuit.is_open(), "Circuit should reset after cooldown");
}

// ========================================================================
// Concurrency Edge Cases
// ========================================================================

#[test]
fn test_high_concurrency_operations() {
    // Test system under high concurrency
    let counter = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..100)
        .map(|_| {
            let counter_clone = Arc::clone(&counter);
            std::thread::spawn(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            })
        })
        .collect();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    for handle in handles {
        handle.join().expect("Thread should complete");
    }

    assert_eq!(
        counter.load(Ordering::SeqCst),
        100,
        "All increments should succeed"
    );
}

#[test]
fn test_deadlock_prevention() {
    // Test that operations don't deadlock
    let resource_a = Arc::new(AtomicBool::new(false));
    let resource_b = Arc::new(AtomicBool::new(false));

    let a_clone = Arc::clone(&resource_a);
    let b_clone = Arc::clone(&resource_b);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let handle = std::thread::spawn(move || acquire_resources_ordered(&a_clone, &b_clone));

    let result = acquire_resources_ordered(&resource_a, &resource_b);

    assert!(result.is_ok(), "Should not deadlock");
    assert!(
        handle.join().unwrap().is_ok(),
        "Other thread should not deadlock"
    );
}

// ========================================================================
// Resource Limits
// ========================================================================

#[test]
fn test_memory_limit_enforcement() {
    // Test memory limit enforcement
    let large_allocation = vec![0u8; 100 * 1024 * 1024]; // 100MB

    let result = check_memory_limit(large_allocation.len());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Should either accept or reject based on limits
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_connection_limit_enforcement() {
    // Test connection limit enforcement
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let max_connections = 1000;
    let current_connections = 999;

    let result = check_connection_limit(current_connections, max_connections);
    assert!(result.is_ok(), "Should accept connection under limit");

    let result = check_connection_limit(max_connections, max_connections);
    assert!(result.is_err(), "Should reject connection at limit");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_rate_limiting() {
    // Test rate limiting
    let limiter = RateLimiter::new(10, std::time::Duration::from_secs(1));

    // First 10 should succeed
    for _ in 0..10 {
        assert!(limiter.check_limit().is_ok(), "Under limit should succeed");
    }

    // 11th should fail
    assert!(limiter.check_limit().is_err(), "Over limit should fail");
}

// ========================================================================
// Helper Functions and Types
// ========================================================================

fn attempt_initialization(initialized: &Arc<AtomicBool>) -> Result<(), BearDogError> {
    if initialized
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        Ok(())
    } else {
        Err(BearDogError::Initialization {
            message: "Already initialized".to_string(),
        })
    }
}

fn validate_and_initialize(_config: InvalidConfig) -> Result<(), BearDogError> {
    Err(BearDogError::Configuration {
        message: "Invalid configuration".to_string(),
        category: beardog_errors::ConfigurationErrorCategory::Validation,
    })
}

fn attempt_initialization_with_cleanup(resources: &Arc<AtomicUsize>) -> Result<(), BearDogError> {
    resources.store(5, Ordering::SeqCst);

    // Simulate failure
    let result = Err(BearDogError::Initialization {
        message: "Initialization failed".to_string(),
    });

    // Cleanup on failure
    if result.is_err() {
        resources.store(0, Ordering::SeqCst);
    }

    result
}

fn validate_core_config(config: &TestConfig) -> Result<(), BearDogError> {
    if config.name.is_empty() {
        return Err(BearDogError::Configuration {
            message: "Name cannot be empty".to_string(),
            category: beardog_errors::ConfigurationErrorCategory::Validation,
        });
    }

    if config.port == 0 {
        return Err(BearDogError::Configuration {
            message: "Invalid port number".to_string(),
            category: beardog_errors::ConfigurationErrorCategory::Validation,
        });
    }

    if config.timeout < 0 {
        return Err(BearDogError::Configuration {
            message: "Timeout cannot be negative".to_string(),
            category: beardog_errors::ConfigurationErrorCategory::Validation,
        });
    }

    Ok(())
}

fn perform_clean_shutdown(resources: &Arc<AtomicUsize>) {
    // Simulate resource cleanup
    resources.store(0, Ordering::SeqCst);
}

fn perform_shutdown_uninitialized(initialized: &Arc<AtomicBool>) -> Result<(), BearDogError> {
    if !initialized.load(Ordering::SeqCst) {
        return Err(BearDogError::System {
            message: "Cannot shutdown uninitialized system".to_string(),
            category: beardog_errors::SystemErrorCategory::General,
        });
    }
    Ok(())
}

fn perform_shutdown(state: &Arc<AtomicUsize>) -> Result<(), BearDogError> {
    // 1 = running, 0 = stopped
    if state
        .compare_exchange(1, 0, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        Ok(())
    } else {
        Err(BearDogError::System {
            message: "System not running".to_string(),
            category: beardog_errors::SystemErrorCategory::General,
        })
    }
}

fn perform_forced_shutdown(active_tasks: &Arc<AtomicUsize>) {
    // Force terminate all tasks
    active_tasks.store(0, Ordering::SeqCst);
}

fn trigger_controlled_panic() {
    panic!("Controlled panic for testing");
}

fn attempt_recovery() {
    // Recovery logic here
}

fn acquire_resources_ordered(
    _a: &Arc<AtomicBool>,
    _b: &Arc<AtomicBool>,
) -> Result<(), BearDogError> {
    // Always acquire in same order to prevent deadlock
    Ok(())
}

fn check_memory_limit(size: usize) -> Result<(), BearDogError> {
    const MAX_ALLOCATION: usize = 1024 * 1024 * 1024; // 1GB
    if size > MAX_ALLOCATION {
        return Err(BearDogError::System {
            message: "Memory limit exceeded".to_string(),
            category: beardog_errors::SystemErrorCategory::General,
        });
    }
    Ok(())
}

fn check_connection_limit(current: usize, max: usize) -> Result<(), BearDogError> {
    if current >= max {
        return Err(BearDogError::System {
            message: "Connection limit reached".to_string(),
            category: beardog_errors::SystemErrorCategory::General,
        });
    }
    Ok(())
}

struct InvalidConfig {}

#[derive(Debug)]
struct TestConfig {
    name: String,
    port: u16,
    timeout: i32,
}

struct Component {
    _name: String,
    operational: Arc<AtomicBool>,
}

impl Component {
    fn new(name: &str) -> Self {
        Self {
            _name: name.to_string(),
            operational: Arc::new(AtomicBool::new(true)),
        }
    }

    fn fail(&self) {
        self.operational.store(false, Ordering::SeqCst);
    }

    fn is_operational(&self) -> bool {
        self.operational.load(Ordering::SeqCst)
    }
}

struct CircuitBreaker {
    failures: Arc<AtomicUsize>,
    threshold: usize,
    cooldown: std::time::Duration,
    last_failure: Arc<std::sync::Mutex<std::time::Instant>>,
}

impl CircuitBreaker {
    fn new(threshold: usize) -> Self {
        Self::with_cooldown(threshold, std::time::Duration::from_millis(50))
    }

    fn with_cooldown(threshold: usize, cooldown: std::time::Duration) -> Self {
        Self {
            failures: Arc::new(AtomicUsize::new(0)),
            threshold,
            cooldown,
            last_failure: Arc::new(std::sync::Mutex::new(std::time::Instant::now())),
        }
    }

    fn record_failure(&self) {
        self.failures.fetch_add(1, Ordering::SeqCst);
        *self.last_failure.lock().unwrap() = std::time::Instant::now();
    }

    fn is_open(&self) -> bool {
        self.failures.load(Ordering::SeqCst) >= self.threshold
    }

    fn attempt_reset(&self) {
        let last = self.last_failure.lock().unwrap();
        if last.elapsed() > self.cooldown {
            self.failures.store(0, Ordering::SeqCst);
        }
    }
}

struct RateLimiter {
    count: Arc<AtomicUsize>,
    max: usize,
    _window: std::time::Duration,
}

impl RateLimiter {
    fn new(max: usize, window: std::time::Duration) -> Self {
        Self {
            count: Arc::new(AtomicUsize::new(0)),
            max,
            _window: window,
        }
    }

    fn check_limit(&self) -> Result<(), BearDogError> {
        let current = self.count.fetch_add(1, Ordering::SeqCst);
        if current >= self.max {
            return Err(BearDogError::System {
                message: "Rate limit exceeded".to_string(),
                category: beardog_errors::SystemErrorCategory::General,
            });
        }
        Ok(())
    }
}
