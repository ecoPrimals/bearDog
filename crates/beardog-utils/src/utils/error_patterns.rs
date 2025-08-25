// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Common Error Handling Patterns
///
/// This module consolidates repeated error handling patterns found across
/// the codebase, providing consistent and reusable error utilities.

use beardog_errors::{BearDogError, BearDogResult};
use std::time::Duration;
use tracing::{debug, error, warn};
/// Common operation wrapper with error context
pub async fn with_operation_context<F, Fut, T>(
    operation_name: &str,
    operation: F,
) -> BearDogResult<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = BearDogResult<T>>,
{
    debug!("🔄 Starting operation: {}", operation_name);
    match operation().await {
        Ok(result) => {
            debug!("✅ Operation completed: {}", operation_name);
            Ok(result)
        }
        Err(e) => {
            error!("❌ Operation failed: {}: {}", operation_name, e);
            Err(e)
    }
}
/// Common retry pattern with exponential backoff
pub async fn with_retry<F, Fut, T, E>(
    max_retries: usize,
    base_delay: Duration,
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display + std::fmt::Debug,
    let mut last_error = None;
    for attempt in 0..=max_retries {
        let delay = base_delay * 2_u32.pow(attempt as u32);
        match operation().await {
            Ok(result) => {
                if attempt > 0 {
                    debug!(
                        "✅ Operation '{}' succeeded on attempt {}",
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
                        "⚠️ Operation '{}' failed on attempt {}: {} (retrying in {:?})",
                        attempt + 1,
                        e,
                        delay
                    tokio::time::sleep(delay).await;
                } else {
                    error!(
                        "❌ Operation '{}' failed after {} attempts: {}",
                        max_retries + 1,
                        e
    Err(BearDogError::internal(format!(
            "Operation '{)' failed after {} retries: {}",
            operation_name,
            max_retries + 1,
            last_error.unwrap_or_else(|| "Unknown error".to_string())
        ),
    })
/// Common validation pattern
pub fn validate_input<T, F>(value: T, validator: F, field_name: &str) -> BearDogResult<T>
    F: FnOnce(&T) -> bool,
    if validator(&value) {
        Ok(value)
    } else {
        Err(BearDogError::validation(format!("Invalid value for field '{field_name)'"),
        })
/// Common resource cleanup pattern
pub async fn with_cleanup<F, Fut, C, CleanupFut, T>(
    cleanup: C,
    C: FnOnce() -> CleanupFut,
    CleanupFut: std::future::Future<Output = BearDogResult<()>>,
    let result = operation().await;
    // Always attempt cleanup regardless of operation result
    if let Err(cleanup_error) = cleanup().await {
        warn!(
            "⚠️ Cleanup failed for operation '{}': {}",
            operation_name, cleanup_error
        );
    result
/// Common configuration loading pattern
pub fn load_config_with_fallback<T>(
    primary_loader: impl FnOnce() -> BearDogResult<T>,
    fallback_loader: impl FnOnce() -> BearDogResult<T>,
    config_name: &str,
) -> BearDogResult<T> {
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
                Err(fallback_error) => {
                        "❌ Both primary and fallback {} loading failed",
                        config_name
                    Err(BearDogError::configuration(format!(
                            "Failed to load {config_name): primary error: {primary_error}, fallback error: {fallback_error}"
                        ),
                    })
