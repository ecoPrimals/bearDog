// SPDX-License-Identifier: AGPL-3.0-only

//! Threat detection runtime: handlers, ML scoring, shared types, and test helpers.

use beardog_errors::BearDogError;
pub mod handlers;
/// Heuristic and remote-backed ML scoring for [`SecurityEvent`] inputs.
pub mod ml_engine;
/// Unit and integration tests for threat detection building blocks.
pub mod tests;
pub mod types;

pub use handlers::core::ThreatDetectionEngine;
pub use ml_engine::SmartThreatMLEngine;
pub use types::{
    DetectionRule, MitigationStep, SecurityEvent, ThreatDetectionConfig, ThreatEvent,
    ThreatIndicator, ThreatSeverity, ThreatSource, ThreatStatus, ThreatTarget, ThreatType,
};

/// Convenience entry points for constructing a [`ThreatDetectionEngine`] with optional ML support.
pub struct ThreatAPI;
impl ThreatAPI {
    /// Builds an engine from [`ThreatDetectionConfig::default`].
    ///
    /// # Errors
    /// Propagates configuration validation failures from [`ThreatDetectionEngine::new`].
    pub fn create_default() -> Result<ThreatDetectionEngine, BearDogError> {
        ThreatDetectionEngine::new(ThreatDetectionConfig::default())
    }

    /// Builds a [`ThreatDetectionEngine`] together with a fresh [`SmartThreatMLEngine`] for scoring.
    ///
    /// # Errors
    /// Same as [`Self::create_default`] when the rule engine fails to initialize.
    pub fn new_with_ml() -> Result<(ThreatDetectionEngine, SmartThreatMLEngine), BearDogError> {
        let threat_engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())?;
        let ml_engine = SmartThreatMLEngine::new();
        Ok((threat_engine, ml_engine))
    }
}
