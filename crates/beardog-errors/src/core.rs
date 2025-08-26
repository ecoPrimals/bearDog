

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use crate::categories::*;

/// The unified error type for the BearDog ecosystem
/// 
/// This enum provides a comprehensive error taxonomy covering all domains
/// within the BearDog system. Each variant includes detailed categorization
/// and contextual information to enable precise error handling, monitoring,
/// and remediation.
/// 
/// # Design Philosophy
/// 
/// - **Domain Categorization**: Errors are organized by functional domain
/// - **Rich Context**: Each error includes detailed categorization
/// - **Actionable Information**: Error messages provide clear guidance
/// - **Monitoring Integration**: Structured for observability systems
/// - **Zero-Cost Abstractions**: Efficient error propagation patterns
/// 
/// # Usage Examples
/// 
/// ```rust
/// use beardog_errors::BearDogError;
/// 
/// // Create domain-specific errors
/// let security_error = BearDogError::security("Authentication failed");
/// let system_error = BearDogError::system("Database connection lost");
/// let business_error = BearDogError::business("Invalid user input");
/// ```
#[derive(Error, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BearDogError {
    /// Security-related errors including authentication, authorization, and cryptographic operations
    /// 
    /// This variant covers all security-related failures within the BearDog ecosystem,
    /// from authentication and authorization to cryptographic operations and HSM interactions.
    /// 
    /// # Fields
    /// * `message` - Human-readable description of the security error
    /// * `category` - Specific security error subcategory for precise classification
    #[error("Security error: {message}")]
    Security {
        /// Human-readable description of the security error
        message: String,
        #[serde(default)]
        /// Specific security error subcategory for precise classification
        category: SecurityErrorCategory,
    },

    /// System-level errors related to infrastructure, resources, and platform operations
    /// 
    /// This variant encompasses all system-level failures including resource exhaustion,
    /// file system operations, memory management, and operating system interactions.
    /// 
    /// # Fields
    /// * `message` - Human-readable description of the system error
    /// * `category` - Specific system error subcategory for precise classification
    #[error("System error: {message}")]
    System {
        /// Human-readable description of the system error
        message: String,
        #[serde(default)]
        /// Specific system error subcategory for precise classification
        category: SystemErrorCategory,
    },

    /// Business logic and application-level errors
    /// 
    /// This variant covers all business rule violations, validation failures,
    /// workflow errors, and application-specific logic problems.
    /// 
    /// # Fields
    /// * `message` - Human-readable description of the business error
    /// * `category` - Specific business error subcategory for precise classification
    #[error("Business error: {message}")]
    Business {
        /// Human-readable description of the business error
        message: String,
        #[serde(default)]
        /// Specific business error subcategory for precise classification
        category: BusinessErrorCategory,
    },

    /// Network communication and connectivity errors
    /// 
    /// This variant encompasses all network-related failures including connection
    /// timeouts, DNS resolution, SSL/TLS issues, and protocol-level problems.
    /// 
    /// # Fields
    /// * `message` - Human-readable description of the network error
    /// * `category` - Specific network error subcategory for precise classification
    #[error("Network error: {message}")]
    Network {
        /// Human-readable description of the network error
        message: String,
        #[serde(default)]
        /// Specific network error subcategory for precise classification
        category: NetworkErrorCategory,
    },

    /// Configuration and setup errors
    /// 
    /// This variant covers all configuration-related problems including parsing
    /// failures, missing required settings, invalid values, and environment issues.
    /// 
    /// # Fields
    /// * `message` - Human-readable description of the configuration error
    /// * `category` - Specific configuration error subcategory for precise classification
    #[error("Configuration error: {message}")]
    Configuration {
        /// Human-readable description of the configuration error
        message: String,
        #[serde(default)]
        /// Specific configuration error subcategory for precise classification
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
    /// * `category` - Specific HSM error subcategory for precise classification
    #[error("HSM error: {message}")]
    Hsm {
        /// Human-readable description of the HSM error
        message: String,
        #[serde(default)]
        /// Specific HSM error subcategory for precise classification
        category: HsmErrorCategory,
    },

    /// API and service interface errors
    /// 
    /// This variant encompasses all API-related failures including HTTP errors,
    /// service unavailability, rate limiting, and endpoint-specific issues.
    /// 
    /// # Fields
    /// * `message` - Human-readable description of the API error
    /// * `category` - Specific API error subcategory for precise classification
    /// * `status_code` - HTTP status code if applicable
    /// * `endpoint` - API endpoint where the error occurred if applicable
    #[error("API error: {message}")]
    Api {
        /// Human-readable description of the API error
        message: String,
        #[serde(default)]
        /// Specific API error subcategory for precise classification
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
    /// * `category` - Specific workflow error subcategory for precise classification
    #[error("Workflow error: {message}")]
    Workflow {
        /// Human-readable description of the workflow error
        message: String,
        #[serde(default)]
        /// Specific workflow error subcategory for precise classification
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


}

/// Type alias for Result types using BearDogError
/// 
/// This provides a convenient shorthand for `Result<T, BearDogError>` throughout
/// the BearDog ecosystem, enabling consistent error handling patterns.
/// 
/// # Usage
/// 
/// ```rust
/// use beardog_errors::BearDogResult;
/// 
/// fn example_operation() -> BearDogResult<String> {
///     Ok("Success".to_string())
/// }
/// ```
pub type BearDogResult<T> = Result<T, BearDogError>; 