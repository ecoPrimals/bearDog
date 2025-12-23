// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
pub mod handlers;
pub mod ml_engine;
/// Test utilities and fixtures
/// Test utilities and fixtures
pub mod tests;
pub mod types;

pub use handlers::core::ThreatDetectionEngine;
pub use ml_engine::SmartThreatMLEngine;
pub use types::{
    DetectionRule, MitigationStep, SecurityEvent, ThreatDetectionConfig, ThreatEvent,
    ThreatIndicator, ThreatSeverity, ThreatSource, ThreatStatus, ThreatTarget, ThreatType,
};

pub struct ThreatAPI;
impl ThreatAPI {
    /// Create a default threat detection engine
    ///
    /// # Errors
    /// Returns an error if the engine configuration is invalid
    /// Creates default
    /// Creates default
    pub fn create_default() -> Result<ThreatDetectionEngine, BearDogError> {
        ThreatDetectionEngine::new(ThreatDetectionConfig::default())
    }

    /// Create a new threat detection engine with ML capabilities
    ///
    /// # Returns
    /// Returns a tuple containing the threat engine and ML engine
    ///
    /// # Errors
    /// Returns an error if engine initialization fails
    /// Creates a new instance
    /// Creates a new instance
    pub fn new_with_ml() -> Result<(ThreatDetectionEngine, SmartThreatMLEngine), BearDogError> {
        let threat_engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())?;
        let ml_engine = SmartThreatMLEngine::new();
        Ok((threat_engine, ml_engine))
    }
}
