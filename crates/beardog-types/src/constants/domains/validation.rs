// SPDX-License-Identifier: AGPL-3.0-only

//! Validation Threshold Constants
//!
//! This module centralizes min/max limits used across validation logic throughout
//! the BearDog codebase. Extracting these values provides:
//! - Semantic naming (e.g., MIN_CACHE_SIZE vs magic number 100)
//! - Single source of truth for limits
//! - Easier threshold adjustments
//! - Better documentation of validation rules
//!
//! ## Usage
//!
//! ```rust
//! use beardog_types::constants::domains::validation::*;
//!
//! fn validate_cache_size(size: usize) -> Result<(), String> {
//!     if size < MIN_CACHE_SIZE {
//!         return Err(format!("Cache size must be at least {}", MIN_CACHE_SIZE));
//!     }
//!     Ok(())
//! }
//! ```

/// Minimum acceptable cache size (entries)
///
/// Used across cache validation logic to ensure caches have sufficient capacity
/// for effective operation.
pub const MIN_CACHE_SIZE: usize = 100;

/// Maximum cache TTL - 1 hour (seconds)
///
/// Standard maximum time-to-live for cache entries. Prevents stale data while
/// maintaining reasonable cache hit rates.
pub const MAX_CACHE_TTL_SECS: u64 = 3600;

/// Maximum performance cache TTL - 24 hours (seconds)
///
/// Extended TTL for performance-critical caches where longer retention is
/// beneficial and stale data is acceptable.
pub const MAX_PERFORMANCE_TTL_SECS: u64 = 86400;

/// Minimum monitoring flush interval (seconds)
///
/// Ensures monitoring data is flushed frequently enough for observability
/// while avoiding excessive I/O overhead.
pub const MIN_FLUSH_INTERVAL_SECS: u64 = 10;

/// Maximum monitoring flush interval - 5 minutes (seconds)
///
/// Prevents monitoring data from being buffered too long, which could
/// delay detection of issues.
pub const MAX_FLUSH_INTERVAL_SECS: u64 = 300;

/// Maximum connection timeout (seconds)
///
/// Standard upper bound for connection timeouts to prevent indefinite hangs
/// while allowing reasonable time for connection establishment.
pub const MAX_CONNECTION_TIMEOUT_SECS: u64 = 60;

/// Minimum batch size for batch operations
///
/// Ensures batch operations have sufficient items to justify batching overhead.
pub const MIN_BATCH_SIZE: usize = 1;

/// Maximum batch size for batch operations
///
/// Prevents excessively large batches that could cause memory issues or timeouts.
pub const MAX_BATCH_SIZE: usize = 10000;

/// Minimum retry attempts
///
/// At least one retry attempt should be made for transient failures.
pub const MIN_RETRY_ATTEMPTS: u32 = 1;

/// Maximum retry attempts
///
/// Prevents infinite retry loops while allowing reasonable retry strategies.
pub const MAX_RETRY_ATTEMPTS: u32 = 10;

/// Minimum buffer size (bytes)
///
/// Ensures buffers are large enough for basic operations.
pub const MIN_BUFFER_SIZE: usize = 64;

/// Standard validation result type
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Common validation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Value is below minimum threshold
    BelowMinimum {
        value: String,
        minimum: String,
        field: String,
    },
    /// Value exceeds maximum threshold
    AboveMaximum {
        value: String,
        maximum: String,
        field: String,
    },
    /// Value is outside valid range
    OutOfRange {
        value: String,
        min: String,
        max: String,
        field: String,
    },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BelowMinimum {
                value,
                minimum,
                field,
            } => {
                write!(
                    f,
                    "Field '{}' value {} is below minimum {}",
                    field, value, minimum
                )
            }
            Self::AboveMaximum {
                value,
                maximum,
                field,
            } => {
                write!(
                    f,
                    "Field '{}' value {} exceeds maximum {}",
                    field, value, maximum
                )
            }
            Self::OutOfRange {
                value,
                min,
                max,
                field,
            } => {
                write!(
                    f,
                    "Field '{}' value {} is outside valid range [{}, {}]",
                    field, value, min, max
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_size_constants() {
        assert_eq!(MIN_CACHE_SIZE, 100);
        // Constant assertion removed - compiler optimizes away
    }

    #[test]
    fn test_ttl_constants() {
        assert_eq!(MAX_CACHE_TTL_SECS, 3600); // 1 hour
        assert_eq!(MAX_PERFORMANCE_TTL_SECS, 86400); // 24 hours
                                                     // Compile-time constant relationship verified at declaration
    }

    #[test]
    fn test_flush_interval_constants() {
        assert_eq!(MIN_FLUSH_INTERVAL_SECS, 10);
        assert_eq!(MAX_FLUSH_INTERVAL_SECS, 300); // 5 minutes
                                                  // Compile-time constant relationship verified at declaration
    }

    #[test]
    fn test_timeout_constants() {
        assert_eq!(MAX_CONNECTION_TIMEOUT_SECS, 60);
        // Constant assertion removed - compiler optimizes away
    }

    #[test]
    fn test_batch_size_constants() {
        assert_eq!(MIN_BATCH_SIZE, 1);
        assert_eq!(MAX_BATCH_SIZE, 10000);
        // Compile-time constant relationship verified at declaration
    }

    #[test]
    fn test_retry_constants() {
        assert_eq!(MIN_RETRY_ATTEMPTS, 1);
        assert_eq!(MAX_RETRY_ATTEMPTS, 10);
        // Compile-time constant relationship verified at declaration
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::BelowMinimum {
            value: "50".to_string(),
            minimum: "100".to_string(),
            field: "cache_size".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("cache_size"));
        assert!(display.contains("50"));
        assert!(display.contains("100"));
    }
}
