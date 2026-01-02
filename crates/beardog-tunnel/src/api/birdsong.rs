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
    State(state): State<BirdSongApiState>,
    Json(req): Json<EncryptForLineageRequest>,
) -> Result<Json<ApiResponse<EncryptForLineageResponse>>, ApiError> {
    info!("🎵 BirdSong encrypt request for lineage");

    // Build encryption request from API request
    let encrypt_req = beardog_genetics::birdsong::types::BirdSongEncryptRequest {
        plaintext: req.plaintext,
        lineage_hint: req.lineage_hint.clone(),
        associated_data: None,
    };

    // Encrypt broadcast using BirdSongManager
    let broadcast = state.manager.encrypt_broadcast(&encrypt_req).map_err(|e| {
        warn!("Failed to encrypt broadcast: {}", e);
        ApiError::internal(format!("Encryption failed: {}", e))
    })?;

    // Build response
    let response = EncryptForLineageResponse {
        ciphertext: broadcast.ciphertext,
        lineage_hint: req.lineage_hint,
    };

    info!("✅ BirdSong broadcast encrypted successfully");
    Ok(Json(ApiResponse::success(response)))
}

/// POST /birdsong/decrypt - Decrypt birdsong broadcast (if in lineage)
async fn decrypt_birdsong(
    State(state): State<BirdSongApiState>,
    Json(req): Json<DecryptBirdSongRequest>,
) -> Result<Json<ApiResponse<DecryptBirdSongResponse>>, ApiError> {
    info!("🎵 BirdSong decrypt request");

    // Build broadcast from ciphertext (simplified - in production would parse full broadcast)
    let broadcast = beardog_genetics::birdsong::types::BirdSongBroadcast {
        ciphertext: req.ciphertext,
        hint: beardog_genetics::birdsong::LineageHint {
            root_id: "".into(), // Would be extracted from broadcast metadata
            min_depth: 0,
            max_depth: 99,
            biome_filter: None,
            version: 1,
        },
        nonce: vec![0; 12], // Would be extracted from broadcast
        associated_data: None,
        broadcast_at: chrono::Utc::now(),
    };

    // Build decryption request
    let decrypt_req = beardog_genetics::birdsong::types::BirdSongDecryptRequest {
        broadcast,
        proof: req.lineage_proof,
    };

    // Decrypt broadcast using BirdSongManager
    let plaintext = state.manager.decrypt_broadcast(&decrypt_req).map_err(|e| {
        warn!("Failed to decrypt broadcast: {}", e);
        ApiError::internal(format!("Decryption failed: {}", e))
    })?;

    // Build response
    let response = DecryptBirdSongResponse {
        plaintext,
        verified: true, // If decrypt succeeded, proof was valid
    };

    info!("✅ BirdSong broadcast decrypted successfully");
    Ok(Json(ApiResponse::success(response)))
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

    #[test]
    fn test_decrypt_request_serialization() {
        use chrono::Utc;

        let proof = LineageProof {
            node_id: "child-node".into(),
            root_id: "root-node".into(),
            path: vec!["root-node".into(), "child-node".into()],
            proof_chain: vec![],
            merkle_root: vec![0; 32],
            generated_at: Utc::now(),
        };

        let req = DecryptBirdSongRequest {
            ciphertext: vec![1, 2, 3, 4],
            lineage_proof: proof,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("child-node"));
        assert!(json.contains("root-node"));
    }

    #[test]
    fn test_encrypt_response_structure() {
        let response = EncryptForLineageResponse {
            ciphertext: vec![1, 2, 3, 4, 5],
            lineage_hint: LineageHint {
                root_id: "test-root".into(),
                min_depth: 0,
                max_depth: 5,
                biome_filter: None,
                version: 1,
            },
        };

        assert_eq!(response.ciphertext.len(), 5);
        assert_eq!(response.lineage_hint.root_id, "test-root");
    }

    #[test]
    fn test_decrypt_response_structure() {
        let response = DecryptBirdSongResponse {
            plaintext: b"Decrypted message".to_vec(),
            verified: true,
        };

        assert!(response.verified);
        assert_eq!(response.plaintext, b"Decrypted message");
    }

    #[test]
    fn test_lineage_hint_validation() {
        let hint = LineageHint {
            root_id: "root".into(),
            min_depth: 0,
            max_depth: 10,
            biome_filter: Some("production".into()),
            version: 1,
        };

        assert_eq!(hint.min_depth, 0);
        assert_eq!(hint.max_depth, 10);
        assert!(hint.biome_filter.is_some());
    }

    #[test]
    fn test_empty_ciphertext() {
        let req = DecryptBirdSongRequest {
            ciphertext: vec![],
            lineage_proof: LineageProof {
                node_id: "node".into(),
                root_id: "root".into(),
                path: vec![],
                proof_chain: vec![],
                merkle_root: vec![],
                generated_at: chrono::Utc::now(),
            },
        };

        assert!(req.ciphertext.is_empty());
    }

    #[test]
    fn test_lineage_hint_with_no_filter() {
        let hint = LineageHint {
            root_id: "root".into(),
            min_depth: 0,
            max_depth: 5,
            biome_filter: None,
            version: 1,
        };

        assert!(hint.biome_filter.is_none());
    }

    #[test]
    fn test_request_response_roundtrip() {
        // Test that we can serialize and deserialize
        let req = EncryptForLineageRequest {
            plaintext: b"Test message".to_vec(),
            lineage_hint: LineageHint {
                root_id: "root".into(),
                min_depth: 0,
                max_depth: 3,
                biome_filter: Some("test".into()),
                version: 1,
            },
        };

        let json = serde_json::to_string(&req).unwrap();
        let deserialized: EncryptForLineageRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(req.plaintext, deserialized.plaintext);
        assert_eq!(req.lineage_hint.root_id, deserialized.lineage_hint.root_id);
    }
}
