//! Configuration Builder Module
//!
//! Provides builder pattern for constructing BearDog configurations.

use super::core::BearDogCanonicalConfig;
use super::{
    app::AppConfig,
    network::NetworkConfig,
};
use super::consolidated::{
    SecurityConfig, HsmConfig, DatabaseConfig, MonitoringConfig,
};

/// Configuration builder for fluent configuration construction
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    config: BearDogCanonicalConfig,
}

impl ConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self {
            config: BearDogCanonicalConfig::default(),
        }
    }

    /// Set application configuration
    pub fn app(mut self, app: AppConfig) -> Self {
        self.config.app = app;
        self
    }

    /// Set network configuration
    pub fn network(mut self, network: NetworkConfig) -> Self {
        self.config.network = network;
        self
    }

    /// Set security configuration
    pub fn security(mut self, security: SecurityConfig) -> Self {
        self.config.security = security;
        self
    }

    /// Set HSM configuration
    pub fn hsm(mut self, hsm: HsmConfig) -> Self {
        self.config.hsm = hsm;
        self
    }

    /// Set database configuration
    pub fn database(mut self, database: DatabaseConfig) -> Self {
        self.config.database = database;
        self
    }

    /// Set monitoring configuration
    pub fn monitoring(mut self, monitoring: MonitoringConfig) -> Self {
        self.config.monitoring = monitoring;
        self
    }

    /// Build the final configuration
    pub fn build(self) -> BearDogCanonicalConfig {
        self.config
    }

    /// Build and validate the configuration
    pub fn build_validated(self) -> Result<BearDogCanonicalConfig, Vec<String>> {
        let config = self.config;
        config.validate()?;
        Ok(config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
} 