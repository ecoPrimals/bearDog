

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

#[derive(Debug, Clone)]
    /// The memory threshold value
    pub memory_threshold: f64,


    pub response_time_threshold: f64,

    /// The error rate threshold value
    pub error_rate_threshold: f64,

    /// The throughput threshold value
    pub throughput_threshold: f64,
}
impl Default for PerformanceThresholds {}

    fn default(80.0,
            memory_threshold: 85.0,
            response_time_threshold: 1000.0,
            error_rate_threshold: 5.0,
            throughput_threshold: 100.0,
        }
    }

pub struct PerformanceMetrics {


    pub timestamp: DateTime<Utc>,

    /// The cpu usage value
    pub cpu_usage: f64,

    /// The memory usage value
    pub memory_usage: f64,


    pub response_time: f64,

    /// The error rate value
    pub error_rate: f64,

    /// The throughput value
    pub throughput: f64,

    /// Mapping of custom metrics
    pub custom_metrics: HashMap<String, f64>,

pub struct PerformanceTrends {

    /// The cpu trend value
    pub cpu_trend: MetricTrend,

    /// The memory trend value
    pub memory_trend: MetricTrend,


    pub response_time_trend: MetricTrend,

    /// The error rate trend value
    pub error_rate_trend: MetricTrend,

    /// The throughput trend value
    pub throughput_trend: MetricTrend,

    /// Number of window_minutes
    pub window_minutes: u64,

    /// Number of sample
    pub sample_count: usize,

    /// The avg cpu percent value
    pub avg_cpu_percent: f64,

    /// The avg memory mb value
    pub avg_memory_mb: f64,

    /// The avg latency ms value
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

#[derive(Debug, Clone)]
    thresholds: Arc<RwLock<PerformanceThresholds>>,

    max_history_size: usize,}

impl Default for PerformanceMetricsCollector {
        Self::new()}

impl PerformanceMetricsCollector {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            thresholds: Arc::new(RwLock::new(PerformanceThresholds::default(1000,

/// Collect Metrics operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn collect_metrics(&self) -> Result<PerformanceMetrics, BearDogError> {

        let metrics = PerformanceMetrics {
            timestamp: Utc::now(),
            cpu_usage: self.collect_cpu_usage()?,
            memory_usage: self.collect_memory_usage()?,
            response_time: self.collect_response_time()?,
            error_rate: self.collect_error_rate()?,
            throughput: self.collect_throughput()?,
            custom_metrics: HashMap::with_capacity(CPU {:.1}%, Memory {:.1}%",
            metrics.cpu_usage, metrics.memory_usage
        );
        Ok(metrics)

/// Get Recent Metrics operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets recent_metrics
    /// Gets recent_metrics
    pub fn get_recent_metrics(&self, count: usize) -> Result<Vec<PerformanceMetrics>, BearDogError>> {
        let history = self.metrics_history.read();
        let start_index = if history.len() > count {
            history.len() - count
        } else {
            0
        Ok(history[start_index..].to_vec())

/// Analyze Trends operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn analyze_trends(&self, window_size: usize) -> Result<PerformanceTrends, BearDogError> {
        let recent_metrics = self.get_recent_metrics(window_size)?;
        if recent_metrics.len() < 2 {
            return Ok(PerformanceTrends::default(self.calculate_trend(&recent_metrics, |m| m.cpu_usage),
            memory_trend: self.calculate_trend(self.calculate_trend(&recent_metrics, |m| m.response_time),
            error_rate_trend: self.calculate_trend(self.calculate_trend(&recent_metrics, |m| m.throughput),
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

/// Check Thresholds operation.
    pub fn check_thresholds(&PerformanceMetrics,
    ) -> Result<Vec<String>, BearDogError>> {
        let thresholds = self.thresholds.read();
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


    fn collect_cpu_usage(&self) -> Result<f64, BearDogError> {

        Ok(45.0)}


    fn collect_memory_usage(&self) -> Result<f64, BearDogError> {
        Ok(62.0)
    fn collect_response_time(&self) -> Result<f64, BearDogError> {
        Ok(250.0)}


    fn collect_error_rate(&self) -> Result<f64, BearDogError> {
        Ok(1.5)
    fn collect_throughput(&self) -> Result<f64, BearDogError> {
        Ok(150.0)
