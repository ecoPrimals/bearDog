// SPDX-License-Identifier: AGPL-3.0-only

use super::*;
use beardog_types::constants::domains::network::ipc_discovery as ipc;

#[test]
fn test_discovery_query_by_name() {
    let query = DiscoveryQuery::by_name("Songbird");
    assert_eq!(query.name.as_deref(), Some("Songbird"));
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
    let query = DiscoveryQuery::by_name("Songbird")
        .with_capability(SimpleCapability::Cryptography)
        .with_timeout(Duration::from_secs(10));

    assert_eq!(query.name.as_deref(), Some("Songbird"));
    assert_eq!(query.capabilities.len(), 1);
    assert_eq!(query.timeout, Duration::from_secs(10));
}

#[tokio::test]
async fn test_discover_from_env_specific_primal() {
    let mut discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);

    let mut env_vars = HashMap::new();
    let dir = tempfile::tempdir().unwrap();
    let sock = dir.path().join("songbird.sock");
    std::fs::File::create(&sock).unwrap();
    env_vars.insert(
        "PRIMAL_SONGBIRD_ADDR".to_string(),
        format!("unix://{}", sock.display()),
    );

    let query = DiscoveryQuery::by_name("Songbird");
    let primals = discovery.discover_with_env(query, env_vars).await.unwrap();

    assert_eq!(
        primals.len(),
        1,
        "Expected 1 primal, got {}. Primals: {:?}",
        primals.len(),
        primals
    );
    assert_eq!(primals[0].name, "Songbird");
    assert_eq!(primals[0].endpoints.len(), 1);
}

#[tokio::test]
async fn test_discover_from_env_scan_all() {
    let mut discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);

    let mut env_vars = HashMap::new();
    let dir = tempfile::tempdir().unwrap();
    let songbird_sock = dir.path().join("songbird.sock");
    let beardog_sock = dir.path().join("beardog.sock");
    std::fs::File::create(&songbird_sock).unwrap();
    std::fs::File::create(&beardog_sock).unwrap();
    env_vars.insert(
        "PRIMAL_SONGBIRD_ADDR".to_string(),
        format!("unix://{}", songbird_sock.display()),
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
        "expected orchid from socket scan, got {:?}",
        primals
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
        "duplicate Environment arms should dedupe by name: {:?}",
        primals
    );
}

#[tokio::test]
#[serial_test::serial]
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
#[serial_test::serial]
fn test_from_env_discovery_method_variants() {
    beardog_errors::process_env::set_var("PRIMAL_DISCOVERY_METHOD", "environment");
    let d = PrimalDiscovery::from_env().expect("env");
    assert!(matches!(d.method, DiscoveryMethod::Environment));
    beardog_errors::process_env::remove_var("PRIMAL_DISCOVERY_METHOD");

    beardog_errors::process_env::set_var("PRIMAL_DISCOVERY_METHOD", "dns-sd");
    let d2 = PrimalDiscovery::from_env().expect("dns-sd");
    assert!(matches!(d2.method, DiscoveryMethod::DnsSd { .. }));
    beardog_errors::process_env::remove_var("PRIMAL_DISCOVERY_METHOD");

    beardog_errors::process_env::set_var("PRIMAL_DISCOVERY_METHOD", "bogus-method");
    assert!(PrimalDiscovery::from_env().is_err());
    beardog_errors::process_env::remove_var("PRIMAL_DISCOVERY_METHOD");
}
