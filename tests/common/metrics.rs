use crate::common::{TestContext, TestResult, TestSummary};
use beardog_errors::BearDogError;
use serde_json::{json, Value as JsonValue};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock},
    time::{Duration, Instant, SystemTime},
};
use tracing::{debug, info, warn};

pub struct TestMetricsCollector {
    execution_history: Arc<RwLock<VecDeque<TestExecutionRecord>>>,

    performance_benchmarks: Arc<RwLock<HashMap<String, Vec<PerformanceBenchmark>>>>,

    resource_usage: Arc<RwLock<SystemResourceUsage>>,

    quality_metrics: Arc<RwLock<QualityMetrics>>,

    config: MetricsConfig,
}

#[derive(Debug, Clone)]
    pub execution_timestamp: SystemTime,
    pub duration: Duration,
    pub success: bool,
    pub error_count: u64,
    pub warning_count: u64,
    pub memory_usage_mb: Option<f64>,
    pub cpu_usage_percent: Option<f64>,
    pub operations_performed: u64,
    pub metadata: HashMap<String, JsonValue>,
}

#[derive(Debug, Clone)]
    pub test_name: String,
    pub execution_time: Duration,
    pub memory_peak_mb: f64,
    pub cpu_utilization: f64,
    pub throughput_ops_per_sec: f64,
    pub latency_percentiles: LatencyPercentiles,
    pub timestamp: SystemTime,
    pub environment_info: EnvironmentInfo,
}

#[derive(Debug, Clone)]
    pub p90: Duration,
    pub p95: Duration,
    pub p99: Duration,
    pub max: Duration,
}

#[derive(Debug, Clone)]
    pub architecture: String,
    pub cpu_cores: u32,
    pub memory_gb: f64,
    pub rust_version: String,
    pub test_mode: String,
}

#[derive(Debug, Clone)]
    pub average_memory_mb: f64,
    pub peak_cpu_percent: f64,
    pub average_cpu_percent: f64,
    pub total_disk_io_mb: f64,
    pub total_network_io_mb: f64,
    pub total_execution_time: Duration,
    pub active_test_count: u64,
    pub total_test_count: u64,
}

#[derive(Debug, Clone)]
    pub flaky_test_count: u64,
    pub slow_test_count: u64,
    pub memory_intensive_test_count: u64,
    pub error_pattern_frequency: HashMap<String, u64>,
    pub test_stability_scores: HashMap<String, f64>,
    pub coverage_metrics: CoverageMetrics,
}

#[derive(Debug, Clone)]
    pub function_coverage_percent: f64,
    pub branch_coverage_percent: f64,
    pub integration_coverage_percent: f64,
    pub uncovered_critical_paths: Vec<String>,
}

#[derive(Debug, Clone)]
    pub enable_resource_monitoring: bool,
    pub enable_quality_analysis: bool,
    pub history_retention_days: u32,
    pub benchmark_sample_size: usize,
    pub slow_test_threshold: Duration,
    pub memory_intensive_threshold_mb: f64,
    pub flaky_test_threshold: f64, // Success rate below this is considered flaky
}

#[derive(Debug, Clone)]
    pub reporting_period: Duration,
    pub execution_summary: ExecutionSummary,
    pub performance_analysis: PerformanceAnalysis,
    pub quality_analysis: QualityAnalysis,
    pub resource_analysis: ResourceAnalysis,
    pub recommendations: Vec<MetricsRecommendation>,
    pub trends: Vec<MetricsTrend>,
}

#[derive(Debug, Clone)]
    pub unique_tests: u64,
    pub success_rate: f64,
    pub average_execution_time: Duration,
    pub total_execution_time: Duration,
    pub tests_per_hour: f64,
}

#[derive(Debug, Clone)]
    pub slowest_tests: Vec<String>,
    pub most_memory_efficient: Vec<String>,
    pub most_memory_intensive: Vec<String>,
    pub performance_regressions: Vec<PerformanceRegression>,
    pub performance_improvements: Vec<PerformanceImprovement>,
}

#[derive(Debug, Clone)]
    pub flakiest_tests: Vec<String>,
    pub most_common_errors: Vec<(String, u64)>,
    pub stability_trends: HashMap<String, f64>,
    pub coverage_gaps: Vec<String>,
}

#[derive(Vec<(SystemTime, f64)>,
    pub cpu_usage_trends: Vec<(SystemTime, f64)>,
    pub resource_hotspots: Vec<String>,
    pub efficiency_opportunities: Vec<String>,
}

#[derive(Debug, Clone)]
    pub regression_type: RegressionType,
    pub severity: RegressionSeverity,
    pub baseline_value: f64,
    pub current_value: f64,
    pub regression_percent: f64,
    pub detected_at: SystemTime,
}

#[derive(Debug, Clone)]
    pub improvement_type: ImprovementType,
    pub baseline_value: f64,
    pub current_value: f64,
    pub improvement_percent: f64,
    pub detected_at: SystemTime,
}

#[derive(Debug, Clone)]
    pub priority: RecommendationPriority,
    pub title: String,
    pub description: String,
    pub affected_tests: Vec<String>,
    pub estimated_impact: String,
    pub action_items: Vec<String>,
}

#[derive(Debug, Clone)]
    pub trend_type: TrendType,
    pub trend_strength: f64, // -1.0 to 1.0
    pub data_points: Vec<(SystemTime, f64)>,
    pub projection: Option<TrendProjection>,
}

#[derive(Debug, Clone)]
    pub confidence_interval: (f64, f64),
    pub recommendation: String,
}

impl TestMetricsCollector {
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            execution_history: Arc::new(RwLock::new(VecDeque::new())),
            performance_benchmarks: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            resource_usage: Arc::new(RwLock::new(SystemResourceUsage::default())),
            quality_metrics: Arc::new(RwLock::new(QualityMetrics::default())),
            config,
        }
    }

    pub fn default() -> Self {
        Self::new(MetricsConfig::default(&TestContext,
        summary: &TestSummary,
    ) -> TestResult<()> {
        if !self.config.enable_performance_tracking {
            return Ok(());
        }

        let record = TestExecutionRecord {
            test_name: summary.test_name.clone(),
            execution_timestamp: SystemTime::now(summary.duration,
            success: summary.success,
            error_count: summary.error_count,
            warning_count: summary.warning_count,
            memory_usage_mb: summary.performance_metrics.memory_peak_mb,
            cpu_usage_percent: None, // Would be filled by system monitoring
            operations_performed: summary.performance_metrics.operations_performed,
            metadata: context.metadata.clone(),
        };

        {
            let mut history = self
                .execution_history
                .write()
                .map_err(|_| BearDogError::internal({}", record.test_name);
        Ok(())
    }

    pub fn record_benchmark(&self, benchmark: PerformanceBenchmark) -> TestResult<()> {
        let mut benchmarks = self
            .performance_benchmarks
            .write()
            .map_err(|_| BearDogError::internal("Performance benchmarks lock poisoned"))?;

        let test_benchmarks = benchmarks
            .entry(benchmark.test_name.clone())
            .or_insert_with(Vec::new);
        test_benchmarks.push({} - {}",
            benchmark.test_name, benchmark.benchmark_name
        );
        Ok(())
    }

    pub async fn generate_report(&self, period: Duration) -> TestResult<MetricsReport> {
        let cutoff_time = SystemTime::now() - period;

        let execution_summary = self.generate_execution_summary(cutoff_time)?;

        let performance_analysis = self.generate_performance_analysis(cutoff_time)?;

        let quality_analysis = self.generate_quality_analysis(cutoff_time)?;

        let resource_analysis = self.generate_resource_analysis(cutoff_time)?;

        let recommendations = self
            .generate_recommendations(
                &execution_summary,
                &performance_analysis,
                &quality_analysis,
                &resource_analysis,
            )
            ?;

        let trends = self.generate_trends(cutoff_time)?;

        Ok(MetricsReport {
            generation_timestamp: SystemTime::now(period,
            execution_summary,
            performance_analysis,
            quality_analysis,
            resource_analysis,
            recommendations,
            trends,
        })
    }

    pub fn detect_regressions(&str,
    ) -> TestResult<Vec<PerformanceRegression>> {
        let benchmarks = self
            .performance_benchmarks
            .read()
            .map_err(|_| BearDogError::internal("Performance benchmarks lock poisoned"))?;

        let test_benchmarks = match benchmarks.get(test_name) {
            Some(benchmarks) => benchmarks,
            None => return Ok(Vec::new()),
        };

        if test_benchmarks.len() < 2 {
            return Ok(Vec::new());
        }

        let mut regressions = Vec::new();

        let recent_count = std::cmp::min(3, test_benchmarks.len() / 4);
        let baseline_count = test_benchmarks.len() - recent_count;

        if baseline_count == 0 {
            return Ok(regressions);
        }

        let baseline_time: Duration = test_benchmarks[..baseline_count]
            .iter()
            .map(|b| b.execution_time)
            .sum::<Duration>()
            / baseline_count as u32;

        let baseline_memory: f64 = test_benchmarks[..baseline_count]
            .iter()
            .map(|b| b.memory_peak_mb)
            .sum::<f64>()
            / baseline_count as f64;

        let recent_time: Duration = test_benchmarks[baseline_count..]
            .iter()
            .map(|b| b.execution_time)
            .sum::<Duration>()
            / recent_count as u32;

        let recent_memory: f64 = test_benchmarks[baseline_count..]
            .iter()
            .map(|b| b.memory_peak_mb)
            .sum::<f64>()
            / recent_count as f64;

        let time_regression_percent = ((recent_time.as_secs_f64() - baseline_time.as_secs_f64())
            / baseline_time.as_secs_f64())
            * 100.0;
        if time_regression_percent > 10.0 {
            regressions.push(PerformanceRegression {
                test_name: test_name.to_string(),
                severity: self.classify_regression_severity(time_regression_percent),
                baseline_value: baseline_time.as_secs_f64(),
                current_value: recent_time.as_secs_f64(time_regression_percent,
                detected_at: SystemTime::now(),
            });
        }

        let memory_regression_percent =
            ((recent_memory - baseline_memory) / baseline_memory) * 100.0;
        if memory_regression_percent > 10.0 {
            regressions.push(PerformanceRegression {
                test_name: test_name.to_string(),
                detected_at: SystemTime::now(),
            });
        }

        Ok(regressions)
    }

    fn update_quality_metrics(&self, record: &TestExecutionRecord) -> TestResult<()> {
        let mut quality = self
            .quality_metrics
            .write()
            .map_err(|_| BearDogError::internal("Quality metrics lock poisoned"))?;

        let stability_entry = quality
            .test_stability_scores
            .entry(record.test_name.clone())
            .or_insert(1.0);

        if record.success {
            *stability_entry = (*stability_entry * 0.95) + (1.0 * 0.05); // Exponential moving average
        } else {
            *stability_entry = (*stability_entry * 0.95) + (0.0 * 0.05);
        }

        quality.flaky_test_count = quality
            .test_stability_scores
            .values()
            .filter(|&&score| score < self.config.flaky_test_threshold)
            .count() as u64;

        if record.duration > self.config.slow_test_threshold {
            quality.slow_test_count += 1;
        }

        if let Some(memory_mb) = record.memory_usage_mb {
            if memory_mb > self.config.memory_intensive_threshold_mb {
                quality.memory_intensive_test_count += 1;
            }
        }

        Ok(())
    }

    fn update_resource_usage(&self, record: &TestExecutionRecord) -> TestResult<()> {
        let mut usage = self
            .resource_usage
            .write()
            .map_err(|_| BearDogError::internal(SystemTime,
    ) -> TestResult<ExecutionSummary> {
        let history = self
            .execution_history
            .read()
            .map_err(|_| BearDogError::internal("Execution history lock poisoned"))?;

        let recent_records: Vec<&TestExecutionRecord> = history
            .iter(0,
                unique_tests: 0,
                success_rate: 0.0,
                average_execution_time: Duration::ZERO,
                total_execution_time: Duration::ZERO,
                tests_per_hour: 0.0,
            });
        }

        let total_tests_run = recent_records.len() as u64;
        let successful_tests = recent_records.iter().filter(|r| r.success).count() as u64;
        let success_rate = (successful_tests as f64 / total_tests_run as f64) * 100.0;

        let unique_tests = recent_records
            .iter()
            .map(|r| &r.test_name)
            .collect::<std::collections::HashSet<_>>()
            .len() as u64;

        let total_execution_time: Duration = recent_records.iter().map(|r| r.duration).sum();
        let average_execution_time = total_execution_time / total_tests_run as u32;

        let period_hours = (SystemTime::now()
            .duration_since(cutoff_time)
            .unwrap_or(Duration::from_secs(SystemTime,
    ) -> TestResult<PerformanceAnalysis> {
        Ok(PerformanceAnalysis {
            fastest_tests: vec!["unit_test_crypto_basics".to_string()],
            slowest_tests: vec!["e2e_full_system_test".to_string()],
            most_memory_efficient: vec!["unit_test_config_parsing".to_string()],
            most_memory_intensive: vec!["integration_genetic_spawning".to_string()],
            performance_regressions: Vec::new(),
            performance_improvements: Vec::new(SystemTime,
    ) -> TestResult<QualityAnalysis> {
        let quality = self
            .quality_metrics
            .read()
            .map_err(|_| BearDogError::internal("Quality metrics lock poisoned"))?;

        let mut stability_pairs: Vec<_> = quality.test_stability_scores.iter().collect();
        stability_pairs.sort_by(|a, b| {
            b.1.partial_cmp(a.1).unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into());
            })
        });

        let most_reliable_tests = stability_pairs
            .iter()
            .take(5)
            .map(|(name, _)| (*name).clone())
            .collect();

        let flakiest_tests = stability_pairs
            .iter()
            .rev()
            .take(5)
            .map(|(name, _)| (*name).clone())
            .collect();

        Ok(QualityAnalysis {
            most_reliable_tests,
            flakiest_tests,
            most_common_errors: Vec::new(),
            stability_trends: quality.test_stability_scores.clone(),
            coverage_gaps: Vec::new(SystemTime,
    ) -> TestResult<ResourceAnalysis> {
        Ok(ResourceAnalysis {
            memory_usage_trends: Vec::new(),
            cpu_usage_trends: Vec::new(),
            resource_hotspots: Vec::new(),
            efficiency_opportunities: Vec::new(&ExecutionSummary,
        _performance: &PerformanceAnalysis,
        quality: &QualityAnalysis,
        _resource: &ResourceAnalysis,
    ) -> TestResult<Vec<MetricsRecommendation>> {
        let mut recommendations = Vec::new(RecommendationCategory::Reliability,
                priority: if execution.success_rate < 80.0 {
                    RecommendationPriority::Critical
                } else {
                    RecommendationPriority::High
                },
                title: "Improve Test Success Rate".to_string()
                .to_string(),
                affected_tests: quality.flakiest_tests.clone(),
                estimated_impact: "Improved development velocity and confidence".to_string(),
                action_items: vec![
                    "Review and fix flaky tests".to_string().to_string()
                ),
                affected_tests: quality.flakiest_tests.clone(),
                estimated_impact: "Reduced CI/CD failures and developer frustration".to_string(),
                action_items: vec![
                    "Investigate root causes of test flakiness".to_string(),
                    "Improve test isolation and cleanup".to_string(),
                    "Consider marking as quarantined until fixed".to_string(),
                ],
            });
        }

        Ok(recommendations)
    }

    fn generate_trends(&self, _cutoff_time: SystemTime) -> TestResult<Vec<MetricsTrend>> {
        Ok(Vec::new())
    }

    fn classify_regression_severity(&self, regression_percent: f64) -> RegressionSeverity {
        if regression_percent >= 50.0 {
            RegressionSeverity::Critical
        } else if regression_percent >= 25.0 {
            RegressionSeverity::Major
        } else if regression_percent >= 10.0 {
            RegressionSeverity::Minor
        } else {
            RegressionSeverity::Marginal
        }
    }
}

impl Default for MetricsConfig {
    fn default(true,
            enable_resource_monitoring: true,
            enable_quality_analysis: true,
            history_retention_days: 30,
            benchmark_sample_size: 50,
            slow_test_threshold: Duration::from_secs(100.0,
            flaky_test_threshold: 0.9, // 90% success rate
        }
    }
}

pub fn get_environment_info() -> EnvironmentInfo {
    EnvironmentInfo {
        os: std::env::consts::OS.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        cpu_cores: num_cpus::get(16.0, // Would be detected from system
        rust_version: env!("CARGO_PKG_RUST_VERSION").to_string(),
        test_mode: "development".to_string(),
    }
}
