// SPDX-License-Identifier: AGPL-3.0-only

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

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{delete, get, post},
    Router,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

use beardog_errors::BearDogError;

/// API server state (shared across handlers)
///
/// ## Concurrency
/// - Wrapped in `Arc` for cheap cloning
/// - Interior mutability via sync primitives where needed
/// - Clone-on-Arc pattern for handlers
///
/// ## Note
/// For this phase, we use simplified in-memory state.
/// Production would use actual provider instances.
#[derive(Clone)]
pub struct ApiState {
    /// Active tunnel sessions (tunnel_id -> metadata)
    tunnels: Arc<RwLock<HashMap<String, TunnelMetadata>>>,
    /// Lineage chains (chain_id -> metadata)
    lineages: Arc<RwLock<HashMap<String, LineageMetadata>>>,
    /// Request counter for metrics
    request_counter: Arc<RwLock<u64>>,
    /// Server start time
    start_time: SystemTime,
}

#[derive(Debug, Clone)]
struct TunnelMetadata {
    tunnel_id: String,
    peer_id: String,
    created_at: SystemTime,
    status: String,
}

#[derive(Debug, Clone)]
struct LineageMetadata {
    node_id: String,
    parent_id: Option<String>,
    chain: Vec<String>,
    depth: u32,
    created_at: SystemTime,
}

impl ApiState {
    /// Create new API state
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

/// API server configuration
#[derive(Debug, Clone)]
pub struct ApiServerConfig {
    /// Port to bind to
    pub port: u16,
    /// Request timeout
    pub timeout: Duration,
    /// Enable CORS
    pub enable_cors: bool,
}

impl Default for ApiServerConfig {
    fn default() -> Self {
        Self {
            port: 9000,
            timeout: Duration::from_secs(30),
            enable_cors: true,
        }
    }
}

/// Create the API router with all endpoints
///
/// ## Middleware Stack
/// - TraceLayer: Request/response logging
/// - TimeoutLayer: Per-request timeout
/// - CorsLayer: Cross-origin support (if enabled)
fn create_router(config: &ApiServerConfig) -> Router {
    let state = ApiState::default();

    let app = Router::new()
        // ===== BTSP Endpoints (6) =====
        .route("/btsp/tunnel/establish", post(btsp_establish_tunnel))
        .route("/btsp/tunnel/:id/encrypt", post(btsp_encrypt))
        .route("/btsp/tunnel/:id/decrypt", post(btsp_decrypt))
        .route("/btsp/tunnel/:id/status", get(btsp_tunnel_status))
        .route("/btsp/tunnel/:id", delete(btsp_close_tunnel))
        .route("/health", get(health_check))
        // ===== BirdSong Endpoints (4) =====
        .route("/birdsong/encrypt", post(birdsong_encrypt))
        .route("/birdsong/decrypt", post(birdsong_decrypt))
        .route("/birdsong/lineage/:node_id", get(birdsong_get_lineage))
        .route("/birdsong/lineage/verify", post(birdsong_verify_lineage))
        // ===== Lineage Endpoints (3) =====
        .route("/lineage/generate", post(lineage_generate))
        .route("/lineage/verify", post(lineage_verify))
        .route("/lineage/proof/:node_id", get(lineage_get_proof))
        // ===== System Endpoints (4) =====
        .route("/metrics", get(get_metrics))
        .route("/capabilities", get(get_capabilities))
        .route("/status", get(get_status))
        .with_state(state);

    // Apply middleware
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(config.timeout));

    let app = app.layer(middleware);

    // Add CORS if enabled
    if config.enable_cors {
        app.layer(CorsLayer::permissive())
    } else {
        app
    }
}

/// Start the API server
///
/// ## Graceful Shutdown
/// Handles SIGTERM/SIGINT for clean shutdown
pub async fn start_api_server(config: ApiServerConfig) -> Result<(), BearDogError> {
    info!(port = config.port, "🌐 Starting integration API server");

    let app = create_router(&config);

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| BearDogError::network(format!("Failed to bind to {}: {}", addr, e)))?;

    info!(addr = %addr, "✅ API server listening");

    axum::serve(listener, app)
        .await
        .map_err(|e| BearDogError::network(format!("Server error: {}", e)))?;

    Ok(())
}

// ============================================================================
// BTSP Endpoints (6) - Secure Tunnel Operations
// ============================================================================

/// Establish a new BTSP tunnel
///
/// Creates a secure tunnel with genetic key exchange.
/// In production, this would call BeardogBtspProvider.
async fn btsp_establish_tunnel(
    State(state): State<ApiState>,
    Json(req): Json<BtspEstablishRequest>,
) -> Result<Json<BtspEstablishResponse>, ApiError> {
    state.increment_requests();
    info!(responder = %req.responder_id, "BTSP: Establishing tunnel");

    // Generate tunnel ID
    let tunnel_id = format!("tunnel_{}", uuid::Uuid::new_v4());

    // Store tunnel metadata
    let metadata = TunnelMetadata {
        tunnel_id: tunnel_id.clone(),
        peer_id: req.responder_id.clone(),
        created_at: SystemTime::now(),
        status: "established".to_string(),
    };
    state.tunnels.write().insert(tunnel_id.clone(), metadata);

    // In production: Call BeardogBtspProvider::establish_tunnel()
    // let handle = btsp_provider.establish_tunnel(peer_endpoint, None).await?;

    Ok(Json(BtspEstablishResponse {
        tunnel_id,
        responder_entropy: format!("entropy_{}", req.initiator_entropy),
    }))
}

/// Encrypt data through BTSP tunnel
///
/// Uses genetic cryptography for secure encryption.
/// In production, this would call BeardogBtspProvider::encrypt_data().
async fn btsp_encrypt(
    State(state): State<ApiState>,
    Path(tunnel_id): Path<String>,
    Json(req): Json<BtspEncryptRequest>,
) -> Result<Json<BtspEncryptResponse>, ApiError> {
    state.increment_requests();
    info!(tunnel_id = %tunnel_id, "BTSP: Encrypting data");

    // Verify tunnel exists
    let tunnels = state.tunnels.read();
    if !tunnels.contains_key(&tunnel_id) {
        return Err(ApiError::BadRequest("Tunnel not found".to_string()));
    }

    // In production: Call BeardogBtspProvider::encrypt_data()
    // let ciphertext = btsp_provider.encrypt_data(tunnel_id, plaintext).await?;

    // Simulate encryption
    let ciphertext = format!("encrypted_{}", req.plaintext);

    Ok(Json(BtspEncryptResponse { ciphertext }))
}

/// Decrypt data from BTSP tunnel
///
/// Uses genetic cryptography for secure decryption.
/// In production, this would call BeardogBtspProvider::decrypt_data().
async fn btsp_decrypt(
    State(state): State<ApiState>,
    Path(tunnel_id): Path<String>,
    Json(req): Json<BtspDecryptRequest>,
) -> Result<Json<BtspDecryptResponse>, ApiError> {
    state.increment_requests();
    info!(tunnel_id = %tunnel_id, "BTSP: Decrypting data");

    // Verify tunnel exists
    let tunnels = state.tunnels.read();
    if !tunnels.contains_key(&tunnel_id) {
        return Err(ApiError::BadRequest("Tunnel not found".to_string()));
    }

    // In production: Call BeardogBtspProvider::decrypt_data()
    // let plaintext = btsp_provider.decrypt_data(tunnel_id, ciphertext).await?;

    // Simulate decryption
    let plaintext = req
        .ciphertext
        .strip_prefix("encrypted_")
        .unwrap_or(&req.ciphertext)
        .to_string();

    Ok(Json(BtspDecryptResponse { plaintext }))
}

/// Get BTSP tunnel status
///
/// Returns current tunnel status and metadata.
async fn btsp_tunnel_status(
    State(state): State<ApiState>,
    Path(tunnel_id): Path<String>,
) -> Result<Json<BtspTunnelStatus>, ApiError> {
    state.increment_requests();
    info!(tunnel_id = %tunnel_id, "BTSP: Getting tunnel status");

    let tunnels = state.tunnels.read();
    let metadata = tunnels
        .get(&tunnel_id)
        .ok_or_else(|| ApiError::BadRequest("Tunnel not found".to_string()))?;

    Ok(Json(BtspTunnelStatus {
        tunnel_id: metadata.tunnel_id.clone(),
        status: metadata.status.clone(),
        created_at: format!("{:?}", metadata.created_at),
    }))
}

/// Close BTSP tunnel
///
/// Gracefully closes the tunnel and cleans up resources.
async fn btsp_close_tunnel(
    State(state): State<ApiState>,
    Path(tunnel_id): Path<String>,
) -> Result<Json<BtspCloseResponse>, ApiError> {
    state.increment_requests();
    info!(tunnel_id = %tunnel_id, "BTSP: Closing tunnel");

    let mut tunnels = state.tunnels.write();
    let existed = tunnels.remove(&tunnel_id).is_some();

    // In production: Call BeardogBtspProvider::close_tunnel()
    // btsp_provider.close_tunnel(tunnel_id).await?;

    Ok(Json(BtspCloseResponse { success: existed }))
}

// ============================================================================
// BirdSong Endpoints (4) - Privacy-Preserving Broadcasts
// ============================================================================

/// Encrypt a broadcast for lineage-based access
///
/// Uses BirdSong encryption for privacy-preserving broadcasts.
/// In production, this would call BirdSongManager::encrypt_broadcast().
async fn birdsong_encrypt(
    State(state): State<ApiState>,
    Json(req): Json<BirdSongEncryptRequest>,
) -> Result<Json<BirdSongEncryptResponse>, ApiError> {
    state.increment_requests();
    info!(lineage_hint = ?req.lineage_hint, "BirdSong: Encrypting broadcast");

    // In production: Call BirdSongManager::encrypt_broadcast()
    // let broadcast = birdsong_manager.encrypt_broadcast(encrypt_req).await?;

    // Simulate encryption
    let ciphertext = format!("birdsong_encrypted_{}", req.payload);
    let metadata = serde_json::json!({
        "lineage_hint": req.lineage_hint,
        "encrypted_at": chrono::Utc::now().to_rfc3339(),
    });

    Ok(Json(BirdSongEncryptResponse {
        ciphertext,
        metadata,
    }))
}

/// Decrypt a lineage-gated broadcast
///
/// Decrypts BirdSong broadcast if lineage requirements are met.
/// In production, this would call BirdSongManager::decrypt_broadcast().
async fn birdsong_decrypt(
    State(state): State<ApiState>,
    Json(req): Json<BirdSongDecryptRequest>,
) -> Result<Json<BirdSongDecryptResponse>, ApiError> {
    state.increment_requests();
    info!("BirdSong: Decrypting broadcast");

    // In production: Call BirdSongManager::decrypt_broadcast()
    // let plaintext = birdsong_manager.decrypt_broadcast(decrypt_req).await?;

    // Simulate decryption
    let payload = req
        .ciphertext
        .strip_prefix("birdsong_encrypted_")
        .unwrap_or(&req.ciphertext)
        .to_string();

    Ok(Json(BirdSongDecryptResponse { payload }))
}

/// Get lineage information for a node
///
/// Returns lineage chain and metadata for a node.
async fn birdsong_get_lineage(
    State(state): State<ApiState>,
    Path(node_id): Path<String>,
) -> Result<Json<LineageInfo>, ApiError> {
    state.increment_requests();
    info!(node_id = %node_id, "BirdSong: Getting lineage");

    let lineages = state.lineages.read();
    let metadata = lineages
        .get(&node_id)
        .ok_or_else(|| ApiError::BadRequest("Lineage not found".to_string()))?;

    Ok(Json(LineageInfo {
        node_id: metadata.node_id.clone(),
        lineage_chain: metadata.chain.clone(),
        depth: metadata.depth,
    }))
}

/// Verify a lineage proof
///
/// Cryptographically verifies lineage proof.
/// In production, this would call LineageProofManager::verify_proof().
async fn birdsong_verify_lineage(
    State(state): State<ApiState>,
    Json(req): Json<VerifyLineageRequest>,
) -> Result<Json<VerifyLineageResponse>, ApiError> {
    state.increment_requests();
    info!(node_id = %req.node_id, "BirdSong: Verifying lineage proof");

    // In production: Call LineageProofManager::verify_proof()
    // let result = proof_manager.verify_proof(proof).await?;

    // Simplified verification
    let lineages = state.lineages.read();
    let valid = lineages.contains_key(&req.node_id);

    Ok(Json(VerifyLineageResponse {
        valid,
        details: if valid {
            Some("Lineage proof verified".to_string())
        } else {
            Some("Lineage not found".to_string())
        },
    }))
}

// ============================================================================
// Lineage Endpoints (3) - Genetic Lineage Management
// ============================================================================

/// Generate a new lineage chain
///
/// Creates a new lineage chain with optional parent.
/// In production, this would call LineageChainManager::generate_root_chain()
/// or add_child_node().
async fn lineage_generate(
    State(state): State<ApiState>,
    Json(req): Json<GenerateLineageRequest>,
) -> Result<Json<GenerateLineageResponse>, ApiError> {
    state.increment_requests();
    info!(node_id = %req.node_id, parent = ?req.parent_id, "Lineage: Generating");

    // Build lineage chain
    let mut chain = Vec::new();
    let mut depth = 0;

    if let Some(parent_id) = &req.parent_id {
        let lineages = state.lineages.read();
        if let Some(parent) = lineages.get(parent_id) {
            chain.extend_from_slice(&parent.chain);
            depth = parent.depth + 1;
        }
    }

    chain.push(req.node_id.clone());

    // In production: Call LineageChainManager::generate_root_chain()
    // let chain = lineage_manager.generate_root_chain(node_id, metadata).await?;

    // Store lineage
    let metadata = LineageMetadata {
        node_id: req.node_id.clone(),
        parent_id: req.parent_id.clone(),
        chain: chain.clone(),
        depth,
        created_at: SystemTime::now(),
    };
    state.lineages.write().insert(req.node_id.clone(), metadata);

    let signature = format!("sig_{}", uuid::Uuid::new_v4());

    Ok(Json(GenerateLineageResponse {
        lineage_chain: chain,
        signature,
    }))
}

/// Verify a lineage chain
///
/// Cryptographically verifies lineage chain integrity.
/// In production, this would call LineageChainManager::verify_chain().
async fn lineage_verify(
    State(state): State<ApiState>,
    Json(req): Json<VerifyLineageChainRequest>,
) -> Result<Json<VerifyLineageChainResponse>, ApiError> {
    state.increment_requests();
    info!("Lineage: Verifying chain");

    // In production: Call LineageChainManager::verify_chain()
    // let valid = lineage_manager.verify_chain(chain_id).await?;

    // Simplified verification: Check if chain exists
    let lineages = state.lineages.read();
    let valid = req
        .lineage_chain
        .iter()
        .any(|node_id| lineages.contains_key(node_id));

    Ok(Json(VerifyLineageChainResponse { valid }))
}

/// Get cryptographic proof for a lineage
///
/// Generates merkle proof for lineage verification.
/// In production, this would call LineageProofManager::generate_proof().
async fn lineage_get_proof(
    State(state): State<ApiState>,
    Path(node_id): Path<String>,
) -> Result<Json<LineageProof>, ApiError> {
    state.increment_requests();
    info!(node_id = %node_id, "Lineage: Getting proof");

    let lineages = state.lineages.read();
    if !lineages.contains_key(&node_id) {
        return Err(ApiError::BadRequest("Lineage not found".to_string()));
    }

    // In production: Call LineageProofManager::generate_proof()
    // let proof = proof_manager.generate_proof(node_id).await?;

    // Generate simplified merkle proof
    let merkle_proof = vec![
        format!("hash1_{}", node_id),
        format!("hash2_{}", node_id),
        format!("root_{}", node_id),
    ];

    let signature = format!("proof_sig_{}", uuid::Uuid::new_v4());

    Ok(Json(LineageProof {
        merkle_proof,
        signature,
    }))
}

// ============================================================================
// System Endpoints (4) - Health, Metrics, Capabilities
// ============================================================================

/// Health check endpoint
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0, // Updated by get_status
    })
}

/// Get service metrics
async fn get_metrics(State(state): State<ApiState>) -> Json<MetricsResponse> {
    let tunnels_count = state.tunnels.read().len() as u32;
    let total_requests = *state.request_counter.read();
    let uptime = state
        .start_time
        .elapsed()
        .unwrap_or(Duration::from_secs(0))
        .as_secs();

    Json(MetricsResponse {
        active_tunnels: tunnels_count,
        total_requests,
        uptime_seconds: uptime,
    })
}

/// Get service capabilities
async fn get_capabilities() -> Json<CapabilitiesResponse> {
    Json(CapabilitiesResponse {
        capabilities: vec![
            "btsp".to_string(),
            "birdsong".to_string(),
            "lineage".to_string(),
            "encryption".to_string(),
            "genetic_crypto".to_string(),
        ],
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Get detailed service status
async fn get_status(State(state): State<ApiState>) -> Json<StatusResponse> {
    let uptime = state
        .start_time
        .elapsed()
        .unwrap_or(Duration::from_secs(0))
        .as_secs();

    Json(StatusResponse {
        status: "operational".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
        active_tunnels: state.tunnels.read().len() as u32,
        active_lineages: state.lineages.read().len() as u32,
        total_requests: *state.request_counter.read(),
    })
}

// ============================================================================
// Request/Response Types
// ============================================================================

// BTSP types
#[derive(Debug, Deserialize)]
struct BtspEstablishRequest {
    responder_id: String,
    initiator_entropy: String,
}

#[derive(Debug, Serialize)]
struct BtspEstablishResponse {
    tunnel_id: String,
    responder_entropy: String,
}

#[derive(Debug, Deserialize)]
struct BtspEncryptRequest {
    plaintext: String,
}

#[derive(Debug, Serialize)]
struct BtspEncryptResponse {
    ciphertext: String,
}

#[derive(Debug, Deserialize)]
struct BtspDecryptRequest {
    ciphertext: String,
}

#[derive(Debug, Serialize)]
struct BtspDecryptResponse {
    plaintext: String,
}

#[derive(Debug, Serialize)]
struct BtspTunnelStatus {
    tunnel_id: String,
    status: String,
    created_at: String,
}

#[derive(Debug, Serialize)]
struct BtspCloseResponse {
    success: bool,
}

// BirdSong types
#[derive(Debug, Deserialize)]
struct BirdSongEncryptRequest {
    payload: String,
    lineage_hint: Option<String>,
}

#[derive(Debug, Serialize)]
struct BirdSongEncryptResponse {
    ciphertext: String,
    metadata: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct BirdSongDecryptRequest {
    ciphertext: String,
    lineage_hint: Option<String>,
}

#[derive(Debug, Serialize)]
struct BirdSongDecryptResponse {
    payload: String,
}

#[derive(Debug, Serialize)]
struct LineageInfo {
    node_id: String,
    lineage_chain: Vec<String>,
    depth: u32,
}

#[derive(Debug, Deserialize)]
struct VerifyLineageRequest {
    proof: String,
    node_id: String,
}

#[derive(Debug, Serialize)]
struct VerifyLineageResponse {
    valid: bool,
    details: Option<String>,
}

// Lineage types
#[derive(Debug, Deserialize)]
struct GenerateLineageRequest {
    node_id: String,
    parent_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct GenerateLineageResponse {
    lineage_chain: Vec<String>,
    signature: String,
}

#[derive(Debug, Deserialize)]
struct VerifyLineageChainRequest {
    lineage_chain: Vec<String>,
}

#[derive(Debug, Serialize)]
struct VerifyLineageChainResponse {
    valid: bool,
}

#[derive(Debug, Serialize)]
struct LineageProof {
    merkle_proof: Vec<String>,
    signature: String,
}

// System types
#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    uptime_seconds: u64,
}

#[derive(Debug, Serialize)]
struct MetricsResponse {
    active_tunnels: u32,
    total_requests: u64,
    uptime_seconds: u64,
}

#[derive(Debug, Serialize)]
struct CapabilitiesResponse {
    capabilities: Vec<String>,
    version: String,
}

#[derive(Debug, Serialize)]
struct StatusResponse {
    status: String,
    version: String,
    uptime_seconds: u64,
    active_tunnels: u32,
    active_lineages: u32,
    total_requests: u64,
}

// ============================================================================
// Error Handling
// ============================================================================

/// API error type
#[derive(Debug)]
enum ApiError {
    Internal(String),
    BadRequest(String),
}

impl From<BearDogError> for ApiError {
    fn from(err: BearDogError) -> Self {
        ApiError::Internal(err.to_string())
    }
}

/// Convert ApiError to HTTP response
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(serde_json::json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_state_creation() {
        let state = ApiState::default();
        // State should be cloneable (Arc-based)
        let _cloned = state.clone();
    }

    #[test]
    fn test_config_defaults() {
        let config = ApiServerConfig::default();
        assert_eq!(config.port, 9000);
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

        let metadata = LineageMetadata {
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
}
