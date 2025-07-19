use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Performance monitoring and optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Performance monitoring configuration
    pub monitoring: PerformanceMonitoringConfig,
    /// Performance targets and thresholds
    pub targets: PerformanceTargetsConfig,
    /// Optimization strategies
    pub optimization: OptimizationStrategiesConfig,
    /// Performance alerting configuration
    pub alerting: PerformanceAlertingConfig,
    /// Benchmarking configuration
    pub benchmarking: BenchmarkingConfig,
}

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    /// Enable performance monitoring
    pub enabled: bool,
    /// Enable detailed metrics collection
    pub enable_detailed_metrics: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Performance metrics to collect
    pub metrics: PerformanceMetricsConfig,
    /// Monitoring storage configuration
    pub storage: MonitoringStorageConfig,
}

/// Performance metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetricsConfig {
    /// Enable latency metrics
    pub latency: bool,
    /// Enable throughput metrics
    pub throughput: bool,
    /// Enable resource utilization metrics
    pub resource_utilization: bool,
    /// Enable error rate metrics
    pub error_rate: bool,
    /// Enable cache hit rate metrics
    pub cache_hit_rate: bool,
    /// Enable queue depth metrics
    pub queue_depth: bool,
    /// Enable custom metrics
    pub custom_metrics: HashMap<String, bool>,
}

/// Monitoring storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringStorageConfig {
    /// Storage backend
    pub backend: StorageBackend,
    /// Data retention period
    pub retention_period: Duration,
    /// Storage compression
    pub enable_compression: bool,
    /// Storage sharding
    pub enable_sharding: bool,
}

/// Storage backend for monitoring data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackend {
    /// In-memory storage
    Memory,
    /// File-based storage
    File { path: String },
    /// Database storage
    Database { url: String },
    /// Time-series database
    TimeSeries { url: String },
}

/// Performance targets and thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTargetsConfig {
    /// Latency targets
    pub latency: LatencyTargetsConfig,
    /// Throughput targets
    pub throughput: ThroughputTargetsConfig,
    /// Resource utilization targets
    pub resource_utilization: ResourceUtilizationTargetsConfig,
    /// Error rate targets
    pub error_rate: ErrorRateTargetsConfig,
}

/// Latency targets configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyTargetsConfig {
    /// P50 latency target
    pub p50_ms: f64,
    /// P95 latency target
    pub p95_ms: f64,
    /// P99 latency target
    pub p99_ms: f64,
    /// P99.9 latency target
    pub p99_9_ms: f64,
    /// Maximum acceptable latency
    pub max_ms: f64,
}

/// Throughput targets configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputTargetsConfig {
    /// Minimum operations per second
    pub min_ops_per_sec: f64,
    /// Target operations per second
    pub target_ops_per_sec: f64,
    /// Maximum operations per second
    pub max_ops_per_sec: f64,
    /// Requests per minute
    pub requests_per_minute: f64,
}

/// Resource utilization targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilizationTargetsConfig {
    /// CPU utilization target (0.0-1.0)
    pub cpu_utilization: f64,
    /// Memory utilization target (0.0-1.0)
    pub memory_utilization: f64,
    /// Disk utilization target (0.0-1.0)
    pub disk_utilization: f64,
    /// Network utilization target (0.0-1.0)
    pub network_utilization: f64,
}

/// Error rate targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRateTargetsConfig {
    /// Maximum error rate (0.0-1.0)
    pub max_error_rate: f64,
    /// Target error rate (0.0-1.0)
    pub target_error_rate: f64,
    /// Critical error rate threshold
    pub critical_error_rate: f64,
}

/// Optimization strategies configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStrategiesConfig {
    /// Enable automatic optimization
    pub enable_auto_optimization: bool,
    /// Optimization strategies
    pub strategies: Vec<OptimizationStrategy>,
    /// Optimization triggers
    pub triggers: OptimizationTriggersConfig,
    /// Optimization cooldown period
    pub cooldown_period: Duration,
}

/// Optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationStrategy {
    /// Scale up resources
    ScaleUp,
    /// Scale down resources
    ScaleDown,
    /// Optimize database queries
    DatabaseOptimization,
    /// Optimize memory usage
    MemoryOptimization,
    /// Optimize async operations
    AsyncOptimization,
    /// Optimize caching
    CacheOptimization,
    /// Load balancing optimization
    LoadBalancing,
}

/// Optimization triggers configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationTriggersConfig {
    /// Latency threshold for triggering optimization
    pub latency_threshold_ms: f64,
    /// Throughput threshold for triggering optimization
    pub throughput_threshold_ops_per_sec: f64,
    /// CPU utilization threshold
    pub cpu_threshold: f64,
    /// Memory utilization threshold
    pub memory_threshold: f64,
    /// Error rate threshold
    pub error_rate_threshold: f64,
}

/// Performance alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlertingConfig {
    /// Enable performance alerting
    pub enabled: bool,
    /// Alert rules
    pub rules: Vec<AlertRule>,
    /// Alert channels
    pub channels: Vec<AlertChannel>,
    /// Alert suppression configuration
    pub suppression: AlertSuppressionConfig,
}

/// Alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Metric to monitor
    pub metric: String,
    /// Threshold value
    pub threshold: f64,
    /// Comparison operator
    pub operator: ComparisonOperator,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert duration before firing
    pub duration: Duration,
}

/// Comparison operator for alert rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
    /// Equal to
    Equal,
    /// Not equal to
    NotEqual,
    /// Greater than or equal to
    GreaterThanOrEqual,
    /// Less than or equal to
    LessThanOrEqual,
}

/// Alert severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Critical alert
    Critical,
    /// High priority alert
    High,
    /// Medium priority alert
    Medium,
    /// Low priority alert
    Low,
    /// Information alert
    Info,
}

/// Alert channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannel {
    /// Channel name
    pub name: String,
    /// Channel type
    pub channel_type: AlertChannelType,
    /// Channel configuration
    pub config: HashMap<String, String>,
}

/// Alert channel type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannelType {
    /// Email alerts
    Email,
    /// Slack alerts
    Slack,
    /// Discord alerts
    Discord,
    /// PagerDuty alerts
    PagerDuty,
    /// Webhook alerts
    Webhook,
    /// SMS alerts
    Sms,
}

/// Alert suppression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSuppressionConfig {
    /// Enable alert suppression
    pub enabled: bool,
    /// Suppression window
    pub window: Duration,
    /// Maximum alerts per window
    pub max_alerts_per_window: u32,
    /// Suppression rules
    pub rules: Vec<SuppressionRule>,
}

/// Suppression rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuppressionRule {
    /// Rule name
    pub name: String,
    /// Rule pattern
    pub pattern: String,
    /// Suppression duration
    pub duration: Duration,
}

/// Benchmarking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkingConfig {
    /// Enable benchmarking
    pub enabled: bool,
    /// Benchmark suites
    pub suites: Vec<BenchmarkSuite>,
    /// Benchmark execution schedule
    pub schedule: BenchmarkSchedule,
    /// Benchmark reporting
    pub reporting: BenchmarkReportingConfig,
}

/// Benchmark suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuite {
    /// Suite name
    pub name: String,
    /// Suite description
    pub description: String,
    /// Benchmarks in this suite
    pub benchmarks: Vec<Benchmark>,
}

/// Individual benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Benchmark {
    /// Benchmark name
    pub name: String,
    /// Benchmark description
    pub description: String,
    /// Benchmark type
    pub benchmark_type: BenchmarkType,
    /// Benchmark configuration
    pub config: BenchmarkBenchmarkConfig,
}

/// Benchmark type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BenchmarkType {
    /// Load test
    Load,
    /// Stress test
    Stress,
    /// Performance test
    Performance,
    /// Endurance test
    Endurance,
    /// Spike test
    Spike,
}

/// Benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkBenchmarkConfig {
    /// Duration of the benchmark
    pub duration: Duration,
    /// Number of concurrent users/operations
    pub concurrency: u32,
    /// Request rate
    pub request_rate: f64,
    /// Ramp-up period
    pub ramp_up: Duration,
    /// Ramp-down period
    pub ramp_down: Duration,
}

/// Benchmark execution schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSchedule {
    /// Enable scheduled benchmarks
    pub enabled: bool,
    /// Cron expression for scheduling
    pub cron: String,
    /// Time zone for scheduling
    pub timezone: String,
}

/// Benchmark reporting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReportingConfig {
    /// Enable reporting
    pub enabled: bool,
    /// Report formats
    pub formats: Vec<ReportFormat>,
    /// Report destinations
    pub destinations: Vec<ReportDestination>,
}

/// Report format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    /// JSON format
    Json,
    /// HTML format
    Html,
    /// CSV format
    Csv,
    /// PDF format
    Pdf,
}

/// Report destination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportDestination {
    /// Local file system
    File { path: String },
    /// Email
    Email { recipients: Vec<String> },
    /// Cloud storage
    Cloud { bucket: String, path: String },
}

impl Default for PerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            enable_detailed_metrics: false,
            metrics_interval: Duration::from_secs(10),
            metrics: PerformanceMetricsConfig::default(),
            storage: MonitoringStorageConfig::default(),
        }
    }
}

impl Default for PerformanceMetricsConfig {
    fn default() -> Self {
        Self {
            latency: true,
            throughput: true,
            resource_utilization: true,
            error_rate: true,
            cache_hit_rate: true,
            queue_depth: true,
            custom_metrics: HashMap::new(),
        }
    }
}

impl Default for MonitoringStorageConfig {
    fn default() -> Self {
        Self {
            backend: StorageBackend::Memory,
            retention_period: Duration::from_secs(86400 * 7), // 7 days
            enable_compression: true,
            enable_sharding: false,
        }
    }
}

impl Default for LatencyTargetsConfig {
    fn default() -> Self {
        Self {
            p50_ms: 50.0,
            p95_ms: 200.0,
            p99_ms: 500.0,
            p99_9_ms: 1000.0,
            max_ms: 5000.0,
        }
    }
}

impl Default for ThroughputTargetsConfig {
    fn default() -> Self {
        Self {
            min_ops_per_sec: 100.0,
            target_ops_per_sec: 1000.0,
            max_ops_per_sec: 10000.0,
            requests_per_minute: 60000.0,
        }
    }
}

impl Default for ResourceUtilizationTargetsConfig {
    fn default() -> Self {
        Self {
            cpu_utilization: 0.7,
            memory_utilization: 0.8,
            disk_utilization: 0.85,
            network_utilization: 0.8,
        }
    }
}

impl Default for ErrorRateTargetsConfig {
    fn default() -> Self {
        Self {
            max_error_rate: 0.01,
            target_error_rate: 0.001,
            critical_error_rate: 0.05,
        }
    }
}

impl Default for OptimizationStrategiesConfig {
    fn default() -> Self {
        Self {
            enable_auto_optimization: true,
            strategies: vec![
                OptimizationStrategy::DatabaseOptimization,
                OptimizationStrategy::MemoryOptimization,
                OptimizationStrategy::AsyncOptimization,
                OptimizationStrategy::CacheOptimization,
            ],
            triggers: OptimizationTriggersConfig::default(),
            cooldown_period: Duration::from_secs(300),
        }
    }
}

impl Default for OptimizationTriggersConfig {
    fn default() -> Self {
        Self {
            latency_threshold_ms: 1000.0,
            throughput_threshold_ops_per_sec: 100.0,
            cpu_threshold: 0.8,
            memory_threshold: 0.85,
            error_rate_threshold: 0.02,
        }
    }
}

impl Default for PerformanceAlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: vec![
                AlertRule {
                    name: "High Latency".to_string(),
                    description: "P95 latency exceeds threshold".to_string(),
                    metric: "latency_p95".to_string(),
                    threshold: 1000.0,
                    operator: ComparisonOperator::GreaterThan,
                    severity: AlertSeverity::High,
                    duration: Duration::from_secs(60),
                },
                AlertRule {
                    name: "High Error Rate".to_string(),
                    description: "Error rate exceeds threshold".to_string(),
                    metric: "error_rate".to_string(),
                    threshold: 0.05,
                    operator: ComparisonOperator::GreaterThan,
                    severity: AlertSeverity::Critical,
                    duration: Duration::from_secs(30),
                },
            ],
            channels: vec![],
            suppression: AlertSuppressionConfig::default(),
        }
    }
}

impl Default for AlertSuppressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            window: Duration::from_secs(300),
            max_alerts_per_window: 5,
            rules: vec![],
        }
    }
}

impl Default for BenchmarkSchedule {
    fn default() -> Self {
        Self {
            enabled: false,
            cron: "0 0 * * *".to_string(), // Daily at midnight
            timezone: "UTC".to_string(),
        }
    }
}

impl Default for BenchmarkReportingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            formats: vec![ReportFormat::Json],
            destinations: vec![],
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            monitoring: PerformanceMonitoringConfig::default(),
            targets: PerformanceTargetsConfig::default(),
            optimization: OptimizationStrategiesConfig::default(),
            alerting: PerformanceAlertingConfig::default(),
            benchmarking: BenchmarkingConfig::default(),
        }
    }
}

impl Default for PerformanceTargetsConfig {
    fn default() -> Self {
        Self {
            latency: LatencyTargetsConfig::default(),
            throughput: ThroughputTargetsConfig::default(),
            resource_utilization: ResourceUtilizationTargetsConfig::default(),
            error_rate: ErrorRateTargetsConfig::default(),
        }
    }
}

impl Default for BenchmarkingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            schedule: BenchmarkSchedule::default(),
            reporting: BenchmarkReportingConfig::default(),
            suites: vec![],
        }
    }
}

impl PerformanceConfig {
    /// Create production-optimized performance configuration
    pub fn production() -> Self {
        let mut config = Self::default();

        // Comprehensive monitoring
        config.monitoring.enabled = true;
        config.monitoring.enable_detailed_metrics = true;
        config.monitoring.metrics_interval = Duration::from_secs(5);
        config.monitoring.storage.backend = StorageBackend::Database {
            url: "postgresql://localhost/beardog_metrics".to_string(),
        };
        config.monitoring.storage.retention_period = Duration::from_secs(86400 * 30); // 30 days

        // Aggressive targets
        config.targets.latency.p95_ms = 100.0;
        config.targets.latency.p99_ms = 250.0;
        config.targets.throughput.target_ops_per_sec = 2000.0;
        config.targets.error_rate.max_error_rate = 0.005;

        // Auto-optimization enabled
        config.optimization.enable_auto_optimization = true;
        config.optimization.strategies = vec![
            OptimizationStrategy::ScaleUp,
            OptimizationStrategy::ScaleDown,
            OptimizationStrategy::DatabaseOptimization,
            OptimizationStrategy::MemoryOptimization,
            OptimizationStrategy::AsyncOptimization,
            OptimizationStrategy::CacheOptimization,
            OptimizationStrategy::LoadBalancing,
        ];
        config.optimization.cooldown_period = Duration::from_secs(180);

        // Comprehensive alerting
        config.alerting.enabled = true;

        // Benchmarking enabled
        config.benchmarking.enabled = true;
        config.benchmarking.schedule.enabled = true;
        config.benchmarking.reporting.enabled = true;

        config
    }

    /// Create development-optimized performance configuration
    pub fn development() -> Self {
        let mut config = Self::default();

        // Detailed monitoring for debugging
        config.monitoring.enabled = true;
        config.monitoring.enable_detailed_metrics = true;
        config.monitoring.metrics_interval = Duration::from_secs(5);
        config.monitoring.storage.backend = StorageBackend::File {
            path: "./dev-metrics".to_string(),
        };
        config.monitoring.storage.retention_period = Duration::from_secs(86400 * 3); // 3 days

        // Relaxed targets for development
        config.targets.latency.p95_ms = 500.0;
        config.targets.latency.p99_ms = 1000.0;
        config.targets.throughput.target_ops_per_sec = 500.0;
        config.targets.error_rate.max_error_rate = 0.02;

        // Conservative auto-optimization
        config.optimization.enable_auto_optimization = false;
        config.optimization.strategies = vec![
            OptimizationStrategy::DatabaseOptimization,
            OptimizationStrategy::MemoryOptimization,
        ];

        // Development alerting
        config.alerting.enabled = false;

        // No benchmarking in development
        config.benchmarking.enabled = false;

        config
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.targets.latency.p50_ms > self.targets.latency.p95_ms {
            return Err("P50 latency must be less than P95 latency".to_string());
        }

        if self.targets.latency.p95_ms > self.targets.latency.p99_ms {
            return Err("P95 latency must be less than P99 latency".to_string());
        }

        if self.targets.throughput.min_ops_per_sec > self.targets.throughput.target_ops_per_sec {
            return Err("Minimum ops/sec must be less than target ops/sec".to_string());
        }

        if self.targets.error_rate.target_error_rate > self.targets.error_rate.max_error_rate {
            return Err("Target error rate must be less than max error rate".to_string());
        }

        Ok(())
    }
}
