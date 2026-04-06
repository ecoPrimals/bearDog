// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical Testing Configuration Module
//!
//! This module consolidates ALL test configuration types from scattered locations
//! across the codebase into a single canonical source of truth.
//!
//! ## 🎯 Consolidation Achievement
//!
//! **Previously Scattered Locations** (Now Unified):
//! - `tests/common/zero_cost_harness.rs::TestConfig`
//! - `tests/api/comprehensive_tests.rs::ApiTestConfig`
//! - `tests/production/deployment_validation.rs::ProductionDeploymentConfig`
//! - `tests/world_class_testing_framework.rs::TestingConfiguration`
//! - `tests/clone_optimization_benchmark.rs::BenchmarkConfig`
//! - `crates/beardog-integration-tests/src/unified_architecture_tests.rs::TestConfig`
//!
//! **Migration Guide**:
//! ```rust
//! // New (canonical)
//! use beardog_types::canonical::config::domains::testing::{
//!     CanonicalTestConfig,
//!     CanonicalApiTestConfig,
//!     CanonicalBenchmarkConfig,
//! };
//!
//! // Create test configuration
//! let test_config = CanonicalTestConfig::default();
//! println!("Parallel execution: {}", test_config.parallel_execution);
//! ```

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

// ============================================================================
// CANONICAL TEST CONFIGURATIONS
// ============================================================================

/// **Canonical Test Configuration** - General test configuration
///
/// Consolidates test settings for unit, integration, and functional testing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalTestConfig {
    /// Test environment name
    pub environment: String,

    /// Enable verbose output
    pub verbose: bool,

    /// Enable parallel test execution
    pub parallel_execution: bool,

    /// Maximum number of parallel test threads
    pub max_threads: usize,

    /// Test timeout in seconds
    pub timeout_seconds: u64,

    /// Enable test coverage collection
    pub collect_coverage: bool,

    /// Test data directory
    pub test_data_dir: PathBuf,

    /// Enable property-based testing
    pub property_testing_enabled: bool,

    /// Number of property test iterations
    pub property_test_iterations: u32,

    /// Enable fuzzing tests
    pub fuzzing_enabled: bool,

    /// Random seed for reproducible tests
    pub random_seed: Option<u64>,
}

impl CanonicalTestConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_parsed;

        Self {
            environment: "test".to_string(),
            verbose: false,
            parallel_execution: true,
            max_threads: std::thread::available_parallelism()
                .map(std::num::NonZeroUsize::get)
                .unwrap_or(4),
            timeout_seconds: get_parsed(source, "BEARDOG_TEST_TIMEOUT_SECS", 300),
            collect_coverage: false,
            test_data_dir: PathBuf::from("test-data"),
            property_testing_enabled: true,
            property_test_iterations: get_parsed(source, "BEARDOG_TEST_PROPERTY_ITERATIONS", 100),
            fuzzing_enabled: false,
            random_seed: None,
        }
    }
}

impl Default for CanonicalTestConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl CanonicalTestConfig {
    /// Create a new test configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Create configuration for fast tests
    pub fn fast() -> Self {
        let default_threads = std::thread::available_parallelism()
            .map(std::num::NonZeroUsize::get)
            .unwrap_or(4);
        Self {
            parallel_execution: true,
            max_threads: default_threads * 2,
            timeout_seconds: std::env::var("BEARDOG_TEST_FAST_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            property_test_iterations: std::env::var("BEARDOG_TEST_FAST_PROPERTY_ITERATIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            ..Default::default()
        }
    }

    /// Create configuration for thorough testing
    pub fn thorough() -> Self {
        Self {
            parallel_execution: true,
            collect_coverage: true,
            property_testing_enabled: true,
            property_test_iterations: std::env::var("BEARDOG_TEST_THOROUGH_PROPERTY_ITERATIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000),
            fuzzing_enabled: true,
            timeout_seconds: std::env::var("BEARDOG_TEST_THOROUGH_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(600),
            ..Default::default()
        }
    }

    /// Validate the configuration
    ///
    /// # Errors
    ///
    /// Returns an error if `max_threads` or `timeout_seconds` is zero.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.max_threads == 0 {
            return Err(beardog_errors::BearDogError::validation(
                "max_threads must be greater than 0",
            ));
        }
        if self.timeout_seconds == 0 {
            return Err(beardog_errors::BearDogError::validation(
                "timeout_seconds must be greater than 0",
            ));
        }
        Ok(())
    }
}

// ============================================================================
// API TEST CONFIGURATION
// ============================================================================

/// **Canonical API Test Configuration** - API testing configuration
///
/// Consolidates API endpoint testing, integration testing, and contract testing settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalApiTestConfig {
    /// Base URL for API tests
    pub base_url: String,

    /// API test timeout in seconds
    pub timeout_seconds: u64,

    /// Enable API contract validation
    pub contract_validation: bool,

    /// Enable authentication in tests
    pub use_authentication: bool,

    /// Test API key
    pub api_key: Option<String>,

    /// Enable TLS/SSL verification
    pub verify_tls: bool,

    /// Maximum concurrent API requests
    pub max_concurrent_requests: usize,

    /// Retry failed requests
    pub retry_failed_requests: bool,

    /// Maximum retry attempts
    pub max_retries: u32,

    /// Request delay between retries (milliseconds)
    pub retry_delay_ms: u64,

    /// Enable response caching for tests
    pub cache_responses: bool,

    /// Test user credentials
    pub test_credentials: Option<TestCredentials>,
}

impl CanonicalApiTestConfig {
    /// Default timeout in seconds
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

    /// Default maximum concurrent requests
    pub const DEFAULT_MAX_CONCURRENT: usize = 10;

    /// Default maximum retry attempts
    pub const DEFAULT_MAX_RETRIES: u32 = 3;

    /// Default retry delay in milliseconds
    pub const DEFAULT_RETRY_DELAY_MS: u64 = 1000;

    /// Create `CanonicalApiTestConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub fn with_defaults() -> Self {
        use beardog_config::domains::network_ports::DEFAULT_API_PORT;

        Self {
            base_url: format!("http://localhost:{DEFAULT_API_PORT}"),
            timeout_seconds: Self::DEFAULT_TIMEOUT_SECS,
            contract_validation: true,
            use_authentication: false,
            api_key: None,
            verify_tls: true,
            max_concurrent_requests: Self::DEFAULT_MAX_CONCURRENT,
            retry_failed_requests: false,
            max_retries: Self::DEFAULT_MAX_RETRIES,
            retry_delay_ms: Self::DEFAULT_RETRY_DELAY_MS,
            cache_responses: false,
            test_credentials: None,
        }
    }

    /// Create `CanonicalApiTestConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_API_TEST_TIMEOUT_SECS`: Timeout in seconds (default: 30)
    /// - `BEARDOG_API_TEST_MAX_CONCURRENT`: Max concurrent requests (default: 10)
    /// - `BEARDOG_API_TEST_MAX_RETRIES`: Max retry attempts (default: 3)
    /// - `BEARDOG_API_TEST_RETRY_DELAY_MS`: Retry delay in ms (default: 1000)
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        use beardog_config::domains::network_ports::DEFAULT_API_PORT;

        Self {
            base_url: format!("http://localhost:{DEFAULT_API_PORT}"),
            timeout_seconds: get("BEARDOG_API_TEST_TIMEOUT_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_TIMEOUT_SECS),
            contract_validation: true,
            use_authentication: false,
            api_key: None,
            verify_tls: true,
            max_concurrent_requests: get("BEARDOG_API_TEST_MAX_CONCURRENT")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_CONCURRENT),
            retry_failed_requests: false,
            max_retries: get("BEARDOG_API_TEST_MAX_RETRIES")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_RETRIES),
            retry_delay_ms: get("BEARDOG_API_TEST_RETRY_DELAY_MS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_RETRY_DELAY_MS),
            cache_responses: false,
            test_credentials: None,
        }
    }

    /// Create a new API test configuration
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            ..Default::default()
        }
    }

    /// Create configuration for local testing
    pub fn local() -> Self {
        use beardog_config::domains::network_ports::DEFAULT_API_PORT;

        Self {
            base_url: format!("http://localhost:{DEFAULT_API_PORT}"),
            verify_tls: false,
            ..Default::default()
        }
    }

    /// Create configuration for production testing
    pub fn production() -> Self {
        Self {
            base_url: "https://api.production.example.com".to_string(),
            timeout_seconds: std::env::var("BEARDOG_E2E_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60),
            verify_tls: true,
            use_authentication: true,
            retry_failed_requests: true,
            ..Default::default()
        }
    }
}

impl Default for CanonicalApiTestConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// Test credentials for API testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCredentials {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// Optional token
    pub token: Option<String>,
}

// ============================================================================
// BENCHMARK CONFIGURATION
// ============================================================================

/// **Canonical Benchmark Configuration** - Performance benchmark configuration
///
/// Consolidates benchmark and performance testing settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalBenchmarkConfig {
    /// Benchmark name
    pub name: String,

    /// Number of iterations per benchmark
    pub iterations: usize,

    /// Warmup iterations before measurement
    pub warmup_iterations: usize,

    /// Measurement duration
    pub measurement_duration: Duration,

    /// Enable statistical analysis
    pub statistical_analysis: bool,

    /// Confidence level for analysis (0.0 - 1.0)
    pub confidence_level: f64,

    /// Enable comparison with baseline
    pub compare_baseline: bool,

    /// Baseline file path
    pub baseline_path: Option<PathBuf>,

    /// Enable memory profiling
    pub memory_profiling: bool,

    /// Enable CPU profiling
    pub cpu_profiling: bool,

    /// Output format (json, csv, text)
    pub output_format: String,

    /// Output directory for results
    pub output_dir: PathBuf,
}

impl CanonicalBenchmarkConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_parsed;
        use std::time::Duration;

        Self {
            name: "benchmark".to_string(),
            iterations: get_parsed(source, "BEARDOG_BENCHMARK_ITERATIONS", 1000),
            warmup_iterations: get_parsed(source, "BEARDOG_BENCHMARK_WARMUP_ITERATIONS", 100),
            measurement_duration: Duration::from_secs(get_parsed(
                source,
                "BEARDOG_BENCHMARK_MEASUREMENT_DURATION_SECS",
                10,
            )),
            statistical_analysis: true,
            confidence_level: std::env::var("BEARDOG_BENCHMARK_CONFIDENCE_LEVEL")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.95),
            compare_baseline: false,
            baseline_path: None,
            memory_profiling: false,
            cpu_profiling: false,
            output_format: "json".to_string(),
            output_dir: PathBuf::from("benchmark-results"),
        }
    }
}

impl Default for CanonicalBenchmarkConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl CanonicalBenchmarkConfig {
    /// Create a new benchmark configuration
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    /// Create configuration for quick benchmarks
    pub fn quick() -> Self {
        Self {
            iterations: std::env::var("BEARDOG_BENCHMARK_QUICK_ITERATIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            warmup_iterations: std::env::var("BEARDOG_BENCHMARK_QUICK_WARMUP_ITERATIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            measurement_duration: Duration::from_secs(
                std::env::var("BEARDOG_BENCHMARK_QUICK_DURATION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(5),
            ),
            statistical_analysis: false,
            ..Default::default()
        }
    }

    /// Create configuration for thorough benchmarks
    pub fn thorough() -> Self {
        Self {
            iterations: std::env::var("BEARDOG_BENCHMARK_THOROUGH_ITERATIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10000),
            warmup_iterations: std::env::var("BEARDOG_BENCHMARK_THOROUGH_WARMUP")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000),
            measurement_duration: Duration::from_secs(60),
            statistical_analysis: true,
            memory_profiling: true,
            cpu_profiling: true,
            compare_baseline: true,
            ..Default::default()
        }
    }
}

// ============================================================================
// PRODUCTION TEST CONFIGURATION
// ============================================================================

/// **Canonical Production Test Configuration** - Production deployment testing
///
/// Consolidates production validation and deployment verification settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalProductionTestConfig {
    /// Environment name
    pub environment: String,

    /// Enable smoke tests
    pub smoke_tests_enabled: bool,

    /// Enable health checks
    pub health_checks_enabled: bool,

    /// Enable security validation
    pub security_validation_enabled: bool,

    /// Enable performance validation
    pub performance_validation_enabled: bool,

    /// Enable compliance checks
    pub compliance_checks_enabled: bool,

    /// Health check endpoint
    pub health_check_endpoint: String,

    /// Health check timeout
    pub health_check_timeout_seconds: u64,

    /// Required services for deployment
    pub required_services: Vec<String>,

    /// Enable canary deployment testing
    pub canary_testing_enabled: bool,

    /// Canary traffic percentage
    pub canary_percentage: f64,

    /// Rollback on failure
    pub rollback_on_failure: bool,
}

impl CanonicalProductionTestConfig {
    /// Default health check timeout in seconds
    pub const DEFAULT_HEALTH_TIMEOUT_SECS: u64 = 30;

    /// Default canary percentage
    pub const DEFAULT_CANARY_PERCENTAGE: f64 = 5.0;

    /// Create `CanonicalProductionTestConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub fn with_defaults() -> Self {
        Self {
            environment: "production".to_string(),
            smoke_tests_enabled: true,
            health_checks_enabled: true,
            security_validation_enabled: true,
            performance_validation_enabled: true,
            compliance_checks_enabled: true,
            health_check_endpoint: "/health".to_string(),
            health_check_timeout_seconds: Self::DEFAULT_HEALTH_TIMEOUT_SECS,
            required_services: Vec::new(),
            canary_testing_enabled: false,
            canary_percentage: Self::DEFAULT_CANARY_PERCENTAGE,
            rollback_on_failure: true,
        }
    }

    /// Create `CanonicalProductionTestConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_PROD_TEST_HEALTH_TIMEOUT_SECS`: Health check timeout (default: 30)
    /// - `BEARDOG_CANARY_PERCENTAGE`: Canary deployment percentage (default: 5.0)
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            environment: "production".to_string(),
            smoke_tests_enabled: true,
            health_checks_enabled: true,
            security_validation_enabled: true,
            performance_validation_enabled: true,
            compliance_checks_enabled: true,
            health_check_endpoint: "/health".to_string(),
            health_check_timeout_seconds: get("BEARDOG_PROD_TEST_HEALTH_TIMEOUT_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_HEALTH_TIMEOUT_SECS),
            required_services: Vec::new(),
            canary_testing_enabled: false,
            canary_percentage: get("BEARDOG_CANARY_PERCENTAGE")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_CANARY_PERCENTAGE),
            rollback_on_failure: true,
        }
    }
}

impl Default for CanonicalProductionTestConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl CanonicalProductionTestConfig {
    /// Create a new production test configuration
    pub fn new(environment: String) -> Self {
        Self {
            environment,
            ..Default::default()
        }
    }

    /// Create minimal production test configuration
    pub fn minimal() -> Self {
        Self {
            smoke_tests_enabled: true,
            health_checks_enabled: true,
            security_validation_enabled: false,
            performance_validation_enabled: false,
            compliance_checks_enabled: false,
            ..Default::default()
        }
    }

    /// Create comprehensive production test configuration
    pub fn comprehensive() -> Self {
        Self {
            smoke_tests_enabled: true,
            health_checks_enabled: true,
            security_validation_enabled: true,
            performance_validation_enabled: true,
            compliance_checks_enabled: true,
            canary_testing_enabled: true,
            ..Default::default()
        }
    }
}

// ============================================================================
// TYPE ALIASES FOR BACKWARD COMPATIBILITY
// ============================================================================

/// Backward compatibility alias
pub type TestConfig = CanonicalTestConfig;

/// Backward compatibility alias
pub type ApiTestConfig = CanonicalApiTestConfig;

/// Backward compatibility alias  
pub type BenchmarkConfig = CanonicalBenchmarkConfig;

/// Backward compatibility alias
pub type ProductionTestConfig = CanonicalProductionTestConfig;

/// Backward compatibility alias (old name from `world_class_testing_framework`)
pub type TestingConfiguration = CanonicalTestConfig;

// ============================================================================
// MODULE EXPORTS
// ============================================================================

/// Re-export all canonical test types
pub mod prelude {
    pub use super::{
        CanonicalApiTestConfig, CanonicalBenchmarkConfig, CanonicalProductionTestConfig,
        CanonicalTestConfig, TestCredentials,
    };
}
