// SPDX-License-Identifier: AGPL-3.0-or-later

//! Targeted coverage tests for announcement, capability environment helpers, and error conversions.

use crate::announcement::Announcer;
use crate::capability_env::{
    discovered_services_from_environment, discovered_services_from_environment_with,
    primary_url_to_ipc_socket_path,
};
use crate::config::{AnnouncementConfig, MdnsAnnouncementConfig};
use crate::error::DiscoveryError;
use crate::types::{Capability, PrimalInfo, ServiceEndpoint};
use std::collections::HashMap;
use std::env::VarError;

fn sample_announcement_config(enabled: bool, methods: Vec<&str>) -> AnnouncementConfig {
    AnnouncementConfig {
        enabled,
        methods: methods.into_iter().map(String::from).collect(),
        announcement_interval_secs: 10,
        ttl_secs: 60,
        mdns: MdnsAnnouncementConfig::default(),
    }
}

fn sample_primal_info(primal_id: &str) -> PrimalInfo {
    PrimalInfo {
        primal_id: primal_id.to_string(),
        primal_type: "test".to_string(),
        version: "0.1.0".to_string(),
        display_name: "Coverage Wave10".to_string(),
        capabilities: vec![Capability {
            capability_type: "orch".to_string(),
            version: "1".to_string(),
            features: vec![],
            parameters: HashMap::new(),
        }],
        endpoint: ServiceEndpoint {
            primary_url: "http://127.0.0.1:9".to_string(),
            fallback_urls: vec![],
            use_tls: false,
            path_prefix: None,
        },
    }
}

#[tokio::test]
async fn announcer_new_and_start_ok_when_disabled() {
    let config = sample_announcement_config(false, vec!["mdns"]);
    let info = sample_primal_info("wave10-new");
    let announcer = Announcer::new(config, info);
    announcer
        .start()
        .await
        .expect("disabled announcement should return Ok");
}

#[tokio::test]
async fn announcer_with_service_registry_url_chains() {
    let config = sample_announcement_config(true, vec!["service_registry"]);
    let info = sample_primal_info("wave10-registry");
    Announcer::new(config, info)
        .with_service_registry_url(Some("http://registry.local:8500".to_string()))
        .start()
        .await
        .expect("service_registry announcement path should return Ok");
}

#[tokio::test]
async fn announcer_start_enabled_mdns() {
    let config = sample_announcement_config(true, vec!["mdns"]);
    let info = sample_primal_info("wave10-mdns");
    Announcer::new(config, info)
        .start()
        .await
        .expect("mDNS announcement path should return Ok");
}

#[tokio::test]
async fn announcer_start_enabled_environment() {
    let config = sample_announcement_config(true, vec!["environment"]);
    let info = sample_primal_info("wave10-env-announce");
    Announcer::new(config, info)
        .start()
        .await
        .expect("environment announcement path should return Ok");
}

#[tokio::test]
async fn announcer_start_service_registry_with_and_without_url() {
    let config = sample_announcement_config(true, vec!["service_registry"]);
    let info = sample_primal_info("wave10-sr");

    Announcer::new(config.clone(), info.clone())
        .with_service_registry_url(None)
        .start()
        .await
        .expect("service_registry without URL should still Ok");

    Announcer::new(config, info)
        .with_service_registry_url(Some("http://consul:8500".to_string()))
        .start()
        .await
        .expect("service_registry with URL should Ok");
}

#[tokio::test]
async fn announcer_start_unknown_method_is_no_op() {
    let config = sample_announcement_config(true, vec!["not_a_real_method"]);
    let info = sample_primal_info("wave10-unknown");
    Announcer::new(config, info)
        .start()
        .await
        .expect("unknown announcement method should be ignored");
}

#[tokio::test]
async fn announcer_all_methods_combined() {
    let config = sample_announcement_config(
        true,
        vec!["mdns", "environment", "service_registry", "custom_unknown"],
    );
    let info = sample_primal_info("wave10-all");
    Announcer::new(config, info)
        .with_service_registry_url(Some("http://etcd:2379".to_string()))
        .start()
        .await
        .expect("combined announcement methods should complete");
}

#[test]
fn discovered_services_wrapper_delegates_to_from_env() {
    let a = discovered_services_from_environment("wave10-cov-unique-cap", 123);
    let b = crate::discovered_services_from_environment_from_env("wave10-cov-unique-cap", 123);
    assert_eq!(a.len(), b.len());
}

#[test]
fn discovered_services_with_primal_endpoint_and_capabilities() {
    let capability = "orch";
    let ttl = 99u64;
    let vars = vec![(
        "PRIMAL_WAVE10COV_ENDPOINT".to_string(),
        "https://primal.example:8443".to_string(),
    )];
    let lookup: HashMap<String, String> = HashMap::from([(
        "PRIMAL_WAVE10COV_CAPABILITIES".to_string(),
        "other,orch".to_string(),
    )]);

    let services = discovered_services_from_environment_with(
        capability,
        ttl,
        |k| lookup.get(k).cloned().ok_or(VarError::NotPresent),
        vars,
    );

    assert_eq!(services.len(), 1, "expected one primal match");
    let s = services.first().expect("one service");
    assert_eq!(s.id, "primal-wave10cov");
    assert_eq!(s.endpoint.primary_url, "https://primal.example:8443");
    assert!(s.endpoint.use_tls);
    assert!(
        s.capabilities.iter().any(|c| c.capability_type == "orch"),
        "should include requested capability"
    );
}

#[test]
fn primary_url_to_ipc_socket_path_unix_triple_slash() {
    assert_eq!(
        primary_url_to_ipc_socket_path("unix:///run/wave10.sock"),
        "/run/wave10.sock"
    );
}

#[test]
fn primary_url_to_ipc_socket_path_unix_colon_prefix() {
    assert_eq!(
        primary_url_to_ipc_socket_path("unix:/run/wave10.sock"),
        "/run/wave10.sock"
    );
}

#[test]
fn primary_url_to_ipc_socket_path_bare_path() {
    assert_eq!(
        primary_url_to_ipc_socket_path("  /tmp/beardog-ipc  "),
        "/tmp/beardog-ipc"
    );
}

#[test]
fn discovery_error_from_toml_de_error() {
    let toml_err = toml::from_str::<toml::Value>("{{{ not valid toml")
        .expect_err("fixture must be invalid TOML");
    let err: DiscoveryError = toml_err.into();
    assert!(
        matches!(err, DiscoveryError::Config(_)),
        "expected Config variant, got {err:?}"
    );
}
