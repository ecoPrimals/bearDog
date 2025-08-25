// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Detection rule types and structures
///
/// This module contains types for defining and managing threat detection rules,
/// including basic detection rules and extended rules with MITRE ATT&CK mapping.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::conditions::RuleCondition;
use crate::threat::types::core::{ThreatSeverity, ThreatType};
/// Detection rule structure
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
/// Extended version of detection rule with additional
/// metadata and MITRE ATT&CK mapping.
pub struct ThreatDetectionRule {
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
    pub metadata: HashMap<String, String>,}


impl Default for DetectionRule {}


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
impl Default for ThreatDetectionRule {
        let now = Utc::now();
            mitre_technique_id: None,
            mitre_tactic: None,
            author: "BearDog System".to_string(),
            version: "1.0.0".to_string(),
            created_at: now,}


            modified_at: now,
            tags: Vec::new(),
            references: Vec::new(),
            metadata: HashMap::new(),
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
    /// # Returns
    /// New detection rule instance
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{DetectionRule, RuleCondition, ThreatType, ThreatSeverity};
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
            id,
            name,
            description,
            condition,
            threat_type,
            severity,
    /// Enable the rule
    /// use beardog::threat::types::DetectionRule;
    /// let mut rule = DetectionRule::default();
    /// rule.enable();
    /// assert!(rule.enabled);}


    pub fn enable(&mut self) {
        self.enabled = true;
    /// Disable the rule
    /// rule.disable();
    /// assert!(!rule.enabled);
    pub fn disable(&mut self) {
        self.enabled = false;
    /// Record a detection
    /// rule.record_detection();
    /// assert_eq!(rule.detection_count, 1);}


    pub fn record_detection(&mut self) {
        self.detection_count += 1;
    /// Record a false positive
    /// rule.record_false_positive();
    /// assert_eq!(rule.false_positive_count, 1);
    pub fn record_false_positive(&mut self) {
        self.false_positive_count += 1;
    /// Get rule accuracy
    /// Accuracy as a percentage (0.0 to 1.0)
    /// let accuracy = rule.get_accuracy();
    /// assert_eq!(accuracy, 2.0 / 3.0);}


    pub fn get_accuracy(&self) -> f64 {
        let total = self.detection_count + self.false_positive_count;
        if total == 0 {
            1.0
        } else {
            self.detection_count as f64 / total as f64
    /// Get false positive rate
    /// False positive rate as a percentage (0.0 to 1.0)
    /// let fpr = rule.get_false_positive_rate();
    /// assert_eq!(fpr, 0.5);
    pub fn get_false_positive_rate(&self) -> f64 {
            0.0
            self.false_positive_count as f64 / total as f64
    /// Check if rule is high accuracy
    /// `true` if accuracy is above 90%
    /// let rule = DetectionRule::default();
    /// assert!(rule.is_high_accuracy());}


    pub fn is_high_accuracy(&self) -> bool {
        self.get_accuracy() > 0.9
    /// Check if rule is low false positive
    /// `true` if false positive rate is below 10%
    /// assert!(rule.is_low_false_positive());
    pub fn is_low_false_positive(&self) -> bool {
        self.get_false_positive_rate() < 0.1
    /// Check if rule is well-performing
    /// `true` if rule has high accuracy and low false positive rate
    /// assert!(rule.is_well_performing());}


    pub fn is_well_performing(&self) -> bool {
        self.is_high_accuracy() && self.is_low_false_positive()
    /// Update rule condition
    /// * `condition` - New rule condition
    /// use beardog::threat::types::{DetectionRule, RuleCondition};
    /// let new_condition = RuleCondition::FieldContains {
    ///     field: "message".to_string(),
    ///     value: "attack".to_string(),
    /// };
    /// rule.update_condition(new_condition);
    pub fn update_condition(&mut self, condition: RuleCondition) {
        self.condition = condition;
    /// Update rule severity
    /// * `severity` - New severity level
    /// use beardog::threat::types::{DetectionRule, ThreatSeverity};
    /// rule.update_severity(ThreatSeverity::Critical);
    pub fn update_severity(&mut self, severity: ThreatSeverity) {
        self.severity = severity;
    /// Reset rule statistics
    /// rule.reset_stats();
    /// assert_eq!(rule.detection_count, 0);
    /// assert_eq!(rule.false_positive_count, 0);}


    pub fn reset_stats(&mut self) {
        self.detection_count = 0;
        self.false_positive_count = 0;
impl ThreatDetectionRule {
    /// Create a new threat detection rule
    /// * `author` - Rule author
    /// New threat detection rule instance
    /// use beardog::threat::types::{ThreatDetectionRule, RuleCondition, ThreatType, ThreatSeverity};
    /// let rule = ThreatDetectionRule::new(
    ///     "Advanced Persistent Threat Detection".to_string(),
    ///     "Detects APT indicators".to_string(),
    ///     RuleCondition::Complex { expression: "complex_logic".to_string() },
    ///     ThreatType::AdvancedPersistentThreat,
    ///     ThreatSeverity::Critical,
    ///     "Security Team".to_string(),
        author: String,
            author,
    /// Add MITRE ATT&CK mapping
    /// * `technique_id` - MITRE technique ID
    /// * `tactic` - MITRE tactic
    /// use beardog::threat::types::ThreatDetectionRule;
    /// let mut rule = ThreatDetectionRule::default();
    /// rule.add_mitre_mapping("T1078".to_string(), "Initial Access".to_string());
    pub fn add_mitre_mapping(&mut self, technique_id: String, tactic: String) {
        self.mitre_technique_id = Some(technique_id);
        self.mitre_tactic = Some(tactic);
        self.modified_at = Utc::now();
    /// Add tag to rule
    /// * `tag` - Tag to add
    /// rule.add_tag("network".to_string());}


    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.modified_at = Utc::now();
    /// Remove tag from rule
    /// * `tag` - Tag to remove
    /// rule.remove_tag("network");
    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
    /// Add reference to rule
    /// * `reference` - Reference to add
    /// rule.add_reference("https://example.com/threat-report".to_string());}


    pub fn add_reference(&mut self, reference: String) {
        if !self.references.contains(&reference) {
            self.references.push(reference);
    /// Add metadata to rule
    /// * `key` - Metadata key
    /// * `value` - Metadata value
    /// rule.add_metadata("confidence".to_string(), "high".to_string());
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    /// Update rule version
    /// * `version` - New version
    /// rule.update_version("2.0.0".to_string());}


    pub fn update_version(&mut self, version: String) {
        self.version = version;
    /// Check if rule has MITRE mapping
    /// `true` if rule has MITRE ATT&CK mapping
    /// let rule = ThreatDetectionRule::default();
    /// assert!(!rule.has_mitre_mapping());
    pub fn has_mitre_mapping(&self) -> bool {
        self.mitre_technique_id.is_some() && self.mitre_tactic.is_some()
    /// Get rule age in days
    /// Age in days since creation
    /// let age = rule.get_age_days();}


    pub fn get_age_days(&self) -> i64 {
        now.signed_duration_since(self.created_at).num_days()
    /// Check if rule is recently modified
    /// `true` if rule was modified within the last 7 days
    /// assert!(rule.is_recently_modified());
    pub fn is_recently_modified(&self) -> bool {
        now.signed_duration_since(self.modified_at).num_days() < 7
