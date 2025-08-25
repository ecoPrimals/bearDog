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


//! # BearDog Unified Error System
//!
//! **CANONICAL ERRORS FOR THE BEARDOG ECOSYSTEM**
//! This crate provides the canonical error system that unifies all BearDog error
//! handling across security, genetics, workflows, and infrastructure domains.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// UNIFIED ERROR TYPES - Consolidated from across the codebase
// ============================================================================

/// **CANONICAL BEARDOG ERROR** - Unified error type for the entire ecosystem
/// 
/// **ERROR SYSTEM UNIFICATION COMPLETE** ✅
/// This enum consolidates ALL error types from across the codebase:
/// 
/// ## **Fragmentation Eliminated:**
/// - `HsmError` from tunnel modules → `BearDogError::Hsm`
/// - `SystemError` from various modules → `BearDogError::System` 
/// - `SecurityError` from security modules → `BearDogError::Security`
/// - `BusinessError` from error categories → `BearDogError::Business`
/// - `ApiErrorType` from API modules → `BearDogError::Api`
/// - `DeployError` from deploy crate → `BearDogError::Deployment`
/// - `SafeMemoryError` from utils → `BearDogError::Memory`
/// - Various workflow, genetics, and adapter errors → respective variants
/// 
/// ## **Design Benefits:**
/// - **Single Error Type**: All operations use `BearDogResult<T>`
/// - **Rich Context**: Each variant carries domain-specific information
/// - **Backward Compatible**: Existing error constructors still work
/// - **Hierarchical Organization**: Logical grouping by error domain
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum BearDogError {
    /// Security-related errors (consolidates SecurityError)
    #[error("Security error: {message}")]
    Security {
        /// Error message
        message: String,
        /// Security error category
        #[serde(default)]
        category: SecurityErrorCategory,
    },

    /// System and infrastructure errors (consolidates SystemError)
    #[error("System error: {message}")]
    System {
        /// Error message  
        message: String,
        /// System error category
        #[serde(default)]
        category: SystemErrorCategory,
    },

    /// Business logic and validation errors (consolidates BusinessError)
    #[error("Business error: {message}")]
    Business {
        /// Error message
        message: String,
        /// Business error category
        #[serde(default)]
        category: BusinessErrorCategory,
    },

    /// Network and connectivity errors
    #[error("Network error: {message}")]
    Network {
        /// Error message
        message: String,
        /// Network error category
        #[serde(default)]
        category: NetworkErrorCategory,
    },

    /// Configuration and setup errors
    #[error("Configuration error: {message}")]
    Configuration {
        /// Error message
        message: String,
        /// Configuration error category
        #[serde(default)]
        category: ConfigurationErrorCategory,
    },

    /// Initialization errors
    #[error("Initialization error: {message}")]
    Initialization {
        /// Error message
        message: String,
    },

    /// **HSM-related errors (consolidates HsmError)**
    #[error("HSM error: {message}")]
    Hsm {
        /// Error message
        message: String,
        /// HSM error category
        #[serde(default)]
        category: HsmErrorCategory,
        /// HSM provider that failed
        provider: Option<String>,
    },

    /// **API-related errors (consolidates ApiErrorType)**
    #[error("API error: {message}")]
    Api {
        /// Error message
        message: String,
        /// API error category
        #[serde(default)]
        category: ApiErrorCategory,
        /// HTTP status code
        status_code: Option<u16>,
        /// API endpoint that failed
        endpoint: Option<String>,
    },

    /// **Workflow-related errors**
    #[error("Workflow error: {message}")]
    Workflow {
        /// Error message
        message: String,
        /// Workflow ID that failed
        workflow_id: Option<String>,
        /// Workflow error category
        #[serde(default)]
        category: WorkflowErrorCategory,
    },

    /// **Genetics system errors**
    #[error("Genetics error: {message}")]
    Genetics {
        /// Error message
        message: String,
        /// Genetics operation that failed
        operation: Option<String>,
    },

    /// **Deployment errors (consolidates DeployError)**
    #[error("Deployment error: {message}")]
    Deployment {
        /// Error message
        message: String,
        /// Deployment stage that failed
        stage: Option<String>,
    },

    /// **Memory management errors (consolidates SafeMemoryError)**
    #[error("Memory error: {message}")]
    Memory {
        /// Error message
        message: String,
        /// Memory operation that failed
        operation: Option<String>,
    },

    /// **Adapter/Integration errors**
    #[error("Adapter error: {message}")]
    Adapter {
        /// Error message
        message: String,
        /// Adapter name that failed
        adapter: Option<String>,
        /// Target system
        target: Option<String>,
    },

    /// **Authentication errors**
    #[error("Authentication error: {message}")]
    Authentication {
        /// Error message
        message: String,
        /// Authentication method that failed
        method: Option<String>,
    },

    /// **Authorization errors**
    #[error("Authorization error: {message}")]
    Authorization {
        /// Error message
        message: String,
        /// Required permission
        required_permission: Option<String>,
    },

    /// **Cryptographic errors**
    #[error("Cryptographic error: {message}")]
    Cryptographic {
        /// Error message
        message: String,
        /// Cryptographic operation that failed
        operation: Option<String>,
        /// Algorithm used
        algorithm: Option<String>,
    },

    /// **Monitoring/Metrics errors**
    #[error("Monitoring error: {message}")]
    Monitoring {
        /// Error message
        message: String,
        /// Metric name that failed
        metric: Option<String>,
    },

    /// **Compliance/Audit errors**
    #[error("Compliance error: {message}")]
    Compliance {
        /// Error message
        message: String,
        /// Compliance standard
        standard: Option<String>,
    },
}

// ============================================================================
// ERROR CATEGORIES - Fine-grained error classification
// ============================================================================

/// Security error categories
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SecurityErrorCategory {
    /// General security errors
    #[default]
    General,
    /// Authentication-related errors
    Authentication,
    /// Authorization and permission errors
    Authorization,
    /// Encryption and cryptographic errors
    Encryption,
    /// Key management errors
    KeyManagement,
    /// Access control violations
    AccessControl,
    /// Audit and logging errors
    Audit,
    /// Compliance and regulatory errors
    Compliance,
}

/// System error categories
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SystemErrorCategory {
    /// General system errors
    #[default]
    General,
    /// File system related errors
    FileSystem,
    /// Process management errors
    Process,
    /// Resource allocation errors
    Resource,
    /// Permission and access errors
    Permission,
    /// Hardware-related errors
    Hardware,
    /// Service management errors
    Service,
}

/// Business error categories
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum BusinessErrorCategory {
    /// General business logic errors
    #[default]
    General,
    /// Input validation errors
    Validation,
    /// Business rule violations
    RuleViolation,
    /// State transition errors
    StateTransition,
    /// Data integrity violations
    DataIntegrity,
    /// Policy enforcement errors
    Policy,
}

/// Network error categories
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum NetworkErrorCategory {
    /// General network errors
    #[default]
    General,
    /// Connection establishment errors
    Connection,
    /// Network timeout errors
    Timeout,
    /// Protocol-related errors
    Protocol,
    /// DNS resolution errors
    Dns,
    /// SSL/TLS errors
    Tls,
}

/// Configuration error categories
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ConfigurationErrorCategory {
    /// General configuration errors
    #[default]
    General,
    /// Configuration parsing errors
    Parsing,
    /// Configuration validation errors
    Validation,
    /// Missing configuration values
    Missing,
    /// Invalid configuration format or values
    Invalid,
}

/// HSM error categories
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum HsmErrorCategory {
    /// General HSM errors
    #[default]
    General,
    /// Key generation failures
    KeyGeneration,
    /// Key not found errors
    KeyNotFound,
    /// Digital signing errors
    Signing,
    /// Encryption/decryption errors
    Encryption,
    /// Hardware-related errors
    Hardware,
    /// Communication with HSM errors
    Communication,
    /// HSM authentication errors
    Authentication,
}

/// Workflow error categories
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum WorkflowErrorCategory {
    /// General workflow errors
    #[default]
    General,
    /// Workflow execution errors
    Execution,
    /// Approval process errors
    Approval,
    /// Workflow timeout errors
    Timeout,
    /// State transition errors
    StateTransition,
    /// Workflow validation errors
    Validation,
}

/// API error categories
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ApiErrorCategory {
    /// General API errors
    #[default]
    General,
    /// Authentication failures
    Authentication,
    /// Authorization denied
    Authorization,
    /// Input validation errors
    Validation,
    /// Resource not found
    NotFound,
    /// Resource conflicts
    Conflict,
    /// Rate limiting errors
    RateLimit,
    /// Internal server errors
    Internal,
    /// Service unavailable
    ServiceUnavailable,
}

/// Standard result type for all BearDog operations
/// 
/// **PRIMARY DEFINITION** - This is the canonical BearDogResult definition.
/// Re-exported by beardog-types/src/aliases.rs for centralized access.
pub type BearDogResult<T> = Result<T, BearDogError>;

// ============================================================================
// ENHANCED ERROR CONSTRUCTORS - Backward compatible + new unified constructors
// ============================================================================

impl BearDogError {
    /// Create a new security error
    pub fn security(message: impl Into<String>) -> Self {
        Self::Security {
            message: message.into(),
            category: SecurityErrorCategory::General,
        }
    }

    /// Create security error with category
    pub fn security_with_category(message: impl Into<String>, category: SecurityErrorCategory) -> Self {
        Self::Security {
            message: message.into(),
            category,
        }
    }

    /// Create a new system error
    pub fn system(message: impl Into<String>) -> Self {
        Self::System {
            message: message.into(),
            category: SystemErrorCategory::General,
        }
    }

    /// Create system error with category
    pub fn system_with_category(message: impl Into<String>, category: SystemErrorCategory) -> Self {
        Self::System {
            message: message.into(),
            category,
        }
    }

    /// Create a new business error
    pub fn business(message: impl Into<String>) -> Self {
        Self::Business {
            message: message.into(),
            category: BusinessErrorCategory::General,
        }
    }

    /// Create a validation error for invalid input
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::Business {
            message: message.into(),
            category: BusinessErrorCategory::Validation,
        }
    }

    /// Create business error with category
    pub fn business_with_category(message: impl Into<String>, category: BusinessErrorCategory) -> Self {
        Self::Business {
            message: message.into(),
            category,
        }
    }

    /// Create a new network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
            category: NetworkErrorCategory::General,
        }
    }

    /// Create a new configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
            category: ConfigurationErrorCategory::General,
        }
    }

    /// Create a new initialization error
    pub fn initialization(message: impl Into<String>) -> Self {
        Self::Initialization {
            message: message.into(),
        }
    }

    /// **NEW: Create API error (consolidates ApiErrorType)**
    pub fn api(message: impl Into<String>, category: ApiErrorCategory) -> Self {
        Self::Api {
            message: message.into(),
            category,
            status_code: None,
            endpoint: None,
        }
    }

    /// **NEW: Create HSM error (consolidates HsmError)**
    pub fn hsm(message: impl Into<String>) -> Self {
        Self::Hsm {
            message: message.into(),
            category: HsmErrorCategory::General,
            provider: None,
        }
    }

    /// Create HSM error with provider and category
    pub fn hsm_with_details(
        message: impl Into<String>, 
        category: HsmErrorCategory, 
        provider: Option<String>
    ) -> Self {
        Self::Hsm {
            message: message.into(),
            category,
            provider,
        }
    }



    /// Create API error with HTTP status and endpoint
    pub fn api_with_details(
        message: impl Into<String>, 
        status_code: Option<u16>, 
        endpoint: Option<String>
    ) -> Self {
        Self::Api {
            message: message.into(),
            category: ApiErrorCategory::General,
            status_code,
            endpoint,
        }
    }

    /// **NEW: Create workflow error**
    pub fn workflow(message: impl Into<String>) -> Self {
        Self::Workflow {
            message: message.into(),
            workflow_id: None,
            category: WorkflowErrorCategory::General,
        }
    }

    /// Create workflow error with ID and category
    pub fn workflow_with_details(
        message: impl Into<String>, 
        workflow_id: Option<String>, 
        category: WorkflowErrorCategory
    ) -> Self {
        Self::Workflow {
            message: message.into(),
            workflow_id,
            category,
        }
    }

    /// **NEW: Create genetics error**
    pub fn genetics(message: impl Into<String>) -> Self {
        Self::Genetics {
            message: message.into(),
            operation: None,
        }
    }

    /// **NEW: Create deployment error (consolidates DeployError)**
    pub fn deployment(message: impl Into<String>) -> Self {
        Self::Deployment {
            message: message.into(),
            stage: None,
        }
    }

    /// Create deployment error with stage
    pub fn deployment_with_stage(message: impl Into<String>, stage: impl Into<String>) -> Self {
        Self::Deployment {
            message: message.into(),
            stage: Some(stage.into()),
        }
    }

    /// **NEW: Create memory error (consolidates SafeMemoryError)**
    pub fn memory(message: impl Into<String>) -> Self {
        Self::Memory {
            message: message.into(),
            operation: None,
        }
    }

    /// **NEW: Create adapter error**
    pub fn adapter(message: impl Into<String>) -> Self {
        Self::Adapter {
            message: message.into(),
            adapter: None,
            target: None,
        }
    }

    /// Create adapter error with details
    pub fn adapter_with_details(
        message: impl Into<String>, 
        adapter: Option<String>, 
        target: Option<String>
    ) -> Self {
        Self::Adapter {
            message: message.into(),
            adapter,
            target,
        }
    }

    // Convenience constructors for common patterns (BACKWARD COMPATIBLE)
    /// Create an authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication {
            message: message.into(),
            method: None,
        }
    }

    /// Create an authorization error
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::Authorization {
            message: message.into(),
            required_permission: None,
        }
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::business_with_category(
            format!("Validation failed: {}", message.into()), 
            BusinessErrorCategory::Validation
        )
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::system_with_category(
            format!("Internal error: {}", message.into()), 
            SystemErrorCategory::General
        )
    }

    /// Create an unsupported operation error
    pub fn unsupported_operation(message: impl Into<String>) -> Self {
        Self::system(format!("Unsupported operation: {}", message.into()))
    }

    /// Create a timeout error
    pub fn timeout(operation: impl Into<String>) -> Self {
        Self::network_with_category(
            format!("Operation timed out: {}", operation.into()),
            NetworkErrorCategory::Timeout
        )
    }

    /// Create a connection error
    pub fn connection(message: impl Into<String>) -> Self {
        Self::network_with_category(
            format!("Connection error: {}", message.into()),
            NetworkErrorCategory::Connection
        )
    }

    /// Create a parsing error
    pub fn parsing(message: impl Into<String>) -> Self {
        Self::business_with_category(
            format!("Parsing error: {}", message.into()),
            BusinessErrorCategory::Validation
        )
    }

    /// Create a not found error
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::business(format!("Resource not found: {}", resource.into()))
    }

    /// Create an already exists error
    pub fn already_exists(resource: impl Into<String>) -> Self {
        Self::business(format!("Resource already exists: {}", resource.into()))
    }

    /// **NEW: Create cryptographic error**
    pub fn cryptographic(message: impl Into<String>) -> Self {
        Self::Cryptographic {
            message: message.into(),
            operation: None,
            algorithm: None,
        }
    }

    /// **NEW: Create monitoring error**
    pub fn monitoring(message: impl Into<String>) -> Self {
        Self::Monitoring {
            message: message.into(),
            metric: None,
        }
    }

    /// **NEW: Create compliance error**
    pub fn compliance(message: impl Into<String>) -> Self {
        Self::Compliance {
            message: message.into(),
            standard: None,
        }
    }

    /// Create network error with category
    pub fn network_with_category(message: impl Into<String>, category: NetworkErrorCategory) -> Self {
        Self::Network {
            message: message.into(),
            category,
        }
    }
}

// ============================================================================
// STANDARD LIBRARY CONVERSIONS - Better interoperability
// ============================================================================

impl From<std::io::Error> for BearDogError {
    fn from(err: std::io::Error) -> Self {
        Self::system_with_category(
            format!("I/O error: {err}"), 
            SystemErrorCategory::FileSystem
        )
    }
}

impl From<serde_json::Error> for BearDogError {
    fn from(err: serde_json::Error) -> Self {
        Self::business_with_category(
            format!("JSON error: {err}"),
            BusinessErrorCategory::Validation
        )
    }
}

impl From<std::fmt::Error> for BearDogError {
    fn from(err: std::fmt::Error) -> Self {
        Self::system(format!("Format error: {err}"))
    }
}

impl From<std::num::ParseIntError> for BearDogError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::business_with_category(
            format!("Integer parsing error: {err}"),
            BusinessErrorCategory::Validation
        )
    }
}

impl From<std::num::ParseFloatError> for BearDogError {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::business_with_category(
            format!("Float parsing error: {err}"),
            BusinessErrorCategory::Validation
        )
    }
}

impl From<std::string::FromUtf8Error> for BearDogError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::business_with_category(
            format!("UTF-8 conversion error: {err}"),
            BusinessErrorCategory::Validation
        )
    }
}

// ============================================================================
// RESULT EXTENSION TRAIT - Enhanced ergonomics
// ============================================================================

/// Extension trait for Result types to provide better error handling ergonomics
pub trait ResultExt<T> {
    /// Convert any error to a BearDog security error
    fn security_context(self, message: impl Into<String>) -> BearDogResult<T>;
    
    /// Convert any error to a BearDog system error
    fn system_context(self, message: impl Into<String>) -> BearDogResult<T>;
    
    /// Convert any error to a BearDog business error
    fn business_context(self, message: impl Into<String>) -> BearDogResult<T>;
    
    /// Convert any error to a BearDog network error
    fn network_context(self, message: impl Into<String>) -> BearDogResult<T>;

    /// Convert any error to a BearDog HSM error
    fn hsm_context(self, message: impl Into<String>) -> BearDogResult<T>;

    /// Convert any error to a BearDog workflow error
    fn workflow_context(self, message: impl Into<String>) -> BearDogResult<T>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn security_context(self, message: impl Into<String>) -> BearDogResult<T> {
        self.map_err(|e| BearDogError::security(format!("{}: {}", message.into(), e)))
    }
    
    fn system_context(self, message: impl Into<String>) -> BearDogResult<T> {
        self.map_err(|e| BearDogError::system(format!("{}: {}", message.into(), e)))
    }
    
    fn business_context(self, message: impl Into<String>) -> BearDogResult<T> {
        self.map_err(|e| BearDogError::business(format!("{}: {}", message.into(), e)))
    }
    
    fn network_context(self, message: impl Into<String>) -> BearDogResult<T> {
        self.map_err(|e| BearDogError::network(format!("{}: {}", message.into(), e)))
    }

    fn hsm_context(self, message: impl Into<String>) -> BearDogResult<T> {
        self.map_err(|e| BearDogError::hsm(format!("{}: {}", message.into(), e)))
    }

    fn workflow_context(self, message: impl Into<String>) -> BearDogResult<T> {
        self.map_err(|e| BearDogError::workflow(format!("{}: {}", message.into(), e)))
    }
}

// Re-export active modules
/// Error type definitions and categorization
pub mod error_types;
/// Idiomatic Rust error handling patterns
pub mod idiomatic;
/// Error implementation utilities and analysis
pub mod implementations;

// Note: builders, categories, improved_results, and types modules
// were removed during modernization as their functionality was
// consolidated into the above active modules.

// ============================================================================
// VALIDATION MODULE
// ============================================================================

/// Validation utilities for the error system
pub mod validation {
    use super::*;

    /// Validate that the error system is working correctly
    pub fn validate_error_usage() -> Result<(), String> {
        // Test that all constructor methods work
        let _security = BearDogError::security("test");
        let _system = BearDogError::system("test");
        let _business = BearDogError::business("test");
        let _config = BearDogError::configuration("test");
        let _network = BearDogError::network("test");
        let _validation = BearDogError::validation("test");
        let _auth = BearDogError::authentication("test");
        let _authz = BearDogError::authorization("test");
        
        // Test new unified constructors
        let _hsm = BearDogError::hsm("test");
        let _api = BearDogError::api("test", ApiErrorCategory::General);
        let _workflow = BearDogError::workflow("test");
        let _genetics = BearDogError::genetics("test");
        let _deployment = BearDogError::deployment("test");
        let _deployment_with_stage = BearDogError::deployment_with_stage("test", "staging");
        let _memory = BearDogError::memory("test");
        let _adapter = BearDogError::adapter("test");
        let _crypto = BearDogError::cryptographic("test");
        let _monitoring = BearDogError::monitoring("test");
        let _compliance = BearDogError::compliance("test");

        Ok(())
    }

    /// Get information about the error system
    pub fn error_system_info() -> Vec<(&'static str, &'static str)> {
        vec![
            ("BearDogError", "Unified error type for all BearDog operations"),
            ("BearDogResult<T>", "Standard result type"),
            ("Security", "Security-related errors with categories"),
            ("System", "System and infrastructure errors with categories"),
            ("Business", "Business logic and validation errors with categories"),
            ("Network", "Network and connectivity errors with categories"),
            ("Hsm", "Hardware Security Module errors"),
            ("Api", "API-related errors with HTTP context"),
            ("Workflow", "Workflow execution errors"),
            ("Genetics", "Genetics system errors"),
            ("Deployment", "Deployment and provisioning errors"),
            ("Memory", "Memory management errors"),
            ("Adapter", "Integration adapter errors"),
            ("Cryptographic", "Cryptographic operation errors"),
            ("Monitoring", "Monitoring and metrics errors"),
            ("Compliance", "Compliance and audit errors"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_error_constructors() {
        let security_error = BearDogError::security("test security");
        let system_error = BearDogError::system("test system");
        let business_error = BearDogError::business("test business");
        let hsm_error = BearDogError::hsm("test hsm");
        let api_error = BearDogError::api("test api", ApiErrorCategory::General);
        let workflow_error = BearDogError::workflow("test workflow");

        assert!(matches!(security_error, BearDogError::Security { .. }));
        assert!(matches!(system_error, BearDogError::System { .. }));
        assert!(matches!(business_error, BearDogError::Business { .. }));
        assert!(matches!(hsm_error, BearDogError::Hsm { .. }));
        assert!(matches!(api_error, BearDogError::Api { .. }));
        assert!(matches!(workflow_error, BearDogError::Workflow { .. }));
    }

    #[test]
    fn test_error_categories() {
        let security_error = BearDogError::security_with_category(
            "auth failed", 
            SecurityErrorCategory::Authentication
        );
        
        if let BearDogError::Security { category, .. } = security_error {
            assert!(matches!(category, SecurityErrorCategory::Authentication));
        } else {
            panic!("Expected Security error");
        }
    }

    #[test]
    fn test_specialized_constructors() {
        let config_error = BearDogError::configuration("test config");
        let network_error = BearDogError::network("test network");
        let validation_error = BearDogError::validation("test validation");
        let auth_error = BearDogError::authentication("test auth");
        let authz_error = BearDogError::authorization("test authz");
        let crypto_error = BearDogError::cryptographic("test crypto");
        let monitoring_error = BearDogError::monitoring("test monitoring");
        let compliance_error = BearDogError::compliance("test compliance");

        assert!(matches!(config_error, BearDogError::Configuration { .. }));
        assert!(matches!(network_error, BearDogError::Network { .. }));
        assert!(matches!(validation_error, BearDogError::Business { .. }));
        assert!(matches!(auth_error, BearDogError::Authentication { .. }));
        assert!(matches!(authz_error, BearDogError::Authorization { .. }));
        assert!(matches!(crypto_error, BearDogError::Cryptographic { .. }));
        assert!(matches!(monitoring_error, BearDogError::Monitoring { .. }));
        assert!(matches!(compliance_error, BearDogError::Compliance { .. }));
    }

    #[test]
    fn test_result_extensions() {
        let result: Result<(), std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound, 
            "file not found"
        ));
        
        let beardog_result = result.system_context("Failed to read file");
        assert!(beardog_result.is_err());
        
        if let Err(BearDogError::System { message, category }) = beardog_result {
            assert!(message.contains("Failed to read file"));
            assert!(matches!(category, SystemErrorCategory::General));
        } else {
            panic!("Expected System error");
        }
    }

    #[test]
    fn test_validation_system() {
        let result = validation::validate_error_usage();
        assert!(result.is_ok(), "Error usage validation should pass");
        
        let info = validation::error_system_info();
        assert!(!info.is_empty());
        let error_names: Vec<&str> = info.iter().map(|(name, _)| *name).collect();
        assert!(error_names.contains(&"BearDogError"));
        assert!(error_names.contains(&"BearDogResult<T>"));
        assert!(error_names.contains(&"Hsm"));
        assert!(error_names.contains(&"Api"));
        assert!(error_names.contains(&"Workflow"));
    }

    #[test]
    fn test_backward_compatibility() {
        // Test that all existing error constructors still work
        let _security = BearDogError::security("test");
        let _system = BearDogError::system("test");
        let _business = BearDogError::business("test");
        let _network = BearDogError::network("test");
        let _config = BearDogError::configuration("test");
        let _init = BearDogError::initialization("test");
        let _auth = BearDogError::authentication("test");
        let _authz = BearDogError::authorization("test");
        let _validation = BearDogError::validation("test");
        let _internal = BearDogError::internal("test");
        let _timeout = BearDogError::timeout("test");
        let _connection = BearDogError::connection("test");
        let _parsing = BearDogError::parsing("test");
        let _not_found = BearDogError::not_found("test");
        let _already_exists = BearDogError::already_exists("test");
    }
}
