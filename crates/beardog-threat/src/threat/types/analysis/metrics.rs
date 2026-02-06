//! # Analysis Metrics
//!
//! This module provides types for tracking threat analysis performance metrics.

use serde::{Deserialize, Serialize};

/// Analysis metrics for threat detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetrics {
    /// Total events analyzed
    pub total_events_analyzed: u64,

    /// Average analysis time in milliseconds
    pub avg_analysis_time_ms: f64,

    /// Maximum analysis time in milliseconds
    pub max_analysis_time_ms: u64,

    /// Minimum analysis time in milliseconds
    pub min_analysis_time_ms: u64,

    /// Number of events with threats detected
    pub events_with_threats: u64,

    /// Total threats detected
    pub total_threats_detected: u64,

    /// Analysis accuracy (0.0 - 1.0)
    pub analysis_accuracy: Option<f64>,

    /// False positive rate (0.0 - 1.0)
    pub false_positive_rate: f64,
}

impl Default for AnalysisMetrics {
    fn default() -> Self {
        Self {
            total_events_analyzed: 0,
            avg_analysis_time_ms: 0.0,
            max_analysis_time_ms: 0,
            min_analysis_time_ms: 0,
            events_with_threats: 0,
            total_threats_detected: 0,
            analysis_accuracy: None,
            false_positive_rate: 0.0,
        }
    }
}

impl AnalysisMetrics {
    /// Create new metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an analysis result
    pub fn record_analysis(&mut self, analysis_time_ms: u64, threats_detected: usize) {
        self.total_events_analyzed += 1;

        if self.total_events_analyzed == 1 {
            self.min_analysis_time_ms = analysis_time_ms;
            self.max_analysis_time_ms = analysis_time_ms;
            self.avg_analysis_time_ms = analysis_time_ms as f64;
        } else {
            self.min_analysis_time_ms = self.min_analysis_time_ms.min(analysis_time_ms);
            self.max_analysis_time_ms = self.max_analysis_time_ms.max(analysis_time_ms);

            let total_time = self.avg_analysis_time_ms * (self.total_events_analyzed - 1) as f64;
            self.avg_analysis_time_ms =
                (total_time + analysis_time_ms as f64) / self.total_events_analyzed as f64;
        }

        if threats_detected > 0 {
            self.events_with_threats += 1;
            self.total_threats_detected += threats_detected as u64;
        }
    }

    /// Calculate threat detection rate
    pub fn threat_detection_rate(&self) -> f64 {
        if self.total_events_analyzed == 0 {
            0.0
        } else {
            self.events_with_threats as f64 / self.total_events_analyzed as f64
        }
    }

    /// Get threat detection rate
    pub fn get_threat_detection_rate(&self) -> f64 {
        self.threat_detection_rate()
    }

    /// Calculate average threats per event
    pub fn avg_threats_per_event(&self) -> f64 {
        if self.total_events_analyzed == 0 {
            0.0
        } else {
            self.total_threats_detected as f64 / self.total_events_analyzed as f64
        }
    }

    /// Get average threats per event
    pub fn get_avg_threats_per_event(&self) -> f64 {
        self.avg_threats_per_event()
    }

    /// Calculate throughput in events per second
    pub fn throughput_events_per_second(&self) -> f64 {
        if self.avg_analysis_time_ms == 0.0 {
            0.0
        } else {
            1000.0 / self.avg_analysis_time_ms
        }
    }

    /// Set accuracy
    pub fn set_accuracy(&mut self, accuracy: f64) {
        self.analysis_accuracy = Some(accuracy.clamp(0.0, 1.0));
    }

    /// Set false positive rate
    pub fn set_false_positive_rate(&mut self, rate: f64) {
        self.false_positive_rate = rate.clamp(0.0, 1.0);
    }

    /// Generate summary string
    pub fn summary(&self) -> String {
        format!(
            "Analyzed {} events, detected {} threats ({:.2}% rate), avg time: {:.2}ms",
            self.total_events_analyzed,
            self.total_threats_detected,
            self.threat_detection_rate() * 100.0,
            self.avg_analysis_time_ms
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let metrics = AnalysisMetrics::default();
        assert_eq!(metrics.total_events_analyzed, 0);
        assert_eq!(metrics.avg_analysis_time_ms, 0.0);
    }

    #[test]
    fn test_record_analysis() {
        let mut metrics = AnalysisMetrics::new();
        metrics.record_analysis(100, 2);
        assert_eq!(metrics.total_events_analyzed, 1);
        assert_eq!(metrics.events_with_threats, 1);
        assert_eq!(metrics.total_threats_detected, 2);
    }

    #[test]
    fn test_threat_detection_rate() {
        let mut metrics = AnalysisMetrics::new();
        metrics.record_analysis(100, 1);
        metrics.record_analysis(100, 0);
        assert_eq!(metrics.threat_detection_rate(), 0.5);
    }

    #[test]
    fn test_throughput() {
        let mut metrics = AnalysisMetrics::new();
        metrics.record_analysis(100, 0);
        assert_eq!(metrics.throughput_events_per_second(), 10.0);
    }

    #[test]
    fn test_accuracy() {
        let mut metrics = AnalysisMetrics::new();
        metrics.set_accuracy(0.95);
        assert_eq!(metrics.analysis_accuracy, Some(0.95));
    }
}
