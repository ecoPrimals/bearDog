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


/// Threat Analysis Result Types
///
/// This module contains types for threat analysis results and outcomes.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::threat::ml_engine::MlPrediction;
use crate::threat::types::core::ThreatEvent;
/// Threat analysis result structure
/// Contains the results of analyzing a security event
/// for potential threats and anomalies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysisResult {
    /// Unique analysis identifier
    pub analysis_id: String,
    /// Event ID that was analyzed
    pub event_id: String,
    /// Analysis timestamp
    pub timestamp: DateTime<Utc>,
    /// Number of threats detected
    pub threats_detected: usize,
    /// Analysis confidence score (0.0 to 1.0)
    pub confidence_score: f64,
    /// Time taken for analysis in milliseconds
    pub analysis_time_ms: u64,
    /// Detected threat events
    pub threat_events: Vec<ThreatEvent>,
    /// ML predictions (if available)
    pub ml_predictions: Vec<MlPrediction>,
    /// Additional analysis metadata
    pub metadata: HashMap<String, String>,
}
impl Default for ThreatAnalysisResult {}


    fn default() -> Self {
        Self {
            analysis_id: String::new(),
            event_id: String::new(),
            timestamp: Utc::now(),
            threats_detected: 0,
            confidence_score: 0.0,
            analysis_time_ms: 0,
            threat_events: Vec::new(),
            ml_predictions: Vec::new(),
            metadata: HashMap::new(),
        }
    }
impl ThreatAnalysisResult {
    /// Create a new threat analysis result
    pub fn new(analysis_id: String, event_id: String) -> Self {
        Self {
            analysis_id,
            event_id,
            threat_events: Vec::new(),
            ml_predictions: Vec::new(),
            threats_detected: 0,
            confidence_score: 0.0,
            analysis_time_ms: 0,
            analysis_timestamp: chrono::Utc::now(),
            recommendations: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Add a threat event to the result


    pub fn add_threat_event(&mut self, event: ThreatEvent) {
        self.threat_events.push(event);
        self.threats_detected = self.threat_events.len();
    }

    /// Add ML prediction to the result
    pub fn add_ml_prediction(&mut self, prediction: MlPrediction) {
        self.ml_predictions.push(prediction);
    }

    /// Set analysis timing


    pub fn set_analysis_time(&mut self, time_ms: u64) {
        self.analysis_time_ms = time_ms;
    }

    /// Set confidence score
    pub fn set_confidence_score(&mut self, score: f64) {
        self.confidence_score = score.clamp(0.0, 1.0);
    }

    /// Check if analysis found any threats


    pub fn has_threats(&self) -> bool {
        self.threats_detected > 0
    }

    /// Get the highest threat severity
    pub fn highest_threat_severity(&self) -> Option<crate::threat::types::ThreatSeverity> {
        self.threat_events.iter().map(|e| e.severity.clone()).max()
    }

    /// Get total threat score


    pub fn total_threat_score(&self) -> f64 {
        self.threat_events.iter().map(|e| e.score as f64).sum()
    }

    /// Get analysis summary
    pub fn summary(&self) -> String {
        format!(
            "Analysis {} found {} threats with {:.2}% confidence in {}ms",
            self.analysis_id,
            self.threats_detected,
            self.confidence_score * 100.0,
            self.analysis_time_ms
        )
    }

    /// Add a recommendation to the analysis result
    pub fn add_recommendation(&mut self, recommendation: String) {
        self.metadata
            .insert("recommendation".to_string(), recommendation);
    }
}
