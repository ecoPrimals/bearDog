// SPDX-License-Identifier: AGPL-3.0-or-later
//! Service discovery factory and provider tests.

#![cfg(test)]

use crate::canonical::capabilities::ServiceCapabilityType;
use crate::canonical::discovery::service_discovery_capability::{
    ConsulDiscovery, DiscoveryError, DnsHttpDiscovery, EtcdDiscovery, KubernetesDiscovery,
    ServiceDiscoveryBackend, ServiceDiscoveryCapability, create_service_discovery,
};

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
