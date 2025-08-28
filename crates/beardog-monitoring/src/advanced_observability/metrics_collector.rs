

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;

pub struct RealTimeMetricCollector {
    metrics_buffer: Arc<RwLock<MetricsBuffer>>,
    collection_interval: Duration,
    metric_sources: Vec<MetricSource>,
    aggregation_rules: HashMap<String, AggregationRule>,
    retention_policy: RetentionPolicy,
    is_running: Arc<RwLock<bool>>,
}

impl RealTimeMetricCollector {

    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            metrics_buffer: Arc::new(RwLock::new(MetricsBuffer::new())),
            collection_interval: Duration::from_millis(100), // 100ms collection interval
            metric_sources: Vec::new(),
            aggregation_rules: HashMap::with_capacity(16),
            retention_policy: RetentionPolicy::default(),
            is_running: Arc::new(RwLock::new(false)),
        })
    }

    pub async fn initialize(&self) -> Result<(), BearDogError> {

        self.setup_default_sources().await?;

        self.setup_default_aggregation_rules();
        
        Ok(())
    }

    pub async fn start(&self) -> Result<(), BearDogError> {
        *self.is_running.write()
            .map_err(|e| BearDogError::system(format_args!("Failed to acquire write lock: {}", e).to_string()))? = true;

        self.start_collection_loop().await?;
        
        Ok(())
    }

    pub async fn is_healthy(&self) -> Result<bool, BearDogError> {
        let is_running = *self.is_running.read()
            .map_err(|e| BearDogError::system(format_args!("Failed to acquire read lock: {}", e).to_string()))?;
        let buffer_size = self.metrics_buffer.read()
            .map_err(|e| BearDogError::system(format_args!("Failed to acquire read lock: {}", e).to_string()))?.len();

        Ok(is_running && buffer_size < 10000)
    }

    pub async fn shutdown(&self) -> Result<(), BearDogError> {
        *self.is_running.write()
            .map_err(|e| BearDogError::system(format_args!("Failed to acquire write lock: {}", e).to_string()))? = false;
        Ok(())
    }

    pub async fn collect_metrics(&self) -> Result<Vec<Metric>, BearDogError>> {
        let mut all_metrics = Vec::new();
        
        for source in &self.metric_sources {
            match source.collect().await {
                Ok(mut metrics) => all_metrics.append(&mut metrics),
                Err(e) => {

                    eprintln!("Failed to collect from source {}: {}", source.name, e);
                }
            }
        }
        
        Ok(all_metrics)
    }

    pub async fn add_metrics(&self, metrics: Vec<Metric>) -> Result<(), BearDogError> {
        let mut buffer = self.metrics_buffer.write().map_err(|e| {
            BearDogError::system(format_args!("Failed to acquire metrics buffer write lock: {}", e).to_string())
        })?;
        
        for metric in metrics {

            if let Some(rule) = self.aggregation_rules.get(&metric.name) {
                let aggregated = rule.apply(&metric)?;
                buffer.add(aggregated);
            } else {
                buffer.add(metric);
            }
        }

        buffer.apply_retention(&self.retention_policy);
        
        Ok(())
    }

    pub async fn get_metrics_snapshot(&self) -> Result<MetricsSnapshot, BearDogError> {
        let buffer = self.metrics_buffer.read().map_err(|e| {
            BearDogError::system(format_args!("Failed to acquire metrics buffer read lock: {}", e).to_string())
        })?;
        Ok(buffer.create_snapshot())
    }

    async fn setup_default_sources(&mut self) -> Result<(), BearDogError> {

        self.metric_sources.push(MetricSource {
            name: "system".to_string(),
            source_type: MetricSourceType::System,
            collection_interval: Duration::from_millis(1000),
            enabled: true,
        });

        self.metric_sources.push(MetricSource {
            name: "application".to_string(),
            source_type: MetricSourceType::Application,
            collection_interval: Duration::from_millis(500),
            enabled: true,
        });

        Ok(())
    }

    fn setup_default_aggregation_rules(&mut self) {

        self.aggregation_rules.insert(
            "cpu.usage".to_string(),
            AggregationRule {
                function: AggregationFunction::Average,
                window: Duration::from_secs(60),
            }
        );

        self.aggregation_rules.insert(
            "memory.usage".to_string(),
            AggregationRule {
                function: AggregationFunction::Maximum,
                window: Duration::from_secs(60),
            }
        );

        self.aggregation_rules.insert(
            "requests.count".to_string(),
            AggregationRule {
                function: AggregationFunction::Sum,
                window: Duration::from_secs(60),
            }
        );
    }

    async fn start_collection_loop(&self) -> Result<(), BearDogError> {
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

#[derive(Debug, Clone)]
pub struct MetricSource {
    pub name: String,
    pub source_type: MetricSourceType,
    pub collection_interval: Duration,
    pub enabled: bool,
}

impl MetricSource {

    pub async fn collect(&self) -> Result<Vec<Metric>, BearDogError>> {
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

    async fn collect_system_metrics(&self) -> Result<Vec<Metric>, BearDogError>> {
        let mut metrics = Vec::new();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        metrics.push(Metric {
            name: "cpu.usage".to_string(),
            value: 45.2, // Simulated CPU usage percentage
            timestamp,
            tags: vec![("source".to_string(), "system".to_string())],
            metric_type: MetricType::Gauge,
        });

        metrics.push(Metric {
            name: "memory.usage".to_string(),
            value: 1024.0 * 1024.0 * 512.0, // 512MB in bytes
            timestamp,
            tags: vec![("source".to_string(), "system".to_string())],
            metric_type: MetricType::Gauge,
        });

        Ok(metrics)
    }

    async fn collect_application_metrics(&self) -> Result<Vec<Metric>, BearDogError>> {
        let mut metrics = Vec::new();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        metrics.push(Metric {
            name: "requests.count".to_string(),
            value: 150.0, // Requests per minute
            timestamp,
            tags: vec![("source".to_string(), "application".to_string())],
            metric_type: MetricType::Counter,
        });

        metrics.push(Metric {
            name: "response.time".to_string(),
            value: 25.5, // Average response time in milliseconds
            timestamp,
            tags: vec![("source".to_string(), "application".to_string())],
            metric_type: MetricType::Histogram,
        });

        Ok(metrics)
    }

    async fn collect_network_metrics(&self) -> Result<Vec<Metric>, BearDogError>> {

        Ok(Vec::new())
    }

    async fn collect_custom_metrics(&self) -> Result<Vec<Metric>, BearDogError>> {

        Ok(Vec::new())
    }
}

#[derive(Debug, Clone)]
pub enum MetricSourceType {
    System,
    Application,
    Network,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub timestamp: u64,
    pub tags: Vec<(String, String)>,
    pub metric_type: MetricType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

#[derive(Debug, Clone)]
pub struct AggregationRule {
    pub function: AggregationFunction,
    pub window: Duration,
}

impl AggregationRule {

    pub fn apply(&self, metric: &Metric) -> Result<Metric, BearDogError> {

        Ok(metric.clone())
    }
}

#[derive(Debug, Clone)]
pub enum AggregationFunction {
    Sum,
    Average,
    Maximum,
    Minimum,
    Count,
}

pub struct MetricsBuffer {
    metrics: VecDeque<Metric>,
    max_size: usize,
}

impl MetricsBuffer {

    pub fn new() -> Self {
        Self {
            metrics: VecDeque::new(),
            max_size: 10000, // Default buffer size
        }
    }

    pub fn add(&mut self, metric: Metric) {
        self.metrics.push_back(metric);

        while self.metrics.len() > self.max_size {
            self.metrics.pop_front();
        }
    }

    pub fn len(&self) -> usize {
        self.metrics.len()
    }

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub metrics: Vec<Metric>,
    pub snapshot_time: u64,
} 