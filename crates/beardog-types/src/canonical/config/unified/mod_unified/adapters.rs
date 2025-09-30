//! # Adapters Configuration Domain

use crate::canonical::config::unified_trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};

/// Adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfiguration {
    /// Whether universal adapters are enabled
    pub enabled: bool,
    /// Timeout for adapter operations in seconds
    pub timeout_seconds: u64,
}

impl Default for AdapterConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout_seconds: 30,
        }
    }
}

impl BearDogConfig for AdapterConfiguration {
    fn validate(&self) -> BearDogResult<()> { Ok(()) }
    fn to_toml(&self) -> BearDogResult<String> { 
        toml::to_string(self).map_err(|e| BearDogError::system(e.to_string()))
    }
    fn from_env() -> BearDogResult<Self> { Ok(Self::default()) }
    fn merge(&self, _other: &Self) -> BearDogResult<Self> { Ok(self.clone()) }
    fn domain() -> &'static str { "adapters" }
} 