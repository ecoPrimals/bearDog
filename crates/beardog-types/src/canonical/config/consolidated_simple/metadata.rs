//! # Metadata Configuration Module
//!
//! This module contains system metadata, environment definitions, and feature management
//! configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

/// **CONFIGURATION METADATA** - System identification and versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationMetadata {
    /// System version information
    pub version: String,
    /// Deployment environment
    pub environment: Environment,
    /// Unique instance identifier
    pub instance_id: String,
    /// Configuration schema version
    pub schema_version: String,
    /// Configuration creation timestamp
    pub created_at: SystemTime,
    /// Last configuration update timestamp
    pub last_updated: SystemTime,
}

/// **FEATURE CONFIGURATION** - Centralized feature management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
    /// Feature flags
    pub flags: HashMap<String, bool>,
    /// Feature rollout percentages
    pub rollouts: HashMap<String, f64>,
    /// Feature metadata
    pub metadata: HashMap<String, HashMap<String, String>>,
}

/// Deployment environment types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Testing,
    Staging,
    Production,
}

// ============================================================================
// Default Implementations
// ============================================================================

impl Default for ConfigurationMetadata {
    fn default() -> Self {
        Self {
            version: crate::constants::domains::system::BEARDOG_VERSION.to_string(),
            environment: Environment::Development,
            instance_id: Uuid::new_v4().to_string(),
            schema_version: "1.0.0".to_string(),
            created_at: SystemTime::now(),
            last_updated: SystemTime::now(),
        }
    }
}

impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
            flags: HashMap::new(),
            rollouts: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Development
    }
}

// ============================================================================
// Implementation Methods
// ============================================================================

impl Environment {
    /// Check if this is a production environment
    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }

    /// Check if this is a development environment
    pub fn is_development(&self) -> bool {
        matches!(self, Environment::Development)
    }

    /// Check if this is a testing environment
    pub fn is_testing(&self) -> bool {
        matches!(self, Environment::Testing | Environment::Staging)
    }

    /// Get environment as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Testing => "testing",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
}

impl FeatureConfig {
    /// Create new feature config
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable a feature flag
    pub fn enable_feature(&mut self, name: &str) {
        self.flags.insert(name.to_string(), true);
    }

    /// Disable a feature flag
    pub fn disable_feature(&mut self, name: &str) {
        self.flags.insert(name.to_string(), false);
    }

    /// Check if a feature is enabled
    pub fn is_feature_enabled(&self, name: &str) -> bool {
        self.flags.get(name).copied().unwrap_or(false)
    }

    /// Set feature rollout percentage
    pub fn set_rollout(&mut self, name: &str, percentage: f64) {
        self.rollouts
            .insert(name.to_string(), percentage.clamp(0.0, 100.0));
    }

    /// Get feature rollout percentage
    pub fn get_rollout(&self, name: &str) -> f64 {
        self.rollouts.get(name).copied().unwrap_or(0.0)
    }
}
