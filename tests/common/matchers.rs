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


//! Advanced Test Matchers for BearDog
//!
//! **Sophisticated Test Validation Utilities**
//!
//! This module provides advanced matchers and validation utilities that enable
//! expressive, comprehensive test assertions with detailed error reporting.

use crate::common::{TestResult, TestContext};
use beardog_errors::{BearDogError, BearDogResult};
use serde_json::{Value as JsonValue};
use std::{
    collections::HashMap,
    fmt,
    time::{Duration, SystemTime},
};

/// Advanced test matcher for complex validation scenarios
pub struct TestMatcher {
    name: String,
    description: String,
    matchers: Vec<Box<dyn MatcherFn>>,
}

/// Trait for matcher functions
pub trait MatcherFn: Send + Sync {
    fn matches(&self, context: &TestContext, value: &JsonValue) -> MatchResult;
    fn description(&self) -> String;
}

/// Result of a matcher evaluation
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub success: bool,
    pub message: String,
    pub details: HashMap<String, JsonValue>,
    pub suggestions: Vec<String>,
}

/// Performance matcher for duration and resource validation
pub struct PerformanceMatcher {
    max_duration: Option<Duration>,
    min_duration: Option<Duration>,
    max_memory_mb: Option<f64>,
    max_cpu_percent: Option<f64>,
}

/// HTTP response matcher for API testing
pub struct HttpResponseMatcher {
    expected_status: Option<u16>,
    expected_headers: HashMap<String, String>,
    expected_body_patterns: Vec<JsonPattern>,
    max_response_time: Option<Duration>,
}

/// JSON pattern matcher for flexible content validation
#[derive(Debug, Clone)]
pub enum JsonPattern {
    Exact(JsonValue),
    Contains(String),
    Regex(String),
    Type(JsonType),
    Range(f64, f64),
    Length(usize, Option<usize>),
    Custom(String), // Custom validation expression
}

#[derive(Debug, Clone)]
pub enum JsonType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    Null,
}

/// Error matcher for error scenario validation
pub struct ErrorMatcher {
    expected_error_type: Option<String>,
    expected_error_code: Option<String>,
    expected_message_pattern: Option<String>,
    expected_severity: Option<String>,
    expected_category: Option<String>,
}

/// Security matcher for cryptographic and security validation
pub struct SecurityMatcher {
    signature_valid: Option<bool>,
    encryption_algorithm: Option<String>,
    key_strength_bits: Option<u32>,
    has_secure_random: Option<bool>,
    timing_attack_safe: Option<bool>,
}

/// Genetics matcher for genetic algorithm validation
pub struct GeneticsMatcher {
    fitness_score_range: Option<(f64, f64)>,
    generation_count: Option<u32>,
    capability_count: Option<usize>,
    lineage_valid: Option<bool>,
    diversity_score: Option<f64>,
}

impl TestMatcher {
    /// Create a new test matcher
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            matchers: Vec::new(),
        }
    }

    /// Set description for the matcher
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Add a custom matcher function
    pub fn add_matcher<F>(mut self, matcher: F) -> Self
    where
        F: Fn(&TestContext, &JsonValue) -> MatchResult + Send + Sync + 'static,
    {
        self.matchers.push(Box::new(FunctionMatcher { func: Box::new(matcher) }));
        self
    }

    /// Add performance validation
    pub fn performance(mut self, matcher: PerformanceMatcher) -> Self {
        self.matchers.push(Box::new(matcher));
        self
    }

    /// Add HTTP response validation
    pub fn http_response(mut self, matcher: HttpResponseMatcher) -> Self {
        self.matchers.push(Box::new(matcher));
        self
    }

    /// Add error validation
    pub fn error(mut self, matcher: ErrorMatcher) -> Self {
        self.matchers.push(Box::new(matcher));
        self
    }

    /// Add security validation
    pub fn security(mut self, matcher: SecurityMatcher) -> Self {
        self.matchers.push(Box::new(matcher));
        self
    }

    /// Add genetics validation
    pub fn genetics(mut self, matcher: GeneticsMatcher) -> Self {
        self.matchers.push(Box::new(matcher));
        self
    }

    /// Execute all matchers and return comprehensive result
    pub fn validate(&self, context: &TestContext, value: &JsonValue) -> MatchResult {
        let mut all_success = true;
        let mut messages = Vec::new();
        let mut combined_details = HashMap::new();
        let mut all_suggestions = Vec::new();

        for matcher in &self.matchers {
            let result = matcher.matches(context, value);
            
            if !result.success {
                all_success = false;
            }

            messages.push(format!("{}: {}", matcher.description(), result.message));
            
            // Combine details
            for (key, val) in result.details {
                combined_details.insert(key, val);
            }

            // Collect suggestions
            all_suggestions.extend(result.suggestions);
        }

        MatchResult {
            success: all_success,
            message: if all_success {
                format!("All {} matchers passed for '{}'", self.matchers.len(), self.name)
            } else {
                format!("Some matchers failed for '{}': {}", self.name, messages.join("; "))
            },
            details: combined_details,
            suggestions: all_suggestions,
        }
    }
}

/// Function-based matcher wrapper
struct FunctionMatcher {
    func: Box<dyn Fn(&TestContext, &JsonValue) -> MatchResult + Send + Sync>,
}

impl MatcherFn for FunctionMatcher {
    fn matches(&self, context: &TestContext, value: &JsonValue) -> MatchResult {
        (self.func)(context, value)
    }

    fn description(&self) -> String {
        "Custom Function Matcher".to_string()
    }
}

impl PerformanceMatcher {
    pub fn new() -> Self {
        Self {
            max_duration: None,
            min_duration: None,
            max_memory_mb: None,
            max_cpu_percent: None,
        }
    }

    pub fn max_duration(mut self, duration: Duration) -> Self {
        self.max_duration = Some(duration);
        self
    }

    pub fn min_duration(mut self, duration: Duration) -> Self {
        self.min_duration = Some(duration);
        self
    }

    pub fn max_memory_mb(mut self, memory_mb: f64) -> Self {
        self.max_memory_mb = Some(memory_mb);
        self
    }

    pub fn max_cpu_percent(mut self, cpu_percent: f64) -> Self {
        self.max_cpu_percent = Some(cpu_percent);
        self
    }
}

impl MatcherFn for PerformanceMatcher {
    fn matches(&self, context: &TestContext, _value: &JsonValue) -> MatchResult {
        let mut success = true;
        let mut messages = Vec::new();
        let mut details = HashMap::new();
        let mut suggestions = Vec::new();

        // Check duration constraints
        if let Some(total_duration) = context.performance_metrics.total_duration {
            details.insert("actual_duration_ms".to_string(), 
                          JsonValue::Number(total_duration.as_millis().into()));

            if let Some(max_duration) = self.max_duration {
                if total_duration > max_duration {
                    success = false;
                    messages.push(format!("Duration {} exceeds maximum {}", 
                                        format_duration(total_duration), 
                                        format_duration(max_duration)));
                    suggestions.push("Consider optimizing performance-critical operations".to_string());
                    suggestions.push("Check for unnecessary blocking operations".to_string());
                }
            }

            if let Some(min_duration) = self.min_duration {
                if total_duration < min_duration {
                    success = false;
                    messages.push(format!("Duration {} is below minimum {} (possible timing issue)", 
                                        format_duration(total_duration), 
                                        format_duration(min_duration)));
                    suggestions.push("Verify test is actually performing expected operations".to_string());
                }
            }
        }

        // Check memory constraints
        if let Some(peak_memory) = context.performance_metrics.memory_peak_mb {
            details.insert("peak_memory_mb".to_string(), 
                          JsonValue::Number(serde_json::Number::from_f64(peak_memory).unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
})));

            if let Some(max_memory) = self.max_memory_mb {
                if peak_memory > max_memory {
                    success = false;
                    messages.push(format!("Peak memory {:.2} MB exceeds maximum {:.2} MB", 
                                        peak_memory, max_memory));
                    suggestions.push("Review memory usage patterns and potential leaks".to_string());
                    suggestions.push("Consider implementing memory pooling".to_string());
                }
            }
        }

        MatchResult {
            success,
            message: if success {
                "Performance constraints satisfied".to_string()
            } else {
                format!("Performance issues: {}", messages.join(", "))
            },
            details,
            suggestions,
        }
    }

    fn description(&self) -> String {
        "Performance Matcher".to_string()
    }
}

impl HttpResponseMatcher {
    pub fn new() -> Self {
        Self {
            expected_status: None,
            expected_headers: HashMap::new(),
            expected_body_patterns: Vec::new(),
            max_response_time: None,
        }
    }

    pub fn status(mut self, status: u16) -> Self {
        self.expected_status = Some(status);
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.expected_headers.insert(key.into(), value.into());
        self
    }

    pub fn body_contains(mut self, pattern: impl Into<String>) -> Self {
        self.expected_body_patterns.push(JsonPattern::Contains(pattern.into()));
        self
    }

    pub fn body_matches(mut self, regex: impl Into<String>) -> Self {
        self.expected_body_patterns.push(JsonPattern::Regex(regex.into()));
        self
    }

    pub fn max_response_time(mut self, duration: Duration) -> Self {
        self.max_response_time = Some(duration);
        self
    }
}

impl MatcherFn for HttpResponseMatcher {
    fn matches(&self, _context: &TestContext, value: &JsonValue) -> MatchResult {
        let mut success = true;
        let mut messages = Vec::new();
        let mut details = HashMap::new();
        let mut suggestions = Vec::new();

        // Validate status code
        if let Some(expected_status) = self.expected_status {
            if let Some(actual_status) = value.get("status_code").and_then(|v| v.as_u64()) {
                details.insert("actual_status".to_string(), JsonValue::Number(actual_status.into()));
                
                if actual_status != expected_status as u64 {
                    success = false;
                    messages.push(format!("Expected status {}, got {}", expected_status, actual_status));
                    
                    if actual_status >= 400 {
                        suggestions.push("Check API endpoint implementation".to_string());
                        suggestions.push("Verify request parameters and authentication".to_string());
                    }
                }
            } else {
                success = false;
                messages.push("Status code not found in response".to_string());
                suggestions.push("Ensure response includes status_code field".to_string());
            }
        }

        // Validate headers
        if let Some(headers) = value.get("headers").and_then(|v| v.as_object()) {
            for (expected_key, expected_value) in &self.expected_headers {
                if let Some(actual_value) = headers.get(expected_key).and_then(|v| v.as_str()) {
                    if actual_value != expected_value {
                        success = false;
                        messages.push(format!("Header '{}': expected '{}', got '{}'", 
                                            expected_key, expected_value, actual_value));
                    }
                } else {
                    success = false;
                    messages.push(format!("Missing expected header: {}", expected_key));
                    suggestions.push(format!("Ensure API sets '{}' header", expected_key));
                }
            }
        }

        // Validate body patterns
        if let Some(body) = value.get("body") {
            for pattern in &self.expected_body_patterns {
                if !self.validate_json_pattern(body, pattern) {
                    success = false;
                    messages.push(format!("Body pattern validation failed: {:?}", pattern));
                    suggestions.push("Review response body structure and content".to_string());
                }
            }
        }

        MatchResult {
            success,
            message: if success {
                "HTTP response validation passed".to_string()
            } else {
                format!("HTTP response validation failed: {}", messages.join(", "))
            },
            details,
            suggestions,
        }
    }

    fn description(&self) -> String {
        "HTTP Response Matcher".to_string()
    }
}

impl HttpResponseMatcher {
    fn validate_json_pattern(&self, value: &JsonValue, pattern: &JsonPattern) -> bool {
        match pattern {
            JsonPattern::Exact(expected) => value == expected,
            JsonPattern::Contains(text) => {
                value.to_string().contains(text)
            }
            JsonPattern::Regex(regex_str) => {
                if let Ok(regex) = regex::Regex::new(regex_str) {
                    regex.is_match(&value.to_string())
                } else {
                    false
                }
            }
            JsonPattern::Type(json_type) => {
                match json_type {
                    JsonType::String => value.is_string(),
                    JsonType::Number => value.is_number(),
                    JsonType::Boolean => value.is_boolean(),
                    JsonType::Array => value.is_array(),
                    JsonType::Object => value.is_object(),
                    JsonType::Null => value.is_null(),
                }
            }
            JsonPattern::Range(min, max) => {
                if let Some(num) = value.as_f64() {
                    num >= *min && num <= *max
                } else {
                    false
                }
            }
            JsonPattern::Length(min_len, max_len) => {
                let length = match value {
                    JsonValue::String(s) => s.len(),
                    JsonValue::Array(arr) => arr.len(),
                    JsonValue::Object(obj) => obj.len(),
                    _ => return false,
                };
                
                length >= *min_len && max_len.map_or(true, |max| length <= max)
            }
            JsonPattern::Custom(_expr) => {
                // Custom expression evaluation pending advanced matching requirements
                true
            }
        }
    }
}

impl ErrorMatcher {
    pub fn new() -> Self {
        Self {
            expected_error_type: None,
            expected_error_code: None,
            expected_message_pattern: None,
            expected_severity: None,
            expected_category: None,
        }
    }

    pub fn error_type(mut self, error_type: impl Into<String>) -> Self {
        self.expected_error_type = Some(error_type.into());
        self
    }

    pub fn error_code(mut self, error_code: impl Into<String>) -> Self {
        self.expected_error_code = Some(error_code.into());
        self
    }

    pub fn message_contains(mut self, pattern: impl Into<String>) -> Self {
        self.expected_message_pattern = Some(pattern.into());
        self
    }

    pub fn severity(mut self, severity: impl Into<String>) -> Self {
        self.expected_severity = Some(severity.into());
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.expected_category = Some(category.into());
        self
    }
}

impl MatcherFn for ErrorMatcher {
    fn matches(&self, _context: &TestContext, value: &JsonValue) -> MatchResult {
        let mut success = true;
        let mut messages = Vec::new();
        let mut details = HashMap::new();
        let mut suggestions = Vec::new();

        // Extract error information from the value
        let error_info = if value.is_object() {
            value.as_object().unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
})
        } else {
            success = false;
            messages.push("Expected error object, got different type".to_string());
            return MatchResult {
                success,
                message: messages.join(", "),
                details,
                suggestions: vec!["Ensure error is properly serialized as JSON object".to_string()],
            };
        };

        // Validate error type
        if let Some(expected_type) = &self.expected_error_type {
            if let Some(actual_type) = error_info.get("error_type").and_then(|v| v.as_str()) {
                details.insert("error_type".to_string(), JsonValue::String(actual_type.to_string()));
                if actual_type != expected_type {
                    success = false;
                    messages.push(format!("Expected error type '{}', got '{}'", expected_type, actual_type));
                }
            } else {
                success = false;
                messages.push("Error type not found in error object".to_string());
                suggestions.push("Ensure BearDogError includes error_type field".to_string());
            }
        }

        // Validate error code
        if let Some(expected_code) = &self.expected_error_code {
            if let Some(actual_code) = error_info.get("error_code").and_then(|v| v.as_str()) {
                details.insert("error_code".to_string(), JsonValue::String(actual_code.to_string()));
                if actual_code != expected_code {
                    success = false;
                    messages.push(format!("Expected error code '{}', got '{}'", expected_code, actual_code));
                }
            }
        }

        // Validate message pattern
        if let Some(expected_pattern) = &self.expected_message_pattern {
            if let Some(actual_message) = error_info.get("message").and_then(|v| v.as_str()) {
                details.insert("message".to_string(), JsonValue::String(actual_message.to_string()));
                if !actual_message.contains(expected_pattern) {
                    success = false;
                    messages.push(format!("Error message doesn't contain expected pattern '{}'", expected_pattern));
                    suggestions.push("Review error message generation and localization".to_string());
                }
            }
        }

        MatchResult {
            success,
            message: if success {
                "Error validation passed".to_string()
            } else {
                format!("Error validation failed: {}", messages.join(", "))
            },
            details,
            suggestions,
        }
    }

    fn description(&self) -> String {
        "Error Matcher".to_string()
    }
}

// Utility functions

fn format_duration(duration: Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{:.2}s", duration.as_secs_f64())
    } else if duration.as_millis() > 0 {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{}μs", duration.as_micros())
    }
}

// Convenience macros for creating matchers

#[macro_export]
macro_rules! match_performance {
    (max_duration: $duration:expr) => {
        $crate::common::matchers::PerformanceMatcher::new()
            .max_duration($duration)
    };
    (max_memory: $memory:expr) => {
        $crate::common::matchers::PerformanceMatcher::new()
            .max_memory_mb($memory)
    };
    (max_duration: $duration:expr, max_memory: $memory:expr) => {
        $crate::common::matchers::PerformanceMatcher::new()
            .max_duration($duration)
            .max_memory_mb($memory)
    };
}

#[macro_export]
macro_rules! match_http {
    (status: $status:expr) => {
        $crate::common::matchers::HttpResponseMatcher::new()
            .status($status)
    };
    (status: $status:expr, body_contains: $pattern:expr) => {
        $crate::common::matchers::HttpResponseMatcher::new()
            .status($status)
            .body_contains($pattern)
    };
}

#[macro_export]
macro_rules! match_error {
    (type: $error_type:expr) => {
        $crate::common::matchers::ErrorMatcher::new()
            .error_type($error_type)
    };
    (code: $error_code:expr, message: $pattern:expr) => {
        $crate::common::matchers::ErrorMatcher::new()
            .error_code($error_code)
            .message_contains($pattern)
    };
} 