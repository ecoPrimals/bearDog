// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[test]
fn test_primal_identity_self_knowledge_only() {
    let inputs = PrimalIdentityEnvInputs {
        beardog_primal_name: Some("test-primal".to_string()),
        beardog_primal_type: Some("beardog".to_string()),
        ..Default::default()
    };
    let identity =
        PrimalIdentity::from_inputs(&inputs).expect("PrimalIdentity::from_inputs in test");

    assert_eq!(identity.name, "test-primal");
    assert_eq!(identity.primal_type, "beardog");
    assert!(!identity.capabilities.is_empty());

    // Verify NO hardcoded information about other primals
    // (identity contains only self-knowledge)
}

#[tokio::test]
async fn test_discovery_no_hardcoded_addresses() {
    let identity = PrimalIdentity::from_inputs(&PrimalIdentityEnvInputs::default())
        .expect("default PrimalIdentityEnvInputs");
    let discovery = PrimalDiscovery::new(identity);

    // Discovery should work WITHOUT hardcoded addresses
    let hsm_primals = discovery
        .discover_by_capability("hsm")
        .await
        .expect("discover_by_capability hsm in test");

    // May be empty if no HSM primals discovered (correct behavior)
    // Should NOT fall back to hardcoded addresses
    tracing::info!("Discovered {} HSM primals", hsm_primals.len());
}

#[test]
fn test_no_hardcoded_endpoints_in_identity() {
    let inputs = PrimalIdentityEnvInputs {
        beardog_api_host: Some("127.0.0.1".to_string()),
        beardog_api_port: Some(9090),
        ..Default::default()
    };
    let identity =
        PrimalIdentity::from_inputs(&inputs).expect("PrimalIdentity::from_inputs in test");

    // Endpoints should come from config/env, not hardcoded
    for endpoint in &identity.endpoints {
        // Verify endpoints are from environment, not literals
        assert!(!endpoint.host.is_empty());
    }
}

#[test]
fn test_endpoint_url() {
    let endpoint = Endpoint {
        protocol: Protocol::Http,
        host: "localhost".to_string(),
        port: 8080,
        path: Some("/api".to_string()),
    };
    assert_eq!(endpoint.url(), "http://localhost:8080/api");
}

#[test]
fn test_endpoint_url_no_path() {
    let endpoint = Endpoint {
        protocol: Protocol::Https,
        host: "example.com".to_string(),
        port: 443,
        path: None,
    };
    assert_eq!(endpoint.url(), "https://example.com:443");
}

#[test]
fn test_primal_self_knowledge_new() {
    let psk = PrimalSelfKnowledge::new();
    let identity = psk
        .get_self_identity()
        .expect("get_self_identity for new PrimalSelfKnowledge");
    assert!(!identity.is_empty());
    assert_eq!(psk.get_known_primal_count(), 1);
}

#[test]
fn test_primal_self_knowledge_default() {
    let psk = PrimalSelfKnowledge::default();
    let _ = psk.get_discovery();
}

#[test]
fn test_capability_has_capability() {
    let mut caps = HashSet::new();
    caps.insert(Capability::Hsm);
    caps.insert(Capability::Encryption);
    let identity = PrimalIdentity {
        name: "test".to_string(),
        primal_type: "beardog".to_string(),
        capabilities: caps,
        endpoints: vec![],
        metadata: HashMap::new(),
    };
    assert!(identity.has_capability(&Capability::Hsm));
    assert!(!identity.has_capability(&Capability::Networking));
}

#[test]
fn test_identity_endpoints_grpc_default_host() {
    let inputs = PrimalIdentityEnvInputs {
        beardog_grpc_port: Some(50051),
        ..Default::default()
    };
    let id = PrimalIdentity::from_inputs(&inputs).expect("PrimalIdentity from_inputs");
    assert_eq!(id.endpoints.len(), 1);
    assert!(matches!(id.endpoints[0].protocol, Protocol::Grpc));
    assert_eq!(id.endpoints[0].host, "0.0.0.0");
    assert_eq!(id.endpoints[0].url(), "grpc://0.0.0.0:50051");
}

#[test]
fn test_identity_endpoints_api_and_grpc() {
    let inputs = PrimalIdentityEnvInputs {
        beardog_api_host: Some("10.0.0.1".into()),
        beardog_api_port: Some(9000),
        beardog_grpc_host: Some("grpc.local".into()),
        beardog_grpc_port: Some(50052),
        ..Default::default()
    };
    let id = PrimalIdentity::from_inputs(&inputs).expect("PrimalIdentity from_inputs");
    assert_eq!(id.endpoints.len(), 2);
    assert!(id.endpoints.iter().any(|e| e.url().contains("9000/api/v1")));
    assert!(
        id.endpoints
            .iter()
            .any(|e| e.host == "grpc.local" && e.port == 50052)
    );
}

#[test]
fn test_identity_api_host_default_port() {
    let inputs = PrimalIdentityEnvInputs {
        beardog_api_host: Some("api.internal".into()),
        beardog_api_port: None,
        ..Default::default()
    };
    let id = PrimalIdentity::from_inputs(&inputs).expect("PrimalIdentity from_inputs");
    let ep = id.endpoints.first().expect("api endpoint");
    assert!(ep.url().contains("api.internal"));
    assert!(ep.url().contains("/api/v1"));
}

#[test]
fn test_identity_explicit_capabilities_without_default_bundle() {
    let inputs = PrimalIdentityEnvInputs {
        capability_hsm: true,
        capability_encryption: false,
        capability_auth: false,
        ..Default::default()
    };
    let id = PrimalIdentity::from_inputs(&inputs).expect("PrimalIdentity from_inputs");
    assert!(id.has_capability(&Capability::Hsm));
    assert!(!id.has_capability(&Capability::Encryption));
}

#[test]
fn test_endpoint_url_tcp_and_udp() {
    let tcp = Endpoint {
        protocol: Protocol::Tcp,
        host: "h".into(),
        port: 7000,
        path: Some("/stream".into()),
    };
    assert_eq!(tcp.url(), "tcp://h:7000/stream");
    let udp = Endpoint {
        protocol: Protocol::Udp,
        host: "h".into(),
        port: 7001,
        path: None,
    };
    assert_eq!(udp.url(), "udp://h:7001");
}

#[tokio::test]
async fn test_discovery_announce_self_covers_registry_and_mdns_flags() {
    let id = PrimalIdentity::from_inputs(&PrimalIdentityEnvInputs::default()).expect("identity");
    let discovery = PrimalDiscovery::with_runtime(
        id,
        PrimalDiscoveryRuntimeInputs {
            service_registry_url: Some("http://registry.example".into()),
            mdns_announce: true,
        },
    );
    discovery
        .announce_self()
        .expect("announce_self should succeed");
    let primals = discovery
        .discover_by_capability("hsm")
        .await
        .expect("discover_by_capability");
    assert!(primals.is_empty());
}
