//! Property-Based Testing Types - Canonical Location
//!
//! **Unified type definitions** for property-based testing framework.
//! All property testing types are consolidated here to eliminate fragmentation.

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;
use tracing::info;

/// Property-based testing framework - main orchestrator
pub struct PropertyBasedTestFramework {
    pub config: PropertyTestConfig,
    /// Test execution statistics
    pub statistics: PropertyTestStatistics,
    /// Generated test cases
    pub test_cases: Vec<TestCase>,
    /// Property validation results
    pub results: Vec<PropertyTestResult>,
}

/// Configuration for property-based testing
#[derive(Debug, Clone)]
pub struct PropertyTestConfig {
    /// Number of test cases to generate
    pub test_cases: usize,
    /// Maximum test execution time
    pub max_execution_time: Duration,
    /// Whether shrinking is enabled
    pub enable_shrinking: bool,
    /// Optional seed for reproducibility
    pub seed: Option<u64>,
    /// Verbosity level
    pub verbose: bool,
}

impl Default for PropertyTestConfig {
    fn default() -> Self {
        Self {
            test_cases: 1000,
            max_execution_time: Duration::from_secs(30),
            enable_shrinking: true,
            seed: None,
            verbose: false,
        }
    }
}

/// Test execution statistics
#[derive(Debug, Default)]
pub struct PropertyTestStatistics {
    pub total_tests: u64,
    pub passed_tests: u64,
    pub failed_tests: u64,
    pub properties_tested: u64,
    pub average_execution_time_ms: f64,
    pub min_execution_time_ms: f64,
    pub max_execution_time_ms: f64,
}

/// Individual test case
#[derive(Debug, Clone)]
pub struct TestCase {
    pub id: u64,
    pub input_data: Vec<u8>,
    pub test_type: String,
    pub expected_properties: Vec<String>,
}

/// Result of a property test
#[derive(Debug, Clone)]
pub struct PropertyTestResult {
    pub test_case_id: u64,
    pub property_name: String,
    pub passed: bool,
    pub execution_time_ms: f64,
    pub error_message: Option<String>,
}

/// Property value types
#[derive(Debug, Clone)]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Bytes(Vec<u8>),
}

/// Property trait - defines what can be tested
pub trait Property<T> {
    /// Test the property against the input
    fn test(&self, input: &T) -> Result<bool, BearDogError>;

    /// Get the property name
    fn name(&self) -> &str;

    /// Get the property description
    fn description(&self) -> &str;
}

// ============================================================================
// Implementation
// ============================================================================

impl PropertyBasedTestFramework {
    /// Create a new property-based testing framework
    pub fn new(config: PropertyTestConfig) -> Self {
        info!("🧪 Initializing Property-Based Testing Framework");
        info!("   📊 Test cases: {}", config.test_cases);
        info!("   ⏱️  Max execution time: {:?}", config.max_execution_time);
        info!("   🔧 Shrinking enabled: {}", config.enable_shrinking);

        Self {
            config,
            statistics: PropertyTestStatistics::default(),
            test_cases: Vec::new(),
            results: Vec::new(),
        }
    }

    /// Generate comprehensive property test report
    pub fn generate_property_test_report(&self) -> HashMap<String, String> {
        let mut report = HashMap::new();

        let success_rate = if self.statistics.total_tests > 0 {
            (self.statistics.passed_tests as f64 / self.statistics.total_tests as f64) * 100.0
        } else {
            0.0
        };

        report.insert("success_rate".to_string(), format!("{:.2}%", success_rate));
        report.insert(
            "total_tests".to_string(),
            self.statistics.total_tests.to_string(),
        );
        report.insert(
            "passed_tests".to_string(),
            self.statistics.passed_tests.to_string(),
        );
        report.insert(
            "failed_tests".to_string(),
            self.statistics.failed_tests.to_string(),
        );
        report.insert(
            "properties_tested".to_string(),
            self.statistics.properties_tested.to_string(),
        );
        report.insert(
            "average_execution_time_ms".to_string(),
            format!("{:.2}", self.statistics.average_execution_time_ms),
        );

        // Property-specific statistics
        let mut property_stats: HashMap<String, (u64, u64)> = HashMap::new();
        for result in &self.results {
            let entry = property_stats
                .entry(result.property_name.clone())
                .or_insert((0, 0));
            if result.passed {
                entry.0 += 1;
            } else {
                entry.1 += 1;
            }
        }

        for (property, (passed, failed)) in property_stats {
            let total = passed + failed;
            let success_rate = if total > 0 {
                (passed as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            report.insert(
                format!(
                    "property_{}_success_rate",
                    property.replace(" ", "_").to_lowercase()
                ),
                format!("{:.1}%", success_rate),
            );
        }

        report
    }

    /// Record test result
    pub fn record_result(&mut self, test_case_id: u64, property_name: &str, passed: bool) {
        self.results.push(PropertyTestResult {
            test_case_id,
            property_name: property_name.to_string(),
            passed,
            execution_time_ms: 0.0,
            error_message: if passed {
                None
            } else {
                Some("Property violation detected".to_string())
            },
        });

        if passed {
            self.statistics.passed_tests += 1;
        } else {
            self.statistics.failed_tests += 1;
        }
        self.statistics.total_tests += 1;
    }

    /// Log comprehensive statistics
    pub fn log_statistics(&self) {
        info!("📊 Property-Based Testing Statistics:");
        info!("   🧪 Total Tests: {}", self.statistics.total_tests);
        info!("   ✅ Passed: {}", self.statistics.passed_tests);
        info!("   ❌ Failed: {}", self.statistics.failed_tests);
        info!(
            "   🔍 Properties Tested: {}",
            self.statistics.properties_tested
        );

        if self.statistics.total_tests > 0 {
            let success_rate =
                (self.statistics.passed_tests as f64 / self.statistics.total_tests as f64) * 100.0;
            info!("   📈 Success Rate: {:.2}%", success_rate);
        }

        info!(
            "   ⏱️  Average Execution Time: {:.2}ms",
            self.statistics.average_execution_time_ms
        );
        info!(
            "   ⚡ Min Execution Time: {:.2}ms",
            self.statistics.min_execution_time_ms
        );
        info!(
            "   🐌 Max Execution Time: {:.2}ms",
            self.statistics.max_execution_time_ms
        );
    }
}

impl Default for PropertyBasedTestFramework {
    fn default() -> Self {
        Self::new(PropertyTestConfig::default())
    }
}
