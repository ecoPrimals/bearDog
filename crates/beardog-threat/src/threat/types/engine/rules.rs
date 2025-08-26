

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::super::core::{ThreatSeverity, ThreatType};
use super::conditions::RuleCondition;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatRuleType {
    Behavioral,
    Signature,
    Anomaly,
    Statistical,
    MachineLearning,
}

impl Default for ThreatRuleType {
    fn default() -> Self {
        Self::Behavioral
    }
}

impl std::fmt::Display for ThreatRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreatRuleType::Behavioral => write!(f, "Behavioral"),
            ThreatRuleType::Signature => write!(f, "Signature"),
            ThreatRuleType::Anomaly => write!(f, "Anomaly"),
            ThreatRuleType::Statistical => write!(f, "Statistical"),
            ThreatRuleType::MachineLearning => write!(f, "Machine Learning"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub condition: RuleCondition,
    pub threat_type: ThreatType,
    pub severity: ThreatSeverity,
    pub enabled: bool,
    pub detection_count: u64,
    pub false_positive_count: u64,
    // Additional fields that are referenced in the code
    pub rule_type: String,
    pub confidence: f64,
    pub detection_logic: String,
    pub mitre_technique_id: Option<String>,
    pub mitre_tactic: Option<String>,
    pub author: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub references: Vec<String>,
    pub last_triggered: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionRule {
    pub rule_id: String,
    pub name: String,
    pub description: String,
    pub severity: ThreatSeverity,
    pub enabled: bool,
    pub conditions: Vec<RuleCondition>,
    pub actions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub mitre_technique_id: Option<String>,
    pub mitre_tactic: Option<String>,
    pub author: String,
    pub version: String,
    pub tags: Vec<String>,
    pub references: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl Default for DetectionRule {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Default Rule".to_string(),
            description: "Default detection rule".to_string(),
            condition: RuleCondition::Always,
            threat_type: ThreatType::Unknown,
            severity: ThreatSeverity::Medium,
            enabled: true,
            detection_count: 0,
            false_positive_count: 0,
            rule_type: "behavioral".to_string(),
            confidence: 0.5,
            detection_logic: "default_detection".to_string(),
            mitre_technique_id: None,
            mitre_tactic: None,
            author: "BearDog System".to_string(),
            version: "1.0.0".to_string(),
            created_at: now,
            modified_at: now,
            tags: Vec::new(),
            references: Vec::new(),
            last_triggered: None,
            metadata: HashMap::new(),
        }
    }
}

impl Default for ThreatDetectionRule {
    fn default() -> Self {
        Self {
            rule_id: uuid::Uuid::new_v4().to_string(),
            name: "Default Rule".to_string(),
            description: String::new(),
            severity: super::super::core::ThreatSeverity::Medium,
            enabled: true,
            conditions: Vec::new(),
            actions: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            mitre_technique_id: None,
            mitre_tactic: None,
            author: "BearDog System".to_string(),
            version: "1.0.0".to_string(),
            tags: Vec::new(),
            references: Vec::new(),
            metadata: HashMap::with_capacity(16),
        }
    }
}

impl DetectionRule {

    pub fn new(
        name: &str,
        description: &str,
        rule_type: ThreatRuleType,
        severity: ThreatSeverity,
        detection_logic: &str,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: description.to_string(),
            condition: RuleCondition::Always, // Default condition
            threat_type: ThreatType::Unknown,
            severity,
            enabled: true,
            detection_count: 0,
            false_positive_count: 0,
            rule_type: rule_type.to_string(),
            confidence: 0.8,
            detection_logic: detection_logic.to_string(),
            mitre_technique_id: None,
            mitre_tactic: None,
            author: "BearDog System".to_string(),
            version: "1.0.0".to_string(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            tags: Vec::new(),
            references: Vec::new(),
            last_triggered: None,
            metadata: HashMap::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.modified_at = Utc::now();
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.modified_at = Utc::now();
    }

    pub fn record_detection(&mut self) {
        self.detection_count += 1;
        self.last_triggered = Some(Utc::now());
        self.modified_at = Utc::now();
    }

    pub fn record_false_positive(&mut self) {
        self.false_positive_count += 1;
        self.modified_at = Utc::now();
    }

    pub fn accuracy_rate(&self) -> f64 {
        let total = self.detection_count + self.false_positive_count;
        if total == 0 {
            1.0
        } else {
            self.detection_count as f64 / total as f64
        }
    }

    pub fn update_confidence(&mut self) {
        let accuracy = self.accuracy_rate();
        self.confidence = accuracy * 0.9; // Scale down slightly for safety
        self.modified_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: &str) {
        let tag_string = tag.to_string();
        if !self.tags.contains(&tag_string) {
            self.tags.push(tag_string);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    pub fn set_mitre_mapping(&mut self, technique_id: &str, tactic: &str) {
        self.mitre_technique_id = Some(technique_id.to_string());
        self.mitre_tactic = Some(tactic.to_string());
    }

    pub fn clear_mitre_mapping(&mut self) {
        self.mitre_technique_id = None;
        self.mitre_tactic = None;
        self.modified_at = Utc::now();
    }

    pub fn has_mitre_mapping(&self) -> bool {
        self.mitre_technique_id.is_some() && self.mitre_tactic.is_some()
    }

    pub fn get_age_days(&self) -> i64 {
        let now = Utc::now();
        now.signed_duration_since(self.created_at).num_days()
    }

    pub fn is_recently_modified(&self) -> bool {
        let now = Utc::now();
        now.signed_duration_since(self.modified_at).num_days() < 7
    }
}
