// SPDX-License-Identifier: AGPL-3.0-or-later

//! Split from coverage_gap_tests_11: HSM config defaults, keys, capabilities, monitoring core/health.

// ===========================================================================
// canonical/config/hsm/mod.rs - 159 uncov (12 Default impls)
// ===========================================================================
mod hsm_mod_defaults {
    use crate::canonical::config::hsm::*;

    #[test]
    fn test_unified_hsm_config_default() {
        let c = UnifiedHsmConfig::default();
        let _ = format!("{c:?}");
        // Test HsmConfigValidation trait
        let _ = c.validate();
        assert!(c.is_compatible_with(1));
    }

    #[test]
    fn test_hsm_retry_policy_default() {
        use crate::canonical::traits::RetryStrategy;
        let c = HsmRetryPolicy::default();
        let _ = format!("{c:?}");
        // Test RetryStrategy trait methods
        assert!(c.max_attempts() > 0);
        let delay = c.delay_for_attempt(0);
        assert!(delay.as_millis() > 0);
        assert!(c.backoff_multiplier() > 1.0);
        let total = c.total_delay(3);
        assert!(total.as_millis() > 0);
        assert!(!c.is_limit_reached(0));
        assert!(c.is_limit_reached(100));
    }

    #[test]
    fn test_hsm_retry_policy_should_retry_error() {
        use crate::canonical::traits::RetryStrategy;
        let c = HsmRetryPolicy::default();
        // Timeout errors should be retried
        let timeout_err = std::io::Error::new(std::io::ErrorKind::TimedOut, "connection timed out");
        assert!(c.should_retry_error(&timeout_err));
        // Connection errors should be retried
        let conn_err =
            std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "connection refused");
        assert!(c.should_retry_error(&conn_err));
        // Auth errors should not be retried
        let auth_err =
            std::io::Error::new(std::io::ErrorKind::PermissionDenied, "unauthorized access");
        assert!(!c.should_retry_error(&auth_err));
    }

    #[test]
    fn test_hsm_retry_policy_disabled() {
        use crate::canonical::traits::RetryStrategy;
        let mut c = HsmRetryPolicy::default();
        c.enabled = false;
        // When disabled, total_delay should be 0
        assert_eq!(c.total_delay(5).as_millis(), 0);
        // When disabled, is_limit_reached should be true
        assert!(c.is_limit_reached(0));
        // When disabled, should_retry_error should be false
        let err = std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout");
        assert!(!c.should_retry_error(&err));
    }

    #[test]
    fn test_hsm_tier_management_config_default() {
        let c = HsmTierManagementConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_compliance_config_default() {
        let c = HsmComplianceConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_monitoring_config_default() {
        let c = HsmMonitoringConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_alerting_config_default() {
        let c = HsmAlertingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_health_check_config_default() {
        let c = HsmHealthCheckConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_integration_config_default() {
        let c = HsmIntegrationConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_load_balancing_config_default() {
        let c = LoadBalancingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_failover_config_default() {
        let c = FailoverConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_session_management_config_default() {
        let c = SessionManagementConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_backup_config_default() {
        let c = HsmBackupConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/hsm/keys.rs - 136 uncov (7 Default impls)
// ===========================================================================
mod hsm_keys_defaults {
    use crate::canonical::hsm::keys::*;

    #[test]
    fn test_hsm_key_default() {
        let k = HsmKey::default();
        let _ = format!("{k:?}");
    }

    #[test]
    fn test_key_material_default() {
        let m = KeyMaterial::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_key_metadata_default() {
        let m = KeyMetadata::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_key_health_default() {
        let h = KeyHealth::default();
        let _ = format!("{h:?}");
    }

    #[test]
    fn test_encryption_info_default() {
        let e = EncryptionInfo::default();
        let _ = format!("{e:?}");
    }

    #[test]
    fn test_backup_info_default() {
        let b = BackupInfo::default();
        let _ = format!("{b:?}");
    }

    #[test]
    fn test_key_lifecycle_state_default() {
        let s = KeyLifecycleState::default();
        let _ = format!("{s:?}");
    }
}

// ===========================================================================
// canonical/hsm/capabilities.rs - 107 uncov (9 Default impls)
// ===========================================================================
mod hsm_capabilities_defaults {
    use crate::canonical::hsm::capabilities::*;

    #[test]
    fn test_hsm_capabilities_default() {
        let c = HsmCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_key_generation_capabilities_default() {
        let c = KeyGenerationCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_key_management_capabilities_default() {
        let c = KeyManagementCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_cryptographic_capabilities_default() {
        let c = CryptographicCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_security_capabilities_default() {
        let c = SecurityCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_performance_capabilities_default() {
        let c = PerformanceCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_advanced_feature_capabilities_default() {
        let c = AdvancedFeatureCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_performance_requirements_default() {
        let c = PerformanceRequirements::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_api_support_capabilities_default() {
        let c = ApiSupportCapabilities::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/monitoring/core.rs - 152 uncov (5 Default impls)
// ===========================================================================
mod monitoring_core_defaults {
    use crate::canonical::monitoring::core::*;

    #[test]
    fn test_core_monitoring_config_default() {
        let c = CoreMonitoringConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_storage_backend_default() {
        let b = StorageBackend::default();
        let _ = format!("{b:?}");
    }

    #[test]
    fn test_retention_policy_default() {
        let p = RetentionPolicy::default();
        let _ = format!("{p:?}");
    }

    #[test]
    fn test_batch_config_default() {
        let c = BatchConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_retry_policy_default() {
        let p = RetryPolicy::default();
        let _ = format!("{p:?}");
    }
}

// ===========================================================================
// canonical/monitoring/health.rs - 93 uncov (8 Default impls)
// ===========================================================================
mod monitoring_health_defaults {
    use crate::canonical::monitoring::health::*;

    #[test]
    fn test_unified_health_config_default() {
        let c = UnifiedHealthConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_http_health_check_config_default() {
        let c = HttpHealthCheckConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_tcp_health_check_config_default() {
        let c = TcpHealthCheckConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_database_health_check_config_default() {
        let c = DatabaseHealthCheckConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_service_health_check_config_default() {
        let c = ServiceHealthCheckConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_health_monitoring_config_default() {
        let c = HealthMonitoringConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_health_alerting_config_default() {
        let c = HealthAlertingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_health_recovery_config_default() {
        let c = HealthRecoveryConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/trait.rs - 111 uncov (validation functions)
// ===========================================================================
