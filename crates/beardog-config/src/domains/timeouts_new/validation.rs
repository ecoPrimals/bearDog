// SPDX-License-Identifier: AGPL-3.0-or-later

//! Timeout Configuration Validation
//!
//! Validates timeout values are within acceptable ranges.

use super::core::TimeoutConfig;
use crate::ConfigError;

#[cfg(test)]
#[path = "validation_comprehensive_tests.rs"]
mod validation_comprehensive_tests;

/// Validate timeout configuration
///
/// Checks all timeout values are within acceptable operational ranges.
///
/// # Errors
///
/// Returns [`ConfigError::InvalidValue`] when any timeout is outside its allowed range.
pub fn validate_config(config: &TimeoutConfig) -> crate::ConfigResult<()> {
    fn range_check(
        field: &str,
        value: u64,
        min: u64,
        max: u64,
        unit: &str,
    ) -> crate::ConfigResult<()> {
        if value < min || value > max {
            return Err(ConfigError::invalid_value(
                field,
                format!("must be {min}-{max} {unit}, got {value}"),
            ));
        }
        Ok(())
    }

    range_check(
        "health_check_secs",
        config.health_check_secs,
        1,
        60,
        "seconds",
    )?;
    range_check(
        "hsm_operation_secs",
        config.hsm_operation_secs,
        1,
        10,
        "seconds",
    )?;
    range_check(
        "hsm_probe_millis",
        config.hsm_probe_millis,
        100,
        5000,
        "milliseconds",
    )?;
    range_check(
        "discovery_operation_secs",
        config.discovery_operation_secs,
        1,
        300,
        "seconds",
    )?;
    range_check(
        "ai_decision_secs",
        config.ai_decision_secs,
        1,
        300,
        "seconds",
    )?;
    range_check("ai_request_secs", config.ai_request_secs, 1, 300, "seconds")?;
    range_check(
        "ai_batch_timeout_millis",
        config.ai_batch_timeout_millis,
        1,
        1000,
        "milliseconds",
    )?;
    range_check("pool_idle_secs", config.pool_idle_secs, 60, 7200, "seconds")?;
    range_check(
        "max_connection_age_secs",
        config.max_connection_age_secs,
        300,
        86400,
        "seconds",
    )?;

    Ok(())
}
