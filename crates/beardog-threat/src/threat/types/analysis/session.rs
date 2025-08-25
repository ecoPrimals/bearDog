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


/// Analysis Session Types
///
/// This module contains types for analysis session management and tracking.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::correlation::EventCorrelationResult;
use super::events::SecurityEvent;
use super::metrics::AnalysisMetrics;
use super::results::ThreatAnalysisResult;
/// Analysis session structure
/// Manages a collection of security events and their
/// analysis results within a specific time window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSession {
    /// Unique session identifier
    pub session_id: String,
    /// Session start time
    pub start_time: DateTime<Utc>,
    /// Session end time
    pub end_time: Option<DateTime<Utc>>,
    /// Events in session
    pub events: Vec<SecurityEvent>,
    /// Analysis results
    pub analysis_results: Vec<ThreatAnalysisResult>,
    /// Session metrics
    pub metrics: AnalysisMetrics,
    /// Correlation results
    pub correlations: Vec<EventCorrelationResult>,
}
impl Default for AnalysisSession {}


    fn default() -> Self {
        Self {
            session_id: String::new(),
            start_time: Utc::now(),
            end_time: None,
            events: vec![],
            analysis_results: vec![],
            metrics: AnalysisMetrics::default(),
            correlations: vec![],
        }
    }
impl AnalysisSession {
    /// Create new analysis session
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            start_time: Utc::now(),
            end_time: None,
            events: Vec::new(),
            analysis_results: Vec::new(),
            correlations: Vec::new(),
            metrics: AnalysisMetrics::new(),
        }
    }

    /// End the session
    pub fn end_session(&mut self) {
        self.end_time = Some(Utc::now());
    }

    /// Add event to session
    pub fn add_event(&mut self, event: SecurityEvent) {
        self.events.push(event);
    }

    /// Add analysis result to session
    pub fn add_analysis_result(&mut self, result: ThreatAnalysisResult) {
        // Update metrics
        self.metrics
            .update_with_analysis(result.analysis_time_ms, result.threats_detected);
        // Add result
        self.analysis_results.push(result);
    }

    /// Add correlation result to session
    pub fn add_correlation(&mut self, correlation: EventCorrelationResult) {
        self.correlations.push(correlation);
    }

    /// Get session duration in minutes
    pub fn duration_minutes(&self) -> i64 {
        let end_time = self.end_time.unwrap_or_else(Utc::now);
        (end_time - self.start_time).num_minutes()
    }

    /// Get total threats detected in session
    pub fn total_threats_detected(&self) -> usize {
        self.analysis_results
            .iter()
            .map(|r| r.threats_detected)
            .sum()
    }

    /// Get events analyzed count
    pub fn events_analyzed_count(&self) -> usize {
        self.analysis_results.len()
    }

    /// Get pending events count
    pub fn pending_events_count(&self) -> usize {
        self.events
            .len()
            .saturating_sub(self.events_analyzed_count())
    }

    /// Get correlation count
    pub fn correlation_count(&self) -> usize {
        self.correlations.len()
    }

    /// Get significant correlations
    pub fn significant_correlations(&self) -> Vec<&EventCorrelationResult> {
        self.correlations
            .iter()
            .filter(|c| c.is_significant())
            .collect()
    }

    /// Get session statistics
    pub fn session_statistics(&self) -> SessionStatistics {
        SessionStatistics {
            session_id: self.session_id.clone(),
            duration_minutes: self.duration_minutes(),
            total_events: self.events.len(),
            events_analyzed: self.events_analyzed_count(),
            pending_events: self.pending_events_count(),
            total_threats: self.total_threats_detected(),
            correlations_found: self.correlation_count(),
            significant_correlations: self.significant_correlations().len(),
            avg_analysis_time_ms: self.metrics.avg_analysis_time_ms,
            threat_detection_rate: self.metrics.threat_detection_rate(),
        }
    }

    /// Check if session is active
    pub fn is_active(&self) -> bool {
        self.end_time.is_none()
    }

    /// Get session summary
    pub fn summary(&self) -> String {
        format!(
            "Session {}: {} events, {} threats detected, {} correlations in {} minutes",
            self.session_id,
            self.events.len(),
            self.total_threats_detected(),
            self.correlation_count(),
            self.duration_minutes()
        )
    }
}

/// Session statistics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatistics {
    /// Session ID
    pub session_id: String,
    /// Duration in minutes
    pub duration_minutes: i64,
    /// Total events in session
    pub total_events: usize,
    /// Events analyzed
    pub events_analyzed: usize,
    /// Pending events
    pub pending_events: usize,
    /// Total threats detected
    pub total_threats: usize,
    /// Correlations found
    pub correlations_found: usize,
    /// Significant correlations
    pub significant_correlations: usize,
    /// Average analysis time
    pub avg_analysis_time_ms: f64,
    /// Threat detection rate
    pub threat_detection_rate: f64,
}
