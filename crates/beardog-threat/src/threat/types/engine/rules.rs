// SPDX-License-Identifier: AGPL-3.0-or-later

// Detection Rules - Modern Implementation
//
// **MODERNIZED**: Clean, production-ready detection rule types for the BearDog threat detection engine.

use super::conditions::RuleCondition;
use crate::threat::types::{ThreatSeverity, ThreatType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Executable detection rule with auditing metadata and MITRE tagging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRule {
    /// Unique rule id for APIs and metrics.
    pub id: String,
    /// Name of the item
    pub name: String,
    /// The description value
    pub description: String,
    /// The rule type value
    pub rule_type: ThreatRuleType,
    /// The severity value
    pub severity: ThreatSeverity,
    /// The condition value
    pub condition: RuleCondition,
    /// Whether feature is enabled
    pub enabled: bool,
    /// Number of priority
    pub priority: u8,
    /// Collection of tags
    pub tags: Vec<String>,
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// The updated at value
    pub updated_at: DateTime<Utc>,
    /// The created by value
    pub created_by: String,
    /// The version value
    pub version: String,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    // Additional fields needed by handlers
    /// The threat type value
    pub threat_type: ThreatType,
    /// Number of detection
    pub detection_count: u64,
    /// Number of `false_positive`
    pub false_positive_count: u64,
    /// Estimated precision prior used when fusing with ML or intel signals.
    pub confidence_score: f64,
    /// The detection logic value
    pub detection_logic: String,
    /// Optional mitre technique
    pub mitre_technique: Option<String>,
    /// Optional mitre tactic
    pub mitre_tactic: Option<String>,
    /// The author value
    pub author: String,
    /// Collection of actions
    pub actions: Vec<String>,
    /// The modified at value
    pub modified_at: DateTime<Utc>,
    /// Collection of references
    pub references: Vec<String>,
}

impl Default for DetectionRule {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: "Default Rule".to_string(),
            description: "Default detection rule".to_string(),
            rule_type: ThreatRuleType::Signature,
            severity: ThreatSeverity::Medium,
            condition: RuleCondition::Always,
            enabled: true,
            priority: 5,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            created_by: "system".to_string(),
            version: "1.0.0".to_string(),
            metadata: HashMap::new(),
            // Additional fields
            threat_type: ThreatType::Unknown,
            detection_count: 0,
            false_positive_count: 0,
            confidence_score: 0.5,
            detection_logic: "default_logic".to_string(),
            mitre_technique: None,
            mitre_tactic: None,
            author: "system".to_string(),
            actions: Vec::new(),
            modified_at: now,
            references: Vec::new(),
        }
    }
}

/// Types of threat detection rules
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of threat rule
pub enum ThreatRuleType {
    /// Signature-based detection
    Signature,

    /// Behavioral analysis
    Behavioral,

    /// Anomaly detection
    Anomaly,

    /// Heuristic analysis
    Heuristic,

    /// Machine learning based
    MachineLearning,

    /// Correlation rule
    Correlation,

    /// Threshold-based rule
    Threshold,

    /// Custom rule type
    Custom(String),
}

/// Rule execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleExecutionResult {
    /// Id of the evaluated rule.
    pub rule_id: String,
    /// Name of the rule
    pub rule_name: String,
    /// Whether matched is enabled
    pub matched: bool,
    /// Confidence assigned by the rule engine when `matched` is true.
    pub confidence: f64,
    /// Time spent evaluating the rule predicate.
    pub execution_time_ms: u64,
    /// Collection of matched conditions
    pub matched_conditions: Vec<String>,
    /// Optional threat type
    pub threat_type: Option<ThreatType>,
    /// The severity value
    pub severity: ThreatSeverity,
    /// Human-readable strings explaining why the rule fired or did not.
    pub evidence: Vec<String>,
    /// Evaluation time in UTC.
    pub timestamp: DateTime<Utc>,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Rolling quality and latency metrics for a single rule id.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulePerformanceMetrics {
    /// Rule these metrics summarize.
    pub rule_id: String,
    /// Number of `total_executions`
    pub total_executions: u64,
    /// Number of `true_positives`
    pub true_positives: u64,
    /// Number of `false_positives`
    pub false_positives: u64,
    /// Number of `true_negatives`
    pub true_negatives: u64,
    /// Number of `false_negatives`
    pub false_negatives: u64,
    /// Mean wall time per `execute` call, updated incrementally.
    pub avg_execution_time_ms: f64,
    /// The accuracy value
    pub accuracy: f64,
    /// The precision value
    pub precision: f64,
    /// The recall value
    pub recall: f64,
    /// The f1 score value
    pub f1_score: f64,
    /// The last updated value
    pub last_updated: DateTime<Utc>,
}

/// Outcome of static checks on a [`DetectionRule`] definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleValidationResult {
    /// True when there are no blocking errors in `errors`.
    pub is_valid: bool,
    /// Collection of errors
    pub errors: Vec<String>,
    /// Collection of warnings
    pub warnings: Vec<String>,
    /// Collection of suggestions
    pub suggestions: Vec<String>,
}

impl DetectionRule {
    /// Create a new detection rule
    /// Creates a new instance
    #[must_use]
    pub fn new(
        id: &str,
        name: &str,
        description: &str,
        rule_type: ThreatRuleType,
        severity: ThreatSeverity,
        condition: RuleCondition,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            rule_type,
            severity,
            condition,
            enabled: true,
            priority: 50, // Medium priority by default
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            created_by: "system".to_string(),
            version: "1.0.0".to_string(),
            metadata: HashMap::new(),
            // Additional fields
            threat_type: ThreatType::Unknown,
            detection_count: 0,
            false_positive_count: 0,
            confidence_score: 0.5,
            detection_logic: format!("Rule: {name}"),
            mitre_technique: None,
            mitre_tactic: None,
            author: "system".to_string(),
            actions: Vec::new(),
            modified_at: now,
            references: Vec::new(),
        }
    }

    /// Execute the rule against event data
    /// Executes operation
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Rule execution duration ms fits u64 for reporting"
    )]
    pub fn execute(&self, event_data: &HashMap<String, String>) -> RuleExecutionResult {
        let start_time = std::time::Instant::now();

        let matched = self.enabled && self.condition.evaluate(event_data);
        let execution_time = start_time.elapsed().as_millis() as u64;

        RuleExecutionResult {
            rule_id: self.id.clone(),
            rule_name: self.name.clone(),
            matched,
            confidence: if matched { 0.8 } else { 0.0 },
            execution_time_ms: execution_time,
            matched_conditions: if matched {
                self.condition.referenced_fields()
            } else {
                Vec::new()
            },
            threat_type: if matched {
                Some(ThreatType::Suspicious)
            } else {
                None
            },
            severity: self.severity.clone(),
            evidence: if matched {
                vec![format!("Rule {} matched", self.name)]
            } else {
                Vec::new()
            },
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Validate the rule configuration
    /// Validates input
    #[must_use]
    pub fn validate(&self) -> RuleValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();

        // Check required fields
        if self.id.is_empty() {
            errors.push("Rule ID cannot be empty".to_string());
        }

        if self.name.is_empty() {
            errors.push("Rule name cannot be empty".to_string());
        }

        if self.description.is_empty() {
            warnings.push("Rule description is empty".to_string());
        }

        // Check complexity
        if self.condition.complexity_score() > 10 {
            warnings.push("Rule condition is very complex, consider simplifying".to_string());
        }

        // Check priority range
        if self.priority > 100 {
            errors.push("Rule priority must be between 0 and 100".to_string());
        }

        // Suggestions
        if self.tags.is_empty() {
            suggestions.push("Consider adding tags for better rule organization".to_string());
        }

        RuleValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            suggestions,
        }
    }

    /// Enable the rule
    pub fn enable(&mut self) {
        self.enabled = true;
        self.updated_at = Utc::now();
    }

    /// Disable the rule
    pub fn disable(&mut self) {
        self.enabled = false;
        self.updated_at = Utc::now();
    }

    /// Add a tag to the rule
    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
            self.updated_at = Utc::now();
        }
    }

    /// Remove a tag from the rule
    /// Removes tag
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = Utc::now();
    }

    /// Update rule priority
    /// Sets priority
    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority.min(100);
        self.updated_at = Utc::now();
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
        self.updated_at = Utc::now();
    }

    /// Check if rule is high priority
    /// Checks if high priority
    #[must_use]
    pub const fn is_high_priority(&self) -> bool {
        self.priority >= 80
    }

    /// Check if rule is complex
    /// Checks if complex
    #[must_use]
    pub const fn is_complex(&self) -> bool {
        self.condition.is_complex()
    }

    /// Get rule age in days
    #[must_use]
    pub fn age_days(&self) -> i64 {
        (Utc::now() - self.created_at).num_days()
    }
}

impl RulePerformanceMetrics {
    /// Creates a new instance
    #[must_use]
    pub fn new(rule_id: &str) -> Self {
        Self {
            rule_id: rule_id.to_string(),
            total_executions: 0,
            true_positives: 0,
            false_positives: 0,
            true_negatives: 0,
            false_negatives: 0,
            avg_execution_time_ms: 0.0,
            accuracy: 0.0,
            precision: 0.0,
            recall: 0.0,
            f1_score: 0.0,
            last_updated: Utc::now(),
        }
    }

    /// Update metrics with execution result
    /// Updates `with_result`
    pub fn update_with_result(
        &mut self,
        execution_time_ms: u64,
        was_correct: bool,
        was_positive: bool,
    ) {
        self.total_executions += 1;

        // Update execution time average with proper precision handling
        #[expect(
            clippy::cast_precision_loss,
            reason = "display/metric conversion, precision loss acceptable"
        )]
        {
            self.avg_execution_time_ms = self
                .avg_execution_time_ms
                .mul_add((self.total_executions - 1) as f64, execution_time_ms as f64)
                / self.total_executions as f64;
        }

        // Update confusion matrix
        match (was_correct, was_positive) {
            (true, true) => self.true_positives += 1,
            (true, false) => self.true_negatives += 1,
            (false, true) => self.false_positives += 1,
            (false, false) => self.false_negatives += 1,
        }

        // Recalculate metrics
        self.calculate_metrics();
        self.last_updated = Utc::now();
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "display/metric conversion, precision loss acceptable"
    )]
    fn calculate_metrics(&mut self) {
        let total =
            self.true_positives + self.true_negatives + self.false_positives + self.false_negatives;

        if total > 0 {
            self.accuracy = (self.true_positives + self.true_negatives) as f64 / total as f64;
        }

        let predicted_positive = self.true_positives + self.false_positives;
        if predicted_positive > 0 {
            self.precision = self.true_positives as f64 / predicted_positive as f64;
        }

        let actual_positive = self.true_positives + self.false_negatives;
        if actual_positive > 0 {
            self.recall = self.true_positives as f64 / actual_positive as f64;
        }

        if self.precision + self.recall > 0.0 {
            self.f1_score = 2.0 * (self.precision * self.recall) / (self.precision + self.recall);
        }
    }

    /// Heuristic check that accuracy and both precision and recall exceed policy floors.
    #[must_use]
    pub fn is_performing_well(&self) -> bool {
        self.accuracy > 0.8 && self.precision > 0.7 && self.recall > 0.7
    }
}

impl std::fmt::Display for ThreatRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Signature => write!(f, "Signature"),
            Self::Behavioral => write!(f, "Behavioral"),
            Self::Anomaly => write!(f, "Anomaly"),
            Self::Heuristic => write!(f, "Heuristic"),
            Self::MachineLearning => write!(f, "Machine Learning"),
            Self::Correlation => write!(f, "Correlation"),
            Self::Threshold => write!(f, "Threshold"),
            Self::Custom(name) => write!(f, "Custom: {name}"),
        }
    }
}

#[allow(
    unused_imports,
    clippy::module_inception,
    clippy::manual_range_contains,
    clippy::assertions_on_constants,
    clippy::useless_vec,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    reason = "detection rule tests: exhaustive patterns (multi-lint expect causes unfulfilled_lint_expectations)"
)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::threat::types::engine::conditions::RuleCondition;

    #[test]
    fn test_detection_rule_creation() {
        let condition = RuleCondition::field_equals("event_type", "login");
        let rule = DetectionRule::new(
            "rule-1",
            "Login Detection",
            "Detects login events",
            ThreatRuleType::Signature,
            ThreatSeverity::Medium,
            condition,
        );

        assert_eq!(rule.id, "rule-1");
        assert_eq!(rule.name, "Login Detection");
        assert!(rule.enabled);
        assert_eq!(rule.priority, 50);
    }

    #[test]
    fn test_rule_execution() {
        let condition = RuleCondition::field_equals("event_type", "login");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let rule = DetectionRule::new(
            "rule-1",
            "Login Detection",
            "Detects login events",
            ThreatRuleType::Signature,
            ThreatSeverity::Medium,
            condition,
        );

        let mut event_data = HashMap::new();
        event_data.insert("event_type".to_string(), "login".to_string());

        let result = rule.execute(&event_data);
        assert!(result.matched);
        assert_eq!(result.rule_id, "rule-1");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(!result.evidence.is_empty());
    }

    #[test]
    fn test_rule_validation() {
        let condition = RuleCondition::field_equals("event_type", "login");
        let rule = DetectionRule::new(
            "rule-1",
            "Login Detection",
            "Detects login events",
            ThreatRuleType::Signature,
            ThreatSeverity::Medium,
            condition,
        );

        let validation = rule.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_rule_validation_errors() {
        let condition = RuleCondition::field_equals("event_type", "login");
        let mut rule = DetectionRule::new(
            "", // Empty ID should cause error
            "", // Empty name should cause error
            "",
            ThreatRuleType::Signature,
            ThreatSeverity::Medium,
            condition,
        );
        rule.priority = 150; // Invalid priority
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important

        let validation = rule.validate();
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }

    #[test]
    fn test_rule_performance_metrics() {
        let mut metrics = RulePerformanceMetrics::new("rule-1");

        // Simulate some executions
        metrics.update_with_result(10, true, true); // True positive
        metrics.update_with_result(15, true, false); // True negative
        metrics.update_with_result(12, false, true); // False positive
        metrics.update_with_result(8, false, false); // False negative
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        assert_eq!(metrics.total_executions, 4);
        assert_eq!(metrics.true_positives, 1);
        assert_eq!(metrics.false_positives, 1);
        assert!(metrics.avg_execution_time_ms > 0.0);
    }

    #[test]
    fn test_rule_tags() {
        let condition = RuleCondition::field_equals("event_type", "login");
        let mut rule = DetectionRule::new(
            "rule-1",
            "Login Detection",
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            "Detects login events",
            ThreatRuleType::Signature,
            ThreatSeverity::Medium,
            condition,
        );

        rule.add_tag("authentication");
        rule.add_tag("security");
        rule.add_tag("authentication"); // Duplicate should be ignored

        assert_eq!(rule.tags.len(), 2);
        assert!(rule.tags.contains(&"authentication".to_string()));
        assert!(rule.tags.contains(&"security".to_string()));

        rule.remove_tag("authentication");
        assert_eq!(rule.tags.len(), 1);
        assert!(!rule.tags.contains(&"authentication".to_string()));
    }

    #[test]
    fn test_rule_priority() {
        let condition = RuleCondition::field_equals("event_type", "login");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let mut rule = DetectionRule::new(
            "rule-1",
            "Login Detection",
            "Detects login events",
            ThreatRuleType::Signature,
            ThreatSeverity::Medium,
            condition,
        );

        assert!(!rule.is_high_priority());

        rule.set_priority(90);
        assert!(rule.is_high_priority());
        assert_eq!(rule.priority, 90);

        // Test priority clamping
        rule.set_priority(150);
        assert_eq!(rule.priority, 100);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_rule_enable_disable() {
        let condition = RuleCondition::field_equals("event_type", "login");
        let mut rule = DetectionRule::new(
            "rule-1",
            "Login Detection",
            "Detects login events",
            ThreatRuleType::Signature,
            ThreatSeverity::Medium,
            condition,
        );

        assert!(rule.enabled);

        rule.disable();
        assert!(!rule.enabled);

        rule.enable();
        assert!(rule.enabled);
    }

    #[test]
    fn test_threat_rule_type_display_all_variants() {
        use ThreatRuleType::*;
        assert_eq!(format!("{Signature}"), "Signature");
        assert_eq!(format!("{Behavioral}"), "Behavioral");
        assert_eq!(format!("{Anomaly}"), "Anomaly");
        assert_eq!(format!("{Heuristic}"), "Heuristic");
        assert_eq!(format!("{MachineLearning}"), "Machine Learning");
        assert_eq!(format!("{Correlation}"), "Correlation");
        assert_eq!(format!("{Threshold}"), "Threshold");
        assert_eq!(format!("{}", Custom("plugin".into())), "Custom: plugin");
    }
}
