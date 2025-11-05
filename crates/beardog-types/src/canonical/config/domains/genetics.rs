//! # Consolidated Genetics Configuration Domain
//!
//! This module consolidates ALL genetics-related configuration structs across the BearDog
//! ecosystem into a single, unified genetics configuration system.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// **CONSOLIDATED GENETICS CONFIGURATION** - Single source of truth for all genetics settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedGeneticsConfiguration {
    /// Enable genetics
    pub enabled: bool,
    /// Mutation rate
    pub mutation_rate: f64,
    /// Population size
    pub population_size: usize,
}

impl ConsolidatedGeneticsConfiguration {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_parsed;
        
        Self {
            enabled: true,
            mutation_rate: 0.1,
            population_size: get_parsed(source, "BEARDOG_GENETICS_POPULATION_SIZE", 100),
        }
    }

    /// Validate genetics configuration
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.mutation_rate < 0.0 || self.mutation_rate > 1.0 {
            return Err(BearDogError::configuration("Mutation rate must be between 0.0 and 1.0"));
        }
        Ok(())
    }
    
    /// Create development configuration
    pub fn development() -> Self {
        Self::default()
    }
    
    /// Create production configuration
    pub fn production() -> Self {
        Self::default()
    }
}

impl Default for ConsolidatedGeneticsConfiguration {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
} 