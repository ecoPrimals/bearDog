// SPDX-License-Identifier: AGPL-3.0-or-later

//! Error-path tests for metrics collection, aggregation, storage, and the monitoring service.

use super::common::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[test]
fn test_metrics_collection_with_invalid_source() {
    let result = collect_metrics_from_invalid_source();
    assert!(result.is_err(), "Should fail with invalid metrics source");
}

#[test]
fn test_metrics_collection_with_timeout() {
    let result = collect_metrics_with_timeout(1);
    assert!(result.is_err(), "Should timeout with short duration");
}

#[test]
fn test_metrics_aggregation_empty_data() {
    let aggregator = MetricsAggregator::new();
    let result = aggregator.aggregate(vec![]);

    assert!(result.is_err(), "Should fail with empty data");
}

#[test]
fn test_metrics_aggregation_invalid_values() {
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
    let service = Arc::new(MonitoringServiceWrapper::new());
    let service_clone = Arc::clone(&service);

    let handle = std::thread::spawn(move || service_clone.start());

    let result1 = service.start();
    let result2 = handle.join().expect("Thread should complete");

    assert!(result1.is_ok() || result2.is_ok());
    assert!(result1.is_err() || result2.is_err());
}

#[test]
fn test_monitoring_service_stop_before_start() {
    let service = MonitoringServiceWrapper::new();
    let result = service.stop();

    assert!(result.is_err(), "Should fail to stop non-running service");
}

#[test]
fn test_threshold_violation_detection() {
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
    let monitor = ThresholdMonitor::new(0.0);

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
    let mut buffer = CircularMetricBuffer::with_capacity(3);

    buffer.push(1.0);
    buffer.push(2.0);
    buffer.push(3.0);
    buffer.push(4.0);

    assert_eq!(buffer.len(), 3);
    assert_eq!(buffer.get_values(), vec![2.0, 3.0, 4.0]);
}

#[test]
fn test_circular_metric_buffer_empty() {
    let buffer = CircularMetricBuffer::with_capacity(10);

    assert_eq!(buffer.len(), 0);
    assert!(buffer.get_values().is_empty());
    assert!(buffer.average().is_none());
}

#[test]
fn test_metric_export_to_invalid_destination() {
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
    let exporter = MetricExporter::new();
    let metrics = vec![Metric {
        value: 1.0,
        timestamp: 0,
    }];

    let result = exporter.export(metrics, "http://unreachable:9999");
    assert!(result.is_err(), "Should fail when connection fails");
}

#[test]
fn test_metrics_retention_policy() {
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
fn test_metric_sampling_rate() {
    let sampler = MetricSampler::with_rate(0.5);

    let mut sampled = 0;
    for _ in 0..1000 {
        if sampler.should_sample() {
            sampled += 1;
        }
    }

    assert!(
        sampled > 400 && sampled < 600,
        "Sampling rate off: {sampled}"
    );
}

#[test]
fn test_monitoring_service_restart() {
    let service = MonitoringServiceWrapper::new();

    service.start().expect("First start should succeed");
    service.stop().expect("Stop should succeed");

    let result = service.start();
    assert!(result.is_ok(), "Should allow restart after stop");
}

#[test]
fn test_metric_overflow_protection() {
    let counter = SafeCounter::new();

    counter.set(u64::MAX);
    let result = counter.increment();

    assert!(result.is_err(), "Should detect overflow at MAX");
}

#[test]
fn test_metrics_aggregation_percentiles() {
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
#[expect(
    clippy::cast_precision_loss,
    reason = "metric rollup averages usize sample counts as f64"
)]
fn test_monitoring_memory_limit() {
    let service = MonitoringServiceWrapper::with_memory_limit(1024);

    for i in 0..10000 {
        let result = service.record_metric(Metric {
            value: i as f64,
            timestamp: i,
        });

        if result.is_err() {
            assert!(i > 10, "Memory limit should allow some metrics");
            return;
        }
    }

    panic!("Should have hit memory limit");
}
