//! # Production Configuration Domain

use crate::canonical::config::r#trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};

/// Production configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfiguration {
    /// Whether production mode is enabled
    pub enabled: bool,
    /// Environment name (e.g., "production", "staging", "development")
    pub environment: String,
}

impl Default for ProductionConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            environment: "development".to_string(),
        }
    }
}

impl BearDogConfig for ProductionConfiguration {
    fn validate(&self) -> BearDogResult<()> { Ok(()) }
    fn to_toml(&self) -> BearDogResult<String> { 
        toml::to_string(self).map_err(|e| BearDogError::system(e.to_string()))
    }
    fn from_env() -> BearDogResult<Self> { Ok(Self::default()) }
    fn merge(&self, _other: &Self) -> BearDogResult<Self> { Ok(self.clone()) }
    fn domain() -> &'static str { "production" }
} 