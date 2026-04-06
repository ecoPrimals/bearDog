// SPDX-License-Identifier: AGPL-3.0-or-later

// Performance Metrics Engine Tests
//
// Comprehensive test coverage for performance monitoring functionality

#![allow(unused_imports, unused_variables, dead_code, clippy::all)]

use super::{PerformanceConfig, PerformanceEngine, PerformanceMetrics};
use crate::metrics::{MetricCategory, MetricEvent, MetricValue};

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_performance_engine_creation() {
        let config = PerformanceConfig::default();
        let engine = PerformanceEngine::new(config);

        assert!(engine.is_ok());
    }

    #[test]
    fn test_performance_engine_start() {
        let config = PerformanceConfig::default();
        let engine = PerformanceEngine::new(config).unwrap();

        assert!(engine.start().is_ok());
    }

    #[test]
    fn test_performance_engine_get_metrics_aggregates_recorded_events() {
        let config = PerformanceConfig::default();
        let engine = PerformanceEngine::new(config).unwrap();

        let empty = engine.get_metrics().unwrap();
        assert_eq!(empty.cpu_usage, 0.0);
        assert_eq!(empty.error_rate, 0.0);

        engine
            .record_event(&MetricEvent {
                category: MetricCategory::Performance,
                name: "cpu_usage_pct".to_string(),
                value: MetricValue::Gauge(42.0),
                labels: std::collections::HashMap::new(),
                timestamp: std::time::SystemTime::now(),
            })
            .unwrap();
        engine
            .record_event(&MetricEvent {
                category: MetricCategory::Performance,
                name: "request_latency_ms".to_string(),
                value: MetricValue::Gauge(10.0),
                labels: std::collections::HashMap::new(),
                timestamp: std::time::SystemTime::now(),
            })
            .unwrap();
        engine
            .record_event(&MetricEvent {
                category: MetricCategory::Performance,
                name: "api_errors".to_string(),
                value: MetricValue::Counter(1),
                labels: std::collections::HashMap::new(),
                timestamp: std::time::SystemTime::now(),
            })
            .unwrap();

        let metrics = engine.get_metrics().unwrap();
        assert_eq!(metrics.cpu_usage, 42.0);
        assert_eq!(metrics.request_latency_ms, 10.0);
        assert!((metrics.error_rate - 1.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_performance_config_default() {
        let config = PerformanceConfig::default();

        assert_eq!(config.sample_rate, 1.0);
        assert_eq!(config.retention_hours, 24);
    }

    #[test]
    fn test_performance_config_custom() {
        let config = PerformanceConfig {
            sample_rate: 0.5,
            retention_hours: 48,
        };

        assert_eq!(config.sample_rate, 0.5);
        assert_eq!(config.retention_hours, 48);
    }

    #[test]
    fn test_performance_metrics_serialization() {
        let metrics = PerformanceMetrics {
            cpu_usage: 0.7,
            memory_usage: 0.4,
            request_latency_ms: 30.0,
            throughput_rps: 2000.0,
            error_rate: 0.02,
        };

        let json = serde_json::to_string(&metrics);
        assert!(json.is_ok());

        let deserialized: Result<PerformanceMetrics, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_performance_engine_record_event() {
        let config = PerformanceConfig::default();
        let engine = PerformanceEngine::new(config).unwrap();

        let event = MetricEvent {
            category: MetricCategory::Performance,
            name: "test_event".to_string(),
            value: MetricValue::Counter(100),
            labels: std::collections::HashMap::new(),
            timestamp: std::time::SystemTime::now(),
        };

        let result = engine.record_event(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_performance_config_clone() {
        let config1 = PerformanceConfig {
            sample_rate: 0.8,
            retention_hours: 72,
        };

        let config2 = config1.clone();

        assert_eq!(config1.sample_rate, config2.sample_rate);
        assert_eq!(config1.retention_hours, config2.retention_hours);
    }

    #[test]
    fn test_performance_engine_debug_impl() {
        let config = PerformanceConfig::default();
        let engine = PerformanceEngine::new(config).unwrap();

        let debug_str = format!("{engine:?}");
        assert!(debug_str.contains("PerformanceEngine"));
    }

    #[test]
    fn test_performance_various_sample_rates() {
        let sample_rates = vec![0.1, 0.5, 0.75, 1.0];

        for rate in sample_rates {
            let config = PerformanceConfig {
                sample_rate: rate,
                retention_hours: 24,
            };

            let engine = PerformanceEngine::new(config).unwrap();
            assert!(engine.start().is_ok());
        }
    }

    #[test]
    fn test_performance_engine_idempotent_start() {
        let config = PerformanceConfig::default();
        let engine = PerformanceEngine::new(config).unwrap();

        assert!(engine.start().is_ok());
        assert!(engine.start().is_ok());
        assert!(engine.start().is_ok());
    }
}
