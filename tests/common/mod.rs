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


//! Modern Test Infrastructure with Unified Error Handling
//!
//! **Production-Grade Test Utilities for BearDog**
//!
//! This module provides comprehensive test infrastructure that integrates seamlessly
//! with BearDog's unified error system, replacing panic-prone patterns with
//! sophisticated error handling and rich debugging capabilities.

use beardog_errors::{BearDogError, BearDogResult};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tracing::{debug, error, info, warn};

// Re-export test utilities for easy access
pub mod assertions;
pub mod fixtures;
pub mod harness;
pub mod matchers;
pub mod metrics;
pub mod test_patterns; // New common patterns module

// Re-export commonly used types and functions
pub use assertions::{
    assert_success, assert_error_contains, assert_eq, assert_ne, assert_true, assert_false,
    assert_contains, assert_not_contains, assert_empty, assert_len, assert_duration_within,
    assert_in_range, AssertionContext, AssertionResult,
};

pub use test_patterns::{
    execute_test_with_context, test_across_platforms, setup_test_harness,
    test_hsm_operation, create_adapter_safely, TestHarnessContext,
};

pub use fixtures::{
    TestFixtures, TestIdentity, CryptoFixtures, NetworkFixtures, GeneticsFixtures,
    Ed25519KeyPair, EncryptedSample, SignatureSample, HttpRequestSample, ApiResponseSample,
    NetworkEventSample, PeerConfigSample, GeneticsConfigSample, SpawnRequestSample,
    GeneticLineageSample, FitnessSample, global_fixtures,
};

pub use harness::{
    BearDogTestHarness, TestHarnessConfig, TestEnvironment, TestResourcePool,
    TestArtifact, MockService, TestMetricsCollector, TestExecutionMetrics,
    PerformanceBenchmark, ResourceUsageMetrics, TestEnvironmentState,
    TestHarnessStatistics, TestMetricsSummary,
};

pub use matchers::{
    TestMatcher, MatcherFn, MatchResult, PerformanceMatcher, HttpResponseMatcher,
    ErrorMatcher, SecurityMatcher, GeneticsMatcher, JsonPattern, JsonType,
};

/// Modern test result type with rich error context
// Moved to beardog-types/src/aliases.rs for centralization
pub use beardog_types::aliases::TestResult;

/// Test execution context with comprehensive error tracking
#[derive(Debug, Clone)]
pub struct TestContext {
    pub test_name: String,
    pub start_time: Instant,
    pub metadata: HashMap<String, JsonValue>,
    pub error_history: Vec<TestError>,
    pub performance_metrics: TestMetrics,
}

/// Rich test error with debugging context
#[derive(Debug, Clone)]
pub struct TestError {
    pub error: BearDogError,
    pub timestamp: Instant,
    pub context: String,
    pub test_phase: TestPhase,
    pub additional_info: HashMap<String, JsonValue>,
}

/// Test execution phases for detailed error tracking
#[derive(Debug, Clone, PartialEq)]
pub enum TestPhase {
    Setup,
    Execution,
    Validation,
    Cleanup,
    Teardown,
}

/// Comprehensive test performance metrics
#[derive(Debug, Clone, Default)]
pub struct TestMetrics {
    pub setup_duration: Option<Duration>,
    pub execution_duration: Option<Duration>,
    pub validation_duration: Option<Duration>,
    pub cleanup_duration: Option<Duration>,
    pub total_duration: Option<Duration>,
    pub memory_peak_mb: Option<f64>,
    pub operations_performed: u64,
    pub errors_encountered: u64,
    pub warnings_generated: u64,
}

/// Test execution summary for reporting
#[derive(Debug, Clone)]
pub struct TestSummary {
    pub test_name: String,
    pub success: bool,
    pub duration: Duration,
    pub error_count: u64,
    pub warning_count: u64,
    pub performance_metrics: TestMetrics,
    pub error_messages: Vec<String>,
}

/// Test suite runner for managing multiple tests
pub struct TestSuiteRunner {
    pub suite_name: String,
    pub test_results: Vec<TestSummary>,
    pub suite_metrics: SuiteMetrics,
}

/// Metrics for an entire test suite
#[derive(Debug, Clone, Default)]
pub struct SuiteMetrics {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub total_duration: Duration,
    pub average_test_duration: Duration,
    pub peak_memory_usage_mb: f64,
    pub total_errors: u64,
    pub total_warnings: u64,
}

/// Report generated after test suite execution
#[derive(Debug, Clone)]
pub struct TestSuiteReport {
    pub suite_name: String,
    pub execution_timestamp: chrono::DateTime<chrono::Utc>,
    pub metrics: SuiteMetrics,
    pub test_summaries: Vec<TestSummary>,
    pub recommendations: Vec<String>,
    pub performance_insights: Vec<String>,
}

impl TestContext {
    /// Create a new test context
    pub fn new(test_name: impl Into<String>) -> Self {
        Self {
            test_name: test_name.into(),
            start_time: Instant::now(),
            metadata: HashMap::new(),
            error_history: Vec::new(),
            performance_metrics: TestMetrics::default(),
        }
    }

    /// Add metadata to the test context
    pub fn add_metadata(&mut self, key: impl Into<String>, value: JsonValue) {
        self.metadata.insert(key.into(), value);
    }

    /// Get metadata from the test context
    pub fn get_metadata(&self, key: &str) -> Option<&JsonValue> {
        self.metadata.get(key)
    }

    /// Record an error that occurred during testing
    pub fn record_error(&mut self, error: BearDogError, phase: TestPhase, context: impl Into<String>) {
        let test_error = TestError {
            error,
            timestamp: Instant::now(),
            context: context.into(),
            test_phase: phase,
            additional_info: HashMap::new(),
        };
        
        self.error_history.push(test_error);
        self.performance_metrics.errors_encountered += 1;
    }

    /// Record a warning that occurred during testing
    pub fn record_warning(&mut self, message: impl Into<String>) {
        self.performance_metrics.warnings_generated += 1;
        warn!("Test warning in '{}': {}", self.test_name, message.into());
    }

    /// Mark the start of a test phase
    pub fn start_phase(&mut self, phase: TestPhase) {
        let now = Instant::now();
        match phase {
            TestPhase::Setup => {
                // Setup timing starts from creation
            }
            TestPhase::Execution => {
                if let Some(setup_start) = self.performance_metrics.setup_duration {
                    self.performance_metrics.setup_duration = Some(now.duration_since(self.start_time));
                }
            }
            TestPhase::Validation => {
                // Execution phase ends, validation begins
            }
            TestPhase::Cleanup => {
                // Validation phase ends, cleanup begins
            }
            TestPhase::Teardown => {
                // Final phase
            }
        }
        
        debug!("Test '{}' entering phase: {:?}", self.test_name, phase);
    }

    /// Mark the completion of a test phase
    pub fn complete_phase(&mut self, phase: TestPhase) {
        let now = Instant::now();
        
        match phase {
            TestPhase::Setup => {
                self.performance_metrics.setup_duration = Some(now.duration_since(self.start_time));
            }
            TestPhase::Execution => {
                if let Some(setup_duration) = self.performance_metrics.setup_duration {
                    self.performance_metrics.execution_duration = 
                        Some(now.duration_since(self.start_time) - setup_duration);
                }
            }
            TestPhase::Validation => {
                // Calculate validation duration
            }
            TestPhase::Cleanup => {
                // Calculate cleanup duration
            }
            TestPhase::Teardown => {
                self.performance_metrics.total_duration = Some(now.duration_since(self.start_time));
            }
        }
        
        debug!("Test '{}' completed phase: {:?}", self.test_name, phase);
    }

    /// Get a summary of the test execution
    pub fn get_summary(&self) -> TestSummary {
        TestSummary {
            test_name: self.test_name.clone(),
            success: self.error_history.is_empty(),
            duration: self.performance_metrics.total_duration.unwrap_or_else(|| self.start_time.elapsed()),
            error_count: self.performance_metrics.errors_encountered,
            warning_count: self.performance_metrics.warnings_generated,
            performance_metrics: self.performance_metrics.clone(),
            error_messages: self.error_history.iter()
                .map(|e| e.error.to_string())
                .collect(),
        }
    }
}

impl TestSuiteRunner {
    /// Create a new test suite runner
    pub fn new(suite_name: impl Into<String>) -> Self {
        Self {
            suite_name: suite_name.into(),
            test_results: Vec::new(),
            suite_metrics: SuiteMetrics::default(),
        }
    }

    /// Run a single test and record results
    pub async fn run_test<F, Fut>(&mut self, test_name: impl Into<String>, test_fn: F) -> TestResult<()>
    where
        F: FnOnce(TestContext) -> Fut,
        Fut: std::future::Future<Output = TestResult<()>>,
    {
        let mut context = TestContext::new(&test_name.into());
        
        context.start_phase(TestPhase::Setup);
        // Test setup would happen here
        context.complete_phase(TestPhase::Setup);
        
        context.start_phase(TestPhase::Execution);
        let test_result = test_fn(context.clone()).await;
        context.complete_phase(TestPhase::Execution);
        
        context.start_phase(TestPhase::Validation);
        // Test validation happens as part of the test function
        context.complete_phase(TestPhase::Validation);
        
        context.start_phase(TestPhase::Teardown);
        // Cleanup happens here
        context.complete_phase(TestPhase::Teardown);
        
        // Record test result
        let summary = context.get_summary();
        self.test_results.push(summary.clone());
        
        // Update suite metrics
        self.update_suite_metrics(&summary);
        
        if let Err(ref e) = test_result {
            error!("Test '{}' failed: {}", context.test_name, e);
        } else {
            info!("Test '{}' passed in {:?}", context.test_name, summary.duration);
        }
        
        test_result
    }

    /// Update suite-level metrics
    fn update_suite_metrics(&mut self, test_summary: &TestSummary) {
        self.suite_metrics.total_tests += 1;
        
        if test_summary.success {
            self.suite_metrics.passed_tests += 1;
        } else {
            self.suite_metrics.failed_tests += 1;
        }
        
        self.suite_metrics.total_duration += test_summary.duration;
        self.suite_metrics.total_errors += test_summary.error_count;
        self.suite_metrics.total_warnings += test_summary.warning_count;
        
        // Update average duration
        if self.suite_metrics.total_tests > 0 {
            self.suite_metrics.average_test_duration = 
                self.suite_metrics.total_duration / self.suite_metrics.total_tests as u32;
        }
        
        // Update peak memory usage
        if let Some(peak_memory) = test_summary.performance_metrics.memory_peak_mb {
            if peak_memory > self.suite_metrics.peak_memory_usage_mb {
                self.suite_metrics.peak_memory_usage_mb = peak_memory;
            }
        }
    }

    /// Generate a comprehensive test suite report
    pub fn generate_report(&mut self) -> TestSuiteReport {
        let mut recommendations = Vec::new();
        let mut performance_insights = Vec::new();

        // Generate recommendations based on results
        if self.suite_metrics.failed_tests > 0 {
            let failure_rate = (self.suite_metrics.failed_tests as f64 / self.suite_metrics.total_tests as f64) * 100.0;
            recommendations.push(format!("Test failure rate: {:.1}% - Review failing tests for patterns", failure_rate));
        }

        if self.suite_metrics.total_errors > 0 {
            recommendations.push(format!("Total errors: {} - Focus on error handling improvements", self.suite_metrics.total_errors));
        }

        if self.suite_metrics.average_test_duration > Duration::from_secs(10) {
            performance_insights.push(format!("Average test duration: {:?} - Consider optimizing slow tests", self.suite_metrics.average_test_duration));
        }

        if self.suite_metrics.peak_memory_usage_mb > 100.0 {
            performance_insights.push(format!("Peak memory usage: {:.1}MB - Monitor for memory leaks", self.suite_metrics.peak_memory_usage_mb));
        }

        // Success rate insights
        let success_rate = (self.suite_metrics.passed_tests as f64 / self.suite_metrics.total_tests as f64) * 100.0;
        if success_rate >= 95.0 {
            performance_insights.push("Excellent test success rate - maintaining high quality".to_string());
        } else if success_rate >= 80.0 {
            performance_insights.push("Good test success rate - some areas for improvement".to_string());
        } else {
            recommendations.push("Low test success rate - requires immediate attention".to_string());
        }

        TestSuiteReport {
            suite_name: self.suite_name.clone(),
            execution_timestamp: chrono::Utc::now(),
            metrics: self.suite_metrics.clone(),
            test_summaries: self.test_results.clone(),
            recommendations,
            performance_insights,
        }
    }

    /// Get current success rate
    pub fn get_success_rate(&self) -> f64 {
        if self.suite_metrics.total_tests > 0 {
            (self.suite_metrics.passed_tests as f64 / self.suite_metrics.total_tests as f64) * 100.0
        } else {
            0.0
        }
    }
}

/// Utility function to create test data
pub fn create_test_data(test_type: &str, size: usize) -> TestResult<JsonValue> {
    match test_type {
        "users" => {
            let users: Vec<JsonValue> = (0..size)
                .map(|i| {
                    json!({
                        "id": format!("user_{:03}", i),
                        "username": format!("testuser{:03}", i),
                        "email": format!("testuser{}@beardog.test", i),
                        "created_at": chrono::Utc::now().to_rfc3339(),
                        "active": true
                    })
                })
                .collect();
            Ok(JsonValue::Array(users))
        }
        "configs" => {
            let configs: Vec<JsonValue> = (0..size)
                .map(|i| {
                    json!({
                        "config_id": format!("config_{:03}", i),
                        "environment": if i % 2 == 0 { "test" } else { "dev" },
                        "features": {
                            "crypto": true,
                            "networking": i % 3 == 0,
                            "genetics": true
                        },
                        "created_at": chrono::Utc::now().to_rfc3339()
                    })
                })
                .collect();
            Ok(JsonValue::Array(configs))
        }
        "genetics" => {
            let genetics: Vec<JsonValue> = (0..size)
                .map(|i| {
                    json!({
                        "genetics_id": format!("gen_{:03}", i),
                        "generation": i as u32 / 10 + 1,
                        "fitness_score": 0.5 + (i as f64 / size as f64) * 0.5,
                        "capabilities": [
                            "crypto",
                            if i % 2 == 0 { "networking" } else { "compute" },
                            "storage"
                        ],
                        "created_at": chrono::Utc::now().to_rfc3339()
                    })
                })
                .collect();
            Ok(JsonValue::Array(genetics))
        }
        _ => Err(BearDogError::invalid_input(&format!("Unknown test data type: {}", test_type)))
    }
}

/// Convenience macros for test setup
#[macro_export]
macro_rules! test_setup {
    ($context:ident) => {
        $context.start_phase($crate::common::TestPhase::Setup);
        // Setup code here
        $context.complete_phase($crate::common::TestPhase::Setup);
    };
}

#[macro_export]
macro_rules! test_assert_ok {
    ($context:ident, $result:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => {
                $context.record_error(e.clone(), $crate::common::TestPhase::Validation, "Assertion failed");
                return Err(e);
            }
        }
    };
}

#[macro_export]
macro_rules! test_assert_err {
    ($context:ident, $result:expr, $expected_pattern:expr) => {
        match $result {
            Err(e) => {
                if !e.to_string().contains($expected_pattern) {
                    let assertion_error = beardog_errors::BearDogError::validation(&format!(
                        "Expected error containing '{}', got: {}", $expected_pattern, e
                    ));
                    $context.record_error(assertion_error.clone(), $crate::common::TestPhase::Validation, "Error pattern assertion failed");
                    return Err(assertion_error);
                }
            }
            Ok(_) => {
                let assertion_error = beardog_errors::BearDogError::validation(&format!(
                    "Expected error containing '{}', but got success", $expected_pattern
                ));
                $context.record_error(assertion_error.clone(), $crate::common::TestPhase::Validation, "Expected error assertion failed");
                return Err(assertion_error);
            }
        }
    };
}

/// Re-export macros from sub-modules
pub use crate::{beardog_test, unit_test, integration_test, e2e_test};
pub use crate::{test_fixtures, match_performance, match_http, match_error};
pub use crate::{test_setup, test_assert_ok, test_assert_err};
pub use crate::{assert_beardog, assert_ok, assert_err_contains}; 