//! Rule condition types and evaluation logic
//!
//! This module contains the `RuleCondition` enum and related types for defining
//! conditions that can be evaluated to determine if a threat is present.

use serde::{Deserialize, Serialize};

/// Rule condition enumeration
///
/// Defines different types of conditions that can be
/// evaluated to determine if a threat is present.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    /// Simple field matching
    FieldEquals {
        /// Field name to match against
        field: String,
        /// Value to match
        value: String,
    },

    /// Field contains substring
    FieldContains {
        /// Field name to check
        field: String,
        /// Substring to look for
        value: String,
    },

    /// Field matches regex pattern
    FieldRegex {
        /// Field name to match against
        field: String,
        /// Regular expression pattern
        pattern: String,
    },

    /// Field value greater than threshold
    FieldGreaterThan {
        /// Field name to compare
        field: String,
        /// Threshold value
        value: String,
    },

    /// Field value less than threshold
    FieldLessThan {
        /// Field name to compare
        field: String,
        /// Threshold value
        value: String,
    },

    /// Pattern matching
    PatternMatch {
        /// Field name to match against
        field: String,
        /// Pattern to match
        pattern: String,
    },

    /// Threshold detection
    Threshold {
        /// Field name to evaluate
        field: String,
        /// Comparison operator
        operator: String,
        /// Threshold value
        value: f64,
    },

    /// Complex boolean logic
    Complex {
        /// Boolean expression
        expression: String,
    },

    /// Machine learning prediction
    MlPrediction {
        /// Model identifier
        model: String,
        /// Confidence threshold
        confidence: f64,
    },

    /// Event type matching
    EventType {
        /// Event type to match
        event_type: String,
    },

    /// Source IP matching
    SourceIp {
        /// IP address to match
        ip_address: String,
    },

    /// User agent matching
    UserAgent {
        /// User agent string to match
        user_agent: String,
    },

    /// Data size threshold
    DataSize {
        /// Size threshold in bytes
        size_bytes: u64,
    },

    /// Time range condition
    TimeRange {
        /// Start hour (0-23)
        start_hour: u8,
        /// End hour (0-23)
        end_hour: u8,
    },

    /// Frequency threshold
    FrequencyThreshold {
        /// Count threshold
        count: u64,
        /// Time window in minutes
        window_minutes: u64,
    },

    /// Logical AND of multiple conditions
    And {
        /// List of conditions to combine with AND
        conditions: Vec<RuleCondition>,
    },

    /// Logical OR of multiple conditions
    Or {
        /// List of conditions to combine with OR
        conditions: Vec<RuleCondition>,
    },

    /// Logical NOT of a condition
    Not {
        /// Condition to negate
        condition: Box<RuleCondition>,
    },
}

impl RuleCondition {
    /// Create a simple field equals condition
    ///
    /// # Arguments
    /// * `field` - Field name to match against
    /// * `value` - Value to match
    ///
    /// # Returns
    /// New field equals condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::field_equals("event_type", "login");
    /// ```
    pub fn field_equals(field: &str, value: &str) -> Self {
        Self::FieldEquals {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    /// Create a field contains condition
    ///
    /// # Arguments
    /// * `field` - Field name to check
    /// * `value` - Substring to look for
    ///
    /// # Returns
    /// New field contains condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::field_contains("message", "error");
    /// ```
    pub fn field_contains(field: &str, value: &str) -> Self {
        Self::FieldContains {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    /// Create a field regex condition
    ///
    /// # Arguments
    /// * `field` - Field name to match against
    /// * `pattern` - Regular expression pattern
    ///
    /// # Returns
    /// New field regex condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::field_regex("ip_address", r"^192\.168\.");
    /// ```
    pub fn field_regex(field: &str, pattern: &str) -> Self {
        Self::FieldRegex {
            field: field.to_string(),
            pattern: pattern.to_string(),
        }
    }

    /// Create a field greater than condition
    ///
    /// # Arguments
    /// * `field` - Field name to compare
    /// * `value` - Threshold value
    ///
    /// # Returns
    /// New field greater than condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::field_greater_than("failed_attempts", "5");
    /// ```
    pub fn field_greater_than(field: &str, value: &str) -> Self {
        Self::FieldGreaterThan {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    /// Create a field less than condition
    ///
    /// # Arguments
    /// * `field` - Field name to compare
    /// * `value` - Threshold value
    ///
    /// # Returns
    /// New field less than condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::field_less_than("response_time", "100");
    /// ```
    pub fn field_less_than(field: &str, value: &str) -> Self {
        Self::FieldLessThan {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    /// Create a threshold condition
    ///
    /// # Arguments
    /// * `field` - Field name to evaluate
    /// * `operator` - Comparison operator
    /// * `value` - Threshold value
    ///
    /// # Returns
    /// New threshold condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::threshold("cpu_usage", ">", 80.0);
    /// ```
    pub fn threshold(field: &str, operator: &str, value: f64) -> Self {
        Self::Threshold {
            field: field.to_string(),
            operator: operator.to_string(),
            value,
        }
    }

    /// Create a frequency threshold condition
    ///
    /// # Arguments
    /// * `count` - Count threshold
    /// * `window_minutes` - Time window in minutes
    ///
    /// # Returns
    /// New frequency threshold condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::frequency_threshold(10, 5);
    /// ```
    pub fn frequency_threshold(count: u64, window_minutes: u64) -> Self {
        Self::FrequencyThreshold {
            count,
            window_minutes,
        }
    }

    /// Create an event type condition
    ///
    /// # Arguments
    /// * `event_type` - Event type to match
    ///
    /// # Returns
    /// New event type condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::event_type("login_failed");
    /// ```
    pub fn event_type(event_type: &str) -> Self {
        Self::EventType {
            event_type: event_type.to_string(),
        }
    }

    /// Create a source IP condition
    ///
    /// # Arguments
    /// * `ip_address` - IP address to match
    ///
    /// # Returns
    /// New source IP condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::source_ip("192.168.1.100");
    /// ```
    pub fn source_ip(ip_address: &str) -> Self {
        Self::SourceIp {
            ip_address: ip_address.to_string(),
        }
    }

    /// Create a user agent condition
    ///
    /// # Arguments
    /// * `user_agent` - User agent string to match
    ///
    /// # Returns
    /// New user agent condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::user_agent("Mozilla/5.0");
    /// ```
    pub fn user_agent(user_agent: &str) -> Self {
        Self::UserAgent {
            user_agent: user_agent.to_string(),
        }
    }

    /// Create a data size condition
    ///
    /// # Arguments
    /// * `size_bytes` - Size threshold in bytes
    ///
    /// # Returns
    /// New data size condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::data_size(1024 * 1024); // 1MB
    /// ```
    pub fn data_size(size_bytes: u64) -> Self {
        Self::DataSize { size_bytes }
    }

    /// Create a time range condition
    ///
    /// # Arguments
    /// * `start_hour` - Start hour (0-23)
    /// * `end_hour` - End hour (0-23)
    ///
    /// # Returns
    /// New time range condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::time_range(22, 6); // 10 PM to 6 AM
    /// ```
    pub fn time_range(start_hour: u8, end_hour: u8) -> Self {
        Self::TimeRange {
            start_hour,
            end_hour,
        }
    }

    /// Create a machine learning prediction condition
    ///
    /// # Arguments
    /// * `model` - Model identifier
    /// * `confidence` - Confidence threshold
    ///
    /// # Returns
    /// New ML prediction condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::ml_prediction("anomaly_detector", 0.8);
    /// ```
    pub fn ml_prediction(model: &str, confidence: f64) -> Self {
        Self::MlPrediction {
            model: model.to_string(),
            confidence,
        }
    }

    /// Create a complex boolean condition
    ///
    /// # Arguments
    /// * `expression` - Boolean expression
    ///
    /// # Returns
    /// New complex condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::complex("(field1 == 'value1' AND field2 > 10) OR field3 == 'value3'");
    /// ```
    pub fn complex(expression: &str) -> Self {
        Self::Complex {
            expression: expression.to_string(),
        }
    }

    /// Create an AND condition
    ///
    /// # Arguments
    /// * `conditions` - List of conditions to combine with AND
    ///
    /// # Returns
    /// New AND condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition1 = RuleCondition::field_equals("event_type", "login");
    /// let condition2 = RuleCondition::field_equals("result", "failed");
    /// let and_condition = RuleCondition::and(vec![condition1, condition2]);
    /// ```
    pub fn and(conditions: Vec<RuleCondition>) -> Self {
        Self::And { conditions }
    }

    /// Create an OR condition
    ///
    /// # Arguments
    /// * `conditions` - List of conditions to combine with OR
    ///
    /// # Returns
    /// New OR condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition1 = RuleCondition::field_equals("event_type", "login");
    /// let condition2 = RuleCondition::field_equals("event_type", "logout");
    /// let or_condition = RuleCondition::or(vec![condition1, condition2]);
    /// ```
    pub fn or(conditions: Vec<RuleCondition>) -> Self {
        Self::Or { conditions }
    }

    /// Create a NOT condition
    ///
    /// # Arguments
    /// * `condition` - Condition to negate
    ///
    /// # Returns
    /// New NOT condition
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::field_equals("event_type", "login");
    /// let not_condition = RuleCondition::not(condition);
    /// ```
    pub fn negate(condition: RuleCondition) -> Self {
        Self::Not {
            condition: Box::new(condition),
        }
    }

    /// Check if condition is a simple field comparison
    ///
    /// # Returns
    /// `true` if condition is a simple field comparison
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let simple = RuleCondition::field_equals("event_type", "login");
    /// let complex = RuleCondition::and(vec![simple.clone()]);
    ///
    /// assert!(simple.is_simple());
    /// assert!(!complex.is_simple());
    /// ```
    pub fn is_simple(&self) -> bool {
        matches!(
            self,
            RuleCondition::FieldEquals { .. }
                | RuleCondition::FieldContains { .. }
                | RuleCondition::FieldRegex { .. }
                | RuleCondition::FieldGreaterThan { .. }
                | RuleCondition::FieldLessThan { .. }
                | RuleCondition::PatternMatch { .. }
                | RuleCondition::Threshold { .. }
                | RuleCondition::EventType { .. }
                | RuleCondition::SourceIp { .. }
                | RuleCondition::UserAgent { .. }
                | RuleCondition::DataSize { .. }
                | RuleCondition::TimeRange { .. }
                | RuleCondition::FrequencyThreshold { .. }
        )
    }

    /// Check if condition is complex (uses logical operators or ML)
    ///
    /// # Returns
    /// `true` if condition is complex
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let simple = RuleCondition::field_equals("event_type", "login");
    /// let complex = RuleCondition::and(vec![simple.clone()]);
    ///
    /// assert!(!simple.is_complex());
    /// assert!(complex.is_complex());
    /// ```
    pub fn is_complex(&self) -> bool {
        matches!(
            self,
            RuleCondition::And { .. }
                | RuleCondition::Or { .. }
                | RuleCondition::Not { .. }
                | RuleCondition::Complex { .. }
        )
    }

    /// Get condition complexity score
    ///
    /// # Returns
    /// Complexity score (higher values indicate more complex conditions)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let simple = RuleCondition::field_equals("event_type", "login");
    /// let complex = RuleCondition::and(vec![simple.clone(), simple.clone()]);
    ///
    /// assert!(complex.complexity_score() > simple.complexity_score());
    /// ```
    pub fn complexity_score(&self) -> u32 {
        match self {
            RuleCondition::FieldEquals { .. }
            | RuleCondition::FieldContains { .. }
            | RuleCondition::FieldRegex { .. }
            | RuleCondition::FieldGreaterThan { .. }
            | RuleCondition::FieldLessThan { .. }
            | RuleCondition::PatternMatch { .. }
            | RuleCondition::Threshold { .. }
            | RuleCondition::EventType { .. }
            | RuleCondition::SourceIp { .. }
            | RuleCondition::UserAgent { .. }
            | RuleCondition::DataSize { .. }
            | RuleCondition::TimeRange { .. }
            | RuleCondition::FrequencyThreshold { .. } => 1,

            RuleCondition::MlPrediction { .. } => 2,
            RuleCondition::Complex { .. } => 3,

            RuleCondition::And { conditions } | RuleCondition::Or { conditions } => {
                1 + conditions.iter().map(|c| c.complexity_score()).sum::<u32>()
            }

            RuleCondition::Not { condition } => 1 + condition.complexity_score(),
        }
    }

    /// Get all field names referenced in the condition
    ///
    /// # Returns
    /// Vector of field names
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let condition = RuleCondition::and(vec![
    ///     RuleCondition::field_equals("event_type", "login"),
    ///     RuleCondition::field_equals("result", "failed"),
    /// ]);
    ///
    /// let fields = condition.get_referenced_fields();
    /// assert!(fields.contains(&"event_type".to_string()));
    /// assert!(fields.contains(&"result".to_string()));
    /// ```
    pub fn get_referenced_fields(&self) -> Vec<String> {
        let mut fields = Vec::new();
        self.collect_fields(&mut fields);
        fields.sort();
        fields.dedup();
        fields
    }

    /// Collect field names recursively
    fn collect_fields(&self, fields: &mut Vec<String>) {
        match self {
            RuleCondition::FieldEquals { field, .. }
            | RuleCondition::FieldContains { field, .. }
            | RuleCondition::FieldRegex { field, .. }
            | RuleCondition::FieldGreaterThan { field, .. }
            | RuleCondition::FieldLessThan { field, .. }
            | RuleCondition::PatternMatch { field, .. }
            | RuleCondition::Threshold { field, .. } => {
                fields.push(field.clone());
            }

            RuleCondition::And { conditions } | RuleCondition::Or { conditions } => {
                for condition in conditions {
                    condition.collect_fields(fields);
                }
            }

            RuleCondition::Not { condition } => {
                condition.collect_fields(fields);
            }

            // These conditions don't reference specific fields
            RuleCondition::Complex { .. }
            | RuleCondition::MlPrediction { .. }
            | RuleCondition::EventType { .. }
            | RuleCondition::SourceIp { .. }
            | RuleCondition::UserAgent { .. }
            | RuleCondition::DataSize { .. }
            | RuleCondition::TimeRange { .. }
            | RuleCondition::FrequencyThreshold { .. } => {}
        }
    }

    /// Check if condition uses machine learning
    ///
    /// # Returns
    /// `true` if condition uses ML prediction
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let ml_condition = RuleCondition::ml_prediction("anomaly_detector", 0.8);
    /// let simple_condition = RuleCondition::field_equals("event_type", "login");
    ///
    /// assert!(ml_condition.uses_ml());
    /// assert!(!simple_condition.uses_ml());
    /// ```
    pub fn uses_ml(&self) -> bool {
        match self {
            RuleCondition::MlPrediction { .. } => true,
            RuleCondition::And { conditions } | RuleCondition::Or { conditions } => {
                conditions.iter().any(|c| c.uses_ml())
            }
            RuleCondition::Not { condition } => condition.uses_ml(),
            _ => false,
        }
    }

    /// Check if condition uses regex
    ///
    /// # Returns
    /// `true` if condition uses regex patterns
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let regex_condition = RuleCondition::field_regex("ip_address", r"^192\.168\.");
    /// let simple_condition = RuleCondition::field_equals("event_type", "login");
    ///
    /// assert!(regex_condition.uses_regex());
    /// assert!(!simple_condition.uses_regex());
    /// ```
    pub fn uses_regex(&self) -> bool {
        match self {
            RuleCondition::FieldRegex { .. } | RuleCondition::PatternMatch { .. } => true,
            RuleCondition::And { conditions } | RuleCondition::Or { conditions } => {
                conditions.iter().any(|c| c.uses_regex())
            }
            RuleCondition::Not { condition } => condition.uses_regex(),
            _ => false,
        }
    }

    /// Check if condition uses time-based logic
    ///
    /// # Returns
    /// `true` if condition uses time-based logic
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let time_condition = RuleCondition::time_range(22, 6);
    /// let freq_condition = RuleCondition::frequency_threshold(10, 5);
    /// let simple_condition = RuleCondition::field_equals("event_type", "login");
    ///
    /// assert!(time_condition.uses_time());
    /// assert!(freq_condition.uses_time());
    /// assert!(!simple_condition.uses_time());
    /// ```
    pub fn uses_time(&self) -> bool {
        match self {
            RuleCondition::TimeRange { .. } | RuleCondition::FrequencyThreshold { .. } => true,
            RuleCondition::And { conditions } | RuleCondition::Or { conditions } => {
                conditions.iter().any(|c| c.uses_time())
            }
            RuleCondition::Not { condition } => condition.uses_time(),
            _ => false,
        }
    }

    /// Validate condition structure
    ///
    /// # Returns
    /// `true` if condition is valid
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    ///
    /// let valid_condition = RuleCondition::field_equals("event_type", "login");
    /// let invalid_condition = RuleCondition::time_range(25, 6); // Invalid hour
    ///
    /// assert!(valid_condition.is_valid());
    /// assert!(!invalid_condition.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        match self {
            RuleCondition::FieldEquals { field, value }
            | RuleCondition::FieldContains { field, value }
            | RuleCondition::FieldGreaterThan { field, value }
            | RuleCondition::FieldLessThan { field, value } => {
                !field.is_empty() && !value.is_empty()
            }

            RuleCondition::FieldRegex { field, pattern }
            | RuleCondition::PatternMatch { field, pattern } => {
                !field.is_empty() && !pattern.is_empty()
            }

            RuleCondition::Threshold {
                field, operator, ..
            } => !field.is_empty() && !operator.is_empty(),

            RuleCondition::Complex { expression } => !expression.is_empty(),

            RuleCondition::MlPrediction { model, confidence } => {
                !model.is_empty() && *confidence >= 0.0 && *confidence <= 1.0
            }

            RuleCondition::EventType { event_type } => !event_type.is_empty(),

            RuleCondition::SourceIp { ip_address } => !ip_address.is_empty(),

            RuleCondition::UserAgent { user_agent } => !user_agent.is_empty(),

            RuleCondition::DataSize { size_bytes: _ } => true,

            RuleCondition::TimeRange {
                start_hour,
                end_hour,
            } => *start_hour <= 23 && *end_hour <= 23,

            RuleCondition::FrequencyThreshold {
                count,
                window_minutes,
            } => *count > 0 && *window_minutes > 0,

            RuleCondition::And { conditions } | RuleCondition::Or { conditions } => {
                !conditions.is_empty() && conditions.iter().all(|c| c.is_valid())
            }

            RuleCondition::Not { condition } => condition.is_valid(),
        }
    }
}
