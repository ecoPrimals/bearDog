//! # HSM Configuration Domain

use crate::canonical::config::unified_trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};

/// HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfiguration {
    /// Whether HSM functionality is enabled
    pub enabled: bool,
    /// HSM provider name (e.g., "yubihsm", "softhsm", "aws-cloudhsm")
    pub provider: String,
}

impl Default for HsmConfiguration {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "software".to_string(),
        }
    }
}

impl BearDogConfig for HsmConfiguration {
    fn validate(&self) -> BearDogResult<()> { Ok(()) }
    fn to_toml(&self) -> BearDogResult<String> { 
        toml::to_string(self).map_err(|e| BearDogError::system(e.to_string()))
    }
    fn from_env() -> BearDogResult<Self> { Ok(Self::default()) }
    fn merge(&self, _other: &Self) -> BearDogResult<Self> { Ok(self.clone()) }
    fn domain() -> &'static str { "hsm" }
} 