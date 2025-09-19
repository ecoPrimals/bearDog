// Canonical Application Configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Canonical application configuration - consolidates all app-related configs
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

/// Log levels
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LogLevel {
    /// Trace level logging (most verbose)
    Trace,
    /// Debug level logging
    Debug,
    /// Info level logging (default)
    #[default]
    /// Represents info variant
    Info,
    /// Warning level logging
    Warn,
    /// Error level logging (least verbose)
    Error,
}

// Compatibility aliases
pub type AppConfig = CanonicalAppConfig;
pub type ApplicationConfig = CanonicalAppConfig;
