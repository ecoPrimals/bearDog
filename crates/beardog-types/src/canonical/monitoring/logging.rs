// SPDX-License-Identifier: AGPL-3.0-or-later

//! Logging level, format, and rotation settings for the canonical monitoring bundle.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
// Removed unused import: use std::time::Duration;
use super::MonitoringConfigValidation;

/// Primary logging sink configuration (level, encoder, rotation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedLoggingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Level
    /// The level value
    pub level: String,
    /// Output encoder (`json`, `pretty`, `compact`). **Default:** `"json"`.
    pub format: String,
    /// Rotation
    /// The rotation value
    pub rotation: LogRotationConfig,
}

impl Default for UnifiedLoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: "info".to_string(),
            format: "json".to_string(),
            rotation: LogRotationConfig::default(),
        }
    }
}

/// Size- and count-based rotation for on-disk log files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Max Size Mb
    /// Number of `max_size_mb`
    pub max_size_mb: u64,
    /// Max Files
    /// Number of `max_files`
    pub max_files: u32,
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size_mb: 100,
            max_files: 10,
        }
    }
}

impl MonitoringConfigValidation for UnifiedLoggingConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
