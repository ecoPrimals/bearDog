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


/// Error Categories - Modular Error Organization
///
/// **REFACTORED ERROR ARCHITECTURE**
/// This module provides a clean, modular organization of BearDog error types
/// that were previously in a single 1429-line enum. The refactoring improves
/// maintainability while preserving all error handling capabilities.
/// ## Module Organization
/// - **`security`**: Security-related errors (crypto, auth, HSM, threats)
/// - **`system`**: System and infrastructure errors (network, storage, config)
/// - **`business`**: Business logic errors (validation, workflows, compliance)
/// - **`unified`**: Unified error enum that encompasses all categories
/// ## Migration Guide
/// ```rust
/// // OLD (monolithic):
/// use beardog_errors::BearDogError;
/// let error = BearDogError::encryption("AES", "Failed");
/// // NEW (categorized, but same interface):
/// // OR use specific categories:
/// use beardog_errors::SecurityError;
/// let error = SecurityError::encryption("AES", "Failed");
/// ```
pub mod business;
pub mod security;
pub mod system;

// Re-export all error types for convenience
pub use business::BusinessError;
pub use security::SecurityError;
pub use system::SystemError;
// ============================================================================
// ERROR CATEGORY SYSTEM - Hierarchical error organization
// ============================================================================

// Re-export the canonical types from crate root
pub use crate::error_types::ErrorSeverity;
// BearDogError is now directly available from crate root
pub use crate::BearDogError;
// Use canonical BearDogResult from crate root instead of duplicate definition
pub use crate::BearDogResult;
use crate::error_types::ErrorSeverity;
/// Error conversion utilities - Removed duplicate implementations
/// These are now handled in the main lib.rs file to avoid conflicts
pub mod conversions {
    // Conversion implementations moved to lib.rs to avoid duplication
}

/// Error analysis utilities
pub mod analysis {
    use super::*;
    
    /// Analyze error for severity and handling recommendations
    pub struct ErrorAnalysis {
        pub severity: ErrorSeverity,
        pub is_critical: bool,
        pub should_retry: bool,
        pub user_facing: bool,
    }
    
    impl BearDogError {
        /// Analyze error characteristics for handling decisions}


        pub fn analyze(&self) -> ErrorAnalysis {
            match self {
                BearDogError::Security { message: _ } => ErrorAnalysis {
                    severity: ErrorSeverity::Critical,
                    is_critical: true,
                    should_retry: false,
                    user_facing: false,
                },
                BearDogError::System { message: _ } => ErrorAnalysis {
                    severity: ErrorSeverity::High,
                    is_critical: false,
                    should_retry: true,
                    user_facing: false,
                },
                BearDogError::Business { message: _ } => ErrorAnalysis {
                    severity: ErrorSeverity::Low,
                    is_critical: false,
                    should_retry: false,
                    user_facing: true,
                },
            }
        }
        
        /// Check if error should trigger alerts
        pub fn should_alert(&self) -> bool {
            self.analyze().is_critical
        }
    }
}

/// Error handling utilities
pub mod handling {
    /// Retry configuration for recoverable errors
    pub struct RetryConfig {
        pub max_attempts: u32,
        pub base_delay_ms: u64,
        pub max_delay_ms: u64,
        pub backoff_multiplier: f64,
    }
    
    impl BearDogError {
        /// Get appropriate retry configuration for this error}


        pub fn retry_config(&self) -> Option<RetryConfig> {
            let analysis = self.analyze();
            if analysis.should_retry {
                Some(match self {
                    BearDogError::System { .. } => RetryConfig {
                        max_attempts: 5,
                        base_delay_ms: 1000,
                        max_delay_ms: 30000,
                        backoff_multiplier: 2.0,
                    },
                    BearDogError::Network { .. } => RetryConfig {
                        max_attempts: 3,
                        base_delay_ms: 500,
                        max_delay_ms: 5000,
                        backoff_multiplier: 1.5,
                    },
                    _ => RetryConfig {
                        max_attempts: 2,
                        base_delay_ms: 100,
                        max_delay_ms: 1000,
                        backoff_multiplier: 1.0,
                    },
                })
            } else {
                None
            }
        }
        
        /// Get user-friendly error message
        pub fn user_message(&self) -> String {
            match self {
                BearDogError::Security { .. } => {
                    "Authentication failed. Please check your credentials and try again."
                        .to_string()
                }
                BearDogError::System { .. } => {
                    "Network connection error. Please check your connection and try again.".to_string()
                }
                BearDogError::Business { .. } => {
                    "Invalid input provided. Please check your data and try again.".to_string()
                }
                _ => {
                    "An unexpected error occurred. Please contact support if the problem persists.".to_string()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversions() {
        let sec_error = SecurityError::authentication("Invalid token");
        let bear_error: BearDogError = sec_error.into();
        assert!(matches!(bear_error, BearDogError::Security(_)));
    }
    
    #[test]
    fn test_error_analysis() {
        let critical_error =
            BearDogError::Security(SecurityError::threat_detection("Malicious activity"));
        let analysis = critical_error.analyze();
        assert_eq!(analysis.severity, ErrorSeverity::Critical);
    }
    
    #[test]
    fn test_error_categorization() {
        let sys_error = SystemError::network("Connection timeout");
        assert_eq!(sys_error.category(), ErrorCategory::Network);
    }
    
    #[test]
    fn test_business_error() {
        let bus_error = BusinessError::validation("Invalid email format");
        let bear_error: BearDogError = bus_error.into();
        assert!(matches!(bear_error, BearDogError::Business(_)));
    }
}
