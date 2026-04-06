// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Integration API Server
//!
//! Expanded REST API with 17 endpoints for BTSP, BirdSong, and Lineage operations.
//!
//! ## Modern Axum Patterns
//! - Type-safe extractors
//! - Middleware composition
//! - Connection pooling via Tower
//! - Graceful shutdown
//! - Request tracing
//!
//! ## Endpoints (17 total)
//! - 6 BTSP (secure tunnels)
//! - 4 BirdSong (privacy-preserving broadcasts)
//! - 3 Lineage (genetic lineage management)
//! - 4 System (health, metrics, capabilities, existing)

mod handlers;
pub(crate) mod types;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use axum::{
    Router,
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Json, Response},
    routing::{delete, get, post},
};
use parking_lot::RwLock;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

use beardog_errors::BearDogError;
use beardog_types::constants::domains::timeouts::HTTP_REQUEST_TIMEOUT;

use crate::connection_tracker::ActiveConnectionGuard;

/// Default listen port (matches [`crate::DEFAULT_INTEGRATION_API_PORT`]).
pub const DEFAULT_API_SERVER_LISTEN_PORT: u16 = crate::DEFAULT_INTEGRATION_API_PORT;

/// Default per-request handler timeout (centralized HTTP API default).
pub const DEFAULT_API_REQUEST_TIMEOUT: Duration = HTTP_REQUEST_TIMEOUT;

// ── Shared state ────────────────────────────────────────────────────────────

/// API server state (shared across handlers via `Arc` clone-on-extract).
#[derive(Clone)]
pub struct ApiState {
    /// Active tunnel sessions (tunnel_id -> metadata).
    tunnels: Arc<RwLock<HashMap<String, TunnelMetadata>>>,
    /// Lineage chains (chain_id -> metadata).
    lineages: Arc<RwLock<HashMap<String, types::LineageMetadata>>>,
    /// Request counter for metrics.
    request_counter: Arc<RwLock<u64>>,
    /// Server start time.
    start_time: SystemTime,
}

#[derive(Debug, Clone)]
struct TunnelMetadata {
    tunnel_id: String,
    peer_id: String,
    created_at: SystemTime,
    status: String,
}

impl ApiState {
    /// Create new API state.
    pub fn new() -> Self {
        Self {
            tunnels: Arc::new(RwLock::new(HashMap::new())),
            lineages: Arc::new(RwLock::new(HashMap::new())),
            request_counter: Arc::new(RwLock::new(0)),
            start_time: SystemTime::now(),
        }
    }

    fn increment_requests(&self) {
        let mut counter = self.request_counter.write();
        *counter += 1;
    }
}

impl Default for ApiState {
    fn default() -> Self {
        Self::new()
    }
}

/// API server configuration.
#[derive(Debug, Clone)]
pub struct ApiServerConfig {
    /// Port to bind to.
    pub port: u16,
    /// Request timeout.
    pub timeout: Duration,
    /// Enable CORS.
    pub enable_cors: bool,
}

impl Default for ApiServerConfig {
    fn default() -> Self {
        Self {
            port: DEFAULT_API_SERVER_LISTEN_PORT,
            timeout: DEFAULT_API_REQUEST_TIMEOUT,
            enable_cors: true,
        }
    }
}

// ── Router ──────────────────────────────────────────────────────────────────

async fn track_in_flight_requests(request: Request, next: Next) -> Response {
    let _guard = ActiveConnectionGuard::new();
    next.run(request).await
}

fn create_router(config: &ApiServerConfig) -> Router {
    let state = ApiState::default();

    let app = Router::new()
        // BTSP Endpoints (6)
        .route("/btsp/tunnel/establish", post(handlers::btsp_establish_tunnel))
        .route("/btsp/tunnel/:id/encrypt", post(handlers::btsp_encrypt))
        .route("/btsp/tunnel/:id/decrypt", post(handlers::btsp_decrypt))
        .route("/btsp/tunnel/:id/status", get(handlers::btsp_tunnel_status))
        .route("/btsp/tunnel/:id", delete(handlers::btsp_close_tunnel))
        .route("/health", get(handlers::health_check))
        // BirdSong Endpoints (4)
        .route("/birdsong/encrypt", post(handlers::birdsong_encrypt))
        .route("/birdsong/decrypt", post(handlers::birdsong_decrypt))
        .route("/birdsong/lineage/:node_id", get(handlers::birdsong_get_lineage))
        .route("/birdsong/lineage/verify", post(handlers::birdsong_verify_lineage))
        // Lineage Endpoints (3)
        .route("/lineage/generate", post(handlers::lineage_generate))
        .route("/lineage/verify", post(handlers::lineage_verify))
        .route("/lineage/proof/:node_id", get(handlers::lineage_get_proof))
        // System Endpoints (4)
        .route("/metrics", get(handlers::get_metrics))
        .route("/capabilities", get(handlers::get_capabilities))
        .route("/status", get(handlers::get_status))
        .with_state(state)
        .layer(middleware::from_fn(track_in_flight_requests));

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(config.timeout));

    let app = app.layer(middleware_stack);

    if config.enable_cors {
        app.layer(CorsLayer::permissive())
    } else {
        app
    }
}

/// Start the API server with graceful shutdown.
///
/// # Errors
/// Returns [`BearDogError`] if the TCP listener fails to bind or the server encounters a fatal error.
pub async fn start_api_server(config: ApiServerConfig) -> Result<(), BearDogError> {
    info!(port = config.port, "Starting integration API server");

    let app = create_router(&config);

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| BearDogError::network(format!("Failed to bind to {addr}: {e}")))?;

    info!(addr = %addr, "API server listening");

    axum::serve(listener, app)
        .await
        .map_err(|e| BearDogError::network(format!("Server error: {e}")))?;

    Ok(())
}

// ── Error handling ──────────────────────────────────────────────────────────

/// API error type with HTTP status mapping.
#[derive(Debug)]
enum ApiError {
    Internal(String),
    BadRequest(String),
}

impl From<BearDogError> for ApiError {
    fn from(err: BearDogError) -> Self {
        Self::Internal(err.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(serde_json::json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use http::{Request, StatusCode};
    use serde_json::json;
    use tower::ServiceExt;

    fn cfg_no_cors() -> ApiServerConfig {
        ApiServerConfig {
            port: 0,
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
    async fn http_btsp_establish_encrypt_decrypt_status_close() {
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
        assert_eq!(res.status(), StatusCode::OK);
        let v = body_json(res).await;
        let tid = v["tunnel_id"].as_str().expect("tunnel id").to_string();

        let enc = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/btsp/tunnel/{tid}/encrypt"))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({ "plaintext": "hi" })).expect("encode"),
                    ))
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(enc.status(), StatusCode::OK);
        let enc_v = body_json(enc).await;
        let ct = enc_v["ciphertext"].as_str().expect("ct");

        let dec = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/btsp/tunnel/{tid}/decrypt"))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({ "ciphertext": ct })).expect("encode"),
                    ))
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(dec.status(), StatusCode::OK);

        let st = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/btsp/tunnel/{tid}/status"))
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(st.status(), StatusCode::OK);

        let del = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(format!("/btsp/tunnel/{tid}"))
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(del.status(), StatusCode::OK);
        assert_eq!(body_json(del).await["success"], true);
    }

    #[tokio::test]
    async fn http_btsp_encrypt_missing_tunnel_bad_request() {
        let app = create_router(&cfg_no_cors());
        let res = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/btsp/tunnel/missing/encrypt")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({ "plaintext": "x" })).expect("encode"),
                    ))
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn http_birdsong_encrypt_then_decrypt_roundtrip() {
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
        assert_eq!(enc.status(), StatusCode::OK);
        let ev = body_json(enc).await;
        let ct = ev["ciphertext"].as_str().expect("ciphertext");

        let dec = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/birdsong/decrypt")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "ciphertext": ct,
                            "lineage_hint": null
                        }))
                        .expect("encode"),
                    ))
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(dec.status(), StatusCode::OK);
        assert_eq!(body_json(dec).await["payload"], "secret");
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
    async fn http_lineage_generate_verify_proof_happy_path() {
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
        assert_eq!(gen_res.status(), StatusCode::OK);

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
        assert_eq!(ver.status(), StatusCode::OK);
        assert_eq!(body_json(ver).await["valid"], true);

        let pr = app
            .oneshot(
                Request::builder()
                    .uri("/lineage/proof/n1")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(pr.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn http_lineage_proof_missing_bad_request() {
        let app = create_router(&cfg_no_cors());
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/lineage/proof/absent")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
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
    }
}
