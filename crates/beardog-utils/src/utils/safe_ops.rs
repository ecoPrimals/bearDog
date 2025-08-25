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


//! Safe operations utilities for BearDog
//! 
//! This module provides safe wrappers around potentially unsafe operations
//! to ensure memory safety and prevent panics.

use beardog_errors::{BearDogError, BearDogResult};
use std::time::Duration;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tokio::time::timeout;

/// Safe lock operations
pub struct SafeLock;

impl SafeLock {
    /// Safely acquire a read lock with timeout
    pub async fn safe_read_lock<T>(
        lock: &RwLock<T>,
        timeout_duration: Duration,
    ) -> BearDogResult<RwLockReadGuard<'_, T>> {
        match timeout(timeout_duration, lock.read()).await {
            Ok(guard) => Ok(guard),
            Err(_) => Err(BearDogError::system(format!(
                "Read lock acquisition timed out after {timeout_duration:?}"
            ))),
        }
    }

    /// Safely acquire a write lock with timeout
    pub async fn safe_write_lock<T>(
        lock: &RwLock<T>,
        timeout_duration: Duration,
    ) -> BearDogResult<RwLockWriteGuard<'_, T>> {
        match timeout(timeout_duration, lock.write()).await {
            Ok(guard) => Ok(guard),
            Err(_) => Err(BearDogError::system(format!(
                "Write lock acquisition timed out after {timeout_duration:?}"
            ))),
        }
    }
}

/// Safe collection operations
pub struct SafeCollection;

impl SafeCollection {
    /// Safely get an element from a vector by index
    pub fn safe_get<T>(vec: &[T], index: usize) -> BearDogResult<&T> {
        vec.get(index).ok_or_else(|| {
            BearDogError::validation(format!(
                "Index {index} out of bounds for collection of length {}",
                vec.len()
            ))
        })
    }

    /// Safely get a mutable element from a vector by index
    pub fn safe_get_mut<T>(vec: &mut [T], index: usize) -> BearDogResult<&mut T> {
        let len = vec.len();
        vec.get_mut(index).ok_or_else(|| {
            BearDogError::validation(format!(
                "Index {index} out of bounds for collection of length {len}"
            ))
        })
    }

    /// Safely get the first element of a collection
    pub fn safe_first<T>(vec: &[T]) -> BearDogResult<&T> {
        vec.first().ok_or_else(|| {
            BearDogError::validation("Cannot get first element of empty collection")
        })
    }

    /// Safely get the last element of a collection
    pub fn safe_last<T>(vec: &[T]) -> BearDogResult<&T> {
        vec.last().ok_or_else(|| {
            BearDogError::validation("Cannot get last element of empty collection")
        })
    }
}

/// Safe arithmetic operations
pub struct SafeArithmetic;

impl SafeArithmetic {
    /// Safely add two numbers
    pub fn safe_add<T>(a: T, b: T) -> BearDogResult<T>
    where
        T: std::ops::Add<Output = T> + Copy + std::fmt::Display,
    {
        Ok(a + b)
    }

    /// Safely divide two numbers
    pub fn safe_divide(a: f64, b: f64) -> BearDogResult<f64> {
        if b == 0.0 {
            Err(BearDogError::validation("Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    /// Safely convert to usize
    pub fn safe_to_usize(value: i64) -> BearDogResult<usize> {
        if value < 0 {
            Err(BearDogError::validation(format!(
                "Cannot convert negative value {value} to usize"
            )))
        } else {
            Ok(value as usize)
        }
    }
}

/// Safe string operations
pub struct SafeString;

impl SafeString {
    /// Safely parse a string to a number
    pub fn safe_parse<T>(s: &str) -> BearDogResult<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        s.parse().map_err(|e| {
            BearDogError::validation(format!("Failed to parse '{s}': {e}"))
        })
    }

    /// Safely get a substring
    pub fn safe_substring(s: &str, start: usize, len: usize) -> BearDogResult<&str> {
        if start >= s.len() {
            return Err(BearDogError::validation(format!(
                "Start index {start} out of bounds for string of length {}",
                s.len()
            )));
        }

        let end = start + len;
        if end > s.len() {
            return Err(BearDogError::validation(format!(
                "End index {end} out of bounds for string of length {}",
                s.len()
            )));
        }

        Ok(&s[start..end])
    }
}

/// General safe operations
pub struct SafeOps;

impl SafeOps {
    /// Safely unwrap an Option with a custom error message
    pub fn safe_unwrap_option<T>(option: Option<T>, error_msg: &str) -> BearDogResult<T> {
        option.ok_or_else(|| BearDogError::validation(error_msg))
    }

    /// Safely unwrap a Result with a custom error message
    pub fn safe_unwrap_result<T, E>(result: Result<T, E>, error_msg: &str) -> BearDogResult<T>
    where
        E: std::fmt::Display,
    {
        result.map_err(|e| BearDogError::validation(format!("{error_msg}: {e}")))
    }

    /// Safely execute a closure with error handling
    pub fn safe_execute<F, T>(f: F, error_msg: &str) -> BearDogResult<T>
    where
        F: FnOnce() -> Result<T, Box<dyn std::error::Error>>,
    {
        f().map_err(|e| BearDogError::system(format!("{error_msg}: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_safe_lock_operations() {
        let data = RwLock::new(42);
        let timeout_duration = Duration::from_millis(100);

        // Test read lock
        let read_guard = SafeLock::safe_read_lock(&data, timeout_duration).await;
        assert!(read_guard.is_ok());

        // Test write lock
        drop(read_guard);
        let write_guard = SafeLock::safe_write_lock(&data, timeout_duration).await;
        assert!(write_guard.is_ok());
    }

    #[test]
    fn test_safe_collection_operations() {
        let vec = vec![1, 2, 3, 4, 5];

        // Test safe get
        assert!(SafeCollection::safe_get(&vec, 0).is_ok());
        assert!(SafeCollection::safe_get(&vec, 10).is_err());

        // Test safe first and last
        assert!(SafeCollection::safe_first(&vec).is_ok());
        assert!(SafeCollection::safe_last(&vec).is_ok());

        // Test empty collection
        let empty: Vec<i32> = vec![];
        assert!(SafeCollection::safe_first(&empty).is_err());
        assert!(SafeCollection::safe_last(&empty).is_err());
    }

    #[test]
    fn test_safe_arithmetic_operations() -> Result<(), Box<dyn std::error::Error>> {
        // Test safe add
        let result = SafeArithmetic::safe_add(5, 3);
        assert!(result.is_ok());
        assert_eq!(result.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?, 8);

        // Test safe divide
        assert!(SafeArithmetic::safe_divide(10.0, 2.0).is_ok());
        assert!(SafeArithmetic::safe_divide(10.0, 0.0).is_err());

        // Test safe to usize
        assert!(SafeArithmetic::safe_to_usize(42).is_ok());
        assert!(SafeArithmetic::safe_to_usize(-1).is_err());
        Ok(())
    }

    #[test]
    fn test_safe_string_operations() {
        // Test safe parse
        assert!(SafeString::safe_parse::<i32>("42").is_ok());
        assert!(SafeString::safe_parse::<i32>("not_a_number").is_err());

        // Test safe substring
        let text = "hello world";
        assert!(SafeString::safe_substring(text, 0, 5).is_ok());
        assert!(SafeString::safe_substring(text, 20, 5).is_err());
    }

    #[test]
    fn test_safe_ops() {
        // Test safe unwrap option
        assert!(SafeOps::safe_unwrap_option(Some(42), "test").is_ok());
        assert!(SafeOps::safe_unwrap_option(None::<i32>, "test").is_err());

        // Test safe unwrap result
        assert!(SafeOps::safe_unwrap_result(Ok::<i32, &str>(42), "test").is_ok());
        assert!(SafeOps::safe_unwrap_result(Err::<i32, _>("error"), "test").is_err());
    }
}
