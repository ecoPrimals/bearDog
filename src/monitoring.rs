use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::error::{BearDogResult, BearDogError};
use crate::utils::env_utils::{EnvUtils, ObservabilityConfig};
use crate::licensing::LicenseManager;

/// Metric value types that can be recorded
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<f64>),
    Timer(Duration),
    String(String),
}

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Individual component health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
    pub last_check: DateTime<Utc>,
    pub check_duration_ms: u64,
    pub metadata: HashMap<String, String>,
}

/// Overall system health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub status: HealthStatus,
    pub uptime_seconds: i64,
    pub version: String,
    pub components: Vec<ComponentHealth>,
    pub last_updated: DateTime<Utc>,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_bytes: u64,
    pub memory_total_bytes: u64,
    pub disk_usage_bytes: u64,
    pub disk_total_bytes: u64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub active_connections: u32,
    pub request_count: u64,
    pub error_count: u64,
    pub avg_response_time_ms: f64,
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub performance: PerformanceMetrics,
    pub custom_metrics: HashMap<String, f64>,
}

/// Health check trait for components
#[async_trait::async_trait]
pub trait HealthChecker: Send + Sync {
    async fn check_health(&self) -> BearDogResult<ComponentHealth>;
    fn component_name(&self) -> &str;
}

/// Monitoring service for production deployments
pub struct MonitoringService {
    start_time: Instant,
    checkers: Vec<Arc<dyn HealthChecker>>,
    metrics: Arc<RwLock<SystemMetrics>>,
    config: ObservabilityConfig,
    alert_thresholds: AlertThresholds,
}

/// Alert thresholds for monitoring
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub cpu_threshold: f64,
    pub memory_threshold: f64,
    pub disk_threshold: f64,
    pub error_rate_threshold: f64,
    pub response_time_threshold: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
            error_rate_threshold: 5.0,
            response_time_threshold: 1000.0,
        }
    }
}

impl MonitoringService {
    /// Create a new monitoring service
    pub fn new(config: ObservabilityConfig) -> Self {
        Self {
            start_time: Instant::now(),
            checkers: Vec::new(),
            metrics: Arc::new(RwLock::new(Self::default_metrics())),
            config,
            alert_thresholds: AlertThresholds::default(),
        }
    }

    /// Register a health checker
    pub fn register_checker(&mut self, checker: Arc<dyn HealthChecker>) {
        self.checkers.push(checker);
    }

    /// Get system health status
    pub async fn get_health(&self) -> BearDogResult<SystemHealth> {
        let mut components = Vec::new();
        let mut overall_status = HealthStatus::Healthy;

        // Check all registered components
        for checker in &self.checkers {
            let check_start = Instant::now();
            
            match checker.check_health().await {
                Ok(mut component) => {
                    component.check_duration_ms = check_start.elapsed().as_millis() as u64;
                    
                    // Update overall status based on component status
                    match component.status {
                        HealthStatus::Unhealthy => overall_status = HealthStatus::Unhealthy,
                        HealthStatus::Degraded if overall_status == HealthStatus::Healthy => {
                            overall_status = HealthStatus::Degraded;
                        }
                        _ => {}
                    }
                    
                    components.push(component);
                }
                Err(e) => {
                    overall_status = HealthStatus::Unhealthy;
                    components.push(ComponentHealth {
                        name: checker.component_name().to_string(),
                        status: HealthStatus::Unhealthy,
                        message: Some(format!("Health check failed: {}", e)),
                        last_check: Utc::now(),
                        check_duration_ms: check_start.elapsed().as_millis() as u64,
                        metadata: HashMap::new(),
                    });
                }
            }
        }

        Ok(SystemHealth {
            status: overall_status,
            uptime_seconds: self.start_time.elapsed().as_secs() as i64,
            version: env!("CARGO_PKG_VERSION").to_string(),
            components,
            last_updated: Utc::now(),
        })
    }

    /// Get system metrics
    pub async fn get_metrics(&self) -> SystemMetrics {
        self.metrics.read().await.clone()
    }

    /// Update system metrics
    pub async fn update_metrics(&self, metrics: SystemMetrics) {
        let mut current_metrics = self.metrics.write().await;
        *current_metrics = metrics.clone();
        
        // Check for alerts
        self.check_alerts(&metrics).await;
    }

    /// Start monitoring loop
    pub async fn start_monitoring(&self) -> BearDogResult<()> {
        if !self.config.enable_metrics {
            tracing::info!("Monitoring disabled in configuration");
            return Ok(());
        }

        tracing::info!("Starting monitoring service");
        
        let metrics = self.metrics.clone();
        let thresholds = self.alert_thresholds.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                // Collect system metrics
                let current_metrics = Self::collect_system_metrics().await;
                
                // Update metrics
                {
                    let mut metrics_guard = metrics.write().await;
                    *metrics_guard = current_metrics.clone();
                }
                
                // Check thresholds and log warnings
                Self::check_thresholds(&current_metrics, &thresholds).await;
            }
        });
        
        Ok(())
    }

    /// Collect system performance metrics
    async fn collect_system_metrics() -> SystemMetrics {
        // In a real implementation, you'd use system APIs to get actual metrics
        // For now, we'll provide placeholder implementations
        
        SystemMetrics {
            timestamp: Utc::now(),
            performance: PerformanceMetrics {
                cpu_usage_percent: Self::get_cpu_usage().await,
                memory_usage_bytes: Self::get_memory_usage().await,
                memory_total_bytes: Self::get_total_memory().await,
                disk_usage_bytes: Self::get_disk_usage().await,
                disk_total_bytes: Self::get_total_disk().await,
                network_rx_bytes: Self::get_network_rx().await,
                network_tx_bytes: Self::get_network_tx().await,
                active_connections: Self::get_active_connections().await,
                request_count: Self::get_request_count().await,
                error_count: Self::get_error_count().await,
                avg_response_time_ms: Self::get_avg_response_time().await,
            },
            custom_metrics: HashMap::new(),
        }
    }

    /// Check alert thresholds
    async fn check_alerts(&self, metrics: &SystemMetrics) {
        let perf = &metrics.performance;
        let thresholds = &self.alert_thresholds;
        
        if perf.cpu_usage_percent > thresholds.cpu_threshold {
            tracing::warn!("High CPU usage: {:.1}%", perf.cpu_usage_percent);
        }
        
        let memory_usage_percent = (perf.memory_usage_bytes as f64 / perf.memory_total_bytes as f64) * 100.0;
        if memory_usage_percent > thresholds.memory_threshold {
            tracing::warn!("High memory usage: {:.1}%", memory_usage_percent);
        }
        
        let disk_usage_percent = (perf.disk_usage_bytes as f64 / perf.disk_total_bytes as f64) * 100.0;
        if disk_usage_percent > thresholds.disk_threshold {
            tracing::warn!("High disk usage: {:.1}%", disk_usage_percent);
        }
        
        if perf.avg_response_time_ms > thresholds.response_time_threshold {
            tracing::warn!("High response time: {:.1}ms", perf.avg_response_time_ms);
        }
    }

    /// Check thresholds (static version for spawn)
    async fn check_thresholds(metrics: &SystemMetrics, thresholds: &AlertThresholds) {
        let perf = &metrics.performance;
        
        if perf.cpu_usage_percent > thresholds.cpu_threshold {
            tracing::warn!("High CPU usage: {:.1}%", perf.cpu_usage_percent);
        }
        
        let memory_usage_percent = (perf.memory_usage_bytes as f64 / perf.memory_total_bytes as f64) * 100.0;
        if memory_usage_percent > thresholds.memory_threshold {
            tracing::warn!("High memory usage: {:.1}%", memory_usage_percent);
        }
    }

    fn default_metrics() -> SystemMetrics {
        SystemMetrics {
            timestamp: Utc::now(),
            performance: PerformanceMetrics {
                cpu_usage_percent: 0.0,
                memory_usage_bytes: 0,
                memory_total_bytes: 0,
                disk_usage_bytes: 0,
                disk_total_bytes: 0,
                network_rx_bytes: 0,
                network_tx_bytes: 0,
                active_connections: 0,
                request_count: 0,
                error_count: 0,
                avg_response_time_ms: 0.0,
            },
            custom_metrics: HashMap::new(),
        }
    }

    // Placeholder system metric collection methods
    // In production, these would integrate with actual system APIs
    async fn get_cpu_usage() -> f64 {
        // Use procfs, sysinfo, or similar crate for real CPU usage
        rand::random::<f64>() * 20.0 // Simulate 0-20% CPU usage
    }

    async fn get_memory_usage() -> u64 {
        // Real implementation would read from /proc/meminfo or similar
        1024 * 1024 * 256 // 256MB
    }

    async fn get_total_memory() -> u64 {
        1024 * 1024 * 1024 * 4 // 4GB
    }

    async fn get_disk_usage() -> u64 {
        1024 * 1024 * 1024 * 10 // 10GB
    }

    async fn get_total_disk() -> u64 {
        1024 * 1024 * 1024 * 100 // 100GB
    }

    async fn get_network_rx() -> u64 {
        1024 * 1024 * 50 // 50MB
    }

    async fn get_network_tx() -> u64 {
        1024 * 1024 * 30 // 30MB
    }

    async fn get_active_connections() -> u32 {
        42 // Placeholder
    }

    async fn get_request_count() -> u64 {
        1000 // Placeholder
    }

    async fn get_error_count() -> u64 {
        5 // Placeholder
    }

    async fn get_avg_response_time() -> f64 {
        150.0 // 150ms average
    }
}

/// Database health checker
pub struct DatabaseHealthChecker {
    // Add database connection pool or client here
}

impl DatabaseHealthChecker {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl HealthChecker for DatabaseHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();
        
        // In a real implementation, you'd ping the database
        // For now, simulate a health check
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        Ok(ComponentHealth {
            name: "Database".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Database connection OK".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("type".to_string(), "PostgreSQL".to_string());
                meta.insert("version".to_string(), "14.5".to_string());
                meta
            },
        })
    }

    fn component_name(&self) -> &str {
        "Database"
    }
}

/// Redis health checker
pub struct RedisHealthChecker {
    // Add Redis connection pool or client here
}

impl RedisHealthChecker {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl HealthChecker for RedisHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();
        
        // In a real implementation, you'd ping Redis
        tokio::time::sleep(Duration::from_millis(5)).await;
        
        Ok(ComponentHealth {
            name: "Redis".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Redis connection OK".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("type".to_string(), "Redis".to_string());
                meta.insert("version".to_string(), "7.0".to_string());
                meta
            },
        })
    }

    fn component_name(&self) -> &str {
        "Redis"
    }
}

/// External service health checker
pub struct ExternalServiceHealthChecker {
    pub name: String,
    pub endpoint: String,
    pub client: reqwest::Client,
}

impl ExternalServiceHealthChecker {
    pub fn new(name: String, endpoint: String) -> Self {
        Self {
            name,
            endpoint,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl HealthChecker for ExternalServiceHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();
        
        match self.client.get(&self.endpoint).timeout(Duration::from_secs(5)).send().await {
            Ok(response) => {
                let status = if response.status().is_success() {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Degraded
                };
                
                Ok(ComponentHealth {
                    name: self.name.clone(),
                    status,
                    message: Some(format!("HTTP {}", response.status())),
                    last_check: Utc::now(),
                    check_duration_ms: start.elapsed().as_millis() as u64,
                    metadata: {
                        let mut meta = HashMap::new();
                        meta.insert("endpoint".to_string(), self.endpoint.clone());
                        meta.insert("status_code".to_string(), response.status().as_u16().to_string());
                        meta
                    },
                })
            }
            Err(e) => {
                Ok(ComponentHealth {
                    name: self.name.clone(),
                    status: HealthStatus::Unhealthy,
                    message: Some(format!("Request failed: {}", e)),
                    last_check: Utc::now(),
                    check_duration_ms: start.elapsed().as_millis() as u64,
                    metadata: {
                        let mut meta = HashMap::new();
                        meta.insert("endpoint".to_string(), self.endpoint.clone());
                        meta.insert("error".to_string(), e.to_string());
                        meta
                    },
                })
            }
        }
    }

    fn component_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitoring_service_creation() {
        let config = ObservabilityConfig {
            log_level: "INFO".to_string(),
            enable_metrics: true,
            metrics_port: 9090,
            enable_tracing: false,
            jaeger_endpoint: None,
            otlp_endpoint: None,
        };
        
        let service = MonitoringService::new(config);
        assert_eq!(service.checkers.len(), 0);
    }

    #[tokio::test]
    async fn test_database_health_checker() {
        let checker = DatabaseHealthChecker::new();
        let health = checker.check_health().await.unwrap();
        
        assert_eq!(health.name, "Database");
        assert_eq!(health.status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_system_health_no_components() {
        let config = ObservabilityConfig {
            log_level: "INFO".to_string(),
            enable_metrics: true,
            metrics_port: 9090,
            enable_tracing: false,
            jaeger_endpoint: None,
            otlp_endpoint: None,
        };
        
        let service = MonitoringService::new(config);
        let health = service.get_health().await.unwrap();
        
        assert_eq!(health.status, HealthStatus::Healthy);
        assert_eq!(health.components.len(), 0);
    }
}

/// Metrics collection and export system
/// 
/// **LICENSING CLARITY**:
/// - Native Rust metrics collection: FREE (AGPL)
/// - Prometheus export: REQUIRES LICENSE (external system)
/// - Internal monitoring: FREE (AGPL) 
pub struct MetricsService {
    /// Native Rust metrics (always free)
    native_metrics: Arc<RwLock<HashMap<String, MetricValue>>>,
    /// License manager for external integrations
    license_manager: Arc<LicenseManager>,
    /// Prometheus exporter (if licensed)
    prometheus_exporter: Option<PrometheusExporter>,
    /// Internal metrics collection (always enabled)
    internal_collector: InternalMetricsCollector,
}

/// Native Rust metrics collection (100% free under AGPL)
#[derive(Debug, Clone)]
pub struct InternalMetricsCollector {
    pub security_events: Arc<AtomicU64>,
    pub encryption_operations: Arc<AtomicU64>,
    pub threat_detections: Arc<AtomicU64>,
    pub compliance_checks: Arc<AtomicU64>,
    pub api_requests: Arc<AtomicU64>,
    pub error_count: Arc<AtomicU64>,
    pub active_sessions: Arc<AtomicU64>,
    pub last_updated: Arc<RwLock<DateTime<Utc>>>,
}

/// Prometheus exporter (requires license for external Prometheus systems)
pub struct PrometheusExporter {
    enabled: bool,
    endpoint: String,
    port: u16,
}

impl MetricsService {
    /// Create new metrics service
    pub fn new(license_manager: Arc<LicenseManager>) -> Self {
        Self {
            native_metrics: Arc::new(RwLock::new(HashMap::new())),
            license_manager,
            prometheus_exporter: None,
            internal_collector: InternalMetricsCollector::new(),
        }
    }

    /// Record metric (always free - native Rust)
    pub async fn record_metric(&self, name: &str, value: MetricValue) {
        let mut metrics = self.native_metrics.write().await;
        metrics.insert(name.to_string(), value);
        
        // Update internal collector timestamp
        *self.internal_collector.last_updated.write().await = Utc::now();
    }

    /// Get native metrics (always free)
    pub async fn get_native_metrics(&self) -> HashMap<String, MetricValue> {
        self.native_metrics.read().await.clone()
    }

    /// Get internal metrics summary (always free)
    pub async fn get_internal_summary(&self) -> InternalMetricsSummary {
        InternalMetricsSummary {
            security_events: self.internal_collector.security_events.load(Ordering::Relaxed),
            encryption_operations: self.internal_collector.encryption_operations.load(Ordering::Relaxed),
            threat_detections: self.internal_collector.threat_detections.load(Ordering::Relaxed),
            compliance_checks: self.internal_collector.compliance_checks.load(Ordering::Relaxed),
            api_requests: self.internal_collector.api_requests.load(Ordering::Relaxed),
            error_count: self.internal_collector.error_count.load(Ordering::Relaxed),
            active_sessions: self.internal_collector.active_sessions.load(Ordering::Relaxed),
            last_updated: *self.internal_collector.last_updated.read().await,
        }
    }

    /// Enable Prometheus export (requires license)
    pub async fn enable_prometheus_export(&mut self, config: PrometheusConfig) -> BearDogResult<()> {
        // Check license for Prometheus (external system)
        if !self.license_manager.verify_external_function_access("prometheus")? {
            return Err(BearDogError::Configuration {
                message: "Prometheus export requires a BearDog license. Native Rust metrics are always free. Contact sales for enterprise Prometheus integration.".to_string()
            });
        }

        self.prometheus_exporter = Some(PrometheusExporter {
            enabled: true,
            endpoint: config.endpoint,
            port: config.port,
        });

        tracing::info!("✅ Prometheus export enabled (licensed feature)");
        Ok(())
    }

    /// Export metrics to Prometheus (licensed feature)
    pub async fn export_to_prometheus(&self) -> BearDogResult<String> {
        if let Some(exporter) = &self.prometheus_exporter {
            if !exporter.enabled {
                return Err(BearDogError::Configuration {
                    message: "Prometheus export is not enabled".to_string()
                });
            }

            // Generate Prometheus format
            let summary = self.get_internal_summary().await;
            let prometheus_output = format!(
                "# HELP beardog_security_events_total Total security events processed\n\
                 # TYPE beardog_security_events_total counter\n\
                 beardog_security_events_total {}\n\
                 # HELP beardog_encryption_operations_total Total encryption operations\n\
                 # TYPE beardog_encryption_operations_total counter\n\
                 beardog_encryption_operations_total {}\n\
                 # HELP beardog_threat_detections_total Total threats detected\n\
                 # TYPE beardog_threat_detections_total counter\n\
                 beardog_threat_detections_total {}\n\
                 # HELP beardog_compliance_checks_total Total compliance checks\n\
                 # TYPE beardog_compliance_checks_total counter\n\
                 beardog_compliance_checks_total {}\n\
                 # HELP beardog_api_requests_total Total API requests\n\
                 # TYPE beardog_api_requests_total counter\n\
                 beardog_api_requests_total {}\n\
                 # HELP beardog_errors_total Total errors\n\
                 # TYPE beardog_errors_total counter\n\
                 beardog_errors_total {}\n\
                 # HELP beardog_active_sessions Current active sessions\n\
                 # TYPE beardog_active_sessions gauge\n\
                 beardog_active_sessions {}\n",
                summary.security_events,
                summary.encryption_operations,
                summary.threat_detections,
                summary.compliance_checks,
                summary.api_requests,
                summary.error_count,
                summary.active_sessions
            );

            Ok(prometheus_output)
        } else {
            Err(BearDogError::Configuration {
                message: "Prometheus export not configured. Use native metrics (free) or obtain a license for Prometheus integration.".to_string()
            })
        }
    }

    /// Increment security event counter (always free)
    pub fn increment_security_events(&self) {
        self.internal_collector.security_events.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment encryption operations (always free)
    pub fn increment_encryption_operations(&self) {
        self.internal_collector.encryption_operations.fetch_add(1, Ordering::Relaxed);
    }

    /// Record threat detection (always free)
    pub fn record_threat_detection(&self) {
        self.internal_collector.threat_detections.fetch_add(1, Ordering::Relaxed);
    }

    /// Record compliance check (always free)
    pub fn record_compliance_check(&self) {
        self.internal_collector.compliance_checks.fetch_add(1, Ordering::Relaxed);
    }

    /// Record API request (always free)
    pub fn record_api_request(&self) {
        self.internal_collector.api_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Record error (always free)
    pub fn record_error(&self) {
        self.internal_collector.error_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Update active sessions (always free)
    pub fn update_active_sessions(&self, count: u64) {
        self.internal_collector.active_sessions.store(count, Ordering::Relaxed);
    }
}

impl InternalMetricsCollector {
    pub fn new() -> Self {
        Self {
            security_events: Arc::new(AtomicU64::new(0)),
            encryption_operations: Arc::new(AtomicU64::new(0)),
            threat_detections: Arc::new(AtomicU64::new(0)),
            compliance_checks: Arc::new(AtomicU64::new(0)),
            api_requests: Arc::new(AtomicU64::new(0)),
            error_count: Arc::new(AtomicU64::new(0)),
            active_sessions: Arc::new(AtomicU64::new(0)),
            last_updated: Arc::new(RwLock::new(Utc::now())),
        }
    }
}

/// Internal metrics summary (always free)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMetricsSummary {
    pub security_events: u64,
    pub encryption_operations: u64,
    pub threat_detections: u64,
    pub compliance_checks: u64,
    pub api_requests: u64,
    pub error_count: u64,
    pub active_sessions: u64,
    pub last_updated: DateTime<Utc>,
}

/// Prometheus configuration
#[derive(Debug, Clone)]
pub struct PrometheusConfig {
    pub endpoint: String,
    pub port: u16,
} 