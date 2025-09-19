// Encryption Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Algorithms
    /// Collection of algorithms
    pub algorithms: Vec<String>,
}

impl EncryptionConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
