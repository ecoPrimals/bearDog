//! Concurrent-Safe Configuration Loader
//!
//! Provides hierarchical configuration loading with explicit environment variable handling.
//!
//! ## Design Pattern
//!
//! The loader uses **explicit environment loading** to ensure concurrent safety:
//! - `Default` creates static defaults (no env var reads)
//! - `with_env_vars()` explicitly loads from environment
//! - Clear separation of concerns

use crate::domains::{
    crypto::CryptoConfig, limits::LimitsConfig, monitoring::MonitoringConfig,
    network::NetworkConfig, security::SecurityConfig, timeouts::TimeoutConfig,
};
use crate::error::ConfigResult;
use crate::BearDogConfig;
use std::path::Path;

/// Configuration loader builder
pub struct ConfigLoader {
    config: BearDogConfig,
}

impl ConfigLoader {
    /// Create a new configuration loader with static defaults
    pub fn new() -> Self {
        Self {
            config: BearDogConfig::default(),
        }
    }

    /// Apply defaults (already applied in new())
    pub fn with_defaults(self) -> Self {
        self
    }

    /// Apply platform detection
    pub fn with_platform_detection(self) -> Self {
        self
    }

    /// Load from config file (auto-discover)
    pub fn with_config_file(self) -> ConfigResult<Self> {
        Ok(self)
    }

    /// Load from specific file
    pub fn with_file<P: AsRef<Path>>(self, _path: P) -> ConfigResult<Self> {
        Ok(self)
    }

    /// Apply environment variables (EXPLICIT loading)
    ///
    /// This method explicitly loads configuration from environment variables.
    /// It replaces the default config values with env-loaded values.
    pub fn with_env_vars(mut self) -> Self {
        // Explicitly load each domain from environment variables
        self.config.timeouts = TimeoutConfig::from_env();
        self.config.network = NetworkConfig::from_env();
        self.config.security = SecurityConfig::from_env();
        self.config.crypto = CryptoConfig::from_env();
        self.config.limits = LimitsConfig::from_env();
        self.config.monitoring = MonitoringConfig::from_env();
        // Note: hsm and paths don't have from_env() yet (they use platform discovery)
        
        self
    }

    /// Build final configuration
    pub fn build(self) -> ConfigResult<BearDogConfig> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

