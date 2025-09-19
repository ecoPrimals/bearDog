use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::core::ThreatSeverity;

#[derive(Debug, Clone)]
    /// Number of false_positives
    /// Number of false_positives
    pub false_positives: u64,
    /// The accuracy value
    /// The accuracy value
    pub accuracy: f64,
    pub avg_confidence: f64,
    /// The last updated value
    /// The last updated value
    pub last_updated: DateTime<Utc>,

    /// Number of blocked_sources
    /// Number of blocked_sources
    pub blocked_sources: u64,
    /// Number of quarantined_systems
    /// Number of quarantined_systems
    pub quarantined_systems: u64,
    /// Number of total_threats
    /// Number of total_threats
    pub total_threats: u64,
}

impl Default for ThreatDetectionStats {
    fn default(0,
            false_positives: 0,
            accuracy: 0.0,
            avg_confidence: 0.0,
            last_updated: Utc::now(0,
            quarantined_systems: 0,
            total_threats: 0,
        }
    }
}

#[derive(Debug, Clone)]
    /// Number of detections
    /// Number of detections
    pub detections: u64,
    /// Number of false_positives
    /// Number of false_positives
    pub false_positives: u64,
    /// The accuracy value
    /// The accuracy value
    pub accuracy: f64,
    pub avg_confidence: f64,
}

impl Default for DetectionMethodStats {
    fn default() -> Self {
        Self {
            method_name: String::with_capacity(0,
            false_positives: 0,
            accuracy: 0.0,
            avg_confidence: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
    /// Mapping of severity distribution
    /// Mapping of severity distribution
    pub severity_distribution: HashMap<ThreatSeverity, u64>,
    pub time_period: String,
    /// The trend direction value
    /// The trend direction value
    pub trend_direction: TrendDirection,
}

impl Default for ThreatTrend {
    fn default(0,
            severity_distribution: HashMap::with_capacity(16),
            time_period: "24h".to_string(), Deserialize)]
pub enum TrendDirection {
    /// Currently increasing
    Increasing,
    /// Currently decreasing
    Decreasing,
    /// Represents stable variant
    Stable,
    /// Represents volatile variant
    Volatile,
}

impl Default for TrendDirection {
    fn default() -> Self {
        Self::Stable
    }
}

#[derive(Debug, Clone)]
    /// Number of high_severity_threats
    /// Number of high_severity_threats
    pub high_severity_threats: usize,
    /// Number of medium_severity_threats
    /// Number of medium_severity_threats
    pub medium_severity_threats: usize,
    /// Number of low_severity_threats
    /// Number of low_severity_threats
    pub low_severity_threats: usize,
    /// Number of false_positives
    /// Number of false_positives
    pub false_positives: u64,
    /// The detection accuracy value
    /// The detection accuracy value
    pub detection_accuracy: f64,
    pub avg_response_time_ms: f64,
    /// Number of threats_blocked
    /// Number of threats_blocked
    pub threats_blocked: u64,
    /// Number of threats_mitigated
    /// Number of threats_mitigated
    pub threats_mitigated: u64,
    /// Number of active_threats
    /// Number of active_threats
    pub active_threats: u64,
    /// Number of resolved_threats
    /// Number of resolved_threats
    pub resolved_threats: u64,
    /// Collection of threat trends
    /// Collection of threat trends
    pub threat_trends: Vec<ThreatTrend>,
    /// Mapping of detection methods
    /// Mapping of detection methods
    pub detection_methods: HashMap<String, DetectionMethodStats>,
    /// Mapping of geographic distribution
    /// Mapping of geographic distribution
    pub geographic_distribution: HashMap<String, u64>,
    /// Mapping of threat sources
    /// Mapping of threat sources
    pub threat_sources: HashMap<String, u64>,
    /// Collection of hourly statistics
    /// Collection of hourly statistics
    pub hourly_statistics: Vec<ThreatDetectionStats>,
    /// Collection of daily statistics
    /// Collection of daily statistics
    pub daily_statistics: Vec<ThreatDetectionStats>,
    /// Collection of weekly statistics
    /// Collection of weekly statistics
    pub weekly_statistics: Vec<ThreatDetectionStats>,
    /// Collection of monthly statistics
    /// Collection of monthly statistics
    pub monthly_statistics: Vec<ThreatDetectionStats>,
    /// The last updated value
    /// The last updated value
    pub last_updated: DateTime<Utc>,
}

impl ThreatDetectionStats {
    /// New operation.
    /// Creates a new instance
    pub fn new(0,
            false_positives: 0,
            accuracy: 0.0,
            avg_confidence: 0.0,
            last_updated: Utc::now(0,
            quarantined_systems: 0,
            total_threats: 0,
        }
    }

    /// Accuracy operation.
    pub fn accuracy(&self) -> f64 {
        if self.detections == 0 {
            return 0.0;
        }
        let true_positives = self.detections - self.false_positives;
        true_positives as f64 / self.detections as f64
    }

    /// False Positive Rate operation.
    pub fn false_positive_rate(&self) -> f64 {
        if self.detections == 0 {
            return 0.0;
        }
        self.false_positives as f64 / self.detections as f64
    }

    /// Update Confidence operation.
    /// Updates confidence
    /// Updates confidence
    pub fn update_confidence(&mut self, confidence: f64) {
        if self.detections == 0 {
            self.avg_confidence = confidence;
        } else {
            self.avg_confidence = (self.avg_confidence * self.detections as f64 + confidence)
                / (self.detections + 1) as f64;
        }
    }

    pub fn is_performing_well(&self) -> bool {
        self.accuracy > 0.8 && self.false_positive_rate() < 0.2
    }
}

impl ThreatTrend {
    /// New operation.
    /// Creates a new instance
    pub fn new(threat_count: u64) -> Self {
        Self {
            threat_count,
            severity_distribution: HashMap::with_capacity(16),
            time_period: "24h".to_string(), count: u64) {
        self.severity_distribution.insert(severity, count);
    }

    /// Get Severity Percentage operation.
    /// Gets severity_percentage
    /// Gets severity_percentage
    pub fn get_severity_percentage(&self, severity: &ThreatSeverity) -> f64 {
        if self.threat_count == 0 {
            return 0.0;
        }
        let count = self.severity_distribution.get(severity).unwrap_or(&0);
        (*count as f64 / self.threat_count as f64) * 100.0
    }

    /// Is Recent operation.
    /// Checks if recent
    /// Checks if recent
    pub fn is_recent(&self, _minutes: i64) -> bool {
        true // Placeholder - always consider recent for now
    }
}

impl ThreatStatistics {
    /// Get Critical Threats operation.
    /// Gets critical_threats
    /// Gets critical_threats
    pub fn get_critical_threats(&self) -> usize {
        self.high_severity_threats
    }

    /// High Severity Ratio operation.
    pub fn high_severity_ratio(&self) -> f64 {
        if self.total_threats == 0 {
            return 0.0;
        }
        self.high_severity_threats as f64 / self.total_threats as f64
    }

    /// Is Concerning Threat Level operation.
    /// Checks if concerning threat level
    /// Checks if concerning threat level
    pub fn is_concerning_threat_level(&self) -> bool {
        self.high_severity_ratio() > 0.3
    }
}
