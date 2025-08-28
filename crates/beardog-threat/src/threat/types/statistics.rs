use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::core::ThreatSeverity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionStats {
    pub detections: u64,
    pub false_positives: u64,
    pub accuracy: f64,
    pub avg_confidence: f64,
    pub last_updated: DateTime<Utc>,
    // Additional fields needed by handlers
    pub blocked_sources: u64,
    pub quarantined_systems: u64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionMethodStats {
    pub method_name: String,
    pub detections: u64,
    pub false_positives: u64,
    pub accuracy: f64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTrend {
    pub threat_count: u64,
    pub severity_distribution: HashMap<ThreatSeverity, u64>,
    pub time_period: String,
    pub trend_direction: TrendDirection,
}

impl Default for ThreatTrend {
    fn default() -> Self {
        Self {
            threat_count: 0,
            severity_distribution: HashMap::new(),
            time_period: "24h".to_string(),
            trend_direction: TrendDirection::Stable,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
}

impl Default for TrendDirection {
    fn default() -> Self {
        Self::Stable
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreatStatistics {
    pub total_threats: u64,
    pub high_severity_threats: usize,
    pub medium_severity_threats: usize,
    pub low_severity_threats: usize,
    pub false_positives: u64,
    pub detection_accuracy: f64,
    pub avg_response_time_ms: f64,
    pub threats_blocked: u64,
    pub threats_mitigated: u64,
    pub active_threats: u64,
    pub resolved_threats: u64,
    pub threat_trends: Vec<ThreatTrend>,
    pub detection_methods: HashMap<String, DetectionMethodStats>,
    pub geographic_distribution: HashMap<String, u64>,
    pub threat_sources: HashMap<String, u64>,
    pub hourly_statistics: Vec<ThreatDetectionStats>,
    pub daily_statistics: Vec<ThreatDetectionStats>,
    pub weekly_statistics: Vec<ThreatDetectionStats>,
    pub monthly_statistics: Vec<ThreatDetectionStats>,
    pub last_updated: DateTime<Utc>,
}

impl ThreatDetectionStats {
    pub fn new() -> Self {
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

    pub fn accuracy(&self) -> f64 {
        if self.detections == 0 {
            return 0.0;
        }
        let true_positives = self.detections - self.false_positives;
        true_positives as f64 / self.detections as f64
    }

    pub fn false_positive_rate(&self) -> f64 {
        if self.detections == 0 {
            return 0.0;
        }
        self.false_positives as f64 / self.detections as f64
    }

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
    pub fn new(threat_count: u64) -> Self {
        Self {
            threat_count,
            severity_distribution: HashMap::new(),
            time_period: "24h".to_string(),
            trend_direction: TrendDirection::Stable,
        }
    }

    pub fn add_severity_data(&mut self, severity: ThreatSeverity, count: u64) {
        self.severity_distribution.insert(severity, count);
    }

    pub fn get_severity_percentage(&self, severity: &ThreatSeverity) -> f64 {
        if self.threat_count == 0 {
            return 0.0;
        }
        let count = self.severity_distribution.get(severity).unwrap_or(&0);
        (*count as f64 / self.threat_count as f64) * 100.0
    }

    pub fn is_recent(&self, _minutes: i64) -> bool {
        // Since we don't have timestamp, we'll use a simple heuristic
        // In a real implementation, you'd want to add a timestamp field
        true // Placeholder - always consider recent for now
    }
}

impl ThreatStatistics {
    pub fn get_critical_threats(&self) -> usize {
        self.high_severity_threats
    }

    pub fn high_severity_ratio(&self) -> f64 {
        if self.total_threats == 0 {
            return 0.0;
        }
        self.high_severity_threats as f64 / self.total_threats as f64
    }

    pub fn is_concerning_threat_level(&self) -> bool {
        self.high_severity_ratio() > 0.3
    }
}
