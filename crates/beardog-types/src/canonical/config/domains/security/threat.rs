// SPDX-License-Identifier: AGPL-3.0-only

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

impl ThreatResponseConfiguration {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_bool;

        Self {
            enabled: get_bool(source, "BEARDOG_THREAT_RESPONSE_ENABLED", false),
            max_response_level: source.get_or("BEARDOG_THREAT_RESPONSE_MAX_LEVEL", "medium"),
            enable_isolation: get_bool(source, "BEARDOG_THREAT_RESPONSE_ISOLATION", false),
            enable_blocking: get_bool(source, "BEARDOG_THREAT_RESPONSE_BLOCKING", false),
            enable_enhanced_monitoring: get_bool(
                source,
                "BEARDOG_THREAT_RESPONSE_ENHANCED_MONITORING",
                true,
            ),
            collect_forensics: get_bool(source, "BEARDOG_THREAT_RESPONSE_COLLECT_FORENSICS", true),
            update_intelligence: get_bool(
                source,
                "BEARDOG_THREAT_RESPONSE_UPDATE_INTELLIGENCE",
                true,
            ),
        }
    }
}

impl Default for ThreatResponseConfiguration {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}
