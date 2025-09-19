// HSM Performance Configuration

use super::HsmConfigValidation;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedHsmPerformanceConfig {
    /// Max Operations Per Second
    /// Optional max operations per second
    pub max_operations_per_second: Option<u32>,
    /// Concurrent Operations
    /// Number of `concurrent_operations`
    pub concurrent_operations: u32,
    /// Cache Enabled
    /// Whether cache is enabled
    pub cache_enabled: bool,
    /// Batch Processing
    /// Whether `batch_processing` is enabled
    pub batch_processing: bool,
}

impl Default for UnifiedHsmPerformanceConfig {
    fn default() -> Self {
        Self {
            max_operations_per_second: None,
            concurrent_operations: 4,
            cache_enabled: true,
            batch_processing: true,
        }
    }
}

impl HsmConfigValidation for UnifiedHsmPerformanceConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
