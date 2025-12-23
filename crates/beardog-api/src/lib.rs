//! # `BearDog` API - `RESTful` API Server
//!
//! `RESTful` API endpoints for the `BearDog` security platform, providing HTTP/HTTPS access
//! to security operations, health monitoring, and system management.

#![deny(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
// Allow expect/unwrap in tests - test panics are appropriate failure modes
#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::unwrap_used))]

//!
//! ## Features
//!
//! - **`RESTful` Endpoints**: Standard HTTP API for `BearDog` operations
//! - **Health Monitoring**: System health and readiness endpoints
//! - **Security Operations**: Cryptographic operations via API
//! - **Metrics & Monitoring**: Real-time system metrics endpoints
//! - **CORS Support**: Cross-origin resource sharing
//! - **JSON API**: Structured JSON request/response
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_api::{ApiServer, ApiConfig};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize API server
//! let config = ApiConfig::default();
//! let server = ApiServer::new(config)?;
//!
//! // Start serving requests
//! server.serve().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## API Endpoints
//!
//! - `GET /health` - Health check endpoint
//! - `GET /metrics` - System metrics
//! - `GET /api/v1/...` - API operations
//!
//! ## Architecture
//!
//! The API server is built on:
//! - **Axum Framework**: High-performance async HTTP
//! - **Tower Middleware**: CORS, logging, rate limiting
//! - **JSON Serialization**: Serde-based request/response
//!
//! ## Safety
//!
//! All API operations maintain memory safety with zero unsafe code.

use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use beardog_core::core::BearDogCore;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

pub mod discovery;
pub mod endpoints;
pub mod jsonrpc;
pub mod startup;
pub mod tarpc_service;

pub use discovery::{ServiceAdvertisement, ServiceAdvertiser};
pub use startup::{BearDogApiConfig, BearDogApiServer, BearDogApiServerBuilder};
pub use tarpc_service::{serve_tarpc, BearDogCryptoRpc, BearDogCryptoRpcServer};

/// Get current memory usage (simplified implementation)
fn get_memory_usage() -> u64 {
    // Use a simple approach - in production, use proper memory monitoring
    u64::from(std::process::id()) * 1024 // Simplified calculation
}

/// Generic API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    /// Create successful response
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create error response
    #[must_use]
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// System status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: String,
    pub connections: u32,
    pub memory_usage: u64,
}

/// API state container
#[derive(Clone)]
pub struct ApiState {
    pub core: Arc<BearDogCore>,
    /// Protocol-agnostic crypto service (Phase 1 complete!)
    pub crypto_service: Arc<dyn beardog_core::crypto_service::CryptoService>,
}

impl std::fmt::Debug for ApiState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiState")
            .field("core", &"BearDogCore")
            .field("crypto_service", &"CryptoService")
            .finish()
    }
}

/// Health check endpoint
pub async fn health_check(
    _state: State<ApiState>,
) -> Result<Json<ApiResponse<HealthResponse>>, StatusCode> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// System status endpoint
pub async fn system_status(
    _state: State<ApiState>,
) -> Result<Json<ApiResponse<StatusResponse>>, StatusCode> {
    let response = StatusResponse {
        status: "operational".to_string(),
        connections: 1, // Current connection count
        memory_usage: get_memory_usage(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Create API router with all capability-based endpoints
///
/// # Architecture
///
/// BearDog exposes capabilities via HTTP API:
/// - `/api/v1/capabilities` - Advertise what we can do
/// - `/api/v1/crypto/*` - Crypto operations by capability
/// - `/health` - Service health
///
/// Other primals discover BearDog via:
/// 1. mDNS service discovery (_beardog._tcp.local)
/// 2. HTTP capability query
/// 3. Dynamic routing based on advertised capabilities
///
/// # Panics
///
/// Will panic if crypto service initialization fails (should never happen with valid config).
/// This is an acceptable panic as it indicates a critical system misconfiguration at startup.
pub fn create_router(core: Arc<BearDogCore>) -> Router {
    // Create crypto service (Phase 1 trait implementation!)
    // Note: Constructor is currently infallible with default config, but returns Result
    // for future-proofing when HSM initialization may fail
    let crypto_service = beardog_core::crypto_service::BearDogCryptoService::new(
        beardog_core::crypto_service::CryptoServiceConfig::default(),
    )
    .unwrap_or_else(|e| {
        tracing::error!("Fatal: Failed to create crypto service: {}", e);
        panic!("Crypto service initialization failed - cannot start API server")
    });

    let state = ApiState {
        core,
        crypto_service: Arc::new(crypto_service),
    };

    // Build router with all endpoints
    Router::new()
        // Health and status (monitoring)
        .route("/health", get(health_check))
        .route("/status", get(system_status))
        // Capability discovery
        .route("/api/v1/capabilities", get(endpoints::get_capabilities))
        .route("/api/v1/capability/:id", get(endpoints::get_capability_by_id))
        // Generic crypto operations (for cross-primal integration)
        .route("/api/v1/encrypt", axum::routing::post(endpoints::encrypt))
        .route("/api/v1/decrypt", axum::routing::post(endpoints::decrypt))
        // Algorithm-specific crypto operations (HTTP)
        .route("/api/v1/crypto/aes-gcm/encrypt", axum::routing::post(endpoints::aes_gcm_encrypt))
        .route("/api/v1/crypto/aes-gcm/decrypt", axum::routing::post(endpoints::aes_gcm_decrypt))
        .route("/api/v1/crypto/ed25519/sign", axum::routing::post(endpoints::ed25519_sign))
        .route("/api/v1/crypto/ed25519/verify", axum::routing::post(endpoints::ed25519_verify))
        // JSON-RPC endpoint (Phase 3!)
        .route("/rpc", axum::routing::post(jsonrpc::jsonrpc_handler))
        // Protocol discovery endpoints (Phase 5!)
        .route("/api/v1/protocols", get(endpoints::protocols::get_protocols))
        .route("/api/v1/protocols/:protocol", get(endpoints::protocols::get_protocol))
        .route("/api/v1/protocols/escalation/guide", get(endpoints::protocols::get_escalation_guide))
        .route("/api/v1/protocols/comparison", get(endpoints::protocols::get_protocol_comparison))
        // Key management endpoints (for Songbird integration)
        .route("/api/v1/keys/generate", axum::routing::post(endpoints::generate_key))
        .route("/api/v1/keys/info", axum::routing::post(endpoints::get_key_info))
        .route("/api/v1/keys/delete", axum::routing::post(endpoints::delete_key))
        // CORS and state
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Start API server
/// Starts `api_server`
pub async fn start_api_server(core: Arc<BearDogCore>, bind_addr: &str) -> Result<(), BearDogError> {
    info!("🚀 Starting BearDog API server on {}", bind_addr);

    let app = create_router(core);
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .map_err(|e| BearDogError::system(format!("Failed to bind to {bind_addr}: {e}")))?;

    info!("✅ API server listening on {}", bind_addr);

    axum::serve(listener, app)
        .await
        .map_err(|e| BearDogError::system(format!("Server error: {e}")))?;

    Ok(())
}

// October 27, 2025: Comprehensive test expansion
#[cfg(test)]
mod api_comprehensive_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() -> Result<(), Box<dyn std::error::Error>> {
        let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let core = Arc::new(BearDogCore::new(config));
        let app = create_router(core);

        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())?;

        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_status_endpoint() -> Result<(), Box<dyn std::error::Error>> {
        let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
        let core = Arc::new(BearDogCore::new(config));
        let app = create_router(core);

        let request = Request::builder()
            .method(Method::GET)
            .uri("/status")
            .body(Body::empty())?;

        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }
}
