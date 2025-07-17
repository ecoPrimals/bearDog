//! Main threat detection engine structure
//!
//! This module contains the central `ThreatDetectionEngine` structure that
//! orchestrates threat detection, rule evaluation, and response coordination.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::threat::types::config::ThreatDetectionConfig;
use crate::threat::types::core::ThreatEvent;
use crate::threat::types::intelligence::ThreatIntelligenceFeed;
use crate::threat::types::statistics::ThreatDetectionStats;

use super::ml_models::MlModel;
use super::rules::DetectionRule;

/// Threat detection engine
///
/// Main engine structure that orchestrates threat detection,
/// rule evaluation, and response coordination.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreatDetectionEngine {
    /// Engine configuration
    pub config: ThreatDetectionConfig,

    /// Currently active threats
    pub active_threats: HashMap<String, ThreatEvent>,

    /// Blocked sources
    pub blocked_sources: HashSet<String>,

    /// Quarantined systems
    pub quarantined_systems: HashSet<String>,

    /// Threat intelligence feeds
    pub threat_feeds: HashMap<String, ThreatIntelligenceFeed>,

    /// Detection rules
    pub detection_rules: Vec<DetectionRule>,

    /// Engine statistics
    pub stats: ThreatDetectionStats,

    /// Machine learning models
    pub ml_models: HashMap<String, MlModel>,
}

impl ThreatDetectionEngine {
    /// Create a new threat detection engine
    ///
    /// # Arguments
    /// * `config` - Engine configuration
    ///
    /// # Returns
    /// New threat detection engine instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatDetectionEngine, ThreatDetectionConfig};
    ///
    /// let config = ThreatDetectionConfig::default();
    /// let engine = ThreatDetectionEngine::new(config);
    /// ```
    pub fn new(config: ThreatDetectionConfig) -> Self {
        Self {
            config,
            active_threats: HashMap::new(),
            blocked_sources: HashSet::new(),
            quarantined_systems: HashSet::new(),
            threat_feeds: HashMap::new(),
            detection_rules: Vec::new(),
            stats: ThreatDetectionStats::default(),
            ml_models: HashMap::new(),
        }
    }

    /// Create a placeholder instance for testing and initialization
    ///
    /// # Deprecated
    /// This method is deprecated. Use `new(config)` instead.
    pub fn placeholder() -> Self {
        tracing::warn!("🚨 Using deprecated ThreatDetectionEngine::placeholder() method. Use ThreatDetectionEngine::new(config) instead.");
        Self::new(ThreatDetectionConfig::default())
    }

    /// Add a detection rule to the engine
    ///
    /// # Arguments
    /// * `rule` - Detection rule to add
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatDetectionEngine, DetectionRule};
    ///
    /// let mut engine = ThreatDetectionEngine::default();
    /// let rule = DetectionRule::default();
    /// engine.add_rule(rule);
    /// ```
    pub fn add_rule(&mut self, rule: DetectionRule) {
        self.detection_rules.push(rule);
    }

    /// Add a detection rule to the engine (alias for add_rule)
    ///
    /// # Arguments
    /// * `rule` - Detection rule to add
    pub fn add_detection_rule(&mut self, rule: DetectionRule) {
        self.add_rule(rule);
    }

    /// Remove a detection rule from the engine
    ///
    /// # Arguments
    /// * `rule_id` - ID of the rule to remove
    ///
    /// # Returns
    /// `true` if the rule was removed, `false` if not found
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionEngine;
    ///
    /// let mut engine = ThreatDetectionEngine::default();
    /// let removed = engine.remove_rule("rule-001");
    /// ```
    pub fn remove_rule(&mut self, rule_id: &str) -> bool {
        let initial_len = self.detection_rules.len();
        self.detection_rules.retain(|rule| rule.id != rule_id);
        self.detection_rules.len() < initial_len
    }

    /// Get a detection rule by ID
    ///
    /// # Arguments
    /// * `rule_id` - ID of the rule to get
    ///
    /// # Returns
    /// Reference to the rule if found
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionEngine;
    ///
    /// let engine = ThreatDetectionEngine::default();
    /// let rule = engine.get_rule("rule-001");
    /// ```
    pub fn get_rule(&self, rule_id: &str) -> Option<&DetectionRule> {
        self.detection_rules.iter().find(|rule| rule.id == rule_id)
    }

    /// Get all detection rules
    ///
    /// # Returns
    /// Reference to the vector of detection rules
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionEngine;
    ///
    /// let engine = ThreatDetectionEngine::default();
    /// let rules = engine.get_rules();
    /// ```
    pub fn get_rules(&self) -> &Vec<DetectionRule> {
        &self.detection_rules
    }

    /// Get enabled detection rules
    ///
    /// # Returns
    /// Vector of references to enabled detection rules
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionEngine;
    ///
    /// let engine = ThreatDetectionEngine::default();
    /// let enabled_rules = engine.get_enabled_rules();
    /// ```
    pub fn get_enabled_rules(&self) -> Vec<&DetectionRule> {
        self.detection_rules
            .iter()
            .filter(|rule| rule.enabled)
            .collect()
    }

    /// Add a machine learning model to the engine
    ///
    /// # Arguments
    /// * `model` - ML model to add
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatDetectionEngine, MlModel};
    ///
    /// let mut engine = ThreatDetectionEngine::default();
    /// let model = MlModel::default();
    /// engine.add_ml_model(model);
    /// ```
    pub fn add_ml_model(&mut self, model: MlModel) {
        self.ml_models.insert(model.id.clone(), model);
    }

    /// Remove a machine learning model from the engine
    ///
    /// # Arguments
    /// * `model_id` - ID of the model to remove
    ///
    /// # Returns
    /// The removed model if it existed
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionEngine;
    ///
    /// let mut engine = ThreatDetectionEngine::default();
    /// let model = engine.remove_ml_model("model-001");
    /// ```
    pub fn remove_ml_model(&mut self, model_id: &str) -> Option<MlModel> {
        self.ml_models.remove(model_id)
    }

    /// Get a machine learning model by ID
    ///
    /// # Arguments
    /// * `model_id` - ID of the model to get
    ///
    /// # Returns
    /// Reference to the model if found
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionEngine;
    ///
    /// let engine = ThreatDetectionEngine::default();
    /// let model = engine.get_ml_model("model-001");
    /// ```
    pub fn get_ml_model(&self, model_id: &str) -> Option<&MlModel> {
        self.ml_models.get(model_id)
    }

    /// Add a threat intelligence feed
    ///
    /// # Arguments
    /// * `feed` - Threat intelligence feed to add
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatDetectionEngine, ThreatIntelligenceFeed};
    ///
    /// let mut engine = ThreatDetectionEngine::default();
    /// let feed = ThreatIntelligenceFeed::default();
    /// engine.add_threat_feed(feed);
    /// ```
    pub fn add_threat_feed(&mut self, feed: ThreatIntelligenceFeed) {
        self.threat_feeds.insert(feed.id.clone(), feed);
    }

    /// Block a source IP address
    ///
    /// # Arguments
    /// * `source` - Source IP address to block
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionEngine;
    ///
    /// let mut engine = ThreatDetectionEngine::default();
    /// engine.block_source("192.168.1.100");
    /// ```
    pub fn block_source(&mut self, source: &str) {
        self.blocked_sources.insert(source.to_string());
    }

    /// Quarantine a system
    ///
    /// # Arguments
    /// * `system` - System identifier to quarantine
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionEngine;
    ///
    /// let mut engine = ThreatDetectionEngine::default();
    /// engine.quarantine_system("workstation-001");
    /// ```
    pub fn quarantine_system(&mut self, system: &str) {
        self.quarantined_systems.insert(system.to_string());
    }

    /// Check if a source is blocked
    ///
    /// # Arguments
    /// * `source` - Source IP address to check
    ///
    /// # Returns
    /// `true` if the source is blocked
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionEngine;
    ///
    /// let engine = ThreatDetectionEngine::default();
    /// let is_blocked = engine.is_source_blocked("192.168.1.100");
    /// ```
    pub fn is_source_blocked(&self, source: &str) -> bool {
        self.blocked_sources.contains(source)
    }

    /// Check if a system is quarantined
    ///
    /// # Arguments
    /// * `system` - System identifier to check
    ///
    /// # Returns
    /// `true` if the system is quarantined
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionEngine;
    ///
    /// let engine = ThreatDetectionEngine::default();
    /// let is_quarantined = engine.is_system_quarantined("workstation-001");
    /// ```
    pub fn is_system_quarantined(&self, system: &str) -> bool {
        self.quarantined_systems.contains(system)
    }

    /// Get engine statistics
    ///
    /// # Returns
    /// Reference to the engine statistics
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionEngine;
    ///
    /// let engine = ThreatDetectionEngine::default();
    /// let stats = engine.get_stats();
    /// ```
    pub fn get_stats(&self) -> &ThreatDetectionStats {
        &self.stats
    }

    /// Update engine statistics
    ///
    /// # Arguments
    /// * `stats` - New statistics to set
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatDetectionEngine, ThreatDetectionStats};
    ///
    /// let mut engine = ThreatDetectionEngine::default();
    /// let stats = ThreatDetectionStats::default();
    /// engine.update_stats(stats);
    /// ```
    pub fn update_stats(&mut self, stats: ThreatDetectionStats) {
        self.stats = stats;
    }
}
