// Types and structures for property-based testing framework
//
// This module contains all type definitions used by the property-based testing
// framework to maintain clean separation and keep files under 1000 lines.

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;

pub struct PropertyBasedTestFramework {
    pub config: PropertyTestConfig,
    /// Test execution statistics
    /// The statistics value
    pub statistics: PropertyTestStatistics,
    /// Generated test cases
    /// Collection of test cases
    pub test_cases: Vec<TestCase>,
    /// Property validation results
    /// Collection of results
    pub results: Vec<PropertyTestResult>,
}

#[derive(Debug, Clone)]
pub struct PropertyTestConfig {
    /// Number of test cases to generate
    /// Number of test_cases
    pub test_cases: usize,
    /// Maximum test execution time
    pub max_execution_time: Duration,
    /// Whether enable_shrinking is enabled
    pub enable_shrinking: bool,
    /// Optional seed
    pub seed: Option<u64>,
    /// Verbosity level
    /// Whether verbose is enabled
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

#[derive(Debug, Default)]
pub struct PropertyTestStatistics {
    /// Number of total_tests
    pub total_tests: u64,
    /// Number of passed_tests
    pub passed_tests: u64,
    /// Number of failed_tests
    pub failed_tests: u64,
    /// Number of shrunk_tests
    pub shrunk_tests: u64,
    pub total_execution_time: Duration,
    pub average_execution_time: Duration,
    pub min_execution_time: Duration,
    pub max_execution_time: Duration,
}

#[derive(Debug, Clone)]
pub struct TestCase {
    pub id: String,
    /// The input value
    pub input: PropertyValue,
    /// Optional expected output
    pub expected_output: Option<PropertyValue>,
    /// Optional actual output
    pub actual_output: Option<PropertyValue>,
    pub execution_time: Duration,
}

/// Result of property-based test execution
#[derive(Debug, Clone)]
pub struct PropertyTestResult {
    /// The test case value
    pub test_case: TestCase,
    /// Whether passed is enabled
    pub passed: bool,
    /// Optional error
    pub error: Option<BearDogError>,
    /// Number of shrink_attempts
    pub shrink_attempts: u32,
    /// Number of finalerexample
    pub final_counterexample: Option<PropertyValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// Represents integer variant
    Integer(i64),
    /// Represents float variant
    Float(f64),
    /// Currently string
    String(String),
    /// Represents boolean variant
    Boolean(bool),
    /// Represents list variant
    List(Vec<PropertyValue>),
    /// Represents map variant
    Map(HashMap<String, PropertyValue>),
    /// Represents binary variant
    Binary(Vec<u8>),
    /// No none specified
    None,
}

pub trait Property<T> {
    /// Test the property with the given input
    fn test(&self, input: &T) -> Result<bool, BearDogError>;

    fn generate(&self) -> T;

    /// Shrink a failing input to find minimal counterexample
    fn shrink(&self, input: &T) -> Vec<T>;

    fn name(&self) -> &str;
}

pub trait Generator<T> {
    /// Generate a random value
    fn generate(&mut self) -> T;

    /// Generate within specific constraints
    fn generate_constrained(&mut self, constraints: &GeneratorConstraints) -> T;

    fn shrink(&self, value: &T) -> Vec<T>;
}

#[derive(Debug, Clone)]
pub struct GeneratorConstraints {
    /// Optional min size
    pub min_size: Option<usize>,
    /// Optional max size
    pub max_size: Option<usize>,
    /// Optional min value
    pub min_value: Option<i64>,
    /// Optional max value
    pub max_value: Option<i64>,
    /// Optional allowed chars
    pub allowed_chars: Option<String>,
    /// Mapping of custom constraints
    pub custom_constraints: HashMap<String, PropertyValue>,
}

/// Test execution context
#[derive(Debug)]
pub struct TestContext {
    pub test_id: String,
    /// Number of iteration
    pub iteration: u64,
    pub start_time: std::time::Instant,
    pub config: PropertyTestConfig,
    /// Number of random_seed
    pub random_seed: u64,
}

#[derive(Debug, Clone)]
pub enum ShrinkStrategy {
    /// Remove elements from collections
    RemoveElements,
    /// Reduce numeric values towards zero
    ReduceToZero,
    /// Simplify strings by removing characters
    SimplifyStrings,
    /// Custom shrinking logic
    Custom(fn(&PropertyValue) -> Vec<PropertyValue>),
}

impl Default for PropertyBasedTestFramework {
    fn default() -> Self {
        Self {
            config: PropertyTestConfig::default(),
            statistics: PropertyTestStatistics::default(),
            test_cases: Vec::new(),
            results: Vec::new(),
        }
    }
}
