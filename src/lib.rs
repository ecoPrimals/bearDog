//! # BearDog Security Manager
//! 
//! **Democratizing Enterprise-Grade Security for Everyone**
//! 
//! BearDog is an open-source security management platform that brings Fortune 500-grade 
//! security capabilities to developers and organizations of all sizes. Released under AGPL 3.0,
//! every security improvement benefits the entire community.
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use beardog::{BearDogConfig, BearDogCore};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load secure defaults that work out-of-the-box
//!     let config = BearDogConfig::from_env()?;
//!     
//!     // Initialize BearDog with democratized security
//!     let beardog = BearDogCore::new(config).await?;
//!     
//!     // Start protecting your application
//!     beardog.start().await?;
//!     
//!     println!("🐕 BearDog is protecting your application!");
//!     Ok(())
//! }
//! ```
//! 
//! ## Core Capabilities
//! 
//! - **🔐 Zero-Trust Encryption**: AES-256-GCM, post-quantum ready
//! - **🏗️ HSM Integration**: Support for any PKCS#11 compatible HSM
//! - **🚨 Threat Detection**: ML-powered real-time threat analysis
//! - **📋 Compliance**: GDPR, HIPAA, SOX, PCI DSS, FedRAMP out-of-box
//! - **🔗 Easy Integration**: Clean APIs for any application
//! 
//! ## Mission
//! 
//! Security should not be a privilege of the wealthy. BearDog democratizes enterprise-grade
//! security capabilities that were previously locked behind expensive enterprise licenses.
//! 
//! Under AGPL 3.0, all improvements flow back to benefit everyone, creating a growing
//! commons of security intelligence that makes the entire internet safer.

#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![warn(missing_docs)]

// Re-export commonly used types
pub use crate::core::BearDogCore;
pub use crate::config::{BearDogConfig, SecurityLevel};
pub use crate::error::{BearDogError, BearDogResult};

// Core modules
pub mod config;
pub mod core;
pub mod error;

// Security modules
pub mod encryption;
pub mod threat_detection;
pub mod compliance;
pub mod audit;
pub mod security_provider;

// Integration modules
pub mod adapters;
pub mod workflows;

// API module
pub mod api;

// Utilities
pub mod utils;

/// BearDog version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// BearDog mission statement
pub const MISSION: &str = "Democratizing enterprise-grade security for everyone";

/// Initialize BearDog with secure defaults
/// 
/// This is the simplest way to get started with BearDog. It will:
/// - Load configuration from environment variables and files
/// - Set secure defaults for all unspecified options
/// - Initialize logging and metrics
/// - Return a ready-to-use BearDogCore instance
/// 
/// ## Example
/// 
/// ```rust,no_run
/// use beardog::{BearDogCore, BearDogConfig};
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = BearDogConfig::from_env()?;
///     let beardog = BearDogCore::new(config).await?;
///     println!("🐕 BearDog initialized and ready!");
///     Ok(())
/// }
/// ```
pub async fn initialize() -> BearDogResult<BearDogCore> {
    let config = BearDogConfig::from_env()?;
    BearDogCore::new(config).await
}

/// Initialize BearDog with custom configuration
/// 
/// For applications that need specific configuration beyond the secure defaults.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use beardog::{BearDogCore, BearDogConfig, SecurityLevel};
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut config = BearDogConfig::from_env()?;
///     config.security.level = SecurityLevel::Maximum;
///     
///     let beardog = BearDogCore::new(config).await?;
///     println!("🐕 BearDog initialized with custom config!");
///     Ok(())
/// }
/// ```
pub async fn initialize_with_config(config: BearDogConfig) -> BearDogResult<BearDogCore> {
    BearDogCore::new(config).await
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_core_initialization() {
        let config = BearDogConfig::default();
        let core = BearDogCore::new(config).await;
        assert!(core.is_ok());
    }
    
    #[test]
    fn test_config_validation() {
        // Test with valid configuration
        let config = BearDogConfig::default();
        assert!(!config.api.bind_address.is_empty());
        // API config doesn't have a port field - it's part of bind_address
        assert!(config.api.bind_address.contains(":"));
    }
    
    #[test]
    fn test_error_types() {
        // Test error type creation and display
        let error = BearDogError::Configuration { message: "Test error".to_string() };
        let error_string = format!("{}", error);
        assert!(error_string.contains("Test error"));
    }
    
    #[test]
    fn test_config_serialization() {
        // Test configuration serialization/deserialization
        let config = BearDogConfig::default();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        let deserialized: Result<BearDogConfig, _> = serde_json::from_str(&json_str);
        assert!(deserialized.is_ok());
    }
    
    #[test]
    fn test_module_imports() {
        // Test that all modules are properly imported
        use crate::config::*;
        use crate::encryption::*;
        use crate::threat_detection::*;
        use crate::compliance::*;
        use crate::workflows::*;
        
        // If we can import all these, the modules are properly structured
        assert!(true);
    }
} 