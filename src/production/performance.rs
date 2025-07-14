//! Performance monitoring and benchmarking
//!
//! This module provides performance monitoring capabilities, including
//! metrics collection, benchmarking, load testing, regression detection,
//! and performance optimization recommendations.

/// Performance metrics collection
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Request throughput in requests per second
    pub request_throughput: f64,
    /// Average response time in milliseconds
    pub average_response_time_ms: f64,
    /// Number of cryptographic operations per second
    pub crypto_operations_per_second: f64,
    /// Memory usage in megabytes
    pub memory_usage_mb: f64,
    /// CPU utilization percentage
    pub cpu_usage_percent: f64,
}

/// Benchmark test results
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    /// Encryption latency in microseconds
    pub encryption_latency_us: u64,
    /// Key generation time in milliseconds
    pub key_generation_time_ms: u64,
    /// Signature verification time in microseconds
    pub signature_verification_time_us: u64,
    /// Operations per second
    pub operations_per_second: u64,
    /// Number of concurrent sessions
    pub concurrent_sessions: u32,
}

/// Performance regression detection
#[derive(Debug, Clone)]
pub struct PerformanceRegressionCheck {
    /// Whether performance regression was detected compared to baseline
    pub regression_detected: bool,
    /// Whether baseline comparison data is valid and reliable
    pub baseline_comparison_valid: bool,
    /// Performance trends observed during testing
    pub performance_trends: Vec<String>,
}

/// Load testing configuration
#[derive(Debug, Clone)]
pub struct LoadTestConfiguration {
    /// Number of concurrent users to simulate during load testing
    pub concurrent_users: u32,
    /// Duration of the load test in seconds
    pub test_duration_seconds: u64,
    /// Time to ramp up to full load in seconds
    pub ramp_up_time_seconds: u64,
    /// Target operations per second to achieve during testing
    pub target_operations_per_second: u64,
}

/// Load testing results
#[derive(Debug, Clone)]
pub struct LoadTestResults {
    /// Whether the test completed successfully
    pub test_completed_successfully: bool,
    /// Whether target throughput was achieved
    pub target_throughput_achieved: bool,
    /// Error rate during the test
    pub error_rate: f64,
    /// 95th percentile response time in milliseconds
    pub p95_response_time_ms: u64,
}

/// Performance optimization recommendations
#[derive(Debug, Clone)]
pub struct PerformanceRecommendations {
    /// List of optimization recommendations
    pub recommendations: Vec<OptimizationRecommendation>,
}

/// Individual optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    /// Category of optimization (CPU, Memory, Network, etc.)
    pub category: String,
    /// Description of the recommendation
    pub description: String,
    /// Impact score (0.0 to 1.0) indicating potential improvement
    pub impact_score: f64,
}

/// Stress testing configuration
#[derive(Debug, Clone)]
pub struct StressTestConfiguration {
    /// CPU stress level as percentage (0-100)
    pub cpu_stress_percentage: u8,
    /// Memory stress level as percentage (0-100)
    pub memory_stress_percentage: u8,
    /// Network stress in megabits per second
    pub network_stress_mbps: u32,
    /// Number of concurrent operations to simulate
    pub concurrent_operations: u32,
    /// Duration of stress test in seconds
    pub stress_duration_seconds: u64,
}

/// Stress testing results
#[derive(Debug, Clone)]
pub struct StressTestResults {
    /// Whether the system remained stable throughout the test
    pub system_remained_stable: bool,
    /// Whether performance degradation was within acceptable limits
    pub performance_degradation_acceptable: bool,
    /// Whether error rates remained within acceptable limits
    pub error_rate_within_limits: bool,
    /// Whether recovery time after stress was acceptable
    pub recovery_time_acceptable: bool,
}

/// Resource exhaustion testing
#[derive(Debug, Clone)]
pub struct ResourceExhaustionTest {
    /// Whether graceful degradation mechanisms are functional
    pub graceful_degradation_functional: bool,
    /// Whether critical operations are preserved under stress
    pub critical_operations_preserved: bool,
    /// Whether recovery procedures are effective
    pub recovery_procedures_effective: bool,
}

/// Cascade failure prevention testing
#[derive(Debug, Clone)]
pub struct CascadePreventionTest {
    /// Whether circuit breakers are functioning correctly
    pub circuit_breakers_functional: bool,
    /// Whether isolation mechanisms are effective
    pub isolation_mechanisms_effective: bool,
    /// Whether system resilience is maintained
    pub system_resilience_maintained: bool,
}

impl PerformanceMetrics {
    /// Create new performance metrics
    pub fn new() -> Self {
        Self {
            request_throughput: 0.0,
            average_response_time_ms: 0.0,
            crypto_operations_per_second: 0.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
        }
    }

    /// Update metrics with new values
    pub fn update(
        &mut self,
        throughput: f64,
        response_time: f64,
        crypto_ops: f64,
        memory_mb: f64,
        cpu_percent: f64,
    ) {
        self.request_throughput = throughput;
        self.average_response_time_ms = response_time;
        self.crypto_operations_per_second = crypto_ops;
        self.memory_usage_mb = memory_mb;
        self.cpu_usage_percent = cpu_percent;
    }

    /// Check if performance is within acceptable bounds
    pub fn is_acceptable(&self, thresholds: &PerformanceThresholds) -> bool {
        self.request_throughput >= thresholds.min_throughput
            && self.average_response_time_ms <= thresholds.max_response_time_ms
            && self.cpu_usage_percent <= thresholds.max_cpu_percent
            && self.memory_usage_mb <= thresholds.max_memory_mb
    }

    /// Get a performance summary string
    pub fn summary(&self) -> String {
        format!(
            "Throughput: {:.1} req/s, Response Time: {:.1}ms, CPU: {:.1}%, Memory: {:.1}MB",
            self.request_throughput,
            self.average_response_time_ms,
            self.cpu_usage_percent,
            self.memory_usage_mb
        )
    }
}

/// Performance thresholds for validation
#[derive(Debug, Clone)]
pub struct PerformanceThresholds {
    /// Minimum acceptable throughput in requests per second
    pub min_throughput: f64,
    /// Maximum acceptable response time in milliseconds
    pub max_response_time_ms: f64,
    /// Maximum acceptable CPU usage percentage
    pub max_cpu_percent: f64,
    /// Maximum acceptable memory usage in megabytes
    pub max_memory_mb: f64,
}

impl PerformanceThresholds {
    /// Create production performance thresholds
    pub fn production() -> Self {
        Self {
            min_throughput: 100.0,
            max_response_time_ms: 500.0,
            max_cpu_percent: 70.0,
            max_memory_mb: 1024.0,
        }
    }

    /// Create development performance thresholds
    pub fn development() -> Self {
        Self {
            min_throughput: 10.0,
            max_response_time_ms: 2000.0,
            max_cpu_percent: 90.0,
            max_memory_mb: 2048.0,
        }
    }
}

impl BenchmarkResults {
    /// Create new benchmark results
    pub fn new() -> Self {
        Self {
            encryption_latency_us: 0,
            key_generation_time_ms: 0,
            signature_verification_time_us: 0,
            operations_per_second: 0,
            concurrent_sessions: 0,
        }
    }

    /// Update with actual benchmark data
    pub fn update(
        &mut self,
        encryption_latency: u64,
        key_gen_time: u64,
        sig_verify_time: u64,
        ops_per_sec: u64,
        sessions: u32,
    ) {
        self.encryption_latency_us = encryption_latency;
        self.key_generation_time_ms = key_gen_time;
        self.signature_verification_time_us = sig_verify_time;
        self.operations_per_second = ops_per_sec;
        self.concurrent_sessions = sessions;
    }

    /// Check if benchmarks meet minimum requirements
    pub fn meets_requirements(&self, requirements: &BenchmarkRequirements) -> bool {
        self.encryption_latency_us <= requirements.max_encryption_latency_us
            && self.key_generation_time_ms <= requirements.max_key_generation_time_ms
            && self.signature_verification_time_us <= requirements.max_signature_verification_time_us
            && self.operations_per_second >= requirements.min_operations_per_second
    }
}

/// Benchmark requirements for validation
#[derive(Debug, Clone)]
pub struct BenchmarkRequirements {
    /// Maximum acceptable encryption latency in microseconds
    pub max_encryption_latency_us: u64,
    /// Maximum acceptable key generation time in milliseconds
    pub max_key_generation_time_ms: u64,
    /// Maximum acceptable signature verification time in microseconds
    pub max_signature_verification_time_us: u64,
    /// Minimum required operations per second
    pub min_operations_per_second: u64,
}

impl BenchmarkRequirements {
    /// Create production benchmark requirements
    pub fn production() -> Self {
        Self {
            max_encryption_latency_us: 100,
            max_key_generation_time_ms: 50,
            max_signature_verification_time_us: 200,
            min_operations_per_second: 1000,
        }
    }

    /// Create development benchmark requirements
    pub fn development() -> Self {
        Self {
            max_encryption_latency_us: 1000,
            max_key_generation_time_ms: 500,
            max_signature_verification_time_us: 2000,
            min_operations_per_second: 100,
        }
    }
}

impl LoadTestConfiguration {
    /// Create a new load test configuration
    pub fn new(concurrent_users: u32, duration_seconds: u64) -> Self {
        Self {
            concurrent_users,
            test_duration_seconds: duration_seconds,
            ramp_up_time_seconds: duration_seconds / 10, // 10% of test duration for ramp-up
            target_operations_per_second: concurrent_users as u64 * 2, // 2 ops per user per second
        }
    }

    /// Create a production load test configuration
    pub fn production() -> Self {
        Self {
            concurrent_users: 1000,
            test_duration_seconds: 300, // 5 minutes
            ramp_up_time_seconds: 30,
            target_operations_per_second: 2000,
        }
    }

    /// Create a development load test configuration
    pub fn development() -> Self {
        Self {
            concurrent_users: 50,
            test_duration_seconds: 60, // 1 minute
            ramp_up_time_seconds: 10,
            target_operations_per_second: 100,
        }
    }

    /// Get the ramp-up rate in users per second
    pub fn ramp_up_rate(&self) -> f64 {
        if self.ramp_up_time_seconds > 0 {
            self.concurrent_users as f64 / self.ramp_up_time_seconds as f64
        } else {
            self.concurrent_users as f64
        }
    }
}

impl LoadTestResults {
    /// Create new load test results
    pub fn new() -> Self {
        Self {
            test_completed_successfully: false,
            target_throughput_achieved: false,
            error_rate: 0.0,
            p95_response_time_ms: 0,
        }
    }

    /// Update with test results
    pub fn update(
        &mut self,
        completed: bool,
        throughput_achieved: bool,
        error_rate: f64,
        p95_response_time: u64,
    ) {
        self.test_completed_successfully = completed;
        self.target_throughput_achieved = throughput_achieved;
        self.error_rate = error_rate;
        self.p95_response_time_ms = p95_response_time;
    }

    /// Check if the load test passed
    pub fn passed(&self) -> bool {
        self.test_completed_successfully
            && self.target_throughput_achieved
            && self.error_rate <= 0.01 // Max 1% error rate
            && self.p95_response_time_ms <= 1000 // Max 1 second for 95th percentile
    }
}

impl PerformanceRecommendations {
    /// Create new performance recommendations
    pub fn new() -> Self {
        Self {
            recommendations: Vec::new(),
        }
    }

    /// Add a recommendation
    pub fn add_recommendation(
        &mut self,
        category: String,
        description: String,
        impact_score: f64,
    ) {
        self.recommendations.push(OptimizationRecommendation {
            category,
            description,
            impact_score: impact_score.max(0.0).min(1.0),
        });
    }

    /// Get high-impact recommendations (impact score > 0.7)
    pub fn high_impact_recommendations(&self) -> Vec<&OptimizationRecommendation> {
        self.recommendations
            .iter()
            .filter(|rec| rec.impact_score > 0.7)
            .collect()
    }

    /// Get recommendations sorted by impact score (highest first)
    pub fn sorted_by_impact(&self) -> Vec<&OptimizationRecommendation> {
        let mut recommendations: Vec<&OptimizationRecommendation> = self.recommendations.iter().collect();
        recommendations.sort_by(|a, b| b.impact_score.partial_cmp(&a.impact_score).unwrap());
        recommendations
    }

    /// Get the number of recommendations
    pub fn count(&self) -> usize {
        self.recommendations.len()
    }
}

impl OptimizationRecommendation {
    /// Create a new optimization recommendation
    pub fn new(category: String, description: String, impact_score: f64) -> Self {
        Self {
            category,
            description,
            impact_score: impact_score.max(0.0).min(1.0),
        }
    }

    /// Check if this is a high-impact recommendation
    pub fn is_high_impact(&self) -> bool {
        self.impact_score > 0.7
    }

    /// Check if this is a medium-impact recommendation
    pub fn is_medium_impact(&self) -> bool {
        self.impact_score > 0.4 && self.impact_score <= 0.7
    }

    /// Check if this is a low-impact recommendation
    pub fn is_low_impact(&self) -> bool {
        self.impact_score <= 0.4
    }
}

impl PerformanceRegressionCheck {
    /// Create a new regression check
    pub fn new() -> Self {
        Self {
            regression_detected: false,
            baseline_comparison_valid: false,
            performance_trends: Vec::new(),
        }
    }

    /// Update with regression analysis results
    pub fn update(
        &mut self,
        regression_detected: bool,
        baseline_valid: bool,
        trends: Vec<String>,
    ) {
        self.regression_detected = regression_detected;
        self.baseline_comparison_valid = baseline_valid;
        self.performance_trends = trends;
    }

    /// Check if performance has regressed
    pub fn has_regressed(&self) -> bool {
        self.baseline_comparison_valid && self.regression_detected
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BenchmarkResults {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LoadTestResults {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PerformanceRecommendations {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PerformanceRegressionCheck {
    fn default() -> Self {
        Self::new()
    }
} 