//! Concurrent-Safe Timeout Configuration Module
//!
//! This module provides centralized timeout configuration for all BearDog operations,
//! eliminating hardcoded timeout values throughout the codebase.
//!
//! ## Design Pattern: Explicit Environment Loading
//!
//! To ensure **concurrent safety** and **testability**, this module separates:
//! - **Static defaults** (`Default` trait) - Pure, no environment reads
//! - **Environment loading** (`from_env()`) - Explicit environment variable reads
//! - **Flexible construction** (`builder()`) - Testing without env var pollution
//!
//! ## Configuration Hierarchy
//!
//! Timeouts are resolved in the following priority order:
//! 1. Environment variables (when using `from_env()` or `builder().from_env()`)
//! 2. Configuration file values (when loaded via `BearDogConfig::from_file()`)
//! 3. Explicit values (when using `builder().timeout_secs(n)`)
//! 4. Sensible defaults (always available via `Default`)
//!
//! ## Environment Variables
//!
//! - `BEARDOG_HEALTH_CHECK_TIMEOUT_SECS` - Health check timeout (default: 5s)
//! - `BEARDOG_HSM_OPERATION_TIMEOUT_SECS` - HSM operation timeout (default: 2s)
//! - `BEARDOG_HSM_PROBE_TIMEOUT_MILLIS` - HSM probe timeout (default: 500ms)
//! - `BEARDOG_DISCOVERY_TIMEOUT_SECS` - Service discovery timeout (default: 10s)
//! - `BEARDOG_DECISION_TIMEOUT_SECS` - AI decision timeout (default: 30s)
//! - `BEARDOG_AI_REQUEST_TIMEOUT_SECS` - AI request timeout (default: 30s)
//! - `BEARDOG_AI_BATCH_TIMEOUT_MS` - AI batch timeout (default: 10ms)
//! - `BEARDOG_POOL_IDLE_TIMEOUT_SECS` - Pool idle timeout (default: 300s)
//! - `BEARDOG_MAX_CONNECTION_AGE_SECS` - Max connection age (default: 3600s)
//!
//! ## Example Usage
//!
//! ```rust
//! use beardog_config::domains::timeouts::TimeoutConfig;
//!
//! // Static defaults (no env reads, concurrent-safe)
//! let config = TimeoutConfig::default();
//! assert_eq!(config.health_check_secs, 5);
//!
//! // Explicit environment loading
//! let env_config = TimeoutConfig::from_env();
//!
//! // Builder for testing (no env pollution)
//! let test_config = TimeoutConfig::builder()
//!     .health_check_secs(10)
//!     .build();
//! ```

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Timeout configuration for all BearDog operations
///
/// All timeout values can be overridden via environment variables or configuration file.
/// Each timeout has a sensible default based on industry standards and operational experience.
///
/// # Concurrent Safety
///
/// This struct is designed for concurrent access:
/// - `Default` implementation is pure (no env var reads)
/// - Use `from_env()` to explicitly load from environment
/// - Use `builder()` for test-friendly construction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct TimeoutConfig {
    /// Health check timeout in seconds
    ///
    /// Default: 5 seconds (industry standard for health checks)
    /// Environment: `BEARDOG_HEALTH_CHECK_TIMEOUT_SECS`
    pub health_check_secs: u64,

    /// HSM operation timeout in seconds
    ///
    /// Default: 2 seconds (typical HSM operation latency)
    /// Environment: `BEARDOG_HSM_OPERATION_TIMEOUT_SECS`
    pub hsm_operation_secs: u64,

    /// HSM probe timeout in milliseconds
    ///
    /// Default: 500 milliseconds (fast probe for availability check)
    /// Environment: `BEARDOG_HSM_PROBE_TIMEOUT_MILLIS`
    pub hsm_probe_millis: u64,

    /// Service discovery operation timeout in seconds
    ///
    /// Default: 10 seconds (allows for network latency)
    /// Environment: `BEARDOG_DISCOVERY_TIMEOUT_SECS`
    pub discovery_operation_secs: u64,

    /// AI decision timeout in seconds
    ///
    /// Default: 30 seconds (allows for complex AI decisions)
    /// Environment: `BEARDOG_DECISION_TIMEOUT_SECS`
    pub ai_decision_secs: u64,

    /// AI request timeout in seconds
    ///
    /// Default: 30 seconds (inference operations)
    /// Environment: `BEARDOG_AI_REQUEST_TIMEOUT_SECS`
    pub ai_request_timeout_secs: u64,

    /// AI batch timeout in milliseconds
    ///
    /// Default: 10 milliseconds (fast batching)
    /// Environment: `BEARDOG_AI_BATCH_TIMEOUT_MS`
    pub ai_batch_timeout_millis: u64,

    /// Pool idle timeout in seconds
    ///
    /// Default: 300 seconds (5 minutes)
    /// Environment: `BEARDOG_POOL_IDLE_TIMEOUT_SECS`
    pub pool_idle_timeout_secs: u64,

    /// Maximum connection age in seconds
    ///
    /// Default: 3600 seconds (1 hour)
    /// Environment: `BEARDOG_MAX_CONNECTION_AGE_SECS`
    pub max_connection_age_secs: u64,

    // ═══════════════════════════════════════════════════════════════════
    // Network Timeouts (Phase 3 - November 21, 2025)
    // ═══════════════════════════════════════════════════════════════════
    /// Connection establishment timeout in seconds
    ///
    /// Default: 30 seconds
    /// Environment: `BEARDOG_CONNECTION_TIMEOUT_SECS`
    pub connection_timeout_secs: u64,

    /// Network handshake timeout in seconds
    ///
    /// Default: 10 seconds
    /// Environment: `BEARDOG_HANDSHAKE_TIMEOUT_SECS`
    pub handshake_timeout_secs: u64,

    /// TLS handshake timeout in seconds
    ///
    /// Default: 30 seconds
    /// Environment: `BEARDOG_TLS_HANDSHAKE_TIMEOUT_SECS`
    pub tls_handshake_timeout_secs: u64,

    /// Keep-alive timeout in seconds
    ///
    /// Default: 60 seconds
    /// Environment: `BEARDOG_KEEP_ALIVE_TIMEOUT_SECS`
    pub keep_alive_timeout_secs: u64,

    /// Idle connection timeout in seconds
    ///
    /// Default: 300 seconds (5 minutes)
    /// Environment: `BEARDOG_IDLE_CONNECTION_TIMEOUT_SECS`
    pub idle_connection_timeout_secs: u64,

    /// Read operation timeout in seconds
    ///
    /// Default: 60 seconds
    /// Environment: `BEARDOG_READ_TIMEOUT_SECS`
    pub read_timeout_secs: u64,

    /// Write operation timeout in seconds
    ///
    /// Default: 30 seconds
    /// Environment: `BEARDOG_WRITE_TIMEOUT_SECS`
    pub write_timeout_secs: u64,

    /// HTTP request timeout in seconds
    ///
    /// Default: 30 seconds
    /// Environment: `BEARDOG_HTTP_REQUEST_TIMEOUT_SECS`
    pub http_request_timeout_secs: u64,

    /// HTTP response timeout in seconds
    ///
    /// Default: 30 seconds
    /// Environment: `BEARDOG_HTTP_RESPONSE_TIMEOUT_SECS`
    pub http_response_timeout_secs: u64,

    /// DNS resolution timeout in seconds
    ///
    /// Default: 5 seconds
    /// Environment: `BEARDOG_DNS_RESOLUTION_TIMEOUT_SECS`
    pub dns_resolution_timeout_secs: u64,

    /// Retry timeout in milliseconds
    ///
    /// Default: 100 milliseconds
    /// Environment: `BEARDOG_RETRY_TIMEOUT_MILLIS`
    pub retry_timeout_millis: u64,

    /// Backoff timeout in milliseconds
    ///
    /// Default: 500 milliseconds
    /// Environment: `BEARDOG_BACKOFF_TIMEOUT_MILLIS`
    pub backoff_timeout_millis: u64,

    /// Ping timeout in seconds
    ///
    /// Default: 1 second
    /// Environment: `BEARDOG_PING_TIMEOUT_SECS`
    pub ping_timeout_secs: u64,

    /// Heartbeat timeout in seconds
    ///
    /// Default: 30 seconds
    /// Environment: `BEARDOG_HEARTBEAT_TIMEOUT_SECS`
    pub heartbeat_timeout_secs: u64,
}

impl TimeoutConfig {
    /// Pure static defaults (no environment variable reads)
    ///
    /// This method returns compile-time constants and is safe for
    /// concurrent access without any risk of race conditions.
    ///
    /// # Examples
    ///
    /// ```
    /// use beardog_config::domains::timeouts::TimeoutConfig;
    ///
    /// let defaults = TimeoutConfig::const_defaults();
    /// assert_eq!(defaults.health_check_secs, 5);
    /// ```
    pub const fn const_defaults() -> Self {
        Self {
            health_check_secs: 5,
            hsm_operation_secs: 2,
            hsm_probe_millis: 500,
            discovery_operation_secs: 10,
            ai_decision_secs: 30,
            ai_request_timeout_secs: 30,
            ai_batch_timeout_millis: 10,
            pool_idle_timeout_secs: 300,
            max_connection_age_secs: 3600,
            // Network timeouts (Phase 3)
            connection_timeout_secs: 30,
            handshake_timeout_secs: 10,
            tls_handshake_timeout_secs: 30,
            keep_alive_timeout_secs: 60,
            idle_connection_timeout_secs: 300,
            read_timeout_secs: 60,
            write_timeout_secs: 30,
            http_request_timeout_secs: 30,
            http_response_timeout_secs: 30,
            dns_resolution_timeout_secs: 5,
            retry_timeout_millis: 100,
            backoff_timeout_millis: 500,
            ping_timeout_secs: 1,
            heartbeat_timeout_secs: 30,
        }
    }

    /// Load configuration from environment variables with fallback to defaults
    ///
    /// This method explicitly reads from environment variables. It is the
    /// **only** method that performs environment variable access, making
    /// the side effects clear and testable.
    ///
    /// # Examples
    ///
    /// ```
    /// use beardog_config::domains::timeouts::TimeoutConfig;
    ///
    /// // Load from environment (or use defaults if not set)
    /// let config = TimeoutConfig::from_env();
    /// ```
    pub fn from_env() -> Self {
        let defaults = Self::const_defaults();

        Self {
            health_check_secs: std::env::var("BEARDOG_HEALTH_CHECK_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.health_check_secs),

            hsm_operation_secs: std::env::var("BEARDOG_HSM_OPERATION_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.hsm_operation_secs),

            hsm_probe_millis: std::env::var("BEARDOG_HSM_PROBE_TIMEOUT_MILLIS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.hsm_probe_millis),

            discovery_operation_secs: std::env::var("BEARDOG_DISCOVERY_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.discovery_operation_secs),

            ai_decision_secs: std::env::var("BEARDOG_DECISION_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.ai_decision_secs),

            ai_request_timeout_secs: std::env::var("BEARDOG_AI_REQUEST_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.ai_request_timeout_secs),

            ai_batch_timeout_millis: std::env::var("BEARDOG_AI_BATCH_TIMEOUT_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.ai_batch_timeout_millis),

            pool_idle_timeout_secs: std::env::var("BEARDOG_POOL_IDLE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.pool_idle_timeout_secs),

            max_connection_age_secs: std::env::var("BEARDOG_MAX_CONNECTION_AGE_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.max_connection_age_secs),

            // Network timeouts (Phase 3)
            connection_timeout_secs: std::env::var("BEARDOG_CONNECTION_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.connection_timeout_secs),

            handshake_timeout_secs: std::env::var("BEARDOG_HANDSHAKE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.handshake_timeout_secs),

            tls_handshake_timeout_secs: std::env::var("BEARDOG_TLS_HANDSHAKE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.tls_handshake_timeout_secs),

            keep_alive_timeout_secs: std::env::var("BEARDOG_KEEP_ALIVE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.keep_alive_timeout_secs),

            idle_connection_timeout_secs: std::env::var("BEARDOG_IDLE_CONNECTION_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.idle_connection_timeout_secs),

            read_timeout_secs: std::env::var("BEARDOG_READ_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.read_timeout_secs),

            write_timeout_secs: std::env::var("BEARDOG_WRITE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.write_timeout_secs),

            http_request_timeout_secs: std::env::var("BEARDOG_HTTP_REQUEST_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.http_request_timeout_secs),

            http_response_timeout_secs: std::env::var("BEARDOG_HTTP_RESPONSE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.http_response_timeout_secs),

            dns_resolution_timeout_secs: std::env::var("BEARDOG_DNS_RESOLUTION_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.dns_resolution_timeout_secs),

            retry_timeout_millis: std::env::var("BEARDOG_RETRY_TIMEOUT_MILLIS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.retry_timeout_millis),

            backoff_timeout_millis: std::env::var("BEARDOG_BACKOFF_TIMEOUT_MILLIS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.backoff_timeout_millis),

            ping_timeout_secs: std::env::var("BEARDOG_PING_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.ping_timeout_secs),

            heartbeat_timeout_secs: std::env::var("BEARDOG_HEARTBEAT_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.heartbeat_timeout_secs),
        }
    }

    /// Create a builder for flexible configuration construction
    ///
    /// Builders are the recommended way to construct configurations in tests,
    /// as they don't pollute the environment with `set_var` calls.
    ///
    /// # Examples
    ///
    /// ```
    /// use beardog_config::domains::timeouts::TimeoutConfig;
    ///
    /// let config = TimeoutConfig::builder()
    ///     .health_check_secs(15)
    ///     .hsm_operation_secs(3)
    ///     .build();
    ///
    /// assert_eq!(config.health_check_secs, 15);
    /// ```
    pub fn builder() -> TimeoutConfigBuilder {
        TimeoutConfigBuilder::new()
    }

    /// Convert health check timeout to Duration
    pub fn health_check_duration(&self) -> Duration {
        Duration::from_secs(self.health_check_secs)
    }

    /// Convert HSM operation timeout to Duration
    pub fn hsm_operation_duration(&self) -> Duration {
        Duration::from_secs(self.hsm_operation_secs)
    }

    /// Convert HSM probe timeout to Duration
    pub fn hsm_probe_duration(&self) -> Duration {
        Duration::from_millis(self.hsm_probe_millis)
    }

    /// Convert discovery operation timeout to Duration
    pub fn discovery_operation_duration(&self) -> Duration {
        Duration::from_secs(self.discovery_operation_secs)
    }

    /// Convert AI decision timeout to Duration
    pub fn ai_decision_duration(&self) -> Duration {
        Duration::from_secs(self.ai_decision_secs)
    }

    /// Convert AI request timeout to Duration
    pub fn ai_request_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.ai_request_timeout_secs)
    }

    /// Convert AI batch timeout to Duration
    pub fn ai_batch_timeout_duration(&self) -> Duration {
        Duration::from_millis(self.ai_batch_timeout_millis)
    }

    /// Convert pool idle timeout to Duration
    pub fn pool_idle_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.pool_idle_timeout_secs)
    }

    /// Convert max connection age to Duration
    pub fn max_connection_age_duration(&self) -> Duration {
        Duration::from_secs(self.max_connection_age_secs)
    }

    /// Validate timeout configuration
    ///
    /// Ensures all timeout values are within sensible ranges.
    ///
    /// # Errors
    ///
    /// Returns a `String` error if any timeout value is out of range.
    pub fn validate(&self) -> Result<(), String> {
        if !(1..=30).contains(&self.health_check_secs) {
            return Err(format!(
                "Health check timeout must be between 1 and 30 seconds, got {}",
                self.health_check_secs
            ));
        }

        if !(1..=10).contains(&self.hsm_operation_secs) {
            return Err(format!(
                "HSM operation timeout must be between 1 and 10 seconds, got {}",
                self.hsm_operation_secs
            ));
        }

        if !(100..=5000).contains(&self.hsm_probe_millis) {
            return Err(format!(
                "HSM probe timeout must be between 100 and 5000 milliseconds, got {}",
                self.hsm_probe_millis
            ));
        }

        if !(1..=60).contains(&self.discovery_operation_secs) {
            return Err(format!(
                "Discovery timeout must be between 1 and 60 seconds, got {}",
                self.discovery_operation_secs
            ));
        }

        if !(5..=300).contains(&self.ai_decision_secs) {
            return Err(format!(
                "AI decision timeout must be between 5 and 300 seconds, got {}",
                self.ai_decision_secs
            ));
        }

        if !(5..=300).contains(&self.ai_request_timeout_secs) {
            return Err(format!(
                "AI request timeout must be between 5 and 300 seconds, got {}",
                self.ai_request_timeout_secs
            ));
        }

        if !(1..=1000).contains(&self.ai_batch_timeout_millis) {
            return Err(format!(
                "AI batch timeout must be between 1 and 1000 milliseconds, got {}",
                self.ai_batch_timeout_millis
            ));
        }

        if !(60..=3600).contains(&self.pool_idle_timeout_secs) {
            return Err(format!(
                "Pool idle timeout must be between 60 and 3600 seconds, got {}",
                self.pool_idle_timeout_secs
            ));
        }

        if !(300..=86400).contains(&self.max_connection_age_secs) {
            return Err(format!(
                "Max connection age must be between 300 and 86400 seconds, got {}",
                self.max_connection_age_secs
            ));
        }

        Ok(())
    }
}

/// Default uses static constants (no environment variable reads)
///
/// This ensures that `Default` is:
/// - Thread-safe (no global state access)
/// - Deterministic (always returns the same values)
/// - Fast (compile-time constants)
/// - Testable (no side effects)
impl Default for TimeoutConfig {
    fn default() -> Self {
        Self::const_defaults()
    }
}

/// Builder for flexible TimeoutConfig construction
///
/// Builders provide a clean, test-friendly way to create configurations
/// without modifying global environment variables.
#[derive(Debug, Default)]
pub struct TimeoutConfigBuilder {
    health_check_secs: Option<u64>,
    hsm_operation_secs: Option<u64>,
    hsm_probe_millis: Option<u64>,
    discovery_operation_secs: Option<u64>,
    ai_decision_secs: Option<u64>,
    ai_request_timeout_secs: Option<u64>,
    ai_batch_timeout_millis: Option<u64>,
    pool_idle_timeout_secs: Option<u64>,
    max_connection_age_secs: Option<u64>,
}

impl TimeoutConfigBuilder {
    /// Create a new builder with no values set
    pub fn new() -> Self {
        Self::default()
    }

    /// Set health check timeout in seconds
    pub fn health_check_secs(mut self, secs: u64) -> Self {
        self.health_check_secs = Some(secs);
        self
    }

    /// Set HSM operation timeout in seconds
    pub fn hsm_operation_secs(mut self, secs: u64) -> Self {
        self.hsm_operation_secs = Some(secs);
        self
    }

    /// Set HSM probe timeout in milliseconds
    pub fn hsm_probe_millis(mut self, millis: u64) -> Self {
        self.hsm_probe_millis = Some(millis);
        self
    }

    /// Set discovery operation timeout in seconds
    pub fn discovery_operation_secs(mut self, secs: u64) -> Self {
        self.discovery_operation_secs = Some(secs);
        self
    }

    /// Set AI decision timeout in seconds
    pub fn ai_decision_secs(mut self, secs: u64) -> Self {
        self.ai_decision_secs = Some(secs);
        self
    }

    /// Set AI request timeout in seconds
    pub fn ai_request_timeout_secs(mut self, secs: u64) -> Self {
        self.ai_request_timeout_secs = Some(secs);
        self
    }

    /// Set AI batch timeout in milliseconds
    pub fn ai_batch_timeout_millis(mut self, millis: u64) -> Self {
        self.ai_batch_timeout_millis = Some(millis);
        self
    }

    /// Set pool idle timeout in seconds
    pub fn pool_idle_timeout_secs(mut self, secs: u64) -> Self {
        self.pool_idle_timeout_secs = Some(secs);
        self
    }

    /// Set max connection age in seconds
    pub fn max_connection_age_secs(mut self, secs: u64) -> Self {
        self.max_connection_age_secs = Some(secs);
        self
    }

    /// Load values from environment variables (for explicitly set fields only)
    ///
    /// This method reads from environment variables but only overwrites
    /// fields that haven't been explicitly set via builder methods.
    pub fn from_env(mut self) -> Self {
        if self.health_check_secs.is_none() {
            if let Ok(val) = std::env::var("BEARDOG_HEALTH_CHECK_TIMEOUT_SECS") {
                if let Ok(secs) = val.parse() {
                    self.health_check_secs = Some(secs);
                }
            }
        }

        if self.hsm_operation_secs.is_none() {
            if let Ok(val) = std::env::var("BEARDOG_HSM_OPERATION_TIMEOUT_SECS") {
                if let Ok(secs) = val.parse() {
                    self.hsm_operation_secs = Some(secs);
                }
            }
        }

        if self.hsm_probe_millis.is_none() {
            if let Ok(val) = std::env::var("BEARDOG_HSM_PROBE_TIMEOUT_MILLIS") {
                if let Ok(millis) = val.parse() {
                    self.hsm_probe_millis = Some(millis);
                }
            }
        }

        if self.discovery_operation_secs.is_none() {
            if let Ok(val) = std::env::var("BEARDOG_DISCOVERY_TIMEOUT_SECS") {
                if let Ok(secs) = val.parse() {
                    self.discovery_operation_secs = Some(secs);
                }
            }
        }

        if self.ai_decision_secs.is_none() {
            if let Ok(val) = std::env::var("BEARDOG_DECISION_TIMEOUT_SECS") {
                if let Ok(secs) = val.parse() {
                    self.ai_decision_secs = Some(secs);
                }
            }
        }

        if self.ai_request_timeout_secs.is_none() {
            if let Ok(val) = std::env::var("BEARDOG_AI_REQUEST_TIMEOUT_SECS") {
                if let Ok(secs) = val.parse() {
                    self.ai_request_timeout_secs = Some(secs);
                }
            }
        }

        if self.ai_batch_timeout_millis.is_none() {
            if let Ok(val) = std::env::var("BEARDOG_AI_BATCH_TIMEOUT_MS") {
                if let Ok(millis) = val.parse() {
                    self.ai_batch_timeout_millis = Some(millis);
                }
            }
        }

        if self.pool_idle_timeout_secs.is_none() {
            if let Ok(val) = std::env::var("BEARDOG_POOL_IDLE_TIMEOUT_SECS") {
                if let Ok(secs) = val.parse() {
                    self.pool_idle_timeout_secs = Some(secs);
                }
            }
        }

        if self.max_connection_age_secs.is_none() {
            if let Ok(val) = std::env::var("BEARDOG_MAX_CONNECTION_AGE_SECS") {
                if let Ok(secs) = val.parse() {
                    self.max_connection_age_secs = Some(secs);
                }
            }
        }

        self
    }

    /// Build the final TimeoutConfig
    ///
    /// Unset fields will use static defaults.
    pub fn build(self) -> TimeoutConfig {
        let defaults = TimeoutConfig::const_defaults();

        TimeoutConfig {
            health_check_secs: self.health_check_secs.unwrap_or(defaults.health_check_secs),
            hsm_operation_secs: self
                .hsm_operation_secs
                .unwrap_or(defaults.hsm_operation_secs),
            hsm_probe_millis: self.hsm_probe_millis.unwrap_or(defaults.hsm_probe_millis),
            discovery_operation_secs: self
                .discovery_operation_secs
                .unwrap_or(defaults.discovery_operation_secs),
            ai_decision_secs: self.ai_decision_secs.unwrap_or(defaults.ai_decision_secs),
            ai_request_timeout_secs: self
                .ai_request_timeout_secs
                .unwrap_or(defaults.ai_request_timeout_secs),
            ai_batch_timeout_millis: self
                .ai_batch_timeout_millis
                .unwrap_or(defaults.ai_batch_timeout_millis),
            pool_idle_timeout_secs: self
                .pool_idle_timeout_secs
                .unwrap_or(defaults.pool_idle_timeout_secs),
            max_connection_age_secs: self
                .max_connection_age_secs
                .unwrap_or(defaults.max_connection_age_secs),
            // Network timeouts (Phase 3) - use defaults
            connection_timeout_secs: defaults.connection_timeout_secs,
            handshake_timeout_secs: defaults.handshake_timeout_secs,
            tls_handshake_timeout_secs: defaults.tls_handshake_timeout_secs,
            keep_alive_timeout_secs: defaults.keep_alive_timeout_secs,
            idle_connection_timeout_secs: defaults.idle_connection_timeout_secs,
            read_timeout_secs: defaults.read_timeout_secs,
            write_timeout_secs: defaults.write_timeout_secs,
            http_request_timeout_secs: defaults.http_request_timeout_secs,
            http_response_timeout_secs: defaults.http_response_timeout_secs,
            dns_resolution_timeout_secs: defaults.dns_resolution_timeout_secs,
            retry_timeout_millis: defaults.retry_timeout_millis,
            backoff_timeout_millis: defaults.backoff_timeout_millis,
            ping_timeout_secs: defaults.ping_timeout_secs,
            heartbeat_timeout_secs: defaults.heartbeat_timeout_secs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_defaults() {
        let config = TimeoutConfig::const_defaults();
        assert_eq!(config.health_check_secs, 5);
        assert_eq!(config.hsm_operation_secs, 2);
        assert_eq!(config.hsm_probe_millis, 500);
        assert_eq!(config.discovery_operation_secs, 10);
    }

    #[test]
    fn test_default_uses_const_defaults() {
        let default_config = TimeoutConfig::default();
        let const_config = TimeoutConfig::const_defaults();
        assert_eq!(default_config, const_config);
    }

    #[test]
    fn test_builder_basic() {
        let config = TimeoutConfig::builder()
            .health_check_secs(15)
            .hsm_operation_secs(3)
            .build();

        assert_eq!(config.health_check_secs, 15);
        assert_eq!(config.hsm_operation_secs, 3);
        // Unset fields use defaults
        assert_eq!(config.hsm_probe_millis, 500);
        assert_eq!(config.discovery_operation_secs, 10);
    }

    #[test]
    fn test_builder_all_fields() {
        let config = TimeoutConfig::builder()
            .health_check_secs(8)
            .hsm_operation_secs(4)
            .hsm_probe_millis(750)
            .discovery_operation_secs(20)
            .build();

        assert_eq!(config.health_check_secs, 8);
        assert_eq!(config.hsm_operation_secs, 4);
        assert_eq!(config.hsm_probe_millis, 750);
        assert_eq!(config.discovery_operation_secs, 20);
    }

    #[test]
    fn test_duration_conversions() {
        let config = TimeoutConfig::default();
        assert_eq!(config.health_check_duration(), Duration::from_secs(5));
        assert_eq!(config.hsm_operation_duration(), Duration::from_secs(2));
        assert_eq!(config.hsm_probe_duration(), Duration::from_millis(500));
        assert_eq!(
            config.discovery_operation_duration(),
            Duration::from_secs(10)
        );
    }

    #[test]
    fn test_validation_success() {
        let config = TimeoutConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_health_check_too_short() {
        let config = TimeoutConfig::builder().health_check_secs(0).build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_health_check_too_long() {
        let config = TimeoutConfig::builder().health_check_secs(31).build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_hsm_operation_out_of_range() {
        let config = TimeoutConfig::builder().hsm_operation_secs(0).build();
        assert!(config.validate().is_err());

        let config = TimeoutConfig::builder().hsm_operation_secs(11).build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_hsm_probe_out_of_range() {
        let config = TimeoutConfig::builder().hsm_probe_millis(50).build();
        assert!(config.validate().is_err());

        let config = TimeoutConfig::builder().hsm_probe_millis(6000).build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_discovery_out_of_range() {
        let config = TimeoutConfig::builder().discovery_operation_secs(0).build();
        assert!(config.validate().is_err());

        let config = TimeoutConfig::builder()
            .discovery_operation_secs(61)
            .build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let config = TimeoutConfig::default();
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: TimeoutConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_custom_timeouts_validation() {
        let config = TimeoutConfig::builder()
            .health_check_secs(15)
            .hsm_operation_secs(5)
            .hsm_probe_millis(1000)
            .discovery_operation_secs(30)
            .build();

        assert!(config.validate().is_ok());
    }
}
