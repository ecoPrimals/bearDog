//! BTSP API endpoints
//!
//! Secure tunnel operations for Songbird integration.

use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};

use crate::btsp_provider::BeardogBtspProvider;
use beardog_capabilities::traits::SecureTunnelProvider;

use super::types::{base64_serde, ApiError, ApiResponse};

/// BTSP API state
#[derive(Clone)]
pub struct BtspApiState {
    /// BTSP provider
    pub provider: Arc<BeardogBtspProvider>,
}

/// Request to establish tunnel
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstablishTunnelRequest {
    /// Peer information
    pub peer: beardog_capabilities::traits::PeerEndpoint,
}

/// Request to encrypt data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptRequest {
    /// Tunnel handle (contains id)
    pub tunnel: beardog_capabilities::traits::TunnelHandle,
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
    /// Tunnel handle (contains id)
    pub tunnel: beardog_capabilities::traits::TunnelHandle,
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

/// Request to exchange contact information via genetic lineage
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactExchangeRequest {
    /// Target peer ID to find contact info for
    pub target_peer_id: String,
    /// Requester's lineage ID for verification
    pub requester_lineage: String,
    /// Maximum hops to search in genetic lineage (default: 3)
    #[serde(default = "default_max_hops")]
    pub max_hops: usize,
}

fn default_max_hops() -> usize {
    3
}

/// Response with peer contact information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactExchangeResponse {
    /// Contact information for the requested peer
    pub contact: crate::btsp_provider::ContactInfo,
}

/// BTSP routes
pub fn routes(state: BtspApiState) -> Router {
    Router::new()
        .route("/tunnel/establish", post(establish_tunnel))
        .route("/tunnel/encrypt", post(encrypt))
        .route("/tunnel/decrypt", post(decrypt))
        .route("/tunnel/status/:id", get(tunnel_status))
        .route("/tunnel/close/:id", delete(close_tunnel))
        .route("/contact/exchange", post(contact_exchange))
        .with_state(state)
}

/// POST /btsp/tunnel/establish - Establish secure tunnel
async fn establish_tunnel(
    State(state): State<BtspApiState>,
    Json(req): Json<EstablishTunnelRequest>,
) -> Result<Json<ApiResponse<beardog_capabilities::traits::TunnelHandle>>, ApiError> {
    info!("🔒 Establishing BTSP tunnel with peer: {}", req.peer.id);

    let handle = state
        .provider
        .establish_tunnel(req.peer)  // Takes by value, not reference
        .await
        .map_err(|e| {
            warn!("Failed to establish tunnel: {}", e);
            ApiError::from(e)
        })?;

    info!("✅ BTSP tunnel established: {}", handle.id);

    Ok(Json(ApiResponse::success(handle)))
}

/// POST /btsp/tunnel/encrypt - Encrypt data through tunnel
async fn encrypt(
    State(state): State<BtspApiState>,
    Json(req): Json<EncryptRequest>,
) -> Result<Json<ApiResponse<EncryptResponse>>, ApiError> {
    info!("🔒 Encrypting data for tunnel: {}", req.tunnel.id);

    let ciphertext = state
        .provider
        .tunnel_encrypt(&req.tunnel, &req.data)  // Correct method name
        .await
        .map_err(|e| {
            warn!("Failed to encrypt data: {}", e);
            ApiError::from(e)
        })?;

    Ok(Json(ApiResponse::success(EncryptResponse { ciphertext })))
}

/// POST /btsp/tunnel/decrypt - Decrypt data from tunnel
async fn decrypt(
    State(state): State<BtspApiState>,
    Json(req): Json<DecryptRequest>,
) -> Result<Json<ApiResponse<DecryptResponse>>, ApiError> {
    info!("🔒 Decrypting data for tunnel: {}", req.tunnel.id);

    let plaintext = state
        .provider
        .tunnel_decrypt(&req.tunnel, &req.data)  // Correct method name
        .await
        .map_err(|e| {
            warn!("Failed to decrypt data: {}", e);
            ApiError::from(e)
        })?;

    Ok(Json(ApiResponse::success(DecryptResponse { plaintext })))
}

/// GET /btsp/tunnel/status/:id - Get tunnel status
async fn tunnel_status(
    State(state): State<BtspApiState>,
    Path(tunnel_id): Path<String>,
) -> Result<Json<ApiResponse<beardog_capabilities::traits::TunnelStatus>>, ApiError> {
    info!("🔍 Getting status for tunnel: {}", tunnel_id);

    // Create minimal tunnel handle for status query
    let handle = beardog_capabilities::traits::TunnelHandle {
        id: tunnel_id.clone(),
        peer_id: String::new(),        // Not needed for status query
        established_at: String::new(), // Not needed for status query
    };

    let status = state
        .provider
        .tunnel_status(&handle)  // Correct method name
        .await
        .map_err(|e| {
            warn!("Failed to get tunnel status: {}", e);
            ApiError::from(e)
        })?;

    Ok(Json(ApiResponse::success(status)))
}

/// DELETE /btsp/tunnel/close/:id - Close tunnel
async fn close_tunnel(
    State(state): State<BtspApiState>,
    Path(tunnel_id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("🔒 Closing tunnel: {}", tunnel_id);

    // Create minimal tunnel handle for close
    let handle = beardog_capabilities::traits::TunnelHandle {
        id: tunnel_id.clone(),
        peer_id: String::new(),
        established_at: String::new(),
    };

    state
        .provider
        .close_tunnel(&handle)  // Correct method name
        .await
        .map_err(|e| {
            warn!("Failed to close tunnel: {}", e);
            ApiError::from(e)
        })?;

    info!("✅ Tunnel closed: {}", tunnel_id);

    Ok(Json(ApiResponse::success_with_message(
        (),
        "Tunnel closed successfully",
    )))
}

/// POST /btsp/contact/exchange - Exchange contact information via genetic lineage
async fn contact_exchange(
    State(state): State<BtspApiState>,
    Json(req): Json<ContactExchangeRequest>,
) -> Result<Json<ApiResponse<ContactExchangeResponse>>, ApiError> {
    info!(
        "🔍 Contact exchange request for peer: {}",
        req.target_peer_id
    );

    let contact = state
        .provider
        .contact_exchange(&req.target_peer_id, &req.requester_lineage, req.max_hops)
        .await
        .map_err(|e| {
            warn!("Contact exchange failed: {}", e);
            ApiError::from(e)
        })?;

    info!(
        "✅ Contact exchange successful: {} addresses found",
        contact.addresses.len()
    );

    Ok(Json(ApiResponse::success(ContactExchangeResponse {
        contact,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_establish_tunnel_request() {
        let req = EstablishTunnelRequest {
            peer: beardog_capabilities::traits::PeerEndpoint {
                id: "peer-123".into(),
                endpoint: "127.0.0.1:8080".into(),
                public_key: Some(vec![1u8; 32]),
            },
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("peer-123"));
    }
}
