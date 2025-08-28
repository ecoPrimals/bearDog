

use crate::monitoring::SystemMetrics;
use beardog_errors::BearDogError;
use beardog_utils::utils::safe_ops::SafeOps; // DEEP DEBT FIX: SafeOps for all operations
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info};

pub struct AlertManager {

}
impl AlertManager {}

    pub fn new() -> Self {
        Self {

        }
    }
impl Default for AlertManager {}

    fn default() -> Self {
        Self::new()

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub max_cpu_percent: f64,
    pub max_memory_mb: f64,
    pub max_latency_ms: u64,
    pub min_success_rate: f64,
    pub max_error_rate: f64,

pub struct PerformanceAlert {
    pub alert_id: String,
    pub component: String,
    pub metric_type: String,
    pub current_value: f64,
    pub threshold_value: f64,
    pub severity: AlertSeverity,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub context: HashMap<String, String>,
}

pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,}

pub struct PerformanceSentinel {
    thresholds: Arc<RwLock<PerformanceThresholds>>,
    alert_manager: Arc<AlertManager>,
    metrics_history: Arc<RwLock<Vec<SystemMetrics>>>,
    active_alerts: Arc<RwLock<HashMap<String, PerformanceAlert>>>,
    last_check_time: Arc<RwLock<Option<Instant>>>,}

impl PerformanceSentinel {

    pub fn new(
        thresholds: PerformanceThresholds,
        alert_manager: Arc<AlertManager>,
    ) -> Result<Self, BearDogError> {
        info!("🎯 Initializing Production-Safe Performance Sentinel");

        Self::validate_thresholds(&thresholds)?;
        Ok(Self {
            thresholds: Arc::new(RwLock::new(thresholds)),
            alert_manager,
            metrics_history: Arc::new(RwLock::new(Vec::with_capacity(1000))),
            active_alerts: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            last_check_time: Arc::new(RwLock::new(None)),
        })

    fn validate_thresholds(thresholds: &PerformanceThresholds) -> Result<(), BearDogError> {
        if thresholds.max_cpu_percent < 0.0 || thresholds.max_cpu_percent > 100.0 {
            return Err(BearDogError::validation(format!(
                "Invalid CPU threshold: {}% (must be 0-100)",
                thresholds.max_cpu_percent
            )));
        if thresholds.max_memory_mb < 0.0 {
                "Invalid memory threshold: {}MB (must be positive)",
                thresholds.max_memory_mb
        if thresholds.min_success_rate < 0.0 || thresholds.min_success_rate > 100.0 {
                "Invalid success rate threshold: {}% (must be 0-100)",
                thresholds.min_success_rate
        if thresholds.max_error_rate < 0.0 || thresholds.max_error_rate > 100.0 {
                "Invalid error rate threshold: {}% (must be 0-100)",
                thresholds.max_error_rate
        Ok(())

    pub async fn analyze_metrics(
        &self,
        metrics: SystemMetrics,
    ) -> Result<Vec<PerformanceAlert>, BearDogError>> {
        info!("📊 Safe performance analysis starting");

        let thresholds = SafeOps::safe_read_lock(&self.thresholds, Duration::from_secs(5))
            .await
            .map_err(|e| {
                BearDogError::internal(format!("Failed to acquire thresholds lock: {e}"))
            })?;
        let mut alerts = Vec::new();

        if let Some(cpu_alert) = self.check_cpu_threshold(&metrics, &thresholds).await? {
            alerts.push(cpu_alert);

        if let Some(memory_alert) = self.check_memory_threshold(&metrics, &thresholds).await? {
            alerts.push(memory_alert);

        if let Some(latency_alert) = self.check_latency_threshold(&metrics, &thresholds).await? {
            alerts.push(latency_alert);

        if let Some(error_alert) = self
            .check_error_rate_threshold(&metrics, &thresholds)
            .await?
        {
            alerts.push(error_alert);

        self.store_metrics_safely(metrics).await?;

        let mut last_check =
            SafeOps::safe_write_lock(&self.last_check_time, Duration::from_secs(5))
                .await
                .map_err(|e| {
                    BearDogError::internal(format!("Failed to update last check time: {e}"))
                })?;
        *last_check = Some(Instant::now());
        info!(
            "✅ Safe performance analysis completed: {} alerts generated",
            alerts.len()
        );
        Ok(alerts)

    async fn check_cpu_threshold(
        metrics: &SystemMetrics,
        thresholds: &PerformanceThresholds,
    ) -> Result<Option<PerformanceAlert>, BearDogError>> {
        if metrics.performance.cpu_usage_percent > thresholds.max_cpu_percent {
            info!(
                "🚨 CPU threshold exceeded: {}% > {}%",
                metrics.performance.cpu_usage_percent, thresholds.max_cpu_percent
            );
            let alert = PerformanceAlert {
                alert_id: format_args!("cpu_alert_{}", chrono::Utc::now().to_string().timestamp()),
                component: "system_cpu".to_string(),
                metric_type: "cpu_usage_percent".to_string(),
                current_value: metrics.performance.cpu_usage_percent,
                threshold_value: thresholds.max_cpu_percent,
                severity: if metrics.performance.cpu_usage_percent
                    > thresholds.max_cpu_percent * 1.5
                {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::Warning
                },
                timestamp: chrono::Utc::now(),
                context: [
                    (
                        "system_load".to_string(),
                        format_args!("{:.2}", metrics.performance.cpu_usage_percent).to_string(),
                    ),
                        "threshold".to_string(),
                        format_args!("{:.2}", thresholds.max_cpu_percent).to_string(),
                ]
                .iter()
                .cloned()
                .collect(),
            };
            Ok(Some(alert))
        } else {
            Ok(None)

    async fn check_memory_threshold(
        let memory_usage_mb = metrics.performance.memory_usage_bytes as f64 / 1024.0 / 1024.0;
        if memory_usage_mb > thresholds.max_memory_mb {
                "🚨 Memory threshold exceeded: {}MB > {}MB",
                memory_usage_mb, thresholds.max_memory_mb
                alert_id: format_args!("memory_alert_{}", chrono::Utc::now().to_string().timestamp()),
                component: "system_memory".to_string(),
                metric_type: "memory_usage_mb".to_string(),
                current_value: memory_usage_mb,
                threshold_value: thresholds.max_memory_mb,
                severity: if memory_usage_mb > thresholds.max_memory_mb * 1.5 {
                        "memory_usage".to_string(),
                        format!(
                            "{:.2}MB",
                            metrics.performance.memory_usage_bytes as f64 / 1024.0 / 1024.0
                        ),
                        format_args!("{:.2}MB", thresholds.max_memory_mb).to_string(),

    async fn check_latency_threshold(
        let avg_latency = metrics.performance.avg_response_time_ms;
        if avg_latency > thresholds.max_latency_ms as f64 {
                "🚨 Latency threshold exceeded: {}ms > {}ms",
                avg_latency, thresholds.max_latency_ms
                alert_id: format_args!("latency_alert_{}", chrono::Utc::now().to_string().timestamp()),
                component: "system_latency".to_string(),
                metric_type: "average_response_time_ms".to_string(),
                current_value: avg_latency,
                threshold_value: thresholds.max_latency_ms as f64,
                severity: if avg_latency > (thresholds.max_latency_ms * 2) as f64 {
                    ("latency".to_string(), format!("{avg_latency}ms")),
                        format_args!("{}ms", thresholds.max_latency_ms).to_string(),

    async fn check_error_rate_threshold(
        let successful_requests =
            metrics.performance.request_count - metrics.performance.error_count;
        let failed_requests = metrics.performance.error_count;
        let total_requests = successful_requests + failed_requests;
        if total_requests == 0 {
            debug!("No requests to analyze for error rate");
            return Ok(None);
        let error_rate = (failed_requests as f64 / total_requests as f64) * 100.0;
        if error_rate > thresholds.max_error_rate {
                "🚨 Error rate threshold exceeded: {:.2}% > {:.2}%",
                error_rate, thresholds.max_error_rate
                alert_id: format_args!("error_rate_alert_{}", chrono::Utc::now().to_string().timestamp()),
                component: "system_error_rate".to_string(),
                metric_type: "error_rate_percent".to_string(),
                current_value: error_rate,
                threshold_value: thresholds.max_error_rate,
                severity: if error_rate > thresholds.max_error_rate * 2.0 {
                    ("error_rate".to_string(), format!("{error_rate:.2}%")),
                        format_args!("{:.2}%", thresholds.max_error_rate).to_string(),
                    ("failed_requests".to_string(), failed_requests.to_string()),
                    ("total_requests".to_string(), total_requests.to_string()),

    async fn store_metrics_safely(&self, metrics: SystemMetrics) -> Result<(), BearDogError> {
        let mut history = SafeOps::safe_write_lock(&self.metrics_history, Duration::from_secs(5))
                BearDogError::internal(format!("Failed to acquire metrics history lock: {e}"))

        if history.len() >= 1000 {
            history.remove(0); // Remove oldest entry
            debug!("Metrics history capacity managed: removed oldest entry");
        history.push(metrics);
        debug!(
            "✅ Metrics stored safely: {} entries in history",
            history.len()

    pub async fn get_performance_trends(
        window_minutes: u64,
    ) -> Result<PerformanceTrends, BearDogError> {
            "📈 Calculating safe performance trends for {} minutes",
            window_minutes
        let history = SafeOps::safe_read_lock(&self.metrics_history, Duration::from_secs(5))
        if history.is_empty() {
            return Ok(PerformanceTrends::default());
        let cutoff_time = chrono::Utc::now() - chrono::Duration::minutes(window_minutes as i64);

        let recent_metrics: Vec<&SystemMetrics> = history
            .iter()
            .filter(|m| m.timestamp > cutoff_time)
            .collect();
        if recent_metrics.is_empty() {

        let avg_cpu = SafeOps::safe_avg(
            recent_metrics
                .map(|m| m.performance.cpu_usage_percent),
        )
        .unwrap_or(0.0);
        let avg_memory = SafeOps::safe_avg(
                .map(|m| m.performance.memory_usage_bytes as f64 / 1024.0 / 1024.0),
        let avg_latency = SafeOps::safe_avg(
                .map(|m| m.performance.avg_response_time_ms),
        let trends = PerformanceTrends {
            window_minutes,
            sample_count: recent_metrics.len(),
            avg_cpu_percent: avg_cpu,
            avg_memory_mb: avg_memory,
            avg_latency_ms: avg_latency as u64,
            trend_direction: self.calculate_trend_direction(&recent_metrics),
        };
            "✅ Performance trends calculated safely: {} samples",
            trends.sample_count
        Ok(trends)
    fn calculate_trend_direction(&self, _metrics: &[&SystemMetrics]) -> String {

        "stable".to_string()
pub struct PerformanceTrends {
    pub window_minutes: u64,
    pub sample_count: usize,
    pub avg_cpu_percent: f64,
    pub avg_memory_mb: f64,
    pub avg_latency_ms: u64,
    pub trend_direction: String,}

impl Default for PerformanceTrends {
            window_minutes: 0,
            sample_count: 0,
            avg_cpu_percent: 0.0,
            avg_memory_mb: 0.0,
            avg_latency_ms: 0,
            trend_direction: "unknown".to_string(),}

impl Default for PerformanceThresholds {
            max_cpu_percent: 80.0,
            max_memory_mb: 1024.0,
            max_latency_ms: 1000,
            min_success_rate: 95.0,
            max_error_rate: 5.0,
