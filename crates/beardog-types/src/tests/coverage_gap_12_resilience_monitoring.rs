// SPDX-License-Identifier: AGPL-3.0-or-later

// Split from coverage_gap_tests_12: resilience + monitoring method coverage.

#[cfg(test)]
mod resilience_methods_tests {
    use crate::canonical::config::r#trait::BearDogConfig;
    use crate::canonical::providers_unified::resilience::*;
    use std::time::Duration;

    #[test]
    fn test_resilience_config_default() {
        let c = ResilienceConfig::default();
        assert!(c.retry.enabled);
        assert!(c.circuit_breaker.enabled);
        assert!(c.timeout.enabled);
        assert!(c.bulkhead.enabled);
        assert!(c.fallback.enabled);
    }

    #[test]
    fn test_retry_config_fields() {
        let c = RetryConfig::default();
        assert!(c.max_attempts > 0);
        assert!(c.enabled);
        let _ = &c.initial_delay;
        let _ = &c.max_delay;
        let _ = &c.backoff_strategy;
        assert!(c.jitter_enabled);
    }

    #[test]
    fn test_retry_strategy_trait() {
        use crate::canonical::traits::RetryStrategy;
        let c = RetryConfig::default();
        assert_eq!(c.max_attempts(), 3);
        let d = c.delay_for_attempt(1);
        assert!(d > Duration::from_millis(0));
        let _ = c.backoff_multiplier();
    }

    #[test]
    fn test_circuit_breaker_config_fields() {
        let c = CircuitBreakerConfig::default();
        assert!(c.enabled);
        assert!(c.failure_threshold > 0);
        let _ = &c.timeout;
        assert!(c.half_open_max_calls > 0);
    }

    #[test]
    fn test_timeout_config_validate() {
        use crate::canonical::traits::timeout::TimeoutPolicy;
        let c = TimeoutConfig::default();
        assert!(TimeoutPolicy::validate(&c).is_ok());
        let _ = c.connection_timeout();
        let _ = c.operation_timeout("read");
        assert!(!c.should_timeout(Duration::from_millis(1), "read"));
        let _ = c.global_timeout();
        let _ = c.read_timeout();
        let _ = c.write_timeout();
        let _ = c.idle_timeout();
        let _ = c.remaining_time(Duration::from_secs(1), "read");
        assert!(c.is_production_ready());
    }

    #[test]
    fn test_bulkhead_config_fields() {
        let c = BulkheadConfig::default();
        assert!(c.enabled);
        assert!(c.max_concurrent_calls > 0);
        let _ = &c.max_wait_duration;
    }

    #[test]
    fn test_fallback_config_fields() {
        let c = FallbackConfig::default();
        assert!(c.enabled);
        let _ = &c.strategy;
    }

    #[test]
    fn test_backoff_strategy_variants() {
        let _ = BackoffStrategy::Fixed;
        let _ = BackoffStrategy::Exponential;
        let _ = BackoffStrategy::Linear;
        let _ = BackoffStrategy::Custom("custom".into());
    }

    #[test]
    fn test_isolation_strategy_variants() {
        let _ = IsolationStrategy::Semaphore;
        let _ = IsolationStrategy::ThreadPool;
    }

    #[test]
    fn test_fallback_strategy_variants() {
        let _ = FallbackStrategy::DefaultValue;
        let _ = FallbackStrategy::Function;
        let _ = FallbackStrategy::Cache;
        let _ = FallbackStrategy::Alternative;
    }
}

#[cfg(test)]
mod monitoring_core_methods_tests {
    use crate::canonical::monitoring::MonitoringConfigValidation;
    use crate::canonical::monitoring::core::*;
    use std::collections::HashMap;
    use std::time::Duration;

    #[test]
    fn test_core_monitoring_config_default_and_validate() {
        let c = CoreMonitoringConfig::default();
        assert!(MonitoringConfigValidation::validate(&c).is_ok());
    }

    #[test]
    fn test_storage_backend_variants() {
        let _ = StorageBackend::Memory { max_entries: 10000 };
        let _ = StorageBackend::File {
            path: "/tmp/test".to_string(),
            rotation_size: 1024 * 1024,
            max_files: 5,
        };
        let _ = StorageBackend::Database {
            connection_string: "sqlite://test.db".to_string(),
            table_name: "metrics".to_string(),
        };
        let _ = StorageBackend::Remote {
            endpoint: "http://localhost:9090".to_string(),
            api_key: None,
            timeout: Duration::from_secs(30),
        };
        let _ = StorageBackend::Custom {
            backend_type: "custom".to_string(),
            config: HashMap::new(),
        };
    }

    #[test]
    fn test_retention_policy_fields() {
        let p = RetentionPolicy::default();
        let _ = &p.max_age;
        let _ = p.max_size;
        let _ = p.max_count;
        let _ = p.cleanup_interval;
    }

    #[test]
    fn test_batch_config_fields() {
        let b = BatchConfig::default();
        let _ = b.batch_size;
        let _ = &b.flush_interval;
    }

    #[test]
    fn test_retry_policy_fields() {
        let r = RetryPolicy::default();
        let _ = r.max_retries;
        let _ = &r.initial_delay;
    }

    #[test]
    fn test_filter_action_variants() {
        let _ = FilterAction::Include;
        let _ = FilterAction::Exclude;
        let _ = FilterAction::Transform {
            operation: "add_prefix".to_string(),
            parameters: HashMap::new(),
        };
    }

    #[test]
    fn test_comparison_operator_variants() {
        let _ = ComparisonOperator::Equals;
        let _ = ComparisonOperator::NotEquals;
        let _ = ComparisonOperator::GreaterThan;
        let _ = ComparisonOperator::LessThan;
        let _ = ComparisonOperator::GreaterThanOrEqual;
        let _ = ComparisonOperator::LessThanOrEqual;
        let _ = ComparisonOperator::Contains;
    }

    #[test]
    fn test_metric_filter_fields() {
        let f = MetricFilter {
            name: "cpu_filter".to_string(),
            pattern: "cpu.*".to_string(),
            action: FilterAction::Include,
            conditions: vec![],
        };
        let _ = &f.name;
        let _ = &f.pattern;
        let _ = &f.action;
    }

    #[test]
    fn test_compression_type_variants() {
        let _ = CompressionType::None;
        let _ = CompressionType::Gzip;
        let _ = CompressionType::Lz4;
        let _ = CompressionType::Zstd;
    }
}

#[cfg(test)]
mod monitoring_health_methods_tests {
    use crate::canonical::monitoring::MonitoringConfigValidation;
    use crate::canonical::monitoring::health::*;
    use std::time::Duration;

    #[test]
    fn test_unified_health_config_default_and_validate() {
        let c = UnifiedHealthConfig::default();
        assert!(MonitoringConfigValidation::validate(&c).is_ok());
    }

    #[test]
    fn test_http_health_check_config() {
        let c = HttpHealthCheckConfig::default();
        assert!(c.enabled);
        let _ = &c.default_timeout;
        assert!(c.verify_ssl);
    }

    #[test]
    fn test_tcp_health_check_config() {
        let c = TcpHealthCheckConfig::default();
        let _ = &c.default_timeout;
    }

    #[test]
    fn test_database_health_check_config() {
        let c = DatabaseHealthCheckConfig::default();
        let _ = &c.default_timeout;
        assert!(c.enabled);
    }

    #[test]
    fn test_service_health_check_config() {
        let c = ServiceHealthCheckConfig::default();
        assert!(c.enabled);
        let _ = c.discovery_enabled;
    }

    #[test]
    fn test_health_monitoring_config() {
        let c = HealthMonitoringConfig::default();
        assert!(c.enabled);
        let _ = &c.history_retention;
    }

    #[test]
    fn test_health_alerting_config() {
        let c = HealthAlertingConfig::default();
        assert!(c.enabled);
        assert!(c.alert_on_failure);
    }

    #[test]
    fn test_health_recovery_config() {
        let c = HealthRecoveryConfig::default();
        let _ = c.enabled;
        let _ = &c.recovery_delay;
        let _ = c.auto_recovery_attempts;
    }

    #[test]
    fn test_http_method_variants() {
        let _ = HttpMethod::GET;
        let _ = HttpMethod::POST;
        let _ = HttpMethod::HEAD;
    }

    #[test]
    fn test_database_type_variants() {
        let _ = DatabaseType::PostgreSQL;
        let _ = DatabaseType::MySQL;
        let _ = DatabaseType::SQLite;
    }

    #[test]
    fn test_escalation_rule_fields() {
        let r = EscalationRule {
            name: "critical_alert".to_string(),
            condition: EscalationCondition::FailureCount(3),
            delay: Duration::from_secs(60),
            channels: vec!["email".to_string()],
        };
        let _ = &r.name;
        let _ = &r.condition;
        let _ = &r.delay;
        let _ = &r.channels;
    }
}
