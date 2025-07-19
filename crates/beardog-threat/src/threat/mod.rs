//! Threat Detection and Response Module
//!
//! This module provides comprehensive threat detection and response capabilities
//! including rule-based detection, behavioral analysis, threat intelligence, and
//! automated incident response.
//!
//! # Key Features
//!
//! * **Rule-Based Detection**: Configurable detection rules and conditions
//! * **Behavioral Analysis**: User and entity behavior analysis for anomaly detection
//! * **Machine Learning**: AI-enhanced anomaly detection and pattern recognition
//! * **Threat Intelligence**: Integration with external threat feeds
//! * **Automated Response**: Configurable incident response workflows
//! * **MITRE ATT&CK Mapping**: Threat categorization using MITRE framework

pub mod handlers;
pub mod ml_engine;
pub mod tests;
pub mod types;

// Export specific items to avoid conflicts
pub use handlers::core::ThreatDetectionEngine;
pub use ml_engine::{MlEngineConfig, MlThreatEngine};
pub use types::{
    DetectionRule, MitigationStep, SecurityEvent, ThreatDetectionConfig, ThreatEvent,
    ThreatIndicator, ThreatSeverity, ThreatSource, ThreatStatus, ThreatTarget, ThreatType,
};

/// Quick access to threat detection functionality
pub struct ThreatAPI;

impl ThreatAPI {
    /// Create a new threat detection engine with default configuration
    pub async fn create_default() -> beardog_errors::BearDogResult<ThreatDetectionEngine> {
        ThreatDetectionEngine::new(ThreatDetectionConfig::default()).await
    }

    /// Create a new ML-powered threat detection engine
    pub async fn new_with_ml(
    ) -> beardog_errors::BearDogResult<(ThreatDetectionEngine, MlThreatEngine)> {
        let threat_engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default()).await?;
        let ml_engine = MlThreatEngine::new(MlEngineConfig::default()).await?;
        Ok((threat_engine, ml_engine))
    }
}
