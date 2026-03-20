// SPDX-License-Identifier: AGPL-3.0-only

// Resilience Configuration
//
// Provider resilience patterns including retry logic, circuit breakers, and fault tolerance.

// Allow pedantic clippy lints for intentional type conversions
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

use crate::canonical::traits::{RetryStrategy, TimeoutPolicy};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Resilience configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResilienceConfig {
    /// Retry configuration
    /// The retry value
    pub retry: RetryConfig,

    /// Circuit breaker configuration
    /// The circuit breaker value
    pub circuit_breaker: CircuitBreakerConfig,

    /// Timeout configuration
    pub timeout: TimeoutConfig,

    /// Bulkhead configuration
    /// The bulkhead value
    pub bulkhead: BulkheadConfig,

    /// Fallback configuration
    /// The fallback value
    pub fallback: FallbackConfig,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Retry enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Maximum retry attempts
    /// Number of `max_attempts`
    pub max_attempts: u32,

    /// Initial retry delay
    /// The initial delay value
    pub initial_delay: Duration,

    /// Maximum retry delay
    /// The max delay value
    pub max_delay: Duration,

    /// Backoff strategy
    /// The backoff strategy value
    pub backoff_strategy: BackoffStrategy,

    /// Retry on specific errors
    /// Collection of retry on errors
    pub retry_on_errors: Vec<String>,

    /// Jitter enabled
    /// Whether jitter is enabled
    pub jitter_enabled: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_strategy: BackoffStrategy::Exponential,
            retry_on_errors: vec!["timeout ".to_string(), "connection_error".to_string()],
            jitter_enabled: true,
        }
    }
}

/// Backoff strategies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackoffStrategy {
    /// Fixed variant
    Fixed,
    /// Linear variant
    Linear,
    /// Exponential variant
    Exponential,
    /// Custom retry strategy
    Custom(String),
}

// Implement RetryStrategy trait for RetryConfig
impl RetryStrategy for RetryConfig {
    fn max_attempts(&self) -> u32 {
        if self.enabled {
            self.max_attempts.max(1) // Ensure at least 1 attempt if enabled
        } else {
            0 // No retries if disabled
        }
    }

    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::ZERO;
        }

        let base_delay = match self.backoff_strategy {
            BackoffStrategy::Fixed => self.initial_delay,
            BackoffStrategy::Linear => {
                let delay_ms = (self.initial_delay.as_millis() as u64) * (attempt as u64);
                Duration::from_millis(delay_ms)
            }
            BackoffStrategy::Exponential => {
                let delay_ms = (self.initial_delay.as_millis() as f64
                    * 2.0_f64.powi((attempt - 1) as i32)) as u64;
                Duration::from_millis(delay_ms)
            }
            BackoffStrategy::Custom(_) => {
                // For custom strategies, use exponential as fallback
                let delay_ms = (self.initial_delay.as_millis() as f64
                    * 2.0_f64.powi((attempt - 1) as i32)) as u64;
                Duration::from_millis(delay_ms)
            }
        };

        // Apply jitter if enabled
        // Note: Jitter implementation requires random number generation
        // For now, we use a simplified approach without external dependencies
        let delay_with_jitter = if self.jitter_enabled {
            // Simple jitter: add 10% variation based on attempt number
            let jitter_percent = (attempt % 10) as f64 * 0.01; // 0-10% variation
            let jitter_factor = 0.95 + jitter_percent; // 95-105% of delay
            Duration::from_millis((base_delay.as_millis() as f64 * jitter_factor) as u64)
        } else {
            base_delay
        };

        // Cap at max_delay
        delay_with_jitter.min(self.max_delay)
    }

    fn backoff_multiplier(&self) -> f64 {
        match self.backoff_strategy {
            BackoffStrategy::Exponential => 2.0,
            BackoffStrategy::Linear => 1.0,
            BackoffStrategy::Fixed => 1.0,
            BackoffStrategy::Custom(_) => 2.0, // Default for custom
        }
    }
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Circuit breaker enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Failure threshold
    /// Number of `failure_threshold`
    pub failure_threshold: u32,

    /// Success threshold
    /// Number of `success_threshold`
    pub success_threshold: u32,

    /// Timeout duration
    pub timeout: Duration,

    /// Half-open max calls
    /// Number of `half_open_max_calls`
    pub half_open_max_calls: u32,

    /// Minimum throughput
    /// Number of `minimum_throughput`
    pub minimum_throughput: u32,

    /// Sliding window size
    pub sliding_window_size: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            half_open_max_calls: 10,
            minimum_throughput: 10,
            sliding_window_size: 100,
        }
    }
}

/// Timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// Timeout enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Default timeout
    pub default_timeout: Duration,

    /// Operation-specific timeouts
    pub operation_timeouts: std::collections::HashMap<String, Duration>,

    /// Timeout escalation enabled
    /// Whether escalation is enabled
    pub escalation_enabled: bool,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_timeout: Duration::from_secs(30),
            operation_timeouts: std::collections::HashMap::new(),
            escalation_enabled: false,
        }
    }
}

// Implement TimeoutPolicy trait for provider resilience timeout configuration
impl TimeoutPolicy for TimeoutConfig {
    fn connection_timeout(&self) -> Duration {
        if !self.enabled {
            return Duration::MAX; // Effectively no timeout when disabled
        }
        self.operation_timeouts
            .get("connection")
            .or_else(|| self.operation_timeouts.get("connect"))
            .copied()
            .unwrap_or(self.default_timeout)
    }

    fn operation_timeout(&self, operation: &str) -> Duration {
        if !self.enabled {
            return Duration::MAX; // Effectively no timeout when disabled
        }
        self.operation_timeouts
            .get(operation)
            .copied()
            .unwrap_or(self.default_timeout)
    }

    fn should_timeout(&self, elapsed: Duration, operation: &str) -> bool {
        if !self.enabled {
            return false; // Never timeout when disabled
        }
        elapsed >= self.operation_timeout(operation)
    }

    fn global_timeout(&self) -> Option<Duration> {
        if !self.enabled {
            return None;
        }
        Some(self.default_timeout)
    }

    fn read_timeout(&self) -> Duration {
        if !self.enabled {
            return Duration::MAX;
        }
        self.operation_timeouts
            .get("read")
            .copied()
            .unwrap_or(self.default_timeout)
    }

    fn write_timeout(&self) -> Duration {
        if !self.enabled {
            return Duration::MAX;
        }
        self.operation_timeouts
            .get("write")
            .copied()
            .unwrap_or(self.default_timeout)
    }

    fn idle_timeout(&self) -> Option<Duration> {
        if !self.enabled {
            return None;
        }
        self.operation_timeouts.get("idle").copied()
    }

    fn remaining_time(&self, elapsed: Duration, operation: &str) -> Duration {
        if !self.enabled {
            return Duration::MAX;
        }
        let timeout = self.operation_timeout(operation);
        timeout.saturating_sub(elapsed)
    }

    fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(()); // Always valid when disabled
        }
        if self.default_timeout.is_zero() {
            return Err("Default timeout cannot be zero when enabled".to_string());
        }
        for (op, timeout) in &self.operation_timeouts {
            if timeout.is_zero() {
                return Err(format!("Timeout for operation '{op}' cannot be zero"));
            }
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        if !self.enabled {
            return true; // Disabled is valid for production
        }
        self.default_timeout >= Duration::from_secs(1)
            && self.default_timeout <= Duration::from_secs(300)
            && self.validate().is_ok()
    }
}

/// Bulkhead configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkheadConfig {
    /// Bulkhead enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Maximum concurrent calls
    /// Number of `max_concurrent_calls`
    pub max_concurrent_calls: u32,

    /// Maximum wait duration
    /// The max wait duration value
    pub max_wait_duration: Duration,

    /// Isolation strategy
    /// The isolation strategy value
    pub isolation_strategy: IsolationStrategy,
}

impl Default for BulkheadConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_concurrent_calls: 100,
            max_wait_duration: Duration::from_secs(10),
            isolation_strategy: IsolationStrategy::Semaphore,
        }
    }
}

/// Isolation strategies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsolationStrategy {
    /// Semaphore variant
    Semaphore,
    /// `ThreadPool` variant
    ThreadPool,
}

/// Fallback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackConfig {
    /// Fallback enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Fallback strategy
    /// The strategy value
    pub strategy: FallbackStrategy,

    /// Fallback value
    /// Optional fallback value
    pub fallback_value: Option<serde_json::Value>,

    /// Fallback function
    /// Optional fallback function
    pub fallback_function: Option<String>,
}

impl Default for FallbackConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: FallbackStrategy::DefaultValue,
            fallback_value: None,
            fallback_function: None,
        }
    }
}

/// Fallback strategies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FallbackStrategy {
    /// `DefaultValue` variant
    DefaultValue,
    /// Function variant
    Function,
    /// Cache variant
    Cache,
    /// Alternative variant
    Alternative,
}
