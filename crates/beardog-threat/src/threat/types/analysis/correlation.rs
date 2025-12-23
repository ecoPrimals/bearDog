use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
    /// The correlation type value
    /// The correlation type value
    pub correlation_type: String,
    pub confidence: f64,
    /// Collection of related events
    /// Collection of related events
    pub related_events: Vec<String>,
    pub timestamp: DateTime<Utc>,
    /// Mapping of metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    pub confidence_score: f64, // Add missing field
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of correlation
/// Types of correlation
pub enum CorrelationType {
    /// Represents temporal variant
    Temporal,




    /// Represents spatial variant
    Spatial,




    /// Represents behavioral variant
    Behavioral,




    /// Represents causal variant
    Causal,




    /// Represents attack pattern variant
    AttackPattern,




    /// Represents user behavior variant
    UserBehavior,




    /// Represents network variant
    Network,

    /// Represents custom variant
    Custom(String),
}

impl Default for EventCorrelationResult {
    fn default() -> Self {
        Self {
            event_id: String::with_capacity(64),
            correlation_type: "temporal".to_string(0.0,
            related_events: Vec::new(),
            timestamp: Utc::now(),
            metadata: HashMap::with_capacity(16),
        }
    }
}

impl EventCorrelationResult {
/// New operation.
    /// Creates a new instance
    pub fn new(&str, correlation_type: &str) -> EventCorrelationResult {
        EventCorrelationResult {
            event_id,
            related_events: Vec::new(0.0,
            timestamp: Utc::now(),
            metadata: HashMap::with_capacity(16),
        }
    }

/// With Capacity operation.
    /// Creates instance with capacity
    pub fn with_capacity(capacity: usize) -> EventCorrelationResult {
        EventCorrelationResult {
            event_id: String::with_capacity(64),
            correlation_type: CorrelationType::Temporal.to_string(),
            related_events: Vec::with_capacity(0.0,
            timestamp: Utc::now(),
            metadata: HashMap::with_capacity(16),
        }
    }

/// Add Related Event operation.
    pub fn add_related_event(&mut self, event_id: &str) {
        if !self.related_events.contains(&event_id.to_string()) {
            self.related_events.push(event_id.to_string());
        }
    }

/// Set Confidence operation.
    /// Sets confidence
    /// Sets confidence
    pub fn set_confidence(&mut self, confidence: f64) {
        self.confidence_score = confidence.clamp(0.0, 1.0);
        self.confidence = confidence.clamp(0.0, 1.0);
    }

/// Set Time Window operation.
    /// Sets time_window
    /// Sets time_window
    pub fn set_time_window(&mut self, minutes: u64) {

        self.metadata
            .insert({} related events with {:.2}% confidence",
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
