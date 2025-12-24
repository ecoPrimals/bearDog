//! Genesis API endpoints
//!
//! Physical genesis bootstrap for new node creation via witness ceremonies.

use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};

use beardog_genetics::birdsong::{
    GenesisCeremonyResult, GenesisLineageProvider, GenesisWitness, GeneticLineage,
    PhysicalChannelProof,
};

use super::types::{ApiError, ApiResponse};

/// Genesis API state
#[derive(Clone)]
pub struct GenesisApiState {
    /// Genesis lineage provider
    pub provider: Arc<GenesisLineageProvider>,
}

/// Request to establish genesis lineage
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstablishLineageRequest {
    /// New node identifier
    pub node_id: String,
    /// Genesis witness
    pub witness: GenesisWitness,
}

/// Request for full genesis ceremony
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenesisCeremonyRequest {
    /// New node identifier
    pub node_id: String,
    /// Genesis witness
    pub witness: GenesisWitness,
    /// Physical channel proof
    pub physical_proof: PhysicalChannelProof,
}

/// Genesis routes
pub fn routes(state: GenesisApiState) -> Router {
    Router::new()
        .route("/lineage", post(establish_lineage))
        .route("/ceremony", post(conduct_ceremony))
        .with_state(state)
}

/// POST /genesis/lineage - Establish genetic lineage for new node
async fn establish_lineage(
    State(state): State<GenesisApiState>,
    Json(req): Json<EstablishLineageRequest>,
) -> Result<Json<ApiResponse<GeneticLineage>>, ApiError> {
    info!("🔐 Establishing genesis lineage for node: {}", req.node_id);

    let lineage = state
        .provider
        .establish_genesis_lineage(&req.node_id, &req.witness)
        .await
        .map_err(|e| {
            warn!("Failed to establish genesis lineage: {}", e);
            ApiError::from(e)
        })?;

    let trust_stars = lineage.trust_level.stars();
    info!(
        "✅ Genesis lineage established for {} with trust level {}",
        req.node_id, trust_stars
    );

    Ok(Json(ApiResponse::success_with_message(
        lineage,
        format!(
            "Genesis lineage established with trust level {}",
            trust_stars
        ),
    )))
}

/// POST /genesis/ceremony - Conduct full genesis ceremony
async fn conduct_ceremony(
    State(state): State<GenesisApiState>,
    Json(req): Json<GenesisCeremonyRequest>,
) -> Result<Json<ApiResponse<GenesisCeremonyResult>>, ApiError> {
    info!("🔐 Conducting genesis ceremony for node: {}", req.node_id);

    let result = state
        .provider
        .conduct_genesis_ceremony(&req.node_id, &req.witness, &req.physical_proof)
        .await
        .map_err(|e| {
            warn!("Failed to conduct genesis ceremony: {}", e);
            ApiError::from(e)
        })?;

    if result.success {
        info!(
            "✅ Genesis ceremony complete for {} with trust level {}",
            req.node_id,
            result.genetic_lineage.trust_level.stars()
        );
        Ok(Json(ApiResponse::success(result)))
    } else {
        warn!(
            "❌ Genesis ceremony failed for {}: {:?}",
            req.node_id, result.error
        );
        Err(ApiError::bad_request(
            result
                .error
                .unwrap_or_else(|| "Genesis ceremony failed".into()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_genetics::birdsong::PhysicalChannelType;

    #[tokio::test]
    async fn test_establish_lineage_request_serialization() {
        let witness = GenesisWitness {
            device_id: "test-witness".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![0u8; 64],
        };

        let req = EstablishLineageRequest {
            node_id: "test-node".into(),
            witness,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test-node"));
        assert!(json.contains("test-witness"));
    }
}
