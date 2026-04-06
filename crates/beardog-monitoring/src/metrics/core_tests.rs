// SPDX-License-Identifier: AGPL-3.0-or-later

// Core Metrics System Tests
//
// Comprehensive test coverage for core metrics functionality

#![allow(unused_imports, unused_variables, dead_code, clippy::all)]

use super::{MetricsCore, MetricsCoreConfig};

#[cfg(test)]
mod metrics_core_tests {
    use super::*;

    #[test]
    fn test_metrics_core_creation_success() {
        let config = MetricsCoreConfig::default();
        let core = MetricsCore::new(config);

        assert!(core.is_ok());
    }

    #[test]
    fn test_metrics_core_creation_with_custom_config() {
        let config = MetricsCoreConfig {
            enabled: true,
            collection_interval_seconds: 30,
        };

        let core = MetricsCore::new(config).unwrap();
        let retrieved_config = core.get_config();

        assert!(retrieved_config.enabled);
        assert_eq!(retrieved_config.collection_interval_seconds, 30);
    }

    #[test]
    fn test_metrics_core_start() {
        let config = MetricsCoreConfig::default();
        let core = MetricsCore::new(config).unwrap();

        let result = core.start();
        assert!(result.is_ok());
    }

    #[test]
    fn test_metrics_core_disabled_config() {
        let config = MetricsCoreConfig {
            enabled: false,
            collection_interval_seconds: 60,
        };

        let core = MetricsCore::new(config).unwrap();
        assert!(!core.get_config().enabled);
    }

    #[test]
    fn test_metrics_core_get_config() {
        let config = MetricsCoreConfig {
            enabled: true,
            collection_interval_seconds: 120,
        };

        let core = MetricsCore::new(config.clone()).unwrap();
        let retrieved = core.get_config();

        assert_eq!(retrieved.enabled, config.enabled);
        assert_eq!(
            retrieved.collection_interval_seconds,
            config.collection_interval_seconds
        );
    }

    #[test]
    fn test_metrics_core_default_config() {
        let config = MetricsCoreConfig::default();

        assert!(config.enabled);
        assert_eq!(config.collection_interval_seconds, 60);
    }

    #[test]
    fn test_metrics_core_various_intervals() {
        let intervals = vec![1, 30, 60, 300, 3600];

        for interval in intervals {
            let config = MetricsCoreConfig {
                enabled: true,
                collection_interval_seconds: interval,
            };

            let core = MetricsCore::new(config).unwrap();
            assert_eq!(core.get_config().collection_interval_seconds, interval);
        }
    }

    #[test]
    fn test_metrics_core_can_start_multiple_times() {
        let config = MetricsCoreConfig::default();
        let core = MetricsCore::new(config).unwrap();

        // Should be idempotent
        assert!(core.start().is_ok());
        assert!(core.start().is_ok());
        assert!(core.start().is_ok());
    }

    #[test]
    fn test_metrics_core_config_clone() {
        let config1 = MetricsCoreConfig {
            enabled: true,
            collection_interval_seconds: 90,
        };

        let config2 = config1.clone();

        assert_eq!(config1.enabled, config2.enabled);
        assert_eq!(
            config1.collection_interval_seconds,
            config2.collection_interval_seconds
        );
    }

    #[test]
    fn test_metrics_core_debug_impl() {
        let config = MetricsCoreConfig::default();
        let core = MetricsCore::new(config).unwrap();

        let debug_str = format!("{core:?}");
        assert!(debug_str.contains("MetricsCore"));
    }
}
