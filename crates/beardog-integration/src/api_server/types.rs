// SPDX-License-Identifier: AGPL-3.0-only

//! Request/response types for the integration API, organized by domain.

use serde::{Deserialize, Serialize};

// ── BTSP ────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub(crate) struct BtspEstablishRequest {
    pub responder_id: String,
    pub initiator_entropy: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct BtspEstablishResponse {
    pub tunnel_id: String,
    pub responder_entropy: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BtspEncryptRequest {
    pub plaintext: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct BtspEncryptResponse {
    pub ciphertext: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BtspDecryptRequest {
    pub ciphertext: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct BtspDecryptResponse {
    pub plaintext: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct BtspTunnelStatus {
    pub tunnel_id: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct BtspCloseResponse {
    pub success: bool,
}

// ── BirdSong ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub(crate) struct BirdSongEncryptRequest {
    pub payload: String,
    pub lineage_hint: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct BirdSongEncryptResponse {
    pub ciphertext: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BirdSongDecryptRequest {
    pub ciphertext: String,
    pub lineage_hint: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct BirdSongDecryptResponse {
    pub payload: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct LineageInfo {
    pub node_id: String,
    pub lineage_chain: Vec<String>,
    pub depth: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct VerifyLineageRequest {
    pub proof: String,
    pub node_id: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct VerifyLineageResponse {
    pub valid: bool,
    pub details: Option<String>,
}

// ── Lineage ─────────────────────────────────────────────────────────────────

/// Internal lineage metadata stored in [`super::ApiState`].
#[derive(Debug, Clone)]
pub(crate) struct LineageMetadata {
    pub node_id: String,
    pub parent_id: Option<String>,
    pub chain: Vec<String>,
    pub depth: u32,
    pub created_at: std::time::SystemTime,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GenerateLineageRequest {
    pub node_id: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct GenerateLineageResponse {
    pub lineage_chain: Vec<String>,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct VerifyLineageChainRequest {
    pub lineage_chain: Vec<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct VerifyLineageChainResponse {
    pub valid: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct LineageProof {
    pub merkle_proof: Vec<String>,
    pub signature: String,
}

// ── System ──────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub(crate) struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
}

#[derive(Debug, Serialize)]
pub(crate) struct MetricsResponse {
    pub active_tunnels: u32,
    pub total_requests: u64,
    pub uptime_seconds: u64,
}

#[derive(Debug, Serialize)]
pub(crate) struct CapabilitiesResponse {
    pub capabilities: Vec<String>,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct StatusResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub active_tunnels: u32,
    pub active_lineages: u32,
    pub total_requests: u64,
}
