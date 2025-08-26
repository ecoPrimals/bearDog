

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::threat::types::config::ThreatDetectionConfig;
use crate::threat::types::core::ThreatEvent;
use crate::threat::types::intelligence::ThreatIntelligenceFeed;
use crate::threat::types::statistics::ThreatDetectionStats;
use super::ml_models::MlModel;
use super::rules::DetectionRule;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreatDetectionEngine {

    pub config: ThreatDetectionConfig,

    pub active_threats: HashMap<String, ThreatEvent>,

    pub blocked_sources: HashSet<String>,

    pub quarantined_systems: HashSet<String>,

    pub threat_feeds: HashMap<String, ThreatIntelligenceFeed>,

    pub detection_rules: Vec<DetectionRule>,

    pub stats: ThreatDetectionStats,

    pub ml_models: HashMap<String, MlModel>,
}
impl ThreatDetectionEngine {

    pub fn new(config: ThreatDetectionConfig) -> Self {
        Self {
            config,
            active_threats: HashMap::with_capacity(16),
            blocked_sources: HashSet::new(),
            quarantined_systems: HashSet::new(),
            threat_feeds: HashMap::with_capacity(16),
            detection_rules: Vec::new(),
            stats: ThreatDetectionStats::default(),
            ml_models: HashMap::with_capacity(16),
        }
    }

    pub fn add_rule(&mut self, rule: DetectionRule) {
        self.detection_rules.push(rule);
    }

    pub fn add_detection_rule(&mut self, rule: DetectionRule) {
        self.add_rule(rule);
    }

    pub fn remove_rule(&mut self, rule_id: &str) -> bool {
        let initial_len = self.detection_rules.len();
        self.detection_rules.retain(|rule| rule.id != rule_id);
        self.detection_rules.len() < initial_len
    }

    pub fn get_rule(&self, rule_id: &str) -> Option<&DetectionRule> {
        self.detection_rules.iter().find(|rule| rule.id == rule_id)
    }

    pub fn get_rules(&self) -> &Vec<DetectionRule> {
        &self.detection_rules
    }

    pub fn get_enabled_rules(&self) -> Vec<&DetectionRule> {
        self.detection_rules
            .iter()
            .filter(|rule| rule.enabled)
            .collect()
    }

    pub fn add_ml_model(&mut self, model: MlModel) {
        self.ml_models.insert(model.id.clone(), model);
    }

    pub fn remove_ml_model(&mut self, model_id: &str) -> Option<MlModel> {
        self.ml_models.remove(model_id)
    }

    pub fn get_ml_model(&self, model_id: &str) -> Option<&MlModel> {
        self.ml_models.get(model_id)
    }

    pub fn add_threat_feed(&mut self, feed: ThreatIntelligenceFeed) {
        self.threat_feeds.insert(feed.id.clone(), feed);
    }

    pub fn block_source(&mut self, source: &str) {
        self.blocked_sources.insert(source.to_string());
    }

    pub fn quarantine_system(&mut self, system: &str) {
        self.quarantined_systems.insert(system.to_string());
    }

    pub fn is_source_blocked(&self, source: &str) -> bool {
        self.blocked_sources.contains(source)
    }

    pub fn is_system_quarantined(&self, system: &str) -> bool {
        self.quarantined_systems.contains(system)
    }

    pub fn get_stats(&self) -> &ThreatDetectionStats {
        &self.stats
    }

    pub fn update_stats(&mut self, stats: ThreatDetectionStats) {
        self.stats = stats;
    }
}
