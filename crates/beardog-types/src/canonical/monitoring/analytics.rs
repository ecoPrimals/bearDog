// SPDX-License-Identifier: AGPL-3.0-or-later

//! Analytics toggles embedded in [`super::MonitoringConfig`].

use super::MonitoringConfigValidation;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Feature flags for real-time, batch, and predictive analytics pipelines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAnalyticsConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Real Time Analytics
    /// Stream processing of metrics/events with sub-second latency. **Default:** `true`.
    pub real_time_analytics: bool,
    /// Batch Analytics
    /// Whether `batch_analytics` is enabled
    pub batch_analytics: bool,
    /// Predictive Analytics
    /// Whether `predictive_analytics` is enabled
    pub predictive_analytics: bool,
}

impl Default for UnifiedAnalyticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            real_time_analytics: true,
            batch_analytics: true,
            predictive_analytics: false,
        }
    }
}

impl MonitoringConfigValidation for UnifiedAnalyticsConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
