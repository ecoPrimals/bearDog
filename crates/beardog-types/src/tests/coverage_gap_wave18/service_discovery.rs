// SPDX-License-Identifier: AGPL-3.0-or-later
//! Service discovery factory and provider tests.

#![cfg(test)]

use crate::canonical::capabilities::ServiceCapabilityType;
use crate::canonical::discovery::service_discovery_capability::{
    ConsulDiscovery, DiscoveryError, DnsHttpDiscovery, EtcdDiscovery, ServiceDiscoveryBackend,
    ServiceDiscoveryCapability, create_service_discovery,
};

#[tokio::test]
async fn create_service_discovery_returns_known_provider() {
    let disc = create_service_discovery()
        .await
        .expect("create_service_discovery should succeed");
    let name = disc.provider_name();
    assert!(
        name.contains("kubernetes") || name.contains("dns"),
        "unexpected discovery provider: {name}"
    );
}

#[test]
fn consul_discovery_reports_unavailable() {
    let err = ConsulDiscovery::try_create().expect_err("consul not implemented");
    assert!(matches!(err, DiscoveryError::BackendUnavailable { .. }));
}

#[test]
fn etcd_discovery_reports_unavailable() {
    let err = EtcdDiscovery::try_create().expect_err("etcd not implemented");
    assert!(matches!(err, DiscoveryError::BackendUnavailable { .. }));
}

#[tokio::test]
async fn dns_http_discovery_default_and_trait_methods() {
    let d = DnsHttpDiscovery::new();

    let cap = ServiceCapabilityType::DataStorage;
    let by_cap_err = d
        .discover_by_capability(cap)
        .await
        .expect_err("capability discovery requires DNS resolver");
    assert!(
        matches!(by_cap_err, DiscoveryError::BackendUnavailable { .. }),
        "expected BackendUnavailable, got: {by_cap_err:?}"
    );

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

#[test]
fn dns_http_discovery_is_cloneable() {
    let a = DnsHttpDiscovery::new();
    let b = a.clone();
    assert_eq!(a.provider_name(), b.provider_name());
}

#[test]
fn service_discovery_enum_dispatch() {
    let d = ServiceDiscoveryBackend::DnsHttp(DnsHttpDiscovery::new());
    assert_eq!(d.provider_name(), "dns-http-fallback");
}
