//! Lineage API endpoints
//!
//! Cryptographic lineage proofs and verification.

use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};

use beardog_genetics::birdsong::{LineageChainManager, LineageProof, LineageProofManager};

use super::types::{ApiError, ApiResponse};

/// Lineage API state
#[derive(Clone)]
pub struct LineageApiState {
    /// Lineage chain manager
    pub chain_manager: Arc<LineageChainManager>,
    /// Lineage proof manager
    pub proof_manager: Arc<LineageProofManager>,
}

/// Request to generate lineage proof
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenerateProofRequest {
    /// Node ID to generate proof for
    pub node_id: String,
    /// Chain ID
    pub chain_id: String,
}

/// Request to verify lineage proof
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerifyProofRequest {
    /// Lineage proof to verify
    pub proof: LineageProof,
}

/// Verification result
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerificationResult {
    /// Whether proof is valid
    pub valid: bool,
    /// Optional verification message
    pub message: Option<String>,
}

/// Lineage routes
pub fn routes(state: LineageApiState) -> Router {
    Router::new()
        .route("/proof/generate", post(generate_proof))
        .route("/proof/verify", post(verify_proof))
        .with_state(state)
}

/// POST /lineage/proof/generate - Generate lineage proof for node
async fn generate_proof(
    State(_state): State<LineageApiState>,
    Json(_req): Json<GenerateProofRequest>,
) -> Result<Json<ApiResponse<LineageProof>>, ApiError> {
    // TODO: Implement when LineageProofManager async methods are available
    warn!("🚧 Lineage proof generation endpoint not yet implemented");
    Err(ApiError::internal(
        "Lineage proof generation not yet implemented - awaiting async manager methods",
    ))
}

/// POST /lineage/proof/verify - Verify lineage proof
async fn verify_proof(
    State(_state): State<LineageApiState>,
    Json(_req): Json<VerifyProofRequest>,
) -> Result<Json<ApiResponse<VerificationResult>>, ApiError> {
    // TODO: Implement when LineageProofManager async methods are available
    warn!("🚧 Lineage proof verification endpoint not yet implemented");
    Err(ApiError::internal(
        "Lineage proof verification not yet implemented - awaiting async manager methods",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_proof_request() {
        let req = GenerateProofRequest {
            node_id: "node-123".into(),
            chain_id: "chain-abc".into(),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("node-123"));
        assert!(json.contains("chain-abc"));
    }

    #[test]
    fn test_verification_result() {
        let result = VerificationResult {
            valid: true,
            message: Some("Success".into()),
        };

        assert!(result.valid);
        assert_eq!(result.message, Some("Success".into()));
    }
}
