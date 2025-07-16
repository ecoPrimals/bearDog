//! Event Correlation Types
//!
//! This module contains types for event correlation analysis and results.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Event correlation result structure
///
/// Contains the results of correlating security events
/// to identify related activities or attack patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCorrelationResult {
    /// Primary event ID
    pub primary_event_id: String,

    /// Related event IDs
    pub related_event_ids: Vec<String>,

    /// Correlation confidence score (0.0 to 1.0)
    pub correlation_confidence: f64,

    /// Type of correlation
    pub correlation_type: CorrelationType,

    /// Time window for correlation in minutes
    pub time_window_minutes: u64,

    /// Correlation timestamp
    pub correlation_timestamp: DateTime<Utc>,
}

/// Correlation type enumeration
///
/// Defines the different types of correlations
/// that can be identified between events.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CorrelationType {
    /// Temporal correlation (events close in time)
    Temporal,
    /// Spatial correlation (events from same location/IP)
    Spatial,
    /// Behavioral correlation (similar user behavior)
    Behavioral,
    /// Causal correlation (one event causes another)
    Causal,
    /// Attack pattern correlation
    AttackPattern,
    /// User behavior correlation
    UserBehavior,
    /// Network correlation
    Network,
    /// Custom correlation type
    Custom(String),
}

impl Default for EventCorrelationResult {
    fn default() -> Self {
        Self {
            primary_event_id: String::new(),
            related_event_ids: vec![],
            correlation_confidence: 0.0,
            correlation_type: CorrelationType::Temporal,
            time_window_minutes: 60,
            correlation_timestamp: Utc::now(),
        }
    }
}

impl EventCorrelationResult {
    /// Create new correlation result
    pub fn new(primary_event_id: String, correlation_type: CorrelationType) -> Self {
        Self {
            primary_event_id,
            related_event_ids: Vec::new(),
            correlation_confidence: 0.0,
            correlation_type,
            time_window_minutes: 60,
            correlation_timestamp: Utc::now(),
        }
    }

    /// Add related event
    pub fn add_related_event(&mut self, event_id: String) {
        if !self.related_event_ids.contains(&event_id) {
            self.related_event_ids.push(event_id);
        }
    }

    /// Set correlation confidence
    pub fn set_confidence(&mut self, confidence: f64) {
        self.correlation_confidence = confidence.clamp(0.0, 1.0);
    }

    /// Set time window
    pub fn set_time_window(&mut self, minutes: u64) {
        self.time_window_minutes = minutes;
    }

    /// Get correlation strength
    pub fn correlation_strength(&self) -> &str {
        match self.correlation_confidence {
            x if x >= 0.8 => "Strong",
            x if x >= 0.6 => "Moderate",
            x if x >= 0.4 => "Weak",
            _ => "Very Weak",
        }
    }

    /// Get number of related events
    pub fn related_event_count(&self) -> usize {
        self.related_event_ids.len()
    }

    /// Check if correlation is significant
    pub fn is_significant(&self) -> bool {
        self.correlation_confidence >= 0.5 && !self.related_event_ids.is_empty()
    }

    /// Get correlation summary
    pub fn summary(&self) -> String {
        format!(
            "{} correlation: {} related events with {:.2}% confidence",
            self.correlation_type,
            self.related_event_count(),
            self.correlation_confidence * 100.0
        )
    }
}

impl std::fmt::Display for CorrelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CorrelationType::Temporal => write!(f, "Temporal"),
            CorrelationType::Spatial => write!(f, "Spatial"),
            CorrelationType::Behavioral => write!(f, "Behavioral"),
            CorrelationType::Causal => write!(f, "Causal"),
            CorrelationType::AttackPattern => write!(f, "Attack Pattern"),
            CorrelationType::UserBehavior => write!(f, "User Behavior"),
            CorrelationType::Network => write!(f, "Network"),
            CorrelationType::Custom(custom) => write!(f, "Custom: {custom}"),
        }
    }
} 