// SPDX-License-Identifier: AGPL-3.0-or-later
//! Network address constants and IPC discovery path tests.

#![cfg(test)]

use crate::constants::domains::network::addresses;
use crate::constants::domains::network::ipc_discovery::{self};

#[test]
fn network_address_constants_and_helpers() {
    assert_eq!(addresses::DEFAULT_LOCALHOST_IPV4_STR, "127.0.0.1");
    assert_eq!(
        addresses::LOCALHOST_IPV4,
        addresses::DEFAULT_LOCALHOST_IPV4_STR
    );
    let bind = addresses::default_bind_address();
    assert!(bind.contains('0'));
    let api = addresses::default_api_bind();
    assert!(api.contains(':'));
    assert!(addresses::default_metrics_bind().contains(':'));
    assert!(addresses::default_health_bind().contains(':'));
    assert_eq!(addresses::multicast_address(), "224.0.0.251".to_string());
    let dns = addresses::dns_servers();
    assert_eq!(dns.len(), 3);
}

#[test]
fn network_addresses_bind_helpers_use_ports() {
    let api = addresses::default_api_bind_from_env();
    assert!(api.contains(':'));
    let m = addresses::default_metrics_bind_from_env();
    assert!(m.contains(':'));
    let h = addresses::default_health_bind_from_env();
    assert!(h.contains(':'));
}

#[test]
fn network_addresses_multicast_from_env_returns_string() {
    let m = addresses::multicast_address_from_env();
    assert!(!m.is_empty());
}

#[test]
fn network_dns_servers_from_env_returns_vec() {
    let v = addresses::dns_servers_from_env();
    assert!(!v.is_empty());
}

#[test]
fn ipc_discovery_resolve_subdir_and_paths() {
    assert_eq!(
        ipc_discovery::resolve_biomeos_ipc_subdir_from_optional(Some("  ns  ")),
        "ns"
    );
    assert_eq!(
        ipc_discovery::resolve_biomeos_ipc_subdir_from_optional(None),
        ipc_discovery::BIOMEOS_RUNTIME_SOCKET_SUBDIR
    );

    let p =
        ipc_discovery::biomeos_ipc_socket_dir_from_components(Some("/override/path"), None, None);
    assert_eq!(p, std::path::PathBuf::from("/override/path"));

    let p2 = ipc_discovery::biomeos_ipc_socket_dir_from_components(
        None,
        Some("/run/user/1"),
        Some("custom"),
    );
    assert!(p2.ends_with("custom"));

    let tmp = ipc_discovery::biomeos_tmp_socket_root();
    let p3 = ipc_discovery::biomeos_ipc_socket_dir_from_components(None, None, Some("z"));
    assert!(p3.starts_with(tmp));

    let uri = ipc_discovery::default_upa_registry_unix_uri();
    assert!(uri.starts_with("unix://"));

    assert_eq!(
        ipc_discovery::resolve_upa_registry_endpoint(),
        ipc_discovery::default_upa_registry_unix_uri()
    );
}

#[test]
fn ipc_discovery_biomeos_ipc_socket_dir_from_env_non_empty() {
    let p = ipc_discovery::biomeos_ipc_socket_dir_from_env();
    assert!(!p.as_os_str().is_empty());
}

#[test]
fn ipc_discovery_resolve_upa_registry_endpoint_from_env_matches_default_uri_or_override() {
    let s = ipc_discovery::resolve_upa_registry_endpoint_from_env();
    assert!(
        s.starts_with("unix://") || s.starts_with("http://") || s.starts_with("https://"),
        "registry endpoint should look like a URI: {s}"
    );
}

#[test]
fn addresses_deprecated_aliases_resolve() {
    #[expect(
        deprecated,
        reason = "migration in progress — see CANONICAL_TYPE_MIGRATION_GUIDE"
    )]
    {
        assert_eq!(addresses::DEFAULT_BIND_ADDRESS, addresses::WILDCARD_IPV4);
        assert!(addresses::DEFAULT_METRICS_BIND.contains(':'));
        assert!(addresses::DEFAULT_HEALTH_BIND.contains(':'));
        assert_eq!(addresses::MULTICAST_ADDRESS, "224.0.0.251");
    }
}

#[test]
fn ipc_discovery_public_constants_are_non_empty() {
    assert!(!ipc_discovery::BIOMEOS_RUNTIME_SOCKET_SUBDIR.is_empty());
    assert!(!ipc_discovery::BEARDOG_TCP_DISCOVERY_FILENAME.is_empty());
    assert!(!ipc_discovery::DEFAULT_UPA_REGISTRY_SOCKET_NAME.is_empty());
    assert!(!ipc_discovery::ENV_BIOMEOS_SOCKET_DIR_OVERRIDE.is_empty());
}
