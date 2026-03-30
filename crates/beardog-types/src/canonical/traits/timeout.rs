// SPDX-License-Identifier: AGPL-3.0-only

//! # Timeout Policy Trait
//!
//! This module provides a polymorphic interface for timeout configurations across different
//! domains (networking, HSM operations, service discovery, AI operations) while preserving
//! their unique timeout requirements.
//!
//! ## Design Rationale
//!
//! Rather than forcing all timeout configs into a single struct, we provide a common
//! trait interface that enables:
//! - **Polymorphic timeout handling**: Functions that work with any timeout config
//! - **Domain preservation**: Each config retains its domain-specific timeouts
//! - **Type safety**: Compiler-enforced timeout checks
//! - **Easy extension**: New timeout configs just implement the trait
//!
//! ## Example Usage
//!
//! ```rust
//! use beardog_types::canonical::traits::TimeoutPolicy;
//! use std::time::Duration;
//!
//! fn execute_with_timeout<T: TimeoutPolicy>(
//!     policy: &T,
//!     operation: &str
//! ) -> Result<(), String> {
//!     let timeout = policy.operation_timeout(operation);
//!     
//!     if policy.should_timeout(timeout, operation) {
//!         return Err(format!("Operation '{}' would timeout", operation));
//!     }
//!     
//!     // Execute operation with timeout...
//!     Ok(())
//! }
//! ```

use std::time::Duration;

/// Trait for timeout policies
///
/// Provides a common interface for timeout settings across different domains while
/// allowing each implementation to maintain domain-specific timeout requirements.
///
/// ## Timeout Types
///
/// - **Connection Timeout**: Time to establish a connection
/// - **Operation Timeout**: Time for a specific operation to complete
/// - **Global Timeout**: Maximum time for any operation
///
/// ## Thread Safety
///
/// The trait requires `Send + Sync` to enable use in async contexts and across threads.
pub trait TimeoutPolicy: Send + Sync {
    /// Timeout for establishing a connection
    ///
    /// This is the maximum time to wait when connecting to a remote service.
    /// Returns `Duration::ZERO` if connections should fail immediately,
    /// or a reasonable duration based on the domain.
    ///
    /// ## Typical Values
    /// - Local/LAN: 1-5 seconds
    /// - WAN: 5-15 seconds
    /// - Unreliable networks: 15-30 seconds
    fn connection_timeout(&self) -> Duration;

    /// Timeout for a specific operation
    ///
    /// Returns the timeout duration for a named operation.
    /// Different operations may have different timeout requirements.
    ///
    /// ## Common Operations
    /// - "read" - Reading data from a connection
    /// - "write" - Writing data to a connection
    /// - "sign" - Cryptographic signing (HSM)
    /// - "verify" - Cryptographic verification
    /// - "discovery" - Service discovery
    /// - "`health_check`" - Health check probe
    ///
    /// ## Parameters
    /// - `operation`: Name of the operation
    ///
    /// ## Returns
    /// The timeout duration for the specified operation.
    /// Implementations should provide sensible defaults for unknown operations.
    fn operation_timeout(&self, operation: &str) -> Duration;

    /// Check if elapsed time exceeds timeout for operation
    ///
    /// Returns `true` if the given elapsed time has exceeded the timeout
    /// for the specified operation.
    ///
    /// ## Parameters
    /// - `elapsed`: Time elapsed since operation started
    /// - `operation`: Name of the operation being checked
    ///
    /// ## Default Implementation
    /// The default implementation compares `elapsed >= operation_timeout(operation)`.
    fn should_timeout(&self, elapsed: Duration, operation: &str) -> bool {
        elapsed >= self.operation_timeout(operation)
    }

    /// Global timeout (maximum for any operation)
    ///
    /// Returns the maximum timeout that applies to all operations.
    /// If an operation would exceed this, it should be rejected.
    ///
    /// ## Returns
    /// - `Some(duration)` if there's a global timeout limit
    /// - `None` if no global limit (operations use their specific timeouts)
    ///
    /// ## Use Cases
    /// - Resource-constrained systems (prevent operations from blocking too long)
    /// - Circuit breakers (fail fast if global timeout exceeded)
    /// - SLA enforcement (ensure operations complete within bounds)
    fn global_timeout(&self) -> Option<Duration> {
        None // No global timeout by default
    }

    /// Read timeout (convenience method)
    ///
    /// Shorthand for `operation_timeout("read")`.
    /// Override this if your config has a dedicated read timeout field.
    fn read_timeout(&self) -> Duration {
        self.operation_timeout("read")
    }

    /// Write timeout (convenience method)
    ///
    /// Shorthand for `operation_timeout("write")`.
    /// Override this if your config has a dedicated write timeout field.
    fn write_timeout(&self) -> Duration {
        self.operation_timeout("write")
    }

    /// Idle timeout
    ///
    /// Returns how long a connection can remain idle before being closed.
    /// Returns `None` if connections never timeout due to idleness.
    ///
    /// ## Typical Values
    /// - Connection pools: 60-300 seconds
    /// - Long-lived connections: 300-3600 seconds
    /// - No timeout: `None`
    fn idle_timeout(&self) -> Option<Duration> {
        None // No idle timeout by default
    }

    /// Calculate remaining time for an operation
    ///
    /// Given the elapsed time, calculates how much time remains before timeout.
    /// Returns `Duration::ZERO` if the operation has already timed out.
    ///
    /// ## Parameters
    /// - `elapsed`: Time elapsed since operation started
    /// - `operation`: Name of the operation
    ///
    /// ## Returns
    /// Remaining time before timeout, or `Duration::ZERO` if already timed out.
    fn remaining_time(&self, elapsed: Duration, operation: &str) -> Duration {
        let timeout = self.operation_timeout(operation);
        timeout.saturating_sub(elapsed)
    }

    /// Check if configuration is valid
    ///
    /// Validates that timeouts are reasonable and consistent.
    /// Returns `Ok(())` if valid, or `Err(String)` with error description.
    ///
    /// ## Validation Rules
    /// - All timeouts should be > 0 (unless explicitly allowing immediate failure)
    /// - Global timeout (if set) should be >= longest operation timeout
    /// - Connection timeout should generally be shorter than operation timeouts
    // EVOLUTION: migrate to `Result<(), beardog_errors::BearDogError>` in a semver-major release
    // once all implementors and call sites are updated (wide trait surface).
    ///
    /// # Errors
    ///
    /// Returns an error if global or per-operation timeouts are zero or inconsistent.
    fn validate(&self) -> Result<(), String> {
        // Check connection timeout is reasonable
        let conn_timeout = self.connection_timeout();
        if conn_timeout == Duration::ZERO {
            eprintln!("WARNING: Connection timeout is zero (immediate failure)");
        } else if conn_timeout > Duration::from_secs(300) {
            eprintln!(
                "WARNING: Connection timeout is very long ({conn_timeout:?}), \
                 consider reducing for better responsiveness"
            );
        }

        // Check global timeout if set
        if let Some(global) = self.global_timeout() {
            if global == Duration::ZERO {
                return Err("Global timeout cannot be zero".to_string());
            }

            // Global should be reasonable
            if global < Duration::from_secs(1) {
                return Err(format!(
                    "Global timeout ({global:?}) is too short for most operations"
                ));
            }
        }

        Ok(())
    }

    /// Returns true if the configuration is suitable for production
    ///
    /// Production configurations should have:
    /// - Reasonable timeouts (not too short, not too long)
    /// - Consistent timeout hierarchy
    /// - Appropriate for expected latencies
    fn is_production_ready(&self) -> bool {
        // Basic validation
        if self.validate().is_err() {
            return false;
        }

        // Check for reasonable values
        let conn_timeout = self.connection_timeout();
        if conn_timeout == Duration::ZERO || conn_timeout > Duration::from_secs(30) {
            return false; // Too short or too long
        }

        // Read/write should be reasonable
        let read = self.read_timeout();
        let write = self.write_timeout();
        if read < Duration::from_secs(1) || write < Duration::from_secs(1) {
            return false; // Too short for production
        }

        if read > Duration::from_secs(600) || write > Duration::from_secs(600) {
            return false; // Too long (>10 minutes)
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test-only [`TimeoutPolicy`] implementation.
    struct TestTimeoutPolicy {
        connection: Duration,
        default_op: Duration,
        global: Option<Duration>,
    }

    impl TimeoutPolicy for TestTimeoutPolicy {
        fn connection_timeout(&self) -> Duration {
            self.connection
        }

        fn operation_timeout(&self, operation: &str) -> Duration {
            match operation {
                "fast" => Duration::from_secs(5),
                "slow" => Duration::from_secs(60),
                _ => self.default_op,
            }
        }

        fn global_timeout(&self) -> Option<Duration> {
            self.global
        }
    }

    #[test]
    fn test_basic_timeouts() {
        let policy = TestTimeoutPolicy {
            connection: Duration::from_secs(10),
            default_op: Duration::from_secs(30),
            global: None,
        };

        assert_eq!(policy.connection_timeout(), Duration::from_secs(10));
        assert_eq!(policy.operation_timeout("unknown"), Duration::from_secs(30));
        assert_eq!(policy.operation_timeout("fast"), Duration::from_secs(5));
        assert_eq!(policy.operation_timeout("slow"), Duration::from_secs(60));
    }

    #[test]
    fn test_should_timeout() {
        let policy = TestTimeoutPolicy {
            connection: Duration::from_secs(10),
            default_op: Duration::from_secs(30),
            global: None,
        };

        // Not timed out
        assert!(!policy.should_timeout(Duration::from_secs(4), "fast"));
        assert!(!policy.should_timeout(Duration::from_secs(29), "unknown"));

        // Timed out
        assert!(policy.should_timeout(Duration::from_secs(5), "fast"));
        assert!(policy.should_timeout(Duration::from_secs(30), "unknown"));
        assert!(policy.should_timeout(Duration::from_secs(100), "slow"));
    }

    #[test]
    fn test_remaining_time() {
        let policy = TestTimeoutPolicy {
            connection: Duration::from_secs(10),
            default_op: Duration::from_secs(30),
            global: None,
        };

        // Time remaining
        assert_eq!(
            policy.remaining_time(Duration::from_secs(2), "fast"),
            Duration::from_secs(3)
        );
        assert_eq!(
            policy.remaining_time(Duration::from_secs(20), "unknown"),
            Duration::from_secs(10)
        );

        // Already timed out
        assert_eq!(
            policy.remaining_time(Duration::from_secs(6), "fast"),
            Duration::ZERO
        );
        assert_eq!(
            policy.remaining_time(Duration::from_secs(100), "slow"),
            Duration::ZERO
        );
    }

    #[test]
    fn test_global_timeout() {
        let policy = TestTimeoutPolicy {
            connection: Duration::from_secs(10),
            default_op: Duration::from_secs(30),
            global: Some(Duration::from_secs(120)),
        };

        assert_eq!(policy.global_timeout(), Some(Duration::from_secs(120)));
    }

    #[test]
    fn test_convenience_methods() {
        let policy = TestTimeoutPolicy {
            connection: Duration::from_secs(10),
            default_op: Duration::from_secs(30),
            global: None,
        };

        // Should use operation_timeout internally
        assert_eq!(policy.read_timeout(), policy.operation_timeout("read"));
        assert_eq!(policy.write_timeout(), policy.operation_timeout("write"));
    }

    #[test]
    fn test_validation() {
        // Valid policy
        let valid = TestTimeoutPolicy {
            connection: Duration::from_secs(10),
            default_op: Duration::from_secs(30),
            global: Some(Duration::from_secs(120)),
        };
        assert!(valid.validate().is_ok());

        // Invalid: zero global timeout
        let invalid = TestTimeoutPolicy {
            connection: Duration::from_secs(10),
            default_op: Duration::from_secs(30),
            global: Some(Duration::ZERO),
        };
        assert!(invalid.validate().is_err());

        // Invalid: very short global timeout
        let invalid2 = TestTimeoutPolicy {
            connection: Duration::from_secs(10),
            default_op: Duration::from_secs(30),
            global: Some(Duration::from_millis(100)),
        };
        assert!(invalid2.validate().is_err());
    }

    #[test]
    fn test_production_readiness() {
        // Production ready
        let prod = TestTimeoutPolicy {
            connection: Duration::from_secs(5),
            default_op: Duration::from_secs(30),
            global: None,
        };
        assert!(prod.is_production_ready());

        // Not production ready: zero connection timeout
        let not_prod = TestTimeoutPolicy {
            connection: Duration::ZERO,
            default_op: Duration::from_secs(30),
            global: None,
        };
        assert!(!not_prod.is_production_ready());

        // Not production ready: too long connection timeout
        let not_prod2 = TestTimeoutPolicy {
            connection: Duration::from_secs(100),
            default_op: Duration::from_secs(30),
            global: None,
        };
        assert!(!not_prod2.is_production_ready());
    }

    #[test]
    fn test_idle_timeout_default() {
        let policy = TestTimeoutPolicy {
            connection: Duration::from_secs(10),
            default_op: Duration::from_secs(30),
            global: None,
        };

        // Default should be None
        assert_eq!(policy.idle_timeout(), None);
    }
}
