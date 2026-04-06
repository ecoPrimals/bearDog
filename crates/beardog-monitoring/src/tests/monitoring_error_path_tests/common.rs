// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared fixtures and helpers for monitoring error-path tests.

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub fn collect_metrics_from_invalid_source() -> Result<Vec<Metric>, BearDogError> {
    Err(BearDogError::System {
        message: "Invalid metrics source".to_string(),
        category: beardog_errors::SystemErrorCategory::General,
    })
}

pub fn collect_metrics_with_timeout(_timeout_ms: u64) -> Result<Vec<Metric>, BearDogError> {
    Err(BearDogError::Network {
        message: "Metrics collection timeout".to_string(),
        category: beardog_errors::NetworkErrorCategory::Timeout,
    })
}

#[derive(Debug, Clone)]
pub struct Metric {
    pub value: f64,
    pub timestamp: u64,
}

pub struct HealthCheckService {
    components: Mutex<HashMap<String, bool>>,
    timeout_ms: u64,
}

impl HealthCheckService {
    pub fn new() -> Self {
        Self {
            components: Mutex::new(HashMap::new()),
            timeout_ms: 5000,
        }
    }

    pub fn with_timeout(timeout_ms: u64) -> Self {
        Self {
            components: Mutex::new(HashMap::new()),
            timeout_ms,
        }
    }

    pub fn check_service(&self, _service: &str) -> Result<HealthStatus, BearDogError> {
        Err(BearDogError::Network {
            message: "Service unreachable".to_string(),
            category: beardog_errors::NetworkErrorCategory::Connection,
        })
    }

    pub fn check_degraded_service(&self) -> Result<HealthStatus, BearDogError> {
        Ok(HealthStatus {
            healthy: false,
            message: "Service degraded".to_string(),
        })
    }

    pub fn register_component(&self, name: &str, healthy: bool) {
        self.components
            .lock()
            .unwrap()
            .insert(name.to_string(), healthy);
    }

    pub fn check_all(&self) -> Result<AggregateHealthStatus, BearDogError> {
        let components = self.components.lock().unwrap();
        let failed: Vec<String> = components
            .iter()
            .filter(|(_, healthy)| !*healthy)
            .map(|(name, _)| name.clone())
            .collect();

        Ok(AggregateHealthStatus {
            all_healthy: failed.is_empty(),
            failed_components: failed,
        })
    }

    pub fn check_slow_service(&self) -> Result<HealthStatus, BearDogError> {
        if self.timeout_ms < 100 {
            Err(BearDogError::Network {
                message: "Health check timeout".to_string(),
                category: beardog_errors::NetworkErrorCategory::Timeout,
            })
        } else {
            Ok(HealthStatus {
                healthy: true,
                message: "OK".to_string(),
            })
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct HealthStatus {
    pub healthy: bool,
    pub message: String,
}

#[derive(Debug)]
pub struct AggregateHealthStatus {
    pub all_healthy: bool,
    pub failed_components: Vec<String>,
}

pub struct AlertManager {
    rate_limit: Option<usize>,
    sent_count: AtomicU64,
    deduplication: bool,
    last_alert: Mutex<Option<String>>,
    queue_size: usize,
    queued: AtomicU64,
    warning_count: AtomicU64,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            rate_limit: None,
            sent_count: AtomicU64::new(0),
            deduplication: false,
            last_alert: Mutex::new(None),
            queue_size: 1000,
            queued: AtomicU64::new(0),
            warning_count: AtomicU64::new(0),
        }
    }

    pub fn with_rate_limit(limit: usize) -> Self {
        Self {
            rate_limit: Some(limit),
            ..Self::new()
        }
    }

    pub fn with_deduplication(enabled: bool) -> Self {
        Self {
            deduplication: enabled,
            ..Self::new()
        }
    }

    pub fn with_queue_size(size: usize) -> Self {
        Self {
            queue_size: size,
            ..Self::new()
        }
    }

    pub fn send_alert(&self, severity: &str, recipients: Vec<&str>) -> Result<(), BearDogError> {
        if recipients.is_empty() {
            return Err(BearDogError::Business {
                message: "No recipients specified".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        if let Some(limit) = self.rate_limit {
            let count = self.sent_count.fetch_add(1, Ordering::SeqCst);
            if count >= limit as u64 {
                return Err(BearDogError::System {
                    message: "Rate limit exceeded".to_string(),
                    category: beardog_errors::SystemErrorCategory::General,
                });
            }
        }

        if self.deduplication {
            let mut last = self.last_alert.lock().unwrap();
            if last.as_ref().is_some_and(|l| l == severity) {
                return Err(BearDogError::Business {
                    message: "Duplicate alert".to_string(),
                    category: beardog_errors::BusinessErrorCategory::Validation,
                });
            }
            *last = Some(severity.to_string());
        }

        let queued = self.queued.fetch_add(1, Ordering::SeqCst);
        if queued >= self.queue_size as u64 {
            return Err(BearDogError::System {
                message: "Alert queue full".to_string(),
                category: beardog_errors::SystemErrorCategory::General,
            });
        }

        if severity == "warning" {
            self.warning_count.fetch_add(1, Ordering::SeqCst);
        }

        Ok(())
    }

    pub fn check_escalation(&self) -> bool {
        self.warning_count.load(Ordering::SeqCst) >= 5
    }
}

pub struct MetricsAggregator {}

impl MetricsAggregator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn aggregate(&self, metrics: Vec<Metric>) -> Result<f64, BearDogError> {
        if metrics.is_empty() {
            return Err(BearDogError::Business {
                message: "No metrics to aggregate".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        for metric in &metrics {
            if metric.value.is_nan() || metric.value.is_infinite() {
                return Err(BearDogError::Business {
                    message: "Invalid metric value".to_string(),
                    category: beardog_errors::BusinessErrorCategory::Validation,
                });
            }
        }

        let sum: f64 = metrics.iter().map(|m| m.value).sum();
        Ok(sum / metrics.len() as f64)
    }

    pub fn percentile(&self, mut metrics: Vec<Metric>, p: f64) -> Result<f64, BearDogError> {
        if metrics.is_empty() {
            return Err(BearDogError::Business {
                message: "No metrics".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        metrics.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
        let index = ((p / 100.0) * metrics.len() as f64) as usize;
        Ok(metrics[index.min(metrics.len() - 1)].value)
    }
}

pub struct MonitoringServiceWrapper {
    running: AtomicBool,
    metrics: Mutex<Vec<Metric>>,
    memory_limit: Option<usize>,
}

impl MonitoringServiceWrapper {
    pub fn new() -> Self {
        Self {
            running: AtomicBool::new(false),
            metrics: Mutex::new(Vec::new()),
            memory_limit: None,
        }
    }

    pub fn with_memory_limit(limit: usize) -> Self {
        Self {
            running: AtomicBool::new(false),
            metrics: Mutex::new(Vec::new()),
            memory_limit: Some(limit),
        }
    }

    pub fn start(&self) -> Result<(), BearDogError> {
        if self
            .running
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Err(BearDogError::System {
                message: "Service already running".to_string(),
                category: beardog_errors::SystemErrorCategory::General,
            });
        }
        Ok(())
    }

    pub fn stop(&self) -> Result<(), BearDogError> {
        if !self.running.load(Ordering::SeqCst) {
            return Err(BearDogError::System {
                message: "Service not running".to_string(),
                category: beardog_errors::SystemErrorCategory::General,
            });
        }
        self.running.store(false, Ordering::SeqCst);
        Ok(())
    }

    pub fn record_metric(&self, metric: Metric) -> Result<(), BearDogError> {
        let mut metrics = self.metrics.lock().unwrap();

        if let Some(limit) = self.memory_limit {
            let current_size = metrics.len() * std::mem::size_of::<Metric>();
            if current_size >= limit {
                return Err(BearDogError::System {
                    message: "Memory limit exceeded".to_string(),
                    category: beardog_errors::SystemErrorCategory::General,
                });
            }
        }

        metrics.push(metric);
        Ok(())
    }

    pub fn get_stored_metrics(&self) -> Vec<Metric> {
        self.metrics.lock().unwrap().clone()
    }
}

pub struct ThresholdMonitor {
    threshold: f64,
}

impl ThresholdMonitor {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub fn check_violation(&self, value: f64) -> bool {
        value > self.threshold
    }
}

pub struct CircularMetricBuffer {
    values: Vec<f64>,
    capacity: usize,
    position: usize,
}

impl CircularMetricBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::new(),
            capacity,
            position: 0,
        }
    }

    pub fn push(&mut self, value: f64) {
        if self.values.len() < self.capacity {
            self.values.push(value);
        } else {
            self.values[self.position] = value;
            self.position = (self.position + 1) % self.capacity;
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get_values(&self) -> Vec<f64> {
        if self.values.len() < self.capacity {
            self.values.clone()
        } else {
            let mut result = Vec::new();
            for i in 0..self.capacity {
                result.push(self.values[(self.position + i) % self.capacity]);
            }
            result
        }
    }

    pub fn average(&self) -> Option<f64> {
        if self.values.is_empty() {
            None
        } else {
            Some(self.values.iter().sum::<f64>() / self.values.len() as f64)
        }
    }
}

pub struct MetricExporter {}

impl MetricExporter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn export(&self, _metrics: Vec<Metric>, destination: &str) -> Result<(), BearDogError> {
        if !destination.starts_with("http://") && !destination.starts_with("https://") {
            return Err(BearDogError::Configuration {
                message: "Invalid destination URL".to_string(),
                category: beardog_errors::ConfigurationErrorCategory::Validation,
            });
        }

        if destination.contains("unreachable") {
            return Err(BearDogError::Network {
                message: "Connection failed".to_string(),
                category: beardog_errors::NetworkErrorCategory::Connection,
            });
        }

        Ok(())
    }
}

pub struct MetricStorage {
    metrics: Vec<Metric>,
    retention_seconds: u64,
}

impl MetricStorage {
    pub fn with_retention_seconds(seconds: u64) -> Self {
        Self {
            metrics: Vec::new(),
            retention_seconds: seconds,
        }
    }

    pub fn store(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }

    pub fn cleanup_old_metrics(&mut self, current_time: u64) {
        self.metrics
            .retain(|m| current_time - m.timestamp <= self.retention_seconds);
    }

    pub fn get_all(&self) -> &[Metric] {
        &self.metrics
    }
}

pub struct MetricSampler {
    rate: f64,
    counter: AtomicU64,
}

impl MetricSampler {
    pub fn with_rate(rate: f64) -> Self {
        Self {
            rate,
            counter: AtomicU64::new(0),
        }
    }

    pub fn should_sample(&self) -> bool {
        let count = self.counter.fetch_add(1, Ordering::SeqCst);
        (count as f64 * self.rate) % 1.0 < self.rate
    }
}

pub struct SafeCounter {
    value: AtomicU64,
}

impl SafeCounter {
    pub fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
        }
    }

    pub fn set(&self, value: u64) {
        self.value.store(value, Ordering::SeqCst);
    }

    pub fn increment(&self) -> Result<u64, BearDogError> {
        let current = self.value.load(Ordering::SeqCst);
        if current == u64::MAX {
            return Err(BearDogError::System {
                message: "Counter overflow".to_string(),
                category: beardog_errors::SystemErrorCategory::General,
            });
        }
        Ok(self.value.fetch_add(1, Ordering::SeqCst))
    }
}
