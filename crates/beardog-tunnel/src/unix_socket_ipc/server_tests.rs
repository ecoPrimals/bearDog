// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{JsonRpcError, JsonRpcResponse, UnixSocketIpcServer};
use crate::btsp_handshake::BtspSecurityMode;
use crate::test_helpers::mocks::create_minimal_beardog_provider;
use beardog_core::socket_config::IpcCapabilitySymlinksConfig;
use beardog_types::primal_identity::PrimalIdentity;
use std::sync::Arc;
use tempfile::tempdir;

#[tokio::test]
async fn wait_ready_flag_times_out_when_never_set() {
    let flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let ok = UnixSocketIpcServer::wait_ready_flag(&flag, std::time::Duration::from_millis(1)).await;
    assert!(!ok);
}

#[tokio::test]
async fn wait_ready_flag_succeeds_when_already_true() {
    let flag = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let ok = UnixSocketIpcServer::wait_ready_flag(&flag, std::time::Duration::from_millis(1)).await;
    assert!(ok);
}

#[tokio::test]
async fn handle_jsonrpc_rejects_non_2_0_version() {
    let dir = tempdir().expect("tempdir");
    let sock = dir.path().join("bd.sock");
    let prov = create_minimal_beardog_provider().await;
    let id = Arc::new(PrimalIdentity::for_test("fam", "node"));
    let server = UnixSocketIpcServer::new(
        &sock,
        prov,
        id,
        BtspSecurityMode::Development,
        IpcCapabilitySymlinksConfig::default(),
    )
    .await
    .expect("server");
    let resp: JsonRpcResponse = server
        .handle_jsonrpc_request(r#"{"jsonrpc":"1.0","method":"health","id":1}"#)
        .await
        .expect("parse ok");
    assert!(resp.error.is_some());
    assert_eq!(resp.error.as_ref().expect("e").code, -32600);
}

#[tokio::test]
async fn handle_jsonrpc_method_not_found_uses_reserved_code() {
    let dir = tempdir().expect("tempdir");
    let sock = dir.path().join("bd2.sock");
    let prov = create_minimal_beardog_provider().await;
    let id = Arc::new(PrimalIdentity::for_test("fam", "node"));
    let server = UnixSocketIpcServer::new(
        &sock,
        prov,
        id,
        BtspSecurityMode::Development,
        IpcCapabilitySymlinksConfig::default(),
    )
    .await
    .expect("server");
    let resp = server
        .handle_jsonrpc_request(r#"{"jsonrpc":"2.0","method":"no.such.method","id":2}"#)
        .await
        .expect("parse ok");
    let e = resp.error.expect("err");
    assert_eq!(e.code, JsonRpcError::METHOD_NOT_FOUND);
}

#[tokio::test]
async fn handle_jsonrpc_invalid_params_branch() {
    let dir = tempdir().expect("tempdir");
    let sock = dir.path().join("bd3.sock");
    let prov = create_minimal_beardog_provider().await;
    let id = Arc::new(PrimalIdentity::for_test("fam", "node"));
    let server = UnixSocketIpcServer::new(
        &sock,
        prov,
        id,
        BtspSecurityMode::Development,
        IpcCapabilitySymlinksConfig::default(),
    )
    .await
    .expect("server");
    let resp = server
        .handle_jsonrpc_request(
            r#"{"jsonrpc":"2.0","method":"crypto.hash_for_cipher","params":{},"id":3}"#,
        )
        .await
        .expect("parse ok");
    let e = resp.error.expect("err");
    assert_eq!(e.code, JsonRpcError::INVALID_PARAMS);
    assert!(e.message.contains("Missing") || e.message.contains("required"));
}

#[tokio::test]
async fn handle_jsonrpc_health_success() {
    let dir = tempdir().expect("tempdir");
    let sock = dir.path().join("bd4.sock");
    let prov = create_minimal_beardog_provider().await;
    let id = Arc::new(PrimalIdentity::for_test("fam", "node"));
    let server = UnixSocketIpcServer::new(
        &sock,
        prov,
        id,
        BtspSecurityMode::Development,
        IpcCapabilitySymlinksConfig::default(),
    )
    .await
    .expect("server");
    let resp = server
        .handle_jsonrpc_request(r#"{"jsonrpc":"2.0","method":"health","id":4}"#)
        .await
        .expect("parse ok");
    assert!(resp.error.is_none());
    assert!(resp.result.is_some());
}
