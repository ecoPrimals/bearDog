//! Timeout Configuration Validation
//!
//! Validates timeout values are within acceptable ranges.

use super::core::TimeoutConfig;

/// Validate timeout configuration
///
/// Checks all timeout values are within acceptable operational ranges.
pub fn validate_config(config: &TimeoutConfig) -> Result<(), String> {
    // Health check: 1-60 seconds
    if config.health_check_secs == 0 || config.health_check_secs > 60 {
        return Err(format!(
            "Health check timeout must be 1-60 seconds, got {}",
            config.health_check_secs
        ));
    }

    // HSM operation: 1-10 seconds
    if config.hsm_operation_secs == 0 || config.hsm_operation_secs > 10 {
        return Err(format!(
            "HSM operation timeout must be 1-10 seconds, got {}",
            config.hsm_operation_secs
        ));
    }

    // HSM probe: 100-5000 milliseconds
    if config.hsm_probe_millis < 100 || config.hsm_probe_millis > 5000 {
        return Err(format!(
            "HSM probe timeout must be 100-5000 milliseconds, got {}",
            config.hsm_probe_millis
        ));
    }

    // Discovery: 1-300 seconds
    if config.discovery_operation_secs == 0 || config.discovery_operation_secs > 300 {
        return Err(format!(
            "Discovery timeout must be 1-300 seconds, got {}",
            config.discovery_operation_secs
        ));
    }

    // AI decision: 1-300 seconds
    if config.ai_decision_secs == 0 || config.ai_decision_secs > 300 {
        return Err(format!(
            "AI decision timeout must be 1-300 seconds, got {}",
            config.ai_decision_secs
        ));
    }

    // AI request: 1-300 seconds
    if config.ai_request_secs == 0 || config.ai_request_secs > 300 {
        return Err(format!(
            "AI request timeout must be 1-300 seconds, got {}",
            config.ai_request_secs
        ));
    }

    // AI batch: 1-1000 milliseconds
    if config.ai_batch_timeout_millis == 0 || config.ai_batch_timeout_millis > 1000 {
        return Err(format!(
            "AI batch timeout must be 1-1000 milliseconds, got {}",
            config.ai_batch_timeout_millis
        ));
    }

    // Pool idle: 60-7200 seconds (1 minute to 2 hours)
    if config.pool_idle_secs < 60 || config.pool_idle_secs > 7200 {
        return Err(format!(
            "Pool idle timeout must be 60-7200 seconds, got {}",
            config.pool_idle_secs
        ));
    }

    // Max connection age: 300-86400 seconds (5 minutes to 24 hours)
    if config.max_connection_age_secs < 300 || config.max_connection_age_secs > 86400 {
        return Err(format!(
            "Max connection age must be 300-86400 seconds, got {}",
            config.max_connection_age_secs
        ));
    }

    Ok(())
}
