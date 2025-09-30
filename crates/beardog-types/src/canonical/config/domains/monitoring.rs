//! # Consolidated Monitoring Configuration Domain
//!
//! This module consolidates ALL monitoring-related configuration structs across the BearDog
//! ecosystem into a single, unified monitoring configuration system.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **CONSOLIDATED MONITORING CONFIGURATION** - Single source of truth for all monitoring settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedMonitoringConfiguration {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics collection interval seconds
    pub metrics_interval_seconds: u64,
    /// Alert configuration
    pub alerts: HashMap<String, AlertConfiguration>,
}

/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfiguration {
    /// Enable alerts
    pub enabled: bool,
    /// Alert threshold
    pub threshold: f64,
}

impl Default for ConsolidatedMonitoringConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval_seconds: 60,
            alerts: HashMap::new(),
        }
    }
}

impl ConsolidatedMonitoringConfiguration {
    /// Validate monitoring configuration
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.metrics_interval_seconds == 0 {
            return Err(BearDogError::configuration("Metrics interval cannot be zero"));
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