//! # Database Configuration Domain

use crate::canonical::config::unified_trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfiguration {
    /// Whether database functionality is enabled
    pub enabled: bool,
    /// List of database connection strings
    pub connections: Vec<String>,
}

impl Default for DatabaseConfiguration {
    fn default() -> Self {
        Self {
            enabled: false,
            connections: vec!["sqlite://beardog.db".to_string()],
        }
    }
}

impl BearDogConfig for DatabaseConfiguration {
    fn validate(&self) -> BearDogResult<()> { Ok(()) }
    fn to_toml(&self) -> BearDogResult<String> { 
        toml::to_string(self).map_err(|e| BearDogError::system(e.to_string()))
    }
    fn from_env() -> BearDogResult<Self> { Ok(Self::default()) }
    fn merge(&self, _other: &Self) -> BearDogResult<Self> { Ok(self.clone()) }
    fn domain() -> &'static str { "database" }
} 