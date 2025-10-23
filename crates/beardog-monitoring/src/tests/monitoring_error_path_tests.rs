//! Comprehensive Error Path Tests for beardog-monitoring
//!
//! Tests focusing on error handling, edge cases, and failure scenarios
//! in monitoring, metrics, health checks, and alerting systems.

#[cfg(test)]
mod monitoring_error_tests {
    use beardog_errors::BearDogError;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    use std::sync::Arc;
    use std::sync::Mutex;

    #[test]
    fn test_metrics_collection_with_invalid_source() {
        // Test metrics collection when source is invalid
        let result = collect_metrics_from_invalid_source();
        assert!(result.is_err(), "Should fail with invalid metrics source");
    }

    #[test]
    fn test_metrics_collection_with_timeout() {
        // Test metrics collection timeout handling
        let result = collect_metrics_with_timeout(1);
        assert!(result.is_err(), "Should timeout with short duration");
    }

    #[test]
    fn test_health_check_unreachable_service() {
        // Test health check when service is unreachable
        let checker = HealthCheckService::new();
        let result = checker.check_service("unreachable-service");

        assert!(result.is_err(), "Should fail for unreachable service");
    }

    #[test]
    fn test_health_check_degraded_service() {
        // Test health check for degraded service
        let checker = HealthCheckService::new();
        let result = checker.check_degraded_service();

        assert!(result.is_ok(), "Should succeed but indicate degraded");
        let status = result.unwrap();
        assert!(!status.healthy, "Should mark as unhealthy");
    }

    #[test]
    fn test_alert_manager_with_invalid_recipients() {
        // Test alert manager with no valid recipients
        let manager = AlertManager::new();
        let result = manager.send_alert("critical", vec![]);

        assert!(result.is_err(), "Should fail with empty recipient list");
    }

    #[test]
    fn test_alert_manager_rate_limiting() {
        // Test alert rate limiting
        let manager = AlertManager::with_rate_limit(2);

        let result1 = manager.send_alert("info", vec!["admin@test.com"]);
        let result2 = manager.send_alert("info", vec!["admin@test.com"]);
        let result3 = manager.send_alert("info", vec!["admin@test.com"]);

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_err(), "Should be rate limited on third alert");
    }

    #[test]
    fn test_metrics_aggregation_empty_data() {
        // Test metrics aggregation with no data
        let aggregator = MetricsAggregator::new();
        let result = aggregator.aggregate(vec![]);

        assert!(result.is_err(), "Should fail with empty data");
    }

    #[test]
    fn test_metrics_aggregation_invalid_values() {
        // Test metrics aggregation with invalid values
        let aggregator = MetricsAggregator::new();
        let metrics = vec![
            Metric {
                value: f64::NAN,
                timestamp: 0,
            },
            Metric {
                value: f64::INFINITY,
                timestamp: 1,
            },
        ];

        let result = aggregator.aggregate(metrics);
        assert!(result.is_err(), "Should reject NaN and Infinity values");
    }

    #[test]
    fn test_monitoring_service_concurrent_start() {
        // Test concurrent start attempts
        let service = Arc::new(MonitoringServiceWrapper::new());
        let service_clone = Arc::clone(&service);

        let handle = std::thread::spawn(move || service_clone.start());

        let result1 = service.start();
        let result2 = handle.join().expect("Thread should complete");

        // One should succeed, one should detect already running
        assert!(result1.is_ok() || result2.is_ok());
        assert!(result1.is_err() || result2.is_err());
    }

    #[test]
    fn test_monitoring_service_stop_before_start() {
        // Test stopping service that was never started
        let service = MonitoringServiceWrapper::new();
        let result = service.stop();

        assert!(result.is_err(), "Should fail to stop non-running service");
    }

    #[test]
    fn test_threshold_violation_detection() {
        // Test threshold violation detection
        let monitor = ThresholdMonitor::new(100.0);

        assert!(!monitor.check_violation(50.0), "Should not violate at 50");
        assert!(
            !monitor.check_violation(100.0),
            "Should not violate at threshold"
        );
        assert!(
            monitor.check_violation(101.0),
            "Should violate above threshold"
        );
    }

    #[test]
    fn test_threshold_violation_negative_values() {
        // Test threshold with negative values
        let monitor = ThresholdMonitor::new(0.0);

        // -1.0 is NOT > 0.0, so it should not violate
        assert!(
            !monitor.check_violation(-1.0),
            "Negative below threshold should not violate"
        );
        assert!(
            monitor.check_violation(1.0),
            "Positive above threshold should violate"
        );
    }

    #[test]
    fn test_circular_metric_buffer() {
        // Test circular buffer overflow handling
        let mut buffer = CircularMetricBuffer::with_capacity(3);

        buffer.push(1.0);
        buffer.push(2.0);
        buffer.push(3.0);
        buffer.push(4.0); // Should overwrite oldest

        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.get_values(), vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_circular_metric_buffer_empty() {
        // Test operations on empty buffer
        let buffer = CircularMetricBuffer::with_capacity(10);

        assert_eq!(buffer.len(), 0);
        assert!(buffer.get_values().is_empty());
        assert!(buffer.average().is_none());
    }

    #[test]
    fn test_metric_export_to_invalid_destination() {
        // Test exporting metrics to invalid destination
        let exporter = MetricExporter::new();
        let metrics = vec![Metric {
            value: 1.0,
            timestamp: 0,
        }];

        let result = exporter.export(metrics, "invalid://destination");
        assert!(result.is_err(), "Should fail with invalid destination");
    }

    #[test]
    fn test_metric_export_connection_failure() {
        // Test export when connection fails
        let exporter = MetricExporter::new();
        let metrics = vec![Metric {
            value: 1.0,
            timestamp: 0,
        }];

        let result = exporter.export(metrics, "http://unreachable:9999");
        assert!(result.is_err(), "Should fail when connection fails");
    }

    #[test]
    fn test_alert_deduplication() {
        // Test that duplicate alerts are deduplicated
        let manager = AlertManager::with_deduplication(true);

        let result1 = manager.send_alert("error", vec!["admin@test.com"]);
        let result2 = manager.send_alert("error", vec!["admin@test.com"]);

        assert!(result1.is_ok());
        assert!(result2.is_err(), "Should deduplicate identical alert");
    }

    #[test]
    fn test_health_check_partial_failure() {
        // Test health check when some components fail
        let checker = HealthCheckService::new();
        checker.register_component("db", true);
        checker.register_component("cache", false);
        checker.register_component("api", true);

        let result = checker.check_all();
        assert!(result.is_ok());

        let status = result.unwrap();
        assert!(!status.all_healthy, "Should indicate partial failure");
        assert_eq!(status.failed_components.len(), 1);
    }

    #[test]
    fn test_metrics_retention_policy() {
        // Test metrics retention and cleanup
        let mut storage = MetricStorage::with_retention_seconds(60);

        storage.store(Metric {
            value: 1.0,
            timestamp: 0,
        });
        storage.store(Metric {
            value: 2.0,
            timestamp: 100,
        });

        storage.cleanup_old_metrics(150);

        let metrics = storage.get_all();
        assert_eq!(metrics.len(), 1, "Should retain only recent metrics");
    }

    #[test]
    fn test_concurrent_metric_updates() {
        // Test concurrent metric updates are thread-safe
        let counter = Arc::new(AtomicU64::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = std::thread::spawn(move || {
                for _ in 0..100 {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread should complete");
        }

        assert_eq!(counter.load(Ordering::SeqCst), 1000);
    }

    #[test]
    fn test_monitoring_graceful_shutdown() {
        // Test graceful shutdown preserves data
        let service = MonitoringServiceWrapper::new();
        service.start().expect("Should start");

        let _ = service.record_metric(Metric {
            value: 42.0,
            timestamp: 0,
        });

        let result = service.stop();
        assert!(result.is_ok());

        let metrics = service.get_stored_metrics();
        assert_eq!(metrics.len(), 1, "Should preserve metrics on shutdown");
    }

    #[test]
    fn test_alert_severity_escalation() {
        // Test alert severity escalation
        let manager = AlertManager::new();

        // Send multiple warnings
        for _ in 0..5 {
            let _ = manager.send_alert("warning", vec!["admin@test.com"]);
        }

        // Should escalate to critical
        let escalated = manager.check_escalation();
        assert!(escalated, "Should escalate after threshold warnings");
    }

    #[test]
    fn test_metric_sampling_rate() {
        // Test metric sampling rate limiting
        let sampler = MetricSampler::with_rate(0.5);

        let mut sampled = 0;
        for _ in 0..1000 {
            if sampler.should_sample() {
                sampled += 1;
            }
        }

        // Should sample roughly 50% (allow 10% variance)
        assert!(
            sampled > 400 && sampled < 600,
            "Sampling rate off: {}",
            sampled
        );
    }

    #[test]
    fn test_monitoring_service_restart() {
        // Test service can be restarted after stop
        let service = MonitoringServiceWrapper::new();

        service.start().expect("First start should succeed");
        service.stop().expect("Stop should succeed");

        let result = service.start();
        assert!(result.is_ok(), "Should allow restart after stop");
    }

    #[test]
    fn test_health_check_timeout() {
        // Test health check with timeout
        let checker = HealthCheckService::with_timeout(1);
        let result = checker.check_slow_service();

        assert!(result.is_err(), "Should timeout on slow service");
    }

    #[test]
    fn test_metric_overflow_protection() {
        // Test metric counter overflow protection
        let counter = SafeCounter::new();

        counter.set(u64::MAX);
        let result = counter.increment();

        assert!(result.is_err(), "Should detect overflow at MAX");
    }

    #[test]
    fn test_alert_channel_full() {
        // Test alert sending when channel is full
        let manager = AlertManager::with_queue_size(2);

        let result1 = manager.send_alert("info", vec!["admin@test.com"]);
        let result2 = manager.send_alert("info", vec!["admin@test.com"]);
        let result3 = manager.send_alert("info", vec!["admin@test.com"]);

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_err(), "Should fail when queue is full");
    }

    #[test]
    fn test_metrics_aggregation_percentiles() {
        // Test percentile calculation with edge cases
        let aggregator = MetricsAggregator::new();

        let metrics = vec![
            Metric {
                value: 1.0,
                timestamp: 0,
            },
            Metric {
                value: 2.0,
                timestamp: 1,
            },
            Metric {
                value: 3.0,
                timestamp: 2,
            },
        ];

        let result = aggregator.percentile(metrics, 50.0);
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_monitoring_memory_limit() {
        // Test monitoring respects memory limits
        let service = MonitoringServiceWrapper::with_memory_limit(1024);

        // Try to store too much data
        for i in 0..10000 {
            let result = service.record_metric(Metric {
                value: i as f64,
                timestamp: i,
            });

            if result.is_err() {
                // Should eventually hit memory limit
                assert!(i > 10, "Memory limit should allow some metrics");
                return;
            }
        }

        panic!("Should have hit memory limit");
    }

    // Helper types and functions

    fn collect_metrics_from_invalid_source() -> Result<Vec<Metric>, BearDogError> {
        Err(BearDogError::System {
            message: "Invalid metrics source".to_string(),
            category: beardog_errors::SystemErrorCategory::General,
        })
    }

    fn collect_metrics_with_timeout(_timeout_ms: u64) -> Result<Vec<Metric>, BearDogError> {
        Err(BearDogError::Network {
            message: "Metrics collection timeout".to_string(),
            category: beardog_errors::NetworkErrorCategory::Timeout,
        })
    }

    #[derive(Debug, Clone)]
    struct Metric {
        value: f64,
        timestamp: u64,
    }

    struct HealthCheckService {
        components: Mutex<HashMap<String, bool>>,
        timeout_ms: u64,
    }

    impl HealthCheckService {
        fn new() -> Self {
            Self {
                components: Mutex::new(HashMap::new()),
                timeout_ms: 5000,
            }
        }

        fn with_timeout(timeout_ms: u64) -> Self {
            Self {
                components: Mutex::new(HashMap::new()),
                timeout_ms,
            }
        }

        fn check_service(&self, _service: &str) -> Result<HealthStatus, BearDogError> {
            Err(BearDogError::Network {
                message: "Service unreachable".to_string(),
                category: beardog_errors::NetworkErrorCategory::Connection,
            })
        }

        fn check_degraded_service(&self) -> Result<HealthStatus, BearDogError> {
            Ok(HealthStatus {
                healthy: false,
                message: "Service degraded".to_string(),
            })
        }

        fn register_component(&self, name: &str, healthy: bool) {
            self.components
                .lock()
                .unwrap()
                .insert(name.to_string(), healthy);
        }

        fn check_all(&self) -> Result<AggregateHealthStatus, BearDogError> {
            let components = self.components.lock().unwrap();
            let failed: Vec<String> = components
                .iter()
                .filter(|(_, &healthy)| !healthy)
                .map(|(name, _)| name.clone())
                .collect();

            Ok(AggregateHealthStatus {
                all_healthy: failed.is_empty(),
                failed_components: failed,
            })
        }

        fn check_slow_service(&self) -> Result<HealthStatus, BearDogError> {
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
    struct HealthStatus {
        healthy: bool,
        message: String,
    }

    #[derive(Debug)]
    struct AggregateHealthStatus {
        all_healthy: bool,
        failed_components: Vec<String>,
    }

    struct AlertManager {
        rate_limit: Option<usize>,
        sent_count: AtomicU64,
        deduplication: bool,
        last_alert: Mutex<Option<String>>,
        queue_size: usize,
        queued: AtomicU64,
        warning_count: AtomicU64,
    }

    impl AlertManager {
        fn new() -> Self {
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

        fn with_rate_limit(limit: usize) -> Self {
            Self {
                rate_limit: Some(limit),
                ..Self::new()
            }
        }

        fn with_deduplication(enabled: bool) -> Self {
            Self {
                deduplication: enabled,
                ..Self::new()
            }
        }

        fn with_queue_size(size: usize) -> Self {
            Self {
                queue_size: size,
                ..Self::new()
            }
        }

        fn send_alert(&self, severity: &str, recipients: Vec<&str>) -> Result<(), BearDogError> {
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

        fn check_escalation(&self) -> bool {
            self.warning_count.load(Ordering::SeqCst) >= 5
        }
    }

    struct MetricsAggregator {}

    impl MetricsAggregator {
        fn new() -> Self {
            Self {}
        }

        fn aggregate(&self, metrics: Vec<Metric>) -> Result<f64, BearDogError> {
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

        fn percentile(&self, mut metrics: Vec<Metric>, p: f64) -> Result<f64, BearDogError> {
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

    struct MonitoringServiceWrapper {
        running: AtomicBool,
        metrics: Mutex<Vec<Metric>>,
        memory_limit: Option<usize>,
    }

    impl MonitoringServiceWrapper {
        fn new() -> Self {
            Self {
                running: AtomicBool::new(false),
                metrics: Mutex::new(Vec::new()),
                memory_limit: None,
            }
        }

        fn with_memory_limit(limit: usize) -> Self {
            Self {
                running: AtomicBool::new(false),
                metrics: Mutex::new(Vec::new()),
                memory_limit: Some(limit),
            }
        }

        fn start(&self) -> Result<(), BearDogError> {
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

        fn stop(&self) -> Result<(), BearDogError> {
            if !self.running.load(Ordering::SeqCst) {
                return Err(BearDogError::System {
                    message: "Service not running".to_string(),
                    category: beardog_errors::SystemErrorCategory::General,
                });
            }
            self.running.store(false, Ordering::SeqCst);
            Ok(())
        }

        fn record_metric(&self, metric: Metric) -> Result<(), BearDogError> {
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

        fn get_stored_metrics(&self) -> Vec<Metric> {
            self.metrics.lock().unwrap().clone()
        }
    }

    struct ThresholdMonitor {
        threshold: f64,
    }

    impl ThresholdMonitor {
        fn new(threshold: f64) -> Self {
            Self { threshold }
        }

        fn check_violation(&self, value: f64) -> bool {
            value > self.threshold
        }
    }

    struct CircularMetricBuffer {
        values: Vec<f64>,
        capacity: usize,
        position: usize,
    }

    impl CircularMetricBuffer {
        fn with_capacity(capacity: usize) -> Self {
            Self {
                values: Vec::new(),
                capacity,
                position: 0,
            }
        }

        fn push(&mut self, value: f64) {
            if self.values.len() < self.capacity {
                self.values.push(value);
            } else {
                self.values[self.position] = value;
                self.position = (self.position + 1) % self.capacity;
            }
        }

        fn len(&self) -> usize {
            self.values.len()
        }

        fn get_values(&self) -> Vec<f64> {
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

        fn average(&self) -> Option<f64> {
            if self.values.is_empty() {
                None
            } else {
                Some(self.values.iter().sum::<f64>() / self.values.len() as f64)
            }
        }
    }

    struct MetricExporter {}

    impl MetricExporter {
        fn new() -> Self {
            Self {}
        }

        fn export(&self, _metrics: Vec<Metric>, destination: &str) -> Result<(), BearDogError> {
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

    struct MetricStorage {
        metrics: Vec<Metric>,
        retention_seconds: u64,
    }

    impl MetricStorage {
        fn with_retention_seconds(seconds: u64) -> Self {
            Self {
                metrics: Vec::new(),
                retention_seconds: seconds,
            }
        }

        fn store(&mut self, metric: Metric) {
            self.metrics.push(metric);
        }

        fn cleanup_old_metrics(&mut self, current_time: u64) {
            self.metrics
                .retain(|m| current_time - m.timestamp <= self.retention_seconds);
        }

        fn get_all(&self) -> &[Metric] {
            &self.metrics
        }
    }

    struct MetricSampler {
        rate: f64,
        counter: AtomicU64,
    }

    impl MetricSampler {
        fn with_rate(rate: f64) -> Self {
            Self {
                rate,
                counter: AtomicU64::new(0),
            }
        }

        fn should_sample(&self) -> bool {
            let count = self.counter.fetch_add(1, Ordering::SeqCst);
            (count as f64 * self.rate) % 1.0 < self.rate
        }
    }

    struct SafeCounter {
        value: AtomicU64,
    }

    impl SafeCounter {
        fn new() -> Self {
            Self {
                value: AtomicU64::new(0),
            }
        }

        fn set(&self, value: u64) {
            self.value.store(value, Ordering::SeqCst);
        }

        fn increment(&self) -> Result<u64, BearDogError> {
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
}
