// SPDX-License-Identifier: AGPL-3.0-or-later

//! Timeout Configuration Builder
//!
//! Provides a fluent builder pattern for constructing `TimeoutConfig`.

use super::core::TimeoutConfig;
use super::defaults::default_timeouts;

#[cfg(test)]
#[path = "builder_comprehensive_tests.rs"]
mod builder_comprehensive_tests;

/// Builder for `TimeoutConfig`
///
/// Provides a fluent interface for constructing timeout configurations
/// without environment variable pollution in tests.
#[derive(Debug, Clone, Default)]
pub struct TimeoutConfigBuilder {
    health_check_secs: Option<u64>,
    hsm_operation_secs: Option<u64>,
    hsm_probe_millis: Option<u64>,
    discovery_operation_secs: Option<u64>,
    ai_decision_secs: Option<u64>,
    ai_request_secs: Option<u64>,
    ai_batch_timeout_millis: Option<u64>,
    pool_idle_secs: Option<u64>,
    max_connection_age_secs: Option<u64>,
    network_operation_secs: Option<u64>,
    dns_resolution_timeout_secs: Option<u64>,
    connection_timeout_secs: Option<u64>,
    request_timeout_secs: Option<u64>,
}

impl TimeoutConfigBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set health check timeout (seconds)
    pub const fn health_check_secs(mut self, secs: u64) -> Self {
        self.health_check_secs = Some(secs);
        self
    }

    /// Set HSM operation timeout (seconds)
    pub const fn hsm_operation_secs(mut self, secs: u64) -> Self {
        self.hsm_operation_secs = Some(secs);
        self
    }

    /// Set HSM probe timeout (milliseconds)
    pub const fn hsm_probe_millis(mut self, millis: u64) -> Self {
        self.hsm_probe_millis = Some(millis);
        self
    }

    /// Set discovery operation timeout (seconds)
    pub const fn discovery_operation_secs(mut self, secs: u64) -> Self {
        self.discovery_operation_secs = Some(secs);
        self
    }

    /// Set AI decision timeout (seconds)
    pub const fn ai_decision_secs(mut self, secs: u64) -> Self {
        self.ai_decision_secs = Some(secs);
        self
    }

    /// Set AI request timeout (seconds)
    pub const fn ai_request_secs(mut self, secs: u64) -> Self {
        self.ai_request_secs = Some(secs);
        self
    }

    /// Set AI batch timeout (milliseconds)
    pub const fn ai_batch_timeout_millis(mut self, millis: u64) -> Self {
        self.ai_batch_timeout_millis = Some(millis);
        self
    }

    /// Set pool idle timeout (seconds)
    pub const fn pool_idle_secs(mut self, secs: u64) -> Self {
        self.pool_idle_secs = Some(secs);
        self
    }

    /// Set max connection age (seconds)
    pub const fn max_connection_age_secs(mut self, secs: u64) -> Self {
        self.max_connection_age_secs = Some(secs);
        self
    }

    /// Set network operation timeout (seconds)
    pub const fn network_operation_secs(mut self, secs: u64) -> Self {
        self.network_operation_secs = Some(secs);
        self
    }

    /// Set DNS resolution timeout (seconds)
    pub const fn dns_resolution_timeout_secs(mut self, secs: u64) -> Self {
        self.dns_resolution_timeout_secs = Some(secs);
        self
    }

    /// Set connection timeout (seconds)
    pub const fn connection_timeout_secs(mut self, secs: u64) -> Self {
        self.connection_timeout_secs = Some(secs);
        self
    }

    /// Set request timeout (seconds)
    pub const fn request_timeout_secs(mut self, secs: u64) -> Self {
        self.request_timeout_secs = Some(secs);
        self
    }

    /// Load values from environment variables
    ///
    /// Only updates values that have environment variables set.
    /// Falls back to builder values or defaults.
    pub fn from_env(mut self) -> Self {
        if let Ok(val) = std::env::var("BEARDOG_HEALTH_CHECK_TIMEOUT_SECS")
            && let Ok(secs) = val.parse()
        {
            self.health_check_secs = Some(secs);
        }

        if let Ok(val) = std::env::var("BEARDOG_HSM_OPERATION_TIMEOUT_SECS")
            && let Ok(secs) = val.parse()
        {
            self.hsm_operation_secs = Some(secs);
        }

        if let Ok(val) = std::env::var("BEARDOG_HSM_PROBE_TIMEOUT_MILLIS")
            && let Ok(millis) = val.parse()
        {
            self.hsm_probe_millis = Some(millis);
        }

        if let Ok(val) = std::env::var("BEARDOG_DISCOVERY_TIMEOUT_SECS")
            && let Ok(secs) = val.parse()
        {
            self.discovery_operation_secs = Some(secs);
        }

        if let Ok(val) = std::env::var("BEARDOG_DECISION_TIMEOUT_SECS")
            && let Ok(secs) = val.parse()
        {
            self.ai_decision_secs = Some(secs);
        }

        if let Ok(val) = std::env::var("BEARDOG_AI_REQUEST_TIMEOUT_SECS")
            && let Ok(secs) = val.parse()
        {
            self.ai_request_secs = Some(secs);
        }

        if let Ok(val) = std::env::var("BEARDOG_AI_BATCH_TIMEOUT_MS")
            && let Ok(millis) = val.parse()
        {
            self.ai_batch_timeout_millis = Some(millis);
        }

        if let Ok(val) = std::env::var("BEARDOG_POOL_IDLE_TIMEOUT_SECS")
            && let Ok(secs) = val.parse()
        {
            self.pool_idle_secs = Some(secs);
        }

        if let Ok(val) = std::env::var("BEARDOG_MAX_CONNECTION_AGE_SECS")
            && let Ok(secs) = val.parse()
        {
            self.max_connection_age_secs = Some(secs);
        }

        self
    }

    /// Build the `TimeoutConfig`
    pub fn build(self) -> TimeoutConfig {
        TimeoutConfig {
            health_check_secs: self
                .health_check_secs
                .unwrap_or(default_timeouts::HEALTH_CHECK_SECS),
            hsm_operation_secs: self
                .hsm_operation_secs
                .unwrap_or(default_timeouts::HSM_OPERATION_SECS),
            hsm_probe_millis: self
                .hsm_probe_millis
                .unwrap_or(default_timeouts::HSM_PROBE_MILLIS),
            discovery_operation_secs: self
                .discovery_operation_secs
                .unwrap_or(default_timeouts::DISCOVERY_OPERATION_SECS),
            ai_decision_secs: self
                .ai_decision_secs
                .unwrap_or(default_timeouts::AI_DECISION_SECS),
            ai_request_secs: self
                .ai_request_secs
                .unwrap_or(default_timeouts::AI_REQUEST_SECS),
            ai_batch_timeout_millis: self
                .ai_batch_timeout_millis
                .unwrap_or(default_timeouts::AI_BATCH_TIMEOUT_MILLIS),
            pool_idle_secs: self
                .pool_idle_secs
                .unwrap_or(default_timeouts::POOL_IDLE_SECS),
            max_connection_age_secs: self
                .max_connection_age_secs
                .unwrap_or(default_timeouts::MAX_CONNECTION_AGE_SECS),
            network_operation_secs: self
                .network_operation_secs
                .unwrap_or(default_timeouts::NETWORK_OPERATION_SECS),
            dns_resolution_timeout_secs: self
                .dns_resolution_timeout_secs
                .unwrap_or(default_timeouts::DNS_RESOLUTION_TIMEOUT_SECS),
            connection_timeout_secs: self
                .connection_timeout_secs
                .unwrap_or(default_timeouts::CONNECTION_TIMEOUT_SECS),
            request_timeout_secs: self
                .request_timeout_secs
                .unwrap_or(default_timeouts::REQUEST_TIMEOUT_SECS),
        }
    }
}
