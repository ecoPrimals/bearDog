// SPDX-License-Identifier: AGPL-3.0-only



use beardog_errors::BearDogError;
use std::time::Duration;
use tracing::{debug, error, warn};


/// Creates instance with operation context
pub fn with_operation_context<F, Fut, T>(
    operation_name: &str,
    operation: F,
) -> Result<T, BearDogError>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, BearDogError>>,
{
    debug!("🔄 Starting operation: {}", operation_name);
    match operation() {
        Ok(result) => {
            debug!("✅ Operation completed: {}", operation_name);
            Ok(result)
        }
        Err(e) => {
            error!("❌ Operation failed: {}: {}", operation_name, e);
            Err(e)
    }
}


/// Creates instance with retry
pub fn with_retry<F, Fut, T, E>(
    operation_name: &str,
    max_retries: usize,
    base_delay: Duration,
    operation: F,
) -> Result<T, BearDogError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display + std::fmt::Debug,
{
    let mut last_error = None;
    for attempt in 0..=max_retries {
        let delay = base_delay * 2_u32.pow(attempt as u32);
        match operation() {
            Ok(result) => {
                if attempt > 0 {
                    debug!(
                        "✅ Operation "{}" succeeded on attempt {}",
                        operation_name,
                        attempt + 1
                    );
                }
                return Ok(result);
            }
            Err(e) => {
                last_error = Some(format!("{e}"));
                if attempt < max_retries {
                    warn!(
                        "⚠️ Operation "{}" failed on attempt {}: {} (retrying in {:?})",
                        operation_name,
                        attempt + 1,
                        e,
                        delay
                    );
                    tokio::time::sleep(delay).await;
                } else {
                    error!(
                        "❌ Operation "{}" failed after {} attempts: {}",
                        operation_name,
                        max_retries + 1,
                        e
                    );
                }
            }
        }
    }
    
    Err(BearDogError::internal(format!(
        "Operation "{}" failed after {} retries: {}",
        operation_name,
        max_retries + 1,
        last_error.unwrap_or_else(|| "Unknown error".to_string())
    )))
}

/// Validates input
pub fn validate_input<T, F>(value: T, validator: F, field_name: &str) -> Result<T, BearDogError>
where
    F: FnOnce(&T) -> bool,
{
    if validator(&value) {
        Ok(value)
    } else {
        Err(BearDogError::validation(format!("Invalid value for field '{field_name}'")))
    }
}


/// Creates instance with cleanup
pub fn with_cleanup<F, Fut, C, CleanupFut, T>(
    operation_name: &str,
    operation: F,
    cleanup: C,
) -> Result<T, BearDogError>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, BearDogError>>,
    C: FnOnce() -> CleanupFut,
    CleanupFut: std::future::Future<Output = Result<(), BearDogError>>,
{
    let result = operation();

    if let Err(cleanup_error) = cleanup() {
        warn!(
            "⚠️ Cleanup failed for operation "{}": {}",
            operation_name, cleanup_error
        );
    }
    
    result
}

/// Loads config_with_fallback
pub fn load_config_with_fallback<T>(
    primary_loader: impl FnOnce() -> Result<T, BearDogError>,
    fallback_loader: impl FnOnce() -> Result<T, BearDogError>,
    config_name: &str,
) -> Result<T, BearDogError> {
    match primary_loader() {
        Ok(config) => {
            debug!("✅ Loaded {} from primary source", config_name);
            Ok(config)
        Err(primary_error) => {
            warn!(
                "⚠️ Primary {} loading failed: {}, trying fallback",
                config_name, primary_error
            );
            match fallback_loader() {
                Ok(config) => {
                    debug!("✅ Loaded {} from fallback source", config_name);
                    Ok(config)
                }
                Err(fallback_error) => {
                    error!(
                        "❌ Both primary and fallback {} loading failed",
                        config_name
                    );
                    Err(BearDogError::configuration(format!(
                        "Failed to load {}: primary error: {}, fallback error: {}",
                        config_name, primary_error, fallback_error
                    )))
                }
            }
        }
    }
}
