//! # Performance Configuration Domain

use crate::canonical::config::unified_trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfiguration {
    /// Whether performance optimizations are enabled
    pub enabled: bool,
    /// Cache size for performance optimizations
    pub cache_size: usize,
}

impl Default for PerformanceConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_size: 1000,
        }
    }
}

impl BearDogConfig for PerformanceConfiguration {
    fn validate(&self) -> BearDogResult<()> { Ok(()) }
    fn to_toml(&self) -> BearDogResult<String> { 
        toml::to_string(self).map_err(|e| BearDogError::system(e.to_string()))
    }
    fn from_env() -> BearDogResult<Self> { Ok(Self::default()) }
    fn merge(&self, _other: &Self) -> BearDogResult<Self> { Ok(self.clone()) }
    fn domain() -> &'static str { "performance" }
} 