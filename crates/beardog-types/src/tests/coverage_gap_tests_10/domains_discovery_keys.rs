// SPDX-License-Identifier: AGPL-3.0-or-later

// ===========================================================================
// canonical/config/domains/timeout.rs - 68 uncov
// ===========================================================================
mod timeout_domain_tests {
    use crate::canonical::config::domains::timeout::*;

    #[test]
    fn test_canonical_timeout_config_default() {
        let c = CanonicalTimeoutConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_network_type_variants() {
        let types = [
            NetworkType::Local,
            NetworkType::Lan,
            NetworkType::Wan,
            NetworkType::Unreliable,
        ];
        for t in &types {
            let _ = format!("{t:?}");
        }
    }
}

// ===========================================================================
// canonical/config/domains/security/mod.rs - 64 uncov
// ===========================================================================
mod security_domain_mod_tests_extra {
    use crate::canonical::config::domains::security::*;

    #[test]
    fn test_consolidated_security_config_default() {
        let c = ConsolidatedSecurityConfiguration::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/security/authorization.rs - 64 uncov
// ===========================================================================
mod authorization_tests_extra {
    use crate::canonical::config::security::authorization::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_canonical_authorization_config_default() {
        let c = CanonicalAuthorizationConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_resource_rule_config_manual() {
        let c = ResourceRuleConfig {
            resource_pattern: "/api/users/*".to_string(),
            required_permissions: vec!["read".to_string()],
            methods: vec!["GET".to_string()],
            conditions: BTreeMap::default(),
        };
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_abac_policy_config_manual() {
        let c = AbacPolicyConfig {
            name: "test_policy".to_string(),
            description: "A test policy".to_string(),
            subject_attributes: BTreeMap::default(),
            resource_attributes: BTreeMap::default(),
            action: "read".to_string(),
            environment_attributes: BTreeMap::default(),
            decision: "allow".to_string(),
        };
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/utils.rs - 272 uncov - file operations
// ===========================================================================
mod config_utils_tests_extra {
    use crate::canonical::config::utils::*;

    #[test]
    fn test_shared_config_operations() {
        UnifiedConfigUtils::clear_shared_configs();
        let c = UnifiedConfigUtils::get_shared_config::<String, _>("test_key", || {
            "test_value".to_string()
        });
        assert_eq!(*c, "test_value");
        let c2 =
            UnifiedConfigUtils::get_shared_config::<String, _>("test_key", || "other".to_string());
        assert_eq!(*c2, "test_value"); // cached
        let stats = UnifiedConfigUtils::get_shared_config_stats();
        assert!(stats.active_configs > 0);
        assert!(UnifiedConfigUtils::remove_shared_config("test_key"));
        assert!(!UnifiedConfigUtils::remove_shared_config("nonexistent"));
        UnifiedConfigUtils::clear_shared_configs();
    }

    #[test]
    fn test_get_performance_metrics() {
        let m = UnifiedConfigUtils::get_performance_metrics();
        let _ = format!("{m:?}");
        assert!(m.config_operations_per_second > 0.0);
    }

    #[test]
    fn test_get_standard_config_paths() {
        let paths = UnifiedConfigUtils::get_standard_config_paths("beardog_test");
        assert!(!paths.is_empty());
    }

    #[test]
    fn test_find_config_file_nonexistent() {
        // Won't find a config file for a made-up app
        let path = UnifiedConfigUtils::find_config_file("beardog_test_nonexistent_xyz");
        // May or may not find one depending on system
        let _ = path;
    }

    #[test]
    fn test_validate_config_file_nonexistent() {
        let valid = UnifiedConfigUtils::validate_config_file("/nonexistent/path/config.toml");
        assert!(!valid);
    }

    #[test]
    fn test_load_from_file_nonexistent() {
        let result =
            UnifiedConfigUtils::load_from_file::<serde_json::Value, _>("/nonexistent/config.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_save_and_load_toml() {
        use std::time::SystemTime;

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct TestCfg {
            name: String,
            value: i32,
        }

        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("beardog_utils_test_{ts}"));
        let _ = std::fs::create_dir_all(&dir);

        let cfg = TestCfg {
            name: "test".to_string(),
            value: 42,
        };
        let path = dir.join("test.toml");
        let save_result = UnifiedConfigUtils::save_to_file(&cfg, &path);
        assert!(save_result.is_ok());

        let loaded: Result<TestCfg, _> = UnifiedConfigUtils::load_from_file(&path);
        assert!(loaded.is_ok());
        assert_eq!(loaded.unwrap().value, 42);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_save_and_load_json() {
        use std::time::SystemTime;

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct TestCfg {
            name: String,
        }

        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("beardog_json_test_{ts}"));
        let _ = std::fs::create_dir_all(&dir);

        let cfg = TestCfg {
            name: "json_test".to_string(),
        };
        let path = dir.join("test.json");
        let save_result = UnifiedConfigUtils::save_to_file(&cfg, &path);
        assert!(save_result.is_ok());

        let loaded: Result<TestCfg, _> = UnifiedConfigUtils::load_from_file(&path);
        assert!(loaded.is_ok());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_with_fallback_all_missing() {
        let result = UnifiedConfigUtils::load_with_fallback::<serde_json::Value>(
            "/nonexistent/primary.toml",
            &["/nonexistent/secondary.toml"],
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_auto_load_config_nonexistent() {
        let result = UnifiedConfigUtils::auto_load_config::<serde_json::Value>(
            "beardog_test_nonexistent_xyz",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_create_default_config() {
        use std::time::SystemTime;

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct Cfg {
            enabled: bool,
        }

        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("beardog_default_cfg_{ts}"));
        let path = dir.join("defaults.toml");

        let result = UnifiedConfigUtils::create_default_config(Cfg { enabled: true }, &path);
        assert!(result.is_ok());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_with_env_overrides() {
        use std::time::SystemTime;

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct Cfg {
            name: String,
        }

        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("beardog_env_test_{ts}"));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test.toml");

        let cfg = Cfg {
            name: "env_test".to_string(),
        };
        let _ = UnifiedConfigUtils::save_to_file(&cfg, &path);

        let result = UnifiedConfigUtils::load_with_env_overrides::<Cfg>(
            path.to_str().unwrap(),
            "TEST_BEARDOG_X_",
        );
        // May or may not work depending on environment
        let _ = result;

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_merge_configs() {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct Cfg {
            a: i32,
            b: String,
        }

        let base = Cfg {
            a: 1,
            b: "base".to_string(),
        };
        let over = Cfg {
            a: 2,
            b: "override".to_string(),
        };
        let result = UnifiedConfigUtils::merge_configs(base, over);
        assert!(result.is_ok());
    }
}

// ===========================================================================
// canonical/discovery/service_discovery_capability.rs - 247 uncov
// ===========================================================================
mod service_discovery_extra_tests {
    use crate::canonical::discovery::service_discovery_capability::*;
    use std::collections::HashMap;

    #[test]
    fn test_service_health_variants() {
        let _ = ServiceHealth::Healthy;
        let _ = ServiceHealth::Unhealthy {
            reason: "test".to_string(),
        };
        let _ = ServiceHealth::Unknown;
        let _ = ServiceHealth::Degraded {
            reason: "degraded".to_string(),
        };
    }

    #[test]
    fn test_service_protocol_variants() {
        let protos = [
            ServiceProtocol::Http,
            ServiceProtocol::Https,
            ServiceProtocol::Grpc,
        ];
        for p in &protos {
            let _ = format!("{p:?}");
        }
    }

    #[test]
    fn test_discovery_health_status_manual() {
        let h = DiscoveryHealthStatus {
            is_healthy: true,
            details: HashMap::default(),
            response_time_ms: 5,
        };
        let _ = format!("{h:?}");
    }

    #[test]
    fn test_discovery_capabilities_manual() {
        let c = DiscoveryCapabilities {
            supports_capability_query: true,
            supports_registration: true,
            supports_health_checks: true,
            supports_metadata: true,
            max_results: Some(100),
            registration_ttl_seconds: Some(300),
        };
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_discovery_error_variants() {
        let errors: Vec<DiscoveryError> = vec![
            DiscoveryError::ServiceNotFound {
                criteria: "test".to_string(),
            },
            DiscoveryError::BackendUnavailable {
                provider: "k8s".to_string(),
                reason: "down".to_string(),
            },
            DiscoveryError::InvalidDescriptor {
                reason: "bad".to_string(),
            },
            DiscoveryError::RegistrationFailed {
                reason: "fail".to_string(),
            },
            DiscoveryError::NetworkError {
                details: "timeout".to_string(),
            },
            DiscoveryError::Timeout {
                operation: "search".to_string(),
                duration_ms: 5000,
            },
            DiscoveryError::PermissionDenied {
                resource: "svc".to_string(),
            },
        ];
        for e in &errors {
            let msg = format!("{e}");
            assert!(!msg.is_empty());
        }
    }

    #[test]
    fn test_dns_http_discovery_default() {
        let d = DnsHttpDiscovery::default();
        let _ = format!("{d:?}");
    }

    #[test]
    fn test_dns_http_discovery_new() {
        let d = DnsHttpDiscovery::new();
        let _ = format!("{d:?}");
    }

    #[test]
    fn test_dns_http_discovery_with_domains() {
        let d = DnsHttpDiscovery::with_domains(vec!["example.com".to_string()]);
        let _ = format!("{d:?}");
    }

    #[tokio::test]
    async fn test_consul_discovery_try_create() {
        let result = ConsulDiscovery::try_create().await;
        assert!(result.is_err()); // Not implemented yet
    }

    #[tokio::test]
    async fn test_etcd_discovery_try_create() {
        let result = EtcdDiscovery::try_create().await;
        assert!(result.is_err()); // Not implemented yet
    }
}

// ===========================================================================
// canonical/discovery/key_management_capability.rs - 156 uncov
// ===========================================================================
mod key_management_extra_tests {
    use crate::canonical::discovery::key_management_capability::*;
    use std::collections::HashMap;

    #[test]
    fn test_key_spec_manual() {
        let k = KeySpec {
            algorithm: KeyAlgorithm::Aes,
            key_size: Some(256),
            usage: KeyUsage::Encrypt,
            extractable: false,
            metadata: HashMap::default(),
        };
        let _ = format!("{k:?}");
    }

    #[test]
    fn test_key_algorithm_variants() {
        let algos = [
            KeyAlgorithm::Aes,
            KeyAlgorithm::Rsa,
            KeyAlgorithm::EcdsaP256,
            KeyAlgorithm::EcdsaP384,
            KeyAlgorithm::ChaCha20Poly1305,
            KeyAlgorithm::Ed25519,
        ];
        for a in &algos {
            let _ = format!("{a:?}");
        }
    }

    #[test]
    fn test_key_state_variants() {
        let states = [
            KeyState::Active,
            KeyState::Disabled,
            KeyState::PendingDeletion,
            KeyState::Destroyed,
        ];
        for s in &states {
            let _ = format!("{s:?}");
        }
    }

    #[test]
    fn test_kms_error_variants() {
        let errors: Vec<KmsError> = vec![
            KmsError::KeyNotFound {
                key_id: "test_id".to_string(),
            },
            KmsError::ProviderUnavailable {
                provider: "aws".to_string(),
                reason: "down".to_string(),
            },
            KmsError::OperationNotSupported {
                operation: "wrap".to_string(),
            },
            KmsError::InvalidKeySpec {
                reason: "bad size".to_string(),
            },
            KmsError::CryptoError {
                details: "padding".to_string(),
            },
            KmsError::PermissionDenied {
                resource: "key-1".to_string(),
            },
            KmsError::RateLimitExceeded {
                retry_after_seconds: 60,
            },
            KmsError::NetworkError {
                details: "timeout".to_string(),
            },
        ];
        for e in &errors {
            let msg = format!("{e}");
            assert!(!msg.is_empty());
        }
    }

    #[test]
    fn test_kms_health_status_manual() {
        let h = KmsHealthStatus {
            is_healthy: true,
            details: HashMap::default(),
            response_time_ms: 5,
        };
        let _ = format!("{h:?}");
    }

    #[test]
    fn test_kms_capabilities_manual() {
        let c = KmsCapabilities {
            supports_symmetric: true,
            supports_asymmetric: true,
            supports_signing: true,
            has_hardware_rng: false,
            supports_rotation: true,
            fips_compliant: false,
            algorithms: vec![KeyAlgorithm::Aes, KeyAlgorithm::Rsa],
        };
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_software_hsm_provider_new() {
        let p = SoftwareHsmProvider::new().expect("Software HSM init in test");
        let _ = format!("{p:?}");
    }
}
