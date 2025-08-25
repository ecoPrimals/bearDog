// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Threat Detection and Response Module
///
/// This module provides comprehensive threat detection and response capabilities
/// including rule-based detection, behavioral analysis, threat intelligence, and
/// automated incident response.
/// # Key Features
/// * **Rule-Based Detection**: Configurable detection rules and conditions
/// * **Behavioral Analysis**: User and entity behavior analysis for anomaly detection
/// * **Machine Learning**: AI-enhanced anomaly detection and pattern recognition
/// * **Threat Intelligence**: Integration with external threat feeds
/// * **Automated Response**: Configurable incident response workflows
/// * **MITRE ATT&CK Mapping**: Threat categorization using MITRE framework
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
