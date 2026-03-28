// SPDX-License-Identifier: AGPL-3.0-only

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

/// Serializable engine snapshot: configuration, feeds, rules, models, and counters.
#[derive(Debug)]
pub struct ThreatDetectionEngine {
    /// Active detection configuration (thresholds, feature flags, etc.).
    pub config: ThreatDetectionConfig,
    /// Mapping of threat feeds
    pub threat_feeds: HashMap<String, ThreatIntelligenceFeed>,
    /// Collection of detection rules
    pub detection_rules: Vec<DetectionRule>,
    /// Collection of ml models
    pub ml_models: Vec<MlModel>,
    /// Collection of blocked sources
    pub blocked_sources: Vec<String>,
    /// Collection of quarantined systems
    pub quarantined_systems: Vec<String>,
    /// Collection of active threats
    pub active_threats: Vec<ThreatEvent>,
    /// The stats value
    pub stats: ThreatDetectionStats,
}

// ThreatDetectionConfig is now imported from the parent module

/// ML model row stored inside the engine’s in-memory registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlModel {
    /// Registry id for this model.
    pub id: String,
    /// Name of the item
    pub name: String,
    /// The description value
    pub description: String,
    /// The version value
    pub version: String,
    /// The accuracy value
    pub accuracy: f64,
    /// The precision value
    pub precision: f64,
    /// The recall value
    pub recall: f64,
    /// The f1 score value
    pub f1_score: f64,
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// The last updated value
    pub last_updated: DateTime<Utc>,
    /// The model type value
    pub model_type: String,
    /// Number of `training_data_size`
    pub training_data_size: u64,
    /// Whether `is_active` is enabled
    pub is_active: bool,
}

/// Threat detection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionStats {
    /// Number of `total_threats_detected`
    pub total_threats_detected: u64,
    /// Mapping of threats by severity
    pub threats_by_severity: HashMap<String, u64>,
    /// The false positive rate value
    pub false_positive_rate: f64,
    /// The detection accuracy value
    pub detection_accuracy: f64,
    /// Mean latency from ingest to classification over recent evaluations.
    pub average_detection_time_ms: f64,
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

    /// Update a threat feed\
    /// Updates `threat_feed`
    pub fn update_threat_feed(&mut self, feed: ThreatIntelligenceFeed) {
        self.threat_feeds.insert(feed.id.clone(), feed);
    }

    /// Add a detection rule
    pub fn add_detection_rule(&mut self, rule: DetectionRule) {
        self.detection_rules.push(rule);
    }

    /// Remove a detection rule
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
    #[must_use]
    pub const fn get_stats(&self) -> &ThreatDetectionStats {
        &self.stats
    }
}
