// SPDX-License-Identifier: AGPL-3.0-only

//! # Threat Statistics
//!
//! This module provides types for tracking threat detection statistics
//! including detection rates, trends, and geographic distribution.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::core::ThreatSeverity;

// ============================================================
// Detection Stats
// ============================================================

/// Threat detection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionStats {
    /// Number of detections
    pub detections: u64,

    /// Number of false positives
    pub false_positives: u64,

    /// Detection accuracy (0.0 - 1.0)
    pub accuracy: f64,

    /// Average confidence score
    pub avg_confidence: f64,

    /// Last update timestamp
    pub last_updated: DateTime<Utc>,

    /// Number of blocked sources
    pub blocked_sources: u64,

    /// Number of quarantined systems
    pub quarantined_systems: u64,

    /// Total threats detected
    pub total_threats: u64,
}

impl Default for ThreatDetectionStats {
    fn default() -> Self {
        Self {
            detections: 0,
            false_positives: 0,
            accuracy: 0.0,
            avg_confidence: 0.0,
            last_updated: Utc::now(),
            blocked_sources: 0,
            quarantined_systems: 0,
            total_threats: 0,
        }
    }
}

impl ThreatDetectionStats {
    /// Create new stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate accuracy
    pub fn calculate_accuracy(&self) -> f64 {
        if self.detections == 0 {
            return 0.0;
        }
        let true_positives = self.detections.saturating_sub(self.false_positives);
        true_positives as f64 / self.detections as f64
    }

    /// Calculate false positive rate
    pub fn false_positive_rate(&self) -> f64 {
        if self.detections == 0 {
            return 0.0;
        }
        self.false_positives as f64 / self.detections as f64
    }

    /// Update confidence average
    pub fn update_confidence(&mut self, confidence: f64) {
        if self.detections == 0 {
            self.avg_confidence = confidence;
        } else {
            self.avg_confidence = (self.avg_confidence * self.detections as f64 + confidence)
                / (self.detections + 1) as f64;
        }
    }

    /// Check if performing well
    pub fn is_performing_well(&self) -> bool {
        self.accuracy > 0.8 && self.false_positive_rate() < 0.2
    }
}

// ============================================================
// Detection Method Stats
// ============================================================

/// Statistics for a detection method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionMethodStats {
    /// Method name
    pub method_name: String,

    /// Number of detections
    pub detections: u64,

    /// Number of false positives
    pub false_positives: u64,

    /// Accuracy (0.0 - 1.0)
    pub accuracy: f64,

    /// Average confidence
    pub avg_confidence: f64,
}

impl Default for DetectionMethodStats {
    fn default() -> Self {
        Self {
            method_name: String::new(),
            detections: 0,
            false_positives: 0,
            accuracy: 0.0,
            avg_confidence: 0.0,
        }
    }
}

impl DetectionMethodStats {
    /// Create new method stats
    pub fn new(method_name: &str) -> Self {
        Self {
            method_name: method_name.to_string(),
            ..Default::default()
        }
    }
}

// ============================================================
// Trend Types
// ============================================================

/// Trend direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendDirection {
    /// Increasing trend
    Increasing,

    /// Decreasing trend
    Decreasing,

    /// Stable trend
    Stable,

    /// Volatile trend
    Volatile,
}

impl Default for TrendDirection {
    fn default() -> Self {
        Self::Stable
    }
}

/// Threat trend over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTrend {
    /// Number of threats in period
    pub threat_count: u64,

    /// Distribution by severity
    pub severity_distribution: HashMap<ThreatSeverity, u64>,

    /// Time period description
    pub time_period: String,

    /// Trend direction
    pub trend_direction: TrendDirection,

    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for ThreatTrend {
    fn default() -> Self {
        Self {
            threat_count: 0,
            severity_distribution: HashMap::with_capacity(16),
            time_period: "24h".to_string(),
            trend_direction: TrendDirection::Stable,
            last_updated: Utc::now(),
        }
    }
}

impl ThreatTrend {
    /// Create new trend
    pub fn new(threat_count: u64) -> Self {
        Self {
            threat_count,
            severity_distribution: HashMap::with_capacity(16),
            time_period: "24h".to_string(),
            trend_direction: TrendDirection::Stable,
            last_updated: Utc::now(),
        }
    }

    /// Add severity count
    pub fn add_severity(&mut self, severity: ThreatSeverity, count: u64) {
        self.severity_distribution.insert(severity, count);
    }

    /// Get severity percentage
    pub fn get_severity_percentage(&self, severity: &ThreatSeverity) -> f64 {
        if self.threat_count == 0 {
            return 0.0;
        }
        let count = self.severity_distribution.get(severity).unwrap_or(&0);
        (*count as f64 / self.threat_count as f64) * 100.0
    }

    /// Check if trend is recent (within specified minutes)
    pub fn is_recent(&self, minutes: i64) -> bool {
        let cutoff = Utc::now() - chrono::Duration::minutes(minutes);
        self.last_updated > cutoff
    }

    /// Update the last_updated timestamp to now
    pub fn touch(&mut self) {
        self.last_updated = Utc::now();
    }
}

// ============================================================
// Overall Statistics
// ============================================================

/// Overall threat statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThreatStatistics {
    /// Total threats detected
    pub total_threats: u64,

    /// High severity threats
    pub high_severity_threats: usize,

    /// Medium severity threats
    pub medium_severity_threats: usize,

    /// Low severity threats
    pub low_severity_threats: usize,

    /// False positives
    pub false_positives: u64,

    /// Detection accuracy
    pub detection_accuracy: f64,

    /// Average response time in ms
    pub avg_response_time_ms: f64,

    /// Threats blocked
    pub threats_blocked: u64,

    /// Threats mitigated
    pub threats_mitigated: u64,

    /// Active threats
    pub active_threats: u64,

    /// Resolved threats
    pub resolved_threats: u64,

    /// Threat trends
    pub threat_trends: Vec<ThreatTrend>,

    /// Detection methods statistics
    pub detection_methods: HashMap<String, DetectionMethodStats>,

    /// Geographic distribution
    pub geographic_distribution: HashMap<String, u64>,

    /// Threat sources
    pub threat_sources: HashMap<String, u64>,

    /// Hourly statistics
    pub hourly_statistics: Vec<ThreatDetectionStats>,

    /// Daily statistics
    pub daily_statistics: Vec<ThreatDetectionStats>,

    /// Weekly statistics
    pub weekly_statistics: Vec<ThreatDetectionStats>,

    /// Monthly statistics
    pub monthly_statistics: Vec<ThreatDetectionStats>,

    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

impl ThreatStatistics {
    /// Create new statistics
    pub fn new() -> Self {
        Self {
            last_updated: Utc::now(),
            ..Default::default()
        }
    }

    /// Get critical threats count
    pub fn get_critical_threats(&self) -> usize {
        self.high_severity_threats
    }

    /// Calculate high severity ratio
    pub fn high_severity_ratio(&self) -> f64 {
        if self.total_threats == 0 {
            return 0.0;
        }
        self.high_severity_threats as f64 / self.total_threats as f64
    }

    /// Check if threat level is concerning
    pub fn is_concerning_threat_level(&self) -> bool {
        self.high_severity_ratio() > 0.3
    }

    /// Calculate mitigation rate
    pub fn mitigation_rate(&self) -> f64 {
        if self.total_threats == 0 {
            return 0.0;
        }
        (self.threats_blocked + self.threats_mitigated) as f64 / self.total_threats as f64
    }

    /// Record a threat detection
    pub fn record_threat(&mut self, severity: ThreatSeverity) {
        self.total_threats += 1;
        match severity {
            ThreatSeverity::High | ThreatSeverity::Critical => {
                self.high_severity_threats += 1;
            }
            ThreatSeverity::Medium => {
                self.medium_severity_threats += 1;
            }
            ThreatSeverity::Low | ThreatSeverity::None => {
                self.low_severity_threats += 1;
            }
        }
        self.last_updated = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection_stats_default() {
        let stats = ThreatDetectionStats::default();
        assert_eq!(stats.detections, 0);
        assert_eq!(stats.false_positives, 0);
    }

    #[test]
    fn test_false_positive_rate() {
        let stats = ThreatDetectionStats {
            detections: 100,
            false_positives: 10,
            ..Default::default()
        };
        assert_eq!(stats.false_positive_rate(), 0.1);
    }

    #[test]
    fn test_trend_direction_default() {
        assert_eq!(TrendDirection::default(), TrendDirection::Stable);
    }

    #[test]
    fn test_threat_trend() {
        let mut trend = ThreatTrend::new(100);
        trend.add_severity(ThreatSeverity::High, 20);
        assert_eq!(trend.get_severity_percentage(&ThreatSeverity::High), 20.0);
    }

    #[test]
    fn test_threat_statistics() {
        let mut stats = ThreatStatistics::new();
        stats.record_threat(ThreatSeverity::High);
        assert_eq!(stats.total_threats, 1);
        assert_eq!(stats.high_severity_threats, 1);
    }

    #[test]
    fn test_high_severity_ratio() {
        let mut stats = ThreatStatistics::new();
        stats.record_threat(ThreatSeverity::High);
        stats.record_threat(ThreatSeverity::Low);
        assert_eq!(stats.high_severity_ratio(), 0.5);
    }
}
