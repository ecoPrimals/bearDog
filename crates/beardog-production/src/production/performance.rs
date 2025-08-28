

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {

    pub request_throughput: f64,

    pub average_response_time_ms: f64,

    pub crypto_operations_per_second: f64,

    pub memory_usage_mb: f64,

    pub cpu_usage_percent: f64,
}

pub struct BenchmarkResults {

    pub encryption_latency_us: u64,

    pub key_generation_time_ms: u64,

    pub signature_verification_time_us: u64,

    pub operations_per_second: u64,

    pub concurrent_sessions: u32,

pub struct PerformanceRegressionCheck {

    pub regression_detected: bool,

    pub baseline_comparison_valid: bool,

    pub performance_trends: Vec<String>,

// LoadTestConfiguration removed - use UnifiedTestingConfig instead

    pub target_operations_per_second: u64,

pub struct LoadTestResults {

    pub test_completed_successfully: bool,

    pub target_throughput_achieved: bool,

    pub error_rate: f64,

    pub p95_response_time_ms: u64,

pub struct PerformanceRecommendations {

    pub recommendations: Vec<OptimizationRecommendation>,

pub struct OptimizationRecommendation {

    pub category: String,

    pub description: String,

    pub impact_score: f64,

pub struct StressTestResults {

    pub system_remained_stable: bool,

    pub performance_degradation_acceptable: bool,

    pub error_rate_within_limits: bool,

    pub recovery_time_acceptable: bool,

pub struct ResourceExhaustionTest {

    pub graceful_degradation_functional: bool,

    pub critical_operations_preserved: bool,

    pub recovery_procedures_effective: bool,

pub struct CascadePreventionTest {

    pub circuit_breakers_functional: bool,

    pub isolation_mechanisms_effective: bool,

    pub system_resilience_maintained: bool,}

impl PerformanceMetrics {

    pub fn new() -> Self {
        Self {
            request_throughput: 0.0,
            average_response_time_ms: 0.0,
            crypto_operations_per_second: 0.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
        }
    }

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

    pub fn is_acceptable(&self, thresholds: &PerformanceThresholds) -> bool {
        self.request_throughput >= thresholds.min_throughput
            && self.average_response_time_ms <= thresholds.max_response_time_ms
            && self.cpu_usage_percent <= thresholds.max_cpu_percent
            && self.memory_usage_mb <= thresholds.max_memory_mb

    pub fn summary(&self) -> String {
        format!(
            "Throughput: {:.1} req/s, Response Time: {:.1}ms, CPU: {:.1}%, Memory: {:.1}MB",
            self.request_throughput,
            self.average_response_time_ms,
            self.cpu_usage_percent,
            self.memory_usage_mb
        )

pub struct PerformanceThresholds {

    pub min_throughput: f64,

    pub max_response_time_ms: f64,

    pub max_cpu_percent: f64,

    pub max_memory_mb: f64,}

impl PerformanceThresholds {

    pub fn production() -> Self {
            min_throughput: 100.0,
            max_response_time_ms: 500.0,
            max_cpu_percent: 70.0,
            max_memory_mb: 1024.0,

    pub fn development() -> Self {
            min_throughput: 10.0,
            max_response_time_ms: 2000.0,
            max_cpu_percent: 90.0,
            max_memory_mb: 2048.0,
impl BenchmarkResults {

            encryption_latency_us: 0,
            key_generation_time_ms: 0,
            signature_verification_time_us: 0,
            operations_per_second: 0,
            concurrent_sessions: 0,

        encryption_latency: u64,
        key_gen_time: u64,
        sig_verify_time: u64,
        ops_per_sec: u64,
        sessions: u32,
        self.encryption_latency_us = encryption_latency;
        self.key_generation_time_ms = key_gen_time;
        self.signature_verification_time_us = sig_verify_time;
        self.operations_per_second = ops_per_sec;
        self.concurrent_sessions = sessions;

    pub fn meets_requirements(&self, requirements: &BenchmarkRequirements) -> bool {
        self.encryption_latency_us <= requirements.max_encryption_latency_us
            && self.key_generation_time_ms <= requirements.max_key_generation_time_ms
            && self.signature_verification_time_us
                <= requirements.max_signature_verification_time_us
            && self.operations_per_second >= requirements.min_operations_per_second

pub struct BenchmarkRequirements {

    pub max_encryption_latency_us: u64,

    pub max_key_generation_time_ms: u64,

    pub max_signature_verification_time_us: u64,

    pub min_operations_per_second: u64,}

impl BenchmarkRequirements {

            max_encryption_latency_us: 100,
            max_key_generation_time_ms: 50,
            max_signature_verification_time_us: 200,
            min_operations_per_second: 1000,

            max_encryption_latency_us: 1000,
            max_key_generation_time_ms: 500,
            max_signature_verification_time_us: 2000,
            min_operations_per_second: 100,}

impl LoadTestConfiguration {

    pub fn new(concurrent_users: u32, duration_seconds: u64) -> Self {
            concurrent_users,
            test_duration_seconds: duration_seconds,
            ramp_up_time_seconds: duration_seconds / 10, // 10% of test duration for ramp-up
            target_operations_per_second: concurrent_users as u64 * 2, // 2 ops per user per second

            concurrent_users: 1000,
            test_duration_seconds: 300, // 5 minutes
            ramp_up_time_seconds: 30,
            target_operations_per_second: 2000,

            concurrent_users: 50,
            test_duration_seconds: 60, // 1 minute
            ramp_up_time_seconds: 10,
            target_operations_per_second: 100,

    pub fn ramp_up_rate(&self) -> f64 {
        if self.ramp_up_time_seconds > 0 {
            self.concurrent_users as f64 / self.ramp_up_time_seconds as f64
        } else {
            self.concurrent_users as f64
impl LoadTestResults {

            test_completed_successfully: false,
            target_throughput_achieved: false,
            error_rate: 0.0,
            p95_response_time_ms: 0,

        completed: bool,
        throughput_achieved: bool,
        error_rate: f64,
        p95_response_time: u64,
        self.test_completed_successfully = completed;
        self.target_throughput_achieved = throughput_achieved;
        self.error_rate = error_rate;
        self.p95_response_time_ms = p95_response_time;

    pub fn passed(&self) -> bool {
        self.test_completed_successfully
            && self.target_throughput_achieved
            && self.error_rate <= 0.01 // Max 1% error rate
            && self.p95_response_time_ms <= 1000 // Max 1 second for 95th percentile
impl PerformanceRecommendations {

            recommendations: Vec::new(),

    pub fn add_recommendation(&mut self, category: &str, description: &str, impact_score: f64) {
        self.recommendations.push(OptimizationRecommendation {
            category,
            description,
            impact_score: impact_score.max(0.0).min(1.0),
        });

    pub fn high_impact_recommendations(&self) -> Vec<&OptimizationRecommendation> {
        self.recommendations
            .iter()
            .filter(|rec| rec.impact_score > 0.7)
            .collect()

    pub fn sorted_by_impact(&self) -> Vec<&OptimizationRecommendation> {
        let mut recommendations: Vec<&OptimizationRecommendation> =
            self.recommendations.iter().collect();
        recommendations.sort_by(|a, b| {
            b.impact_score.partial_cmp(&a.impact_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        recommendations

    pub fn count(&self) -> usize {
        self.recommendations.len()
impl OptimizationRecommendation {

    pub fn new(category: &str, description: &str, impact_score: f64) -> Self {

    pub fn is_high_impact(&self) -> bool {
        self.impact_score > 0.7

    pub fn is_medium_impact(&self) -> bool {
        self.impact_score > 0.4 && self.impact_score <= 0.7

    pub fn is_low_impact(&self) -> bool {
        self.impact_score <= 0.4
impl PerformanceRegressionCheck {

            regression_detected: false,
            baseline_comparison_valid: false,
            performance_trends: Vec::new(),

    pub fn update(&mut self, regression_detected: bool, baseline_valid: bool, trends: Vec<&str>) {
        self.regression_detected = regression_detected;
        self.baseline_comparison_valid = baseline_valid;
        self.performance_trends = trends;

    pub fn has_regressed(&self) -> bool {
        self.baseline_comparison_valid && self.regression_detected
impl Default for PerformanceMetrics {}

    fn default() -> Self {
        Self::new()
impl Default for BenchmarkResults {}

impl Default for LoadTestResults {
impl Default for PerformanceRecommendations {}

impl Default for PerformanceRegressionCheck {
