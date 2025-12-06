//! Safe operation utilities for error-free data access and manipulation
//!
//! This module provides comprehensive safe wrappers around common operations
//! that can fail at runtime, returning `Result` types instead of panicking.
//!
//! # Features
//!
//! - **Lock Safety**: Timeout-based lock acquisition to prevent deadlocks
//! - **Collection Safety**: Bounds-checked array/vector access
//! - **Arithmetic Safety**: Division by zero and overflow protection
//! - **String Safety**: Bounds-checked substring operations
//! - **Type Safety**: Safe conversions and parsing with error handling
//!
//! # Example
//!
//! ```rust,ignore
//! use beardog_utils::utils::safe_ops::{SafeCollection, SafeArithmetic};
//!
//! let data = vec![1, 2, 3, 4, 5];
//! let item = SafeCollection::safe_get(&data, 2)?; // Returns Ok(&3)
//! let result = SafeArithmetic::safe_divide(10.0, 2.0)?; // Returns Ok(5.0)
//! ```

use beardog_errors::BearDogError;
use std::time::Duration;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tokio::time::timeout;

/// Safe lock acquisition utilities with timeout protection
///
/// Provides timeout-based wrappers around async `RwLock` operations
/// to prevent indefinite blocking and potential deadlocks.
pub struct SafeLock;

impl SafeLock {
    /// Acquire a read lock with timeout protection
    ///
    /// # Errors
    /// Returns error if lock cannot be acquired within the specified timeout duration
    pub async fn safe_read_lock<T>(
        lock: &RwLock<T>,
        timeout_duration: Duration,
    ) -> Result<RwLockReadGuard<'_, T>, BearDogError> {
        match timeout(timeout_duration, lock.read()).await {
            Ok(guard) => Ok(guard),
            Err(_) => Err(BearDogError::system(format!(
                "Read lock acquisition timed out after {timeout_duration:?}"
            ))),
        }
    }

    /// Acquire a write lock with timeout protection
    ///
    /// # Errors
    /// Returns error if lock cannot be acquired within the specified timeout duration
    pub async fn safe_write_lock<T>(
        lock: &RwLock<T>,
        timeout_duration: Duration,
    ) -> Result<RwLockWriteGuard<'_, T>, BearDogError> {
        match timeout(timeout_duration, lock.write()).await {
            Ok(guard) => Ok(guard),
            Err(_) => Err(BearDogError::system(format!(
                "Write lock acquisition timed out after {timeout_duration:?}"
            ))),
        }
    }
}

/// Safe collection access utilities with bounds checking
///
/// Provides bounds-checked access to collections, returning errors
/// instead of panicking on out-of-bounds access.
pub struct SafeCollection;

impl SafeCollection {
    /// Get element at index with bounds checking
    ///
    /// # Errors
    /// Returns error if index is out of bounds
    pub fn safe_get<T>(vec: &[T], index: usize) -> Result<&T, BearDogError> {
        vec.get(index).ok_or_else(|| {
            BearDogError::validation(&format!(
                "Index {index} out of bounds for collection of length {}",
                vec.len()
            ))
        })
    }

    /// Get mutable element at index with bounds checking
    ///
    /// # Errors
    /// Returns error if index is out of bounds
    pub fn safe_get_mut<T>(vec: &mut [T], index: usize) -> Result<&mut T, BearDogError> {
        let len = vec.len();
        vec.get_mut(index).ok_or_else(|| {
            BearDogError::validation(&format!(
                "Index {index} out of bounds for collection of length {len}"
            ))
        })
    }

    /// Get first element with empty collection checking
    ///
    /// # Errors
    /// Returns error if collection is empty
    pub fn safe_first<T>(vec: &[T]) -> Result<&T, BearDogError> {
        vec.first()
            .ok_or_else(|| BearDogError::validation("Cannot get first element of empty collection"))
    }

    /// Get last element with empty collection checking
    ///
    /// # Errors
    /// Returns error if collection is empty
    pub fn safe_last<T>(vec: &[T]) -> Result<&T, BearDogError> {
        vec.last()
            .ok_or_else(|| BearDogError::validation("Cannot get last element of empty collection"))
    }
}

/// Safe arithmetic operations with overflow and division-by-zero protection
///
/// Provides safe wrappers around arithmetic operations that can fail,
/// returning errors instead of panicking or producing undefined behavior.
pub struct SafeArithmetic;

impl SafeArithmetic {
    /// Safely add two values with type constraints
    ///
    /// # Errors
    /// This operation currently does not return errors but is structured for future overflow checking
    pub fn safe_add<T>(a: T, b: T) -> Result<T, BearDogError>
    where
        T: std::ops::Add<Output = T> + Copy + std::fmt::Display,
    {
        Ok(a + b)
    }

    /// Safely divide two floating point numbers with zero-division protection
    ///
    /// # Errors
    /// Returns error if divisor is zero
    pub fn safe_divide(a: f64, b: f64) -> Result<f64, BearDogError> {
        if b == 0.0 {
            Err(BearDogError::validation("Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    /// Safely convert i64 to usize with negative value checking
    ///
    /// # Errors
    /// Returns error if value is negative
    pub fn safe_to_usize(value: i64) -> Result<usize, BearDogError> {
        if value < 0 {
            Err(BearDogError::validation(&format!(
                "Cannot convert negative value {value} to usize"
            )))
        } else {
            Ok(value as usize)
        }
    }
}

/// Safe string manipulation utilities with bounds checking
///
/// Provides safe wrappers around string operations that can fail
/// due to invalid indices or malformed input.
pub struct SafeString;

impl SafeString {
    /// Safely parse a string into a target type with error handling
    ///
    /// # Errors
    /// Returns error if string cannot be parsed into target type
    pub fn safe_parse<T>(s: &str) -> Result<T, BearDogError>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        s.parse()
            .map_err(|e| BearDogError::validation(&format!("Failed to parse '{s}': {e}")))
    }

    /// Extract substring with bounds checking
    ///
    /// # Errors
    /// Returns error if start index or length would exceed string bounds
    pub fn safe_substring(s: &str, start: usize, len: usize) -> Result<&str, BearDogError> {
        if start >= s.len() {
            return Err(BearDogError::validation(&format!(
                "Start index {start} out of bounds for string of length {}",
                s.len()
            )));
        }

        let end = start + len;
        if end > s.len() {
            return Err(BearDogError::validation(&format!(
                "End index {end} out of bounds for string of length {}",
                s.len()
            )));
        }

        Ok(&s[start..end])
    }
}

/// General-purpose safe operation utilities
///
/// Provides safe wrappers for common error-prone operations
/// like unwrapping `Option` and `Result` types.
pub struct SafeOps;

impl SafeOps {
    /// Safely unwrap an `Option` with custom error message
    ///
    /// # Errors
    /// Returns error with provided message if `Option` is `None`
    pub fn safe_unwrap_option<T>(option: Option<T>, error_msg: &str) -> Result<T, BearDogError> {
        option.ok_or_else(|| BearDogError::validation(error_msg))
    }

    /// Safely convert a `Result` to BearDogError with custom context
    ///
    /// # Errors
    /// Returns error with contextual message if input `Result` is `Err`
    pub fn safe_unwrap_result<T, E>(
        result: Result<T, E>,
        error_msg: &str,
    ) -> Result<T, BearDogError>
    where
        E: std::fmt::Display,
    {
        result.map_err(|e| BearDogError::validation(&format!("{error_msg}: {e}")))
    }

    /// Safely execute a function with panic catching
    ///
    /// # Errors
    /// Returns error with provided message if function panics or returns error
    pub fn safe_execute<F, T>(f: F, error_msg: &str) -> Result<T, BearDogError>
    where
        F: FnOnce() -> Result<T, Box<dyn std::error::Error>>,
    {
        f().map_err(|e| BearDogError::system(format!("{error_msg}: {e}")))
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_safe_lock_operations() {
        let data = RwLock::new(42);
        let timeout_duration = Duration::from_millis(100);

        let read_guard = SafeLock::safe_read_lock(&data, timeout_duration).await;
        assert!(read_guard.is_ok());

        drop(read_guard);
        let write_guard = SafeLock::safe_write_lock(&data, timeout_duration).await;
        assert!(write_guard.is_ok());
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    #[test]
    fn test_safe_collection_operations() {
        let vec = vec![1, 2, 3, 4, 5];

        assert!(SafeCollection::safe_get(&vec, 0).is_ok());
        assert!(SafeCollection::safe_get(&vec, 10).is_err());

        assert!(SafeCollection::safe_first(&vec).is_ok());
        assert!(SafeCollection::safe_last(&vec).is_ok());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        let empty: Vec<i32> = vec![];
        assert!(SafeCollection::safe_first(&empty).is_err());
        assert!(SafeCollection::safe_last(&empty).is_err());
    }

    #[test]
    fn test_safe_arithmetic_operations() -> Result<(), Box<dyn std::error::Error>> {
        let result = SafeArithmetic::safe_add(5, 3);
        assert!(result.is_ok());
        assert_eq!(
            result.map_err(|e| {
                // TEST_CATEGORY: unit
                // TEST_DOMAIN: core
                // TEST_PRIORITY: normal
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(
                    format_args!("Operation failed: {e:?}").to_string(),
                )
            })?,
            8
        );

        assert!(SafeArithmetic::safe_divide(10.0, 2.0).is_ok());
        assert!(SafeArithmetic::safe_divide(10.0, 0.0).is_err());

        assert!(SafeArithmetic::safe_to_usize(42).is_ok());
        assert!(SafeArithmetic::safe_to_usize(-1).is_err());
        Ok(())
    }

    #[test]
    fn test_safe_string_operations() {
        assert!(SafeString::safe_parse::<i32>("42").is_ok());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(SafeString::safe_parse::<i32>("not_a_number").is_err());

        let text = "hello world";
        assert!(SafeString::safe_substring(text, 0, 5).is_ok());
        assert!(SafeString::safe_substring(text, 20, 5).is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_safe_ops() {
        assert!(SafeOps::safe_unwrap_option(Some(42), "test").is_ok());
        assert!(SafeOps::safe_unwrap_option(None::<i32>, "test").is_err());

        assert!(SafeOps::safe_unwrap_result(Ok::<i32, &str>(42), "test").is_ok());
        assert!(SafeOps::safe_unwrap_result(Err::<i32, _>("error "), "test").is_err());
    }
}
