//! BirdSong API endpoints
//!
//! Privacy-preserving broadcast encryption/decryption based on lineage.

use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};

use beardog_genetics::birdsong::{BirdSongManager, LineageHint, LineageProof};

use super::types::{base64_serde, ApiError, ApiResponse};

/// BirdSong API state
#[derive(Clone)]
pub struct BirdSongApiState {
    /// BirdSong manager
    pub manager: Arc<BirdSongManager>,
}

/// Request to encrypt for lineage
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptForLineageRequest {
    /// Base64-encoded plaintext
    #[serde(with = "base64_serde")]
    pub plaintext: Vec<u8>,
    /// Lineage hint for key derivation
    pub lineage_hint: LineageHint,
}

/// Response with encrypted broadcast
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptForLineageResponse {
    /// Base64-encoded ciphertext
    #[serde(with = "base64_serde")]
    pub ciphertext: Vec<u8>,
    /// Lineage hint (included in broadcast)
    pub lineage_hint: LineageHint,
}

/// Request to decrypt birdsong
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecryptBirdSongRequest {
    /// Base64-encoded ciphertext
    #[serde(with = "base64_serde")]
    pub ciphertext: Vec<u8>,
    /// Lineage proof for verification
    pub lineage_proof: LineageProof,
}

/// Response with decrypted plaintext
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecryptBirdSongResponse {
    /// Base64-encoded plaintext
    #[serde(with = "base64_serde")]
    pub plaintext: Vec<u8>,
    /// Whether decryption succeeded
    pub verified: bool,
}

/// BirdSong routes
pub fn routes(state: BirdSongApiState) -> Router {
    Router::new()
        .route("/encrypt", post(encrypt_for_lineage))
        .route("/decrypt", post(decrypt_birdsong))
        .with_state(state)
}

/// POST /birdsong/encrypt - Encrypt broadcast for specific lineage
async fn encrypt_for_lineage(
    State(_state): State<BirdSongApiState>,
    Json(_req): Json<EncryptForLineageRequest>,
) -> Result<Json<ApiResponse<EncryptForLineageResponse>>, ApiError> {
    // TODO: Implement when BirdSongManager methods are available
    warn!("🚧 BirdSong encrypt endpoint not yet implemented");
    Err(ApiError::internal(
        "BirdSong encryption not yet implemented - awaiting manager methods",
    ))
}

/// POST /birdsong/decrypt - Decrypt birdsong broadcast (if in lineage)
async fn decrypt_birdsong(
    State(_state): State<BirdSongApiState>,
    Json(_req): Json<DecryptBirdSongRequest>,
) -> Result<Json<ApiResponse<DecryptBirdSongResponse>>, ApiError> {
    // TODO: Implement when BirdSongManager methods are available
    warn!("🚧 BirdSong decrypt endpoint not yet implemented");
    Err(ApiError::internal(
        "BirdSong decryption not yet implemented - awaiting manager methods",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_request_serialization() {
        let req = EncryptForLineageRequest {
            plaintext: b"Hello family!".to_vec(),
            lineage_hint: LineageHint {
                root_id: "root-node".into(),
                min_depth: 0,
                max_depth: 3,
                biome_filter: Some("test".into()),
                version: 1,
            },
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("root-node"));
    }
}
