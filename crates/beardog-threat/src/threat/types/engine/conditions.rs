// Engine Conditions - Modern Implementation
//
// **MODERNIZED**: Clean, production-ready condition types for the BearDog threat detection engine.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    /// Always matches (default condition)
    Always,

    /// Field equals a specific value
    FieldEquals { field: String, value: String },

    /// Field contains a specific value
    FieldContains { field: String, value: String },

    /// Field matches a regex pattern
    FieldMatches { field: String, pattern: String },

    /// Field is greater than a numeric value
    FieldGreaterThan { field: String, value: f64 },

    /// Field is less than a numeric value
    FieldLessThan { field: String, value: f64 },

    /// Field is between two numeric values
    FieldBetween { field: String, min: f64, max: f64 },

    /// Field exists in the event data
    FieldExists { field: String },

    /// Field is in a list of values
    FieldIn { field: String, values: Vec<String> },

    /// Logical AND of multiple conditions
    LogicalAnd { conditions: Vec<RuleCondition> },

    /// Logical OR of multiple conditions
    LogicalOr { conditions: Vec<RuleCondition> },

    /// Logical NOT of a condition
    LogicalNot { condition: Box<RuleCondition> },

    /// Frequency threshold condition
    FrequencyThreshold { count: u32, window_minutes: u32 },

    /// Time-based condition
    TimeWindow { start_hour: u8, end_hour: u8 },

    /// Custom condition with arbitrary logic
    Custom {
        name: String,
        parameters: HashMap<String, String>,
    },
}

impl RuleCondition {
    /// Check if this condition is complex (has nested conditions)
    /// Checks if complex
    /// Checks if complex
    #[must_use]
    pub fn is_complex(&self) -> bool {
        matches!(
            self,
            RuleCondition::LogicalAnd { .. }
                | RuleCondition::LogicalOr { .. }
                | RuleCondition::LogicalNot { .. }
                | RuleCondition::Custom { .. }
        )
    }

    /// Calculate complexity score of the condition
    #[must_use]
    pub fn complexity_score(&self) -> u32 {
        match self {
            RuleCondition::LogicalAnd { conditions } | RuleCondition::LogicalOr { conditions } => {
                1 + conditions
                    .iter()
                    .map(RuleCondition::complexity_score)
                    .sum::<u32>()
            }
            RuleCondition::LogicalNot { condition } => 1 + condition.complexity_score(),
            RuleCondition::Custom { .. } => 3,
            RuleCondition::FrequencyThreshold { .. } => 2,
            _ => 1,
        }
    }

    /// Get all field names referenced by this condition
    #[must_use]
    pub fn referenced_fields(&self) -> Vec<String> {
        match self {
            RuleCondition::FieldEquals { field, .. }
            | RuleCondition::FieldContains { field, .. }
            | RuleCondition::FieldMatches { field, .. }
            | RuleCondition::FieldGreaterThan { field, .. }
            | RuleCondition::FieldLessThan { field, .. }
            | RuleCondition::FieldBetween { field, .. }
            | RuleCondition::FieldExists { field }
            | RuleCondition::FieldIn { field, .. } => vec![field.clone()],

            RuleCondition::LogicalAnd { conditions } | RuleCondition::LogicalOr { conditions } => {
                conditions
                    .iter()
                    .flat_map(RuleCondition::referenced_fields)
                    .collect()
            }

            RuleCondition::LogicalNot { condition } => condition.referenced_fields(),

            _ => vec![],
        }
    }

    /// Evaluate the condition against event data
    #[must_use]
    pub fn evaluate(&self, event_data: &HashMap<String, String>) -> bool {
        match self {
            RuleCondition::FieldEquals { field, value } => event_data.get(field) == Some(value),

            RuleCondition::FieldContains { field, value } => {
                event_data.get(field).is_some_and(|v| v.contains(value))
            }

            RuleCondition::FieldExists { field } => event_data.contains_key(field),

            RuleCondition::FieldIn { field, values } => {
                event_data.get(field).is_some_and(|v| values.contains(v))
            }

            RuleCondition::FieldGreaterThan { field, value } => event_data
                .get(field)
                .and_then(|v| v.parse::<f64>().ok())
                .is_some_and(|v| v > *value),

            RuleCondition::FieldLessThan { field, value } => event_data
                .get(field)
                .and_then(|v| v.parse::<f64>().ok())
                .is_some_and(|v| v < *value),

            RuleCondition::FieldBetween { field, min, max } => event_data
                .get(field)
                .and_then(|v| v.parse::<f64>().ok())
                .is_some_and(|v| v >= *min && v <= *max),

            RuleCondition::LogicalAnd { conditions } => {
                conditions.iter().all(|c| c.evaluate(event_data))
            }

            RuleCondition::LogicalOr { conditions } => {
                conditions.iter().any(|c| c.evaluate(event_data))
            }

            RuleCondition::LogicalNot { condition } => !condition.evaluate(event_data),

            // Always condition always matches
            RuleCondition::Always => true,

            // Complex conditions require additional context
            RuleCondition::FieldMatches { .. }
            | RuleCondition::FrequencyThreshold { .. }
            | RuleCondition::TimeWindow { .. }
            | RuleCondition::Custom { .. } => false, // Simplified for now
        }
    }

    /// Create a simple field equals condition
    #[must_use]
    pub fn field_equals(field: &str, value: &str) -> Self {
        RuleCondition::FieldEquals {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    /// Create a field contains condition
    #[must_use]
    pub fn field_contains(field: &str, value: &str) -> Self {
        RuleCondition::FieldContains {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    /// Create a field exists condition
    #[must_use]
    pub fn field_exists(field: &str) -> Self {
        RuleCondition::FieldExists {
            field: field.to_string(),
        }
    }

    /// Create a logical AND condition
    #[must_use]
    pub fn and(conditions: Vec<RuleCondition>) -> Self {
        RuleCondition::LogicalAnd { conditions }
    }

    /// Create a logical OR condition
    #[must_use]
    pub fn or(conditions: Vec<RuleCondition>) -> Self {
        RuleCondition::LogicalOr { conditions }
    }

    /// Create a logical NOT condition
    #[must_use]
    pub fn logical_not(condition: RuleCondition) -> Self {
        RuleCondition::LogicalNot {
            condition: Box::new(condition),
        }
    }
}

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
    /// Builds and
    #[must_use]
    pub fn build_and(self) -> RuleCondition {
        if self.conditions.len() == 1 {
            self.conditions.into_iter().next()
                .expect("Invariant violated: conditions.len() == 1 but iterator empty")
        } else {
            RuleCondition::and(self.conditions)
        }
    }

    /// Build as OR condition
    /// Builds or
    /// Builds or
    #[must_use]
    pub fn build_or(self) -> RuleCondition {
        if self.conditions.len() == 1 {
            self.conditions.into_iter().next()
                .expect("Invariant violated: conditions.len() == 1 but iterator empty")
        } else {
            RuleCondition::or(self.conditions)
        }
    }
}

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

        event_data.insert("message".to_string(), "Success message".to_string());
        assert!(!condition.evaluate(&event_data));
    }

    #[test]
    fn test_field_exists() {
        let condition = RuleCondition::field_exists("user_id");

        let mut event_data = HashMap::new();
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
        ]);

        let mut event_data = HashMap::new();
        event_data.insert("event_type".to_string(), "login".to_string());
        event_data.insert("user_id".to_string(), "admin".to_string());

        assert!(condition.evaluate(&event_data));

        event_data.remove("user_id");
        assert!(!condition.evaluate(&event_data));
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

        event_data.insert("status".to_string(), "success ".to_string());
        assert!(!condition.evaluate(&event_data));
    }

    #[test]
    fn test_complexity_score() {
        let simple = RuleCondition::field_equals("field", "value");
        assert_eq!(simple.complexity_score(), 1);

        let complex = RuleCondition::and(vec![
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
            RuleCondition::field_equals("event_type", "login"),
            RuleCondition::field_exists("user_id"),
            RuleCondition::field_contains("message", "success "),
        ]);

        let fields = condition.referenced_fields();
        assert!(fields.contains(&"event_type".to_string()));
        assert!(fields.contains(&"user_id".to_string()));
        assert!(fields.contains(&"message".to_string()));
    }

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
}
