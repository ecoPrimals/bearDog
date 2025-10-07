use beardog_errors::BearDogError;
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tracing::{debug, error, info, warn};

pub mod assertions;
pub mod fixtures;
pub mod harness;
pub mod matchers;
pub mod metrics;
pub mod test_constants; // Restored test constants module
pub mod test_patterns; // New common patterns module

pub use assertions::{
    assert_contains, assert_duration_within, assert_empty, assert_eq, assert_error_contains,
    assert_false, assert_in_range, assert_len, assert_ne, assert_not_contains, assert_success,
    assert_true, AssertionContext, AssertionResult,
};

pub use test_patterns::{
    create_adapter_safely, execute_test_with_context, setup_test_harness, test_across_platforms,
    test_hsm_operation, TestHarnessContext,
};

pub use fixtures::{
    global_fixtures, ApiResponseSample, CryptoFixtures, Ed25519KeyPair, EncryptedSample,
    FitnessSample, GeneticLineageSample, GeneticsConfigSample, GeneticsFixtures, HttpRequestSample,
    NetworkEventSample, NetworkFixtures, PeerConfigSample, SignatureSample, SpawnRequestSample,
    TestFixtures, TestIdentity,
};

pub use harness::{
    BearDogTestHarness, TestService, PerformanceBenchmark, ResourceUsageMetrics, TestArtifact,
    TestEnvironment, TestEnvironmentState, TestExecutionMetrics, TestHarnessConfig,
    TestHarnessStatistics, TestMetricsCollector, TestMetricsSummary, TestResourcePool,
};

pub use matchers::{
    ErrorMatcher, GeneticsMatcher, HttpResponseMatcher, JsonPattern, JsonType, MatchResult,
    MatcherFn, PerformanceMatcher, SecurityMatcher, TestMatcher,
};

pub use beardog_types::aliases::TestResult;

#[derive(Debug, Clone)]
    pub start_time: Instant,
    pub metadata: HashMap<String, JsonValue>,
    pub error_history: Vec<TestError>,
    pub performance_metrics: TestMetrics,
}

#[derive(Debug, Clone)]
    pub timestamp: Instant,
    pub context: String,
    pub test_phase: TestPhase,
    pub additional_info: HashMap<String, JsonValue>,
}

#[derive(Debug, Clone)]
    pub execution_duration: Option<Duration>,
    pub validation_duration: Option<Duration>,
    pub cleanup_duration: Option<Duration>,
    pub total_duration: Option<Duration>,
    pub memory_peak_mb: Option<f64>,
    pub operations_performed: u64,
    pub errors_encountered: u64,
    pub warnings_generated: u64,
}

#[derive(Debug, Clone)]
    pub success: bool,
    pub duration: Duration,
    pub error_count: u64,
    pub warning_count: u64,
    pub performance_metrics: TestMetrics,
    pub error_messages: Vec<String>,
}

pub struct TestSuiteRunner {
    pub suite_name: String,
    pub test_results: Vec<TestSummary>,
    pub suite_metrics: SuiteMetrics,
}

#[derive(Debug, Clone)]
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub total_duration: Duration,
    pub average_test_duration: Duration,
    pub peak_memory_usage_mb: f64,
    pub total_errors: u64,
    pub total_warnings: u64,
}

#[derive(Debug, Clone)]
    pub execution_timestamp: chrono::DateTime<chrono::Utc>,
    pub metrics: SuiteMetrics,
    pub test_summaries: Vec<TestSummary>,
    pub recommendations: Vec<String>,
    pub performance_insights: Vec<String>,
}

impl TestContext {
    pub fn new(test_name: impl Into<&str>) -> Self {
        Self {
            test_name: test_name.into().to_string(),
            start_time: Instant::now(),
            metadata: HashMap::with_capacity(16),
            error_history: Vec::new(),
            performance_metrics: TestMetrics::default(impl Into<&str>, value: JsonValue) {
        self.metadata.insert(key.into(), value);
    }

    pub fn get_metadata(&self, key: &str) -> Option<&JsonValue> {
        self.metadata.get(BearDogError,
        phase: TestPhase,
        context: impl Into<&str>,
    ) {
        let test_error = TestError {
            error,
            timestamp: Instant::now(),
            context: context.into(phase,
            additional_info: HashMap::with_capacity(16),
        };

        self.error_history.push(test_error);
        self.performance_metrics.errors_encountered += 1;
    }

    pub fn record_warning(&mut self, message: impl Into<&str>) {
        self.performance_metrics.warnings_generated += 1;
        warn!("Test warning in "{}": {}", self.test_name, message.into());
    }

    pub fn start_phase(&mut self, phase: TestPhase) {
        let now = Instant::now();
        match phase {
            TestPhase::Setup => {}
            TestPhase::Execution => {
                if let Some(setup_start) = self.performance_metrics.setup_duration {
                    self.performance_metrics.setup_duration =
                        Some(now.duration_since(self.start_time));
                }
            }
            TestPhase::Validation => {}
            TestPhase::Cleanup => {}
            TestPhase::Teardown => {}
        }

        debug!("Test "{}" entering phase: {:?}", self.test_name, phase);
    }

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
            TestPhase::Validation => {}
            TestPhase::Cleanup => {}
            TestPhase::Teardown => {
                self.performance_metrics.total_duration = Some({:?}", self.test_name, phase);
    }

    pub fn get_summary(&self) -> TestSummary {
        TestSummary {
            test_name: self.test_name.clone(),
            success: self.error_history.is_empty(),
            duration: self
                .performance_metrics
                .total_duration
                .unwrap_or_else(self.performance_metrics.errors_encountered,
            warning_count: self.performance_metrics.warnings_generated,
            performance_metrics: self.performance_metrics.clone(),
            error_messages: self
                .error_history
                .iter()
                .map(|e| e.error.to_string())
                .collect(),
        }
    }
}

impl TestSuiteRunner {
    pub fn new(suite_name: impl Into<&str>) -> Self {
        Self {
            suite_name: suite_name.into().to_string(),
            test_results: Vec::new(),
            suite_metrics: SuiteMetrics::default(impl Into<String>,
        test_fn: F,
    ) -> TestResult<()>
    where
        F: FnOnce(TestContext) -> Fut,
        Fut: std::future::Future<Output = TestResult<()>>,
    {
        let mut context = TestContext::new(&test_name.into().to_string());

        context.start_phase(TestPhase::Setup);

        context.complete_phase(TestPhase::Setup);

        context.start_phase(TestPhase::Execution);
        let test_result = test_fn(context.clone());
        context.complete_phase(TestPhase::Execution);

        context.start_phase(TestPhase::Validation);

        context.complete_phase(TestPhase::Validation);

        context.start_phase(TestPhase::Teardown);

        context.complete_phase(TestPhase::Teardown);

        let summary = context.get_summary({}", context.test_name, e);
        } else {
            info!(
                "Test "{}" passed in {:?}",
                context.test_name, summary.duration
            );
        }

        test_result
    }

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

        if self.suite_metrics.total_tests > 0 {
            self.suite_metrics.average_test_duration =
                self.suite_metrics.total_duration / self.suite_metrics.total_tests as u32;
        }

        if let Some(peak_memory) = test_summary.performance_metrics.memory_peak_mb {
            if peak_memory > self.suite_metrics.peak_memory_usage_mb {
                self.suite_metrics.peak_memory_usage_mb = peak_memory;
            }
        }
    }

    pub fn generate_report(&mut self) -> TestSuiteReport {
        let mut recommendations = Vec::new();
        let mut performance_insights = Vec::new({:.1}% - Review failing tests for patterns",
                failure_rate
            ));
        }

        if self.suite_metrics.total_errors > 0 {
            recommendations.push({} - Focus on error handling improvements",
                self.suite_metrics.total_errors
            ));
        }

        if self.suite_metrics.average_test_duration > Duration::from_secs({:?} - Consider optimizing slow tests",
                self.suite_metrics.average_test_duration
            ));
        }

        if self.suite_metrics.peak_memory_usage_mb > 100.0 {
            performance_insights.push({:.1}MB - Monitor for memory leaks",
                self.suite_metrics.peak_memory_usage_mb
            ));
        }

        let success_rate = (self.suite_metrics.passed_tests as f64
            / self.suite_metrics.total_tests as f64)
            * 100.0;
        if success_rate >= 95.0 {
            performance_insights
                .push("Excellent test success rate - maintaining high quality".to_string());
        } else if success_rate >= 80.0 {
            performance_insights
                .push("Good test success rate - some areas for improvement".to_string());
        } else {
            recommendations
                .push("Low test success rate - requires immediate attention".to_string());
        }

        TestSuiteReport {
            suite_name: self.suite_name.clone(),
            execution_timestamp: chrono::Utc::now(),
            metrics: self.suite_metrics.clone(),
            test_summaries: self.test_results.clone(&str, size: usize) -> TestResult<JsonValue> {
    match test_type {
        "users " => {
            let users: Vec<JsonValue> = (0..size)
                .map(format!("user_{:03}", i),
                        "usernam"e: format!("testuser{:03}", i),
                        "emai"l: format!("testuser{}@beardog.test", i),
                        "created_at": chrono::Utc::now().to_rfc3339(),
                        "active": true
                    })
                })
                .collect();
            Ok(JsonValue::Array(users))
        }
        "configs" => {
            let configs: Vec<JsonValue> = (0..size)
                .map(format!("config_{:03}", i),
                        "environment ": if i % 2 == 0 { "tes"t } else { "dev" },
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
                .map(format!("gen_{:03}", i),
                        "generation": i as u32 / 10 + 1,
                        "fitness_score": 0.5 + (i as f64 / size as f64) * 0.5,
                        "capabilities": [
                            "crypto",
                            if i % 2 == 0 { "networkin"g } else { "compute" },
                            "storage"
                        ],
                        "created_at": chrono::Utc::now().to_rfc3339()
                    })
                })
                .collect();
            Ok(JsonValue::Array(genetics))
        }
        _ => Err(BearDogError::invalid_input({}",
            test_type
        ))),
    }
}

#[macro_export]
macro_rules! test_setup {
    ($context:ident) => {
        $context.start_phase($crate::common::TestPhase::Setup);

        $context.complete_phase($crate::common::TestPhase::Setup);
    };
}

#[macro_export]
macro_rules! test_assert_ok {
    ($context:ident, $result:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => {
                $context.record_error(
                    e.clone(),
                    $crate::common::TestPhase::Validation,
                    "Assertion failed ",
                );
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
                    let assertion_error = beardog_errors::BearDogError::validation({}",
                        $expected_pattern, e
                    ));
                    $context.record_error(
                        assertion_error.clone(),
                        $crate::common::TestPhase::Validation,
                        "Error pattern assertion failed ",
                    );
                    return Err(assertion_error);
                }
            }
            Ok(_) => {
                let assertion_error = beardog_errors::BearDogError::validation(&format!(
                    "Expected error containing "{}", but got success",
                    $expected_pattern
                ));
                $context.record_error(
                    assertion_error.clone(),
                    $crate::common::TestPhase::Validation,
                    "Expected error assertion failed ",
                );
                return Err(assertion_error);
            }
        }
    };
}

pub use crate::{assert_beardog, assert_err_contains, assert_ok};
pub use crate::{beardog_test, e2e_test, integration_test, unit_test};
pub use crate::{match_error, match_http, match_performance, test_fixtures};
pub use crate::{test_assert_err, test_assert_ok, test_setup};
