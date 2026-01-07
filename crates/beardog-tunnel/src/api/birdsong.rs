//! BirdSong API endpoints
//!
//! Privacy-preserving broadcast encryption/decryption based on lineage.
//! Also includes family-based discovery encryption for simpler use cases.

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

/// Request to encrypt discovery packet for family
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptDiscoveryRequest {
    /// Base64-encoded plaintext discovery message
    #[serde(with = "base64_serde")]
    pub plaintext: Vec<u8>,
    /// Family ID for key derivation
    pub family_id: String,
}

/// Response with encrypted discovery packet
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptDiscoveryResponse {
    /// Base64-encoded encrypted message
    #[serde(with = "base64_serde")]
    pub encrypted: Vec<u8>,
    /// Family ID (echoed back)
    pub family_id: String,
}

/// Request to decrypt discovery packet from family
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecryptDiscoveryRequest {
    /// Base64-encoded encrypted message
    #[serde(with = "base64_serde")]
    pub encrypted: Vec<u8>,
    /// Family ID for key derivation
    pub family_id: String,
}

/// Response with decrypted discovery plaintext
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecryptDiscoveryResponse {
    /// Base64-encoded plaintext discovery message
    #[serde(with = "base64_serde")]
    pub plaintext: Vec<u8>,
    /// Family ID (echoed back)
    pub family_id: String,
}

// ============================================================================
// V2 API Types (Songbird-compatible, clean generic interface)
// ============================================================================

/// V2 encrypt request - generic, works for discovery, signal, or any BirdSong use case
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptRequestV2 {
    /// Base64-encoded plaintext to encrypt
    #[serde(with = "base64_serde")]
    pub plaintext: Vec<u8>,
    /// Optional family ID (uses node's default family if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_id: Option<String>,
}

/// V2 encrypt response - clean, generic
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptResponseV2 {
    /// Base64-encoded ciphertext
    #[serde(with = "base64_serde")]
    pub ciphertext: Vec<u8>,
    /// Family ID used for encryption
    pub family_id: String,
}

/// V2 decrypt request - generic, works for discovery, signal, or any BirdSong use case
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecryptRequestV2 {
    /// Base64-encoded ciphertext to decrypt
    #[serde(with = "base64_serde")]
    pub ciphertext: Vec<u8>,
    /// Optional family ID hint (will try to decrypt with this family's key)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_id: Option<String>,
}

/// V2 decrypt response - clean, with success indicator
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecryptResponseV2 {
    /// Base64-encoded plaintext (if successful)
    #[serde(with = "base64_serde")]
    pub plaintext: Vec<u8>,
    /// Family ID that encrypted this data
    pub family_id: String,
    /// Whether decryption succeeded (false if different family/corrupted)
    pub success: bool,
}

/// BirdSong v1 routes (backward compatible)
pub fn routes(state: BirdSongApiState) -> Router {
    Router::new()
        .route("/encrypt", post(encrypt_for_lineage))
        .route("/decrypt", post(decrypt_birdsong))
        // Discovery encryption endpoints (simpler, family-based)
        .route("/encrypt_discovery", post(encrypt_discovery))
        .route("/decrypt_discovery", post(decrypt_discovery))
        .with_state(state)
}

/// BirdSong v2 routes (Songbird-compatible, clean generic interface)
pub fn routes_v2(state: BirdSongApiState) -> Router {
    Router::new()
        // Generic encrypt/decrypt - works for discovery, signal, or any use case
        // Routing based on genetics (family_id) of the key
        .route("/encrypt", post(encrypt_v2))
        .route("/decrypt", post(decrypt_v2))
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

/// POST /birdsong/encrypt_discovery - Encrypt discovery packet for family
///
/// Uses family-specific keys so only same-family towers can decrypt.
/// This is simpler than lineage-based encryption for UDP discovery.
async fn encrypt_discovery(
    State(state): State<BirdSongApiState>,
    Json(req): Json<EncryptDiscoveryRequest>,
) -> Result<Json<ApiResponse<EncryptDiscoveryResponse>>, ApiError> {
    info!(
        "🎵 BirdSong discovery encrypt for family: {}",
        req.family_id
    );

    // Encrypt using family-specific discovery keys
    let encrypted = state
        .manager
        .encrypt_discovery_for_family(&req.plaintext, &req.family_id)
        .map_err(|e| {
            warn!("Failed to encrypt discovery packet: {}", e);
            ApiError::internal(format!("Discovery encryption failed: {}", e))
        })?;

    let response = EncryptDiscoveryResponse {
        encrypted,
        family_id: req.family_id,
    };

    info!("✅ Discovery packet encrypted successfully");
    Ok(Json(ApiResponse::success(response)))
}

/// POST /birdsong/decrypt_discovery - Decrypt discovery packet from family
///
/// Attempts to decrypt using family-specific keys. Will fail if:
/// - Wrong family (different keys)
/// - Corrupted ciphertext
/// - Invalid authentication tag
async fn decrypt_discovery(
    State(state): State<BirdSongApiState>,
    Json(req): Json<DecryptDiscoveryRequest>,
) -> Result<Json<ApiResponse<DecryptDiscoveryResponse>>, ApiError> {
    info!(
        "🎵 BirdSong discovery decrypt for family: {}",
        req.family_id
    );

    // Decrypt using family-specific discovery keys
    let plaintext = state
        .manager
        .decrypt_discovery_from_family(&req.encrypted, &req.family_id)
        .map_err(|e| {
            // Different family or corrupted data - this is expected noise, not an error
            warn!(
                "Cannot decrypt discovery packet (likely different family): {}",
                e
            );
            ApiError::bad_request(format!("Decryption failed: {}", e))
        })?;

    let response = DecryptDiscoveryResponse {
        plaintext,
        family_id: req.family_id,
    };

    info!("✅ Discovery packet decrypted successfully");
    Ok(Json(ApiResponse::success(response)))
}

// ============================================================================
// V2 API Handlers (Clean, generic, Songbird-compatible)
// ============================================================================

/// POST /api/v2/birdsong/encrypt - Generic encryption endpoint
///
/// Encrypts data using family-based keys. The genetics (family_id) of the key
/// determines routing/behavior. Works for discovery, signal, or any BirdSong use case.
///
/// ## Use Cases
/// - Discovery: Encrypt UDP broadcast packets
/// - Signal: Encrypt signaling messages
/// - Generic: Any family-based encrypted communication
///
/// ## Key Derivation
/// The family_id parameter determines which family's key is used:
/// - If provided: Uses that family's key
/// - If omitted: Uses node's default family key
///
/// The genetics of the key (family lineage) automatically handles routing.
async fn encrypt_v2(
    State(state): State<BirdSongApiState>,
    Json(req): Json<EncryptRequestV2>,
) -> Result<Json<ApiResponse<EncryptResponseV2>>, ApiError> {
    let family_id = req.family_id.clone().unwrap_or_else(|| {
        // TODO: Get node's default family_id from manager
        "default".to_string()
    });

    info!("🎵 BirdSong v2 encrypt for family: {}", family_id);

    // Use existing family-based encryption (same implementation as v1 discovery)
    let ciphertext = state
        .manager
        .encrypt_discovery_for_family(&req.plaintext, &family_id)
        .map_err(|e| {
            warn!("Failed to encrypt: {}", e);
            ApiError::internal(format!("Encryption failed: {}", e))
        })?;

    let response = EncryptResponseV2 {
        ciphertext,
        family_id,
    };

    info!(
        "✅ BirdSong v2 encrypted successfully ({} bytes)",
        response.ciphertext.len()
    );
    Ok(Json(ApiResponse::success(response)))
}

/// POST /api/v2/birdsong/decrypt - Generic decryption endpoint
///
/// Decrypts data using family-based keys. The genetics (family_id) of the key
/// determines routing/behavior. Works for discovery, signal, or any BirdSong use case.
///
/// ## Behavior
/// - Same family: Decrypts successfully, returns plaintext
/// - Different family: Returns success=false (privacy preserved)
/// - Corrupted data: Returns error
///
/// ## Privacy Model
/// Different families cannot decrypt each other's messages. This preserves
/// privacy in multi-family environments (e.g., UDP multicast).
async fn decrypt_v2(
    State(state): State<BirdSongApiState>,
    Json(req): Json<DecryptRequestV2>,
) -> Result<Json<ApiResponse<DecryptResponseV2>>, ApiError> {
    let family_id = req.family_id.clone().unwrap_or_else(|| {
        // TODO: Get node's default family_id from manager
        "default".to_string()
    });

    info!("🎵 BirdSong v2 decrypt for family: {}", family_id);

    // Attempt decryption with family-based key
    match state
        .manager
        .decrypt_discovery_from_family(&req.ciphertext, &family_id)
    {
        Ok(plaintext) => {
            info!(
                "✅ BirdSong v2 decrypted successfully ({} bytes)",
                plaintext.len()
            );
            let response = DecryptResponseV2 {
                plaintext,
                family_id,
                success: true,
            };
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            // Different family or authentication failure - not an error, just noise
            info!(
                "🔇 BirdSong v2 decrypt failed (likely different family): {}",
                e
            );

            // Return success=false to indicate "not for us" (graceful privacy)
            let response = DecryptResponseV2 {
                plaintext: vec![],
                family_id,
                success: false,
            };
            Ok(Json(ApiResponse::success(response)))
        }
    }
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
