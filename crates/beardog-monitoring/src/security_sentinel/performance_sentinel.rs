// SPDX-License-Identifier: AGPL-3.0-or-later



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


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

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {

        }
    }
impl Default for AlertManager {}

    fn default() -> Self {
        Self::new(f64,
    /// The max memory mb value
    pub max_memory_mb: f64,
    /// Number of max_latency_ms
    pub max_latency_ms: u64,
    /// The min success rate value
    pub min_success_rate: f64,
    /// The max error rate value
    pub max_error_rate: f64,

pub struct PerformanceAlert {
    pub alert_id: String,
    /// The component value
    pub component: String,
    /// The metric type value
    pub metric_type: String,
    /// The current value value
    pub current_value: f64,
    /// The threshold value value
    pub threshold_value: f64,
    /// The severity value
    pub severity: AlertSeverity,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Mapping of context
    pub context: HashMap<String, String>,
}

pub enum AlertSeverity {
    /// Represents info variant
    Info,
    /// Currently warning
    Warning,
    /// Represents critical variant
    Critical,
    Emergency,}
    Emergency,}
    Emergency,}

pub struct PerformanceSentinel {
    thresholds: Arc<RwLock<PerformanceThresholds>>,
    alert_manager: Arc<AlertManager>,
    metrics_history: Arc<RwLock<Vec<SystemMetrics>>>,
    active_alerts: Arc<RwLock<HashMap<String, PerformanceAlert>>>,
    last_check_time: Arc<RwLock<Option<Instant>>>,}

impl PerformanceSentinel {

/// New operation.
    /// Creates a new instance
    pub fn new(PerformanceThresholds,
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

    /// Validates thresholds
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
        Ok(SystemMetrics,
    ) -> Result<Vec<PerformanceAlert>, BearDogError>> {
        info!("📊 Safe performance analysis starting");

        let thresholds = SafeOps::safe_read_lock(&self.thresholds, Duration::from_secs(5))
            .map_err(|e| {
                BearDogError::internal(format!("Failed to acquire thresholds lock: {e}"))
            })?;
        let mut alerts = Vec::new();

        if let Some(cpu_alert) = self.check_cpu_threshold(&metrics, &thresholds)? {
            alerts.push(cpu_alert);

        if let Some(memory_alert) = self.check_memory_threshold(&metrics, &thresholds)? {
            alerts.push(memory_alert);

        if let Some(latency_alert) = self.check_latency_threshold(&metrics, &thresholds)? {
            alerts.push(latency_alert);

        if let Some(error_alert) = self
            .check_error_rate_threshold(&metrics, &thresholds)
            ?
        {
            alerts.push(error_alert);

        self.store_metrics_safely(metrics)?;

        let mut last_check =
            SafeOps::safe_write_lock(&self.last_check_time, Duration::from_secs(5))
                .map_err(|e| {
                    BearDogError::internal(format!("Failed to update last check time: {e}"))
                })?;
        *last_check = Some(Instant::now({} alerts generated",
            alerts.len(&SystemMetrics,
        thresholds: &PerformanceThresholds,
    ) -> Result<Option<PerformanceAlert>, BearDogError>> {
        if metrics.performance.cpu_usage_percent > thresholds.max_cpu_percent {
            info!(
                "🚨 CPU threshold exceeded: {}% > {}%",
                metrics.performance.cpu_usage_percent, thresholds.max_cpu_percent
            );
            let alert = PerformanceAlert {
                alert_id: format!("cpu_alert_{}", chrono::Utc::now().timestamp()),
                component: "system_cpu".to_string(),
                metric_type: "cpu_usage_percent".to_string(),
                context: [
                    (
                        "system_load".to_string(),
                        format!("{:.2}", metrics.performance.cpu_usage_percent)),
                        "threshold ".to_string(),
                        format!("{:.2}", thresholds.max_cpu_percent),
                ]
                .iter({}MB > {}MB",
                memory_usage_mb, thresholds.max_memory_mb
                alert_id: format!("memory_alert_{}", chrono::Utc::now().timestamp()),
                component: "system_memory".to_string(),
                metric_type: "memory_usage_mb".to_string(),
                        format!("{:.2}MB", thresholds.max_memory_mb),


    fn check_latency_threshold({}ms > {}ms",
                avg_latency, thresholds.max_latency_ms
                alert_id: format!("latency_alert_{}", chrono::Utc::now().timestamp()),
                component: "system_latency".to_string(),
                metric_type: "average_response_time_ms".to_string(),
                severity: if avg_latency > (thresholds.max_latency_ms * 2) as f64 {
                    ("latency".to_string().timestamp()),
                component: "system_error_rate".to_string(),
                metric_type: "error_rate_percent".to_string()),
                        format!("{:.2}%", thresholds.max_error_rate),
                    ("failed_requests".to_string(), failed_requests.to_string()),
                    ("total_requests".to_string(), total_requests.to_string()),


    fn store_metrics_safely(&self, metrics: SystemMetrics) -> Result<(), BearDogError> {
        let mut history = SafeOps::safe_write_lock(&self.metrics_history, Duration::from_secs(5))
                BearDogError::internal(format!("Failed to acquire metrics history lock: {e}"))

        if history.len() >= 1000 {
            history.remove(0); // Remove oldest entry
            debug!("Metrics history capacity managed: removed oldest entry");
        history.push({} entries in history",
            history.len(u64,
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
            sample_count: recent_metrics.len(avg_cpu,
            avg_memory_mb: avg_memory,
            avg_latency_ms: avg_latency as u64,
            trend_direction: self.calculate_trend_direction({} samples",
            trends.sample_count
        Ok(trends)
    fn calculate_trend_direction(&self, _metrics: &[&SystemMetrics]) -> String {

        "stable".to_string(),
