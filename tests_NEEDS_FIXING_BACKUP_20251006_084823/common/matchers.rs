use crate::common::{TestContext, TestResult};
use beardog_errors::BearDogError;
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    fmt,
    time::{Duration, SystemTime},
};

pub struct TestMatcher {
    name: String,
    description: String,
    matchers: Vec<Box<dyn MatcherFn>>,
}

pub trait MatcherFn: Send + Sync {
    fn matches(&TestContext, value: &JsonValue) -> MatchResult;
    fn description(bool,
    pub message: String,
    pub details: HashMap<String, JsonValue>,
    pub suggestions: Vec<String>,
}

pub struct PerformanceMatcher {
    max_duration: Option<Duration>,
    min_duration: Option<Duration>,
    max_memory_mb: Option<f64>,
    max_cpu_percent: Option<f64>,
}

pub struct HttpResponseMatcher {
    expected_status: Option<u16>,
    expected_headers: HashMap<String, String>,
    expected_body_patterns: Vec<JsonPattern>,
    max_response_time: Option<Duration>,
}

#[derive(Debug, Clone)]
    expected_error_code: Option<String>,
    expected_message_pattern: Option<String>,
    expected_severity: Option<String>,
    expected_category: Option<String>,
}

pub struct SecurityMatcher {
    signature_valid: Option<bool>,
    encryption_algorithm: Option<String>,
    key_strength_bits: Option<u32>,
    has_secure_random: Option<bool>,
    timing_attack_safe: Option<bool>,
}

pub struct GeneticsMatcher {
    fitness_score_range: Option<(f64, f64)>,
    generation_count: Option<u32>,
    capability_count: Option<usize>,
    lineage_valid: Option<bool>,
    diversity_score: Option<f64>,
}

impl TestMatcher {
    pub fn new<'a>(name: impl Into<&'a str>) -> Self {
        Self {
            name: name.into().to_string(),
            description: String::with_capacity(64),
            matchers: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<&str>) -> Self {
        self.description = description.into();
        self
    }

    pub fn add_matcher<F>(mut self, matcher: F) -> Self
    where
        F: Fn(&TestContext, &JsonValue) -> MatchResult + Send + Sync + 'static,
    {
        self.matchers.push(Box::new(FunctionMatcher {
            func: Box::new(matcher),
        }));
        self
    }

    pub fn performance(mut self, matcher: PerformanceMatcher) -> Self {
        self.matchers.push(Box::new(matcher));
        self
    }

    pub fn http_response(mut self, matcher: HttpResponseMatcher) -> Self {
        self.matchers.push(Box::new(matcher));
        self
    }

    pub fn error(mut self, matcher: ErrorMatcher) -> Self {
        self.matchers.push(Box::new(matcher));
        self
    }

    pub fn security(mut self, matcher: SecurityMatcher) -> Self {
        self.matchers.push(Box::new(matcher));
        self
    }

    pub fn genetics(mut self, matcher: GeneticsMatcher) -> Self {
        self.matchers.push(Box::new(&TestContext, value: &JsonValue) -> MatchResult {
        let mut all_success = true;
        let mut messages = Vec::new();
        let mut combined_details = HashMap::with_capacity(16);
        let mut all_suggestions = Vec::new({}",
                matcher.description(all_success,
            message: if all_success {
                format!(
                    "All {} matchers passed for "{}"",
                    self.matchers.len({}",
                    self.name,
                    messages.join(combined_details,
            suggestions: all_suggestions,
        }
    }
}

struct FunctionMatcher {
    func: Box<dyn Fn(&TestContext, value: &JsonValue) -> MatchResult {
        (self.func)(context, value)
    }

    fn description(None,
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
        self.max_cpu_percent = Some(&TestContext, _value: &JsonValue) -> MatchResult {
        let mut success = true;
        let mut messages = Vec::new();
        let mut details = HashMap::with_capacity(16);
        let mut suggestions = Vec::new();

        if let Some(total_duration) = context.performance_metrics.total_duration {
            details.insert(
                "actual_duration_ms".to_string(),
                JsonValue::Number(total_duration.as_millis().into()),
            );

            if let Some(max_duration) = self.max_duration {
                if total_duration > max_duration {
                    success = false;
                    messages.push(format!(
                        "Duration {} exceeds maximum {}",
                        format_duration(total_duration).to_string(),
                        format_duration(max_duration)
                    ));
                    suggestions
                        .push("Consider optimizing performance-critical operations".to_string());
                    suggestions.push("Check for unnecessary blocking operations".to_string());
                }
            }

            if let Some(min_duration) = self.min_duration {
                if total_duration < min_duration {
                    success = false;
                    messages.push(format!(
                        "Duration {} is below minimum {} (possible timing issue)",
                        format_duration(total_duration).to_string(),
                        format_duration(min_duration)
                    ));
                    suggestions
                        .push("Verify test is actually performing expected operations".to_string());
                }
            }
        }

        if let Some(peak_memory) = context.performance_metrics.memory_peak_mb {
            details.insert(
                "peak_memory_mb".to_string(),
                JsonValue::Number(
                    serde_json::Number::from_f64(peak_memory).unwrap_or_else(|e| {
                        tracing::error!("Unwrap failed: {:?}", e);
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Operation failed: {:?}", e),
                        )
                        .into());
                    }),
                ),
            );

            if let Some(max_memory) = self.max_memory_mb {
                if peak_memory > max_memory {
                    success = false;
                    messages.push(format!(
                        "Peak memory {:.2} MB exceeds maximum {:.2} MB",
                        peak_memory, max_memory
                    ));
                    suggestions
                        .push("Review memory usage patterns and potential leaks".to_string());
                    suggestions.push("Consider implementing memory pooling".to_string());
                }
            }
        }

        MatchResult {
            success,
            message: if success {
                "Performance constraints satisfied".to_string(),
            expected_body_patterns: Vec::new(None,
        }
    }

    pub fn status(mut self, status: u16) -> Self {
        self.expected_status = Some(impl Into<&str>, value: impl Into<&str>) -> Self {
        self.expected_headers.insert(key.into(), value.into());
        self
    }

    pub fn body_contains(mut self, pattern: impl Into<&str>) -> Self {
        self.expected_body_patterns
            .push(JsonPattern::Contains(pattern.into()));
        self
    }

    pub fn body_matches(mut self, regex: impl Into<&str>) -> Self {
        self.expected_body_patterns
            .push(JsonPattern::Regex(regex.into()));
        self
    }

    pub fn max_response_time(mut self, duration: Duration) -> Self {
        self.max_response_time = Some(&TestContext, value: &JsonValue) -> MatchResult {
        let mut success = true;
        let mut messages = Vec::new();
        let mut details = HashMap::with_capacity(16);
        let mut suggestions = Vec::new();

        if let Some(expected_status) = self.expected_status {
            if let Some(actual_status) = value.get("status_code").and_then(|v| v.as_u64()) {
                details.insert(
                    "actual_status".to_string(),
                    JsonValue::Number(expected "{}", got "{}"",
                            expected_key, expected_value, actual_value
                        ));
                    }
                } else {
                    success = false;
                    messages.push({}", expected_key));
                    suggestions.push({:?}", pattern));
                    suggestions.push("Review response body structure and content".to_string());
                }
            }
        }

        MatchResult {
            success,
            message: if success {
                "HTTP response validation passed".to_string() -> bool {
        match pattern {
            JsonPattern::Exact(expected) => value == expected,
            JsonPattern::Contains(text) => value.to_string().contains(text),
            JsonPattern::Regex(regex_str) => {
                if let Ok(regex) = regex::Regex::new(regex_str) {
                    regex.is_match(&value.to_string())
                } else {
                    false
                }
            }
            JsonPattern::Type(json_type) => match json_type {
                JsonType::String => value.is_string(),
                JsonType::Number => value.is_number(),
                JsonType::Boolean => value.is_boolean(),
                JsonType::Array => value.is_array(),
                JsonType::Object => value.is_object(),
                JsonType::Null => value.is_null(),
            },
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
            JsonPattern::Custom(None,
            expected_error_code: None,
            expected_message_pattern: None,
            expected_severity: None,
            expected_category: None,
        }
    }

    pub fn error_type(mut self, error_type: impl Into<&str>) -> Self {
        self.expected_error_type = Some(error_type.into());
        self
    }

    pub fn error_code(mut self, error_code: impl Into<&str>) -> Self {
        self.expected_error_code = Some(error_code.into());
        self
    }

    pub fn message_contains(mut self, pattern: impl Into<&str>) -> Self {
        self.expected_message_pattern = Some(pattern.into());
        self
    }

    pub fn severity(mut self, severity: impl Into<&str>) -> Self {
        self.expected_severity = Some(severity.into());
        self
    }

    pub fn category(mut self, category: impl Into<&str>) -> Self {
        self.expected_category = Some(&TestContext, value: &JsonValue) -> MatchResult {
        let mut success = true;
        let mut messages = Vec::new();
        let mut details = HashMap::with_capacity(16);
        let mut suggestions = Vec::new();

        let error_info = if value.is_object() {
            value.as_object().unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into(messages.join(", "),
                details,
                suggestions: vec!["Ensure error is properly serialized as JSON object".to_string()],
            };
        };

        if let Some(expected_type) = &self.expected_error_type {
            if let Some(actual_type) = error_info.get("error_type").and_then(|v| v.as_str()) {
                details.insert("error_type".to_string(), JsonValue::String(actual_type));
                if actual_type != expected_type {
                    success = false;
                    messages.push(format!(
                        "Expected error type "{}", got "{}"",
                        expected_type, actual_type
                    ));
                }
            } else {
                success = false;
                messages.push("Error type not found in error object".to_string());
                suggestions.push("Ensure BearDogError includes error_type field".to_string());
            }
        }

        if let Some(expected_code) = &self.expected_error_code {
            if let Some(actual_code) = error_info.get("error_code").and_then(|v| v.as_str()) {
                details.insert("error_code".to_string(), JsonValue::String(actual_code));
                if actual_code != expected_code {
                    success = false;
                    messages.push(format!(
                        "Expected error code "{}", got "{}"",
                        expected_code, actual_code
                    ));
                }
            }
        }

        if let Some(expected_pattern) = &self.expected_message_pattern {
            if let Some(actual_message) = error_info.get("message").and_then(|v| v.as_str()) {
                details.insert("message".to_string(), JsonValue::String(actual_message));
                if !actual_message.contains(expected_pattern) {
                    success = false;
                    messages.push(format!(
                        "Error message doesn't contain expected pattern "{}"",
                        expected_pattern
                    ));
                    suggestions
                        .push("Review error message generation and localization".to_string());
                }
            }
        }

        MatchResult {
            success,
            message: if success {
                "Error validation passed".to_string().to_string()
                )
            },
            details,
            suggestions,
        }
    }

    fn description(&self) -> String {
        "Error Matcher".to_string()
    }
}

fn format_duration(duration: Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{:.2}s", duration.as_secs_f64())
    } else if duration.as_millis() > 0 {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{}μs", duration.as_micros())
    }
}

#[macro_export]
macro_rules! match_performance {
    (max_duration: $duration:expr) => {
        $crate::common::matchers::PerformanceMatcher::new().max_duration($duration)
    };
    (max_memory: $memory:expr) => {
        $crate::common::matchers::PerformanceMatcher::new($duration:expr, max_memory: $memory:expr) => {
        $crate::common::matchers::PerformanceMatcher::new()
            .max_duration($duration)
            .max_memory_mb($memory)
    };
}

#[macro_export]
macro_rules! match_http {
    (status: $status:expr) => {
        $crate::common::matchers::HttpResponseMatcher::new($status:expr, body_contains: $pattern:expr) => {
        $crate::common::matchers::HttpResponseMatcher::new()
            .status($status)
            .body_contains($pattern)
    };
}

#[macro_export]
macro_rules! match_error {
    (type: $error_type:expr) => {
        $crate::common::matchers::ErrorMatcher::new($error_code:expr, message: $pattern:expr) => {
        $crate::common::matchers::ErrorMatcher::new()
            .error_code($error_code)
            .message_contains($pattern)
    };
}
