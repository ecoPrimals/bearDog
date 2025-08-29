//! Application Configuration Module
//!
//! Core application configuration types and settings.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core application configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub environment: Environment,
    pub log_level: LogLevel,
    pub enable_metrics: bool,
    pub enable_tracing: bool,
    pub features: HashMap<String, bool>,
    pub rollout_percentages: HashMap<String, f64>,
    pub dependencies: HashMap<String, Vec<String>>,
}

/// Application environment types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Environment {
    #[default]
    Development,
    Testing,
    Staging,
    Production,
}

/// Logging level configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

impl AppConfig {
    /// Create a new app configuration
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            ..Default::default()
        }
    }

    /// Check if a feature is enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.features.get(feature).copied().unwrap_or(false)
    }

    /// Get rollout percentage for a feature
    pub fn get_rollout_percentage(&self, feature: &str) -> f64 {
        self.rollout_percentages.get(feature).copied().unwrap_or(0.0)
    }
} 