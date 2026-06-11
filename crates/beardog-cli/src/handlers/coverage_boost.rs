// SPDX-License-Identifier: AGPL-3.0-or-later

//! Targeted coverage tests for low-coverage CLI handlers (server, hsm, daemon, `cross_primal`).

use crate::{BindMode, ServerArgs};

// -- resolve_server_socket_path: bare filename fallback to /tmp parent --

#[test]
fn resolve_socket_path_family_bare_filename_uses_empty_parent() {
    let args = ServerArgs {
        bind_mode: BindMode::Auto,
        socket: "beardog.sock".to_string(),
        r#abstract: false,
        port: None,
        listen: None,
        audit_dir: None,
        family_id: Some("bare".to_string()),
        orchestrator_id: None,
    };
    let p = super::server::resolve_server_socket_path(&args);
    assert!(
        p.ends_with("beardog-bare.sock"),
        "should derive family-scoped socket: {p}"
    );
}

#[test]
fn resolve_socket_path_with_listen_addr_still_returns_socket_for_explicit() {
    let args = ServerArgs {
        bind_mode: BindMode::Auto,
        socket: "/run/bd.sock".to_string(),
        r#abstract: false,
        port: None,
        listen: Some("0.0.0.0:0".to_string()),
        audit_dir: None,
        family_id: None,
        orchestrator_id: None,
    };
    let p = super::server::resolve_server_socket_path(&args);
    assert_eq!(p, "/run/bd.sock");
}

// -- HSM handlers: verbose discover and verbose capabilities --

#[tokio::test]
async fn hsm_discover_verbose_true_ok() {
    super::hsm::handle_hsm_discover(true)
        .await
        .expect("verbose HSM discover should succeed");
}

#[tokio::test]
async fn hsm_capabilities_of_likely_software_hsm() {
    let hsms = super::hsm::discover_hsms_agnostic()
        .await
        .expect("discover");
    if let Some(hsm) = hsms.first() {
        super::hsm::handle_hsm_capabilities(&hsm.id)
            .await
            .expect("capabilities for discovered HSM should succeed");
    }
}

#[tokio::test]
async fn hsm_test_of_likely_software_hsm() {
    let hsms = super::hsm::discover_hsms_agnostic()
        .await
        .expect("discover");
    if let Some(hsm) = hsms.first() {
        super::hsm::handle_hsm_test(&hsm.id, 2)
            .await
            .expect("test for discovered HSM should succeed");
    }
}

// -- cross_primal: dispatch through handle_cross_primal wrapper --

#[tokio::test]
async fn cross_primal_dispatch_discover_network() {
    use clap::Parser;

    let cmd = super::cross_primal::CrossPrimalCommand::try_parse_from([
        "beardog",
        "discover-primals",
        "--capability",
        "network",
    ])
    .expect("parse discover-primals");

    super::cross_primal::handle_cross_primal(cmd)
        .await
        .expect("dispatch discover should succeed even with no primals");
}

#[tokio::test]
async fn cross_primal_dispatch_discover_compute() {
    use clap::Parser;

    let cmd = super::cross_primal::CrossPrimalCommand::try_parse_from([
        "beardog",
        "discover-primals",
        "--capability",
        "compute",
    ])
    .expect("parse discover-primals compute");

    super::cross_primal::handle_cross_primal(cmd)
        .await
        .expect("dispatch discover compute should succeed");
}

#[tokio::test]
async fn cross_primal_dispatch_discover_storage() {
    use clap::Parser;

    let cmd = super::cross_primal::CrossPrimalCommand::try_parse_from([
        "beardog",
        "discover-primals",
        "--capability",
        "storage",
    ])
    .expect("parse discover-primals storage");

    super::cross_primal::handle_cross_primal(cmd)
        .await
        .expect("dispatch discover storage should succeed");
}

#[tokio::test]
async fn cross_primal_dispatch_discover_security() {
    use clap::Parser;

    let cmd = super::cross_primal::CrossPrimalCommand::try_parse_from([
        "beardog",
        "discover-primals",
        "--capability",
        "security",
    ])
    .expect("parse discover-primals security");

    super::cross_primal::handle_cross_primal(cmd)
        .await
        .expect("dispatch discover security should succeed");
}

#[tokio::test]
async fn cross_primal_dispatch_discover_service_mesh() {
    use clap::Parser;

    let cmd = super::cross_primal::CrossPrimalCommand::try_parse_from([
        "beardog",
        "discover-primals",
        "--capability",
        "service_mesh",
    ])
    .expect("parse discover-primals service_mesh");

    super::cross_primal::handle_cross_primal(cmd)
        .await
        .expect("dispatch discover service_mesh should succeed");
}

#[tokio::test]
async fn cross_primal_dispatch_discover_invalid_errors() {
    use clap::Parser;

    let cmd = super::cross_primal::CrossPrimalCommand::try_parse_from([
        "beardog",
        "discover-primals",
        "--capability",
        "nonexistent_cap",
    ])
    .expect("parse");

    let err = super::cross_primal::handle_cross_primal(cmd)
        .await
        .expect_err("invalid capability should error");
    let msg = format!("{err}");
    assert!(
        msg.contains("Unknown capability") || msg.contains("nonexistent_cap"),
        "unexpected error: {msg}"
    );
}

#[tokio::test]
async fn cross_primal_dispatch_key_ceremony_missing_seed() {
    use clap::Parser;

    let cmd = super::cross_primal::CrossPrimalCommand::try_parse_from([
        "beardog",
        "key-ceremony",
        "--seed-file",
        "/nonexistent/seed_wave10_cov",
        "--output",
        "/tmp/wave10out",
    ])
    .expect("parse key-ceremony");

    let err = super::cross_primal::handle_cross_primal(cmd)
        .await
        .expect_err("missing seed file should error");
    let msg = format!("{err}");
    assert!(
        msg.contains("seed file") || msg.contains("No such file"),
        "unexpected error: {msg}"
    );
}

#[tokio::test]
async fn cross_primal_dispatch_send_secure_missing_message_file() {
    use clap::Parser;

    let cmd = super::cross_primal::CrossPrimalCommand::try_parse_from([
        "beardog",
        "send-secure",
        "--message",
        "/nonexistent/msg_wave10_cov",
        "--capability",
        "network",
    ])
    .expect("parse send-secure");

    let err = super::cross_primal::handle_cross_primal(cmd)
        .await
        .expect_err("missing message file should error");
    let msg = format!("{err}");
    assert!(
        msg.contains("message file") || msg.contains("No such file"),
        "unexpected error: {msg}"
    );
}

// -- daemon: pid file edge cases --

#[test]
fn daemon_pid_file_double_prepare_succeeds_with_stale_self() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("wave10.pid");
    let p = path.to_str().expect("utf8");

    super::daemon::prepare_daemon_pid_file(p).expect("first prepare");
    let pid1 = std::fs::read_to_string(p).expect("read pid1");

    // Simulate stale by writing a dead PID
    std::fs::write(p, "4194302\n").expect("overwrite with dead pid");
    super::daemon::prepare_daemon_pid_file(p).expect("replace dead pid");
    let pid2 = std::fs::read_to_string(p).expect("read pid2");

    assert_eq!(pid1.trim(), pid2.trim(), "both should record current pid");
}
