// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};
use thiserror::Error;

// Explicit re-exports instead of glob import for pedantic compliance
pub use crate::categories::{
    ApiErrorCategory, BusinessErrorCategory, ConfigurationErrorCategory, HsmErrorCategory,
    NetworkErrorCategory, SecurityErrorCategory, SystemErrorCategory, TestingErrorCategory,
    WorkflowErrorCategory,
};

///
/// This enum provides a comprehensive error taxonomy covering all domains
/// within the `BearDog` system. Each variant includes detailed categorization
/// and remediation.
///
/// # Design Philosophy
///
/// - **Domain Categorization**: Errors are organized by functional domain
/// - **Rich Context**: Each error includes detailed categorization
/// - **Zero-Cost Abstractions**: Efficient error propagation patterns
///
/// # Usage Examples
///
/// ```rust
/// use beardog_errors::BearDogError;
///
/// // Create domain-specific errors
/// let _security_error = BearDogError::security("Authentication failed".to_string());
/// let _system_error = BearDogError::system("Database connection lost".to_string());
/// let _business_error = BearDogError::business("Invalid user input".to_string());
/// ```
#[derive(Error, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BearDogError {
    /// Security-related errors including authentication, authorization, and cryptographic operations
    ///
    /// This variant covers all security-related failures within the `BearDog` ecosystem,
    /// from authentication and authorization to cryptographic operations and HSM interactions.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the security error
    #[error("Security error: {message}")]
    Security {
        /// Human-readable description of the security error
        message: String,
        /// Specific category of security error for detailed classification
        #[serde(default)]
        category: SecurityErrorCategory,
    },

    ///
    /// This variant encompasses all system-level failures including resource exhaustion,
    /// file system operations, memory management, and operating system interactions.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the system error
    #[error("System error: {message}")]
    System {
        /// Human-readable description of the system error
        message: String,
        /// Specific category of system error for detailed classification
        #[serde(default)]
        category: SystemErrorCategory,
    },

    /// Business logic and application-level errors
    ///
    /// This variant covers all business rule violations, validation failures,
    /// workflow errors, and application-specific logic problems.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the business error
    #[error("Business error: {message}")]
    Business {
        /// Human-readable description of the business error
        message: String,
        /// Specific category of business error for detailed classification
        #[serde(default)]
        category: BusinessErrorCategory,
    },

    /// Network communication and connectivity errors
    ///
    /// This variant encompasses all network-related failures including connection
    /// timeouts, DNS resolution, SSL/TLS issues, and protocol-level problems.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the network error
    #[error("Network error: {message}")]
    Network {
        /// Human-readable description of the network error
        message: String,
        /// Specific category of network error for detailed classification
        #[serde(default)]
        category: NetworkErrorCategory,
    },

    /// Configuration and setup errors
    ///
    /// This variant covers all configuration-related problems including parsing
    /// failures, missing required settings, invalid values, and environment issues.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the configuration error
    #[error("Configuration error: {message}")]
    Configuration {
        /// Human-readable description of the configuration error
        message: String,
        /// Specific category of configuration error for detailed classification
        #[serde(default)]
        category: ConfigurationErrorCategory,
    },

    /// System initialization and startup errors
    ///
    /// This variant represents failures that occur during system startup,
    /// component initialization, or bootstrap processes.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the initialization error
    #[error("Initialization error: {message}")]
    Initialization {
        /// Human-readable description of the initialization error
        message: String,
    },

    /// Hardware Security Module (HSM) errors
    ///
    /// This variant covers all HSM-related failures including key generation,
    /// cryptographic operations, hardware communication, and capacity issues.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the HSM error
    #[error("HSM error: {message}")]
    Hsm {
        /// Human-readable description of the HSM error
        message: String,
        /// Specific category of HSM error for detailed classification
        #[serde(default)]
        category: HsmErrorCategory,
    },

    /// API and service interface errors
    ///
    /// This variant encompasses all API-related failures including HTTP errors,
    /// service unavailability, rate limiting, and endpoint-specific issues.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the API error
    /// * `status_code` - HTTP status code if applicable
    /// * `endpoint` - API endpoint where the error occurred if applicable
    #[error("API error: {message}")]
    Api {
        /// Human-readable description of the API error
        message: String,
        /// Specific category of API error for detailed classification
        #[serde(default)]
        category: ApiErrorCategory,
        /// HTTP status code if applicable
        status_code: Option<u16>,
        /// API endpoint where the error occurred if applicable
        endpoint: Option<String>,
    },

    /// Workflow execution and management errors
    ///
    /// This variant covers all workflow-related failures including execution
    /// errors, approval process issues, state transition problems, and timeouts.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the workflow error
    #[error("Workflow error: {message}")]
    Workflow {
        /// Human-readable description of the workflow error
        message: String,
        /// Specific category of workflow error for detailed classification
        #[serde(default)]
        category: WorkflowErrorCategory,
    },

    /// Genetics and AI-related errors
    ///
    /// This variant encompasses all genetics system failures including
    /// spawning errors, evolution problems, and AI operation issues.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the genetics error
    #[error("Genetics error: {message}")]
    Genetics {
        /// Human-readable description of the genetics error
        message: String,
    },

    /// Deployment and infrastructure errors
    ///
    /// This variant covers all deployment-related failures including
    /// provisioning errors, infrastructure setup issues, and deployment validation problems.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the deployment error
    #[error("Deployment error: {message}")]
    Deployment {
        /// Human-readable description of the deployment error
        message: String,
    },

    /// Memory management and allocation errors
    ///
    /// This variant encompasses all memory-related failures including
    /// allocation failures, memory exhaustion, and memory safety violations.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the memory error
    #[error("Memory error: {message}")]
    Memory {
        /// Human-readable description of the memory error
        message: String,
    },

    /// Monitoring and observability errors
    ///
    /// This variant covers all monitoring system failures including
    /// metric collection issues, alerting problems, and observability infrastructure errors.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the monitoring error
    #[error("Monitoring error: {message}")]
    Monitoring {
        /// Human-readable description of the monitoring error
        message: String,
    },

    /// Compliance and regulatory errors
    ///
    /// This variant encompasses all compliance-related failures including
    /// regulatory violations, audit failures, and policy compliance issues.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the compliance error
    #[error("Compliance error: {message}")]
    Compliance {
        /// Human-readable description of the compliance error
        message: String,
    },

    /// Cryptographic operation errors
    ///
    /// This variant covers all cryptographic failures including
    /// encryption/decryption errors, key management issues, and cryptographic protocol problems.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the cryptographic error
    #[error("Cryptographic error: {message}")]
    Cryptographic {
        /// Human-readable description of the cryptographic error
        message: String,
    },

    /// Tunnel and secure communication errors
    ///
    /// This variant encompasses all secure tunnel failures including
    /// tunnel establishment errors, secure communication problems, and protocol issues.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the tunnel error
    #[error("Tunnel error: {message}")]
    Tunnel {
        /// Human-readable description of the tunnel error
        message: String,
    },

    /// Adapter and integration errors
    ///
    /// This variant covers all adapter-related failures including
    /// integration issues, protocol translation problems, and compatibility errors.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the adapter error
    #[error("Adapter error: {message}")]
    Adapter {
        /// Human-readable description of the adapter error
        message: String,
    },

    /// Testing and validation errors
    ///
    /// This variant encompasses all testing framework failures including
    /// property-based testing, mutation testing, invariant validation, and test execution issues.
    ///
    /// # Fields
    /// * `message` - Human-readable description of the testing error
    #[error("Testing error: {message}")]
    Testing {
        /// Human-readable description of the testing error
        message: String,
        /// Specific category of testing error for detailed classification
        #[serde(default)]
        category: TestingErrorCategory,
    },
}

impl BearDogError {
    /// Create a security error with default authentication category
    #[must_use]
    pub const fn security(message: String) -> Self {
        Self::Security {
            message,
            category: SecurityErrorCategory::Authentication,
        }
    }

    /// Create a system error with default general category
    #[must_use]
    pub const fn system(message: String) -> Self {
        Self::System {
            message,
            category: SystemErrorCategory::General,
        }
    }

    /// Create a business error with default validation category
    #[must_use]
    pub const fn business(message: String) -> Self {
        Self::Business {
            message,
            category: BusinessErrorCategory::Validation,
        }
    }

    /// Create a network error (as system error)
    #[must_use]
    pub const fn network(message: String) -> Self {
        Self::System {
            message,
            category: SystemErrorCategory::General,
        }
    }

    /// Create a validation error (as business error)
    #[must_use]
    pub fn validation(message: &str) -> Self {
        Self::Business {
            message: message.to_string(),
            category: BusinessErrorCategory::Validation,
        }
    }

    /// Create an internal error (as system error)
    #[must_use]
    pub const fn internal(message: String) -> Self {
        Self::System {
            message,
            category: SystemErrorCategory::Internal,
        }
    }

    /// Create a configuration error (as system error)
    #[must_use]
    pub fn configuration(message: &str) -> Self {
        Self::System {
            message: message.to_string(),
            category: SystemErrorCategory::General,
        }
    }

    /// Create an invalid input error (as business error)
    #[must_use]
    pub fn invalid_input(message: &str) -> Self {
        Self::Business {
            message: message.to_string(),
            category: BusinessErrorCategory::Validation,
        }
    }

    /// Create a not found error (as business error)
    #[must_use]
    pub const fn not_found(message: String) -> Self {
        Self::Business {
            message,
            category: BusinessErrorCategory::General,
        }
    }

    /// Create an unavailable error (as system error)
    #[must_use]
    pub const fn unavailable(message: String) -> Self {
        Self::System {
            message,
            category: SystemErrorCategory::General,
        }
    }

    /// Create an unauthorized error (as security error)
    #[must_use]
    pub const fn unauthorized(message: String) -> Self {
        Self::Security {
            message,
            category: SecurityErrorCategory::Authorization,
        }
    }

    /// Create an API error
    #[must_use]
    pub const fn api(message: String) -> Self {
        Self::Api {
            message,
            category: ApiErrorCategory::General,
            status_code: None,
            endpoint: None,
        }
    }

    /// Create a workflow error
    #[must_use]
    pub const fn workflow(message: String) -> Self {
        Self::Workflow {
            message,
            category: WorkflowErrorCategory::Execution,
        }
    }

    /// Create a genetics error
    #[must_use]
    pub const fn genetics(message: String) -> Self {
        Self::Genetics { message }
    }

    /// Create an initialization error (as system error)
    #[must_use]
    pub const fn initialization(message: String) -> Self {
        Self::System {
            message,
            category: SystemErrorCategory::General,
        }
    }

    /// Create an HSM error (as cryptographic error)
    #[must_use]
    pub const fn hsm(message: String) -> Self {
        Self::Cryptographic { message }
    }

    /// Create a security error with specific category
    #[must_use]
    pub fn security_with_category(message: &str, category: SecurityErrorCategory) -> Self {
        Self::Security {
            message: message.to_string(),
            category,
        }
    }

    /// Create a system error with specific category
    #[must_use]
    pub fn system_with_category(message: &str, category: SystemErrorCategory) -> Self {
        Self::System {
            message: message.to_string(),
            category,
        }
    }

    /// Create a business error with specific category
    #[must_use]
    pub fn business_with_category(message: &str, category: BusinessErrorCategory) -> Self {
        Self::Business {
            message: message.to_string(),
            category,
        }
    }

    /// Create a testing error with default general category
    #[must_use]
    pub fn testing(message: &str) -> Self {
        Self::Testing {
            message: message.to_string(),
            category: TestingErrorCategory::General,
        }
    }

    /// Create a testing error with specific category
    #[must_use]
    pub fn testing_with_category(message: &str, category: TestingErrorCategory) -> Self {
        Self::Testing {
            message: message.to_string(),
            category,
        }
    }

    /// Create an unsupported operation error
    #[must_use]
    pub fn unsupported_operation(operation: String) -> Self {
        Self::Business {
            message: format!("Unsupported operation: {operation}"),
            category: BusinessErrorCategory::Validation,
        }
    }

    /// Create a not implemented error
    #[must_use]
    pub fn not_implemented(feature: String) -> Self {
        Self::Business {
            message: format!("Not yet implemented: {feature}"),
            category: BusinessErrorCategory::Validation,
        }
    }

    /// Create an I/O error
    #[must_use]
    pub fn io_error(details: String) -> Self {
        Self::System {
            message: format!("I/O operation failed: {details}"),
            category: SystemErrorCategory::FileSystem,
        }
    }

    /// Create a cryptographic error
    #[must_use]
    pub fn crypto_error(details: String) -> Self {
        Self::Cryptographic {
            message: format!("Cryptographic operation failed: {details}"),
        }
    }

    /// Create a serialization error
    #[must_use]
    pub fn serialization(details: String) -> Self {
        Self::System {
            message: format!("Serialization failed: {details}"),
            category: SystemErrorCategory::General,
        }
    }
}

// ============================================================================
// FROM TRAIT IMPLEMENTATIONS - Automatic Error Conversions
// ============================================================================

/// Convert `std::io::Error` to `BearDogError::System`
impl From<std::io::Error> for BearDogError {
    fn from(err: std::io::Error) -> Self {
        Self::System {
            message: format!("IO error: {err}"),
            category: SystemErrorCategory::FileSystem,
        }
    }
}

/// Convert `std::fmt::Error` to `BearDogError::System`
impl From<std::fmt::Error> for BearDogError {
    fn from(err: std::fmt::Error) -> Self {
        Self::System {
            message: format!("Formatting error: {err}"),
            category: SystemErrorCategory::General,
        }
    }
}

// Result<T, BearDogError> type alias has been removed in favor of idiomatic Result<T, BearDogError>
//
// Migration completed! All code should now use Result<T, BearDogError> directly.
// This provides better IDE support, clearer error types, and follows Rust conventions.
//
// For examples of the new patterns, see the crate documentation.
