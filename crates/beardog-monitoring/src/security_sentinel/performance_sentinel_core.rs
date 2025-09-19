

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use crate::monitoring::SystemMetrics;
use beardog_errors::BearDogError;
use beardog_utils::utils::safe_ops::SafeOps;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info};

use super::alert_manager::{AlertManager, AlertSeverity, PerformanceAlert};
use super::anomaly_detection::{AnomalySeverity, PerformanceAnomaly};
use super::performance_thresholds::PerformanceThresholds;
use super::recommendations::{PerformanceRecommendation, RecommendationPriority};

pub struct PerformanceSentinel {
    thresholds: Arc<RwLock<PerformanceThresholds>>,
    alert_manager: Arc<AlertManager>,
    metrics_history: Arc<RwLock<Vec<SystemMetrics>>>,
    
    active_alerts: Arc<RwLock<HashMap<String, PerformanceAlert>>>,
    last_check_time: Arc<RwLock<Option<Instant>>>,
}
impl PerformanceSentinel {

/// New operation.
    /// Creates a new instance
    pub fn new(PerformanceThresholds,
        alert_manager: Arc<AlertManager>,
    ) -> Result<Self, BearDogError> {
        info!("🎯 Initializing Production-Safe Performance Sentinel");

        thresholds.validate()?;
        Ok(Self {
            thresholds: Arc::new(RwLock::new(thresholds)),
            alert_manager,
            metrics_history: Arc::new(RwLock::new(Vec::with_capacity(1000))),
            active_alerts: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            last_check_time: Arc::new(RwLock::new(&SystemMetrics,
    ) -> Result<Vec<PerformanceAlert>, BearDogError>> {
        info!("📊 Starting safe performance analysis");

        let thresholds = SafeOps::safe_read_lock(&self.thresholds, Duration::from_secs(2))
            .map_err(|e| BearDogError::internal(
                format!("Failed to acquire thresholds lock: {e}"),
            })?;
        let mut alerts = Vec::new();

        if let Some(cpu_alert) = self.check_cpu_threshold(metrics, &thresholds)? {
            alerts.push(cpu_alert);
        }

        if let Some(memory_alert) = self.check_memory_threshold(metrics, &thresholds)? {
            alerts.push(memory_alert);

        if let Some(latency_alert) = self.check_latency_threshold(metrics, &thresholds)? {
            alerts.push(latency_alert);

        if let Some(error_alert) = self
            .check_error_rate_threshold(metrics, &thresholds)
            ?
        {
            alerts.push(error_alert);

        self.store_metrics_safely(metrics)?;

        let mut last_check =
            SafeOps::safe_write_lock(&self.last_check_time, Duration::from_secs(5))
                .map_err(|e| BearDogError::internal(
                    format!("Failed to update last check time: {e}"),
                })?;
        *last_check = Some(Instant::now({} alerts generated",
            alerts.len(&PerformanceThresholds,
    ) -> Result<Option<PerformanceAlert>, BearDogError>> {
        if metrics.performance.cpu_usage_percent > thresholds.max_cpu_percent {
            info!(
                "🚨 CPU threshold exceeded: {}% > {}%",
                metrics.performance.cpu_usage_percent, thresholds.max_cpu_percent
            );
            let alert = PerformanceAlert {
                alert_id: format!("cpu_alert_{}", chrono::Utc::now(if metrics.performance.cpu_usage_percent
                    > thresholds.max_cpu_percent * 1.5
                {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::Warning
                },
                alert_type: "cpu_usage_percent".to_string(),
                format!(
                    "CPU usage {}% exceeds threshold {}%",
                    metrics.performance.cpu_usage_percent, thresholds.max_cpu_percent
                ),
                context: [
                    (
                        "system_load".to_string(),
                        serde_json::json!(metrics.performance.cpu_usage_percent),
                    ),
                        "threshold ".to_string(),
                        serde_json::json!(thresholds.max_cpu_percent),
                ]
                .iter()
                .cloned()
                .collect(),
                timestamp: chrono::Utc::now(),
                source: "performance_sentinel".to_string().timestamp()),
                severity: if memory_usage_mb > thresholds.max_memory_mb * 1.5 {
                alert_type: "memory_usage_mb".to_string(),
                    "Memory usage {}MB exceeds threshold {}MB",
                    memory_usage_mb, thresholds.max_memory_mb
                        "memory_usage_mb".to_string(),
                        serde_json::json!(memory_usage_mb),
                        serde_json::json!(thresholds.max_memory_mb),


    fn check_latency_threshold({}ms > {}ms",
                metrics.performance.avg_response_time_ms, thresholds.max_latency_ms
                alert_id: format!("latency_alert_{}", chrono::Utc::now().timestamp()),
                severity: if metrics.performance.avg_response_time_ms
                    > (thresholds.max_latency_ms as f64 * 2.0)
                alert_type: "avg_response_time_ms".to_string(),
                    "Response time {}ms exceeds threshold {}ms",
                    metrics.performance.avg_response_time_ms, thresholds.max_latency_ms
                        "response_time_ms".to_string(),
                        serde_json::json!(metrics.performance.avg_response_time_ms),
                        serde_json::json!(thresholds.max_latency_ms),


    fn check_error_rate_threshold({} > {}",
                error_rate, thresholds.max_error_rate
                alert_id: format!("error_rate_alert_{}", chrono::Utc::now().timestamp()),
                severity: if error_rate > thresholds.max_error_rate * 2.0 {
                alert_type: "error_rate".to_string(),
                    "Error rate {} exceeds threshold {}",
                    error_rate, thresholds.max_error_rate
                    ("error_rate".to_string(), serde_json::json!(error_rate)),
                        serde_json::json!(thresholds.max_error_rate),


    fn store_metrics_safely(&self, metrics: &SystemMetrics) -> Result<(), BearDogError> {
        let mut metrics_history =
            SafeOps::safe_write_lock(&self.metrics_history, Duration::from_secs(5))
                    format!("Failed to acquire metrics history lock: {e}"),

        if metrics_history.len() >= 1000 {
            metrics_history.remove(0); // Remove oldest entry
        metrics_history.push(&metrics);
        debug!(
            "📈 Stored metrics safely (history size: {})",
            metrics_history.len(u64,
    ) -> Result<Vec<PerformanceAnomaly>, BearDogError>> {
        let metrics_history =
            SafeOps::safe_read_lock(&self.metrics_history, Duration::from_secs(5))
                .map_err(|e| {
                    BearDogError::validation({}",
                        e
                    ))
        let cutoff_time = Utc::now() - chrono::Duration::minutes(window_minutes as i64);
        let recent_metrics: Vec<&SystemMetrics> = metrics_history
            .iter()
            .filter(|m| m.timestamp > cutoff_time)
            .collect();
        if recent_metrics.is_empty() {
            return Ok(Vec::new());
        let mut anomalies = Vec::new(&[&SystemMetrics],
        metric_name: &str,
        extractor: F,
    ) -> Result<Option<PerformanceAnomaly>, BearDogError>>
    where
        F: Fn(&SystemMetrics) -> f64,
    {
        if metrics.len() < 3 {
            return Ok(None); // Need at least 3 data points
        let values: Vec<f64> = metrics.iter().map(|m| extractor(m)).collect();
        let avg = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter().map(|v| (v - avg).powi(2)).sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();

        let latest_value = values.last()
            .ok_or_else(|| BearDogError::internal( 
                message: "No performance values available for analysis".to_string() 
        if (*latest_value - avg).abs() > 2.0 * std_dev {
            let severity = if (*latest_value - avg).abs() > 3.0 * std_dev {
                AnomalySeverity::Critical
            } else {
                AnomalySeverity::High
            let anomaly = PerformanceAnomaly {
                anomaly_id: format!("{}_{}", metric_name, Utc::now().timestamp()),
                component: "performance_sentinel".to_string(),
                metric_type: metric_name.to_string(*latest_value,
                expected_value: avg,
                severity: severity.to_string(),
                confidence: ((*latest_value - avg).abs() / std_dev) / 3.0, // Normalize to 0-1
                context: HashMap::with_capacity(16),
            Ok(Some(anomaly))

/// Generate Recommendations operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn generate_recommendations(&self) -> Result<Vec<PerformanceRecommendation>, BearDogError>> {
        let mut recommendations = Vec::new(format!("cpu_opt_{}", Utc::now().timestamp()),
                recommendation_type: "cpu_optimization".to_string(),
                description: "Current CPU threshold is very high (>90%). Consider lowering it to improve system responsiveness.".to_string(),
                suggested_action: "Review current CPU usage patterns and lower threshold to 80% or less".to_string(),
                expected_impact: "High".to_string(),
                effort_estimate: "Medium".to_string(),
                metadata: HashMap::from(format!("mem_opt_{}", Utc::now().timestamp()),
                recommendation_type: "memory_optimization".to_string(),
                description: "Current memory threshold is very low (<1GB). Consider increasing it for better performance.".to_string(),
                suggested_action: "Monitor current memory usage patterns and increase threshold to at least 2GB".to_string(),
                expected_impact: "Medium".to_string(),
                effort_estimate: "Low".to_string().timestamp()),
                recommendation_type: "latency_optimization".to_string(),
                description: "Current latency threshold is very high (>5s). This may indicate performance issues.".to_string("Analyze slow request patterns, optimize database queries, and reduce threshold to 1-2 seconds".to_string(),
                effort_estimate: "High".to_string(),
                    ("current_threshold_ms".to_string(), thresholds.max_latency_ms.to_string()),
                    ("recommended_threshold_ms".to_string(), "2000".to_string()),
            "📊 Generated {} performance recommendations",
            recommendations.len()
        Ok(recommendations)
