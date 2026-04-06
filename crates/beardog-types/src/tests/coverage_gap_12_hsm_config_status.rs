// SPDX-License-Identifier: AGPL-3.0-or-later

//! Split from `coverage_gap_tests_12`: HSM capabilities, config, status.

#[cfg(test)]
mod hsm_capabilities_methods_tests {
    use crate::canonical::hsm::capabilities::*;

    #[test]
    fn test_hsm_capabilities_new() {
        let c = HsmCapabilities::new();
        assert!(!c.key_generation.supported_algorithms.is_empty());
    }

    #[test]
    fn test_supports_key_algorithm() {
        let c = HsmCapabilities::new();
        // Default has empty algorithms — exercises the method path
        assert!(!c.supports_key_algorithm("AES-256"));
        assert!(!c.supports_key_algorithm("RSA-2048"));
    }

    #[test]
    fn test_supports_key_size() {
        let c = HsmCapabilities::default();
        assert!(c.supports_key_size(256));
        assert!(c.supports_key_size(2048));
    }

    #[test]
    fn test_supports_encryption() {
        let c = HsmCapabilities::default();
        assert!(c.supports_encryption("AES-GCM"));
    }

    #[test]
    fn test_supports_signature() {
        let c = HsmCapabilities::default();
        assert!(c.supports_signature("RSA-PSS"));
    }

    #[test]
    fn test_max_key_capacity() {
        let c = HsmCapabilities::default();
        let _ = c.max_key_capacity();
    }

    #[test]
    fn test_meets_security_requirements() {
        let c = HsmCapabilities::default();
        assert!(c.meets_security_requirements("Level 1"));
        assert!(c.meets_security_requirements("Level 3"));
        assert!(!c.meets_security_requirements("Level 4"));
    }

    #[test]
    fn test_performance_rating() {
        let c = HsmCapabilities::default();
        let rating = c.performance_rating();
        assert!((0.0..=1.0).contains(&rating));
    }

    #[test]
    fn test_has_advanced_features() {
        let c = HsmCapabilities::default();
        let _ = c.has_advanced_features();
    }

    #[test]
    fn test_capability_summary() {
        let c = HsmCapabilities::default();
        let s = c.capability_summary();
        assert!(s.total_algorithms > 0);
        let _ = &s.security_level;
    }

    #[test]
    fn test_meets_requirements() {
        let c = HsmCapabilities::new();
        // Empty requirements should pass
        let req = CapabilityRequirements {
            required_algorithms: vec![],
            required_key_sizes: vec![],
            min_security_level: "Level 1".to_string(),
            min_performance: PerformanceRequirements::default(),
            required_certifications: vec![],
            required_advanced_features: vec![],
        };
        let _ = c.meets_requirements(&req);
    }

    #[test]
    fn test_security_capabilities_default() {
        let s = SecurityCapabilities::default();
        assert!(s.access_control);
        assert!(s.audit_logging);
        assert!(s.tamper_detection);
        let _ = &s.physical_security_level;
    }

    #[test]
    fn test_performance_capabilities_default() {
        let p = PerformanceCapabilities::default();
        assert!(p.max_operations_per_second > 0);
        assert!(p.average_latency_ms > 0.0);
    }

    #[test]
    fn test_advanced_feature_capabilities_default() {
        let a = AdvancedFeatureCapabilities::default();
        let _ = a.secure_enclaves;
        let _ = a.post_quantum_crypto;
    }

    #[test]
    fn test_api_support_capabilities_default() {
        let a = ApiSupportCapabilities::default();
        assert!(a.rest_api_support);
        assert!(a.grpc_support);
        assert!(a.rate_limiting);
    }
}

#[cfg(test)]
mod hsm_config_methods_tests {
    use crate::canonical::hsm::config::*;
    use std::time::Duration;

    #[test]
    fn test_hsm_config_default() {
        let c = HsmConfig::default();
        let _ = &c.provider;
        let _ = &c.connection;
        let _ = &c.security;
        let _ = &c.auth_method;
        let _ = &c.operation_timeout;
    }

    #[test]
    fn test_connection_config_default() {
        let c = ConnectionConfig::default();
        assert!(c.timeout_ms > 0);
        assert!(c.max_retries > 0);
    }

    #[test]
    fn test_security_config_default() {
        let c = SecurityConfig::default();
        assert!(c.strict_mode);
        assert!(c.audit_logging);
    }

    #[test]
    fn test_auth_method_variants() {
        let _ = AuthMethod::None;
        let _ = AuthMethod::Password {
            password: "secret".to_string(),
        };
        let _ = AuthMethod::Certificate {
            cert_path: "/path/cert".to_string(),
            key_path: "/path/key".to_string(),
        };
        let _ = AuthMethod::Token {
            token_id: 1,
            pin: None,
        };
        let _ = AuthMethod::Biometric {
            method: "fingerprint".to_string(),
        };
        assert!(matches!(AuthMethod::default(), AuthMethod::None));
    }

    #[test]
    fn test_software_hsm_config_default() {
        let c = SoftwareHsmConfig::default();
        assert!(c.memory_protection);
        let _ = &c.storage_path;
    }

    #[test]
    fn test_network_hsm_config_default() {
        let c = NetworkHsmConfig::default();
        assert!(c.use_tls);
        let _ = &c.server_address;
        let _ = c.port;
    }

    #[test]
    fn test_hsm_provider_config_variants() {
        let _ = HsmProviderConfig::default(); // Software
        let _ = HsmProviderConfig::Software(SoftwareHsmConfig::default());
        let _ = HsmProviderConfig::Network(NetworkHsmConfig::default());
    }

    #[test]
    fn test_hsm_provider_config_base_config() {
        let p = HsmProviderConfig::default();
        let _ = p.base_config();
    }

    #[test]
    fn test_universal_hsm_provider() {
        let _ = UniversalHsmProvider::Discovered {
            provider_id: "test".into(),
            capability_type: "software".into(),
            endpoint: "localhost:9000".into(),
        };
    }
}

#[cfg(test)]
mod hsm_status_methods_tests {
    use crate::canonical::hsm::status::*;
    use std::collections::HashMap;
    use std::time::SystemTime;

    #[test]
    fn test_hsm_status_new() {
        let s = HsmStatus::new();
        assert!(s.is_healthy());
    }

    #[test]
    fn test_hsm_status_is_healthy() {
        let s = HsmStatus::new();
        assert!(s.is_healthy());
    }

    #[test]
    fn test_hsm_status_update_health() {
        let mut s = HsmStatus::new();
        s.update_health(HsmHealthStatus::Unhealthy);
        assert!(!s.is_healthy());
    }

    #[test]
    fn test_hsm_status_record_error() {
        let mut s = HsmStatus::new();
        s.record_error("test error");
        assert!(s.errors.total_errors > 0);
    }

    #[test]
    fn test_hsm_status_update_performance() {
        let mut s = HsmStatus::new();
        let metrics = PerformanceMetrics {
            operations_per_second: 5.0,
            average_latency_ms: 2.0,
            success_rate: 99.0,
            ..PerformanceMetrics::default()
        };
        s.update_performance(metrics);
    }

    #[test]
    fn test_hsm_status_summary() {
        let s = HsmStatus::new();
        let summary = s.summary();
        assert!(!summary.is_empty());
    }

    #[test]
    fn test_error_info_default() {
        let e = ErrorInfo::default();
        assert_eq!(e.total_errors, 0);
        assert_eq!(e.error_rate_per_hour, 0.0);
    }

    #[test]
    fn test_hsm_operation_result_success() {
        let r = HsmOperationResult::<String>::success("done".into());
        assert!(r.success);
        assert!(r.data.is_some());
    }

    #[test]
    fn test_hsm_operation_result_failure() {
        let r = HsmOperationResult::<String>::failure("fail");
        assert!(!r.success);
        assert!(r.error.is_some());
    }

    #[test]
    fn test_hsm_operation_result_with_processing_time() {
        let r = HsmOperationResult::<String>::success("ok".into()).with_processing_time(42);
        assert_eq!(r.processing_time_ms, 42);
    }

    #[test]
    fn test_hsm_operation_result_with_metadata() {
        let r = HsmOperationResult::<String>::success("ok".into()).with_metadata("key", "value");
        assert!(r.metadata.contains_key("key"));
    }

    #[test]
    fn test_hsm_health_check_config_default() {
        let c = HsmHealthCheckConfig::default();
        assert!(c.interval_seconds > 0);
        assert!(!c.enabled_checks.is_empty());
    }

    #[test]
    fn test_health_thresholds_default() {
        let t = HealthThresholds::default();
        assert!(t.max_latency_ms > 0.0);
        assert!(t.min_success_rate > 0.0);
    }
}
