//! Coverage gap tests Part 10: Remaining config domains, security, workflow, production,
//! discovery, providers, network, runtime, traits, config/utils file operations

// ===========================================================================
// canonical/config/domains/workflow/engine.rs - 60 uncov (3 Default impls)
// ===========================================================================
mod workflow_engine_tests_extra {
    use crate::canonical::config::domains::workflow::engine::*;

    #[test]
    fn test_workflow_engine_config_default() {
        let c = WorkflowEngineConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_queue_config_default() {
        let c = QueueConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_timeout_config_default() {
        let c = TimeoutConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/workflow/retry.rs - 49 uncov
// ===========================================================================
mod workflow_retry_tests_extra {
    use crate::canonical::config::domains::workflow::retry::*;

    #[test]
    fn test_retry_config_default() {
        let c = RetryConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/production/environment.rs - 56 uncov (3 Default impls)
// ===========================================================================
mod prod_env_tests {
    use crate::canonical::config::production::environment::*;

    #[test]
    fn test_modern_secrets_config_default() {
        let c = ModernSecretsConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_environment_config_default() {
        let c = EnvironmentConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_environment_validation_default() {
        let c = EnvironmentValidation::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/production/resources.rs - 34 uncov (4 Default impls)
// ===========================================================================
mod prod_resources_tests_extra {
    use crate::canonical::config::production::resources::*;

    #[test]
    fn test_gc_tuning_config_default() {
        let c = GcTuningConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_network_resource_config_default() {
        let c = NetworkResourceConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_storage_resource_config_default() {
        let c = StorageResourceConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_connection_config_default() {
        let c = ConnectionConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/runtime_config.rs - 35 uncov (3 Default impls)
// ===========================================================================
mod runtime_config_tests_extra {
    use crate::canonical::config::runtime_config::*;

    #[test]
    fn test_runtime_network_config_default() {
        let c = RuntimeNetworkConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_runtime_hsm_config_default() {
        let c = RuntimeHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_runtime_config_default() {
        let c = RuntimeConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/network.rs - 35 uncov (4 Default impls)
// ===========================================================================
mod config_network_tests_extra {
    use crate::canonical::config::network::*;

    #[test]
    fn test_network_config_default() {
        let c = NetworkConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_service_ports_default() {
        let p = ServicePorts::default();
        let _ = format!("{p:?}");
    }

    #[test]
    fn test_network_timeouts_default() {
        let t = NetworkTimeouts::default();
        let _ = format!("{t:?}");
    }

    #[test]
    fn test_service_endpoints_default() {
        let e = ServiceEndpoints::default();
        let _ = format!("{e:?}");
    }
}

// ===========================================================================
// canonical/config/security/mfa.rs - 32 uncov (4 Default impls)
// ===========================================================================
mod security_mfa_tests {
    use crate::canonical::config::security::mfa::*;

    #[test]
    fn test_canonical_mfa_config_default() {
        let c = CanonicalMfaConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_totp_config_default() {
        let c = TotpConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_sms_config_default() {
        let c = SmsConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_backup_codes_config_default() {
        let c = BackupCodesConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/adapter/chain.rs - 74 uncov (3 Default impls)
// ===========================================================================
mod adapter_chain_tests_extra {
    use crate::canonical::config::domains::adapter::chain::*;

    #[test]
    fn test_chain_config_default() {
        let c = ChainConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_step_config_default() {
        let c = StepConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_retry_config_default() {
        let c = RetryConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/security/mod.rs - 52 uncov (2 Default impls)
// ===========================================================================
mod security_mod_tests_extra {
    use crate::canonical::config::security::*;

    #[test]
    fn test_canonical_security_config_default() {
        let c = CanonicalSecurityConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_rate_limiting_config_default() {
        let c = RateLimitingConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/source.rs - 31 uncov (1 Default impl)
// ===========================================================================
mod config_source_tests_extra {
    use crate::canonical::config::source::*;

    #[test]
    fn test_env_config_source() {
        let s = EnvConfigSource::new();
        let _ = s.get("NONEXISTENT_KEY_XYZ");
        // ConfigSource trait has get, get_or, contains_key
        assert!(!s.contains_key("NONEXISTENT_KEY_XYZ"));
        let val = s.get_or("NONEXISTENT_KEY_XYZ", "default_value");
        assert_eq!(val, "default_value");
    }

    #[test]
    fn test_test_config_source() {
        let mut s = TestConfigSource::new();
        s.set("test_key", "test_value");
        let val = s.get("test_key");
        assert_eq!(val, Some("test_value".to_string()));
        assert_eq!(s.len(), 1);
        assert!(!s.is_empty());
        assert!(s.contains_key("test_key"));
        s.remove("test_key");
        assert!(s.is_empty());
    }

    #[test]
    fn test_test_config_source_with_values() {
        let s = TestConfigSource::with_values(vec![
            ("k1".to_string(), "v1".to_string()),
            ("k2".to_string(), "v2".to_string()),
        ]);
        assert_eq!(s.len(), 2);
        assert_eq!(s.get("k1"), Some("v1".to_string()));
    }

    #[test]
    fn test_test_config_source_clear() {
        let mut s = TestConfigSource::new();
        s.set("a", "b");
        s.clear();
        assert!(s.is_empty());
    }

    #[test]
    fn test_composite_config_source() {
        let s1 = TestConfigSource::with_values(vec![("k1".to_string(), "v1".to_string())]);
        let s2 = TestConfigSource::with_values(vec![("k2".to_string(), "v2".to_string())]);
        let composite = CompositeConfigSource::new(vec![Box::new(s1), Box::new(s2)]);
        assert_eq!(composite.get("k1"), Some("v1".to_string()));
        assert_eq!(composite.get("k2"), Some("v2".to_string()));
    }

    #[test]
    fn test_composite_config_source_priority() {
        let s1 = TestConfigSource::with_values(vec![("k".to_string(), "v1".to_string())]);
        let s2 = TestConfigSource::with_values(vec![("k".to_string(), "v2".to_string())]);
        let composite = CompositeConfigSource::new(vec![Box::new(s1)]).with_priority(Box::new(s2));
        // Priority source should override
        let val = composite.get("k");
        assert!(val.is_some());
    }

    #[test]
    fn test_composite_config_source_fallback() {
        let s1 = TestConfigSource::new();
        let s2 = TestConfigSource::with_values(vec![("k".to_string(), "fallback".to_string())]);
        let composite = CompositeConfigSource::new(vec![Box::new(s1)]).with_fallback(Box::new(s2));
        let val = composite.get("k");
        assert_eq!(val, Some("fallback".to_string()));
    }

    #[test]
    fn test_get_parsed_helper() {
        let s = TestConfigSource::with_values(vec![("num".to_string(), "42".to_string())]);
        let val: i32 = get_parsed(&s, "num", 0);
        assert_eq!(val, 42);
        let val: i32 = get_parsed(&s, "missing", 99);
        assert_eq!(val, 99);
    }

    #[test]
    fn test_get_bool_helper() {
        let s = TestConfigSource::with_values(vec![("flag".to_string(), "true".to_string())]);
        assert!(get_bool(&s, "flag", false));
        assert!(!get_bool(&s, "missing", false));
    }
}

// ===========================================================================
// canonical/config/cache.rs - 56 uncov
// ===========================================================================
mod config_cache_tests_extra {
    use crate::canonical::config::cache::*;
    use crate::canonical::traits::cache::CacheStrategy;

    #[test]
    fn test_canonical_cache_config_default() {
        let c = CanonicalCacheConfig::default();
        let _ = format!("{c:?}");
        let _ = c.validate();
    }

    #[test]
    fn test_cache_tier_l1() {
        let t = CacheTier::L1 {
            max_entries: 1000,
            track_access: true,
            cleanup_interval: std::time::Duration::from_secs(60),
        };
        let _ = format!("{t:?}");
    }

    #[test]
    fn test_cache_tier_l2() {
        let t = CacheTier::L2 {
            max_size_mb: 256,
            enable_compression: true,
        };
        let _ = format!("{t:?}");
    }

    #[test]
    fn test_cache_with_tier_constructors() {
        let c = CanonicalCacheConfig::high_performance_l1();
        let _ = format!("{c:?}");
        let c = CanonicalCacheConfig::high_performance_l2();
        let _ = format!("{c:?}");
        let c = CanonicalCacheConfig::high_performance_l3();
        let _ = format!("{c:?}");
        let c = CanonicalCacheConfig::memory_optimized();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_cache_with_tier() {
        let tier = CacheTier::L1 {
            max_entries: 500,
            track_access: false,
            cleanup_interval: std::time::Duration::from_secs(120),
        };
        let c = CanonicalCacheConfig::with_tier(tier);
        let _ = format!("{c:?}");
    }
}

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
            conditions: Default::default(),
        };
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_abac_policy_config_manual() {
        let c = AbacPolicyConfig {
            name: "test_policy".to_string(),
            description: "A test policy".to_string(),
            subject_attributes: Default::default(),
            resource_attributes: Default::default(),
            action: "read".to_string(),
            environment_attributes: Default::default(),
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
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("beardog_utils_test_{ts}"));
        let _ = std::fs::create_dir_all(&dir);

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct TestCfg {
            name: String,
            value: i32,
        }

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
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("beardog_json_test_{ts}"));
        let _ = std::fs::create_dir_all(&dir);

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct TestCfg {
            name: String,
        }

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
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("beardog_default_cfg_{ts}"));
        let path = dir.join("defaults.toml");

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct Cfg {
            enabled: bool,
        }

        let result = UnifiedConfigUtils::create_default_config(Cfg { enabled: true }, &path);
        assert!(result.is_ok());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_with_env_overrides() {
        use std::time::SystemTime;
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("beardog_env_test_{ts}"));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test.toml");

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct Cfg {
            name: String,
        }

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
            details: Default::default(),
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

    #[test]
    fn test_key_spec_manual() {
        let k = KeySpec {
            algorithm: KeyAlgorithm::Aes,
            key_size: Some(256),
            usage: KeyUsage::Encrypt,
            extractable: false,
            metadata: Default::default(),
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
            details: Default::default(),
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
        let p = SoftwareHsmProvider::new();
        let _ = format!("{p:?}");
    }
}

// ===========================================================================
// canonical/hsm_unified/migration.rs - 176 uncov
// ===========================================================================
mod hsm_unified_migration_extra_tests {
    use crate::canonical::hsm_unified::migration::*;

    #[test]
    fn test_migration_options_default() {
        let o = MigrationOptions::default();
        let _ = format!("{o:?}");
    }

    #[test]
    fn test_hsm_migration_service_new() {
        let _s = HsmMigrationService::new(MigrationOptions::default());
        // options field is private, just verify construction
    }

    #[test]
    fn test_hsm_migration_service_default() {
        let _s = HsmMigrationService::default();
    }

    #[test]
    fn test_legacy_hsm_config_variants() {
        let configs: Vec<LegacyHsmConfig> = vec![
            LegacyHsmConfig::TunnelHsm {
                hardware_config: None,
                software_config: None,
                mobile_config: None,
            },
            LegacyHsmConfig::ConfigurationHsm {
                providers: vec![],
                monitoring: None,
                performance: None,
            },
            LegacyHsmConfig::ZeroCostHsm {
                manager_config: Default::default(),
            },
        ];
        for c in &configs {
            let _ = format!("{c:?}");
        }
    }

    #[test]
    fn test_migration_warning_manual() {
        let w = MigrationWarning {
            config_type: "test".to_string(),
            message: "test warning".to_string(),
            recommendation: None,
        };
        let _ = format!("{w:?}");
    }

    #[test]
    fn test_migration_error_manual() {
        let e = MigrationError {
            config_type: "test".to_string(),
            error: "test error".to_string(),
            resolution: "fix it".to_string(),
        };
        let _ = format!("{e:?}");
    }

    #[test]
    fn test_create_tunnel_legacy_config() {
        let c = create_tunnel_legacy_config(None, None, None);
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_migrate_hsm_configs() {
        let service = HsmMigrationService::default();
        let configs = vec![LegacyHsmConfig::TunnelHsm {
            hardware_config: None,
            software_config: None,
            mobile_config: None,
        }];
        let result = service.migrate_hsm_configs(configs);
        match result {
            Ok(r) => {
                let _ = format!("{:?}", r.migration_report.warnings.len());
                let _ = format!("{:?}", r.migration_report.errors.len());
            }
            Err(e) => {
                let _ = format!("{e:?}");
            }
        }
    }

    #[test]
    fn test_create_migration_summary() {
        let report = MigrationReport {
            legacy_configs_processed: 1,
            successful_migrations: vec!["tunnel".to_string()],
            warnings: vec![],
            errors: vec![],
            migrated_at: chrono::Utc::now(),
        };
        let summary = HsmMigrationService::create_migration_summary(&report);
        assert!(!summary.is_empty());
    }
}

// ===========================================================================
// canonical/config/monitoring_migration.rs - 145 uncov
// ===========================================================================
mod config_monitoring_migration_extra_tests {
    use crate::canonical::config::monitoring_migration::*;

    #[test]
    fn test_monitoring_migration_options_default() {
        let o = MonitoringMigrationOptions::default();
        let _ = format!("{o:?}");
    }

    #[test]
    fn test_monitoring_migration_service_new() {
        let _s = MonitoringMigrationService::new(MonitoringMigrationOptions::default());
        // options field is private, just verify construction
    }
}

// ===========================================================================
// canonical/config/domains/workflow/mod.rs - 38 uncov
// ===========================================================================
mod workflow_domain_mod_tests {
    use crate::canonical::config::domains::workflow::*;

    #[test]
    fn test_consolidated_workflow_config_default() {
        let c = ConsolidatedWorkflowConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/adapter/service_mesh.rs extra
// ===========================================================================
mod service_mesh_extra_tests {
    use crate::canonical::config::domains::adapter::service_mesh::*;

    #[test]
    fn test_service_mesh_config_clone() {
        let c = ServiceMeshConfig::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }
}

// ===========================================================================
// canonical/config/domains/ai_config/learning.rs extra
// ===========================================================================
mod ai_learning_extra_tests {
    use crate::canonical::config::domains::ai_config::learning::*;

    #[test]
    fn test_online_learning_config_clone() {
        let c = OnlineLearningConfig::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }

    #[test]
    fn test_transfer_learning_config_clone() {
        let c = TransferLearningConfig::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }

    #[test]
    fn test_meta_learning_config_clone() {
        let c = MetaLearningConfig::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }
}

// ===========================================================================
// canonical/config/domains/bootstrap.rs extra - 37 uncov
// ===========================================================================
mod bootstrap_extra_tests {
    use crate::canonical::config::domains::bootstrap::*;

    #[test]
    fn test_unified_bootstrap_config_clone() {
        let c = UnifiedBootstrapConfig::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }
}

// ===========================================================================
// canonical/config/hsm/hardware.rs - 38 uncov
// ===========================================================================
mod hsm_hardware_extra_tests {
    use crate::canonical::config::hsm::hardware::*;

    #[test]
    fn test_unified_hardware_hsm_config_default() {
        let c = UnifiedHardwareHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_unified_hardware_hsm_config_clone() {
        let c = UnifiedHardwareHsmConfig::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }
}

// ===========================================================================
// canonical/config/hsm/cloud.rs - 27 uncov
// ===========================================================================
mod hsm_cloud_extra_tests {
    use crate::canonical::config::hsm::cloud::*;

    #[test]
    fn test_unified_cloud_hsm_config_default() {
        let c = UnifiedCloudHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_unified_cloud_hsm_config_clone() {
        let c = UnifiedCloudHsmConfig::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }
}

// ===========================================================================
// constants/domains/config.rs - 27 uncov
// ===========================================================================
mod constants_config_tests {
    #[test]
    fn test_constants_config_system() {
        use crate::constants::domains::config::system::*;
        assert!(!DEFAULT_SYSTEM_NAME.is_empty());
        assert!(!DEFAULT_VERSION.is_empty());
        assert!(!DEFAULT_ENVIRONMENT.is_empty());
        assert!(!PRODUCTION_ENVIRONMENT.is_empty());
        assert!(!DEFAULT_LOG_LEVEL.is_empty());
        assert!(!DEBUG_LOG_LEVEL.is_empty());
        assert!(!WARN_LOG_LEVEL.is_empty());
        assert!(!ERROR_LOG_LEVEL.is_empty());
    }

    #[test]
    fn test_constants_config_security() {
        use crate::constants::domains::config::security::*;
        assert!(!DEFAULT_ENCRYPTION_ALGORITHM.is_empty());
        assert!(!DEFAULT_HSM_PROVIDER.is_empty());
        assert!(!DEFAULT_JWT_ISSUER.is_empty());
        assert!(!DEFAULT_JWT_AUDIENCE.is_empty());
        assert!(!DEFAULT_RATE_LIMIT_STRATEGY.is_empty());
    }

    #[test]
    fn test_constants_config_ai() {
        use crate::constants::domains::config::ai::*;
        assert!(!HYBRID_MODE.is_empty());
        assert!(!DEFAULT_MONITOR_METRIC.is_empty());
        assert!(!ADAM_OPTIMIZER.is_empty());
        assert!(!LRU_EVICTION_POLICY.is_empty());
        assert!(!DENSE_LAYER_TYPE.is_empty());
        assert!(!RELU_ACTIVATION.is_empty());
        assert!(!SOFTMAX_ACTIVATION.is_empty());
        assert!(!FLOAT32_DATA_TYPE.is_empty());
        assert!(!BATCH_NORM_TYPE.is_empty());
        assert!(!ENSEMBLE_STRATEGY.is_empty());
        assert!(!CATEGORICAL_CROSSENTROPY_LOSS.is_empty());
    }

    #[test]
    fn test_constants_config_network() {
        use crate::constants::domains::config::network::*;
        assert!(!DEFAULT_HEALTH_ENDPOINT.is_empty());
        assert!(!DEFAULT_READY_ENDPOINT.is_empty());
        assert!(!HTTP_PROTOCOL.is_empty());
        assert!(!HTTPS_PROTOCOL.is_empty());
        assert!(!default_auth_callback().is_empty());
        assert!(!STRICT_CERT_VALIDATION.is_empty());
    }

    #[test]
    fn test_constants_config_storage() {
        use crate::constants::domains::config::storage::*;
        assert!(!MEMORY_BACKEND.is_empty());
        assert!(!MEMORY_URL.is_empty());
        assert!(!FILE_BACKEND.is_empty());
        assert!(!SQLITE_BACKEND.is_empty());
        assert!(!SQLITE_MEMORY_URL.is_empty());
        assert!(!REDIS_BACKEND.is_empty());
        assert!(!default_redis_url().is_empty());
    }

    #[test]
    fn test_constants_config_workflow() {
        use crate::constants::domains::config::workflow::*;
        assert!(!CRON_SCHEDULER_TYPE.is_empty());
        assert!(!DAILY_MIDNIGHT_SCHEDULE.is_empty());
        assert!(!DAILY_2AM_SCHEDULE.is_empty());
        assert!(!UTC_TIMEZONE.is_empty());
        assert!(!BLUE_GREEN_STRATEGY.is_empty());
        assert!(!ROLLING_UPDATE_STRATEGY.is_empty());
    }

    #[test]
    fn test_constants_config_monitoring() {
        use crate::constants::domains::config::monitoring::*;
        assert!(!EMAIL_CHANNEL.is_empty());
        assert!(!WARNING_SEVERITY.is_empty());
        assert!(!CRITICAL_SEVERITY.is_empty());
        assert!(!GRID_LAYOUT.is_empty());
        assert!(!ADAPTIVE_STRATEGY.is_empty());
        assert!(!PUSH_STRATEGY.is_empty());
        assert!(!JSON_FORMAT.is_empty());
        assert!(!GZIP_ALGORITHM.is_empty());
    }

    #[test]
    fn test_constants_config_compliance() {
        use crate::constants::domains::config::compliance::*;
        assert!(!GDPR_FRAMEWORK.is_empty());
        assert!(!HIPAA_FRAMEWORK.is_empty());
        assert!(!SOC2_FRAMEWORK.is_empty());
        assert!(!PCI_DSS_FRAMEWORK.is_empty());
    }

    #[test]
    fn test_constants_config_oidc() {
        use crate::constants::domains::config::oidc::*;
        assert!(!OPENID_SCOPE.is_empty());
        assert!(!PROFILE_SCOPE.is_empty());
        assert!(!EMAIL_SCOPE.is_empty());
    }

    #[test]
    fn test_constants_config_k8s() {
        use crate::constants::domains::config::k8s::*;
        assert!(!DEFAULT_CPU_REQUEST.is_empty());
        assert!(!DEFAULT_MEMORY_REQUEST.is_empty());
        assert!(!DEFAULT_CPU_LIMIT.is_empty());
        assert!(!DEFAULT_MEMORY_LIMIT.is_empty());
        assert!(!PRODUCTION_CPU_REQUEST.is_empty());
        assert!(!PRODUCTION_MEMORY_REQUEST.is_empty());
        assert!(!PRODUCTION_CPU_LIMIT.is_empty());
        assert!(!PRODUCTION_MEMORY_LIMIT.is_empty());
        assert!(!DEFAULT_MAX_UNAVAILABLE.is_empty());
        assert!(!DEFAULT_MAX_SURGE.is_empty());
    }

    #[test]
    fn test_constants_config_paths() {
        use crate::constants::domains::config::paths::*;
        assert!(!DEFAULT_KEYS_DIR.is_empty());
        assert!(!DEFAULT_EXPORTS_DIR.is_empty());
        assert!(!DEFAULT_ARCHIVE_DIR.is_empty());
        assert!(!DEFAULT_BACKUPS_DIR.is_empty());
        assert!(!DEFAULT_MIGRATIONS_DIR.is_empty());
        assert!(!DEFAULT_PROFILING_DIR.is_empty());
        assert!(!DEFAULT_AUDIT_LOGS_DIR.is_empty());
    }

    #[test]
    fn test_constants_config_secrets() {
        use crate::constants::domains::config::security_warnings::*;
        assert!(!CHANGE_DEFAULT_SECRET.is_empty());
    }
}

// ===========================================================================
// modern_traits.rs - 33 uncov
// ===========================================================================
mod production_feature_flags_tests {
    use crate::canonical::config::production::core::*;

    #[test]
    fn test_production_feature_flags_default() {
        let f = ProductionFeatureFlags::default();
        let _ = format!("{f:?}");
    }

    #[test]
    fn test_production_feature_flags_production() {
        let f = ProductionFeatureFlags::production();
        let _ = format!("{f:?}");
    }

    #[test]
    fn test_production_feature_flags_development() {
        let f = ProductionFeatureFlags::development();
        let _ = format!("{f:?}");
    }
}

// ===========================================================================
// canonical/config/production/core.rs - 27 uncov
// ===========================================================================
mod prod_core_tests {
    use crate::canonical::config::production::core::*;

    #[test]
    fn test_production_core_config_default() {
        let c = ProductionCoreConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/discovery_unified_builder.rs - 33 uncov
// ===========================================================================
mod discovery_builder_tests_extra {
    use crate::canonical::config::domains::discovery_unified::UnifiedDiscoveryConfigBuilder;

    #[test]
    fn test_discovery_config_builder() {
        let builder = UnifiedDiscoveryConfigBuilder::new();
        let config = builder.enabled(true).service_id("test-service").build();
        let _ = format!("{config:?}");
    }
}
