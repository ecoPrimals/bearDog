// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use beardog_types::constants::domains::network::ipc_discovery as ipc;
use std::sync::{Mutex, OnceLock};

static DISCOVERY_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn discovery_env_lock() -> std::sync::MutexGuard<'static, ()> {
    DISCOVERY_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("discovery env lock")
}

#[test]
fn test_discovery_query_by_name() {
    let query = DiscoveryQuery::by_name("PeerAlpha");
    assert_eq!(query.name.as_deref(), Some("PeerAlpha"));
    assert!(query.capabilities.is_empty());
}

#[test]
fn test_discovery_query_by_capability() {
    let query = DiscoveryQuery::by_capability(SimpleCapability::SecureTunneling);
    assert!(query.name.is_none());
    assert_eq!(query.capabilities.len(), 1);
}

#[test]
fn test_discovery_query_builder() {
    let query = DiscoveryQuery::by_name("PeerAlpha")
        .with_capability(SimpleCapability::Cryptography)
        .with_timeout(Duration::from_secs(10));

    assert_eq!(query.name.as_deref(), Some("PeerAlpha"));
    assert_eq!(query.capabilities.len(), 1);
    assert_eq!(query.timeout, Duration::from_secs(10));
}

#[tokio::test]
async fn test_discover_from_env_specific_primal() {
    let mut discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);

    let mut env_vars = HashMap::new();
    let dir = tempfile::tempdir().unwrap();
    let sock = dir.path().join("peer_alpha.sock");
    std::fs::File::create(&sock).unwrap();
    env_vars.insert(
        "PRIMAL_PEERALPHA_ADDR".to_string(),
        format!("unix://{}", sock.display()),
    );

    let query = DiscoveryQuery::by_name("PeerAlpha");
    let primals = discovery.discover_with_env(query, env_vars).await.unwrap();

    assert_eq!(
        primals.len(),
        1,
        "Expected 1 primal, got {}. Primals: {:?}",
        primals.len(),
        primals
    );
    assert_eq!(primals[0].name, "PeerAlpha");
    assert_eq!(primals[0].endpoints.len(), 1);
}

#[tokio::test]
async fn test_discover_from_env_scan_all() {
    let mut discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);

    let mut env_vars = HashMap::new();
    let dir = tempfile::tempdir().unwrap();
    let peer_alpha_sock = dir.path().join("peer_alpha.sock");
    let beardog_sock = dir.path().join("beardog.sock");
    std::fs::File::create(&peer_alpha_sock).unwrap();
    std::fs::File::create(&beardog_sock).unwrap();
    env_vars.insert(
        "PRIMAL_PEERALPHA_ADDR".to_string(),
        format!("unix://{}", peer_alpha_sock.display()),
    );
    env_vars.insert(
        "PRIMAL_BEARDOG_ADDR".to_string(),
        format!("unix://{}", beardog_sock.display()),
    );

    let query = DiscoveryQuery {
        name: None,
        capabilities: Vec::new(),
        timeout: std::time::Duration::from_secs(5),
    };
    let primals = discovery.discover_with_env(query, env_vars).await.unwrap();

    assert!(primals.len() >= 2);
}

#[test]
fn test_discovery_method_detection_env() {
    let discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);
    assert!(matches!(discovery.method, DiscoveryMethod::Environment));
}

#[test]
fn test_discovery_method_detection_upa() {
    let discovery = PrimalDiscovery::new(DiscoveryMethod::UniversalPrimalAuthority {
        registry_addr: ipc::default_upa_registry_unix_uri(),
    });
    assert!(matches!(
        discovery.method,
        DiscoveryMethod::UniversalPrimalAuthority { .. }
    ));
}

#[test]
fn test_discovery_method_detection_mdns() {
    let discovery = PrimalDiscovery::new(DiscoveryMethod::Mdns {
        service_type: "_ecoprimal._tcp".to_string(),
    });
    assert!(matches!(discovery.method, DiscoveryMethod::Mdns { .. }));
}

#[test]
fn test_discovery_method_detection_multi_default() {
    let discovery =
        PrimalDiscovery::new(DiscoveryMethod::Multi(vec![DiscoveryMethod::Environment]));
    assert!(matches!(discovery.method, DiscoveryMethod::Multi(_)));
}

#[tokio::test]
async fn test_discovered_primal_trust_score() {
    let mut discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);

    let mut env_vars = HashMap::new();
    let dir = tempfile::tempdir().unwrap();
    let sock = dir.path().join("trusted.sock");
    std::fs::File::create(&sock).unwrap();
    env_vars.insert(
        "PRIMAL_TRUSTED_ADDR".to_string(),
        format!("unix://{}", sock.display()),
    );

    let query = DiscoveryQuery::by_name("Trusted");
    let primals = discovery.discover_with_env(query, env_vars).await.unwrap();

    assert_eq!(primals.len(), 1);
    assert_eq!(primals[0].trust_score, Some(1.0));
}

#[tokio::test]
async fn test_discover_biomeos_runtime_socket_scan() {
    let mut discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);
    let base = tempfile::tempdir().unwrap();
    let biomeos = base.path().join(ipc::BIOMEOS_RUNTIME_SOCKET_SUBDIR);
    std::fs::create_dir_all(&biomeos).unwrap();
    let sock = biomeos.join("orchid.sock");
    std::fs::File::create(&sock).unwrap();

    let mut env_vars = HashMap::new();
    env_vars.insert(
        "XDG_RUNTIME_DIR".to_string(),
        base.path().to_string_lossy().into_owned(),
    );
    env_vars.insert(
        "PRIMAL_ORCHID_CAPABILITIES".to_string(),
        "Discovery".to_string(),
    );

    let query = DiscoveryQuery::by_capability(SimpleCapability::Discovery);
    let primals = discovery.discover_with_env(query, env_vars).await.unwrap();
    assert!(
        primals.iter().any(|p| p.name == "orchid"),
        "expected orchid from socket scan, got {primals:?}"
    );
}

#[test]
fn test_parse_capabilities_str_accepts_all_variants_and_unknown() {
    let s =
        "SecureTunneling, GeneticLineage, Cryptography, HsmIntegration, Discovery, UnknownThing";
    let caps = PrimalDiscovery::parse_capabilities_str(s, "PRIMAL_TEST_CAPABILITIES");
    assert_eq!(caps.len(), 5);
    assert!(caps.contains(&SimpleCapability::SecureTunneling));
    assert!(caps.contains(&SimpleCapability::Discovery));
}

#[tokio::test]
async fn test_discover_mdns_and_dns_sd_smoke() {
    let mut mdns = PrimalDiscovery::new(DiscoveryMethod::Mdns {
        service_type: "_ecoprimal._tcp".to_string(),
    });
    let q = DiscoveryQuery::by_capability(SimpleCapability::Cryptography);
    let r = mdns.discover(q).await.expect("mdns");
    assert!(r.is_empty());

    let mut dns_sd = PrimalDiscovery::new(DiscoveryMethod::DnsSd {
        domain: "local.".to_string(),
    });
    let q2 = DiscoveryQuery::by_name("nope");
    let r2 = dns_sd.discover(q2).await.expect("dns-sd");
    assert!(r2.is_empty());
}

#[tokio::test]
async fn test_discover_multi_runs_environment_and_dedupes() {
    let mut discovery = PrimalDiscovery::new(DiscoveryMethod::Multi(vec![
        DiscoveryMethod::Environment,
        DiscoveryMethod::Environment,
    ]));
    let dir = tempfile::tempdir().unwrap();
    let biomeos = dir.path().join(ipc::BIOMEOS_RUNTIME_SOCKET_SUBDIR);
    std::fs::create_dir_all(&biomeos).unwrap();
    let sock = biomeos.join("dedupe.sock");
    std::fs::File::create(&sock).unwrap();
    let mut env_vars = HashMap::new();
    env_vars.insert(
        "XDG_RUNTIME_DIR".to_string(),
        dir.path().to_string_lossy().into_owned(),
    );
    env_vars.insert(
        "PRIMAL_DEDUPE_ADDR".to_string(),
        format!("unix://{}", sock.display()),
    );
    let query = DiscoveryQuery {
        name: None,
        capabilities: Vec::new(),
        timeout: Duration::from_secs(2),
    };
    let primals = discovery
        .discover_with_env(query, env_vars)
        .await
        .expect("multi env");
    assert_eq!(
        primals.len(),
        1,
        "duplicate Environment arms should dedupe by name: {primals:?}"
    );
}

#[tokio::test]
async fn test_discover_multi_async_with_upa_fails_gracefully() {
    let mut discovery = PrimalDiscovery::new(DiscoveryMethod::Multi(vec![
        DiscoveryMethod::Environment,
        DiscoveryMethod::UniversalPrimalAuthority {
            registry_addr: "/nonexistent/beardog_upa.sock".to_string(),
        },
    ]));
    let query = DiscoveryQuery::by_capability(SimpleCapability::Cryptography);
    let r = discovery.discover(query).await.expect("multi async");
    assert!(r.is_empty());
}

#[test]
fn test_discovery_method_explicit_constructors() {
    let e = PrimalDiscovery::new(DiscoveryMethod::Environment);
    assert!(matches!(e.method, DiscoveryMethod::Environment));

    let d = PrimalDiscovery::new(DiscoveryMethod::DnsSd {
        domain: "local.".to_string(),
    });
    assert!(matches!(d.method, DiscoveryMethod::DnsSd { .. }));
}

#[test]
fn discovery_query_by_name_sets_timeout() {
    let q = DiscoveryQuery::by_name("X").with_timeout(Duration::from_secs(3));
    assert_eq!(q.timeout, Duration::from_secs(3));
}

#[test]
fn from_env_rejects_unknown_primal_discovery_method() {
    let _g = discovery_env_lock();
    beardog_errors::process_env::set_var("PRIMAL_DISCOVERY_METHOD", "not-a-valid-method-xyz");
    let res = PrimalDiscovery::from_env();
    assert!(res.is_err(), "expected unknown discovery method error");
    let err = res.err().expect("err");
    assert!(
        err.to_string().contains("Unknown discovery method"),
        "unexpected: {err}"
    );
    beardog_errors::process_env::remove_var("PRIMAL_DISCOVERY_METHOD");
}

#[tokio::test]
async fn discover_from_upa_parses_jsonrpc_result_array() {
    let dir = tempfile::tempdir().unwrap();
    let sock_path = dir.path().join("upa_registry.sock");
    let sock_path_str = sock_path.to_string_lossy().to_string();

    let listener = tokio::net::UnixListener::bind(&sock_path).expect("bind unix listener");
    let server = tokio::spawn(async move {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let (mut stream, _) = listener.accept().await.expect("accept");
        let mut buf = vec![0u8; 16384];
        let _n = stream.read(&mut buf).await.expect("read req");
        let p = DiscoveredPrimal {
            name: "from-upa".to_string(),
            endpoints: vec![],
            capabilities: vec![],
            trust_score: Some(0.42),
            discovered_at: std::time::SystemTime::UNIX_EPOCH,
        };
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": [ serde_json::to_value(&p).expect("serialize primal") ]
        });
        let mut s = body.to_string();
        s.push('\n');
        stream.write_all(s.as_bytes()).await.expect("write resp");
    });

    let mut discovery = PrimalDiscovery::new(DiscoveryMethod::UniversalPrimalAuthority {
        registry_addr: sock_path_str,
    });
    let q = DiscoveryQuery::by_capability(SimpleCapability::Cryptography);
    let primals = discovery.discover(q).await.expect("upa discover");
    server.await.expect("server join");
    assert_eq!(primals.len(), 1);
    assert_eq!(primals[0].name, "from-upa");
    assert_eq!(primals[0].trust_score, Some(0.42));
}

#[tokio::test]
async fn discover_by_name_missing_primal_returns_empty_without_error() {
    let mut discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);
    let mut env = HashMap::new();
    env.insert(
        "PRIMAL_OTHER_ADDR".to_string(),
        "unix:///tmp/nope.sock".to_string(),
    );
    let q = DiscoveryQuery::by_name("missing");
    let primals = discovery.discover_with_env(q, env).await.expect("discover");
    assert!(primals.is_empty());
}
