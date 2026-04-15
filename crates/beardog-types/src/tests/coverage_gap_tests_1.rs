// SPDX-License-Identifier: AGPL-3.0-or-later

//! Coverage gap tests Part 1: 0%-coverage canonical types
//! Targets: `monitoring_config`, hsm/config, hsm/capabilities, hsm/keys,
//!          hsm/android, hsm/status, hsm/discovery, network, metrics,
//!          capabilities, workflow, `health_status`

// ===========================================================================
// canonical/config/domains/monitoring_config.rs (268 lines, 0%)
// ===========================================================================
mod monitoring_config_tests {
    use crate::canonical::config::domains::monitoring_config::*;
    use crate::canonical::config::r#trait::BearDogConfig;

    #[test]
    fn test_consolidated_monitoring_config_default() {
        let c = ConsolidatedMonitoringConfig::default();
        assert!(c.enabled);
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_consolidated_monitoring_config_validate() {
        let c = ConsolidatedMonitoringConfig::default();
        assert!(c.validate().is_ok());
    }

    #[test]
    fn test_consolidated_monitoring_config_merge() {
        let c1 = ConsolidatedMonitoringConfig::default();
        let c2 = ConsolidatedMonitoringConfig::default();
        let merged = c1.merge(&c2);
        assert!(merged.is_ok());
    }

    #[test]
    fn test_consolidated_monitoring_config_from_env() {
        // from_env may fail but should not panic
        let _ = ConsolidatedMonitoringConfig::from_env();
    }

    #[test]
    fn test_metrics_collection_config_default() {
        let c = MetricsCollectionConfig::default();
        assert!(!c.metrics.is_empty());
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_analysis_processing_config_default() {
        let c = AnalysisProcessingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_health_monitoring_config_default() {
        let c = HealthMonitoringConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_anomaly_detection_config_default() {
        let c = AnomalyDetectionConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_trend_analysis_config_default() {
        let c = TrendAnalysisConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_security_monitoring_config_default() {
        let c = SecurityMonitoringConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_export_integration_config_default() {
        let c = ExportIntegrationConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_compression_config_default() {
        let c = CompressionConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_monitoring_config_default() {
        let c = MonitoringConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_alert_config_default() {
        let c = AlertConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_dashboard_config_default() {
        let c = DashboardConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_consolidated_clone_eq() {
        let c1 = ConsolidatedMonitoringConfig::default();
        let c2 = c1.clone();
        assert_eq!(c1, c2);
    }
}

// ===========================================================================
// canonical/hsm/config.rs (205 lines, 0%)
// ===========================================================================
mod hsm_config_tests {
    use crate::canonical::hsm::config::*;

    #[test]
    #[expect(
        deprecated,
        reason = "migration in progress — see CANONICAL_TYPE_MIGRATION_GUIDE"
    )]
    fn test_legacy_hsm_provider_type() {
        let p = LegacyHsmProviderType::Software;
        let _ = format!("{p:?}");
        let p2 = p.clone();
        assert_eq!(p, p2);
    }

    #[test]
    fn test_connection_config_default() {
        let c = ConnectionConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_security_config_default() {
        let c = SecurityConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_auth_method_variants() {
        let a = AuthMethod::None;
        let _ = format!("{a:?}");
        let b = AuthMethod::Token {
            token_id: 1,
            pin: None,
        };
        let _ = format!("{b:?}");
        let c = AuthMethod::Certificate {
            cert_path: "/tmp/cert".to_string(),
            key_path: "/tmp/key".to_string(),
        };
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_config_default() {
        let c = HsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_software_hsm_config_default() {
        let c = SoftwareHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hardware_hsm_config_default() {
        let c = HardwareHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_network_hsm_config_default() {
        let c = NetworkHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_cloud_hsm_config_default() {
        let c = CloudHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    #[expect(
        deprecated,
        reason = "migration in progress — see CANONICAL_TYPE_MIGRATION_GUIDE"
    )]
    fn test_legacy_provider_type_display() {
        let variants = vec![
            LegacyHsmProviderType::Software,
            LegacyHsmProviderType::Hardware,
            LegacyHsmProviderType::Network,
            LegacyHsmProviderType::Cloud,
            LegacyHsmProviderType::Mobile,
            LegacyHsmProviderType::Custom {
                name: "test".to_string(),
            },
        ];
        for v in variants {
            let s = format!("{v}");
            assert!(!s.is_empty());
        }
    }
}

// ===========================================================================
// canonical/hsm/capabilities.rs (196 lines, 0%)
// ===========================================================================
mod hsm_capabilities_tests {
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
    fn test_api_support_capabilities_default() {
        let c = ApiSupportCapabilities::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/hsm/keys.rs (158 lines, 0%)
// ===========================================================================
mod hsm_keys_tests {
    use crate::canonical::hsm::keys::*;

    #[test]
    fn test_hsm_key_default() {
        let k = HsmKey::default();
        let _ = format!("{k:?}");
    }

    #[test]
    fn test_key_usage_variants() {
        let variants = vec![
            KeyUsage::Encrypt,
            KeyUsage::Decrypt,
            KeyUsage::Sign,
            KeyUsage::Verify,
            KeyUsage::Wrap,
            KeyUsage::Unwrap,
            KeyUsage::Derive,
        ];
        for v in variants {
            let _ = format!("{v:?}");
        }
    }

    #[test]
    fn test_key_material_default() {
        let m = KeyMaterial::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_storage_type_variants() {
        let s = StorageType::Software;
        let _ = format!("{s:?}");
        let s2 = StorageType::Hardware;
        let _ = format!("{s2:?}");
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
    fn test_key_lifecycle_state_variants() {
        let s = KeyLifecycleState::Active;
        let _ = format!("{s:?}");
    }

    #[test]
    fn test_key_lifecycle_state_all_variants() {
        let variants = vec![
            KeyLifecycleState::Active,
            KeyLifecycleState::Suspended,
            KeyLifecycleState::Compromised,
        ];
        for v in variants {
            let _ = format!("{v:?}");
        }
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
}

// ===========================================================================
// canonical/hsm/android.rs (146 lines, 0%)
// ===========================================================================
mod hsm_android_tests {
    use crate::canonical::hsm::android::*;

    #[test]
    fn test_android_device_info_default() {
        let d = AndroidDeviceInfo::default();
        let _ = format!("{d:?}");
    }

    #[test]
    fn test_device_integrity_default() {
        let d = DeviceIntegrity::default();
        let _ = format!("{d:?}");
    }

    #[test]
    fn test_android_keystore_config_default() {
        let c = AndroidKeystoreConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_strongbox_config_default() {
        let c = StrongBoxConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_biometric_config_default() {
        let c = BiometricConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_biometric_type_variants() {
        let _ = format!("{:?}", BiometricType::Fingerprint);
        let _ = format!("{:?}", BiometricType::Face);
        let _ = format!("{:?}", BiometricType::Iris);
        let _ = format!("{:?}", BiometricType::Voice);
    }

    #[test]
    fn test_safety_net_config_default() {
        let c = SafetyNetConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_android_hsm_session_default() {
        let s = AndroidHsmSession::default();
        let _ = format!("{s:?}");
    }

    #[test]
    fn test_android_hsm_config_default() {
        let c = AndroidHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_security_requirements_default() {
        let r = SecurityRequirements::default();
        let _ = format!("{r:?}");
    }
}

// ===========================================================================
// canonical/hsm/status.rs (133 lines, 0%)
// ===========================================================================
mod hsm_status_tests {
    use crate::canonical::hsm::status::*;

    #[test]
    fn test_hsm_status_default() {
        let s = HsmStatus::default();
        let _ = format!("{s:?}");
    }

    #[test]
    fn test_hsm_health_status_variants() {
        let variants = vec![
            HsmHealthStatus::Healthy,
            HsmHealthStatus::Degraded,
            HsmHealthStatus::Unhealthy,
            HsmHealthStatus::Unknown,
        ];
        for v in variants {
            let _ = format!("{v:?}");
        }
    }

    #[test]
    fn test_hsm_performance_metrics_default() {
        let m = PerformanceMetrics::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_hsm_health_check_config_default() {
        let c = HsmHealthCheckConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_health_metrics_default() {
        let m = HealthMetrics::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_hsm_health_default() {
        let h = HsmHealth::default();
        let _ = format!("{h:?}");
    }

    #[test]
    fn test_hsm_status_clone() {
        let s1 = HsmStatus::default();
        let s2 = s1;
        let _ = format!("{s2:?}");
    }
}

// ===========================================================================
// canonical/network.rs (105 lines, 0%)
// ===========================================================================
mod canonical_network_tests {
    use crate::canonical::network::*;

    #[test]
    fn test_network_config_default() {
        let c = NetworkConfig::default();
        let _ = format!("{c:?}");
        assert!(!c.bind_address.is_empty());
    }

    #[test]
    fn test_timeout_config_default() {
        let c = TimeoutConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_network_config_clone() {
        let c1 = NetworkConfig::default();
        let c2 = c1.clone();
        assert_eq!(c1.port, c2.port);
    }
}

// ===========================================================================
// metrics.rs (71 lines, 0%)
// ===========================================================================
mod metrics_tests {
    use crate::metrics::*;

    #[test]
    fn test_system_metrics_default() {
        let m = SystemMetrics::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_cpu_metrics_default() {
        let m = CpuMetrics::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_memory_metrics_default() {
        let m = MemoryMetrics::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_disk_metrics_default() {
        let m = DiskMetrics::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_network_metrics_default() {
        let m = NetworkMetrics::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_system_metrics_clone() {
        let m1 = SystemMetrics::default();
        let m2 = m1;
        let _ = format!("{m2:?}");
    }
}

// ===========================================================================
// capabilities.rs (47 lines, 0%)
// ===========================================================================
mod capabilities_tests_gap {
    #[test]
    fn test_capability_metadata_default() {
        let m = crate::capabilities::CapabilityMetadata::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_canonical_security_level_default() {
        let l = crate::canonical::capabilities::SecurityLevel::default();
        let _ = format!("{l:?}");
    }

    #[test]
    fn test_canonical_circuit_breaker_config_default() {
        let c = crate::canonical::capabilities::CircuitBreakerConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_canonical_performance_metrics_default() {
        let m = crate::canonical::capabilities::PerformanceMetrics::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_canonical_security_capabilities_default() {
        let c = crate::canonical::capabilities::SecurityCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_canonical_network_capabilities_default() {
        let c = crate::canonical::capabilities::NetworkCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_canonical_storage_capabilities_default() {
        let c = crate::canonical::capabilities::StorageCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_canonical_compute_capabilities_default() {
        let c = crate::canonical::capabilities::ComputeCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_canonical_capability_requirements_default() {
        let r = crate::canonical::capabilities::CapabilityRequirements::default();
        let _ = format!("{r:?}");
    }
}

// ===========================================================================
// workflow.rs (32 lines, 0%)
// ===========================================================================
mod workflow_tests_gap {
    use crate::workflow::*;

    #[test]
    fn test_workflow_default() {
        let w = Workflow::default();
        let _ = format!("{w:?}");
    }

    #[test]
    fn test_workflow_clone() {
        let w1 = Workflow::default();
        let w2 = w1;
        let _ = format!("{w2:?}");
    }
}

// ===========================================================================
// canonical/health_status.rs (30 lines, 0%)
// ===========================================================================
mod canonical_health_status_tests {
    use crate::canonical::health_status::*;

    #[test]
    fn test_health_status_variants() {
        let variants = vec![
            HealthStatus::Healthy,
            HealthStatus::Degraded,
            HealthStatus::Unhealthy,
            HealthStatus::Unknown,
        ];
        for v in variants {
            let _ = format!("{v:?}");
        }
    }

    #[test]
    fn test_component_status_variants() {
        let _ = format!("{:?}", ComponentStatus::Running);
        let _ = format!("{:?}", ComponentStatus::Starting);
        let _ = format!("{:?}", ComponentStatus::Stopping);
        let _ = format!("{:?}", ComponentStatus::Active);
        let _ = format!("{:?}", ComponentStatus::Inactive);
        let _ = format!("{:?}", ComponentStatus::Failed);
        let _ = format!("{:?}", ComponentStatus::Maintenance);
        let _ = format!("{:?}", ComponentStatus::Error("test error".to_string()));
    }

    #[test]
    fn test_operation_status_variants() {
        let _ = format!("{:?}", OperationStatus::Pending);
        let _ = format!("{:?}", OperationStatus::InProgress);
        let _ = format!("{:?}", OperationStatus::Completed);
        let _ = format!("{:?}", OperationStatus::Success);
        let _ = format!("{:?}", OperationStatus::Cancelled);
    }

    #[test]
    fn test_key_status_variants() {
        let _ = format!("{:?}", KeyStatus::Compromised);
        let _ = format!("{:?}", KeyStatus::Revoked);
        let _ = format!("{:?}", KeyStatus::PendingActivation);
    }

    #[test]
    fn test_workflow_status_variants() {
        let _ = format!("{:?}", WorkflowStatus::Created);
        let _ = format!("{:?}", WorkflowStatus::Running);
        let _ = format!("{:?}", WorkflowStatus::Paused);
        let _ = format!("{:?}", WorkflowStatus::PendingApprovals);
        let _ = format!("{:?}", WorkflowStatus::Approved);
        let _ = format!("{:?}", WorkflowStatus::Rejected);
        let _ = format!("{:?}", WorkflowStatus::Expired);
    }

    #[test]
    fn test_default_impls() {
        let _ = format!("{:?}", HealthStatus::default());
        let _ = format!("{:?}", ComponentStatus::default());
        let _ = format!("{:?}", OperationStatus::default());
        let _ = format!("{:?}", KeyStatus::default());
        let _ = format!("{:?}", WorkflowStatus::default());
    }
}

// ===========================================================================
// canonical/hsm/discovery.rs (31 lines, 0%)
// ===========================================================================
mod hsm_discovery_tests {
    use crate::canonical::hsm::discovery::*;

    #[test]
    fn test_hsm_discovery_config_default() {
        let c = HsmDiscoveryConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_discovery_config_clone() {
        let c1 = HsmDiscoveryConfig::default();
        let c2 = c1;
        let _ = format!("{c2:?}");
    }
}
