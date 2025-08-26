

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCorrelationResult {
    pub event_id: String,
    pub correlation_type: String,
    pub confidence: f64,
    pub related_events: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub confidence_score: f64, // Add missing field
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CorrelationType {

    Temporal,

    Spatial,

    Behavioral,

    Causal,

    AttackPattern,

    UserBehavior,

    Network,

    Custom(String),
}

impl Default for EventCorrelationResult {
    fn default() -> Self {
        Self {
            primary_event_id: String::with_capacity(64),
            correlation_type: CorrelationType::Temporal,
            related_event_ids: Vec::new(),
            confidence_score: 0.0,
            time_window_minutes: 60,
        }
    }
}

impl EventCorrelationResult {
    pub fn new(event_id: String, correlation_type: String) -> EventCorrelationResult {
        EventCorrelationResult {
            event_id,
            related_events: Vec::new(),
            correlation_type,
            confidence: 0.0,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
            confidence_score: 0.0,
        }
    }

    pub fn with_capacity(capacity: usize) -> EventCorrelationResult {
        EventCorrelationResult {
            event_id: String::with_capacity(64),
            correlation_type: CorrelationType::Temporal.to_string(),
            related_events: Vec::with_capacity(capacity),
            confidence: 0.0,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
            confidence_score: 0.0,
        }
    }

    pub fn add_related_event(&mut self, event_id: &str) {
        if !self.related_events.contains(&event_id.to_string()) {
            self.related_events.push(event_id.to_string());
        }
    }

    pub fn set_confidence(&mut self, confidence: f64) {
        self.confidence_score = confidence.clamp(0.0, 1.0);
        self.confidence = confidence.clamp(0.0, 1.0);
    }

    pub fn set_time_window(&mut self, minutes: u64) {
        // Store time window in metadata since it's not a direct field
        self.metadata.insert("time_window_minutes".to_string(), minutes.to_string());
    }

    pub fn get_correlation_strength(&self) -> f64 {
        // Use confidence_score for correlation strength
        self.confidence_score
    }

    pub fn get_related_event_count(&self) -> usize {
        self.related_events.len()
    }

    pub fn is_significant(&self) -> bool {
        self.confidence_score >= 0.5 && !self.related_events.is_empty()
    }

    pub fn summary(&self) -> String {
        format!(
            "{} correlation: {} related events with {:.2}% confidence",
            self.correlation_type,
            self.get_related_event_count(),
            self.confidence_score * 100.0
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
