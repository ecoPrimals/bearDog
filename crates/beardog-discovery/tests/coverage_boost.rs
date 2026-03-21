#![allow(missing_docs, clippy::all)]

//! Integration coverage tests for `beardog-discovery`.

use beardog_discovery::announcement::Announcer;
use beardog_discovery::capability_env::{
    discovered_services_from_environment_with, primary_url_to_ipc_socket_path,
};
use beardog_discovery::config::DiscoveryConfig;
use beardog_discovery::discovery::CapabilityDiscovery;
use beardog_discovery::dns_sd::DnsSdDiscovery;
use beardog_discovery::error::DiscoveryError;
use beardog_discovery::mdns::MdnsDiscovery;
use beardog_discovery::service_registry::ServiceRegistryDiscovery;
use beardog_discovery::types::{
    DiscoveredService, HealthStatus, QoSMetrics, QoSWeights, RequiredCapability, ServiceEndpoint,
};
use std::collections::HashMap;
use std::env::VarError;
use std::time::{Duration, SystemTime};

fn sample_endpoint() -> ServiceEndpoint {
    ServiceEndpoint {
        primary_url: "https://localhost:1".to_string(),
        fallback_urls: vec!["http://127.0.0.1:2".to_string()],
        use_tls: true,
        path_prefix: Some("/p".to_string()),
    }
}

#[test]
fn discovery_error_display_all_variants() {
    let cases: Vec<DiscoveryError> = vec![
        DiscoveryError::Config("bad".to_string()),
        DiscoveryError::DiscoveryFailed("d".to_string()),
        DiscoveryError::ServiceNotFound("s".to_string()),
        DiscoveryError::CapabilityNotFound("c".to_string()),
        DiscoveryError::AnnouncementFailed("a".to_string()),
        DiscoveryError::Network("n".to_string()),
        DiscoveryError::Parse("p".to_string()),
        DiscoveryError::Timeout("t".to_string()),
        DiscoveryError::InvalidEndpoint("e".to_string()),
        DiscoveryError::QueryFailed("q".to_string()),
        DiscoveryError::InvalidServiceInfo("i".to_string()),
        DiscoveryError::InitializationFailed("init".to_string()),
        DiscoveryError::SystemError("sys".to_string()),
        DiscoveryError::NotImplemented("ni".to_string()),
        DiscoveryError::BackendUnavailable {
            provider: "mdns".to_string(),
            reason: "down".to_string(),
        },
        DiscoveryError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "io")),
        DiscoveryError::Other(anyhow::anyhow!("any")),
    ];
    for e in cases {
        let s = e.to_string();
        assert!(!s.is_empty());
    }
}

#[test]
fn primary_url_to_ipc_socket_path_variants() {
    assert_eq!(
        primary_url_to_ipc_socket_path("unix:///run/s.sock"),
        "/run/s.sock"
    );
    assert_eq!(
        primary_url_to_ipc_socket_path("unix:/run/s.sock"),
        "/run/s.sock"
    );
    assert_eq!(
        primary_url_to_ipc_socket_path("  /tmp/x  "),
        "/tmp/x".to_string()
    );
}

#[test]
fn discovered_services_from_environment_with_capability_key() {
    let mut m = HashMap::new();
    m.insert(
        "CAPABILITY_ALPHA_BETA_ENDPOINT".to_string(),
        "https://h.example".to_string(),
    );
    let lookup = m.clone();
    let get = move |k: &str| lookup.get(k).cloned().ok_or(VarError::NotPresent);
    let v = discovered_services_from_environment_with("alpha-beta", 60, get, m.into_iter());
    assert_eq!(v.len(), 1);
    assert!(v[0].endpoint.primary_url.contains("example"));
}

#[test]
fn discovered_services_from_environment_with_primal_scan() {
    let mut m = HashMap::new();
    m.insert(
        "PRIMAL_MY_SERVICE_ENDPOINT".to_string(),
        "https://x.test:9".to_string(),
    );
    m.insert(
        "PRIMAL_MY_SERVICE_CAPABILITIES".to_string(),
        "orch,other".to_string(),
    );
    let lookup = m.clone();
    let get = move |k: &str| lookup.get(k).cloned().ok_or(VarError::NotPresent);
    let out = discovered_services_from_environment_with("orch", 120, get, m.into_iter());
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].id, "primal-my_service");
}

#[test]
fn qos_metrics_score_with_extreme_latency() {
    let m = QoSMetrics {
        latency_ms: 500.0,
        throughput_ops_sec: 0.0,
        availability: 0.0,
        reliability: 0.0,
        updated_at: SystemTime::now(),
    };
    let w = QoSWeights {
        latency: 1.0,
        throughput: 0.0,
        availability: 0.0,
        reliability: 0.0,
    };
    let s = m.calculate_score(&w);
    assert!(s.is_finite());
}

#[test]
fn service_endpoint_full_url_no_prefix() {
    let ep = ServiceEndpoint {
        primary_url: "http://127.0.0.1:1".to_string(),
        fallback_urls: vec![],
        use_tls: false,
        path_prefix: None,
    };
    assert_eq!(ep.full_url("/z"), "http://127.0.0.1:1/z");
}

#[tokio::test]
async fn discovery_config_from_file_missing() {
    let p = std::path::PathBuf::from("/nonexistent/beardog-discovery-coverage-boost.toml");
    let e = DiscoveryConfig::from_file(&p).expect_err("missing file");
    match e {
        DiscoveryError::Io(_) => {}
        _ => panic!("expected io: {e:?}"),
    }
}

#[tokio::test]
async fn capability_discovery_from_temp_config_and_cache_hit() {
    let dir = tempfile::tempdir().expect("tmp");
    let path = dir.path().join("c.toml");
    let toml = r#"
[primal_self]
primal_id = "p"
primal_type = "security"
version = "1"
display_name = "P"
self_capabilities = ["a"]

[primal_self.endpoint]
host = "127.0.0.1"
port = 1
scheme = "http"
path_prefix = "/api"

[primal_self.announcement]
enabled = false
methods = []
announcement_interval_secs = 60
ttl_secs = 300

[required_capabilities.x]
required = false
preferred = false
features = []
fallback = "none"

[discovery]
methods = ["environment"]
discovery_timeout_secs = 1
discovery_interval_secs = 300
cache_ttl_secs = 600

[service_selection]
strategy = "qos_based"

[service_selection.qos_weights]
latency = 0.25
throughput = 0.25
availability = 0.25
reliability = 0.25
"#;
    std::fs::write(&path, toml).expect("write");
    let mut env_map = HashMap::new();
    env_map.insert(
        "CAPABILITY_ORCH_ENDPOINT".to_string(),
        "http://127.0.0.1:9".to_string(),
    );
    let env_map_clone = env_map.clone();
    let d = CapabilityDiscovery::from_config(&path)
        .await
        .expect("from_config")
        .with_environment_discovery(move |cap, ttl| {
            discovered_services_from_environment_with(
                cap,
                ttl,
                |k| env_map_clone.get(k).cloned().ok_or(VarError::NotPresent),
                env_map_clone.iter().map(|(a, b)| (a.clone(), b.clone())),
            )
        });
    let a = d.find_by_capability("orch").await.expect("discover");
    assert_eq!(a.len(), 1);
    let b = d.find_by_capability("orch").await.expect("cache");
    assert_eq!(b.len(), 1);
}

#[tokio::test]
async fn announcer_all_methods_and_disabled() {
    let cfg_off: DiscoveryConfig = toml::from_str(
        r#"
[primal_self]
primal_id = "p"
primal_type = "security"
version = "1"
display_name = "P"
self_capabilities = ["x"]

[primal_self.endpoint]
host = "0.0.0.0"
port = 8443
scheme = "https"
path_prefix = "/api"

[primal_self.announcement]
enabled = false
methods = ["mdns", "environment"]
announcement_interval_secs = 1
ttl_secs = 1

[required_capabilities._p]
required = false
preferred = false
features = []
fallback = "none"

[discovery]
methods = ["environment"]
discovery_timeout_secs = 1
discovery_interval_secs = 300
cache_ttl_secs = 600

[service_selection]
strategy = "qos_based"

[service_selection.qos_weights]
latency = 0.25
throughput = 0.25
availability = 0.25
reliability = 0.25
"#,
    )
    .expect("parse");
    let info = cfg_off.primal_info();
    let ann = Announcer::new(cfg_off.primal_self.announcement.clone(), info);
    ann.start().await.expect("noop announce");

    let cfg_on: DiscoveryConfig = toml::from_str(
        r#"
[primal_self]
primal_id = "p"
primal_type = "security"
version = "1"
display_name = "P"
self_capabilities = ["x"]

[primal_self.endpoint]
host = "0.0.0.0"
port = 8443
scheme = "https"
path_prefix = "/api"

[primal_self.announcement]
enabled = true
methods = ["mdns", "environment", "service_registry", "unknown_method"]
announcement_interval_secs = 1
ttl_secs = 1

[required_capabilities._p]
required = false
preferred = false
features = []
fallback = "none"

[discovery]
methods = ["environment"]
discovery_timeout_secs = 1
discovery_interval_secs = 300
cache_ttl_secs = 600

[service_selection]
strategy = "qos_based"

[service_selection.qos_weights]
latency = 0.25
throughput = 0.25
availability = 0.25
reliability = 0.25
"#,
    )
    .expect("parse2");
    let ann2 = Announcer::new(
        cfg_on.primal_self.announcement.clone(),
        cfg_on.primal_info(),
    )
    .with_service_registry_url(Some("http://127.0.0.1:8500".to_string()));
    ann2.start().await.expect("paths");
}

#[tokio::test]
async fn service_registry_discover_and_refresh() {
    let r = ServiceRegistryDiscovery::new().await.expect("new");
    let err = r.discover("any-cap").await.expect_err("no providers");
    assert!(matches!(err, DiscoveryError::BackendUnavailable { .. }));
    let e2 = r.refresh_providers().await;
    assert!(e2.is_err());
    r.clear_cache().await;
}

#[tokio::test]
async fn mdns_short_timeout_and_clear() {
    let d = MdnsDiscovery::with_config(beardog_discovery::mdns::MdnsConfig {
        timeout: Duration::from_millis(1),
        ..Default::default()
    })
    .expect("mdns");
    let v = d.discover(&format!("cap_{}", line!())).await.expect("disc");
    assert!(v.is_empty());
    d.clear_cache().await;
}

#[tokio::test]
async fn dns_sd_short_timeout() {
    let d = DnsSdDiscovery::with_config(beardog_discovery::dns_sd::DnsSdConfig {
        timeout: Duration::from_millis(50),
        ..Default::default()
    })
    .await
    .expect("dns");
    let v = d.discover("noop-cap").await.expect("ok empty");
    assert!(v.is_empty());
    d.clear_cache().await;
}

#[test]
fn required_capability_and_discovered_service_roundtrip() {
    let rc = RequiredCapability {
        capability_type: "t".to_string(),
        required: true,
        min_version: Some("1".to_string()),
        features: vec!["f".to_string()],
        fallback: "x".to_string(),
    };
    let j = serde_json::to_string(&rc).expect("ser");
    let rc2: RequiredCapability = serde_json::from_str(&j).expect("de");
    assert_eq!(rc2.capability_type, "t");

    let ds = DiscoveredService {
        id: "i".to_string(),
        service_type: "st".to_string(),
        display_name: "d".to_string(),
        endpoint: sample_endpoint(),
        capabilities: vec![],
        qos: QoSMetrics::default(),
        health: HealthStatus::Degraded,
        discovered_at: SystemTime::now(),
        ttl_secs: 1,
        discovery_method: "test".to_string(),
        metadata: HashMap::new(),
    };
    let j2 = serde_json::to_string(&ds).expect("ser");
    let ds2: DiscoveredService = serde_json::from_str(&j2).expect("de");
    assert_eq!(ds2.id, "i");
}
