//! Threat detection statistics and metrics
//!
//! This module contains types for tracking threat detection performance,
//! statistics, and trend analysis.
//!
//! ## Features
//! - Comprehensive threat detection statistics
//! - Detection method performance metrics
//! - Threat trend analysis
//! - Performance tracking
//! - False positive rate monitoring
//!
//! ## Example
//! ```rust
//! use beardog::threat::types::{ThreatDetectionStats, DetectionMethodStats, ThreatTrend};
//! use std::collections::HashMap;
//!
//! let stats = ThreatDetectionStats {
//!     total_threats: 1000,
//!     false_positive_rate: 0.05,
//!     avg_response_time_seconds: 30.0,
//!     blocked_sources: 150,
//!     ..Default::default()
//! };
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::core::ThreatSeverity;

/// Engine-wide threat detection statistics
///
/// This structure tracks comprehensive statistics about threat detection
/// performance, accuracy, and system health over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionStats {
    /// Total threats detected
    pub total_threats: u64,

    /// Threats by severity level
    pub threats_by_severity: HashMap<String, u64>,

    /// Threats by type
    pub threats_by_type: HashMap<String, u64>,

    /// Detection methods performance
    pub detection_methods: HashMap<String, DetectionMethodStats>,

    /// False positive rate (0.0 to 1.0)
    pub false_positive_rate: f64,

    /// Total false positives
    pub false_positives: u64,

    /// Average response time in seconds
    pub avg_response_time_seconds: f64,

    /// Number of blocked sources
    pub blocked_sources: u64,

    /// Number of quarantined systems
    pub quarantined_systems: u64,

    /// Threat trends over time
    pub threat_trends: Vec<ThreatTrend>,

    /// Number of rules triggered (missing field added)
    pub rules_triggered: u64,

    /// Number of events analyzed (missing field added)
    pub events_analyzed: u64,

    /// Number of threats detected (alias for total_threats)
    pub threats_detected: u64,

    /// Total events processed by the engine
    pub total_events_processed: u64,
}

/// Detection method performance statistics
///
/// Tracks the performance of individual detection methods
/// including accuracy and false positive rates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionMethodStats {
    /// Number of detections by this method
    pub detections: u64,

    /// Detection accuracy percentage (0.0 to 1.0)
    pub accuracy: f64,

    /// Number of false positives generated
    pub false_positives: u64,

    /// Average confidence score of detections
    pub avg_confidence: f64,
}

/// Threat trend data point
///
/// Represents threat activity at a specific point in time
/// for trend analysis and forecasting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTrend {
    /// Timestamp of the trend data point
    pub timestamp: DateTime<Utc>,

    /// Total number of threats at this timestamp
    pub threat_count: u64,

    /// Distribution of threats by severity level
    pub severity_distribution: HashMap<ThreatSeverity, u64>,
}

/// Threat statistics summary
///
/// Simplified statistics structure for quick overview
/// and dashboard displays.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreatStatistics {
    /// Total threats detected
    pub total_threats: usize,

    /// High severity threats
    pub high_severity_threats: usize,

    /// Medium severity threats
    pub medium_severity_threats: usize,

    /// Low severity threats
    pub low_severity_threats: usize,

    /// Active incidents
    pub active_incidents: usize,

    /// Number of detection rules
    pub detection_rules_count: usize,

    /// Number of threat feeds
    pub threat_feeds_count: usize,

    /// Number of ML models
    pub ml_models_count: usize,
}

// Default implementations

impl Default for ThreatDetectionStats {
    fn default() -> Self {
        Self {
            total_threats: 0,
            threats_by_severity: HashMap::new(),
            threats_by_type: HashMap::new(),
            detection_methods: HashMap::new(),
            false_positive_rate: 0.0,
            false_positives: 0,
            avg_response_time_seconds: 0.0,
            blocked_sources: 0,
            quarantined_systems: 0,
            threat_trends: Vec::new(),
            rules_triggered: 0,
            events_analyzed: 0,
            threats_detected: 0,
            total_events_processed: 0,
        }
    }
}

impl Default for DetectionMethodStats {
    fn default() -> Self {
        Self {
            detections: 0,
            accuracy: 0.0,
            false_positives: 0,
            avg_confidence: 0.0,
        }
    }
}

impl Default for ThreatTrend {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            threat_count: 0,
            severity_distribution: HashMap::new(),
        }
    }
}

// Utility implementations

impl ThreatDetectionStats {
    /// Create new threat detection statistics
    ///
    /// # Returns
    /// A new `ThreatDetectionStats` instance with default values
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionStats;
    ///
    /// let stats = ThreatDetectionStats::new();
    /// assert_eq!(stats.total_threats, 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Add threat detection
    ///
    /// # Arguments
    /// * `threat_type` - Type of threat detected
    /// * `severity` - Severity level of the threat
    /// * `detection_method` - Method used for detection
    /// * `response_time_seconds` - Time taken to respond
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatDetectionStats, ThreatSeverity};
    ///
    /// let mut stats = ThreatDetectionStats::new();
    /// stats.add_threat_detection(
    ///     "Malware".to_string(),
    ///     ThreatSeverity::High,
    ///     "Signature".to_string(),
    ///     15.0
    /// );
    ///
    /// assert_eq!(stats.total_threats, 1);
    /// ```
    pub fn add_threat_detection(
        &mut self,
        threat_type: String,
        severity: ThreatSeverity,
        detection_method: String,
        response_time_seconds: f64,
    ) {
        self.total_threats += 1;

        // Update severity counts
        let severity_key = severity.to_string();
        *self.threats_by_severity.entry(severity_key).or_insert(0) += 1;

        // Update type counts
        *self.threats_by_type.entry(threat_type).or_insert(0) += 1;

        // Update detection method stats
        let method_stats = self.detection_methods.entry(detection_method).or_default();
        method_stats.detections += 1;

        // Update response time
        self.avg_response_time_seconds = (self.avg_response_time_seconds
            * (self.total_threats - 1) as f64
            + response_time_seconds)
            / self.total_threats as f64;
    }

    /// Add false positive
    ///
    /// # Arguments
    /// * `detection_method` - Method that generated the false positive
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionStats;
    ///
    /// let mut stats = ThreatDetectionStats::new();
    /// stats.add_false_positive("Anomaly".to_string());
    ///
    /// assert_eq!(stats.false_positives, 1);
    /// ```
    pub fn add_false_positive(&mut self, detection_method: String) {
        self.false_positives += 1;

        // Update detection method stats
        if let Some(method_stats) = self.detection_methods.get_mut(&detection_method) {
            method_stats.false_positives += 1;
        }

        // Update false positive rate
        if self.total_threats > 0 {
            self.false_positive_rate = self.false_positives as f64 / self.total_threats as f64;
        }
    }

    /// Add blocked source
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionStats;
    ///
    /// let mut stats = ThreatDetectionStats::new();
    /// stats.add_blocked_source();
    ///
    /// assert_eq!(stats.blocked_sources, 1);
    /// ```
    pub fn add_blocked_source(&mut self) {
        self.blocked_sources += 1;
    }

    /// Add quarantined system
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionStats;
    ///
    /// let mut stats = ThreatDetectionStats::new();
    /// stats.add_quarantined_system();
    ///
    /// assert_eq!(stats.quarantined_systems, 1);
    /// ```
    pub fn add_quarantined_system(&mut self) {
        self.quarantined_systems += 1;
    }

    /// Add threat trend data point
    ///
    /// # Arguments
    /// * `trend` - Threat trend data point
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatDetectionStats, ThreatTrend};
    ///
    /// let mut stats = ThreatDetectionStats::new();
    /// let trend = ThreatTrend::default();
    /// stats.add_threat_trend(trend);
    ///
    /// assert_eq!(stats.threat_trends.len(), 1);
    /// ```
    pub fn add_threat_trend(&mut self, trend: ThreatTrend) {
        self.threat_trends.push(trend);

        // Keep only last 100 trend points
        if self.threat_trends.len() > 100 {
            self.threat_trends.remove(0);
        }
    }

    /// Get overall detection accuracy
    ///
    /// # Returns
    /// Overall accuracy (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionStats;
    ///
    /// let stats = ThreatDetectionStats::new();
    /// let accuracy = stats.get_overall_accuracy();
    /// assert!(accuracy >= 0.0 && accuracy <= 1.0);
    /// ```
    pub fn get_overall_accuracy(&self) -> f64 {
        if self.total_threats == 0 {
            return 0.0;
        }

        let true_positives = self.total_threats - self.false_positives;
        true_positives as f64 / self.total_threats as f64
    }

    /// Get threat count by severity
    ///
    /// # Arguments
    /// * `severity` - Severity level to query
    ///
    /// # Returns
    /// Number of threats with specified severity
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatDetectionStats, ThreatSeverity};
    ///
    /// let stats = ThreatDetectionStats::new();
    /// let count = stats.get_threat_count_by_severity(&ThreatSeverity::High);
    /// assert_eq!(count, 0);
    /// ```
    pub fn get_threat_count_by_severity(&self, severity: &ThreatSeverity) -> u64 {
        self.threats_by_severity
            .get(&severity.to_string())
            .cloned()
            .unwrap_or(0)
    }

    /// Get most common threat type
    ///
    /// # Returns
    /// Most frequently detected threat type
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionStats;
    ///
    /// let stats = ThreatDetectionStats::new();
    /// let most_common = stats.get_most_common_threat_type();
    /// // Returns None if no threats detected
    /// ```
    pub fn get_most_common_threat_type(&self) -> Option<String> {
        self.threats_by_type
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(threat_type, _)| threat_type.clone())
    }

    /// Get best performing detection method
    ///
    /// # Returns
    /// Detection method with highest accuracy
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionStats;
    ///
    /// let stats = ThreatDetectionStats::new();
    /// let best_method = stats.get_best_detection_method();
    /// // Returns None if no methods have stats
    /// ```
    pub fn get_best_detection_method(&self) -> Option<String> {
        self.detection_methods
            .iter()
            .max_by(|(_, a), (_, b)| {
                a.accuracy
                    .partial_cmp(&b.accuracy)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(method, _)| method.clone())
    }
}

impl DetectionMethodStats {
    /// Create new detection method statistics
    ///
    /// # Returns
    /// A new `DetectionMethodStats` instance with default values
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionMethodStats;
    ///
    /// let stats = DetectionMethodStats::new();
    /// assert_eq!(stats.detections, 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate false positive rate
    ///
    /// # Returns
    /// False positive rate (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionMethodStats;
    ///
    /// let mut stats = DetectionMethodStats::new();
    /// stats.detections = 100;
    /// stats.false_positives = 5;
    ///
    /// assert_eq!(stats.false_positive_rate(), 0.05);
    /// ```
    pub fn false_positive_rate(&self) -> f64 {
        if self.detections == 0 {
            return 0.0;
        }

        self.false_positives as f64 / self.detections as f64
    }

    /// Update confidence score
    ///
    /// # Arguments
    /// * `confidence` - New confidence score to include
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionMethodStats;
    ///
    /// let mut stats = DetectionMethodStats::new();
    /// stats.update_confidence(0.8);
    /// stats.update_confidence(0.9);
    ///
    /// assert_eq!(stats.avg_confidence, 0.85);
    /// ```
    pub fn update_confidence(&mut self, confidence: f64) {
        if self.detections == 0 {
            self.avg_confidence = confidence;
        } else {
            self.avg_confidence = (self.avg_confidence * (self.detections - 1) as f64 + confidence)
                / self.detections as f64;
        }
    }

    /// Check if method performance is acceptable
    ///
    /// # Returns
    /// `true` if accuracy is above 80% and false positive rate is below 20%
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionMethodStats;
    ///
    /// let mut stats = DetectionMethodStats::new();
    /// stats.accuracy = 0.85;
    /// stats.detections = 100;
    /// stats.false_positives = 10;
    ///
    /// assert!(stats.is_performing_well());
    /// ```
    pub fn is_performing_well(&self) -> bool {
        self.accuracy > 0.8 && self.false_positive_rate() < 0.2
    }
}

impl ThreatTrend {
    /// Create new threat trend data point
    ///
    /// # Arguments
    /// * `threat_count` - Number of threats at this time
    ///
    /// # Returns
    /// A new `ThreatTrend` instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatTrend;
    ///
    /// let trend = ThreatTrend::new(50);
    /// assert_eq!(trend.threat_count, 50);
    /// ```
    pub fn new(threat_count: u64) -> Self {
        Self {
            timestamp: Utc::now(),
            threat_count,
            severity_distribution: HashMap::new(),
        }
    }

    /// Add severity data
    ///
    /// # Arguments
    /// * `severity` - Threat severity level
    /// * `count` - Number of threats with this severity
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatTrend, ThreatSeverity};
    ///
    /// let mut trend = ThreatTrend::new(100);
    /// trend.add_severity_data(ThreatSeverity::High, 20);
    /// trend.add_severity_data(ThreatSeverity::Medium, 50);
    ///
    /// assert_eq!(trend.severity_distribution.len(), 2);
    /// ```
    pub fn add_severity_data(&mut self, severity: ThreatSeverity, count: u64) {
        self.severity_distribution.insert(severity, count);
    }

    /// Get severity percentage
    ///
    /// # Arguments
    /// * `severity` - Severity level to query
    ///
    /// # Returns
    /// Percentage of threats with specified severity
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatTrend, ThreatSeverity};
    ///
    /// let mut trend = ThreatTrend::new(100);
    /// trend.add_severity_data(ThreatSeverity::High, 20);
    ///
    /// assert_eq!(trend.get_severity_percentage(&ThreatSeverity::High), 20.0);
    /// ```
    pub fn get_severity_percentage(&self, severity: &ThreatSeverity) -> f64 {
        if self.threat_count == 0 {
            return 0.0;
        }

        let count = self
            .severity_distribution
            .get(severity)
            .cloned()
            .unwrap_or(0);
        (count as f64 / self.threat_count as f64) * 100.0
    }

    /// Get age in minutes
    ///
    /// # Returns
    /// Age of trend data in minutes
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatTrend;
    ///
    /// let trend = ThreatTrend::new(50);
    /// let age = trend.age_minutes();
    /// assert!(age >= 0);
    /// ```
    pub fn age_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.timestamp).num_minutes()
    }
}

impl ThreatStatistics {
    /// Create new threat statistics
    ///
    /// # Returns
    /// A new `ThreatStatistics` instance with default values
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatStatistics;
    ///
    /// let stats = ThreatStatistics::new();
    /// assert_eq!(stats.total_threats, 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Get total high and critical threats
    ///
    /// # Returns
    /// Sum of high and critical severity threats
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatStatistics;
    ///
    /// let mut stats = ThreatStatistics::new();
    /// stats.high_severity_threats = 10;
    ///
    /// assert_eq!(stats.get_critical_threats(), 10);
    /// ```
    pub fn get_critical_threats(&self) -> usize {
        self.high_severity_threats
    }

    /// Calculate threat severity ratio
    ///
    /// # Returns
    /// Ratio of high severity to total threats
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatStatistics;
    ///
    /// let mut stats = ThreatStatistics::new();
    /// stats.total_threats = 100;
    /// stats.high_severity_threats = 20;
    ///
    /// assert_eq!(stats.high_severity_ratio(), 0.2);
    /// ```
    pub fn high_severity_ratio(&self) -> f64 {
        if self.total_threats == 0 {
            return 0.0;
        }

        self.high_severity_threats as f64 / self.total_threats as f64
    }

    /// Check if threat levels are concerning
    ///
    /// # Returns
    /// `true` if high severity threats exceed 30% of total
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatStatistics;
    ///
    /// let mut stats = ThreatStatistics::new();
    /// stats.total_threats = 100;
    /// stats.high_severity_threats = 40;
    ///
    /// assert!(stats.is_concerning_threat_level());
    /// ```
    pub fn is_concerning_threat_level(&self) -> bool {
        self.high_severity_ratio() > 0.3
    }
}
