// SPDX-License-Identifier: AGPL-3.0-only

//! Core Timeout Configuration Types
//!
//! Defines the main `TimeoutConfig` struct and its primary methods.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Timeout configuration for all BearDog operations
///
///All timeout values can be overridden via environment variables or configuration file.
/// Each timeout has a sensible default based on industry standards.
///
/// # Concurrent Safety
///
/// - `Default` implementation is pure (no env var reads)
/// - Use `from_env()` to explicitly load from environment
/// - Use `builder()` for test-friendly construction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct TimeoutConfig {
    /// Health check timeout in seconds (default: 5s)
    pub health_check_secs: u64,

    /// HSM operation timeout in seconds (default: 2s)
    pub hsm_operation_secs: u64,

    /// HSM probe timeout in milliseconds (default: 500ms)
    pub hsm_probe_millis: u64,

    /// Service discovery timeout in seconds (default: 10s)
    pub discovery_operation_secs: u64,

    /// AI decision timeout in seconds (default: 30s)
    pub ai_decision_secs: u64,

    /// AI request timeout in seconds (default: 30s)
    pub ai_request_secs: u64,

    /// AI batch timeout in milliseconds (default: 10ms)
    pub ai_batch_timeout_millis: u64,

    /// Pool idle timeout in seconds (default: 300s)
    pub pool_idle_secs: u64,

    /// Max connection age in seconds (default: 3600s)
    pub max_connection_age_secs: u64,

    /// Network operation timeout in seconds (default: 30s)
    pub network_operation_secs: u64,

    /// DNS resolution timeout in seconds (default: 5s)
    pub dns_resolution_timeout_secs: u64,

    /// Connection timeout in seconds (default: 10s)
    pub connection_timeout_secs: u64,

    /// Request timeout in seconds (default: 60s)
    pub request_timeout_secs: u64,
}

impl TimeoutConfig {
    /// Create a new builder for TimeoutConfig
    pub fn builder() -> crate::domains::timeouts::TimeoutConfigBuilder {
        crate::domains::timeouts::TimeoutConfigBuilder::new()
    }

    /// Load timeout configuration from environment variables
    ///
    /// Environment variables:
    /// - `BEARDOG_HEALTH_CHECK_TIMEOUT_SECS`
    /// - `BEARDOG_HSM_OPERATION_TIMEOUT_SECS`
    /// - `BEARDOG_HSM_PROBE_TIMEOUT_MILLIS`
    /// - `BEARDOG_DISCOVERY_TIMEOUT_SECS`
    /// - `BEARDOG_DECISION_TIMEOUT_SECS`
    /// - `BEARDOG_AI_REQUEST_TIMEOUT_SECS`
    /// - `BEARDOG_AI_BATCH_TIMEOUT_MS`
    /// - `BEARDOG_POOL_IDLE_TIMEOUT_SECS`
    /// - `BEARDOG_MAX_CONNECTION_AGE_SECS`
    pub fn from_env() -> Self {
        Self::builder().from_env().build()
    }

    /// Validate timeout configuration
    ///
    /// Returns an error if any timeout values are invalid:
    /// - Health check: 1-60 seconds
    /// - HSM operation: 1-10 seconds
    /// - HSM probe: 100-5000 milliseconds
    /// - Discovery: 1-300 seconds
    /// - AI decision: 1-300 seconds
    /// - AI request: 1-300 seconds
    /// - AI batch: 1-1000 milliseconds
    /// - Pool idle: 60-7200 seconds
    /// - Max connection age: 300-86400 seconds
    pub fn validate(&self) -> Result<(), String> {
        crate::domains::timeouts::validation::validate_config(self)
    }

    // Duration conversion methods

    /// Get health check timeout as Duration
    pub const fn health_check_duration(&self) -> Duration {
        Duration::from_secs(self.health_check_secs)
    }

    /// Get HSM operation timeout as Duration
    pub const fn hsm_operation_duration(&self) -> Duration {
        Duration::from_secs(self.hsm_operation_secs)
    }

    /// Get HSM probe timeout as Duration
    pub const fn hsm_probe_duration(&self) -> Duration {
        Duration::from_millis(self.hsm_probe_millis)
    }

    /// Get discovery operation timeout as Duration
    pub const fn discovery_operation_duration(&self) -> Duration {
        Duration::from_secs(self.discovery_operation_secs)
    }

    /// Get AI decision timeout as Duration
    pub const fn ai_decision_duration(&self) -> Duration {
        Duration::from_secs(self.ai_decision_secs)
    }

    /// Get AI request timeout as Duration
    pub const fn ai_request_duration(&self) -> Duration {
        Duration::from_secs(self.ai_request_secs)
    }

    /// Get AI batch timeout as Duration
    pub const fn ai_batch_timeout_duration(&self) -> Duration {
        Duration::from_millis(self.ai_batch_timeout_millis)
    }

    /// Get pool idle timeout as Duration
    pub const fn pool_idle_duration(&self) -> Duration {
        Duration::from_secs(self.pool_idle_secs)
    }

    /// Get max connection age as Duration
    pub const fn max_connection_age_duration(&self) -> Duration {
        Duration::from_secs(self.max_connection_age_secs)
    }

    /// Get network operation timeout as Duration
    pub const fn network_operation_duration(&self) -> Duration {
        Duration::from_secs(self.network_operation_secs)
    }

    /// Get DNS resolution timeout as Duration
    pub const fn dns_resolution_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.dns_resolution_timeout_secs)
    }

    /// Get connection timeout as Duration
    pub const fn connection_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.connection_timeout_secs)
    }

    /// Get request timeout as Duration
    pub const fn request_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.request_timeout_secs)
    }
}
