//! # Genetics Configuration Domain

use crate::canonical::config::unified_trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};

/// Genetics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsConfiguration {
    /// Whether genetic algorithms are enabled
    pub enabled: bool,
    /// Size of the genetic algorithm population
    pub population_size: usize,
}

impl Default for GeneticsConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            population_size: 100,
        }
    }
}

impl BearDogConfig for GeneticsConfiguration {
    fn validate(&self) -> BearDogResult<()> { Ok(()) }
    fn to_toml(&self) -> BearDogResult<String> { 
        toml::to_string(self).map_err(|e| BearDogError::system(e.to_string()))
    }
    fn from_env() -> BearDogResult<Self> { Ok(Self::default()) }
    fn merge(&self, _other: &Self) -> BearDogResult<Self> { Ok(self.clone()) }
    fn domain() -> &'static str { "genetics" }
} 