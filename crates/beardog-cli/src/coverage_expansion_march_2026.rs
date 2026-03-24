// SPDX-License-Identifier: AGPL-3.0-only
//! Additional unit tests for CLI argument structs and discovery URL parsing (coverage expansion).

#![allow(clippy::expect_used, clippy::unwrap_used)]

use crate::ecosystem_discovery_adapter::EcosystemDiscoveryAdapter;
use crate::{ClientArgs, DaemonArgs, DoctorArgs, ServerArgs};
use clap::Parser;

// --- ServerArgs ---

#[test]
fn server_args_parse_uses_defaults_when_only_program_name() {
    let a = ServerArgs::try_parse_from(["beardog"]).expect("parse");
    assert!(!a.socket.is_empty());
}

#[test]
fn server_args_explicit_socket() {
    let a = ServerArgs::try_parse_from(["beardog", "--socket", "/tmp/x.sock"]).expect("parse");
    assert_eq!(a.socket, "/tmp/x.sock");
}

#[test]
fn server_args_abstract_flag() {
    let a = ServerArgs::try_parse_from(["beardog", "--abstract"]).expect("parse");
    assert!(a.r#abstract);
}

#[test]
fn server_args_listen_loopback() {
    let a = ServerArgs::try_parse_from(["beardog", "--listen", "127.0.0.1:9900"]).expect("parse");
    assert_eq!(a.listen.as_deref(), Some("127.0.0.1:9900"));
}

#[test]
fn server_args_family_and_orchestrator_ids() {
    let a = ServerArgs::try_parse_from([
        "beardog",
        "--family-id",
        "fam-1",
        "--orchestrator-id",
        "orch-9",
    ])
    .expect("parse");
    assert_eq!(a.family_id.as_deref(), Some("fam-1"));
    assert_eq!(a.orchestrator_id.as_deref(), Some("orch-9"));
}

// --- DaemonArgs ---

#[test]
fn daemon_args_custom_socket_pid_log() {
    let a = DaemonArgs::try_parse_from([
        "beardog",
        "--socket",
        "/run/a.sock",
        "--pid-file",
        "/var/pid",
        "--log-file",
        "/var/log",
    ])
    .expect("parse");
    assert_eq!(a.socket, "/run/a.sock");
    assert_eq!(a.pid_file, "/var/pid");
    assert_eq!(a.log_file, "/var/log");
}

#[test]
fn daemon_args_optional_ids() {
    let a = DaemonArgs::try_parse_from(["beardog", "--family-id", "f", "--orchestrator-id", "o"])
        .expect("parse");
    assert_eq!(a.family_id.as_deref(), Some("f"));
    assert_eq!(a.orchestrator_id.as_deref(), Some("o"));
}

// --- ClientArgs ---

#[test]
fn client_args_with_inline_command() {
    let a = ClientArgs::try_parse_from(["beardog", "--command", "status"]).expect("parse");
    assert_eq!(a.command.as_deref(), Some("status"));
}

#[test]
fn client_args_socket_override() {
    let a = ClientArgs::try_parse_from(["beardog", "--socket", "@abstract"]).expect("parse");
    assert_eq!(a.socket, "@abstract");
}

// --- DoctorArgs ---

#[test]
fn doctor_args_text_format_default() {
    let a = DoctorArgs::try_parse_from(["beardog"]).expect("parse");
    assert_eq!(a.format, "text");
    assert!(!a.comprehensive);
}

#[test]
fn doctor_args_json_format_and_comprehensive() {
    let a = DoctorArgs::try_parse_from(["beardog", "--format", "json", "--comprehensive"])
        .expect("parse");
    assert_eq!(a.format, "json");
    assert!(a.comprehensive);
}

#[test]
fn doctor_args_component_filter() {
    let a = DoctorArgs::try_parse_from(["beardog", "--component", "socket"]).expect("parse");
    assert_eq!(a.component.as_deref(), Some("socket"));
}

// --- parse_endpoint_url (error and boundary paths) ---

#[test]
fn parse_endpoint_url_malformed_port_falls_back_to_default() {
    let (p, h, port, path) =
        EcosystemDiscoveryAdapter::parse_endpoint_url("https://example.com:notaport/api");
    assert_eq!(p, "https");
    assert_eq!(h, "example.com");
    assert_eq!(port, 8080);
    assert_eq!(path.as_deref(), Some("/api"));
}

#[test]
fn parse_endpoint_url_https_only_scheme() {
    let (p, h, port, path) = EcosystemDiscoveryAdapter::parse_endpoint_url("https://");
    assert_eq!(p, "https");
    assert_eq!(h, "");
    assert_eq!(port, 8080);
    assert_eq!(path, None);
}

#[test]
fn parse_endpoint_url_scheme_host_no_slash_no_path() {
    let (p, h, port, path) = EcosystemDiscoveryAdapter::parse_endpoint_url("wss://relay.example");
    assert_eq!(p, "wss");
    assert_eq!(h, "relay.example");
    assert_eq!(port, 8080);
    assert_eq!(path, None);
}

#[test]
fn parse_endpoint_url_host_with_embedded_colon_no_slash_uses_default_port() {
    // Without a `/` path segment, the parser treats the entire rest as the host string
    // (port split only runs when a `/` exists).
    let (p, h, port, path) = EcosystemDiscoveryAdapter::parse_endpoint_url("tcp://10.0.0.5:12345");
    assert_eq!(p, "tcp");
    assert_eq!(h, "10.0.0.5:12345");
    assert_eq!(port, 8080);
    assert_eq!(path, None);
}

#[test]
fn parse_endpoint_url_root_path_only() {
    let (p, h, port, path) = EcosystemDiscoveryAdapter::parse_endpoint_url("http://127.0.0.1:1/");
    assert_eq!(p, "http");
    assert_eq!(h, "127.0.0.1");
    assert_eq!(port, 1);
    assert_eq!(path.as_deref(), Some("/"));
}

#[test]
fn parse_endpoint_url_empty_string_fallback() {
    let (p, h, port, path) = EcosystemDiscoveryAdapter::parse_endpoint_url("");
    assert_eq!(p, "http");
    assert_eq!(h, "");
    assert_eq!(port, 8080);
    assert_eq!(path, None);
}

#[test]
fn parse_endpoint_url_colon_in_path_not_port() {
    let (p, h, port, path) =
        EcosystemDiscoveryAdapter::parse_endpoint_url("https://cdn.example/static:v2/data");
    assert_eq!(p, "https");
    assert_eq!(h, "cdn.example");
    assert_eq!(port, 8080);
    assert_eq!(path.as_deref(), Some("/static:v2/data"));
}

#[test]
fn server_args_debug_impl_non_empty() {
    let a = ServerArgs::try_parse_from(["x", "--listen", "a:1"]).expect("parse");
    let s = format!("{a:?}");
    assert!(s.contains("ServerArgs"));
}

#[test]
fn daemon_args_debug_impl_non_empty() {
    let a = DaemonArgs::try_parse_from(["x"]).expect("parse");
    assert!(format!("{a:?}").contains("DaemonArgs"));
}

#[test]
fn client_args_clone_roundtrip() {
    let a = ClientArgs::try_parse_from(["x", "--socket", "s"]).expect("parse");
    let b = a.clone();
    assert_eq!(a.socket, b.socket);
}

#[test]
fn doctor_args_clone_roundtrip() {
    let a = DoctorArgs::try_parse_from(["x", "--comprehensive"]).expect("parse");
    let b = a.clone();
    assert_eq!(a.comprehensive, b.comprehensive);
}

#[test]
fn server_args_clone_preserves_listen() {
    let a = ServerArgs::try_parse_from(["p", "--listen", "0.0.0.0:1"]).expect("parse");
    assert_eq!(a.clone().listen, a.listen);
}

#[test]
fn parse_endpoint_url_scheme_plus_host_port_and_path() {
    let (p, h, port, path) =
        EcosystemDiscoveryAdapter::parse_endpoint_url("http://10.0.0.5:12345/");
    assert_eq!(p, "http");
    assert_eq!(h, "10.0.0.5");
    assert_eq!(port, 12345);
    assert_eq!(path.as_deref(), Some("/"));
}

#[test]
fn parse_endpoint_url_max_u16_port() {
    let (p, h, port, path) =
        EcosystemDiscoveryAdapter::parse_endpoint_url("http://edge.case:65535/z");
    assert_eq!(p, "http");
    assert_eq!(h, "edge.case");
    assert_eq!(port, 65535);
    assert_eq!(path.as_deref(), Some("/z"));
}
