// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use beardog_config::global::BEARDOG_CONFIG;
use beardog_core::ecosystem::primal_types::{
    AuthRequirements, DiscoveredPrimal, EndpointSecurityConfig, ErrorRateMetrics, LoadMetrics,
    PrimalMetadata, PrimalMetrics, ResponseTimeMetrics, UniversalEndpoint,
};
use beardog_core::ecosystem_integration::PrimalDiscoveryService;
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use beardog_types::canonical::discovery::universal::CollaborationFunction;
use beardog_types::canonical::discovery::{
    AuthenticationMethod, ComputeAbility, NetworkFunction, OrchestrationFeature,
    PerformanceProfile, ServiceEndpoint, StorageCharacteristic, UniversalCapabilityType,
    UniversalServiceDescriptor,
};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;
use tokio::sync::Notify;

fn minimal_discovered_primal(
    id: &str,
    url: &str,
    caps: Vec<ServiceCapabilityType>,
) -> DiscoveredPrimal {
    DiscoveredPrimal {
        primal_id: id.to_string(),
        capabilities: caps,
        endpoint: UniversalEndpoint {
            url: url.to_string(),
            protocols: vec!["https".to_string()],
            auth_requirements: AuthRequirements::default(),
            security_config: EndpointSecurityConfig::default(),
        },
        metadata: PrimalMetadata {
            display_name: None,
            version: "1.0.0".to_string(),
            protocol_versions: vec![],
            security_attestations: vec![],
            custom_fields: HashMap::new(),
            capabilities: vec![],
            dependencies: vec![],
            supported_protocols: vec![],
            health_check_endpoint: "/health".to_string(),
            metrics_endpoint: "/metrics".to_string(),
        },
        discovered_at: std::time::SystemTime::now(),
        metrics: PrimalMetrics {
            response_times: ResponseTimeMetrics::default(),
            availability: 100.0,
            load_metrics: LoadMetrics::default(),
            error_rates: ErrorRateMetrics::default(),
        },
    }
}

#[test]
fn test_parse_endpoint_url_https_with_port_and_path() {
    let (proto, host, port, path) =
        EcosystemDiscoveryAdapter::parse_endpoint_url("https://example.com:9443/api/v1");
    assert_eq!(proto, "https");
    assert_eq!(host, "example.com");
    assert_eq!(port, 9443);
    assert_eq!(path.as_deref(), Some("/api/v1"));
}

#[test]
fn test_parse_endpoint_url_http_default_port_no_path() {
    let (proto, host, port, path) =
        EcosystemDiscoveryAdapter::parse_endpoint_url("http://localhost");
    assert_eq!(proto, "http");
    assert_eq!(host, "localhost");
    assert_eq!(port, BEARDOG_CONFIG.network.ports.api_port);
    assert_eq!(path, None);
}

#[test]
fn test_parse_endpoint_url_fallback_no_scheme() {
    let (proto, host, port, path) = EcosystemDiscoveryAdapter::parse_endpoint_url("192.168.1.10");
    assert_eq!(proto, "http");
    assert_eq!(host, "192.168.1.10");
    assert_eq!(port, BEARDOG_CONFIG.network.ports.api_port);
    assert_eq!(path, None);
}

#[test]
fn test_parse_endpoint_url_unix_scheme() {
    let (proto, host, port, path) = EcosystemDiscoveryAdapter::parse_endpoint_url(
        "unix:///run/user/1000/biomeos/capability_peer.sock",
    );
    assert_eq!(proto, "unix");
    assert_eq!(host, "");
    assert_eq!(port, 0);
    assert_eq!(
        path.as_deref(),
        Some("/run/user/1000/biomeos/capability_peer.sock")
    );
}

#[test]
fn test_parse_endpoint_url_absolute_sock_path() {
    let p = "/tmp/biomeos/discovered-capability.sock";
    let (proto, host, port, path) = EcosystemDiscoveryAdapter::parse_endpoint_url(p);
    assert_eq!(proto, "unix");
    assert_eq!(host, "");
    assert_eq!(port, 0);
    assert_eq!(path.as_deref(), Some(p));
}

#[test]
fn test_parse_endpoint_url_scheme_with_host_port_and_slash_path() {
    let (proto, host, port, path) =
        EcosystemDiscoveryAdapter::parse_endpoint_url("https://api.internal:444/v1");
    assert_eq!(proto, "https");
    assert_eq!(host, "api.internal");
    assert_eq!(port, 444);
    assert_eq!(path.as_deref(), Some("/v1"));
}

#[test]
fn test_primal_has_capability_security_positive() {
    let p = minimal_discovered_primal(
        "p-sec",
        "https://svc.local:443/x",
        vec![ServiceCapabilityType::Security],
    );
    let req = UniversalCapabilityType::Security { services: vec![] };
    assert!(EcosystemDiscoveryAdapter::primal_has_capability(&p, &req));
}

#[test]
fn test_primal_has_capability_security_negative() {
    let p = minimal_discovered_primal(
        "p-store",
        "https://svc.local:443/x",
        vec![ServiceCapabilityType::DataStorage],
    );
    let req = UniversalCapabilityType::Security { services: vec![] };
    assert!(!EcosystemDiscoveryAdapter::primal_has_capability(&p, &req));
}

#[test]
fn test_primal_has_capability_compute_storage_network_orchestration_collab() {
    let c = minimal_discovered_primal(
        "p-c",
        "https://x",
        vec![ServiceCapabilityType::ComputeIntelligence],
    );
    assert!(EcosystemDiscoveryAdapter::primal_has_capability(
        &c,
        &UniversalCapabilityType::Compute {
            abilities: vec![ComputeAbility::DataAnalysis],
        },
    ));

    let s = minimal_discovered_primal("p-s", "https://x", vec![ServiceCapabilityType::DataStorage]);
    assert!(EcosystemDiscoveryAdapter::primal_has_capability(
        &s,
        &UniversalCapabilityType::Storage {
            characteristics: vec![StorageCharacteristic::Persistent],
        },
    ));

    let n = minimal_discovered_primal("p-n", "https://x", vec![ServiceCapabilityType::ServiceMesh]);
    assert!(EcosystemDiscoveryAdapter::primal_has_capability(
        &n,
        &UniversalCapabilityType::Network {
            functions: vec![NetworkFunction::TrafficRouting],
        },
    ));

    let o = minimal_discovered_primal(
        "p-o",
        "https://x",
        vec![ServiceCapabilityType::WorkflowOrchestration],
    );
    assert!(EcosystemDiscoveryAdapter::primal_has_capability(
        &o,
        &UniversalCapabilityType::Orchestration {
            features: vec![OrchestrationFeature::ContainerManagement],
        },
    ));

    let col = minimal_discovered_primal(
        "p-col",
        "https://x",
        vec![ServiceCapabilityType::Authentication],
    );
    assert!(EcosystemDiscoveryAdapter::primal_has_capability(
        &col,
        &UniversalCapabilityType::Collaboration {
            functions: vec![CollaborationFunction::TemplateStorage],
        },
    ));
}

#[tokio::test]
async fn test_discover_by_capability_matches_injected_primal() {
    let adapter = EcosystemDiscoveryAdapter::new().expect("creation failed");
    let primal = minimal_discovered_primal(
        "injected-sec",
        "https://example.com:443/path",
        vec![ServiceCapabilityType::Security],
    );
    adapter.insert_primal_for_test(primal).await;

    let found = adapter
        .discover_by_capability(UniversalCapabilityType::Security { services: vec![] })
        .await
        .expect("discover");

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].service_id, "injected-sec");
    assert_eq!(found[0].endpoint.host, "example.com");
    assert_eq!(found[0].endpoint.port, 443);
}

#[tokio::test]
async fn test_send_request_rejects_http_endpoint() {
    let adapter = EcosystemDiscoveryAdapter::new().expect("creation failed");
    let svc = UniversalServiceDescriptor {
        service_id: "discovered-capability-peer".to_string(),
        capabilities: vec![],
        endpoint: ServiceEndpoint {
            protocol: "https".to_string(),
            host: "api.example".to_string(),
            port: 8443,
            path: Some("/rpc".to_string()),
            parameters: HashMap::new(),
        },
        auth_method: AuthenticationMethod::None,
        performance_profile: PerformanceProfile::default(),
        trust_score: 0.5,
    };
    let err = adapter
        .send_request(&svc, json!({"ping": true}))
        .await
        .expect_err("HTTP endpoint must be rejected for IPC-first adapter");
    assert!(
        err.to_string().contains("IPC-first"),
        "expected IPC refusal, got {err}"
    );
}

#[tokio::test]
async fn test_send_request_tower_atomic_unix_socket() {
    let dir = tempfile::tempdir().expect("tempdir for unix socket");
    let sock_path = dir.path().join("capability_peer.sock");
    let listener = UnixListener::bind(&sock_path).expect("bind mock primal unix listener");
    let ready = Arc::new(Notify::new());

    tokio::spawn({
        let ready_tx = Arc::clone(&ready);
        async move {
            ready_tx.notify_one();
            let (mut stream, _) = listener
                .accept()
                .await
                .expect("mock server accepts connection");
            let mut reader = BufReader::new(&mut stream);
            let mut request = String::new();
            reader
                .read_line(&mut request)
                .await
                .expect("mock server reads JSON-RPC line");

            let response = json!({
                "jsonrpc": "2.0",
                "result": { "status": "ok", "echo": "tower-atomic" },
                "id": 1
            });
            stream
                .write_all(
                    serde_json::to_string(&response)
                        .expect("JSON-RPC response serializes")
                        .as_bytes(),
                )
                .await
                .expect("mock server writes response body");
            stream
                .write_all(b"\n")
                .await
                .expect("mock server writes newline");
        }
    });

    ready.notified().await;

    let adapter = EcosystemDiscoveryAdapter::new().expect("creation failed");
    let path_str = sock_path.to_string_lossy();
    let svc = UniversalServiceDescriptor {
        service_id: "capability_peer".to_string(),
        capabilities: vec![],
        endpoint: ServiceEndpoint {
            protocol: "unix".to_string(),
            host: String::new(),
            port: 0,
            path: Some(path_str.into_owned()),
            parameters: HashMap::new(),
        },
        auth_method: AuthenticationMethod::None,
        performance_profile: PerformanceProfile::default(),
        trust_score: 0.5,
    };

    let body = adapter
        .send_request(
            &svc,
            json!({
                "method": "ecosystem.ping",
                "params": { "probe": true }
            }),
        )
        .await
        .expect("Tower Atomic JSON-RPC over unix socket");

    assert_eq!(body["status"], "ok");
    assert_eq!(body["echo"], "tower-atomic");
}

#[tokio::test]
async fn test_adapter_creation() {
    let adapter = EcosystemDiscoveryAdapter::new();
    assert!(adapter.is_ok(), "Adapter creation should succeed");
}

#[tokio::test]
async fn test_adapter_with_custom_config() {
    let config = EcosystemDiscoveryAdapter::default_config();
    let adapter = EcosystemDiscoveryAdapter::with_config(config);
    assert!(adapter.is_ok(), "Adapter with config should succeed");
}

#[tokio::test]
async fn test_discover_returns_empty_without_announcements() {
    let adapter = EcosystemDiscoveryAdapter::new().expect("creation failed - test cannot proceed");

    let capability = UniversalCapabilityType::Network {
        functions: vec![NetworkFunction::TrafficRouting],
    };

    let primals = adapter
        .discover_by_capability(capability)
        .await
        .expect("discovery failed");

    // Should return empty without announcements (correct - no hardcoded primals)
    assert_eq!(
        primals.len(),
        0,
        "Should not hardcode any primals without announcements"
    );
}

#[tokio::test]
async fn test_trigger_discovery_starts_listener() {
    let adapter = EcosystemDiscoveryAdapter::new().expect("creation failed");
    let result = adapter.trigger_discovery().await;
    assert!(result.is_ok(), "Discovery trigger should succeed");

    // Verify listener was started
    let listener_guard = adapter.listener.read().await;
    assert!(
        listener_guard.is_some(),
        "Listener should be initialized after trigger"
    );
}

#[tokio::test]
async fn test_adapter_clone() {
    let adapter1 = EcosystemDiscoveryAdapter::new().expect("creation failed");
    let adapter2 = adapter1.clone();

    // Both should share the same underlying data (Arc)
    let addr1 = Arc::as_ptr(&adapter1.discovered_primals);
    let addr2 = Arc::as_ptr(&adapter2.discovered_primals);
    assert_eq!(addr1, addr2, "Cloned adapters should share data");
}
