// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use axum::body::{Body, to_bytes};
use http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

fn cfg_no_cors() -> ApiServerConfig {
    ApiServerConfig {
        port: 0,
        bind_address: None,
        timeout: DEFAULT_API_REQUEST_TIMEOUT,
        enable_cors: false,
    }
}

async fn body_json(response: axum::response::Response) -> serde_json::Value {
    let raw = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("read body");
    serde_json::from_slice(&raw).expect("json body")
}

#[test]
fn test_api_state_creation() {
    let state = ApiState::default();
    let _cloned = state;
}

#[test]
fn test_config_defaults() {
    let config = ApiServerConfig::default();
    assert_eq!(config.port, DEFAULT_API_SERVER_LISTEN_PORT);
    assert_eq!(config.timeout, DEFAULT_API_REQUEST_TIMEOUT);
    assert!(config.enable_cors);
}

#[test]
fn test_tunnel_metadata() {
    let state = ApiState::default();
    let tunnel_id = "test_tunnel".to_string();

    let metadata = TunnelMetadata {
        tunnel_id: tunnel_id.clone(),
        peer_id: "test_peer".to_string(),
        created_at: SystemTime::now(),
        status: "active".to_string(),
    };

    state.tunnels.write().insert(tunnel_id.clone(), metadata);

    let tunnels = state.tunnels.read();
    assert!(tunnels.contains_key(&tunnel_id));
}

#[test]
fn test_lineage_metadata() {
    let state = ApiState::default();
    let node_id = "test_node".to_string();

    let metadata = types::LineageMetadata {
        node_id: node_id.clone(),
        parent_id: None,
        chain: vec![node_id.clone()],
        depth: 0,
        created_at: SystemTime::now(),
    };

    state.lineages.write().insert(node_id.clone(), metadata);

    let lineages = state.lineages.read();
    assert!(lineages.contains_key(&node_id));
}

#[test]
fn test_request_counter() {
    let state = ApiState::default();
    assert_eq!(*state.request_counter.read(), 0);

    state.increment_requests();
    assert_eq!(*state.request_counter.read(), 1);

    state.increment_requests();
    assert_eq!(*state.request_counter.read(), 2);
}

#[tokio::test]
async fn http_get_health_ok() {
    let app = create_router(&cfg_no_cors());
    let res = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(res.status(), StatusCode::OK);
    let v = body_json(res).await;
    assert_eq!(v["status"], "healthy");
    assert!(v["uptime_seconds"].as_u64().is_some());
}

#[tokio::test]
async fn http_get_metrics_returns_counts() {
    let app = create_router(&cfg_no_cors());
    let res = app
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(res.status(), StatusCode::OK);
    let v = body_json(res).await;
    assert_eq!(v["active_tunnels"], 0);
}

#[tokio::test]
async fn http_btsp_crypto_endpoints_return_not_implemented() {
    let app = create_router(&cfg_no_cors());

    let establish = json!({
        "responder_id": "peer-1",
        "initiator_entropy": "ent-a",
    });
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/btsp/tunnel/establish")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&establish).expect("encode")))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(res.status(), StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
        body_json(res).await["error"],
        "BTSP handshake requires tunnel provider integration"
    );

    let enc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/btsp/tunnel/tunnel-1/encrypt")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({ "plaintext": "hi" })).expect("encode"),
                ))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(enc.status(), StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
        body_json(enc).await["error"],
        "Crypto operations require AEAD provider"
    );

    let dec = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/btsp/tunnel/tunnel-1/decrypt")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({ "ciphertext": "ct" })).expect("encode"),
                ))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(dec.status(), StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
        body_json(dec).await["error"],
        "Crypto operations require AEAD provider"
    );
}

#[tokio::test]
async fn http_birdsong_crypto_endpoints_return_not_implemented() {
    let app = create_router(&cfg_no_cors());
    let enc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/birdsong/encrypt")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "payload": "secret",
                        "lineage_hint": null
                    }))
                    .expect("encode"),
                ))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(enc.status(), StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
        body_json(enc).await["error"],
        "BirdSong operations require BirdSongManager integration"
    );

    let dec = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/birdsong/decrypt")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "ciphertext": "birdsong_encrypted_secret",
                        "lineage_hint": null
                    }))
                    .expect("encode"),
                ))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(dec.status(), StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
        body_json(dec).await["error"],
        "BirdSong operations require BirdSongManager integration"
    );
}

#[tokio::test]
async fn http_birdsong_verify_lineage_returns_not_implemented() {
    let app = create_router(&cfg_no_cors());
    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/birdsong/lineage/verify")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "node_id": "n1",
                        "proof": "proof-bytes"
                    }))
                    .expect("encode"),
                ))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(res.status(), StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
        body_json(res).await["error"],
        "Lineage verification requires cryptographic proof validation"
    );
}

#[tokio::test]
async fn http_birdsong_lineage_missing_node_bad_request() {
    let app = create_router(&cfg_no_cors());
    let res = app
        .oneshot(
            Request::builder()
                .uri("/birdsong/lineage/ghost")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn http_lineage_crypto_endpoints_return_not_implemented() {
    let app = create_router(&cfg_no_cors());
    let gen_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/lineage/generate")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "node_id": "n1",
                        "parent_id": null
                    }))
                    .expect("encode"),
                ))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(gen_res.status(), StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
        body_json(gen_res).await["error"],
        "Lineage operations require HSM-backed signing"
    );

    let ver = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/lineage/verify")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({ "lineage_chain": ["n1"] })).expect("encode"),
                ))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(ver.status(), StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
        body_json(ver).await["error"],
        "Chain integrity requires ordered signature validation"
    );

    let pr = app
        .oneshot(
            Request::builder()
                .uri("/lineage/proof/n1")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(pr.status(), StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
        body_json(pr).await["error"],
        "Lineage operations require HSM-backed signing"
    );
}

#[tokio::test]
async fn http_unknown_route_404() {
    let app = create_router(&cfg_no_cors());
    let res = app
        .oneshot(
            Request::builder()
                .uri("/no/such")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[test]
fn api_error_into_response_maps_status_codes() {
    let r = ApiError::Internal("boom".to_string()).into_response();
    assert_eq!(r.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let r2 = ApiError::BadRequest("bad".to_string()).into_response();
    assert_eq!(r2.status(), StatusCode::BAD_REQUEST);
    let r3 = ApiError::NotImplemented("later".to_string()).into_response();
    assert_eq!(r3.status(), StatusCode::NOT_IMPLEMENTED);
}
