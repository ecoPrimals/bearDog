// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{BindMode, ServerArgs};
use beardog_types::constants::domains::network::ipc_discovery::resolve_biomeos_ipc_subdir_from_optional;

use super::health::run_health_socket;
use super::registration::attempt_orchestrator_registration;
use super::transport::{
    neural_registration_address, resolve_effective_tcp_listen, resolve_server_socket_path,
};

#[test]
fn neural_registration_address_prefers_tcp_when_present() {
    assert_eq!(
        neural_registration_address(Some("0.0.0.0:9000"), "/tmp/x.sock"),
        "0.0.0.0:9000"
    );
}

#[test]
fn neural_registration_address_falls_back_to_unix_path() {
    assert_eq!(
        neural_registration_address(None, "@abstract_sock"),
        "@abstract_sock"
    );
}

#[test]
fn resolve_effective_tcp_listen_from_port_only() {
    assert_eq!(
        resolve_effective_tcp_listen(Some(9900), None).as_deref(),
        Some("0.0.0.0:9900")
    );
}

#[test]
fn resolve_effective_tcp_listen_from_listen_only() {
    assert_eq!(
        resolve_effective_tcp_listen(None, Some("127.0.0.1:7777")).as_deref(),
        Some("127.0.0.1:7777")
    );
}

#[test]
fn resolve_effective_tcp_listen_none_when_neither_set() {
    assert_eq!(resolve_effective_tcp_listen(None, None), None);
}

#[test]
fn resolve_effective_tcp_listen_none_when_both_set_like_invalid_cli_state() {
    assert_eq!(
        resolve_effective_tcp_listen(Some(8080), Some("127.0.0.1:1")),
        None
    );
}

#[test]
fn resolve_server_socket_path_abstract_default_family() {
    let args = ServerArgs {
        bind_mode: BindMode::Auto,
        socket: "/tmp/ignored.sock".to_string(),
        r#abstract: true,
        port: None,
        listen: None,
        audit_dir: None,
        family_id: None,
        orchestrator_id: None,
        health_socket: None,
    };
    let ns = resolve_biomeos_ipc_subdir_from_optional(None);
    assert_eq!(
        resolve_server_socket_path(&args),
        format!("@{ns}_beardog_default")
    );
}

#[test]
fn resolve_server_socket_path_abstract_named_family() {
    let args = ServerArgs {
        bind_mode: BindMode::Auto,
        socket: "/tmp/ignored.sock".to_string(),
        r#abstract: true,
        port: None,
        listen: None,
        audit_dir: None,
        family_id: Some("alpha".to_string()),
        orchestrator_id: None,
        health_socket: None,
    };
    let ns = resolve_biomeos_ipc_subdir_from_optional(None);
    assert_eq!(
        resolve_server_socket_path(&args),
        format!("@{ns}_beardog_alpha")
    );
}

#[test]
fn resolve_server_socket_path_family_scoped_file() {
    let args = ServerArgs {
        bind_mode: BindMode::Auto,
        socket: "/var/run/beardog.sock".to_string(),
        r#abstract: false,
        port: None,
        listen: None,
        audit_dir: None,
        family_id: Some("fam99".to_string()),
        orchestrator_id: None,
        health_socket: None,
    };
    assert_eq!(
        resolve_server_socket_path(&args),
        "/var/run/beardog-fam99.sock"
    );
}

#[test]
fn resolve_server_socket_path_family_with_socket_filename_only_uses_parent_join() {
    let args = ServerArgs {
        bind_mode: BindMode::Auto,
        socket: "beardog.sock".to_string(),
        r#abstract: false,
        port: None,
        listen: None,
        audit_dir: None,
        family_id: Some("rel".to_string()),
        orchestrator_id: None,
        health_socket: None,
    };
    let resolved = resolve_server_socket_path(&args);
    assert!(
        resolved.ends_with("beardog-rel.sock"),
        "unexpected path: {resolved}"
    );
}

#[test]
fn resolve_server_socket_path_explicit_when_no_family() {
    let args = ServerArgs {
        bind_mode: BindMode::Auto,
        socket: "/tmp/custom.sock".to_string(),
        r#abstract: false,
        port: None,
        listen: None,
        audit_dir: None,
        family_id: None,
        orchestrator_id: None,
        health_socket: None,
    };
    assert_eq!(resolve_server_socket_path(&args), "/tmp/custom.sock");
}

#[test]
fn resolve_server_socket_path_bind_mode_abstract_without_legacy_flag() {
    let args = ServerArgs {
        bind_mode: BindMode::Abstract,
        socket: "/tmp/ignored.sock".to_string(),
        r#abstract: false,
        port: None,
        listen: None,
        audit_dir: None,
        family_id: Some("gamma".to_string()),
        orchestrator_id: None,
        health_socket: None,
    };
    let ns = resolve_biomeos_ipc_subdir_from_optional(None);
    assert_eq!(
        resolve_server_socket_path(&args),
        format!("@{ns}_beardog_gamma")
    );
}

#[test]
fn resolve_server_socket_path_bind_mode_tcp_returns_empty() {
    let args = ServerArgs {
        bind_mode: BindMode::Tcp,
        socket: "/tmp/ignored.sock".to_string(),
        r#abstract: false,
        port: Some(9100),
        listen: None,
        audit_dir: None,
        family_id: None,
        orchestrator_id: None,
        health_socket: None,
    };
    assert_eq!(resolve_server_socket_path(&args), "");
}

#[test]
fn resolve_server_socket_path_bind_mode_filesystem_uses_explicit() {
    let args = ServerArgs {
        bind_mode: BindMode::Filesystem,
        socket: "/run/beardog.sock".to_string(),
        r#abstract: false,
        port: None,
        listen: None,
        audit_dir: None,
        family_id: None,
        orchestrator_id: None,
        health_socket: None,
    };
    assert_eq!(resolve_server_socket_path(&args), "/run/beardog.sock");
}

#[tokio::test]
async fn attempt_orchestrator_registration_completes_without_panic() {
    attempt_orchestrator_registration("/tmp/beardog_unit_test_orchestrator.sock", None).await;
}

#[tokio::test]
async fn attempt_orchestrator_registration_with_tcp_addr_completes_without_panic() {
    attempt_orchestrator_registration("/tmp/beardog.sock", Some("127.0.0.1:9900")).await;
}

#[tokio::test]
async fn health_socket_responds_to_plain_json_rpc() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixStream;

    let dir = tempfile::tempdir().expect("tempdir");
    let sock_path = dir.path().join("health-test.sock");
    let path_str = sock_path.to_string_lossy().to_string();

    let path_clone = path_str.clone();
    tokio::spawn(async move {
        let _ = run_health_socket(&path_clone).await;
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let mut stream = UnixStream::connect(&path_str).await.expect("connect");
    let req = b"{\"jsonrpc\":\"2.0\",\"method\":\"health\",\"id\":42}\n";
    stream.write_all(req).await.expect("write");

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).await.expect("read response");

    let resp: serde_json::Value = serde_json::from_str(&line).expect("parse JSON");
    assert_eq!(resp["jsonrpc"], "2.0");
    assert_eq!(resp["id"], 42);
    assert_eq!(resp["result"]["status"], "alive");
    assert_eq!(resp["result"]["primal"], "beardog");
    assert!(resp["result"]["version"].is_string());
}

#[tokio::test]
async fn health_socket_tolerates_ribocipher_prefix() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixStream;

    let dir = tempfile::tempdir().expect("tempdir");
    let sock_path = dir.path().join("health-ribo.sock");
    let path_str = sock_path.to_string_lossy().to_string();

    let path_clone = path_str.clone();
    tokio::spawn(async move {
        let _ = run_health_socket(&path_clone).await;
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let mut stream = UnixStream::connect(&path_str).await.expect("connect");
    // riboCipher prefix [0xEC, 0x01] then JSON-RPC
    let mut payload = vec![0xEC, 0x01];
    payload.extend_from_slice(b"{\"jsonrpc\":\"2.0\",\"method\":\"health.liveness\",\"id\":7}\n");
    stream.write_all(&payload).await.expect("write");

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).await.expect("read response");

    let resp: serde_json::Value = serde_json::from_str(&line).expect("parse JSON");
    assert_eq!(resp["id"], 7);
    assert_eq!(resp["result"]["status"], "alive");
}

#[tokio::test]
async fn health_socket_rejects_non_health_method() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixStream;

    let dir = tempfile::tempdir().expect("tempdir");
    let sock_path = dir.path().join("health-reject.sock");
    let path_str = sock_path.to_string_lossy().to_string();

    let path_clone = path_str.clone();
    tokio::spawn(async move {
        let _ = run_health_socket(&path_clone).await;
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let mut stream = UnixStream::connect(&path_str).await.expect("connect");
    let req = b"{\"jsonrpc\":\"2.0\",\"method\":\"crypto.sign_ed25519\",\"id\":99}\n";
    stream.write_all(req).await.expect("write");

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).await.expect("read response");

    let resp: serde_json::Value = serde_json::from_str(&line).expect("parse JSON");
    assert_eq!(resp["jsonrpc"], "2.0");
    assert_eq!(resp["id"], 99);
    assert_eq!(resp["error"]["code"], -32601);
    assert_eq!(resp["error"]["message"], "Method not found");
    assert!(resp["error"]["data"]["reason"].as_str().unwrap().contains("health probe socket"));
}
