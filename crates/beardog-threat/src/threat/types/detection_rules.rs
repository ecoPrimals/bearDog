// SPDX-License-Identifier: AGPL-3.0-or-later

//! Declarative detection rules and field predicates over telemetry maps.

use super::taxonomy::{ThreatRuleType, ThreatSeverity};
use serde::{Deserialize, Serialize};

/// Predicate evaluated against string-keyed event fields for detection rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    /// Field equals a specific value
    FieldEquals {
        /// Key in the event’s string map.
        field: String,
        /// Expected value for equality.
        value: String,
    },
    /// Field greater than a threshold
    FieldGreaterThan {
        /// Numeric field parsed as `f64` for comparison.
        field: String,
        /// Exclusive lower bound for a match.
        threshold: f64,
    },
    /// Field less than a threshold
    FieldLessThan {
        /// Numeric field parsed as `f64` for comparison.
        field: String,
        /// Exclusive upper bound for a match.
        threshold: f64,
    },
    /// Field contains a pattern
    FieldContains {
        /// Field whose string value is substring-searched.
        field: String,
        /// Substring that must appear in the field value.
        pattern: String,
    },
    /// Complex condition with multiple criteria
    Complex {
        /// Child predicates combined with implicit AND semantics in the engine.
        conditions: Vec<Self>,
    },
}

/// Declarative rule evaluated against security-event maps or fused telemetry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRule {
    /// Unique rule identifier
    pub id: String,
    /// Human-readable rule name
    /// Name of the item
    pub name: String,
    /// Rule description
    /// The description value
    pub description: String,
    /// Rule pattern or signature
    /// The pattern value
    pub pattern: String,
    /// Rule severity
    /// The severity value
    pub severity: ThreatSeverity,
    /// Rule enabled status
    /// Whether feature is enabled
    pub enabled: bool,
    /// Tuning weight or belief in the rule’s precision when fused with other signals.
    pub confidence: f64,
    /// The condition value
    pub condition: RuleCondition,
    /// The rule type value
    pub rule_type: ThreatRuleType,
}

impl Default for DetectionRule {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            pattern: String::new(),
            severity: ThreatSeverity::Medium,
            enabled: true,
            confidence: 0.5,
            condition: RuleCondition::FieldEquals {
                field: String::new(),
                value: String::new(),
            },
            rule_type: ThreatRuleType::Signature,
        }
    }
}

impl DetectionRule {
    /// Create a new detection rule
    /// Creates a new instance
    #[must_use]
    pub const fn new(
        id: String,
        name: String,
        description: String,
        pattern: String,
        severity: ThreatSeverity,
    ) -> Self {
        Self {
            id,
            name,
            description,
            pattern,
            severity,
            enabled: true,
            confidence: 0.8,
            condition: RuleCondition::FieldEquals {
                field: String::new(),
                value: String::new(),
            },
            rule_type: ThreatRuleType::Signature,
        }
    }

    /// Enable the detection rule
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the detection rule
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}
