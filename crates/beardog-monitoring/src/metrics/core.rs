// Core Metrics System
//
// Fundamental metrics storage, management, and processing functionality.

use beardog_errors::BearDogError;
// Removed unused import: use crate::monitoring::types::*;

/// Core metrics functionality
#[derive(Debug, Clone)]
pub struct MetricsCore {
    config: MetricsCoreConfig,
}

#[derive(Debug, Clone)]
pub struct MetricsCoreConfig {
    /// Whether core metrics are enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Metrics collection interval in seconds
    /// Number of `collection_interval_seconds`
    pub collection_interval_seconds: u64,
}

impl Default for MetricsCoreConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval_seconds: 60,
        }
    }
}

impl MetricsCore {
    /// Creates a new metrics core instance
    ///
    /// # Errors
    /// Returns an error if the metrics core cannot be initialized
    /// Creates a new instance
    pub const fn new(config: MetricsCoreConfig) -> Result<Self, BearDogError> {
        Ok(Self { config })
    }

    /// Starts the core metrics system
    ///
    /// # Errors
    /// Returns an error if the metrics system cannot be started
    /// Starts service
    /// Starts service
    pub fn start(&self) -> Result<(), BearDogError> {
        tracing::info!("Core metrics system started");
        Ok(())
    }

    /// Gets metrics configuration
    #[must_use]
    pub const fn get_config(&self) -> &MetricsCoreConfig {
        &self.config
    }
}
