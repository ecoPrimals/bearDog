// Threat Detection Engine Types - Modernized
//
// **MODERNIZED**: Clean, production-ready engine types using canonical definitions

use crate::threat::types::{
    DetectionRule, ThreatDetectionConfig, ThreatEvent, ThreatIntelligenceFeed,
};
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// SystemTime is available via std::time:: in usage

/// Main threat detection engine
#[derive(Debug)]
pub struct ThreatDetectionEngine {
    pub config: ThreatDetectionConfig,
    /// Mapping of threat feeds
    /// Mapping of threat feeds
    pub threat_feeds: HashMap<String, ThreatIntelligenceFeed>,
    /// Collection of detection rules
    /// Collection of detection rules
    pub detection_rules: Vec<DetectionRule>,
    /// Collection of ml models
    /// Collection of ml models
    pub ml_models: Vec<MlModel>,
    /// Collection of blocked sources
    /// Collection of blocked sources
    pub blocked_sources: Vec<String>,
    /// Collection of quarantined systems
    /// Collection of quarantined systems
    pub quarantined_systems: Vec<String>,
    /// Collection of active threats
    /// Collection of active threats
    pub active_threats: Vec<ThreatEvent>,
    /// The stats value
    /// The stats value
    pub stats: ThreatDetectionStats,
}

// ThreatDetectionConfig is now imported from the parent module

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlModel {
    pub id: String,
    /// Name of the item
    /// Name of the item
    pub name: String,
    /// The description value
    /// The description value
    pub description: String,
    /// The version value
    /// The version value
    pub version: String,
    /// The accuracy value
    /// The accuracy value
    pub accuracy: f64,
    /// The precision value
    /// The precision value
    pub precision: f64,
    /// The recall value
    /// The recall value
    pub recall: f64,
    /// The f1 score value
    /// The f1 score value
    pub f1_score: f64,
    /// The created at value
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// The last updated value
    /// The last updated value
    pub last_updated: DateTime<Utc>,
    /// The model type value
    /// The model type value
    pub model_type: String,
    /// Number of training_data_size
    /// Number of training_data_size
    pub training_data_size: u64,
    /// Whether is_active is enabled
    /// Whether is_active is enabled
    pub is_active: bool,
}

/// Threat detection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionStats {
    /// Number of total_threats_detected
    /// Number of total_threats_detected
    pub total_threats_detected: u64,
    /// Mapping of threats by severity
    /// Mapping of threats by severity
    pub threats_by_severity: HashMap<String, u64>,
    /// The false positive rate value
    /// The false positive rate value
    pub false_positive_rate: f64,
    /// The detection accuracy value
    /// The detection accuracy value
    pub detection_accuracy: f64,
    pub average_detection_time_ms: f64,
    /// The last updated value
    /// The last updated value
    pub last_updated: DateTime<Utc>,
}

// Default implementation is provided in the parent module

impl Default for ThreatDetectionStats {
    fn default() -> Self {
        Self {
            total_threats_detected: 0,
            threats_by_severity: HashMap::new(),
            false_positive_rate: 0.0,
            detection_accuracy: 0.0,
            average_detection_time_ms: 0.0,
            last_updated: Utc::now(),
        }
    }
}

impl ThreatDetectionEngine {
    /// Create a new threat detection engine
    /// Creates a new instance
    pub fn new(config: ThreatDetectionConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            config,
            threat_feeds: HashMap::new(),
            detection_rules: Vec::new(),
            ml_models: Vec::new(),
            blocked_sources: Vec::new(),
            quarantined_systems: Vec::new(),
            active_threats: Vec::new(),
            stats: ThreatDetectionStats::default(),
        })
    }

    /// Add a threat feed
    pub fn add_threat_feed(&mut self, feed: ThreatIntelligenceFeed) {
        self.threat_feeds.insert(feed.id.clone(), feed);
    }

    /// Update a threat feed  
    /// Updates threat_feed
    /// Updates threat_feed
    pub fn update_threat_feed(&mut self, feed: ThreatIntelligenceFeed) {
        self.threat_feeds.insert(feed.id.clone(), feed);
    }

    /// Add a detection rule
    pub fn add_detection_rule(&mut self, rule: DetectionRule) {
        self.detection_rules.push(rule);
    }

    /// Remove a detection rule
    /// Removes rule
    /// Removes rule
    pub fn remove_rule(&mut self, rule_id: &str) -> bool {
        if let Some(pos) = self.detection_rules.iter().position(|r| r.id == rule_id) {
            self.detection_rules.remove(pos);
            true
        } else {
            false
        }
    }

    /// Add an ML model
    pub fn add_ml_model(&mut self, model: MlModel) {
        self.ml_models.push(model);
    }

    /// Get detection statistics
    /// Gets stats
    /// Gets stats
    pub fn get_stats(&self) -> &ThreatDetectionStats {
        &self.stats
    }
}
