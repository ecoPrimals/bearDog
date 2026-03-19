// SPDX-License-Identifier: AGPL-3.0-only

//! Default Timeout Values
//!
//! Provides the Default implementation for TimeoutConfig.

use super::core::TimeoutConfig;

/// Default timeout values (seconds)
pub mod default_timeouts {
    pub const HEALTH_CHECK_SECS: u64 = 5;
    pub const HSM_OPERATION_SECS: u64 = 2;
    pub const HSM_PROBE_MILLIS: u64 = 500;
    pub const DISCOVERY_OPERATION_SECS: u64 = 10;
    pub const AI_DECISION_SECS: u64 = 30;
    pub const AI_REQUEST_SECS: u64 = 30;
    pub const AI_BATCH_TIMEOUT_MILLIS: u64 = 10;
    pub const POOL_IDLE_SECS: u64 = 300;
    pub const MAX_CONNECTION_AGE_SECS: u64 = 3600;
    pub const NETWORK_OPERATION_SECS: u64 = 30;
    pub const DNS_RESOLUTION_TIMEOUT_SECS: u64 = 5;
    pub const CONNECTION_TIMEOUT_SECS: u64 = 10;
    pub const REQUEST_TIMEOUT_SECS: u64 = 60;
}

impl Default for TimeoutConfig {
    /// Pure default implementation (no environment variable reads)
    ///
    /// Use `TimeoutConfig::from_env()` for environment-aware configuration.
    fn default() -> Self {
        Self {
            health_check_secs: default_timeouts::HEALTH_CHECK_SECS,
            hsm_operation_secs: default_timeouts::HSM_OPERATION_SECS,
            hsm_probe_millis: default_timeouts::HSM_PROBE_MILLIS,
            discovery_operation_secs: default_timeouts::DISCOVERY_OPERATION_SECS,
            ai_decision_secs: default_timeouts::AI_DECISION_SECS,
            ai_request_secs: default_timeouts::AI_REQUEST_SECS,
            ai_batch_timeout_millis: default_timeouts::AI_BATCH_TIMEOUT_MILLIS,
            pool_idle_secs: default_timeouts::POOL_IDLE_SECS,
            max_connection_age_secs: default_timeouts::MAX_CONNECTION_AGE_SECS,
            network_operation_secs: default_timeouts::NETWORK_OPERATION_SECS,
            dns_resolution_timeout_secs: default_timeouts::DNS_RESOLUTION_TIMEOUT_SECS,
            connection_timeout_secs: default_timeouts::CONNECTION_TIMEOUT_SECS,
            request_timeout_secs: default_timeouts::REQUEST_TIMEOUT_SECS,
        }
    }
}
