//! BearDog Deployment - Platform Deployment Automation
//!
//! Comprehensive deployment capabilities for the BearDog ecosystem including
//! platform-specific deployment strategies, environment validation, and
//! optimization features for production deployments.
//!
//! # Features
//!
//! - Android deployment with hardware security module integration
//! - Environment validation and verification
//! - Deployment optimization and performance tuning
//! - Device-specific configuration management
//! - Error handling and recovery mechanisms
//!
//! # Example
//!
//! ```rust,no_run
//! use beardog_deploy::{DeploymentConfig, DeploymentManager};
//!
//! let config = DeploymentConfig::default();
//! let manager = DeploymentManager::new(config);
//! manager.initialize().expect("deployment init");
//! ```

// Production code must use proper error handling - deny panicking methods
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
// Allow expect in tests - test panics are appropriate failure modes
#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::unwrap_used))]

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

pub mod android;
pub mod builder;
pub mod device;

/// Error types and handling for deployment operations
pub mod error;
pub mod optimization;
#[cfg(test)]
mod tests;

pub use android::*;
pub use builder::*;
pub use device::*;
pub use error::*;

// October 27, 2025: Comprehensive test expansion
#[cfg(test)]
mod deploy_comprehensive_tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Target deployment environment (dev, staging, prod)
    pub environment: String,
    /// Target deployment region
    pub region: String,
    /// Number of instances to deploy
    pub instance_count: u32,
    /// Whether monitoring is enabled
    pub monitoring_enabled: bool,
}

impl Default for DeploymentConfig {
    fn default() -> Self {
        Self {
            environment: "development".to_string(),
            region: "local".to_string(),
            instance_count: 1,
            monitoring_enabled: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeploymentManager {
    config: DeploymentConfig,
}

impl DeploymentManager {
    /// Creates a new deployment manager
    ///
    /// # Arguments
    /// * `config` - Deployment configuration
    ///
    /// # Returns
    /// A new `DeploymentManager` instance
    #[must_use]
    pub const fn new(config: DeploymentConfig) -> Self {
        Self { config }
    }

    /// Initialize the deployment environment.
    ///
    /// Validates configuration and prepares the deployment target.
    ///
    /// # Errors
    ///
    /// Returns an error if configuration validation fails.
    pub fn initialize(&self) -> Result<(), BearDogError> {
        println!("🚀 Initializing BearDog deployment environment");

        self.validate_config()?;
        println!("✅ Deployment environment initialized successfully");
        Ok(())
    }

    /// Validates the deployment configuration.
    ///
    /// # Errors
    ///
    /// Returns error if configuration is invalid or incomplete.
    fn validate_config(&self) -> Result<(), BearDogError> {
        if self.config.environment.is_empty() {
            return Err(BearDogError::system(
                "Environment configuration cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}
