// SPDX-License-Identifier: AGPL-3.0-only

//! # Threat Detection Configuration - Canonical Location
//!
//! **Unified threat detection configuration** - consolidates scattered threat configs.
//!
//! This module provides the canonical configuration types for the BearDog threat
//! detection system, eliminating fragmentation from multiple duplicate definitions.
//!
//! ## Consolidation
//!
//! This replaces:
//! - `beardog-threat/src/threat/types/modules/core.rs::ThreatDetectionConfig`
//! - `beardog-threat/src/threat/types/mod.rs::ThreatDetectionConfig`
//! - `beardog-threat/src/threat/handlers/analysis.rs::ThreatDetectionConfig`
//! - `beardog-types/src/canonical/monitoring/security.rs::ThreatDetectionConfig`
//!
//! All threat configuration should now use this canonical definition.

use crate::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Canonical threat detection configuration
///
/// Consolidates all threat detection configuration options from scattered definitions
/// into a single, comprehensive configuration type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalThreatDetectionConfig {
    // Core Detection Settings
    /// Enable machine learning enhancement for threat detection
    pub ml_enhancement: bool,

    /// Enable real-time monitoring and analysis
    pub real_time_monitoring: bool,

    /// Enable automatic response to detected threats
    pub auto_response: bool,

    // Capacity & Performance
    /// Maximum number of active threats to track
    pub max_active_threats: usize,

    /// Maximum number of concurrent threat analyses
    pub max_concurrent_analyses: usize,

    // Sensitivity & Thresholds
    /// Threat detection sensitivity level (0.0 - 1.0)
    /// Higher values = more sensitive (more false positives)
    /// Lower values = less sensitive (more false negatives)
    pub sensitivity: f64,

    /// Sensitivity level (categorical alternative to numeric sensitivity)
    pub sensitivity_level: SensitivityLevel,

    /// Quarantine threshold score (0.0 - 1.0)
    pub quarantine_threshold: f64,

    /// Block threshold score (0.0 - 1.0)
    pub block_threshold: f64,

    // Rules & Patterns
    /// Detection rules (rule IDs or definitions)
    pub detection_rules: Vec<String>,

    /// Custom threat detection patterns
    pub custom_patterns: Vec<String>,

    // Integration Features
    /// Enable external threat intelligence feeds
    pub enable_threat_feeds: bool,

    /// Additional configuration options
    pub additional_config: HashMap<String, String>,
}

impl CanonicalThreatDetectionConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::{get_bool, get_parsed};

        Self {
            // Core Detection
            ml_enhancement: get_bool(source, "BEARDOG_THREAT_ML_ENHANCEMENT", true),
            real_time_monitoring: get_bool(source, "BEARDOG_THREAT_REAL_TIME_MONITORING", true),
            auto_response: get_bool(source, "BEARDOG_THREAT_AUTO_RESPONSE", false),

            // Capacity
            max_active_threats: get_parsed(source, "BEARDOG_THREAT_MAX_ACTIVE", DEFAULT_QUEUE_SIZE),
            max_concurrent_analyses: get_parsed(
                source,
                "BEARDOG_THREAT_MAX_CONCURRENT_ANALYSES",
                10,
            ),

            // Sensitivity
            sensitivity: get_parsed(source, "BEARDOG_THREAT_SENSITIVITY", 0.7),
            sensitivity_level: SensitivityLevel::Normal,
            quarantine_threshold: get_parsed(source, "BEARDOG_THREAT_QUARANTINE_THRESHOLD", 0.8),
            block_threshold: get_parsed(source, "BEARDOG_THREAT_BLOCK_THRESHOLD", 0.9),

            // Rules & Patterns
            detection_rules: Vec::new(),
            custom_patterns: Vec::new(),

            // Integration
            enable_threat_feeds: get_bool(source, "BEARDOG_THREAT_ENABLE_FEEDS", true),
            additional_config: HashMap::new(),
        }
    }
}

impl Default for CanonicalThreatDetectionConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

/// Sensitivity level for threat detection (categorical)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SensitivityLevel {
    /// Low sensitivity - fewer false positives, may miss threats
    Low,
    /// Normal sensitivity - balanced approach
    Normal,
    /// High sensitivity - more false positives, catches more threats
    High,
    /// Maximum sensitivity - maximum detection, many false positives
    Maximum,
}

impl Default for SensitivityLevel {
    fn default() -> Self {
        Self::Normal
    }
}

impl SensitivityLevel {
    /// Convert sensitivity level to numeric value (0.0 - 1.0)
    pub fn to_numeric(&self) -> f64 {
        match self {
            Self::Low => 0.5,
            Self::Normal => 0.7,
            Self::High => 0.85,
            Self::Maximum => 0.95,
        }
    }

    /// Create sensitivity level from numeric value
    pub fn from_numeric(value: f64) -> Self {
        if value < 0.6 {
            Self::Low
        } else if value < 0.75 {
            Self::Normal
        } else if value < 0.9 {
            Self::High
        } else {
            Self::Maximum
        }
    }
}

/// Threat response action configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatResponseConfig {
    /// Enable automatic blocking of detected threats
    pub auto_block: bool,

    /// Enable automatic quarantine of suspicious activity
    pub auto_quarantine: bool,

    /// Enable alerting for threat events
    pub enable_alerts: bool,

    /// Alert severity threshold
    pub alert_threshold: f64,

    /// Maximum response actions per minute (rate limiting)
    pub max_actions_per_minute: usize,
}

impl ThreatResponseConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::{get_bool, get_parsed};

        Self {
            auto_block: get_bool(source, "BEARDOG_THREAT_AUTO_BLOCK", false),
            auto_quarantine: get_bool(source, "BEARDOG_THREAT_AUTO_QUARANTINE", true),
            enable_alerts: get_bool(source, "BEARDOG_THREAT_ENABLE_ALERTS", true),
            alert_threshold: get_parsed(source, "BEARDOG_THREAT_ALERT_THRESHOLD", 0.7),
            max_actions_per_minute: get_parsed(
                source,
                "BEARDOG_THREAT_MAX_ACTIONS_PER_MINUTE",
                100,
            ),
        }
    }
}

impl Default for ThreatResponseConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

/// Unified threat system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedThreatConfig {
    /// Threat detection configuration
    pub detection: CanonicalThreatDetectionConfig,

    /// Threat response configuration
    pub response: ThreatResponseConfig,

    /// Enable comprehensive logging
    pub enable_logging: bool,

    /// Enable metrics collection
    pub enable_metrics: bool,
}

impl Default for UnifiedThreatConfig {
    fn default() -> Self {
        Self {
            detection: CanonicalThreatDetectionConfig::default(),
            response: ThreatResponseConfig::default(),
            enable_logging: true,
            enable_metrics: true,
        }
    }
}

// Backward compatibility type aliases
/// Backward compatibility alias
pub type ThreatDetectionConfig = CanonicalThreatDetectionConfig;

/// Backward compatibility alias for older code
pub type ThreatConfig = UnifiedThreatConfig;
