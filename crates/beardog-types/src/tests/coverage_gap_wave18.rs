// SPDX-License-Identifier: AGPL-3.0-or-later
//! Coverage wave 18: cloud HSM, service discovery, workflow/security/monitoring config, constants, HSM config, performance.

#![cfg(test)]

use std::sync::Arc;
use std::time::Duration;

use serde::Serialize;

use crate::canonical::capabilities::CapabilityType;
use crate::canonical::capabilities::ServiceCapabilityType;
use crate::canonical::config::domains::adapter::chain::{
    ChainConfig, RetryConfig as AdapterRetryConfig, StepConfig,
};
use crate::canonical::config::domains::workflow::engine::{
    QueueConfig, TimeoutConfig, WorkflowEngineConfig,
};
use crate::canonical::config::domains::workflow::retry::RetryConfig as WorkflowRetryConfig;
use crate::canonical::config::hsm::{HsmConfigValidation, UnifiedCloudHsmConfig};
use crate::canonical::config::security::{
    CanonicalAuthorizationConfig, CanonicalEncryptionConfig, CanonicalMfaConfig,
    CanonicalSecurityConfig, RateLimitingConfig,
};
use crate::canonical::discovery::service_discovery_capability::{
    ConsulDiscovery, DiscoveryError, DnsHttpDiscovery, EtcdDiscovery, KubernetesDiscovery,
    ServiceDiscoveryCapability, create_service_discovery,
};
use crate::canonical::hsm::config::{
    AuthMethod, CloudCredentials, CloudHsmConfig, HsmConfigBuilder, HsmProviderConfig,
    LoadBalancingStrategy, NetworkHsmConfig, UniversalHsmProvider,
};
use crate::canonical::monitoring::MonitoringConfigValidation;
use crate::canonical::monitoring::alerting::{
    AlertCondition, AlertRule, AlertSeverity, ComparisonOperator, NotificationChannelType,
    UnifiedAlertingConfig,
};
use crate::canonical::providers_unified::performance::{
    BufferConfig, CacheType, CachingConfig, CompressionAlgorithm, CompressionConfig,
    PerformanceConfig, PerformanceThresholds,
};
use crate::canonical::traits::RetryStrategy;
use crate::canonical::traits::TimeoutPolicy;
use crate::canonical::traits::cache::CacheStrategy;
use crate::constants::domains::network::addresses;
use crate::constants::domains::network::ipc_discovery::{self};

fn assert_serde_json_roundtrip<T>(v: &T)
where
    T: Serialize + for<'de> serde::Deserialize<'de> + std::fmt::Debug,
{
    let json = serde_json::to_value(v).expect("serialize to serde_json::Value");
    let back: T = serde_json::from_value(json.clone()).expect("deserialize from Value");
    let again = serde_json::to_value(&back).expect("re-serialize after roundtrip");
    assert_eq!(json, again, "serde roundtrip must preserve JSON form");
}

// --- UnifiedCloudHsmConfig (canonical/config/hsm/cloud.rs) ---

#[test]
fn unified_cloud_hsm_default_serde_roundtrip() {
    let d = UnifiedCloudHsmConfig::default();
    assert_serde_json_roundtrip(&d);
    let json = serde_json::to_string(&d).expect("serialize cloud hsm");
    let rt: UnifiedCloudHsmConfig = serde_json::from_str(&json).expect("deserialize cloud hsm");
    assert_eq!(d.enabled, rt.enabled);
}

#[test]
fn unified_cloud_hsm_validate_rejects_enabled_without_capabilities() {
    let cfg = UnifiedCloudHsmConfig {
        enabled: true,
        required_capabilities: None,
    };
    let err = cfg
        .validate()
        .expect_err("validation should fail when enabled without capabilities");
    let _ = format!("{err:?}");
}

#[test]
fn unified_cloud_hsm_validate_accepts_disabled_without_capabilities() {
    let cfg = UnifiedCloudHsmConfig {
        enabled: false,
        required_capabilities: None,
    };
    cfg.validate().expect("disabled cloud HSM without caps");
}

#[test]
fn unified_cloud_hsm_validate_accepts_enabled_with_capabilities() {
    let cfg =
        UnifiedCloudHsmConfig::with_required_capabilities(vec![CapabilityType::KeyManagement]);
    cfg.validate().expect("enabled with caps");
}

#[test]
fn unified_cloud_hsm_get_required_capabilities_falls_back_when_none() {
    let cfg = UnifiedCloudHsmConfig {
        enabled: false,
        required_capabilities: None,
    };
    let caps = cfg.get_required_capabilities();
    assert!(caps.contains(&CapabilityType::KeyManagement));
}

#[test]
fn unified_cloud_hsm_with_required_capabilities_sets_enabled() {
    let cfg = UnifiedCloudHsmConfig::with_required_capabilities(vec![
        CapabilityType::HardwareSecurityModule,
    ]);
    assert!(cfg.is_enabled());
    assert_eq!(cfg.get_required_capabilities().len(), 1);
}

#[test]
fn unified_cloud_hsm_hsm_config_validation_trait_is_compatible() {
    let cfg = UnifiedCloudHsmConfig::default();
    assert!(HsmConfigValidation::is_compatible_with(&cfg, 1));
}

#[test]
fn unified_cloud_hsm_clone_debug() {
    let a = UnifiedCloudHsmConfig::default();
    let b = a.clone();
    assert_eq!(format!("{a:?}"), format!("{b:?}"));
}

// --- discovery factory, kubernetes, providers ---

#[tokio::test]
async fn create_service_discovery_returns_known_provider() {
    let disc = create_service_discovery()
        .await
        .expect("create_service_discovery should succeed");
    let name = disc.provider_name();
    assert!(
        name == "dns-http-fallback" || name == "kubernetes",
        "unexpected discovery provider: {name}"
    );
}

#[tokio::test]
async fn consul_discovery_try_create_is_unimplemented() {
    let err = ConsulDiscovery::try_create()
        .await
        .expect_err("consul should be unavailable");
    assert!(matches!(err, DiscoveryError::BackendUnavailable { .. }));
}

#[tokio::test]
async fn etcd_discovery_try_create_is_unimplemented() {
    let err = EtcdDiscovery::try_create()
        .await
        .expect_err("etcd should be unavailable");
    assert!(matches!(err, DiscoveryError::BackendUnavailable { .. }));
}

#[tokio::test]
async fn dns_http_discovery_default_and_trait_methods() {
    let d = DnsHttpDiscovery::new();
    let d2 = DnsHttpDiscovery::with_domains(vec!["a.test".to_string()]);
    assert_ne!(
        format!("{d:?}"),
        format!("{d2:?}"),
        "debug output should differ for different configs"
    );

    let cap = ServiceCapabilityType::DataStorage;
    let by_cap = d
        .discover_by_capability(cap)
        .await
        .expect("discover by capability");
    assert!(by_cap.is_empty());

    let by_name = d
        .discover_by_name("localhost")
        .await
        .expect("discover localhost");
    assert!(!by_name.is_empty());

    let reg = crate::canonical::discovery::service_discovery_capability::ServiceDescriptor {
        instance_id: crate::canonical::types::ids::ServiceInstanceId::new("x"),
        endpoint: "http://127.0.0.1:1".to_string(),
        capabilities: vec![],
        metadata: std::collections::HashMap::new(),
        health: crate::canonical::discovery::service_discovery_capability::ServiceHealth::Healthy,
        priority: 1,
        protocol: crate::canonical::discovery::service_discovery_capability::ServiceProtocol::Http,
    };
    let _ = d
        .register_service(reg)
        .await
        .expect_err("registration unsupported");

    let id = crate::canonical::types::ids::RegistrationId::new("r1");
    d.unregister_service(&id).await.expect("unregister noop");
    d.renew_registration(&id).await.expect("renew noop");

    let health = d.health_check().await.expect("health");
    assert!(health.is_healthy);
    assert_eq!(d.provider_name(), "dns-http-fallback");
    let caps = d.capabilities();
    assert!(!caps.supports_registration);
}

#[tokio::test]
async fn kubernetes_discovery_try_create_succeeds_or_reports_unavailable() {
    match KubernetesDiscovery::try_create().await {
        Ok(k) => {
            let list = k
                .discover_by_capability(ServiceCapabilityType::ServiceMesh)
                .await
                .expect("discover by capability");
            assert!(list.is_empty());
            let qualified = k
                .discover_by_name("api.ns.svc.cluster.local")
                .await
                .expect("qualified name");
            assert_eq!(qualified.len(), 1);
            assert_eq!(k.provider_name(), "kubernetes");
        }
        Err(e) => {
            assert!(matches!(e, DiscoveryError::BackendUnavailable { .. }));
        }
    }
}

// --- workflow retry (domains/workflow/retry.rs) ---

#[test]
#[expect(
    clippy::cast_possible_truncation,
    reason = "max_attempts fits u32 for RetryStrategy::max_attempts contract in test"
)]
fn workflow_retry_config_default_trait_and_serde() {
    let c = WorkflowRetryConfig::default();
    assert_serde_json_roundtrip(&c);
    let json = serde_json::to_string(&c).expect("serialize workflow retry");
    let rt: WorkflowRetryConfig = serde_json::from_str(&json).expect("deserialize workflow retry");
    assert_eq!(c.max_attempts, rt.max_attempts);

    assert_eq!(RetryStrategy::max_attempts(&c), c.max_attempts as u32);
    let d0 = RetryStrategy::delay_for_attempt(&c, 0);
    assert!(d0 > Duration::ZERO || c.max_delay.is_zero());
    assert!(RetryStrategy::should_retry_error(
        &c,
        &std::io::Error::other("x")
    ));
    assert!(!RetryStrategy::is_limit_reached(&c, 0));
    let total = RetryStrategy::total_delay(&c, 2);
    assert!(total >= Duration::from_millis(0));
}

// --- workflow engine (domains/workflow/engine.rs) ---

#[test]
fn workflow_engine_config_queue_timeout_serde_and_timeout_policy() {
    let cfg = WorkflowEngineConfig {
        engine_type: std::sync::Arc::from("custom"),
        worker_pool_size: 4,
        queue: QueueConfig {
            queue_type: std::sync::Arc::from("memory"),
            capacity: 10,
            message_ttl: Duration::from_secs(60),
            dead_letter_queue: Some("dlq".to_string()),
        },
        timeouts: TimeoutConfig {
            default: Duration::from_secs(20),
            maximum: Duration::from_secs(200),
            connection: Duration::from_secs(5),
            read: Duration::from_secs(15),
        },
    };
    assert_serde_json_roundtrip(&cfg);
    let json = serde_json::to_string(&cfg).expect("serialize workflow engine");
    let rt: WorkflowEngineConfig =
        serde_json::from_str(&json).expect("deserialize workflow engine");
    assert_eq!(cfg.worker_pool_size, rt.worker_pool_size);

    let t = &cfg.timeouts;
    assert_eq!(t.connection_timeout(), t.connection);
    assert_eq!(t.operation_timeout("read"), t.read);
    assert_eq!(t.operation_timeout("connect"), t.connection);
    assert_eq!(t.operation_timeout("other"), t.default);
    assert!(t.should_timeout(Duration::from_secs(100), "read"));
    assert_eq!(t.global_timeout(), Some(t.maximum));
    assert_eq!(t.read_timeout(), t.read);
    assert_eq!(t.write_timeout(), t.default);
    assert!(t.idle_timeout().is_none());
    assert_eq!(
        t.remaining_time(Duration::from_secs(1), "read"),
        t.read.saturating_sub(Duration::from_secs(1))
    );
    t.validate().expect("valid timeouts");

    assert!(t.is_production_ready());
    let mut bad = cfg.clone();
    bad.timeouts.connection = Duration::ZERO;
    assert!(!bad.timeouts.is_production_ready());
}

// --- MFA & security & encryption ---

#[test]
fn canonical_mfa_config_and_nested_validate() {
    let m = CanonicalMfaConfig::default();
    assert_serde_json_roundtrip(&m);
    let json = serde_json::to_string(&m).expect("serialize mfa");
    let rt: CanonicalMfaConfig = serde_json::from_str(&json).expect("deserialize mfa");
    assert_eq!(m.enabled, rt.enabled);

    let prod = CanonicalMfaConfig::production();
    assert!(prod.required);

    m.validate().expect("default mfa validates");

    let mut bad = CanonicalMfaConfig::default();
    bad.totp.enabled = true;
    bad.totp.issuer = String::new();
    bad.validate().expect_err("empty totp issuer");

    let mut bad_sms = CanonicalMfaConfig::default();
    bad_sms.sms.enabled = true;
    bad_sms.sms.from_number = String::new();
    bad_sms.validate().expect_err("sms from number");
}

#[test]
fn canonical_security_config_roundtrip_validate_and_rate_limiting() {
    let mut c = CanonicalSecurityConfig::default();
    c.authentication.jwt_secret = Arc::from("01234567890123456789012345678901");
    c.authorization = CanonicalAuthorizationConfig::production();
    assert_serde_json_roundtrip(&c);
    let json = serde_json::to_string(&c).expect("serialize security");
    let rt: CanonicalSecurityConfig = serde_json::from_str(&json).expect("deserialize security");
    assert_eq!(c.enable_encryption, rt.enable_encryption);

    let p = CanonicalSecurityConfig::production();
    assert!(p.enable_hsm);

    c.validate()
        .expect("security validates with non-placeholder JWT secret");

    let rl = RateLimitingConfig::with_defaults();
    assert_eq!(
        rl.max_requests_per_minute,
        RateLimitingConfig::DEFAULT_MAX_REQUESTS_PER_MINUTE
    );

    let from_map = RateLimitingConfig::from_env_provider(|k| match k {
        "BEARDOG_RATE_LIMIT_MAX_REQUESTS_PER_MIN" => Some("42".to_string()),
        _ => None,
    });
    assert_eq!(from_map.max_requests_per_minute, 42);

    let prod_rl = RateLimitingConfig::production();
    assert!(prod_rl.max_requests_per_minute > 0);
}

#[test]
fn canonical_encryption_validate_and_key_derivation_errors() {
    let e = CanonicalEncryptionConfig::default();
    e.validate().expect("default encryption ok");
    assert_serde_json_roundtrip(&e);
    let json = serde_json::to_string(&e).expect("serialize encryption");
    let rt: CanonicalEncryptionConfig =
        serde_json::from_str(&json).expect("deserialize encryption");
    assert_eq!(e.default_algorithm, rt.default_algorithm);

    let mut bad = CanonicalEncryptionConfig::default();
    bad.default_algorithm = String::new();
    bad.validate().expect_err("empty algorithm");

    let mut enc = CanonicalEncryptionConfig::default();
    enc.key_derivation.iterations = 100;
    enc.validate().expect_err("low iterations");

    let mut enc2 = CanonicalEncryptionConfig::default();
    enc2.key_derivation.salt_length = 4;
    enc2.validate().expect_err("salt too short");
}

// --- alerting ---

#[test]
fn unified_alerting_validate_happy_and_error_paths() {
    let mut a = UnifiedAlertingConfig::default();
    MonitoringConfigValidation::validate(&a).expect("default ok");

    a.evaluation_interval = Duration::ZERO;
    MonitoringConfigValidation::validate(&a).expect_err("zero eval interval");

    let mut a2 = UnifiedAlertingConfig::default();
    a2.notification_timeout = Duration::ZERO;
    MonitoringConfigValidation::validate(&a2).expect_err("zero notification timeout");

    let mut a3 = UnifiedAlertingConfig::default();
    a3.rules.push(AlertRule {
        name: String::new(),
        condition: AlertCondition::MetricThreshold {
            metric: "m".to_string(),
            operator: ComparisonOperator::GreaterThan,
            threshold: 1.0,
        },
        severity: AlertSeverity::High,
        duration: Duration::from_secs(5),
        labels: std::collections::HashMap::new(),
        annotations: std::collections::HashMap::new(),
    });
    MonitoringConfigValidation::validate(&a3).expect_err("empty rule name");

    let mut a4 = UnifiedAlertingConfig::default();
    a4.rules.push(AlertRule {
        name: "r".to_string(),
        condition: AlertCondition::HealthCheckFailure {
            service: "s".to_string(),
            failure_count: 3,
        },
        severity: AlertSeverity::Low,
        duration: Duration::ZERO,
        labels: std::collections::HashMap::new(),
        annotations: std::collections::HashMap::new(),
    });
    MonitoringConfigValidation::validate(&a4).expect_err("zero duration");

    assert!(MonitoringConfigValidation::is_compatible_with(
        &UnifiedAlertingConfig::default(),
        0
    ));
}

#[test]
fn unified_alerting_enums_serde_roundtrip() {
    let cond = AlertCondition::ErrorRate {
        service: "svc".to_string(),
        rate_threshold: 0.05,
        time_window: Duration::from_secs(60),
    };
    assert_serde_json_roundtrip(&cond);

    let n = NotificationChannelType::Custom("x".to_string());
    assert_serde_json_roundtrip(&n);
}

// --- adapter chain ---

#[test]
fn adapter_chain_config_and_retry_strategy() {
    let c = ChainConfig::with_defaults();
    assert_serde_json_roundtrip(&c);
    let json = serde_json::to_string(&c).expect("serialize chain");
    let rt: ChainConfig = serde_json::from_str(&json).expect("deserialize chain");
    assert_eq!(c.max_chain_length, rt.max_chain_length);

    let r = AdapterRetryConfig::default();
    assert_eq!(RetryStrategy::max_attempts(&r), r.max_attempts);
    let d = RetryStrategy::delay_for_attempt(&r, 1);
    assert!(d <= r.max_delay || r.max_delay.is_zero());

    let mut linear = r.clone();
    linear.exponential_backoff = false;
    let d0 = RetryStrategy::delay_for_attempt(&linear, 0);
    assert_eq!(d0, linear.initial_delay.min(linear.max_delay));

    assert!(RetryStrategy::is_limit_reached(&r, r.max_attempts));
    assert!(RetryStrategy::total_delay(&r, 2) >= Duration::ZERO);
}

#[test]
fn step_config_default_serde() {
    let s = StepConfig::default();
    assert_serde_json_roundtrip(&s);
}

// --- network addresses constants ---

#[test]
fn network_address_constants_and_helpers() {
    assert_eq!(addresses::DEFAULT_LOCALHOST_IPV4_STR, "127.0.0.1");
    assert_eq!(
        addresses::LOCALHOST_IPV4,
        addresses::DEFAULT_LOCALHOST_IPV4_STR
    );
    let bind = addresses::default_bind_address();
    assert!(bind.contains('0'));
    let api = addresses::default_api_bind();
    assert!(api.contains(':'));
    assert!(addresses::default_metrics_bind().contains(':'));
    assert!(addresses::default_health_bind().contains(':'));
    assert_eq!(addresses::multicast_address(), "224.0.0.251".to_string());
    let dns = addresses::dns_servers();
    assert_eq!(dns.len(), 3);
}

#[test]
fn network_addresses_bind_helpers_use_ports() {
    let api = addresses::default_api_bind_from_env();
    assert!(api.contains(':'));
    let m = addresses::default_metrics_bind_from_env();
    assert!(m.contains(':'));
    let h = addresses::default_health_bind_from_env();
    assert!(h.contains(':'));
}

#[test]
fn network_addresses_multicast_from_env_returns_string() {
    let m = addresses::multicast_address_from_env();
    assert!(!m.is_empty());
}

#[test]
fn network_dns_servers_from_env_returns_vec() {
    let v = addresses::dns_servers_from_env();
    assert!(!v.is_empty());
}

// --- ipc discovery ---

#[test]
fn ipc_discovery_resolve_subdir_and_paths() {
    assert_eq!(
        ipc_discovery::resolve_biomeos_ipc_subdir_from_optional(Some("  ns  ")),
        "ns"
    );
    assert_eq!(
        ipc_discovery::resolve_biomeos_ipc_subdir_from_optional(None),
        ipc_discovery::BIOMEOS_RUNTIME_SOCKET_SUBDIR
    );

    let p =
        ipc_discovery::biomeos_ipc_socket_dir_from_components(Some("/override/path"), None, None);
    assert_eq!(p, std::path::PathBuf::from("/override/path"));

    let p2 = ipc_discovery::biomeos_ipc_socket_dir_from_components(
        None,
        Some("/run/user/1"),
        Some("custom"),
    );
    assert!(p2.ends_with("custom"));

    let tmp = ipc_discovery::biomeos_tmp_socket_root();
    let p3 = ipc_discovery::biomeos_ipc_socket_dir_from_components(None, None, Some("z"));
    assert!(p3.starts_with(tmp));

    let uri = ipc_discovery::default_upa_registry_unix_uri();
    assert!(uri.starts_with("unix://"));

    assert_eq!(
        ipc_discovery::resolve_upa_registry_endpoint(),
        ipc_discovery::default_upa_registry_unix_uri()
    );
}

#[test]
fn ipc_discovery_biomeos_ipc_socket_dir_from_env_non_empty() {
    let p = ipc_discovery::biomeos_ipc_socket_dir_from_env();
    assert!(!p.as_os_str().is_empty());
}

#[test]
fn ipc_discovery_resolve_upa_registry_endpoint_from_env_matches_default_uri_or_override() {
    let s = ipc_discovery::resolve_upa_registry_endpoint_from_env();
    assert!(
        s.starts_with("unix://") || s.starts_with("http://") || s.starts_with("https://"),
        "registry endpoint should look like a URI: {s}"
    );
}

// --- canonical HSM config ---

#[test]
fn hsm_provider_config_validate_and_description() {
    let mut sw = HsmProviderConfig::default();
    match &mut sw {
        HsmProviderConfig::Software(s) => s.storage_path = String::new(),
        _ => panic!("expected software default"),
    }
    sw.validate().expect_err("empty storage");

    let hw = HsmProviderConfig::Hardware(crate::canonical::hsm::config::HardwareHsmConfig {
        base: crate::canonical::hsm::config::HsmConfig::default(),
        device_path: String::new(),
        hardware_settings: std::collections::HashMap::new(),
        min_firmware_version: None,
        hardware_attestation: false,
    });
    hw.validate().expect_err("empty device");

    let net = HsmProviderConfig::Network(NetworkHsmConfig {
        base: crate::canonical::hsm::config::HsmConfig::default(),
        server_address: String::new(),
        port: 1,
        use_tls: false,
        tls_cert_path: None,
        load_balancing: None,
    });
    net.validate().expect_err("empty server");

    let net2 = HsmProviderConfig::Network(NetworkHsmConfig {
        base: crate::canonical::hsm::config::HsmConfig::default(),
        server_address: "h".to_string(),
        port: 0,
        use_tls: false,
        tls_cert_path: None,
        load_balancing: None,
    });
    net2.validate().expect_err("zero port");

    let cloud = HsmProviderConfig::Cloud(CloudHsmConfig {
        base: crate::canonical::hsm::config::HsmConfig::default(),
        provider: "p".to_string(),
        region: String::new(),
        credentials: CloudCredentials::default(),
        endpoints: std::collections::HashMap::new(),
        options: std::collections::HashMap::new(),
    });
    cloud.validate().expect_err("empty region");

    let cloud2 = HsmProviderConfig::Cloud(CloudHsmConfig {
        base: crate::canonical::hsm::config::HsmConfig::default(),
        provider: "p".to_string(),
        region: "r".to_string(),
        credentials: CloudCredentials::default(),
        endpoints: std::collections::HashMap::new(),
        options: std::collections::HashMap::new(),
    });
    cloud2.validate().expect_err("empty access key");

    let ok = HsmProviderConfig::default();
    ok.validate().expect("default software config");
    let _ = ok.description();
    let _ = ok.provider_type();
    let _ = ok.base_config();
}

#[test]
fn hsm_config_builder_and_auth_methods() {
    let built = HsmConfigBuilder::software()
        .operation_timeout(Duration::from_secs(2))
        .cache_size(50)
        .auth_method(AuthMethod::Password {
            password: "x".to_string(),
        })
        .build()
        .expect("build software");
    assert!(matches!(built, HsmProviderConfig::Software(_)));

    let u = UniversalHsmProvider::Discovered {
        provider_id: "a".to_string(),
        capability_type: "b".to_string(),
        endpoint: "c".to_string(),
    };
    assert_serde_json_roundtrip(&u);

    let lb = LoadBalancingStrategy::RoundRobin;
    assert_serde_json_roundtrip(&lb);
}

#[test]
#[expect(
    deprecated,
    reason = "migration in progress — see CANONICAL_TYPE_MIGRATION_GUIDE"
)]
fn legacy_hsm_provider_type_display_and_default() {
    use crate::canonical::hsm::config::LegacyHsmProviderType;
    let c = LegacyHsmProviderType::Custom {
        name: "x".to_string(),
    };
    assert_eq!(format!("{c}"), "Custom(x)");
    let d: LegacyHsmProviderType = LegacyHsmProviderType::default();
    let _ = format!("{d}");
}

// --- providers_unified performance ---

#[test]
fn performance_config_and_subcomponents_roundtrip() {
    let p = PerformanceConfig::default();
    assert_serde_json_roundtrip(&p);
    let json = serde_json::to_string(&p).expect("serialize performance");
    let rt: PerformanceConfig = serde_json::from_str(&json).expect("deserialize performance");
    assert_eq!(p.max_concurrent_requests, rt.max_concurrent_requests);

    let ct = CacheType::Custom("redis-cluster".to_string());
    assert_serde_json_roundtrip(&ct);

    let alg = CompressionAlgorithm::Zstd;
    assert_serde_json_roundtrip(&alg);

    let comp = CompressionConfig::default();
    assert_serde_json_roundtrip(&comp);

    let buf = BufferConfig::default();
    assert_serde_json_roundtrip(&buf);

    let t = PerformanceThresholds::default();
    assert_serde_json_roundtrip(&t);
}

#[test]
fn performance_from_env_loads_without_panic() {
    let _ = PerformanceConfig::from_env();
    let _ = CachingConfig::from_env();
    let _ = CompressionConfig::from_env();
    let _ = BufferConfig::from_env();
    let _ =
        crate::canonical::providers_unified::performance::PerformanceMonitoringConfig::from_env();
    let _ = PerformanceThresholds::from_env();
    let _ = crate::canonical::providers_unified::performance::PerformanceAlertingConfig::from_env();
}

#[test]
fn caching_config_trait_validate_errors() {
    let mut c = CachingConfig::default();
    c.enabled = true;
    c.max_entries = 0;
    assert!(CacheStrategy::validate(&c).is_err());
}

// --- Additional focused tests (wave 18 target: broad surface coverage) ---

#[test]
fn unified_alerting_config_default_serde_roundtrip() {
    let u = UnifiedAlertingConfig::default();
    assert_serde_json_roundtrip(&u);
}

#[test]
fn alert_severity_and_comparison_operator_roundtrip() {
    assert_serde_json_roundtrip(&AlertSeverity::Critical);
    assert_serde_json_roundtrip(&ComparisonOperator::GreaterThanOrEqual);
}

#[test]
fn chain_config_default_matches_with_defaults() {
    let a = ChainConfig::default();
    let b = ChainConfig::with_defaults();
    assert_eq!(a.max_chain_length, b.max_chain_length);
}

#[test]
fn workflow_engine_config_from_source_uses_engine_type() {
    use crate::canonical::config::source::EnvConfigSource;
    let src = EnvConfigSource::new();
    let w = WorkflowEngineConfig::from_source(&src);
    assert!(!w.engine_type.as_ref().is_empty());
}

#[test]
fn timeout_config_validate_errors_on_zero_connection() {
    let t = TimeoutConfig {
        default: Duration::from_secs(10),
        maximum: Duration::from_secs(100),
        connection: Duration::ZERO,
        read: Duration::from_secs(5),
    };
    t.validate().expect_err("zero connection timeout");
}

#[test]
fn timeout_config_validate_errors_when_max_lt_default() {
    let t = TimeoutConfig {
        default: Duration::from_secs(50),
        maximum: Duration::from_secs(10),
        connection: Duration::from_secs(5),
        read: Duration::from_secs(5),
    };
    t.validate().expect_err("max < default");
}

#[test]
fn queue_config_default_serde() {
    let q = QueueConfig::default();
    assert_serde_json_roundtrip(&q);
}

#[test]
fn canonical_encryption_production_sets_rotation() {
    let p = CanonicalEncryptionConfig::production();
    assert_eq!(p.key_rotation_days, 30);
}

#[test]
fn canonical_mfa_totp_and_backup_serde() {
    use crate::canonical::config::security::{BackupCodesConfig, TotpConfig};
    assert_serde_json_roundtrip(&TotpConfig::default());
    assert_serde_json_roundtrip(&BackupCodesConfig::default());
}

#[test]
fn hsm_encryption_config_default_serde() {
    use crate::canonical::config::security::HsmEncryptionConfig;
    assert_serde_json_roundtrip(&HsmEncryptionConfig::default());
}

#[test]
fn key_derivation_config_validate_ok_at_defaults() {
    use crate::canonical::config::security::KeyDerivationConfig;
    let k = KeyDerivationConfig::default();
    k.validate().expect("default key derivation");
}

#[test]
fn auth_method_variants_roundtrip() {
    assert_serde_json_roundtrip(&AuthMethod::Token {
        token_id: 9,
        pin: Some("p".to_string()),
    });
    assert_serde_json_roundtrip(&AuthMethod::Certificate {
        cert_path: "/c".to_string(),
        key_path: "/k".to_string(),
    });
}

#[test]
fn hsm_provider_config_base_config_mut_updates_timeout() {
    let mut p = HsmProviderConfig::default();
    p.base_config_mut().operation_timeout = Duration::from_secs(99);
    assert_eq!(p.base_config().operation_timeout, Duration::from_secs(99));
}

#[test]
fn load_balancing_and_server_config_serde() {
    use crate::canonical::hsm::config::{LoadBalancingConfig, ServerConfig};
    let lb = LoadBalancingConfig {
        strategy: LoadBalancingStrategy::LeastConnections,
        servers: vec![ServerConfig {
            address: "10.0.0.1".to_string(),
            port: 443,
            weight: 1,
            enabled: true,
        }],
        health_check_interval: 30,
    };
    assert_serde_json_roundtrip(&lb);
}

#[test]
fn dns_http_discovery_is_cloneable() {
    let a = DnsHttpDiscovery::new();
    let b = a.clone();
    assert_eq!(a.provider_name(), b.provider_name());
}

#[test]
fn service_discovery_capability_object_safe() {
    let d: Box<dyn ServiceDiscoveryCapability> = Box::new(DnsHttpDiscovery::new());
    assert_eq!(d.provider_name(), "dns-http-fallback");
}

#[test]
fn addresses_deprecated_aliases_resolve() {
    #[expect(
        deprecated,
        reason = "migration in progress — see CANONICAL_TYPE_MIGRATION_GUIDE"
    )]
    {
        assert_eq!(addresses::DEFAULT_BIND_ADDRESS, addresses::WILDCARD_IPV4);
        assert!(addresses::DEFAULT_METRICS_BIND.contains(':'));
        assert!(addresses::DEFAULT_HEALTH_BIND.contains(':'));
        assert_eq!(addresses::MULTICAST_ADDRESS, "224.0.0.251");
    }
}

#[test]
fn ipc_discovery_public_constants_are_non_empty() {
    assert!(!ipc_discovery::BIOMEOS_RUNTIME_SOCKET_SUBDIR.is_empty());
    assert!(!ipc_discovery::BEARDOG_TCP_DISCOVERY_FILENAME.is_empty());
    assert!(!ipc_discovery::DEFAULT_UPA_REGISTRY_SOCKET_NAME.is_empty());
    assert!(!ipc_discovery::ENV_BIOMEOS_SOCKET_DIR_OVERRIDE.is_empty());
}

#[test]
fn performance_caching_is_production_ready_by_default() {
    let c = CachingConfig::default();
    assert!(c.is_production_ready());
}

#[test]
fn performance_monitoring_thresholds_from_env_runs() {
    let _ = crate::canonical::providers_unified::performance::PerformanceThresholds::from_env();
}

#[test]
fn workflow_retry_total_delay_increases_with_attempts() {
    let c = WorkflowRetryConfig {
        max_attempts: 3,
        initial_delay: Duration::from_millis(10),
        backoff_multiplier: 2.0,
        max_delay: Duration::from_secs(60),
    };
    let t0 = RetryStrategy::total_delay(&c, 0);
    let t2 = RetryStrategy::total_delay(&c, 2);
    assert!(t2 >= t0);
}

#[test]
fn adapter_retry_limit_reached_edge() {
    let r = AdapterRetryConfig {
        max_attempts: 1,
        initial_delay: Duration::from_millis(1),
        max_delay: Duration::from_secs(1),
        backoff_multiplier: 2.0,
        exponential_backoff: true,
        jitter_factor: 0.0,
    };
    assert!(RetryStrategy::is_limit_reached(&r, 1));
}

#[test]
fn unified_cloud_hsm_validate_error_displayed() {
    let cfg = UnifiedCloudHsmConfig {
        enabled: true,
        required_capabilities: None,
    };
    let e = cfg.validate().expect_err("expected validation error");
    let s = format!("{e}");
    assert!(!s.is_empty());
}

#[test]
fn alerting_custom_condition_serde() {
    let mut params = std::collections::HashMap::new();
    params.insert("k".to_string(), serde_json::json!(1));
    let c = AlertCondition::Custom {
        expression: "1 > 0".to_string(),
        parameters: params,
    };
    assert_serde_json_roundtrip(&c);
}

#[test]
fn rate_limiting_config_default_equals_with_defaults() {
    assert_eq!(
        RateLimitingConfig::default().max_requests_per_minute,
        RateLimitingConfig::with_defaults().max_requests_per_minute
    );
}

#[test]
fn canonical_security_new_matches_default() {
    assert_eq!(
        CanonicalSecurityConfig::new().enable_encryption,
        CanonicalSecurityConfig::default().enable_encryption
    );
}
