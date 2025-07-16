//! Detection rule types and structures
//!
//! This module contains types for defining and managing threat detection rules,
//! including basic detection rules and extended rules with MITRE ATT&CK mapping.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::threat::types::core::{ThreatSeverity, ThreatType};
use super::conditions::RuleCondition;

/// Detection rule structure
///
/// Defines a rule for detecting specific types of threats
/// with conditions and response actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRule {
    /// Rule identifier
    pub id: String,

    /// Rule name
    pub name: String,

    /// Rule description
    pub description: String,

    /// Rule condition
    pub condition: RuleCondition,

    /// Threat type to generate
    pub threat_type: ThreatType,

    /// Severity level
    pub severity: ThreatSeverity,

    /// Rule enabled status
    pub enabled: bool,

    /// Detection count
    pub detection_count: u64,

    /// False positive count
    pub false_positive_count: u64,
}

/// Threat detection rule (extended)
///
/// Extended version of detection rule with additional
/// metadata and MITRE ATT&CK mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionRule {
    /// Rule identifier
    pub id: String,

    /// Rule name
    pub name: String,

    /// Rule description
    pub description: String,

    /// Rule condition
    pub condition: RuleCondition,

    /// Threat type to generate
    pub threat_type: ThreatType,

    /// Severity level
    pub severity: ThreatSeverity,

    /// Rule enabled status
    pub enabled: bool,

    /// Detection count
    pub detection_count: u64,

    /// False positive count
    pub false_positive_count: u64,

    /// MITRE ATT&CK technique ID
    pub mitre_technique_id: Option<String>,

    /// MITRE ATT&CK tactic
    pub mitre_tactic: Option<String>,

    /// Rule author
    pub author: String,

    /// Rule version
    pub version: String,

    /// Rule creation date
    pub created_at: DateTime<Utc>,

    /// Rule last modified date
    pub modified_at: DateTime<Utc>,

    /// Rule tags
    pub tags: Vec<String>,

    /// Rule references
    pub references: Vec<String>,

    /// Rule metadata
    pub metadata: HashMap<String, String>,
}

impl Default for DetectionRule {
    fn default() -> Self {
        Self {
            id: "default-rule".to_string(),
            name: "Default Rule".to_string(),
            description: "Default detection rule".to_string(),
            condition: RuleCondition::FieldEquals {
                field: "event_type".to_string(),
                value: "unknown".to_string(),
            },
            threat_type: ThreatType::Unknown,
            severity: ThreatSeverity::Low,
            enabled: true,
            detection_count: 0,
            false_positive_count: 0,
        }
    }
}

impl Default for ThreatDetectionRule {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: "default-rule".to_string(),
            name: "Default Rule".to_string(),
            description: "Default detection rule".to_string(),
            condition: RuleCondition::FieldEquals {
                field: "event_type".to_string(),
                value: "unknown".to_string(),
            },
            threat_type: ThreatType::Unknown,
            severity: ThreatSeverity::Low,
            enabled: true,
            detection_count: 0,
            false_positive_count: 0,
            mitre_technique_id: None,
            mitre_tactic: None,
            author: "BearDog System".to_string(),
            version: "1.0.0".to_string(),
            created_at: now,
            modified_at: now,
            tags: Vec::new(),
            references: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

impl DetectionRule {
    /// Create a new detection rule
    ///
    /// # Arguments
    /// * `id` - Rule identifier
    /// * `name` - Rule name
    /// * `description` - Rule description
    /// * `condition` - Rule condition
    /// * `threat_type` - Threat type to generate
    /// * `severity` - Severity level
    ///
    /// # Returns
    /// New detection rule instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{DetectionRule, RuleCondition, ThreatType, ThreatSeverity};
    ///
    /// let rule = DetectionRule::new(
    ///     "rule-001".to_string(),
    ///     "Brute Force Detection".to_string(),
    ///     "Detects brute force attacks".to_string(),
    ///     RuleCondition::FrequencyThreshold { count: 10, window_minutes: 5 },
    ///     ThreatType::BruteForce,
    ///     ThreatSeverity::High,
    /// );
    /// ```
    pub fn new(
        id: String,
        name: String,
        description: String,
        condition: RuleCondition,
        threat_type: ThreatType,
        severity: ThreatSeverity,
    ) -> Self {
        Self {
            id,
            name,
            description,
            condition,
            threat_type,
            severity,
            enabled: true,
            detection_count: 0,
            false_positive_count: 0,
        }
    }

    /// Enable the rule
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionRule;
    ///
    /// let mut rule = DetectionRule::default();
    /// rule.enable();
    /// assert!(rule.enabled);
    /// ```
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the rule
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionRule;
    ///
    /// let mut rule = DetectionRule::default();
    /// rule.disable();
    /// assert!(!rule.enabled);
    /// ```
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Record a detection
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionRule;
    ///
    /// let mut rule = DetectionRule::default();
    /// rule.record_detection();
    /// assert_eq!(rule.detection_count, 1);
    /// ```
    pub fn record_detection(&mut self) {
        self.detection_count += 1;
    }

    /// Record a false positive
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionRule;
    ///
    /// let mut rule = DetectionRule::default();
    /// rule.record_false_positive();
    /// assert_eq!(rule.false_positive_count, 1);
    /// ```
    pub fn record_false_positive(&mut self) {
        self.false_positive_count += 1;
    }

    /// Get rule accuracy
    ///
    /// # Returns
    /// Accuracy as a percentage (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionRule;
    ///
    /// let mut rule = DetectionRule::default();
    /// rule.record_detection();
    /// rule.record_detection();
    /// rule.record_false_positive();
    /// 
    /// let accuracy = rule.get_accuracy();
    /// assert_eq!(accuracy, 2.0 / 3.0);
    /// ```
    pub fn get_accuracy(&self) -> f64 {
        let total = self.detection_count + self.false_positive_count;
        if total == 0 {
            1.0
        } else {
            self.detection_count as f64 / total as f64
        }
    }

    /// Get false positive rate
    ///
    /// # Returns
    /// False positive rate as a percentage (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionRule;
    ///
    /// let mut rule = DetectionRule::default();
    /// rule.record_detection();
    /// rule.record_false_positive();
    /// 
    /// let fpr = rule.get_false_positive_rate();
    /// assert_eq!(fpr, 0.5);
    /// ```
    pub fn get_false_positive_rate(&self) -> f64 {
        let total = self.detection_count + self.false_positive_count;
        if total == 0 {
            0.0
        } else {
            self.false_positive_count as f64 / total as f64
        }
    }

    /// Check if rule is high accuracy
    ///
    /// # Returns
    /// `true` if accuracy is above 90%
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionRule;
    ///
    /// let rule = DetectionRule::default();
    /// assert!(rule.is_high_accuracy());
    /// ```
    pub fn is_high_accuracy(&self) -> bool {
        self.get_accuracy() > 0.9
    }

    /// Check if rule is low false positive
    ///
    /// # Returns
    /// `true` if false positive rate is below 10%
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionRule;
    ///
    /// let rule = DetectionRule::default();
    /// assert!(rule.is_low_false_positive());
    /// ```
    pub fn is_low_false_positive(&self) -> bool {
        self.get_false_positive_rate() < 0.1
    }

    /// Check if rule is well-performing
    ///
    /// # Returns
    /// `true` if rule has high accuracy and low false positive rate
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionRule;
    ///
    /// let rule = DetectionRule::default();
    /// assert!(rule.is_well_performing());
    /// ```
    pub fn is_well_performing(&self) -> bool {
        self.is_high_accuracy() && self.is_low_false_positive()
    }

    /// Update rule condition
    ///
    /// # Arguments
    /// * `condition` - New rule condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{DetectionRule, RuleCondition};
    ///
    /// let mut rule = DetectionRule::default();
    /// let new_condition = RuleCondition::FieldContains {
    ///     field: "message".to_string(),
    ///     value: "attack".to_string(),
    /// };
    /// rule.update_condition(new_condition);
    /// ```
    pub fn update_condition(&mut self, condition: RuleCondition) {
        self.condition = condition;
    }

    /// Update rule severity
    ///
    /// # Arguments
    /// * `severity` - New severity level
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{DetectionRule, ThreatSeverity};
    ///
    /// let mut rule = DetectionRule::default();
    /// rule.update_severity(ThreatSeverity::Critical);
    /// ```
    pub fn update_severity(&mut self, severity: ThreatSeverity) {
        self.severity = severity;
    }

    /// Reset rule statistics
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionRule;
    ///
    /// let mut rule = DetectionRule::default();
    /// rule.record_detection();
    /// rule.record_false_positive();
    /// rule.reset_stats();
    /// assert_eq!(rule.detection_count, 0);
    /// assert_eq!(rule.false_positive_count, 0);
    /// ```
    pub fn reset_stats(&mut self) {
        self.detection_count = 0;
        self.false_positive_count = 0;
    }
}

impl ThreatDetectionRule {
    /// Create a new threat detection rule
    ///
    /// # Arguments
    /// * `id` - Rule identifier
    /// * `name` - Rule name
    /// * `description` - Rule description
    /// * `condition` - Rule condition
    /// * `threat_type` - Threat type to generate
    /// * `severity` - Severity level
    /// * `author` - Rule author
    ///
    /// # Returns
    /// New threat detection rule instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatDetectionRule, RuleCondition, ThreatType, ThreatSeverity};
    ///
    /// let rule = ThreatDetectionRule::new(
    ///     "rule-001".to_string(),
    ///     "Advanced Persistent Threat Detection".to_string(),
    ///     "Detects APT indicators".to_string(),
    ///     RuleCondition::Complex { expression: "complex_logic".to_string() },
    ///     ThreatType::AdvancedPersistentThreat,
    ///     ThreatSeverity::Critical,
    ///     "Security Team".to_string(),
    /// );
    /// ```
    pub fn new(
        id: String,
        name: String,
        description: String,
        condition: RuleCondition,
        threat_type: ThreatType,
        severity: ThreatSeverity,
        author: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            description,
            condition,
            threat_type,
            severity,
            enabled: true,
            detection_count: 0,
            false_positive_count: 0,
            mitre_technique_id: None,
            mitre_tactic: None,
            author,
            version: "1.0.0".to_string(),
            created_at: now,
            modified_at: now,
            tags: Vec::new(),
            references: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add MITRE ATT&CK mapping
    ///
    /// # Arguments
    /// * `technique_id` - MITRE technique ID
    /// * `tactic` - MITRE tactic
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionRule;
    ///
    /// let mut rule = ThreatDetectionRule::default();
    /// rule.add_mitre_mapping("T1078".to_string(), "Initial Access".to_string());
    /// ```
    pub fn add_mitre_mapping(&mut self, technique_id: String, tactic: String) {
        self.mitre_technique_id = Some(technique_id);
        self.mitre_tactic = Some(tactic);
        self.modified_at = Utc::now();
    }

    /// Add tag to rule
    ///
    /// # Arguments
    /// * `tag` - Tag to add
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionRule;
    ///
    /// let mut rule = ThreatDetectionRule::default();
    /// rule.add_tag("network".to_string());
    /// ```
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.modified_at = Utc::now();
        }
    }

    /// Remove tag from rule
    ///
    /// # Arguments
    /// * `tag` - Tag to remove
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionRule;
    ///
    /// let mut rule = ThreatDetectionRule::default();
    /// rule.add_tag("network".to_string());
    /// rule.remove_tag("network");
    /// ```
    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
            self.modified_at = Utc::now();
        }
    }

    /// Add reference to rule
    ///
    /// # Arguments
    /// * `reference` - Reference to add
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionRule;
    ///
    /// let mut rule = ThreatDetectionRule::default();
    /// rule.add_reference("https://example.com/threat-report".to_string());
    /// ```
    pub fn add_reference(&mut self, reference: String) {
        if !self.references.contains(&reference) {
            self.references.push(reference);
            self.modified_at = Utc::now();
        }
    }

    /// Add metadata to rule
    ///
    /// # Arguments
    /// * `key` - Metadata key
    /// * `value` - Metadata value
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionRule;
    ///
    /// let mut rule = ThreatDetectionRule::default();
    /// rule.add_metadata("confidence".to_string(), "high".to_string());
    /// ```
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.modified_at = Utc::now();
    }

    /// Update rule version
    ///
    /// # Arguments
    /// * `version` - New version
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionRule;
    ///
    /// let mut rule = ThreatDetectionRule::default();
    /// rule.update_version("2.0.0".to_string());
    /// ```
    pub fn update_version(&mut self, version: String) {
        self.version = version;
        self.modified_at = Utc::now();
    }

    /// Check if rule has MITRE mapping
    ///
    /// # Returns
    /// `true` if rule has MITRE ATT&CK mapping
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionRule;
    ///
    /// let rule = ThreatDetectionRule::default();
    /// assert!(!rule.has_mitre_mapping());
    /// ```
    pub fn has_mitre_mapping(&self) -> bool {
        self.mitre_technique_id.is_some() && self.mitre_tactic.is_some()
    }

    /// Get rule age in days
    ///
    /// # Returns
    /// Age in days since creation
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionRule;
    ///
    /// let rule = ThreatDetectionRule::default();
    /// let age = rule.get_age_days();
    /// ```
    pub fn get_age_days(&self) -> i64 {
        let now = Utc::now();
        now.signed_duration_since(self.created_at).num_days()
    }

    /// Check if rule is recently modified
    ///
    /// # Returns
    /// `true` if rule was modified within the last 7 days
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatDetectionRule;
    ///
    /// let rule = ThreatDetectionRule::default();
    /// assert!(rule.is_recently_modified());
    /// ```
    pub fn is_recently_modified(&self) -> bool {
        let now = Utc::now();
        now.signed_duration_since(self.modified_at).num_days() < 7
    }
} 