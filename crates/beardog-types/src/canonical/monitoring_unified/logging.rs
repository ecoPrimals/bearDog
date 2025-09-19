// Logging Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Level
    /// The level value
    pub level: String,
}

impl LoggingConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
