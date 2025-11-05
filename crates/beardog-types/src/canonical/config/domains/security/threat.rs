//! Threat Response Configuration
//!
//! This module provides automated threat response configuration
//! for the BearDog security system.

use serde::{Deserialize, Serialize};

/// **THREAT RESPONSE CONFIGURATION** - Automated threat response settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatResponseConfiguration {
    /// Enable automated threat response
    pub enabled: bool,
    /// Maximum response level severity
    pub max_response_level: String,
    /// Enable automatic isolation
    pub enable_isolation: bool,
    /// Enable automatic blocking
    pub enable_blocking: bool,
    /// Enable enhanced monitoring on threats
    pub enable_enhanced_monitoring: bool,
    /// Collect forensics data
    pub collect_forensics: bool,
    /// Update threat intelligence
    pub update_intelligence: bool,
}

impl Default for ThreatResponseConfiguration {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled by default for safety
            max_response_level: "medium".to_string(),
            enable_isolation: false,
            enable_blocking: false,
            enable_enhanced_monitoring: true,
            collect_forensics: true,
            update_intelligence: true,
        }
    }
}
