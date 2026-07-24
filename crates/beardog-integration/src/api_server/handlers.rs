// SPDX-License-Identifier: AGPL-3.0-or-later

//! Domain handlers for the integration REST API.
//!
//! Secondary HTTP surface for ecosystem tooling. The primary interface is the
//! UDS JSON-RPC server (`crypto.*`, `btsp.*`, `lineage.*`), which has full
//! implementations. These REST handlers provide an HTTP bridge; crypto-related
//! endpoints return `501 Not Implemented` until REST→UDS forwarding is wired.
//! System endpoints (health, metrics, capabilities, status) and in-memory
//! tunnel/lineage lookups are functional.

use std::time::Duration;

use axum::{
    extract::{Path, State},
    response::Json,
};
use tracing::info;

use super::types::{
    BirdSongDecryptRequest, BirdSongDecryptResponse, BirdSongEncryptRequest,
    BirdSongEncryptResponse, BtspCloseResponse, BtspDecryptRequest, BtspDecryptResponse,
    BtspEncryptRequest, BtspEncryptResponse, BtspEstablishRequest, BtspEstablishResponse,
    BtspTunnelStatus, CapabilitiesResponse, GenerateLineageRequest, GenerateLineageResponse,
    HealthResponse, LineageInfo, LineageProof, MetricsResponse, StatusResponse,
    VerifyLineageChainRequest, VerifyLineageChainResponse, VerifyLineageRequest,
    VerifyLineageResponse,
};
use super::{ApiError, ApiState};

// ── BTSP ────────────────────────────────────────────────────────────────────

/// Establish a new BTSP tunnel.
///
/// Requires tunnel provider integration (not yet wired).
pub(crate) async fn btsp_establish_tunnel(
    State(state): State<ApiState>,
    Json(req): Json<BtspEstablishRequest>,
) -> Result<Json<BtspEstablishResponse>, ApiError> {
    state.increment_requests();
    info!(responder = %req.responder_id, "BTSP: Establishing tunnel");

    Err(ApiError::NotImplemented(
        "BTSP handshake requires tunnel provider integration".to_string(),
    ))
}

/// Encrypt data through a BTSP tunnel.
///
/// Requires AEAD provider integration (not yet wired).
pub(crate) async fn btsp_encrypt(
    State(state): State<ApiState>,
    Path(tunnel_id): Path<String>,
    Json(req): Json<BtspEncryptRequest>,
) -> Result<Json<BtspEncryptResponse>, ApiError> {
    state.increment_requests();
    info!(tunnel_id = %tunnel_id, plaintext_len = req.plaintext.len(), "BTSP: Encrypting data");

    Err(ApiError::NotImplemented(
        "Crypto operations require AEAD provider".to_string(),
    ))
}

/// Decrypt data from a BTSP tunnel.
///
/// Requires AEAD provider integration (not yet wired).
pub(crate) async fn btsp_decrypt(
    State(state): State<ApiState>,
    Path(tunnel_id): Path<String>,
    Json(req): Json<BtspDecryptRequest>,
) -> Result<Json<BtspDecryptResponse>, ApiError> {
    state.increment_requests();
    info!(
        tunnel_id = %tunnel_id,
        ciphertext_len = req.ciphertext.len(),
        "BTSP: Decrypting data"
    );

    Err(ApiError::NotImplemented(
        "Crypto operations require AEAD provider".to_string(),
    ))
}

/// Get BTSP tunnel status.
pub(crate) async fn btsp_tunnel_status(
    State(state): State<ApiState>,
    Path(tunnel_id): Path<String>,
) -> Result<Json<BtspTunnelStatus>, ApiError> {
    state.increment_requests();
    info!(tunnel_id = %tunnel_id, "BTSP: Getting tunnel status");

    let tunnels = state.tunnels.read();
    let metadata = tunnels
        .get(&tunnel_id)
        .ok_or_else(|| ApiError::BadRequest("Tunnel not found".to_string()))?;

    info!(peer_id = %metadata.peer_id, "BTSP: Tunnel metadata");

    Ok(Json(BtspTunnelStatus {
        tunnel_id: metadata.tunnel_id.clone(),
        status: metadata.status.clone(),
        created_at: format!("{:?}", metadata.created_at),
    }))
}

/// Close a BTSP tunnel.
pub(crate) async fn btsp_close_tunnel(
    State(state): State<ApiState>,
    Path(tunnel_id): Path<String>,
) -> Result<Json<BtspCloseResponse>, ApiError> {
    state.increment_requests();
    info!(tunnel_id = %tunnel_id, "BTSP: Closing tunnel");

    let existed = state.tunnels.write().remove(&tunnel_id).is_some();

    Ok(Json(BtspCloseResponse { success: existed }))
}

// ── BirdSong ────────────────────────────────────────────────────────────────

/// Encrypt a broadcast for lineage-based access.
///
/// Requires BirdSongManager integration (not yet wired).
pub(crate) async fn birdsong_encrypt(
    State(state): State<ApiState>,
    Json(req): Json<BirdSongEncryptRequest>,
) -> Result<Json<BirdSongEncryptResponse>, ApiError> {
    state.increment_requests();
    info!(lineage_hint = ?req.lineage_hint, "BirdSong: Encrypting broadcast");

    Err(ApiError::NotImplemented(
        "BirdSong operations require BirdSongManager integration".to_string(),
    ))
}

/// Decrypt a lineage-gated broadcast.
///
/// Requires BirdSongManager integration (not yet wired).
pub(crate) async fn birdsong_decrypt(
    State(state): State<ApiState>,
    Json(req): Json<BirdSongDecryptRequest>,
) -> Result<Json<BirdSongDecryptResponse>, ApiError> {
    state.increment_requests();
    info!(lineage_hint = ?req.lineage_hint, "BirdSong: Decrypting broadcast");

    Err(ApiError::NotImplemented(
        "BirdSong operations require BirdSongManager integration".to_string(),
    ))
}

/// Get lineage information for a node.
pub(crate) async fn birdsong_get_lineage(
    State(state): State<ApiState>,
    Path(node_id): Path<String>,
) -> Result<Json<LineageInfo>, ApiError> {
    state.increment_requests();
    info!(node_id = %node_id, "BirdSong: Getting lineage");

    let lineages = state.lineages.read();
    let metadata = lineages
        .get(&node_id)
        .ok_or_else(|| ApiError::BadRequest("Lineage not found".to_string()))?;

    info!(
        parent_id = ?metadata.parent_id,
        created_at = ?metadata.created_at,
        "BirdSong: Lineage record"
    );

    Ok(Json(LineageInfo {
        node_id: metadata.node_id.clone(),
        lineage_chain: metadata.chain.clone(),
        depth: metadata.depth,
    }))
}

/// Verify a lineage proof.
///
/// Requires cryptographic proof validation (not yet wired).
pub(crate) async fn birdsong_verify_lineage(
    State(state): State<ApiState>,
    Json(req): Json<VerifyLineageRequest>,
) -> Result<Json<VerifyLineageResponse>, ApiError> {
    state.increment_requests();
    info!(
        node_id = %req.node_id,
        proof_len = %req.proof.len(),
        "BirdSong: Verifying lineage proof"
    );

    Err(ApiError::NotImplemented(
        "Lineage verification requires cryptographic proof validation".to_string(),
    ))
}

// ── Lineage ─────────────────────────────────────────────────────────────────

/// Generate a new lineage chain.
///
/// Requires HSM-backed signing (not yet wired).
pub(crate) async fn lineage_generate(
    State(state): State<ApiState>,
    Json(req): Json<GenerateLineageRequest>,
) -> Result<Json<GenerateLineageResponse>, ApiError> {
    state.increment_requests();
    info!(node_id = %req.node_id, parent = ?req.parent_id, "Lineage: Generating");

    Err(ApiError::NotImplemented(
        "Lineage operations require HSM-backed signing".to_string(),
    ))
}

/// Verify a lineage chain.
///
/// Requires ordered signature validation (not yet wired).
pub(crate) async fn lineage_verify(
    State(state): State<ApiState>,
    Json(_req): Json<VerifyLineageChainRequest>,
) -> Result<Json<VerifyLineageChainResponse>, ApiError> {
    state.increment_requests();
    info!("Lineage: Verifying chain");

    Err(ApiError::NotImplemented(
        "Chain integrity requires ordered signature validation".to_string(),
    ))
}

/// Get cryptographic proof for a lineage.
///
/// Requires HSM-backed signing (not yet wired).
pub(crate) async fn lineage_get_proof(
    State(state): State<ApiState>,
    Path(node_id): Path<String>,
) -> Result<Json<LineageProof>, ApiError> {
    state.increment_requests();
    info!(node_id = %node_id, "Lineage: Getting proof");

    Err(ApiError::NotImplemented(
        "Lineage operations require HSM-backed signing".to_string(),
    ))
}

// ── System ──────────────────────────────────────────────────────────────────

/// Health check endpoint.
pub(crate) async fn health_check(State(state): State<ApiState>) -> Json<HealthResponse> {
    let uptime = state
        .start_time
        .elapsed()
        .unwrap_or(Duration::from_secs(0))
        .as_secs();

    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
    })
}

/// Get service metrics.
pub(crate) async fn get_metrics(State(state): State<ApiState>) -> Json<MetricsResponse> {
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

/// Get service capabilities.
pub(crate) async fn get_capabilities() -> Json<CapabilitiesResponse> {
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

/// Get detailed service status.
pub(crate) async fn get_status(State(state): State<ApiState>) -> Json<StatusResponse> {
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
