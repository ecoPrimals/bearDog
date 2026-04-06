// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Consolidated HSM Configuration Domain
//!
//! This module consolidates ALL HSM-related configuration structs across the BearDog
//! ecosystem into a single, unified HSM configuration system.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// **CONSOLIDATED HSM CONFIGURATION** - Single source of truth for all HSM settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedHsmConfiguration {
    /// Enable HSM
    pub enabled: bool,
    /// HSM provider type
    pub provider: String,
    /// HSM connection timeout seconds
    pub timeout_seconds: u64,
}

impl Default for ConsolidatedHsmConfiguration {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "software".to_string(),
            timeout_seconds: 30,
        }
    }
}

impl ConsolidatedHsmConfiguration {
    /// Validate HSM configuration
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.timeout_seconds == 0 {
            return Err(BearDogError::configuration("HSM timeout cannot be zero"));
        }
        Ok(())
    }
    
    /// Create development configuration
    pub fn development() -> Self {
        Self::default()
    }
    
    /// Create production configuration
    pub fn production() -> Self {
        let mut config = Self::default();
        config.enabled = true;
        config.provider = "hardware".to_string();
        config
    }
} 