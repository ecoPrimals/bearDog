

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {

    pub cpu_threshold: f64,

    pub memory_threshold: f64,

    pub response_time_threshold: f64,

    pub error_rate_threshold: f64,

    pub throughput_threshold: f64,
}
impl Default for PerformanceThresholds {}

    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            response_time_threshold: 1000.0,
            error_rate_threshold: 5.0,
            throughput_threshold: 100.0,
        }
    }

pub struct PerformanceMetrics {

    pub timestamp: DateTime<Utc>,

    pub cpu_usage: f64,

    pub memory_usage: f64,

    pub response_time: f64,

    pub error_rate: f64,

    pub throughput: f64,

    pub custom_metrics: HashMap<String, f64>,

pub struct PerformanceTrends {

    pub cpu_trend: MetricTrend,

    pub memory_trend: MetricTrend,

    pub response_time_trend: MetricTrend,

    pub error_rate_trend: MetricTrend,

    pub throughput_trend: MetricTrend,

    pub window_minutes: u64,

    pub sample_count: usize,

    pub avg_cpu_percent: f64,

    pub avg_memory_mb: f64,

    pub avg_latency_ms: f64,}

impl Default for PerformanceTrends {
            cpu_trend: MetricTrend::Stable,
            memory_trend: MetricTrend::Stable,
            response_time_trend: MetricTrend::Stable,
            error_rate_trend: MetricTrend::Stable,
            throughput_trend: MetricTrend::Stable,
            window_minutes: 0,
            sample_count: 0,
            avg_cpu_percent: 0.0,
            avg_memory_mb: 0.0,
            avg_latency_ms: 0.0,

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MetricTrend {

    Increasing,

    Decreasing,

    Stable,

    Volatile,

#[derive(Debug)]}

pub struct PerformanceMetricsCollector {

    metrics_history: Arc<RwLock<Vec<PerformanceMetrics>>>,

    thresholds: Arc<RwLock<PerformanceThresholds>>,

    max_history_size: usize,}

impl Default for PerformanceMetricsCollector {
        Self::new()}

impl PerformanceMetricsCollector {

    pub fn new() -> Self {
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            thresholds: Arc::new(RwLock::new(PerformanceThresholds::default())),
            max_history_size: 1000,

    pub async fn collect_metrics(&self) -> Result<PerformanceMetrics, BearDogError> {

        let metrics = PerformanceMetrics {
            timestamp: Utc::now(),
            cpu_usage: self.collect_cpu_usage().await?,
            memory_usage: self.collect_memory_usage().await?,
            response_time: self.collect_response_time().await?,
            error_rate: self.collect_error_rate().await?,
            throughput: self.collect_throughput().await?,
            custom_metrics: HashMap::with_capacity(16),
        };

        {
            let mut history = self.metrics_history.write().await;
            history.push(metrics.clone());

            if history.len() > self.max_history_size {
                history.remove(0);
            }
        debug!(
            "Collected performance metrics: CPU {:.1}%, Memory {:.1}%",
            metrics.cpu_usage, metrics.memory_usage
        );
        Ok(metrics)

    pub async fn get_recent_metrics(&self, count: usize) -> Result<Vec<PerformanceMetrics>, BearDogError>> {
        let history = self.metrics_history.read().await;
        let start_index = if history.len() > count {
            history.len() - count
        } else {
            0
        Ok(history[start_index..].to_vec())

    pub async fn analyze_trends(&self, window_size: usize) -> Result<PerformanceTrends, BearDogError> {
        let recent_metrics = self.get_recent_metrics(window_size).await?;
        if recent_metrics.len() < 2 {
            return Ok(PerformanceTrends::default());
        Ok(PerformanceTrends {
            cpu_trend: self.calculate_trend(&recent_metrics, |m| m.cpu_usage),
            memory_trend: self.calculate_trend(&recent_metrics, |m| m.memory_usage),
            response_time_trend: self.calculate_trend(&recent_metrics, |m| m.response_time),
            error_rate_trend: self.calculate_trend(&recent_metrics, |m| m.error_rate),
            throughput_trend: self.calculate_trend(&recent_metrics, |m| m.throughput),
            sample_count: recent_metrics.len(),
            avg_cpu_percent: recent_metrics.iter().map(|m| m.cpu_usage).sum::<f64>()
                / recent_metrics.len() as f64,
            avg_memory_mb: recent_metrics.iter().map(|m| m.memory_usage).sum::<f64>()
            avg_latency_ms: recent_metrics.iter().map(|m| m.response_time).sum::<f64>()
        })

    fn calculate_trend<F>(&self, metrics: &[PerformanceMetrics], extractor: F) -> MetricTrend
    where
        F: Fn(&PerformanceMetrics) -> f64,
    {
        if metrics.len() < 2 {
            return MetricTrend::Stable;
        let values: Vec<f64> = metrics.iter().map(|m| extractor(m)).collect();
        let first_half: f64 =
            values[0..values.len() / 2].iter().sum::<f64>() / (values.len() / 2) as f64;
        let second_half: f64 = values[values.len() / 2..].iter().sum::<f64>()
            / (values.len() - values.len() / 2) as f64;
        let change_percent = ((second_half - first_half) / first_half) * 100.0;
        match change_percent {
            x if x > 10.0 => MetricTrend::Increasing,
            x if x < -10.0 => MetricTrend::Decreasing,
            _ => MetricTrend::Stable,

    pub async fn check_thresholds(
        &self,
        metrics: &PerformanceMetrics,
    ) -> Result<Vec<String>, BearDogError>> {
        let thresholds = self.thresholds.read().await;
        let mut violations = Vec::new();
        if metrics.cpu_usage > thresholds.cpu_threshold {
            violations.push(format!(
                "CPU usage {:.1}% exceeds threshold {:.1}%",
                metrics.cpu_usage, thresholds.cpu_threshold
            ));
        if metrics.memory_usage > thresholds.memory_threshold {
                "Memory usage {:.1}% exceeds threshold {:.1}%",
                metrics.memory_usage, thresholds.memory_threshold
        if metrics.response_time > thresholds.response_time_threshold {
                "Response time {:.1}ms exceeds threshold {:.1}ms",
                metrics.response_time, thresholds.response_time_threshold
        if metrics.error_rate > thresholds.error_rate_threshold {
                "Error rate {:.1}% exceeds threshold {:.1}%",
                metrics.error_rate, thresholds.error_rate_threshold
        Ok(violations)

    async fn collect_cpu_usage(&self) -> Result<f64, BearDogError> {

        Ok(45.0)}

    async fn collect_memory_usage(&self) -> Result<f64, BearDogError> {
        Ok(62.0)
    async fn collect_response_time(&self) -> Result<f64, BearDogError> {
        Ok(250.0)}

    async fn collect_error_rate(&self) -> Result<f64, BearDogError> {
        Ok(1.5)
    async fn collect_throughput(&self) -> Result<f64, BearDogError> {
        Ok(150.0)
