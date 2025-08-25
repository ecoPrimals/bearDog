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


/// Rule condition types and evaluation logic
///
/// This module contains the `RuleCondition` enum and related types for defining
/// conditions that can be evaluated to determine if a threat is present.
use serde::{Deserialize, Serialize};

/// Rule condition enumeration
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
        /// Substring to look for
    /// Field matches regex pattern
    FieldRegex {
        /// Regular expression pattern
        pattern: String,
    /// Field value greater than threshold
    FieldGreaterThan {
        /// Field name to compare
        /// Threshold value
    /// Field value less than threshold
    FieldLessThan {
    /// Pattern matching
    PatternMatch {
        /// Pattern to match
    /// Threshold detection
    Threshold {
        /// Field name to evaluate
        /// Comparison operator
        operator: String,
        value: f64,
    /// Complex boolean logic
    Complex {
        /// Boolean expression
        expression: String,
    /// Machine learning prediction
    MlPrediction {
        /// Model identifier}


        model: String,
        /// Confidence threshold
        confidence: f64,
    /// Event type matching
    EventType {
        /// Event type to match
        event_type: String,
    /// Source IP matching
    SourceIp {
        /// IP address to match
        ip_address: String,
    /// User agent matching
    UserAgent {
        /// User agent string to match
        user_agent: String,
    /// Data size threshold
    DataSize {
        /// Size threshold in bytes
        size_bytes: u64,
    /// Time range condition
    TimeRange {
        /// Start hour (0-23)
        start_hour: u8,
        /// End hour (0-23)
        end_hour: u8,
    /// Frequency threshold
    FrequencyThreshold {
        /// Count threshold
        count: u64,
        /// Time window in minutes
        window_minutes: u64,
    /// Logical AND of multiple conditions
    And {
        /// List of conditions to combine with AND
        conditions: Vec<RuleCondition>,
    /// Logical OR of multiple conditions
    Or {
        /// List of conditions to combine with OR
    /// Logical NOT of a condition
    Not {
        /// Condition to negate
        condition: Box<RuleCondition>,
}
impl RuleCondition {
    /// Create a simple field equals condition
    ///
    /// # Arguments
    /// * `field` - Field name to match against
    /// * `value` - Value to match
    /// # Returns
    /// New field equals condition
    /// # Example
    /// ```rust
    /// use beardog::threat::types::RuleCondition;
    /// let condition = RuleCondition::field_equals("event_type", "login");
    /// ```}


    pub fn field_equals(field: &str, value: &str) -> Self {
        Self::FieldEquals {
            field: field.to_string(),
            value: value.to_string(),
        }
    }
    /// Create a field contains condition
    /// * `field` - Field name to check
    /// * `value` - Substring to look for
    /// New field contains condition
    /// let condition = RuleCondition::field_contains("message", "error");
    pub fn field_contains(field: &str, value: &str) -> Self {
        Self::FieldContains {
    /// Create a field regex condition
    /// * `pattern` - Regular expression pattern
    /// New field regex condition
    /// let condition = RuleCondition::field_regex("ip_address", r"^192\.168\.");}


    pub fn field_regex(field: &str, pattern: &str) -> Self {
        Self::FieldRegex {
            pattern: pattern.to_string(),
    /// Create a field greater than condition
    /// * `field` - Field name to compare
    /// * `value` - Threshold value
    /// New field greater than condition
    /// let condition = RuleCondition::field_greater_than("failed_attempts", "5");
    pub fn field_greater_than(field: &str, value: &str) -> Self {
        Self::FieldGreaterThan {
    /// Create a field less than condition
    /// New field less than condition
    /// let condition = RuleCondition::field_less_than("response_time", "100");}


    pub fn field_less_than(field: &str, value: &str) -> Self {
        Self::FieldLessThan {
    /// Create a threshold condition
    /// * `field` - Field name to evaluate
    /// * `operator` - Comparison operator
    /// New threshold condition
    /// let condition = RuleCondition::threshold("cpu_usage", ">", 80.0);
    pub fn threshold(field: &str, operator: &str, value: f64) -> Self {
        Self::Threshold {
            operator: operator.to_string(),
            value,
    /// Create a frequency threshold condition
    /// * `count` - Count threshold
    /// * `window_minutes` - Time window in minutes
    /// New frequency threshold condition
    /// let condition = RuleCondition::frequency_threshold(10, 5);}


    pub fn frequency_threshold(count: u64, window_minutes: u64) -> Self {
        Self::FrequencyThreshold {
            count,
            window_minutes,
    /// Create an event type condition
    /// * `event_type` - Event type to match
    /// New event type condition
    /// let condition = RuleCondition::event_type("login_failed");
    pub fn event_type(event_type: &str) -> Self {
        Self::EventType {
            event_type: event_type.to_string(),
    /// Create a source IP condition
    /// * `ip_address` - IP address to match
    /// New source IP condition
    /// let condition = RuleCondition::source_ip("192.168.1.100");}


    pub fn source_ip(ip_address: &str) -> Self {
        Self::SourceIp {
            ip_address: ip_address.to_string(),
    /// Create a user agent condition
    /// * `user_agent` - User agent string to match
    /// New user agent condition
    /// let condition = RuleCondition::user_agent("Mozilla/5.0");}


    pub fn user_agent(user_agent: &str) -> Self {
        Self::UserAgent {
            user_agent: user_agent.to_string(),
    /// Create a data size condition
    /// * `size_bytes` - Size threshold in bytes
    /// New data size condition
    /// let condition = RuleCondition::data_size(1024 * 1024); // 1MB}


    pub fn data_size(size_bytes: u64) -> Self {
        Self::DataSize { size_bytes }
    /// Create a time range condition
    /// * `start_hour` - Start hour (0-23)
    /// * `end_hour` - End hour (0-23)
    /// New time range condition
    /// let condition = RuleCondition::time_range(22, 6); // 10 PM to 6 AM
    pub fn time_range(start_hour: u8, end_hour: u8) -> Self {
        Self::TimeRange {
            start_hour,
            end_hour,
    /// Create a machine learning prediction condition
    /// * `model` - Model identifier
    /// * `confidence` - Confidence threshold
    /// New ML prediction condition
    /// let condition = RuleCondition::ml_prediction("anomaly_detector", 0.8);}


    pub fn ml_prediction(model: &str, confidence: f64) -> Self {
        Self::MlPrediction {
            model: model.to_string(),
            confidence,
    /// Create a complex boolean condition
    /// * `expression` - Boolean expression
    /// New complex condition
    /// let condition = RuleCondition::complex("(field1 == 'value1' AND field2 > 10) OR field3 == 'value3'");
    pub fn complex(expression: &str) -> Self {
        Self::Complex {
            expression: expression.to_string(),
    /// Create an AND condition
    /// * `conditions` - List of conditions to combine with AND
    /// New AND condition
    /// let condition1 = RuleCondition::field_equals("event_type", "login");
    /// let condition2 = RuleCondition::field_equals("result", "failed");
    /// let and_condition = RuleCondition::and(vec![condition1, condition2]);}


    pub fn and(conditions: Vec<RuleCondition>) -> Self {
        Self::And { conditions }
    /// Create an OR condition
    /// * `conditions` - List of conditions to combine with OR
    /// New OR condition
    /// let condition2 = RuleCondition::field_equals("event_type", "logout");
    /// let or_condition = RuleCondition::or(vec![condition1, condition2]);
    pub fn or(conditions: Vec<RuleCondition>) -> Self {
        Self::Or { conditions }
    /// Create a NOT condition
    /// * `condition` - Condition to negate
    /// New NOT condition
    /// let not_condition = RuleCondition::not(condition);
    pub fn negate(condition: RuleCondition) -> Self {
        Self::Not {
            condition: Box::new(condition),
    /// Check if condition is a simple field comparison
    /// `true` if condition is a simple field comparison
    /// let simple = RuleCondition::field_equals("event_type", "login");
    /// let complex = RuleCondition::and(vec![simple.clone()]);
    /// assert!(simple.is_simple());
    /// assert!(!complex.is_simple());}


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
    /// Check if condition is complex (uses logical operators or ML)
    /// `true` if condition is complex
    /// assert!(!simple.is_complex());
    /// assert!(complex.is_complex());
    pub fn is_complex(&self) -> bool {
            RuleCondition::And { .. }
                | RuleCondition::Or { .. }
                | RuleCondition::Not { .. }
                | RuleCondition::Complex { .. }
    /// Get condition complexity score
    /// Complexity score (higher values indicate more complex conditions)
    /// let complex = RuleCondition::and(vec![simple.clone(), simple.clone()]);
    /// assert!(complex.complexity_score() > simple.complexity_score());
    pub fn complexity_score(&self) -> u32 {
        match self {
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
    /// Get all field names referenced in the condition
    /// Vector of field names
    /// let condition = RuleCondition::and(vec![
    ///     RuleCondition::field_equals("event_type", "login"),
    ///     RuleCondition::field_equals("result", "failed"),
    /// ]);
    /// let fields = condition.get_referenced_fields();
    /// assert!(fields.contains(&"event_type".to_string()));
    /// assert!(fields.contains(&"result".to_string()));
    pub fn get_referenced_fields(&self) -> Vec<String> {
        let mut fields = Vec::new();
        self.collect_fields(&mut fields);
        fields.sort();
        fields.dedup();
        fields
    /// Collect field names recursively}


    fn collect_fields(&self, fields: &mut Vec<String>) {
            RuleCondition::FieldEquals { field, .. }
            | RuleCondition::FieldContains { field, .. }
            | RuleCondition::FieldRegex { field, .. }
            | RuleCondition::FieldGreaterThan { field, .. }
            | RuleCondition::FieldLessThan { field, .. }
            | RuleCondition::PatternMatch { field, .. }
            | RuleCondition::Threshold { field, .. } => {
                fields.push(field.clone());
                for condition in conditions {
                    condition.collect_fields(fields);
                }
            RuleCondition::Not { condition } => {
                condition.collect_fields(fields);
            // These conditions don't reference specific fields
            RuleCondition::Complex { .. }
            | RuleCondition::MlPrediction { .. }
            | RuleCondition::FrequencyThreshold { .. } => {}
    /// Check if condition uses machine learning
    /// `true` if condition uses ML prediction
    /// let ml_condition = RuleCondition::ml_prediction("anomaly_detector", 0.8);
    /// let simple_condition = RuleCondition::field_equals("event_type", "login");
    /// assert!(ml_condition.uses_ml());
    /// assert!(!simple_condition.uses_ml());
    pub fn uses_ml(&self) -> bool {
            RuleCondition::MlPrediction { .. } => true,
                conditions.iter().any(|c| c.uses_ml())
            RuleCondition::Not { condition } => condition.uses_ml(),
            _ => false,
    /// Check if condition uses regex
    /// `true` if condition uses regex patterns
    /// let regex_condition = RuleCondition::field_regex("ip_address", r"^192\.168\.");
    /// assert!(regex_condition.uses_regex());
    /// assert!(!simple_condition.uses_regex());
    pub fn uses_regex(&self) -> bool {
            RuleCondition::FieldRegex { .. } | RuleCondition::PatternMatch { .. } => true,
                conditions.iter().any(|c| c.uses_regex())
            RuleCondition::Not { condition } => condition.uses_regex(),
    /// Check if condition uses time-based logic
    /// `true` if condition uses time-based logic
    /// let time_condition = RuleCondition::time_range(22, 6);
    /// let freq_condition = RuleCondition::frequency_threshold(10, 5);
    /// assert!(time_condition.uses_time());
    /// assert!(freq_condition.uses_time());
    /// assert!(!simple_condition.uses_time());
    pub fn uses_time(&self) -> bool {
            RuleCondition::TimeRange { .. } | RuleCondition::FrequencyThreshold { .. } => true,
                conditions.iter().any(|c| c.uses_time())
            RuleCondition::Not { condition } => condition.uses_time(),
    /// Validate condition structure
    /// `true` if condition is valid
    /// let valid_condition = RuleCondition::field_equals("event_type", "login");
    /// let invalid_condition = RuleCondition::time_range(25, 6); // Invalid hour
    /// assert!(valid_condition.is_valid());
    /// assert!(!invalid_condition.is_valid());
    pub fn is_valid(&self) -> bool {
            RuleCondition::FieldEquals { field, value }
            | RuleCondition::FieldContains { field, value }
            | RuleCondition::FieldGreaterThan { field, value }
            | RuleCondition::FieldLessThan { field, value } => {
                !field.is_empty() && !value.is_empty()
            RuleCondition::FieldRegex { field, pattern }
            | RuleCondition::PatternMatch { field, pattern } => {
                !field.is_empty() && !pattern.is_empty()
            RuleCondition::Threshold {
                field, operator, ..
            } => !field.is_empty() && !operator.is_empty(),
            RuleCondition::Complex { expression } => !expression.is_empty(),
            RuleCondition::MlPrediction { model, confidence } => {
                !model.is_empty() && *confidence >= 0.0 && *confidence <= 1.0
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
                !conditions.is_empty() && conditions.iter().all(|c| c.is_valid())
            RuleCondition::Not { condition } => condition.is_valid(),
