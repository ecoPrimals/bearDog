//! Observability Configuration
//!
//! This module defines observability configurations for monitoring.

use serde::{Deserialize, Serialize};

/// Observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Enable observability
    pub enabled: bool,
    /// Tracing enabled
    pub tracing: bool,
    /// Logging enabled
    pub logging: bool,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            tracing: true,
            logging: true,
        }
    }
}

impl ObservabilityConfig {
    /// Create production observability configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            tracing: true,
            logging: true,
        }
    }

    /// Create development observability configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            tracing: false,
            logging: false,
        }
    }
}
