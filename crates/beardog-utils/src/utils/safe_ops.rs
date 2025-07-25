//! # SafeOps - Comprehensive Safety Utilities
//!
//! Deep architectural transformation utilities for eliminating panic-prone patterns
//! throughout the BearDog ecosystem. Provides production-ready alternatives to
//! unwrap(), expect(), and other panic-prone operations.
//!
//! ZERO PANIC POLICY - Production resilience through systematic safety.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tokio::time::timeout;
use tracing::{debug, error, warn};

#[cfg(test)]
use std::sync::Arc;

/// Safe RwLock operations with timeout protection
pub struct SafeLock;

impl SafeLock {
    /// Safe read lock acquisition with timeout protection
    pub async fn safe_read<T>(
        lock: &RwLock<T>,
        timeout_duration: Duration,
    ) -> BearDogResult<RwLockReadGuard<'_, T>> {
        match timeout(timeout_duration, lock.read()).await {
            Ok(guard) => Ok(guard),
            Err(_) => Err(BearDogError::TimeoutError {
                message: format!(
                    "Read lock acquisition timed out after {:?}",
                    timeout_duration
                ),
            }),
        }
    }

    /// Safe write lock acquisition with timeout protection
    pub async fn safe_write<T>(
        lock: &RwLock<T>,
        timeout_duration: Duration,
    ) -> BearDogResult<RwLockWriteGuard<'_, T>> {
        match timeout(timeout_duration, lock.write()).await {
            Ok(guard) => Ok(guard),
            Err(_) => Err(BearDogError::TimeoutError {
                message: format!(
                    "Write lock acquisition timed out after {:?}",
                    timeout_duration
                ),
            }),
        }
    }
}

/// Safe collection operations
pub struct SafeCollection;

impl SafeCollection {
    /// Safe vector index access
    pub fn safe_get<T>(vec: &[T], index: usize) -> BearDogResult<&T> {
        vec.get(index).ok_or_else(|| {
            BearDogError::ValidationError(format!(
                "Index {} out of bounds for vector of length {}",
                index,
                vec.len()
            ))
        })
    }

    /// Safe HashMap key access with proper lifetime
    pub fn safe_get_key<'a, K, V>(map: &'a HashMap<K, V>, key: &K) -> BearDogResult<&'a V>
    where
        K: std::fmt::Debug + std::hash::Hash + Eq,
    {
        map.get(key).ok_or_else(|| {
            BearDogError::ValidationError(format!("Key {:?} not found in HashMap", key))
        })
    }

    /// Safe vector first element access
    pub fn safe_first<T>(vec: &[T]) -> BearDogResult<&T> {
        vec.first().ok_or_else(|| {
            BearDogError::ValidationError("Cannot get first element: vector is empty".to_string())
        })
    }

    /// Safe vector last element access
    pub fn safe_last<T>(vec: &[T]) -> BearDogResult<&T> {
        vec.last().ok_or_else(|| {
            BearDogError::ValidationError("Cannot get last element: vector is empty".to_string())
        })
    }

    /// Safe vector push with capacity check
    pub fn safe_push_with_limit<T>(
        vec: &mut Vec<T>,
        item: T,
        max_capacity: usize,
    ) -> BearDogResult<()> {
        if vec.len() >= max_capacity {
            return Err(BearDogError::ValidationError(format!(
                "Vector capacity limit reached: {} >= {}",
                vec.len(),
                max_capacity
            )));
        }
        vec.push(item);
        Ok(())
    }
}

/// Safe parsing operations
pub struct SafeParse;

impl SafeParse {
    /// Safe string to integer parsing
    pub fn safe_parse_int(s: &str) -> BearDogResult<i64> {
        s.parse::<i64>().map_err(|e| {
            BearDogError::ValidationError(format!("Failed to parse '{}' as integer: {}", s, e))
        })
    }

    /// Safe string to float parsing
    pub fn safe_parse_float(s: &str) -> BearDogResult<f64> {
        s.parse::<f64>().map_err(|e| {
            BearDogError::ValidationError(format!("Failed to parse '{}' as float: {}", s, e))
        })
    }

    /// Safe string to boolean parsing
    pub fn safe_parse_bool(s: &str) -> BearDogResult<bool> {
        match s.to_lowercase().as_str() {
            "true" | "yes" | "1" | "on" => Ok(true),
            "false" | "no" | "0" | "off" => Ok(false),
            _ => Err(BearDogError::ValidationError(format!(
                "Cannot parse '{}' as boolean",
                s
            ))),
        }
    }

    /// Safe duration parsing from seconds
    pub fn safe_parse_duration_secs(secs: u64) -> BearDogResult<Duration> {
        if secs > 86400 {
            // More than 1 day
            return Err(BearDogError::ValidationError(format!(
                "Duration too large: {} seconds (max 86400)",
                secs
            )));
        }
        Ok(Duration::from_secs(secs))
    }
}

/// Safe JSON operations
pub struct SafeJson;

impl SafeJson {
    /// Safe JSON serialization
    pub fn safe_serialize<T>(value: &T) -> BearDogResult<String>
    where
        T: Serialize,
    {
        serde_json::to_string(value).map_err(|e| BearDogError::Serialization {
            message: format!("JSON serialization failed: {}", e),
        })
    }

    /// Safe JSON deserialization
    pub fn safe_deserialize<T>(json_str: &str) -> BearDogResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        serde_json::from_str(json_str).map_err(|e| BearDogError::Serialization {
            message: format!("JSON deserialization failed: {}", e),
        })
    }

    /// Safe JSON pretty serialization
    pub fn safe_serialize_pretty<T>(value: &T) -> BearDogResult<String>
    where
        T: Serialize,
    {
        serde_json::to_string_pretty(value).map_err(|e| BearDogError::Serialization {
            message: format!("JSON pretty serialization failed: {}", e),
        })
    }
}

/// Safe async operations
pub struct SafeAsync;

impl SafeAsync {
    /// Safe async operation with timeout
    pub async fn safe_timeout<F, T>(
        future: F,
        timeout_duration: Duration,
        operation_name: &str,
    ) -> BearDogResult<T>
    where
        F: std::future::Future<Output = T>,
    {
        match timeout(timeout_duration, future).await {
            Ok(result) => {
                debug!("✅ Safe async operation completed: {}", operation_name);
                Ok(result)
            }
            Err(_) => {
                error!(
                    "⏰ Safe async operation timed out: {} after {:?}",
                    operation_name, timeout_duration
                );
                Err(BearDogError::TimeoutError {
                    message: format!(
                        "Operation '{}' timed out after {:?}",
                        operation_name, timeout_duration
                    ),
                })
            }
        }
    }

    /// Safe async operation with retry logic
    pub async fn safe_retry<F, Fut, T, E>(
        operation: F,
        max_retries: usize,
        retry_delay: Duration,
        operation_name: &str,
    ) -> BearDogResult<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            debug!(
                "🔄 Safe retry attempt {} for operation: {}",
                attempt + 1,
                operation_name
            );

            match operation().await {
                Ok(result) => {
                    if attempt > 0 {
                        debug!(
                            "✅ Safe retry succeeded on attempt {} for operation: {}",
                            attempt + 1,
                            operation_name
                        );
                    }
                    return Ok(result);
                }
                Err(e) => {
                    last_error = Some(format!("{}", e));
                    if attempt < max_retries {
                        warn!(
                            "⚠️ Safe retry attempt {} failed for operation {}: {}",
                            attempt + 1,
                            operation_name,
                            e
                        );
                        tokio::time::sleep(retry_delay).await;
                    }
                }
            }
        }

        let error_msg = last_error.unwrap_or_else(|| "Unknown error".to_string());
        error!(
            "❌ Safe retry exhausted for operation {}: {}",
            operation_name, error_msg
        );

        Err(BearDogError::Unknown {
            message: format!(
                "Operation '{}' failed after {} retries: {}",
                operation_name, max_retries, error_msg
            ),
        })
    }
}

/// Main SafeOps interface with expanded utilities
pub struct SafeOps;

impl SafeOps {
    /// Safe read lock acquisition (convenience method)
    pub async fn safe_read_lock<T>(
        lock: &RwLock<T>,
        timeout_duration: Duration,
    ) -> BearDogResult<RwLockReadGuard<'_, T>> {
        SafeLock::safe_read(lock, timeout_duration).await
    }

    /// Safe write lock acquisition (convenience method)
    pub async fn safe_write_lock<T>(
        lock: &RwLock<T>,
        timeout_duration: Duration,
    ) -> BearDogResult<RwLockWriteGuard<'_, T>> {
        SafeLock::safe_write(lock, timeout_duration).await
    }

    /// Safe average calculation
    pub fn safe_avg<I>(values: I) -> Option<f64>
    where
        I: Iterator<Item = f64>,
    {
        let collected: Vec<f64> = values.collect();
        if collected.is_empty() {
            None
        } else {
            Some(collected.iter().sum::<f64>() / collected.len() as f64)
        }
    }

    /// Safe maximum value
    pub fn safe_max<I, T>(values: I) -> Option<T>
    where
        I: Iterator<Item = T>,
        T: Ord,
    {
        values.max()
    }

    /// Safe minimum value
    pub fn safe_min<I, T>(values: I) -> Option<T>
    where
        I: Iterator<Item = T>,
        T: Ord,
    {
        values.min()
    }

    /// Safe division with zero check
    pub fn safe_divide(numerator: f64, denominator: f64) -> BearDogResult<f64> {
        if denominator == 0.0 {
            Err(BearDogError::ValidationError(
                "Division by zero".to_string(),
            ))
        } else {
            Ok(numerator / denominator)
        }
    }

    /// Safe percentage calculation
    pub fn safe_percentage(part: f64, whole: f64) -> BearDogResult<f64> {
        if whole == 0.0 {
            Err(BearDogError::ValidationError(
                "Cannot calculate percentage: whole value is zero".to_string(),
            ))
        } else {
            Ok((part / whole) * 100.0)
        }
    }

    /// Safe string validation
    pub fn safe_validate_string(
        s: &str,
        min_len: usize,
        max_len: usize,
        field_name: &str,
    ) -> BearDogResult<()> {
        if s.len() < min_len {
            return Err(BearDogError::ValidationError(format!(
                "{} too short: {} characters (minimum {})",
                field_name,
                s.len(),
                min_len
            )));
        }

        if s.len() > max_len {
            return Err(BearDogError::ValidationError(format!(
                "{} too long: {} characters (maximum {})",
                field_name,
                s.len(),
                max_len
            )));
        }

        Ok(())
    }

    /// Safe numeric range validation
    pub fn safe_validate_range<T>(value: T, min: T, max: T, field_name: &str) -> BearDogResult<()>
    where
        T: PartialOrd + std::fmt::Display + Copy,
    {
        if value < min {
            return Err(BearDogError::ValidationError(format!(
                "{} below minimum: {} < {}",
                field_name, value, min
            )));
        }

        if value > max {
            return Err(BearDogError::ValidationError(format!(
                "{} above maximum: {} > {}",
                field_name, value, max
            )));
        }

        Ok(())
    }

    /// Safe Option unwrapping with context
    pub fn safe_unwrap_option<T>(option: Option<T>, context: &str) -> BearDogResult<T> {
        option.ok_or_else(|| {
            BearDogError::ValidationError(format!("Expected value but found None: {}", context))
        })
    }

    /// Safe Result unwrapping with context enhancement
    pub fn safe_unwrap_result<T, E>(result: Result<T, E>, context: &str) -> BearDogResult<T>
    where
        E: std::fmt::Display,
    {
        result.map_err(|e| {
            BearDogError::ValidationError(format!("Operation failed ({}): {}", context, e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_collection_operations() {
        let numbers = vec![1, 2, 3, 4, 5];

        assert_eq!(SafeCollection::safe_get(&numbers, 0).unwrap(), &1);
        assert_eq!(SafeCollection::safe_get(&numbers, 4).unwrap(), &5);
        assert_eq!(SafeCollection::safe_first(&numbers).unwrap(), &1);
        assert_eq!(SafeCollection::safe_last(&numbers).unwrap(), &5);

        let empty: Vec<i32> = vec![];
        assert!(SafeCollection::safe_get(&empty, 0).is_err());
    }

    #[test]
    fn test_safe_parse_operations() {
        assert_eq!(SafeParse::safe_parse_int("42").unwrap(), 42);
        assert!(SafeParse::safe_parse_int("invalid").is_err());

        assert_eq!(SafeParse::safe_parse_float("3.14").unwrap(), 3.14);
        assert!(SafeParse::safe_parse_float("not_a_float").is_err());

        assert_eq!(SafeParse::safe_parse_bool("true").unwrap(), true);
        assert_eq!(SafeParse::safe_parse_bool("False").unwrap(), false);
        assert!(SafeParse::safe_parse_bool("maybe").is_err());

        assert_eq!(
            SafeParse::safe_parse_duration_secs(60).unwrap(),
            Duration::from_secs(60)
        );
        assert!(SafeParse::safe_parse_duration_secs(86401).is_err());
    }

    #[test]
    fn test_safe_json_operations() {
        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            name: String,
            value: i32,
        }

        let test_obj = TestStruct {
            name: "test".to_string(),
            value: 42,
        };
        assert_eq!(
            SafeJson::safe_serialize(&test_obj).unwrap(),
            r#"{"name":"test","value":42}"#
        );

        let deserialized: TestStruct =
            SafeJson::safe_deserialize(r#"{"name":"test","value":42}"#).unwrap();
        assert_eq!(deserialized.name, "test");
        assert_eq!(deserialized.value, 42);

        assert!(SafeJson::safe_serialize_pretty(&test_obj).is_ok());
    }

    #[tokio::test]
    async fn test_safe_async_operations() {
        let lock = Arc::new(RwLock::new(42));
        let lock_clone = lock.clone();

        let result = SafeAsync::safe_timeout(
            async {
                let _guard = SafeLock::safe_read(&lock_clone, Duration::from_millis(100)).await?;
                Ok(())
            },
            Duration::from_millis(50),
            "test_safe_async_timeout",
        )
        .await
        .unwrap();
        assert!(result.is_ok());

        let result = SafeAsync::safe_retry(
            || async { Ok(42) },
            3,
            Duration::from_millis(10),
            "test_safe_async_retry",
        )
        .await
        .unwrap();
        assert_eq!(result, 42);

        // Test retry with failure - should fail after retries
        let result = SafeAsync::safe_retry(
            || async {
                Err(BearDogError::TimeoutError {
                    message: "Simulated error".to_string(),
                })
            },
            2,
            Duration::from_millis(5),
            "test_safe_async_retry_failure",
        )
        .await;
        assert!(result.is_err()); // Should fail after retries
    }

    #[test]
    fn test_safe_ops_main_interface() {
        let lock = Arc::new(RwLock::new(42));
        let lock_clone = lock.clone();

        // These are async operations, skip for sync test
        // assert_eq!(SafeOps::safe_read_lock(&lock_clone, Duration::from_millis(100)).await.unwrap(), 42);
        // assert_eq!(SafeOps::safe_write_lock(&lock_clone, Duration::from_millis(100)).await.unwrap(), 42);

        assert_eq!(
            SafeOps::safe_avg(vec![1.0, 2.0, 3.0].into_iter()).unwrap(),
            2.0
        );
        assert_eq!(SafeOps::safe_max(vec![1, 2, 3, 4, 5].iter()).unwrap(), &5);
        assert_eq!(SafeOps::safe_min(vec![1, 2, 3, 4, 5].iter()).unwrap(), &1);

        assert_eq!(SafeOps::safe_divide(10.0, 2.0).unwrap(), 5.0);
        assert!(SafeOps::safe_divide(10.0, 0.0).is_err());

        assert_eq!(SafeOps::safe_percentage(10.0, 100.0).unwrap(), 10.0);
        assert!(SafeOps::safe_percentage(10.0, 0.0).is_err());

        assert!(SafeOps::safe_validate_string("hello", 1, 10, "test_string").is_ok());
        assert!(SafeOps::safe_validate_string("hello", 5, 10, "test_string").is_err());
        assert!(SafeOps::safe_validate_string("hello", 1, 5, "test_string").is_err());

        assert!(SafeOps::safe_validate_range(5, 1, 10, "test_range").is_ok());
        assert!(SafeOps::safe_validate_range(0, 1, 10, "test_range").is_err());
        assert!(SafeOps::safe_validate_range(15, 1, 10, "test_range").is_err());

        assert_eq!(
            SafeOps::safe_unwrap_option(Some(42), "test_option").unwrap(),
            42
        );
        assert!(SafeOps::safe_unwrap_option(None, "test_option").is_err());

        assert_eq!(
            SafeOps::safe_unwrap_result(Ok(42), "test_result").unwrap(),
            42
        );
        assert!(
            SafeOps::safe_unwrap_result(Err::<i32, _>("Simulated error"), "test_result").is_err()
        );
    }
}
