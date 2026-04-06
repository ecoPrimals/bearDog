// SPDX-License-Identifier: AGPL-3.0-or-later

// Unified Tracing Configuration

use super::MonitoringConfigValidation;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Unified Tracing Configuration
///
/// ecosystem, including sampling rates, trace duration limits, and batch processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedTracingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Sampling Rate
    /// The sampling rate value
    pub sampling_rate: f64,
    /// Max Trace Duration
    /// The max trace duration value
    pub max_trace_duration: Duration,
    /// Batch Size
    /// Number of `batch_size`
    pub batch_size: usize,
}

impl Default for UnifiedTracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sampling_rate: 0.1,
            max_trace_duration: Duration::from_secs(300),
            batch_size: 512,
        }
    }
}

impl MonitoringConfigValidation for UnifiedTracingConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        if self.sampling_rate < 0.0 || self.sampling_rate > 1.0 {
            return Err(BearDogError::business(
                "Sampling rate must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(())
    }
    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
