// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Integration API Server
//!
//! REST API surface for ecosystem integration with 17 endpoints for BTSP, BirdSong,
//! Lineage, and system operations. Crypto-related routes are **not yet wired** to real
//! tunnel, HSM, or genetics providers and return `501 Not Implemented` until those
//! integrations land. Non-crypto system endpoints (health, metrics, capabilities,
//! status) are functional.
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
    /// Bind address (IP or hostname). When unset, uses `BEARDOG_BIND_ADDRESS` or `127.0.0.1`.
    pub bind_address: Option<String>,
    /// Request timeout.
    pub timeout: Duration,
    /// Enable CORS.
    pub enable_cors: bool,
}

impl Default for ApiServerConfig {
    fn default() -> Self {
        Self {
            port: DEFAULT_API_SERVER_LISTEN_PORT,
            bind_address: None,
            timeout: DEFAULT_API_REQUEST_TIMEOUT,
            enable_cors: true,
        }
    }
}

fn resolve_bind_address(config: &ApiServerConfig) -> String {
    if let Some(addr) = &config.bind_address {
        return addr.clone();
    }
    std::env::var("BEARDOG_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string())
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

    let bind_host = resolve_bind_address(&config);
    let addr = format!("{bind_host}:{}", config.port);
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
    NotImplemented(String),
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
            Self::NotImplemented(msg) => (StatusCode::NOT_IMPLEMENTED, msg),
        };

        let body = Json(serde_json::json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
#[path = "api_server_tests.rs"]
mod tests;
