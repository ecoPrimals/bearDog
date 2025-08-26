

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuleCondition {
    Always,
    Never,
    FieldEquals { field: String, value: String },
    FieldContains { field: String, value: String },
    FieldMatches { field: String, pattern: String },
    FieldRegex { pattern: String },
    FieldGreaterThan { field: String, value: String }, // Changed to String for comparison
    FieldLessThan { field: String, value: String },    // Changed to String for comparison
    LogicalAnd(Vec<RuleCondition>),
    LogicalOr(Vec<RuleCondition>),
    LogicalNot(Box<RuleCondition>),
    And { conditions: Vec<RuleCondition> },
    Or { conditions: Vec<RuleCondition> },
    Not { condition: Box<RuleCondition> },
}

impl RuleCondition {

    pub fn field_equals(field: &str, value: &str) -> Self {
        Self::FieldEquals {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    pub fn field_contains(field: &str, value: &str) -> Self {
        Self::FieldContains {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    pub fn field_regex(pattern: &str) -> Self {
        Self::FieldRegex {
            pattern: pattern.to_string(),
        }
    }

    pub fn field_greater_than(field: &str, value: f64) -> Self {
        Self::FieldGreaterThan {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    pub fn field_less_than(field: &str, value: f64) -> Self {
        Self::FieldLessThan {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    pub fn and(conditions: Vec<RuleCondition>) -> Self {
        Self::And { conditions }
    }

    pub fn or(conditions: Vec<RuleCondition>) -> Self {
        Self::Or { conditions }
    }

    pub fn not(condition: RuleCondition) -> Self {
        Self::Not {
            condition: Box::new(condition),
        }
    }

    pub fn uses_ml(&self) -> bool {
        match self {
            Self::And { conditions } | Self::Or { conditions } => {
                conditions.iter().any(|c| c.uses_ml())
            }
            Self::Not { condition } => condition.uses_ml(),
            _ => false, // Simplified - no ML conditions in this version
        }
    }

    pub fn evaluate(&self, data: &std::collections::HashMap<&str, &str>) -> bool {
        match self {
            Self::FieldEquals { field, value } => {
                data.get(field.as_str()).map_or(false, |v| v == value)
            }
            Self::FieldContains { field, value } => {
                data.get(field.as_str()).map_or(false, |v| v.contains(value))
            }
            Self::FieldMatches { field, pattern } => {
                // Simple pattern matching - in production would use proper regex
                data.get(field.as_str())
                    .map_or(false, |v| v.contains(pattern))
            }
            Self::FieldRegex { pattern } => {

                data.values().any(|v| v.contains(pattern))
            }
            Self::FieldGreaterThan { field, value } => {
                data.get(field.as_str())
                    .and_then(|v| v.parse::<f64>().ok())
                    .map_or(false, |v| v > value.parse::<f64>().unwrap_or(0.0))
            }
            Self::FieldLessThan { field, value } => {
                data.get(field.as_str())
                    .and_then(|v| v.parse::<f64>().ok())
                    .map_or(false, |v| v < value.parse::<f64>().unwrap_or(0.0))
            }
            Self::And { conditions } => conditions.iter().all(|c| c.evaluate(data)),
            Self::Or { conditions } => conditions.iter().any(|c| c.evaluate(data)),
            Self::Not { condition } => !condition.evaluate(data),
            Self::Always => true,
            Self::Never => false,
            Self::LogicalAnd(conditions) => conditions.iter().all(|c| c.evaluate(data)),
            Self::LogicalOr(conditions) => conditions.iter().any(|c| c.evaluate(data)),
            Self::LogicalNot(condition) => !condition.evaluate(data),
        }
    }
}

impl Default for RuleCondition {
    fn default() -> Self {
        Self::FieldEquals {
            field: "default".to_string(),
            value: "true".to_string(),
        }
    }
}