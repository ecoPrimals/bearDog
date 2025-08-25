// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Performance Metrics Collection and Analysis
///
/// Provides metrics collection, analysis, and trend detection for performance monitoring.

use beardog_errors::BearDogResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;
/// Performance thresholds for alerting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// CPU usage threshold (percentage)
    pub cpu_threshold: f64,
    /// Memory usage threshold (percentage)
    pub memory_threshold: f64,
    /// Response time threshold (milliseconds)
    pub response_time_threshold: f64,
    /// Error rate threshold (percentage)
    pub error_rate_threshold: f64,
    /// Throughput threshold (requests per second)
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
/// Performance metrics data point
pub struct PerformanceMetrics {
    /// Timestamp of the metrics
    pub timestamp: DateTime<Utc>,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Average response time in milliseconds
    pub response_time: f64,
    /// Error rate percentage
    pub error_rate: f64,
    /// Requests per second
    pub throughput: f64,
    /// Additional custom metrics
    pub custom_metrics: HashMap<String, f64>,
/// Performance trend analysis
pub struct PerformanceTrends {
    /// CPU usage trend
    pub cpu_trend: MetricTrend,
    /// Memory usage trend
    pub memory_trend: MetricTrend,
    /// Response time trend
    pub response_time_trend: MetricTrend,
    /// Error rate trend
    pub error_rate_trend: MetricTrend,
    /// Throughput trend
    pub throughput_trend: MetricTrend,
    /// Analysis window in minutes
    pub window_minutes: u64,
    /// Number of samples analyzed
    pub sample_count: usize,
    /// Average CPU percentage
    pub avg_cpu_percent: f64,
    /// Average memory usage in MB
    pub avg_memory_mb: f64,
    /// Average latency in milliseconds
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
/// Trend direction for metrics}


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MetricTrend {
    /// Metric is increasing
    Increasing,
    /// Metric is decreasing
    Decreasing,
    /// Metric is stable/unchanged
    Stable,
    /// Trend is volatile/unpredictable
    Volatile,
/// Performance metrics collector
#[derive(Debug)]}


pub struct PerformanceMetricsCollector {
    /// Historical metrics
    metrics_history: Arc<RwLock<Vec<PerformanceMetrics>>>,
    /// Performance thresholds
    thresholds: Arc<RwLock<PerformanceThresholds>>,
    /// Maximum history size
    max_history_size: usize,}


impl Default for PerformanceMetricsCollector {
        Self::new()}


impl PerformanceMetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            thresholds: Arc::new(RwLock::new(PerformanceThresholds::default())),
            max_history_size: 1000,
    /// Collect current performance metrics}


    pub async fn collect_metrics(&self) -> BearDogResult<PerformanceMetrics> {
        // This is a placeholder implementation
        // In a real system, this would collect actual system metrics
        let metrics = PerformanceMetrics {
            timestamp: Utc::now(),
            cpu_usage: self.collect_cpu_usage().await?,
            memory_usage: self.collect_memory_usage().await?,
            response_time: self.collect_response_time().await?,
            error_rate: self.collect_error_rate().await?,
            throughput: self.collect_throughput().await?,
            custom_metrics: HashMap::new(),
        };
        // Store in history
        {
            let mut history = self.metrics_history.write().await;
            history.push(metrics.clone());
            // Trim history if needed
            if history.len() > self.max_history_size {
                history.remove(0);
            }
        debug!(
            "Collected performance metrics: CPU {:.1}%, Memory {:.1}%",
            metrics.cpu_usage, metrics.memory_usage
        );
        Ok(metrics)
    /// Get recent metrics for analysis
    pub async fn get_recent_metrics(&self, count: usize) -> BearDogResult<Vec<PerformanceMetrics>> {
        let history = self.metrics_history.read().await;
        let start_index = if history.len() > count {
            history.len() - count
        } else {
            0
        Ok(history[start_index..].to_vec())
    /// Analyze performance trends
    pub async fn analyze_trends(&self, window_size: usize) -> BearDogResult<PerformanceTrends> {
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
    /// Calculate trend for a specific metric
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
    /// Check if any metrics exceed thresholds}


    pub async fn check_thresholds(
        &self,
        metrics: &PerformanceMetrics,
    ) -> BearDogResult<Vec<String>> {
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
    // Placeholder methods for actual metric collection
    // In a real implementation, these would interface with system APIs
    async fn collect_cpu_usage(&self) -> BearDogResult<f64> {
        // Placeholder: return a mock value
        Ok(45.0)}


    async fn collect_memory_usage(&self) -> BearDogResult<f64> {
        Ok(62.0)
    async fn collect_response_time(&self) -> BearDogResult<f64> {
        Ok(250.0)}


    async fn collect_error_rate(&self) -> BearDogResult<f64> {
        Ok(1.5)
    async fn collect_throughput(&self) -> BearDogResult<f64> {
        Ok(150.0)
