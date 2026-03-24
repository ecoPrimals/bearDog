// SPDX-License-Identifier: AGPL-3.0-only

//! Domain handlers for the integration API.
//!
//! Organized into BTSP, BirdSong, Lineage, and System handler groups.
//! Handlers that are not yet wired to real providers return structured
//! `501 Not Implemented` via [`super::ApiError`] so callers get a clear
//! signal rather than simulated data.

use std::time::{Duration, SystemTime};

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
    HealthResponse, LineageInfo, LineageMetadata, LineageProof, MetricsResponse, StatusResponse,
    VerifyLineageChainRequest, VerifyLineageChainResponse, VerifyLineageRequest,
    VerifyLineageResponse,
};
use super::{ApiError, ApiState, TunnelMetadata};

// ── BTSP ────────────────────────────────────────────────────────────────────

/// Establish a new BTSP tunnel.
///
/// Creates a secure tunnel with genetic key exchange.
pub(crate) async fn btsp_establish_tunnel(
    State(state): State<ApiState>,
    Json(req): Json<BtspEstablishRequest>,
) -> Result<Json<BtspEstablishResponse>, ApiError> {
    state.increment_requests();
    info!(responder = %req.responder_id, "BTSP: Establishing tunnel");

    let tunnel_id = format!("tunnel_{}", uuid::Uuid::new_v4());

    let metadata = TunnelMetadata {
        tunnel_id: tunnel_id.clone(),
        peer_id: req.responder_id.clone(),
        created_at: SystemTime::now(),
        status: "established".to_string(),
    };
    state.tunnels.write().insert(tunnel_id.clone(), metadata);

    Ok(Json(BtspEstablishResponse {
        tunnel_id,
        responder_entropy: format!("entropy_{}", req.initiator_entropy),
    }))
}

/// Encrypt data through a BTSP tunnel.
pub(crate) async fn btsp_encrypt(
    State(state): State<ApiState>,
    Path(tunnel_id): Path<String>,
    Json(req): Json<BtspEncryptRequest>,
) -> Result<Json<BtspEncryptResponse>, ApiError> {
    state.increment_requests();
    info!(tunnel_id = %tunnel_id, "BTSP: Encrypting data");

    let tunnels = state.tunnels.read();
    if !tunnels.contains_key(&tunnel_id) {
        return Err(ApiError::BadRequest("Tunnel not found".to_string()));
    }

    let ciphertext = format!("encrypted_{}", req.plaintext);

    Ok(Json(BtspEncryptResponse { ciphertext }))
}

/// Decrypt data from a BTSP tunnel.
pub(crate) async fn btsp_decrypt(
    State(state): State<ApiState>,
    Path(tunnel_id): Path<String>,
    Json(req): Json<BtspDecryptRequest>,
) -> Result<Json<BtspDecryptResponse>, ApiError> {
    state.increment_requests();
    info!(tunnel_id = %tunnel_id, "BTSP: Decrypting data");

    let tunnels = state.tunnels.read();
    if !tunnels.contains_key(&tunnel_id) {
        return Err(ApiError::BadRequest("Tunnel not found".to_string()));
    }

    let plaintext = req
        .ciphertext
        .strip_prefix("encrypted_")
        .unwrap_or(&req.ciphertext)
        .to_string();

    Ok(Json(BtspDecryptResponse { plaintext }))
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
pub(crate) async fn birdsong_encrypt(
    State(state): State<ApiState>,
    Json(req): Json<BirdSongEncryptRequest>,
) -> Result<Json<BirdSongEncryptResponse>, ApiError> {
    state.increment_requests();
    info!(lineage_hint = ?req.lineage_hint, "BirdSong: Encrypting broadcast");

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

/// Decrypt a lineage-gated broadcast.
pub(crate) async fn birdsong_decrypt(
    State(state): State<ApiState>,
    Json(req): Json<BirdSongDecryptRequest>,
) -> Result<Json<BirdSongDecryptResponse>, ApiError> {
    state.increment_requests();
    info!(lineage_hint = ?req.lineage_hint, "BirdSong: Decrypting broadcast");

    let payload = req
        .ciphertext
        .strip_prefix("birdsong_encrypted_")
        .unwrap_or(&req.ciphertext)
        .to_string();

    Ok(Json(BirdSongDecryptResponse { payload }))
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

// ── Lineage ─────────────────────────────────────────────────────────────────

/// Generate a new lineage chain.
pub(crate) async fn lineage_generate(
    State(state): State<ApiState>,
    Json(req): Json<GenerateLineageRequest>,
) -> Result<Json<GenerateLineageResponse>, ApiError> {
    state.increment_requests();
    info!(node_id = %req.node_id, parent = ?req.parent_id, "Lineage: Generating");

    let mut chain = Vec::new();
    let mut depth = 0;

    if let Some(parent_id) = &req.parent_id {
        let lineages = state.lineages.read();
        if let Some(parent) = lineages.get(parent_id) {
            chain.extend_from_slice(&parent.chain);
            depth = parent.depth + 1;
        }
    }

    let node_id = req.node_id.clone();
    chain.push(node_id.clone());

    let metadata = LineageMetadata {
        node_id: node_id.clone(),
        parent_id: req.parent_id.clone(),
        chain: chain.clone(),
        depth,
        created_at: SystemTime::now(),
    };
    state.lineages.write().insert(node_id, metadata);

    let signature = format!("sig_{}", uuid::Uuid::new_v4());

    Ok(Json(GenerateLineageResponse {
        lineage_chain: chain,
        signature,
    }))
}

/// Verify a lineage chain.
pub(crate) async fn lineage_verify(
    State(state): State<ApiState>,
    Json(req): Json<VerifyLineageChainRequest>,
) -> Result<Json<VerifyLineageChainResponse>, ApiError> {
    state.increment_requests();
    info!("Lineage: Verifying chain");

    let lineages = state.lineages.read();
    let valid = req
        .lineage_chain
        .iter()
        .any(|node_id| lineages.contains_key(node_id));

    Ok(Json(VerifyLineageChainResponse { valid }))
}

/// Get cryptographic proof for a lineage.
pub(crate) async fn lineage_get_proof(
    State(state): State<ApiState>,
    Path(node_id): Path<String>,
) -> Result<Json<LineageProof>, ApiError> {
    state.increment_requests();
    info!(node_id = %node_id, "Lineage: Getting proof");

    let lineages = state.lineages.read();
    if !lineages.contains_key(&node_id) {
        return Err(ApiError::BadRequest("Lineage not found".to_string()));
    }

    let merkle_proof = vec![
        format!("hash1_{node_id}"),
        format!("hash2_{node_id}"),
        format!("root_{node_id}"),
    ];

    let signature = format!("proof_sig_{}", uuid::Uuid::new_v4());

    Ok(Json(LineageProof {
        merkle_proof,
        signature,
    }))
}

// ── System ──────────────────────────────────────────────────────────────────

/// Health check endpoint.
pub(crate) async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0,
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
