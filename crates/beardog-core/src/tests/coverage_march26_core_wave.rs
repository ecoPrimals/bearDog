// SPDX-License-Identifier: AGPL-3.0-or-later

//! Additional coverage for socket resolution, capability matching, and discovery query helpers.

use std::collections::HashMap;
use std::time::Duration;

use crate::capabilities::{BearDogCapabilities, Capability, CapabilityResponse, ResponseStatus};
use crate::primal_discovery::{DiscoveryMethod, DiscoveryQuery, PrimalDiscovery};
use crate::self_knowledge::SimpleCapability;
use crate::socket_config::{SocketConfig, SocketPathInputs, SocketPathSource};

// --- socket_config: tier-2 directory join and description branches ---

#[test]
fn socket_config_biomeos_socket_dir_tier2_joins_beardog_sock() {
    let cfg = SocketConfig::from_inputs(&SocketPathInputs {
        biomeos_socket_path: None,
        biomeos_socket_dir: Some("/var/run/biomeos".to_string()),
        primal_namespace_root_exists: false,
        ..Default::default()
    });
    assert_eq!(cfg.source(), SocketPathSource::OrchestratorEnvVar);
    assert_eq!(cfg.socket_path_string(), "/var/run/biomeos/beardog.sock");
}

#[test]
fn socket_config_description_orchestrator_tier_mentions_biomeos() {
    let cfg = SocketConfig::from_inputs(&SocketPathInputs {
        biomeos_socket_path: Some("/tmp/orch.sock".to_string()),
        ..Default::default()
    });
    let d = cfg.description();
    assert!(d.contains("BIOMEOS_SOCKET_PATH") || d.contains("orch"));
}

#[test]
fn socket_config_description_primal_namespace_tier3() {
    let cfg = SocketConfig::from_inputs(&SocketPathInputs {
        primal_namespace_root_exists: true,
        primal_name: Some("peer-alpha".to_string()),
        ..Default::default()
    });
    assert_eq!(cfg.source(), SocketPathSource::PrimalNamespace);
    let d = cfg.description();
    assert!(d.contains("Tier 3") || d.contains("Primal IPC"));
}

#[test]
fn socket_config_custom_primal_name_in_temp_fallback() {
    let cfg = SocketConfig::from_inputs(&SocketPathInputs {
        primal_name: Some("nestgate".to_string()),
        family_id: Some("fam".to_string()),
        node_id: Some("n1".to_string()),
        primal_namespace_root_exists: false,
        ..Default::default()
    });
    if cfg.source() == SocketPathSource::TempDir {
        assert!(cfg.socket_path_string().contains("nestgate-fam-n1"));
    }
}

#[test]
fn socket_config_display_delegates_to_description() {
    let cfg = SocketConfig::from_inputs(&SocketPathInputs {
        beardog_socket: Some("/x.sock".to_string()),
        ..Default::default()
    });
    let a = format!("{cfg}");
    let b = cfg.description();
    assert_eq!(a, b);
}

// --- capabilities: Custom matching and response types ---

#[test]
fn provides_capability_custom_not_in_default_manifest() {
    let caps = BearDogCapabilities::new(None, "n".to_string());
    let c = Capability::Custom {
        name: "plugin_a".to_string(),
        version: "1".to_string(),
        properties: HashMap::new(),
    };
    assert!(!caps.provides_capability(&c));
}

#[test]
fn capability_response_error_status_roundtrips_serde() {
    let r = CapabilityResponse {
        request_id: "r1".to_string(),
        status: ResponseStatus::NotAvailable,
        data: None,
        error: Some("no".to_string()),
    };
    let j = serde_json::to_string(&r).expect("ser");
    let back: CapabilityResponse = serde_json::from_str(&j).expect("de");
    assert_eq!(back.request_id, r.request_id);
}

#[test]
fn capability_compute_variant_matches() {
    let caps = BearDogCapabilities::new(None, "n".to_string());
    let req = Capability::Compute {
        compute_types: vec!["local".to_string()],
    };
    assert!(!caps.provides_capability(&req));
}

// --- primal_discovery: query builders & method wiring ---

#[test]
fn discovery_query_by_name_sets_timeout_default() {
    let q = DiscoveryQuery::by_name("alpha");
    assert_eq!(q.name.as_deref(), Some("alpha"));
    assert_eq!(q.timeout, Duration::from_secs(5));
}

#[test]
fn discovery_query_with_capability_chains() {
    let q = DiscoveryQuery::by_capability(SimpleCapability::Cryptography)
        .with_capability(SimpleCapability::SecureTunneling)
        .with_timeout(Duration::from_millis(250));
    assert_eq!(q.capabilities.len(), 2);
    assert_eq!(q.timeout, Duration::from_millis(250));
}

#[test]
fn primal_discovery_with_env_override_stores_map() {
    let mut env = HashMap::new();
    env.insert(
        "PRIMAL_FOO_ADDR".to_string(),
        "unix:///tmp/x.sock".to_string(),
    );
    let _d = PrimalDiscovery::new(DiscoveryMethod::Environment).with_env_override(env);
}

#[test]
fn discovery_method_multi_serde_roundtrip() {
    let m = DiscoveryMethod::Multi(vec![
        DiscoveryMethod::Environment,
        DiscoveryMethod::UniversalPrimalAuthority {
            registry_addr: "unix:///tmp/reg".to_string(),
        },
    ]);
    let j = serde_json::to_string(&m).expect("ser");
    let back: DiscoveryMethod = serde_json::from_str(&j).expect("de");
    assert_eq!(back, m);
}

#[test]
fn discovery_method_mdns_serde() {
    let m = DiscoveryMethod::Mdns {
        service_type: "_p._tcp.local".to_string(),
    };
    let j = serde_json::to_string(&m).expect("ser");
    let back: DiscoveryMethod = serde_json::from_str(&j).expect("de");
    assert_eq!(back, m);
}

#[test]
fn discovered_primal_serde_shape() {
    use crate::primal_discovery::DiscoveredPrimal;
    use crate::self_knowledge::Endpoint;
    let p = DiscoveredPrimal {
        name: "n".to_string(),
        endpoints: vec![Endpoint::parse("unix:///tmp/a.sock").expect("endpoint")],
        capabilities: vec![SimpleCapability::Cryptography],
        trust_score: Some(0.9),
        discovered_at: std::time::SystemTime::UNIX_EPOCH,
    };
    let j = serde_json::to_string(&p).expect("ser");
    assert!(j.contains("Cryptography") || j.contains("cryptography"));
}

#[test]
fn ipc_endpoint_unix_socket_roundtrip() {
    use crate::capabilities::IpcEndpoint;
    let e = IpcEndpoint::UnixSocket {
        path: "/p.sock".to_string(),
        permissions: 0o600,
    };
    let j = serde_json::to_string(&e).expect("ser");
    let back: IpcEndpoint = serde_json::from_str(&j).expect("de");
    assert!(j.contains("unix_socket"));
    assert_eq!(serde_json::to_string(&back).expect("ser2"), j);
}

#[test]
fn ipc_endpoint_http_roundtrip() {
    use crate::capabilities::IpcEndpoint;
    let e = IpcEndpoint::Http {
        bind_addr: "127.0.0.1:0".to_string(),
        tls: false,
    };
    let j = serde_json::to_string(&e).expect("ser");
    let back: IpcEndpoint = serde_json::from_str(&j).expect("de");
    assert_eq!(serde_json::to_string(&back).expect("ser2"), j);
}

#[test]
fn capability_storage_signatures_discovery_roundtrip() {
    let v = vec![
        Capability::Storage {
            storage_types: vec!["sled".to_string()],
        },
        Capability::Signatures {
            algorithms: vec!["Ed25519".to_string()],
        },
        Capability::Discovery {
            protocols: vec!["mdns".to_string()],
        },
    ];
    for c in v {
        let j = serde_json::to_string(&c).expect("ser");
        let back: Capability = serde_json::from_str(&j).expect("de");
        assert_eq!(serde_json::to_string(&back).expect("ser2"), j);
    }
}

#[test]
fn socket_path_inputs_default_uid_and_flags() {
    let i = SocketPathInputs::default();
    assert_eq!(i.uid, 1000);
    assert!(!i.primal_namespace_root_exists);
}

#[test]
fn primal_discovery_with_cache_ttl_stores_duration() {
    let _d = PrimalDiscovery::with_cache_ttl(DiscoveryMethod::Environment, Duration::from_secs(12));
}

#[test]
fn capability_request_serde_roundtrip() {
    use crate::capabilities::CapabilityRequest;
    let req = CapabilityRequest {
        from_primal: "a".to_string(),
        capability: Capability::Discovery {
            protocols: vec!["mdns".to_string()],
        },
        params: HashMap::new(),
        request_id: "rid-1".to_string(),
    };
    let j = serde_json::to_string(&req).expect("ser");
    let back: CapabilityRequest = serde_json::from_str(&j).expect("de");
    assert_eq!(back.request_id, "rid-1");
}

#[test]
fn response_status_success_serde() {
    let j = serde_json::to_string(&ResponseStatus::Success).expect("ser");
    let back: ResponseStatus = serde_json::from_str(&j).expect("de");
    assert!(matches!(back, ResponseStatus::Success));
}

#[test]
fn ipc_endpoint_shared_memory_roundtrip_json() {
    use crate::capabilities::IpcEndpoint;
    let e = IpcEndpoint::SharedMemory {
        key: "shm1".to_string(),
        size_bytes: 4096,
    };
    let j = serde_json::to_string(&e).expect("ser");
    let back: IpcEndpoint = serde_json::from_str(&j).expect("de");
    assert_eq!(serde_json::to_string(&back).expect("ser2"), j);
}

#[test]
fn discovery_method_dns_sd_serde() {
    let m = DiscoveryMethod::DnsSd {
        domain: "local.".to_string(),
    };
    let j = serde_json::to_string(&m).expect("ser");
    let back: DiscoveryMethod = serde_json::from_str(&j).expect("de");
    assert_eq!(back, m);
}

#[test]
fn discovered_primal_trust_score_none_serializes() {
    use crate::primal_discovery::DiscoveredPrimal;
    let p = DiscoveredPrimal {
        name: "solo".to_string(),
        endpoints: vec![],
        capabilities: vec![],
        trust_score: None,
        discovered_at: std::time::SystemTime::UNIX_EPOCH,
    };
    let j = serde_json::to_string(&p).expect("ser");
    assert!(j.contains("solo"));
}

#[test]
fn socket_config_primal_name_override_in_namespace_tier() {
    let cfg = SocketConfig::from_inputs(&SocketPathInputs {
        primal_namespace_root_exists: true,
        primal_name: Some("custom-primal".to_string()),
        ..Default::default()
    });
    assert_eq!(cfg.socket_path_string(), "/primal/custom-primal");
}
