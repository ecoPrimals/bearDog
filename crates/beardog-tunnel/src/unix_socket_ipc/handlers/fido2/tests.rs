// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for the FIDO2 IPC handler module.

use super::super::MethodHandler;
use super::*;
use serde_json::json;

#[test]
fn fido2_handler_method_list() {
    let handler = Fido2Handler::new();
    let methods = handler.methods();
    assert_eq!(methods.len(), 5);
    assert!(methods.contains(&"beardog.fido2.discover"));
    assert!(methods.contains(&"beardog.fido2.register"));
    assert!(methods.contains(&"beardog.fido2.authenticate"));
    assert!(methods.contains(&"beardog.fido2.entropy"));
    assert!(methods.contains(&"beardog.fido2.ceremony"));
}

#[tokio::test]
async fn fido2_discover_returns_valid_response() {
    let result = discover::handle_fido2_discover(None)
        .await
        .expect("discover");
    assert!(result.get("devices").is_some());
    assert!(result.get("count").is_some());
    let count = result["count"].as_u64().expect("count is u64");
    let devices = result["devices"].as_array().expect("devices is array");
    assert_eq!(devices.len() as u64, count);
}

#[tokio::test]
async fn fido2_register_requires_params() {
    let err = register::handle_fido2_register(None)
        .await
        .expect_err("no params");
    assert!(err.contains("Missing params"));
}

#[tokio::test]
async fn fido2_register_requires_rp_id() {
    let params = json!({"user_id": "dXNlcg==", "user_name": "test"});
    let err = register::handle_fido2_register(Some(&params))
        .await
        .expect_err("no rp_id");
    assert!(err.contains("rp_id"));
}

#[tokio::test]
async fn fido2_register_requires_user_id() {
    let params = json!({"rp_id": "primals.eco", "user_name": "test"});
    let err = register::handle_fido2_register(Some(&params))
        .await
        .expect_err("no user_id");
    assert!(err.contains("user_id"));
}

#[tokio::test]
async fn fido2_register_requires_user_name() {
    let params = json!({"rp_id": "primals.eco", "user_id": "dXNlcg=="});
    let err = register::handle_fido2_register(Some(&params))
        .await
        .expect_err("no user_name");
    assert!(err.contains("user_name"));
}

#[tokio::test]
async fn fido2_authenticate_requires_params() {
    let err = authenticate::handle_fido2_authenticate(None)
        .await
        .expect_err("no params");
    assert!(err.contains("Missing params"));
}

#[tokio::test]
async fn fido2_authenticate_requires_rp_id() {
    let params = json!({"credential_id": "Y3JlZA==", "challenge": "Y2hhbA=="});
    let err = authenticate::handle_fido2_authenticate(Some(&params))
        .await
        .expect_err("no rp_id");
    assert!(err.contains("rp_id"));
}

#[tokio::test]
async fn fido2_authenticate_requires_credential_id() {
    let params = json!({"rp_id": "primals.eco", "challenge": "Y2hhbA=="});
    let err = authenticate::handle_fido2_authenticate(Some(&params))
        .await
        .expect_err("no credential_id");
    assert!(err.contains("credential_id"));
}

#[tokio::test]
async fn fido2_authenticate_requires_challenge() {
    let params = json!({"rp_id": "primals.eco", "credential_id": "Y3JlZA=="});
    let err = authenticate::handle_fido2_authenticate(Some(&params))
        .await
        .expect_err("no challenge");
    assert!(err.contains("challenge"));
}

#[tokio::test]
async fn fido2_entropy_requires_params() {
    let err = entropy::handle_fido2_entropy(None)
        .await
        .expect_err("no params");
    assert!(err.contains("Missing params"));
}

#[tokio::test]
async fn fido2_entropy_requires_rp_id() {
    let params = json!({"credential_id": "Y3JlZA=="});
    let err = entropy::handle_fido2_entropy(Some(&params))
        .await
        .expect_err("no rp_id");
    assert!(err.contains("rp_id"));
}

#[tokio::test]
async fn fido2_entropy_requires_credential_id() {
    let params = json!({"rp_id": "primals.eco"});
    let err = entropy::handle_fido2_entropy(Some(&params))
        .await
        .expect_err("no credential_id");
    assert!(err.contains("credential_id"));
}

#[tokio::test]
async fn fido2_handler_routes_discover() {
    let handler = Fido2Handler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let result = handler
        .handle("beardog.fido2.discover", None, &provider)
        .await
        .expect("discover via handler");
    assert!(result.get("devices").is_some());
}

#[tokio::test]
async fn fido2_handler_unknown_method_errors() {
    let handler = Fido2Handler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let err = handler
        .handle("beardog.fido2.nonexistent", None, &provider)
        .await
        .expect_err("unknown method");
    assert!(err.contains("Unknown FIDO2 method"));
}

#[tokio::test]
async fn fido2_ceremony_requires_params() {
    let err = ceremony::handle_fido2_ceremony(None)
        .await
        .expect_err("no params");
    assert!(err.contains("Missing params"));
}

#[tokio::test]
async fn fido2_ceremony_requires_rp_id() {
    let params = json!({"credential_id": "Y3JlZA=="});
    let err = ceremony::handle_fido2_ceremony(Some(&params))
        .await
        .expect_err("no rp_id");
    assert!(err.contains("rp_id"));
}

#[tokio::test]
async fn fido2_ceremony_requires_credential_id() {
    let params = json!({"rp_id": "primals.eco"});
    let err = ceremony::handle_fido2_ceremony(Some(&params))
        .await
        .expect_err("no credential_id");
    assert!(err.contains("credential_id"));
}
