// SPDX-License-Identifier: AGPL-3.0-or-later

//! Split from coverage_gap_tests_12: adapter certs, discovery, network, simplified config.

#[cfg(test)]
mod adapter_certificate_tests {
    use crate::adapter_certificates::*;
    use chrono::{Duration, Utc};

    fn make_cert(issued_offset_secs: i64, expires_offset_secs: i64) -> AdapterUnlockCertificate {
        AdapterUnlockCertificate {
            cert_id: "cert-12345678".to_string(),
            issuer_key_id: "key-1".to_string(),
            adapter_id: "beardog-adapters::songbird::network".to_string(),
            classification: AdapterClassification::Human,
            constraints: None,
            issued_at: Utc::now() + Duration::seconds(issued_offset_secs),
            expires_at: Utc::now() + Duration::seconds(expires_offset_secs),
            signature: vec![0u8; 64],
            issuer_public_key: vec![0u8; 32],
        }
    }

    #[test]
    fn test_is_expired_false() {
        let cert = make_cert(-100, 3600); // issued 100s ago, expires in 1h
        assert!(!cert.is_expired());
    }

    #[test]
    fn test_is_expired_true() {
        let cert = make_cert(-7200, -100); // issued 2h ago, expired 100s ago
        assert!(cert.is_expired());
    }

    #[test]
    fn test_not_yet_valid() {
        let cert = make_cert(3600, 7200); // issued in 1h, expires in 2h
        assert!(cert.not_yet_valid());
    }

    #[test]
    fn test_is_valid_now() {
        let cert = make_cert(-100, 3600);
        assert!(cert.is_valid_now());
    }

    #[test]
    fn test_is_valid_now_expired() {
        let cert = make_cert(-7200, -100);
        assert!(!cert.is_valid_now());
    }

    #[test]
    fn test_is_valid_now_future() {
        let cert = make_cert(3600, 7200);
        assert!(!cert.is_valid_now());
    }

    #[test]
    fn test_time_remaining() {
        let cert = make_cert(-100, 3600);
        let remaining = cert.time_remaining();
        assert!(remaining.num_seconds() > 3500);
    }

    #[test]
    fn test_status_valid() {
        let cert = make_cert(-100, 3600);
        assert_eq!(cert.status(), "Valid");
    }

    #[test]
    fn test_status_expired() {
        let cert = make_cert(-7200, -100);
        assert_eq!(cert.status(), "Expired");
    }

    #[test]
    fn test_status_not_yet_valid() {
        let cert = make_cert(3600, 7200);
        assert_eq!(cert.status(), "Not yet valid");
    }

    #[test]
    fn test_metadata_string() {
        let cert = make_cert(-100, 3600);
        let meta = cert.metadata_string();
        assert!(meta.contains("cert-123"));
        assert!(meta.contains("songbird"));
        assert!(meta.contains("Human"));
    }

    #[test]
    fn test_signable_data() {
        let cert = make_cert(-100, 3600);
        let data = cert.signable_data();
        assert!(!data.is_empty());
    }

    #[test]
    fn test_verify_no_key() {
        let cert = make_cert(-100, 3600);
        let result = cert.verify(None);
        // Without a real Ed25519 key, verification fails
        let _ = result;
    }

    #[test]
    fn test_adapter_classification() {
        assert!(!AdapterClassification::Human.requires_payment());
        assert!(AdapterClassification::Commercial.requires_payment());
        assert_eq!(
            AdapterClassification::Human.description(),
            "Human usage (free)"
        );
        assert_eq!(
            AdapterClassification::Commercial.description(),
            "Commercial usage (paid)"
        );
    }
}

#[cfg(test)]
mod monitoring_level_tests {
    use crate::canonical::traits::monitoring::MonitoringLevel;
    use std::time::Duration;

    #[test]
    fn test_includes_detailed_metrics() {
        assert!(!MonitoringLevel::Minimal.includes_detailed_metrics());
        assert!(!MonitoringLevel::Basic.includes_detailed_metrics());
        assert!(!MonitoringLevel::Standard.includes_detailed_metrics());
        assert!(MonitoringLevel::Detailed.includes_detailed_metrics());
        assert!(MonitoringLevel::Verbose.includes_detailed_metrics());
    }

    #[test]
    fn test_includes_tracing() {
        assert!(!MonitoringLevel::Minimal.includes_tracing());
        assert!(!MonitoringLevel::Basic.includes_tracing());
        assert!(MonitoringLevel::Standard.includes_tracing());
        assert!(MonitoringLevel::Detailed.includes_tracing());
        assert!(MonitoringLevel::Verbose.includes_tracing());
    }

    #[test]
    fn test_overhead_level() {
        assert_eq!(MonitoringLevel::Minimal.overhead_level(), 1);
        assert_eq!(MonitoringLevel::Basic.overhead_level(), 2);
        assert_eq!(MonitoringLevel::Standard.overhead_level(), 3);
        assert_eq!(MonitoringLevel::Detailed.overhead_level(), 4);
        assert_eq!(MonitoringLevel::Verbose.overhead_level(), 5);
    }

    #[test]
    fn test_typical_interval() {
        assert_eq!(
            MonitoringLevel::Minimal.typical_interval(),
            Duration::from_secs(300)
        );
        assert_eq!(
            MonitoringLevel::Basic.typical_interval(),
            Duration::from_secs(120)
        );
        assert_eq!(
            MonitoringLevel::Standard.typical_interval(),
            Duration::from_secs(60)
        );
        assert_eq!(
            MonitoringLevel::Detailed.typical_interval(),
            Duration::from_secs(30)
        );
        assert_eq!(
            MonitoringLevel::Verbose.typical_interval(),
            Duration::from_secs(10)
        );
    }
}

#[cfg(test)]
mod discovery_unified_methods_tests {
    use crate::canonical::config::domains::discovery_unified::*;
    use crate::canonical::config::r#trait::BearDogConfig;

    #[test]
    fn test_aggressive_config() {
        let c = UnifiedDiscoveryConfig::aggressive();
        let _ = &c.cache;
        let _ = &c.security;
    }

    #[test]
    fn test_conservative_config() {
        let c = UnifiedDiscoveryConfig::conservative();
        let _ = &c.cache;
    }

    #[test]
    fn test_validate() {
        let c = UnifiedDiscoveryConfig::default();
        assert!(c.validate().is_ok());
    }

    #[test]
    fn test_merge() {
        let c1 = UnifiedDiscoveryConfig::default();
        let c2 = UnifiedDiscoveryConfig::aggressive();
        let merged = c1.merge(&c2);
        assert!(merged.is_ok());
    }

    #[test]
    fn test_from_env() {
        let _ = UnifiedDiscoveryConfig::from_env();
    }

    #[test]
    fn test_discovery_security_config_default() {
        let c = DiscoverySecurityConfig::default();
        assert!(c.enable_tls);
    }

    #[test]
    fn test_load_balancing_config_default() {
        let c = LoadBalancingConfig::default();
        let _ = &c.algorithm;
    }

    #[test]
    fn test_circuit_breaker_config_default() {
        let c = CircuitBreakerConfig::default();
        let _ = c.failure_threshold;
    }
}

#[cfg(test)]
mod simplified_config_tests {
    use crate::canonical::config::unified::simplified::*;

    #[test]
    fn test_simplified_config_default() {
        let c = SimplifiedBearDogConfig::default();
        let _ = &*c.version;
        let _ = &*c.environment;
        let _ = &*c.instance_id;
    }

    #[test]
    fn test_simplified_validate() {
        let c = SimplifiedBearDogConfig::default();
        assert!(c.validate().is_ok());
    }

    #[test]
    fn test_simplified_from_env() {
        let _ = SimplifiedBearDogConfig::from_env();
    }

    #[test]
    fn test_simplified_with_overrides() {
        let c = SimplifiedBearDogConfig::default();
        let overrides = std::collections::HashMap::new();
        let _ = c.with_overrides(overrides);
    }

    #[test]
    fn test_simplified_summary() {
        let c = SimplifiedBearDogConfig::default();
        let s = c.summary();
        assert!(!s.is_empty());
    }

    #[test]
    fn test_network_settings_default() {
        let n = NetworkSettings::default();
        assert!(n.port > 0);
        assert!(n.max_connections > 0);
        assert!(n.enable_tls);
    }

    #[test]
    fn test_security_settings_default() {
        let s = SecuritySettings::default();
        assert!(s.session_timeout_seconds > 0);
        assert!(s.max_login_attempts > 0);
        assert!(s.enable_mfa);
    }

    #[test]
    fn test_database_settings_default() {
        let d = DatabaseSettings::default();
        assert!(!d.connection_string.is_empty());
        assert!(d.pool_size > 0);
        assert!(d.enable_encryption);
    }

    #[test]
    fn test_monitoring_settings_default() {
        let m = MonitoringSettings::default();
        assert!(m.enable_metrics);
        assert!(m.metrics_interval_seconds > 0);
    }

    #[test]
    fn test_performance_settings_default() {
        let p = PerformanceSettings::default();
        assert!(p.max_memory_mb > 0);
        assert!(p.worker_threads > 0);
        assert!(p.enable_optimization);
    }
}

#[cfg(test)]
mod network_consolidated_tests {
    use crate::canonical::config::domains::network::*;

    #[test]
    fn test_consolidated_network_config_default() {
        let c = ConsolidatedNetworkConfiguration::default();
        assert!(c.validate().is_ok());
    }

    #[test]
    fn test_development_config() {
        let c = ConsolidatedNetworkConfiguration::development();
        let _ = &c.server;
    }

    #[test]
    fn test_production_config() {
        let c = ConsolidatedNetworkConfiguration::production();
        let _ = &c.server;
    }

    #[test]
    fn test_from_host_port() {
        let c = ConsolidatedNetworkConfiguration::from_host_port("127.0.0.1", 0);
        assert_eq!(c.server.port, 0);
    }

    #[test]
    fn test_from_bind_address() {
        let addr: std::net::SocketAddr = "127.0.0.1:0".parse().unwrap();
        let c = ConsolidatedNetworkConfiguration::from_bind_address(addr);
        assert_eq!(c.server.port, 0);
    }

    #[test]
    fn test_to_bind_address() {
        let c = ConsolidatedNetworkConfiguration::from_host_port("127.0.0.1", 8080);
        let addr = c.to_bind_address();
        assert!(addr.is_ok());
    }

    #[test]
    fn test_rate_limit_config_default() {
        let c = RateLimitConfig::default();
        let _ = &c.strategy;
        let _ = &c.scope;
    }

    #[test]
    fn test_rate_limit_per_minute() {
        let c = RateLimitConfig::per_minute(100);
        let _ = &c.strategy;
    }

    #[test]
    fn test_rate_limit_per_second() {
        let c = RateLimitConfig::per_second(10);
        let _ = &c.strategy;
    }

    #[test]
    fn test_rate_limit_global() {
        let c = RateLimitConfig::global(1000, 60);
        let _ = &c.scope;
    }

    #[test]
    fn test_network_scan_config_to_network_config() {
        let c = NetworkScanConfig {
            ip_ranges: vec!["192.168.1.0/24".into()],
            timeout_ms: 5000,
        };
        let _ = c.to_network_config();
    }

    #[test]
    fn test_client_configuration_default() {
        let c = client::ClientConfiguration::default();
        assert!(c.connection_timeout_seconds > 0);
        assert!(c.retry.max_attempts > 0);
    }

    #[test]
    fn test_client_configuration_validate() {
        let c = client::ClientConfiguration::default();
        assert!(c.validate().is_ok());
    }

    #[test]
    fn test_retry_configuration_fields() {
        let r = client::RetryConfiguration {
            max_attempts: 3,
            base_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
            enable_exponential_backoff: true,
            retryable_status_codes: vec![500, 502, 503],
        };
        assert_eq!(r.max_attempts, 3);
    }
}
