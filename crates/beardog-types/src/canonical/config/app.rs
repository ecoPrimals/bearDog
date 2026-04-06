// SPDX-License-Identifier: AGPL-3.0-or-later

//! Application Configuration
//!
//! Canonical application configuration module providing centralized app-level settings.
//!
//! # Overview
//!
//! `CanonicalAppConfig` consolidates all application-level configuration including:
//! - Application identity (name, version, environment)
//! - Logging and debugging settings
//! - Feature flags for runtime behavior control
//! - Custom metadata for application-specific needs
//!
//! # Quick Start
//!
//! ```rust
//! use beardog_types::canonical::config::app::{CanonicalAppConfig, LogLevel};
//! use std::collections::HashMap;
//!
//! // Create development configuration
//! let mut config = CanonicalAppConfig {
//!     name: "beardog-api".to_string(),
//!     version: "3.0.0".to_string(),
//!     environment: "development".to_string(),
//!     debug: true,
//!     log_level: LogLevel::Debug,
//!     features: HashMap::new(),
//!     metadata: HashMap::new(),
//! };
//!
//! // Enable feature flags
//! config.features.insert("experimental_crypto".to_string(), true);
//! config.features.insert("hsm_integration".to_string(), false);
//!
//! // Add metadata
//! config.metadata.insert("team".to_string(), "security".to_string());
//! config.metadata.insert("region".to_string(), "us-east-1".to_string());
//! ```
//!
//! # Production Configuration
//!
//! ```rust
//! use beardog_types::canonical::config::app::{CanonicalAppConfig, LogLevel};
//! use std::collections::HashMap;
//!
//! // Production configuration
//! let config = CanonicalAppConfig {
//!     name: "beardog-api".to_string(),
//!     version: "3.0.0".to_string(),
//!     environment: "production".to_string(),
//!     debug: false,
//!     log_level: LogLevel::Warn,  // Less verbose in production
//!     features: HashMap::new(),
//!     metadata: HashMap::new(),
//! };
//!
//! assert!(!config.debug);
//! ```
//!
//! # Environment Detection
//!
//! ```rust
//! use beardog_types::canonical::config::app::CanonicalAppConfig;
//!
//! let config = CanonicalAppConfig::default();
//!
//! // Check environment
//! let is_production = config.environment == "production";
//! let is_development = config.environment == "development";
//! let is_staging = config.environment == "staging";
//!
//! // Conditional logic based on environment
//! if is_development {
//!     // Enable debug features
//! }
//! ```
//!
//! # Feature Flags
//!
//! Feature flags provide runtime control over application behavior:
//!
//! ```rust
//! use beardog_types::canonical::config::app::CanonicalAppConfig;
//! use std::collections::HashMap;
//!
//! let mut config = CanonicalAppConfig::default();
//!
//! // Set feature flags
//! config.features.insert("new_auth_flow".to_string(), true);
//! config.features.insert("legacy_api".to_string(), false);
//!
//! // Check flags
//! if config.features.get("new_auth_flow").copied().unwrap_or(false) {
//!     // Use new authentication flow
//! }
//! ```
//!
//! # Design Principles
//!
//! - **Type Safety**: All configuration is strongly typed
//! - **Flexibility**: Feature flags and metadata for extensibility
//! - **Clarity**: Self-documenting field names and structures
//! - **Environment Awareness**: Explicit environment configuration
//!
//! # Thread Safety
//!
//! `CanonicalAppConfig` implements `Clone`, making it safe to share across threads
//! when wrapped in `Arc<AppConfig>`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Canonical application configuration - consolidates all app-related configs
///
/// The primary configuration structure for application-level settings including
/// identity, logging, feature flags, and metadata.
///
/// # Fields
///
/// * `name` - Application identifier (e.g., "beardog-api", "beardog-cli")
/// * `version` - Semantic version string (e.g., "3.0.0")
/// * `environment` - Runtime environment ("development", "staging", "production")
/// * `debug` - Whether debug mode is enabled (affects logging and error details)
/// * `log_level` - Minimum logging level (Trace, Debug, Info, Warn, Error)
/// * `features` - Runtime feature flags for behavior control
/// * `metadata` - Application-specific custom metadata
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::app::{CanonicalAppConfig, LogLevel};
/// use std::collections::HashMap;
///
/// // Create configuration
/// let mut config = CanonicalAppConfig {
///     name: "my-service".to_string(),
///     version: "1.0.0".to_string(),
///     environment: "production".to_string(),
///     debug: false,
///     log_level: LogLevel::Info,
///     features: HashMap::new(),
///     metadata: HashMap::new(),
/// };
///
/// // Configure features
/// config.features.insert("auto_scaling".to_string(), true);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalAppConfig {
    /// Application name
    /// Name of the item
    pub name: String,

    /// Application version
    /// The version value
    pub version: String,

    /// Environment (development, staging, production)
    /// The environment value
    pub environment: String,

    /// Debug mode enabled
    /// Whether debug is enabled
    pub debug: bool,

    /// Log level
    /// The log level value
    pub log_level: LogLevel,

    /// Feature flags
    /// Mapping of features
    pub features: HashMap<String, bool>,

    /// Metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Log levels for application logging
///
/// Defines the minimum severity level for log messages. Messages below this level
/// will be filtered out.
///
/// # Verbosity Order (most to least):
/// 1. `Trace` - Everything including low-level details
/// 2. `Debug` - Debug information for troubleshooting
/// 3. `Info` - General informational messages (default)
/// 4. `Warn` - Warning messages for potential issues
/// 5. `Error` - Error messages only
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::app::LogLevel;
///
/// // Development - verbose logging
/// let dev_level = LogLevel::Debug;
///
/// // Production - only warnings and errors
/// let prod_level = LogLevel::Warn;
///
/// // Debugging - maximum verbosity
/// let debug_level = LogLevel::Trace;
/// ```
///
/// # Performance
///
/// Lower log levels (Error, Warn) have better performance as fewer messages are processed.
/// Use `Trace` and `Debug` only in development/troubleshooting scenarios.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LogLevel {
    /// Trace level logging (most verbose)
    ///
    /// Includes all log messages including low-level execution traces.
    /// **Warning**: Very high volume, use only for deep debugging.
    Trace,

    /// Debug level logging
    ///
    /// Includes debug information useful for troubleshooting.
    /// Appropriate for development environments.
    Debug,

    /// Info level logging (default)
    ///
    /// General informational messages about application state and operations.
    /// Appropriate for most production environments.
    #[default]
    /// Represents info variant
    Info,

    /// Warning level logging
    ///
    /// Potentially harmful situations that don't prevent operation.
    /// Recommended for production environments with high traffic.
    Warn,

    /// Error level logging (least verbose)
    ///
    /// Error events that might still allow the application to continue.
    /// Use when only critical errors need to be logged.
    Error,
}

// Compatibility aliases
/// Historical name for [`CanonicalAppConfig`]; prefer the canonical type in new code.
pub type AppConfig = CanonicalAppConfig;
/// Historical name for [`CanonicalAppConfig`]; prefer the canonical type in new code.
pub type ApplicationConfig = CanonicalAppConfig;
