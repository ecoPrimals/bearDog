// BTSP HTTP API Server
//!
//! HTTP/JSON API wrapper for the BTSP provider, enabling remote access
//! from Songbird orchestrator.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │   Songbird (Remote HTTP Client)         │
//! └─────────────────────────────────────────┘
//!                    │
//!                    │ HTTP/JSON
//!                    ▼
//! ┌─────────────────────────────────────────┐
//! │   BTSP HTTP API (Axum Server)           │
//! ├─────────────────────────────────────────┤
//! │  POST /btsp/tunnel/establish            │
//! │  POST /btsp/tunnel/encrypt              │
//! │  POST /btsp/tunnel/decrypt              │
//! │  GET  /btsp/tunnel/status/:id           │
//! │  DELETE /btsp/tunnel/close/:id          │
//! └─────────────────────────────────────────┘
//!                    │
//!                    ▼
//! ┌─────────────────────────────────────────┐
//! │   BeardogBtspProvider (Core Logic)      │
//! └─────────────────────────────────────────┘
//! ```

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::btsp_provider::{
    BeardogBtspProvider, BtspProvider, Direction, PeerInfo, SecurityContext, TunnelHandle,
    TunnelStatus,
};
use beardog_errors::BearDogError;

// =============================================================================
// HTTP Request/Response Types
// =============================================================================

/// Request to establish a tunnel
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstablishTunnelRequest {
    /// Peer information
    pub peer: PeerInfo,
}

/// Response with tunnel handle
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstablishTunnelResponse {
    /// Tunnel handle
    pub handle: TunnelHandle,
}

/// Request to encrypt data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptRequest {
    /// Tunnel ID
    pub tunnel_id: String,
    /// Direction (outbound/inbound)
    pub direction: Direction,
    /// Base64-encoded plaintext
    #[serde(with = "base64_serde")]
    pub data: Vec<u8>,
}

/// Response with encrypted data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptResponse {
    /// Base64-encoded ciphertext
    #[serde(with = "base64_serde")]
    pub ciphertext: Vec<u8>,
}

/// Request to decrypt data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecryptRequest {
    /// Tunnel ID
    pub tunnel_id: String,
    /// Direction (outbound/inbound)
    pub direction: Direction,
    /// Base64-encoded ciphertext
    #[serde(with = "base64_serde")]
    pub data: Vec<u8>,
}

/// Response with decrypted data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecryptResponse {
    /// Base64-encoded plaintext
    #[serde(with = "base64_serde")]
    pub plaintext: Vec<u8>,
}

/// Error response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
    /// Error details (optional)
    pub details: Option<String>,
}

// =============================================================================
// Base64 Serialization Helper
// =============================================================================

mod base64_serde {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&STANDARD.encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        STANDARD.decode(&s).map_err(serde::de::Error::custom)
    }
}

// =============================================================================
// API Server State
// =============================================================================

/// Shared server state
#[derive(Clone)]
struct ApiState {
    /// BTSP provider
    provider: Arc<BeardogBtspProvider>,
}

// =============================================================================
// HTTP Handlers
// =============================================================================

/// Establish a new tunnel
async fn establish_tunnel_handler(
    State(state): State<ApiState>,
    Json(request): Json<EstablishTunnelRequest>,
) -> Result<Json<EstablishTunnelResponse>, ApiError> {
    info!(
        "🌉 HTTP: Establishing tunnel with peer: {}",
        request.peer.id
    );

    let handle = state.provider.establish_tunnel(&request.peer).await?;

    Ok(Json(EstablishTunnelResponse { handle }))
}

/// Encrypt data through a tunnel
async fn encrypt_handler(
    State(state): State<ApiState>,
    Json(request): Json<EncryptRequest>,
) -> Result<Json<EncryptResponse>, ApiError> {
    let context = SecurityContext {
        tunnel_id: request.tunnel_id.clone(),
        direction: request.direction,
    };

    let ciphertext = state.provider.encrypt(&request.data, &context).await?;

    Ok(Json(EncryptResponse { ciphertext }))
}

/// Decrypt data from a tunnel
async fn decrypt_handler(
    State(state): State<ApiState>,
    Json(request): Json<DecryptRequest>,
) -> Result<Json<DecryptResponse>, ApiError> {
    let context = SecurityContext {
        tunnel_id: request.tunnel_id.clone(),
        direction: request.direction,
    };

    let plaintext = state.provider.decrypt(&request.data, &context).await?;

    Ok(Json(DecryptResponse { plaintext }))
}

/// Get tunnel status
async fn status_handler(
    State(state): State<ApiState>,
    Path(tunnel_id): Path<String>,
) -> Result<Json<TunnelStatus>, ApiError> {
    let handle = TunnelHandle {
        id: tunnel_id.clone(),
        peer_id: String::new(), // Not needed for status query
        established_at: chrono::Utc::now().to_rfc3339(), // ISO 8601 format
    };

    let status = state.provider.tunnel_status(&handle).await?;

    Ok(Json(status))
}

/// Close a tunnel
async fn close_tunnel_handler(
    State(state): State<ApiState>,
    Path(tunnel_id): Path<String>,
) -> Result<StatusCode, ApiError> {
    info!("🔒 HTTP: Closing tunnel: {}", tunnel_id);

    let handle = TunnelHandle {
        id: tunnel_id.clone(),
        peer_id: String::new(), // Not needed for close
        established_at: chrono::Utc::now().to_rfc3339(), // ISO 8601 format
    };

    state.provider.close_tunnel(&handle).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Health check endpoint
async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "beardog-btsp",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

// =============================================================================
// Error Handling
// =============================================================================

/// API error wrapper
struct ApiError(BearDogError);

impl From<BearDogError> for ApiError {
    fn from(err: BearDogError) -> Self {
        Self(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // Map BearDogError to HTTP status codes
        let status = match &self.0 {
            BearDogError::Business { .. } => StatusCode::BAD_REQUEST,
            BearDogError::System { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = format!("{}", self.0);

        let body = Json(ErrorResponse {
            error: message,
            details: Some(format!("{:?}", self.0)),
        });

        (status, body).into_response()
    }
}

// =============================================================================
// Server Builder
// =============================================================================

/// BTSP HTTP API server
pub struct BtspApiServer {
    provider: Arc<BeardogBtspProvider>,
    port: u16,
}

impl BtspApiServer {
    /// Create new API server
    ///
    /// # Arguments
    ///
    /// * `provider` - BTSP provider instance
    /// * `port` - Port to listen on
    pub fn new(provider: Arc<BeardogBtspProvider>, port: u16) -> Self {
        Self { provider, port }
    }

    /// Build the API router
    fn build_router(&self) -> Router {
        let state = ApiState {
            provider: self.provider.clone(),
        };

        Router::new()
            // Health check
            .route("/health", get(health_handler))
            // BTSP endpoints
            .route("/btsp/tunnel/establish", post(establish_tunnel_handler))
            .route("/btsp/tunnel/encrypt", post(encrypt_handler))
            .route("/btsp/tunnel/decrypt", post(decrypt_handler))
            .route("/btsp/tunnel/status/:id", get(status_handler))
            .route("/btsp/tunnel/close/:id", delete(close_tunnel_handler))
            // State and middleware
            .with_state(state)
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
    }

    /// Start the API server
    ///
    /// # Errors
    ///
    /// Returns error if server fails to bind or start
    pub async fn start(self) -> Result<(), BearDogError> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let app = self.build_router();

        info!("🚀 BearDog BTSP API listening on http://{}", addr);
        info!("   Endpoints:");
        info!("   - POST   /btsp/tunnel/establish");
        info!("   - POST   /btsp/tunnel/encrypt");
        info!("   - POST   /btsp/tunnel/decrypt");
        info!("   - GET    /btsp/tunnel/status/:id");
        info!("   - DELETE /btsp/tunnel/close/:id");
        info!("   - GET    /health");

        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| BearDogError::system(format!("Failed to bind to {}: {}", addr, e)))?;

        axum::serve(listener, app)
            .await
            .map_err(|e| BearDogError::system(format!("Server error: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_serialization() {
        let error = ErrorResponse {
            error: "Test error".to_string(),
            details: Some("Details here".to_string()),
        };

        let json = serde_json::to_string(&error).expect("Serialize failed");
        assert!(json.contains("Test error"));
    }

    #[test]
    fn test_establish_request_serialization() {
        let request = EstablishTunnelRequest {
            peer: PeerInfo {
                id: "test-peer".to_string(),
                endpoint: "192.168.1.1:8080".to_string(),
                public_key: Some(vec![1, 2, 3, 4]),
            },
        };

        let json = serde_json::to_string(&request).expect("Serialize failed");
        let deserialized: EstablishTunnelRequest =
            serde_json::from_str(&json).expect("Deserialize failed");

        assert_eq!(request.peer.id, deserialized.peer.id);
    }
}
