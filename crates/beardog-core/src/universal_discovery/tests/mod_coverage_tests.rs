// SPDX-License-Identifier: AGPL-3.0-only

//! Exercises `universal_discovery/mod.rs` (orchestrator, protocol factory, minimal handler).

use crate::universal_discovery::{
    DiscoveryEvent, DiscoveryProtocol, MinimalProtocolHandler, ProtocolHandler,
    UniversalDiscoveryConfig, UniversalServiceDiscovery,
};
use beardog_types::canonical::providers_unified::traits::ServiceInfo;
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

#[test]
fn discovery_protocol_hash_smoke() {
    let mut h = DefaultHasher::new();
    DiscoveryProtocol::Http {
        endpoint: "http://127.0.0.1:1".to_string(),
        headers: HashMap::new(),
    }
    .hash(&mut h);
    let a = h.finish();

    let mut h = DefaultHasher::new();
    DiscoveryProtocol::Http {
        endpoint: "http://127.0.0.1:2".to_string(),
        headers: HashMap::new(),
    }
    .hash(&mut h);
    let b = h.finish();
    assert_ne!(a, b);
}

#[test]
fn discovery_event_roundtrip_debug() {
    let e = DiscoveryEvent::ServiceRegistered {
        service_id: "s1".to_string(),
        service_name: "svc".to_string(),
    };
    let s = format!("{e:?}");
    assert!(s.contains("ServiceRegistered"));
}

#[tokio::test]
async fn universal_service_discovery_full_smoke() {
    let mut config = UniversalDiscoveryConfig::default();
    config.service_id = "coverage-test".to_string();
    // Omit mDNS here: with `mdns` feature the handler uses `block_on` and panics inside `#[tokio::test]`.
    config.enabled_protocols = vec![
        DiscoveryProtocol::Http {
            endpoint: "http://127.0.0.1:18501/v1/catalog/services".to_string(),
            headers: HashMap::new(),
        },
        DiscoveryProtocol::Dns {
            domain: "local.".to_string(),
            servers: vec!["127.0.0.1:53".to_string()],
        },
        DiscoveryProtocol::Consul {
            address: "127.0.0.1:8500".to_string(),
            datacenter: "dc1".to_string(),
        },
        DiscoveryProtocol::Etcd {
            endpoints: vec!["http://127.0.0.1:2379".to_string()],
            key_prefix: "/beardog/".to_string(),
            timeout_ms: 1000,
        },
    ];

    let usd = UniversalServiceDiscovery::new(config).expect("construct");
    usd.start().expect("start");

    let svc = ServiceInfo {
        name: "orch-test".to_string(),
        service_type: "grpc".to_string(),
        address: "127.0.0.1".to_string(),
        port: 9000,
        metadata: HashMap::new(),
    };
    usd.register_service(svc).expect("register");

    let discovered = usd.discover_services("orch-test").await.expect("discover");
    assert!(discovered.len() <= 8);

    let _health = usd.get_service_health("orch-test").expect("get health");

    let stats = usd.get_discovery_statistics().expect("stats");
    assert_eq!(stats.total_services, 4);
    assert!(stats.uptime <= std::time::Duration::from_secs(1));

    let healthy_n = usd.get_healthy_service_count().expect("healthy count");
    assert!(healthy_n <= 100);

    let mut rx = usd.subscribe_events();
    let _ = rx.try_recv();

    usd.deregister_service("orch-test").expect("deregister");
    usd.stop().expect("stop");
}

#[test]
fn minimal_protocol_handler_trait_smoke() {
    let h = MinimalProtocolHandler::new();
    let _: &dyn ProtocolHandler = &h;
    assert!(h.start().is_ok());
    let si = ServiceInfo {
        name: "n".to_string(),
        service_type: "t".to_string(),
        address: "a".to_string(),
        port: 1,
        metadata: HashMap::new(),
    };
    assert!(h.register_service(&si).is_ok());
    assert!(h.deregister_service(&si).is_ok());
    assert!(h.discover_services("any").unwrap().is_empty());
    assert!(h.get_statistics().is_ok());
    assert!(h.stop().is_ok());
}
