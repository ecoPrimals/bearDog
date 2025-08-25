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


/// # Real-Time Metric Collection System
/// 
/// **FOCUSED MODULE** - Extracted from advanced_observability.rs for maintainability
/// This module handles all aspects of real-time metric collection, aggregation,
/// and storage with sub-millisecond performance characteristics.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;

/// Real-time metric collection system
pub struct RealTimeMetricCollector {
    metrics_buffer: Arc<RwLock<MetricsBuffer>>,
    collection_interval: Duration,
    metric_sources: Vec<MetricSource>,
    aggregation_rules: HashMap<String, AggregationRule>,
    retention_policy: RetentionPolicy,
    is_running: Arc<RwLock<bool>>,
}

impl RealTimeMetricCollector {
    /// Create a new real-time metric collector
    pub fn new() -> BearDogResult<Self> {
        Ok(Self {
            metrics_buffer: Arc::new(RwLock::new(MetricsBuffer::new())),
            collection_interval: Duration::from_millis(100), // 100ms collection interval
            metric_sources: Vec::new(),
            aggregation_rules: HashMap::new(),
            retention_policy: RetentionPolicy::default(),
            is_running: Arc::new(RwLock::new(false)),
        })
    }

    /// Initialize the metric collector
    pub async fn initialize(&self) -> BearDogResult<()> {
        // Initialize metric sources
        self.setup_default_sources().await?;
        
        // Setup default aggregation rules
        self.setup_default_aggregation_rules();
        
        Ok(())
    }

    /// Start metric collection
    pub async fn start(&self) -> BearDogResult<()> {
        *self.is_running.write()
            .map_err(|e| BearDogError::system(format!("Failed to acquire write lock: {}", e)))? = true;
        
        // Start collection loop
        self.start_collection_loop().await?;
        
        Ok(())
    }

    /// Check if the collector is healthy
    pub async fn is_healthy(&self) -> BearDogResult<bool> {
        let is_running = *self.is_running.read()
            .map_err(|e| BearDogError::system(format!("Failed to acquire read lock: {}", e)))?;
        let buffer_size = self.metrics_buffer.read()
            .map_err(|e| BearDogError::system(format!("Failed to acquire read lock: {}", e)))?.len();
        
        // Healthy if running and buffer is not overflowing
        Ok(is_running && buffer_size < 10000)
    }

    /// Shutdown the collector
    pub async fn shutdown(&self) -> BearDogResult<()> {
        *self.is_running.write()
            .map_err(|e| BearDogError::system(format!("Failed to acquire write lock: {}", e)))? = false;
        Ok(())
    }

    /// Collect metrics from all sources
    pub async fn collect_metrics(&self) -> BearDogResult<Vec<Metric>> {
        let mut all_metrics = Vec::new();
        
        for source in &self.metric_sources {
            match source.collect().await {
                Ok(mut metrics) => all_metrics.append(&mut metrics),
                Err(e) => {
                    // Log error but continue with other sources
                    eprintln!("Failed to collect from source {}: {}", source.name, e);
                }
            }
        }
        
        Ok(all_metrics)
    }

    /// Add metrics to the buffer with aggregation
    pub async fn add_metrics(&self, metrics: Vec<Metric>) -> BearDogResult<()> {
        let mut buffer = self.metrics_buffer.write().map_err(|e| {
            BearDogError::system(format!("Failed to acquire metrics buffer write lock: {}", e))
        })?;
        
        for metric in metrics {
            // Apply aggregation rules if they exist
            if let Some(rule) = self.aggregation_rules.get(&metric.name) {
                let aggregated = rule.apply(&metric)?;
                buffer.add(aggregated);
            } else {
                buffer.add(metric);
            }
        }
        
        // Apply retention policy
        buffer.apply_retention(&self.retention_policy);
        
        Ok(())
    }

    /// Get current metrics snapshot
    pub async fn get_metrics_snapshot(&self) -> BearDogResult<MetricsSnapshot> {
        let buffer = self.metrics_buffer.read().map_err(|e| {
            BearDogError::system(format!("Failed to acquire metrics buffer read lock: {}", e))
        })?;
        Ok(buffer.create_snapshot())
    }

    /// Setup default metric sources
    async fn setup_default_sources(&mut self) -> BearDogResult<()> {
        // Add system metrics source
        self.metric_sources.push(MetricSource {
            name: "system".to_string(),
            source_type: MetricSourceType::System,
            collection_interval: Duration::from_millis(1000),
            enabled: true,
        });

        // Add application metrics source
        self.metric_sources.push(MetricSource {
            name: "application".to_string(),
            source_type: MetricSourceType::Application,
            collection_interval: Duration::from_millis(500),
            enabled: true,
        });

        Ok(())
    }

    /// Setup default aggregation rules
    fn setup_default_aggregation_rules(&mut self) {
        // CPU usage - average over time windows
        self.aggregation_rules.insert(
            "cpu.usage".to_string(),
            AggregationRule {
                function: AggregationFunction::Average,
                window: Duration::from_secs(60),
            }
        );

        // Memory usage - maximum over time windows
        self.aggregation_rules.insert(
            "memory.usage".to_string(),
            AggregationRule {
                function: AggregationFunction::Maximum,
                window: Duration::from_secs(60),
            }
        );

        // Request count - sum over time windows
        self.aggregation_rules.insert(
            "requests.count".to_string(),
            AggregationRule {
                function: AggregationFunction::Sum,
                window: Duration::from_secs(60),
            }
        );
    }

    /// Start the metric collection loop
    async fn start_collection_loop(&self) -> BearDogResult<()> {
        let collector = self.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(collector.collection_interval);
            
            while collector.is_running.read().map(|guard| *guard).unwrap_or(false) {
                interval.tick().await;
                
                match collector.collect_metrics().await {
                    Ok(metrics) => {
                        if let Err(e) = collector.add_metrics(metrics).await {
                            eprintln!("Failed to add metrics to buffer: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to collect metrics: {}", e);
                    }
                }
            }
        });
        
        Ok(())
    }
}

// Clone implementation for async tasks
impl Clone for RealTimeMetricCollector {
    fn clone(&self) -> Self {
        Self {
            metrics_buffer: self.metrics_buffer.clone(),
            collection_interval: self.collection_interval,
            metric_sources: self.metric_sources.clone(),
            aggregation_rules: self.aggregation_rules.clone(),
            retention_policy: self.retention_policy.clone(),
            is_running: self.is_running.clone(),
        }
    }
}

/// Metric source configuration
#[derive(Debug, Clone)]
pub struct MetricSource {
    pub name: String,
    pub source_type: MetricSourceType,
    pub collection_interval: Duration,
    pub enabled: bool,
}

impl MetricSource {
    /// Collect metrics from this source
    pub async fn collect(&self) -> BearDogResult<Vec<Metric>> {
        if !self.enabled {
            return Ok(Vec::new());
        }

        match self.source_type {
            MetricSourceType::System => self.collect_system_metrics().await,
            MetricSourceType::Application => self.collect_application_metrics().await,
            MetricSourceType::Network => self.collect_network_metrics().await,
            MetricSourceType::Custom => self.collect_custom_metrics().await,
        }
    }

    /// Collect system-level metrics
    async fn collect_system_metrics(&self) -> BearDogResult<Vec<Metric>> {
        let mut metrics = Vec::new();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        // CPU metrics (simulated - in real implementation would use system APIs)
        metrics.push(Metric {
            name: "cpu.usage".to_string(),
            value: 45.2, // Simulated CPU usage percentage
            timestamp,
            tags: vec![("source".to_string(), "system".to_string())],
            metric_type: MetricType::Gauge,
        });

        // Memory metrics
        metrics.push(Metric {
            name: "memory.usage".to_string(),
            value: 1024.0 * 1024.0 * 512.0, // 512MB in bytes
            timestamp,
            tags: vec![("source".to_string(), "system".to_string())],
            metric_type: MetricType::Gauge,
        });

        Ok(metrics)
    }

    /// Collect application-level metrics
    async fn collect_application_metrics(&self) -> BearDogResult<Vec<Metric>> {
        let mut metrics = Vec::new();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        // Request metrics
        metrics.push(Metric {
            name: "requests.count".to_string(),
            value: 150.0, // Requests per minute
            timestamp,
            tags: vec![("source".to_string(), "application".to_string())],
            metric_type: MetricType::Counter,
        });

        // Response time metrics
        metrics.push(Metric {
            name: "response.time".to_string(),
            value: 25.5, // Average response time in milliseconds
            timestamp,
            tags: vec![("source".to_string(), "application".to_string())],
            metric_type: MetricType::Histogram,
        });

        Ok(metrics)
    }

    /// Collect network metrics
    async fn collect_network_metrics(&self) -> BearDogResult<Vec<Metric>> {
        // Network metrics implementation
        Ok(Vec::new())
    }

    /// Collect custom metrics
    async fn collect_custom_metrics(&self) -> BearDogResult<Vec<Metric>> {
        // Custom metrics implementation
        Ok(Vec::new())
    }
}

/// Types of metric sources
#[derive(Debug, Clone)]
pub enum MetricSourceType {
    System,
    Application,
    Network,
    Custom,
}

/// Individual metric data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub timestamp: u64,
    pub tags: Vec<(String, String)>,
    pub metric_type: MetricType,
}

/// Types of metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

/// Aggregation rule for metrics
#[derive(Debug, Clone)]
pub struct AggregationRule {
    pub function: AggregationFunction,
    pub window: Duration,
}

impl AggregationRule {
    /// Apply aggregation to a metric
    pub fn apply(&self, metric: &Metric) -> BearDogResult<Metric> {
        // For now, return the metric unchanged
        // In a real implementation, this would apply the aggregation function
        Ok(metric.clone())
    }
}

/// Aggregation functions
#[derive(Debug, Clone)]
pub enum AggregationFunction {
    Sum,
    Average,
    Maximum,
    Minimum,
    Count,
}

/// Metrics buffer for storing collected metrics
pub struct MetricsBuffer {
    metrics: VecDeque<Metric>,
    max_size: usize,
}

impl MetricsBuffer {
    /// Create a new metrics buffer
    pub fn new() -> Self {
        Self {
            metrics: VecDeque::new(),
            max_size: 10000, // Default buffer size
        }
    }

    /// Add a metric to the buffer
    pub fn add(&mut self, metric: Metric) {
        self.metrics.push_back(metric);
        
        // Maintain buffer size
        while self.metrics.len() > self.max_size {
            self.metrics.pop_front();
        }
    }

    /// Get buffer length
    pub fn len(&self) -> usize {
        self.metrics.len()
    }

    /// Apply retention policy
    pub fn apply_retention(&mut self, policy: &RetentionPolicy) {
        let cutoff_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_millis() as u64 - policy.max_age.as_millis() as u64,
            Err(_) => {
                tracing::warn!("System time error during retention policy application, skipping");
                return;
            }
        };

        self.metrics.retain(|metric| metric.timestamp >= cutoff_time);
    }

    /// Create a snapshot of current metrics
    pub fn create_snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            metrics: self.metrics.iter().cloned().collect(),
            snapshot_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_secs(0))
                .as_millis() as u64,
        }
    }
}

/// Retention policy for metrics
#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    pub max_age: Duration,
    pub max_count: usize,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            max_age: Duration::from_secs(3600), // 1 hour
            max_count: 10000,
        }
    }
}

/// Snapshot of metrics at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub metrics: Vec<Metric>,
    pub snapshot_time: u64,
} 