// SPDX-License-Identifier: AGPL-3.0-only

// Engine Conditions - Modern Implementation
//
// **MODERNIZED**: Clean, production-ready condition types for the BearDog threat detection engine.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Rich predicate tree evaluated against string-keyed security events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    /// Always matches (default condition)
    Always,

    /// Field equals a specific value
    FieldEquals {
        /// Event field key.
        field: String,
        /// Expected string value.
        value: String,
    },

    /// Field contains a specific value
    FieldContains {
        /// Event field key.
        field: String,
        /// Substring that must be present.
        value: String,
    },

    /// Field matches a regex pattern
    FieldMatches {
        /// Event field key.
        field: String,
        /// Rust regex pattern text (engine must compile it).
        pattern: String,
    },

    /// Field is greater than a numeric value
    FieldGreaterThan {
        /// Event field key parsed as `f64`.
        field: String,
        /// Lower bound; match when field value is strictly greater.
        value: f64,
    },

    /// Field is less than a numeric value
    FieldLessThan {
        /// Event field key parsed as `f64`.
        field: String,
        /// Upper bound; match when field value is strictly less.
        value: f64,
    },

    /// Field is between two numeric values
    FieldBetween {
        /// Event field key parsed as `f64`.
        field: String,
        /// Inclusive minimum.
        min: f64,
        /// Inclusive maximum.
        max: f64,
    },

    /// Field exists in the event data
    FieldExists {
        /// Key that must be present in the map.
        field: String,
    },

    /// Field is in a list of values
    FieldIn {
        /// Event field key.
        field: String,
        /// Allowed string values.
        values: Vec<String>,
    },

    /// Logical AND of multiple conditions
    LogicalAnd {
        /// All must evaluate true.
        conditions: Vec<Self>,
    },

    /// Logical OR of multiple conditions
    LogicalOr {
        /// At least one must evaluate true.
        conditions: Vec<Self>,
    },

    /// Logical NOT of a condition
    LogicalNot {
        /// Negated subtree.
        condition: Box<Self>,
    },

    /// Frequency threshold condition
    FrequencyThreshold {
        /// Number of matching events required.
        count: u32,
        /// Rolling window length in minutes.
        window_minutes: u32,
    },

    /// Time-based condition
    TimeWindow {
        /// Inclusive start hour (0–23).
        start_hour: u8,
        /// Inclusive end hour (0–23).
        end_hour: u8,
    },

    /// Custom condition with arbitrary logic
    Custom {
        /// Registered plugin or handler name.
        name: String,
        /// Opaque string parameters interpreted by the named handler.
        parameters: HashMap<String, String>,
    },
}

impl RuleCondition {
    /// Check if this condition is complex (has nested conditions)
    /// Checks if complex
    #[must_use]
    pub const fn is_complex(&self) -> bool {
        matches!(
            self,
            Self::LogicalAnd { .. }
                | Self::LogicalOr { .. }
                | Self::LogicalNot { .. }
                | Self::Custom { .. }
        )
    }

    /// Calculate complexity score of the condition
    #[must_use]
    pub fn complexity_score(&self) -> u32 {
        match self {
            Self::LogicalAnd { conditions } | Self::LogicalOr { conditions } => {
                1 + conditions.iter().map(Self::complexity_score).sum::<u32>()
            }
            Self::LogicalNot { condition } => 1 + condition.complexity_score(),
            Self::Custom { .. } => 3,
            Self::FrequencyThreshold { .. } => 2,
            _ => 1,
        }
    }

    /// Get all field names referenced by this condition
    #[must_use]
    pub fn referenced_fields(&self) -> Vec<String> {
        match self {
            Self::FieldEquals { field, .. }
            | Self::FieldContains { field, .. }
            | Self::FieldMatches { field, .. }
            | Self::FieldGreaterThan { field, .. }
            | Self::FieldLessThan { field, .. }
            | Self::FieldBetween { field, .. }
            | Self::FieldExists { field }
            | Self::FieldIn { field, .. } => vec![field.clone()],

            Self::LogicalAnd { conditions } | Self::LogicalOr { conditions } => conditions
                .iter()
                .flat_map(Self::referenced_fields)
                .collect(),

            Self::LogicalNot { condition } => condition.referenced_fields(),

            _ => vec![],
        }
    }

    /// Evaluate the condition against event data
    #[must_use]
    pub fn evaluate(&self, event_data: &HashMap<String, String>) -> bool {
        match self {
            Self::FieldEquals { field, value } => event_data.get(field) == Some(value),

            Self::FieldContains { field, value } => {
                event_data.get(field).is_some_and(|v| v.contains(value))
            }

            Self::FieldExists { field } => event_data.contains_key(field),

            Self::FieldIn { field, values } => {
                event_data.get(field).is_some_and(|v| values.contains(v))
            }

            Self::FieldGreaterThan { field, value } => event_data
                .get(field)
                .and_then(|v| v.parse::<f64>().ok())
                .is_some_and(|v| v > *value),

            Self::FieldLessThan { field, value } => event_data
                .get(field)
                .and_then(|v| v.parse::<f64>().ok())
                .is_some_and(|v| v < *value),

            Self::FieldBetween { field, min, max } => event_data
                .get(field)
                .and_then(|v| v.parse::<f64>().ok())
                .is_some_and(|v| v >= *min && v <= *max),

            Self::LogicalAnd { conditions } => conditions.iter().all(|c| c.evaluate(event_data)),

            Self::LogicalOr { conditions } => conditions.iter().any(|c| c.evaluate(event_data)),

            Self::LogicalNot { condition } => !condition.evaluate(event_data),

            // Always condition always matches
            Self::Always => true,

            // Complex conditions require additional context
            Self::FieldMatches { .. }
            | Self::FrequencyThreshold { .. }
            | Self::TimeWindow { .. }
            | Self::Custom { .. } => false, // Simplified for now
        }
    }

    /// Create a simple field equals condition
    #[must_use]
    pub fn field_equals(field: &str, value: &str) -> Self {
        Self::FieldEquals {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    /// Create a field contains condition
    #[must_use]
    pub fn field_contains(field: &str, value: &str) -> Self {
        Self::FieldContains {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    /// Create a field exists condition
    #[must_use]
    pub fn field_exists(field: &str) -> Self {
        Self::FieldExists {
            field: field.to_string(),
        }
    }

    /// Create a logical AND condition
    #[must_use]
    pub const fn and(conditions: Vec<Self>) -> Self {
        Self::LogicalAnd { conditions }
    }

    /// Create a logical OR condition
    #[must_use]
    pub const fn or(conditions: Vec<Self>) -> Self {
        Self::LogicalOr { conditions }
    }

    /// Create a logical NOT condition
    #[must_use]
    pub fn logical_not(condition: Self) -> Self {
        Self::LogicalNot {
            condition: Box::new(condition),
        }
    }
}

/// Fluent builder for chaining [`RuleCondition`] fragments before `build_and` / `build_or`.
#[derive(Debug, Default)]
pub struct ConditionBuilder {
    conditions: Vec<RuleCondition>,
}

impl ConditionBuilder {
    /// Create a new condition builder
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a field equals condition
    #[must_use]
    pub fn field_equals(mut self, field: &str, value: &str) -> Self {
        self.conditions
            .push(RuleCondition::field_equals(field, value));
        self
    }

    /// Add a field contains condition
    #[must_use]
    pub fn field_contains(mut self, field: &str, value: &str) -> Self {
        self.conditions
            .push(RuleCondition::field_contains(field, value));
        self
    }

    /// Add a field exists condition
    #[must_use]
    pub fn field_exists(mut self, field: &str) -> Self {
        self.conditions.push(RuleCondition::field_exists(field));
        self
    }

    /// Add a custom condition
    #[must_use]
    pub fn add_condition(mut self, condition: RuleCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Build as AND condition
    /// Builds and
    #[must_use]
    pub fn build_and(self) -> RuleCondition {
        if self.conditions.len() == 1 {
            // Safe: We just verified len() == 1, so we can safely remove and return the first element
            self.conditions
                .into_iter()
                .next()
                .unwrap_or(RuleCondition::Always)
        } else {
            RuleCondition::and(self.conditions)
        }
    }

    /// Build as OR condition
    /// Builds or
    #[must_use]
    pub fn build_or(self) -> RuleCondition {
        if self.conditions.len() == 1 {
            // Safe: We just verified len() == 1, so we can safely remove and return the first element
            self.conditions
                .into_iter()
                .next()
                .unwrap_or(RuleCondition::Always)
        } else {
            RuleCondition::or(self.conditions)
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
    unused_comparisons
)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_conditions() {
        let condition = RuleCondition::field_equals("event_type", "login");

        let mut event_data = HashMap::new();
        event_data.insert("event_type".to_string(), "login".to_string());

        assert!(condition.evaluate(&event_data));

        event_data.insert("event_type".to_string(), "logout".to_string());
        assert!(!condition.evaluate(&event_data));
    }

    #[test]
    fn test_field_contains() {
        let condition = RuleCondition::field_contains("message", "error ");

        let mut event_data = HashMap::new();
        event_data.insert(
            "message".to_string(),
            "This is an error message".to_string(),
        );

        assert!(condition.evaluate(&event_data));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        event_data.insert("message".to_string(), "Success message".to_string());
        assert!(!condition.evaluate(&event_data));
    }

    #[test]
    fn test_field_exists() {
        let condition = RuleCondition::field_exists("user_id");

        let mut event_data = HashMap::new();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        event_data.insert("user_id".to_string(), "admin".to_string());

        assert!(condition.evaluate(&event_data));

        event_data.remove("user_id");
        assert!(!condition.evaluate(&event_data));
    }

    #[test]
    fn test_logical_and() {
        let condition = RuleCondition::and(vec![
            RuleCondition::field_equals("event_type", "login"),
            RuleCondition::field_exists("user_id"),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
        ]);

        let mut event_data = HashMap::new();
        event_data.insert("event_type".to_string(), "login".to_string());
        event_data.insert("user_id".to_string(), "admin".to_string());

        assert!(condition.evaluate(&event_data));

        event_data.remove("user_id");
        assert!(!condition.evaluate(&event_data));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_logical_or() {
        let condition = RuleCondition::or(vec![
            RuleCondition::field_equals("event_type", "login"),
            RuleCondition::field_equals("event_type", "logout"),
        ]);

        let mut event_data = HashMap::new();
        event_data.insert("event_type".to_string(), "login".to_string());

        assert!(condition.evaluate(&event_data));

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        event_data.insert("event_type".to_string(), "logout".to_string());
        assert!(condition.evaluate(&event_data));

        event_data.insert("event_type".to_string(), "error ".to_string());
        assert!(!condition.evaluate(&event_data));
    }

    #[test]
    fn test_logical_not() {
        let condition =
            RuleCondition::logical_not(RuleCondition::field_equals("status", "success "));

        let mut event_data = HashMap::new();
        event_data.insert("status".to_string(), "error ".to_string());

        assert!(condition.evaluate(&event_data));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        event_data.insert("status".to_string(), "success ".to_string());
        assert!(!condition.evaluate(&event_data));
    }

    #[test]
    fn test_complexity_score() {
        let simple = RuleCondition::field_equals("field", "value");
        assert_eq!(simple.complexity_score(), 1);

        let complex = RuleCondition::and(vec![
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            simple.clone(),
            RuleCondition::or(vec![
                RuleCondition::field_exists("field1"),
                RuleCondition::field_exists("field2"),
            ]),
        ]);

        assert!(complex.complexity_score() > simple.complexity_score());
    }

    #[test]
    fn test_referenced_fields() {
        let condition = RuleCondition::and(vec![
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            RuleCondition::field_equals("event_type", "login"),
            RuleCondition::field_exists("user_id"),
            RuleCondition::field_contains("message", "success "),
        ]);

        let fields = condition.referenced_fields();
        assert!(fields.contains(&"event_type".to_string()));
        assert!(fields.contains(&"user_id".to_string()));
        assert!(fields.contains(&"message".to_string()));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_condition_builder() {
        let condition = ConditionBuilder::new()
            .field_equals("event_type", "login")
            .field_exists("user_id")
            .field_contains("message", "success ")
            .build_and();

        assert!(condition.is_complex());
        assert_eq!(condition.complexity_score(), 4); // 1 + 3 sub-conditions
    }

    #[test]
    fn test_numeric_and_membership_and_always() {
        let mut m = HashMap::from([("n".to_string(), "10".to_string())]);
        assert!(
            RuleCondition::FieldGreaterThan {
                field: "n".into(),
                value: 5.0,
            }
            .evaluate(&m)
        );
        assert!(
            !RuleCondition::FieldGreaterThan {
                field: "n".into(),
                value: 15.0,
            }
            .evaluate(&m)
        );
        m.insert("x".to_string(), "not-a-number".to_string());
        assert!(
            !RuleCondition::FieldGreaterThan {
                field: "x".into(),
                value: 0.0,
            }
            .evaluate(&m)
        );

        let mut m2 = HashMap::from([("v".to_string(), "3".to_string())]);
        assert!(
            RuleCondition::FieldBetween {
                field: "v".into(),
                min: 1.0,
                max: 5.0,
            }
            .evaluate(&m2)
        );
        m2.insert("v".to_string(), "0".to_string());
        assert!(
            !RuleCondition::FieldBetween {
                field: "v".into(),
                min: 1.0,
                max: 5.0,
            }
            .evaluate(&m2)
        );

        assert!(
            RuleCondition::FieldLessThan {
                field: "n".into(),
                value: 20.0,
            }
            .evaluate(&m)
        );
        assert!(
            RuleCondition::FieldIn {
                field: "n".into(),
                values: vec!["10".into(), "11".into()],
            }
            .evaluate(&m)
        );
        assert!(RuleCondition::Always.evaluate(&HashMap::new()));
    }

    #[test]
    fn test_logical_and_or_empty() {
        assert!(RuleCondition::and(vec![]).evaluate(&HashMap::new()));
        assert!(!RuleCondition::or(vec![]).evaluate(&HashMap::new()));
    }

    #[test]
    fn test_field_matches_frequency_time_custom_evaluate_false() {
        let m = HashMap::from([("f".to_string(), "v".to_string())]);
        assert!(
            !RuleCondition::FieldMatches {
                field: "f".into(),
                pattern: ".*".into(),
            }
            .evaluate(&m)
        );
        assert!(
            !RuleCondition::FrequencyThreshold {
                count: 1,
                window_minutes: 5,
            }
            .evaluate(&m)
        );
        assert!(
            !RuleCondition::TimeWindow {
                start_hour: 0,
                end_hour: 23,
            }
            .evaluate(&m)
        );
        assert!(
            !RuleCondition::Custom {
                name: "x".into(),
                parameters: HashMap::new(),
            }
            .evaluate(&m)
        );
    }

    #[test]
    fn test_complexity_frequency_threshold_and_custom() {
        let ft = RuleCondition::FrequencyThreshold {
            count: 2,
            window_minutes: 1,
        };
        assert_eq!(ft.complexity_score(), 2);
        let c = RuleCondition::Custom {
            name: "p".into(),
            parameters: HashMap::new(),
        };
        assert_eq!(c.complexity_score(), 3);
    }

    #[test]
    fn test_rule_condition_logical_not_inverts_evaluation() {
        let not = RuleCondition::LogicalNot {
            condition: Box::new(RuleCondition::field_equals("k", "v")),
        };
        let mut m = HashMap::new();
        m.insert("k".to_string(), "v".to_string());
        assert!(!not.evaluate(&m));
        m.insert("k".to_string(), "other".to_string());
        assert!(not.evaluate(&m));
    }
}
