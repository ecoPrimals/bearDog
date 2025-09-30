//! # Cache Configuration Domain

use crate::canonical::config::r#trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfiguration {
    /// Whether caching is enabled
    pub enabled: bool,
    /// Cache size limit in megabytes
    pub size_mb: usize,
    /// Default time-to-live for cache entries in seconds
    pub default_ttl_seconds: u64,
    /// Whether cache persistence to disk is enabled
    pub persistence_enabled: bool,
}

impl Default for CacheConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            size_mb: 256,
            default_ttl_seconds: 3600,
            persistence_enabled: false,
        }
    }
}

impl BearDogConfig for CacheConfiguration {
    fn validate(&self) -> BearDogResult<()> { Ok(()) }
    fn to_toml(&self) -> BearDogResult<String> { 
        toml::to_string(self).map_err(|e| BearDogError::system(e.to_string()))
    }
    fn from_env() -> BearDogResult<Self> { Ok(Self::default()) }
    fn merge(&self, _other: &Self) -> BearDogResult<Self> { Ok(self.clone()) }
    fn domain() -> &'static str { "cache" }
} 