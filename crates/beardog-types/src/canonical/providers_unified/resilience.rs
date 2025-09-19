// Resilience Configuration
//
// Provider resilience patterns including retry logic, circuit breakers, and fault tolerance.

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
