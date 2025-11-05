// Testing utilities and types for BearDog
// Provides testing configuration and helper types

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    /// Enable verbose output
    /// Whether verbose is enabled
    pub verbose: bool,
    /// Test timeout duration
    pub timeout: Duration,
    /// Enable parallel execution
    /// Whether `parallel_execution` is enabled
    pub parallel_execution: bool,
    /// Maximum number of retries
    /// Number of `max_retries`
    pub max_retries: u32,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            timeout: Duration::from_secs(30),
            parallel_execution: true,
            max_retries: 3,
        }
    }
}

/// Test result status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestStatus {
    /// Test passed
    Passed,
    /// Test failed
    Failed,
    /// Test was skipped
    Skipped,
    /// Test is running
    Running,
    /// Test timed out
    Timeout,
}

impl Default for TestStatus {
    fn default() -> Self {
        Self::Running
    }
}

/// Test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Test name
    /// Name of the item
    pub name: String,
    /// Test status
    /// Current status of the component
    pub status: TestStatus,
    /// Execution duration
    /// The duration value
    pub duration: Duration,
    /// Error message if failed
    /// Optional error message
    pub error_message: Option<String>,
    /// Additional metadata
    /// The metadata value
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for TestResult {
    fn default() -> Self {
        Self {
            name: "Unknown Test".to_string(),
            status: TestStatus::default(),
            duration: Duration::from_secs(0),
            error_message: None,
            metadata: std::collections::HashMap::new(),
        }
    }
}

/// Test suite configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    /// Suite name
    /// Name of the item
    pub name: String,
    /// Test configuration
    pub config: TestConfig,
    /// Test results
    /// Collection of results
    pub results: Vec<TestResult>,
    /// Suite start time
    /// Optional started at
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Suite completion time
    /// Optional completed at
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for TestSuite {
    fn default() -> Self {
        Self {
            name: "Default Test Suite".to_string(),
            config: TestConfig::default(),
            results: Vec::new(),
            started_at: None,
            completed_at: None,
        }
    }
}

impl TestConfig {
    /// Create a new test configuration
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fast() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            parallel_execution: true,
            max_retries: 1,
            ..Default::default()
        }
    }

    pub fn comprehensive() -> Self {
        Self {
            verbose: true,
            timeout: Duration::from_secs(300),
            parallel_execution: false,
            max_retries: 5,
        }
    }
}

impl TestResult {
    /// Create a new test result
    /// Creates a new instance
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    /// Mark test as passed
    pub fn pass(mut self, duration: Duration) -> Self {
        self.status = TestStatus::Passed;
        self.duration = duration;
        self
    }

    /// Mark test as failed
    pub fn fail(mut self, duration: Duration, error_message: String) -> Self {
        self.status = TestStatus::Failed;
        self.duration = duration;
        self.error_message = Some(error_message);
        self
    }

    /// Mark test as skipped
    pub fn skip(mut self, reason: String) -> Self {
        self.status = TestStatus::Skipped;
        self.error_message = Some(reason);
        self
    }

    /// Mark test as timed out
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.status = TestStatus::Timeout;
        self.duration = duration;
        self.error_message = Some("Test timed out".to_string());
        self
    }

    /// Add metadata
    /// Creates instance with metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Check if test was successful
    /// Checks if successful
    /// Checks if successful
    pub fn is_successful(&self) -> bool {
        matches!(self.status, TestStatus::Passed)
    }

    /// Check if test failed
    /// Checks if failed
    /// Checks if failed
    pub fn is_failed(&self) -> bool {
        matches!(self.status, TestStatus::Failed | TestStatus::Timeout)
    }
}

impl TestSuite {
    /// Create a new test suite
    /// Creates a new instance
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    /// Start the test suite
    /// Starts service
    /// Starts service
    pub fn start(&mut self) {
        self.started_at = Some(chrono::Utc::now());
    }

    /// Complete the test suite
    pub fn complete(&mut self) {
        self.completed_at = Some(chrono::Utc::now());
    }

    /// Add a test result
    pub fn add_result(&mut self, result: TestResult) {
        self.results.push(result);
    }

    /// Get total test count
    pub fn total_tests(&self) -> usize {
        self.results.len()
    }

    /// Get passed test count
    pub fn passed_tests(&self) -> usize {
        self.results.iter().filter(|r| r.is_successful()).count()
    }

    /// Get failed test count
    pub fn failed_tests(&self) -> usize {
        self.results.iter().filter(|r| r.is_failed()).count()
    }

    /// Get skipped test count
    pub fn skipped_tests(&self) -> usize {
        self.results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Skipped))
            .count()
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_tests() == 0 {
            0.0
        } else {
            self.passed_tests() as f64 / self.total_tests() as f64
        }
    }

    /// Get total execution time
    pub fn total_duration(&self) -> Duration {
        self.results.iter().map(|r| r.duration).sum()
    }

    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.failed_tests() == 0 && self.total_tests() > 0
    }

    /// Generate summary
    pub fn summary(&self) -> TestSummary {
        TestSummary {
            suite_name: self.name.clone(),
            total_tests: self.total_tests(),
            passed_tests: self.passed_tests(),
            failed_tests: self.failed_tests(),
            skipped_tests: self.skipped_tests(),
            success_rate: self.success_rate(),
            total_duration: self.total_duration(),
            started_at: self.started_at,
            completed_at: self.completed_at,
        }
    }
}

/// Test suite summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    /// Suite name
    /// Name of the suite
    pub suite_name: String,
    /// Total number of tests
    /// Number of `total_tests`
    pub total_tests: usize,
    /// Number of passed tests
    /// Number of `passed_tests`
    pub passed_tests: usize,
    /// Number of failed tests
    /// Number of `failed_tests`
    pub failed_tests: usize,
    /// Number of skipped tests
    /// Number of `skipped_tests`
    pub skipped_tests: usize,
    /// Success rate (0.0 to 1.0)
    /// The success rate value
    pub success_rate: f64,
    /// Total execution duration
    /// The total duration value
    pub total_duration: Duration,
    /// Suite start time
    /// Optional started at
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Suite completion time
    /// Optional completed at
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = TestConfig::new();
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert!(config.parallel_execution);

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let fast_config = TestConfig::fast();
        assert_eq!(fast_config.timeout, Duration::from_secs(5));
        assert_eq!(fast_config.max_retries, 1);
    }

    #[test]
    fn test_result_creation() {
        let result = TestResult::new("test1".to_string()).pass(crate::constants::domains::system::defaults::DEFAULT_RETRY_DELAY);
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

        assert_eq!(result.name, "test1");
        assert!(result.is_successful());
        assert_eq!(result.duration, crate::constants::domains::system::defaults::DEFAULT_RETRY_DELAY);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_suite_operations() {
        let mut suite = TestSuite::new("Test Suite".to_string());
        suite.start();

        let result1 = TestResult::new("test1".to_string()).pass(crate::constants::domains::system::defaults::DEFAULT_RETRY_DELAY);
        let result2 = TestResult::new("test2".to_string())
            .fail(Duration::from_millis(200), "Test failed".to_string());

        suite.add_result(result1);
        suite.add_result(result2);
        suite.complete();

        assert_eq!(suite.total_tests(), 2);
        assert_eq!(suite.passed_tests(), 1);
        assert_eq!(suite.failed_tests(), 1);
        assert_eq!(suite.success_rate(), 0.5);
        assert!(!suite.all_passed());
    }
}
