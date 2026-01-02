//! Lineage API endpoints
//!
//! Cryptographic lineage proofs and verification for biomeOS integration.

use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};

use beardog_genetics::birdsong::{
    types::LineageMetadata, LineageChainManager, LineageID, LineageProof, LineageProofManager,
};
use parking_lot::RwLock;
use std::collections::HashMap;

use super::types::{ApiError, ApiResponse};

/// Lineage API state with proper mapping management
#[derive(Clone)]
pub struct LineageApiState {
    /// Lineage chain manager
    pub chain_manager: Arc<LineageChainManager>,
    /// Lineage proof manager
    pub proof_manager: Arc<LineageProofManager>,
    /// Mapping: lineage_id -> (chain_id, node_id)
    pub lineage_map: Arc<RwLock<HashMap<String, (String, String)>>>,
    /// Reverse mapping: root_node_id -> chain_id (for proof verification)
    pub root_to_chain: Arc<RwLock<HashMap<String, String>>>,
    /// Current node's lineage ID (if set)
    pub current_lineage: Arc<RwLock<Option<String>>>,
}

impl LineageApiState {
    /// Create new lineage API state
    pub fn new(
        chain_manager: Arc<LineageChainManager>,
        proof_manager: Arc<LineageProofManager>,
    ) -> Self {
        Self {
            chain_manager,
            proof_manager,
            lineage_map: Arc::new(RwLock::new(HashMap::new())),
            root_to_chain: Arc::new(RwLock::new(HashMap::new())),
            current_lineage: Arc::new(RwLock::new(None)),
        }
    }

    /// Store lineage mapping
    fn store_lineage(&self, lineage_id: &str, chain_id: &str, node_id: &str) {
        self.lineage_map.write().insert(
            lineage_id.to_string(),
            (chain_id.to_string(), node_id.to_string()),
        );
    }

    /// Store root node to chain mapping (for genesis lineages)
    fn store_root_mapping(&self, root_node_id: &str, chain_id: &str) {
        self.root_to_chain.write().insert(
            root_node_id.to_string(),
            chain_id.to_string(),
        );
    }

    /// Get chain_id and node_id for a lineage
    fn get_lineage(&self, lineage_id: &str) -> Option<(String, String)> {
        self.lineage_map.read().get(lineage_id).cloned()
    }

    /// Get chain ID from root node ID
    fn get_chain_from_root(&self, root_node_id: &str) -> Option<String> {
        self.root_to_chain.read().get(root_node_id).cloned()
    }

    /// Set current lineage
    pub fn set_current_lineage(&self, lineage_id: &str) {
        *self.current_lineage.write() = Some(lineage_id.to_string());
    }

    /// Get current lineage
    pub fn get_current_lineage(&self) -> Option<String> {
        self.current_lineage.read().clone()
    }
}

// ====================================================================================
// Request/Response Types
// ====================================================================================

/// Request to create a new genesis lineage
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateLineageRequest {
    /// Service type (e.g., "tower", "songbird")
    pub service_type: String,
    /// Optional metadata for the root node
    pub metadata: Option<LineageMetadata>,
}

/// Response from lineage creation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateLineageResponse {
    /// Lineage ID
    pub lineage_id: String,
    /// Creation timestamp
    pub created_at: String,
}

/// Request to spawn a child lineage
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpawnLineageRequest {
    /// Parent lineage ID
    pub parent_lineage: String,
    /// Service type for child (e.g., "songbird")
    pub service_type: String,
    /// Optional metadata for child node
    pub metadata: Option<LineageMetadata>,
}

/// Response from lineage spawn
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpawnLineageResponse {
    /// New child lineage ID
    pub lineage_id: String,
    /// Lineage proof for the child
    pub proof: LineageProof,
}

/// Request to sign/generate a lineage proof
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignLineageRequest {
    /// Lineage ID to sign
    pub lineage_id: String,
}

/// Request to check if two lineages share the same family
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SameFamilyRequest {
    /// First lineage ID
    pub lineage_a: String,
    /// Second lineage ID
    pub lineage_b: String,
}

/// Response from same family check
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SameFamilyResponse {
    /// Whether they share the same genesis
    pub same_family: bool,
    /// Common ancestor node ID (if they're in the same family)
    pub common_ancestor: Option<String>,
}

/// Response from current lineage query
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CurrentLineageResponse {
    /// Current lineage ID
    pub lineage_id: String,
    /// Genesis (root) node ID
    pub genesis: String,
    /// Children of this node
    pub children: Vec<String>,
    /// Depth in lineage tree
    pub depth: u32,
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

/// Verification result (Enhanced for biomeOS)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerificationResult {
    /// Whether proof is valid
    pub valid: bool,
    /// Whether they share the same genesis (biomeOS requirement)
    pub same_genesis: bool,
    /// Optional verification message
    pub message: Option<String>,
}

/// Lineage routes
pub fn routes(state: LineageApiState) -> Router {
    Router::new()
        // Genesis and spawning
        .route("/create", post(create_lineage))
        .route("/spawn", post(spawn_lineage))
        // Signing and verification
        .route("/sign", post(sign_lineage))
        .route("/proof/generate", post(generate_proof))
        .route("/proof/verify", post(verify_proof))
        // Family checking
        .route("/same_family", post(check_same_family))
        // Current state
        .route("/current", get(get_current_lineage))
        .with_state(state)
}

// ====================================================================================
// Endpoint Handlers
// ====================================================================================

/// POST /lineage/create - Create new genesis lineage
async fn create_lineage(
    State(state): State<LineageApiState>,
    Json(req): Json<CreateLineageRequest>,
) -> Result<Json<ApiResponse<CreateLineageResponse>>, ApiError> {
    info!("🌱 Creating new genesis lineage for service: {}", req.service_type);

    // Generate root chain
    let root_node_id = format!("{}-genesis", req.service_type);
    let chain = state
        .chain_manager
        .generate_root_chain(root_node_id.clone(), req.metadata)
        .await
        .map_err(|e| ApiError::internal(format!("Failed to create lineage: {}", e)))?;

    // Create LineageID
    let lineage_id = LineageID::format(&req.service_type, &chain.chain_id, &root_node_id);

    // Store mapping for future operations
    state.store_lineage(lineage_id.as_str(), &chain.chain_id, &root_node_id);
    
    // Store root mapping for proof verification
    state.store_root_mapping(&root_node_id, &chain.chain_id);

    // Set as current lineage (for genesis nodes)
    state.set_current_lineage(lineage_id.as_str());

    let response = CreateLineageResponse {
        lineage_id: lineage_id.to_string(),
        created_at: chain.created_at.to_rfc3339(),
    };

    info!("✅ Genesis lineage created: {}", lineage_id);
    Ok(Json(ApiResponse::success(response)))
}

/// POST /lineage/spawn - Spawn child lineage from parent
async fn spawn_lineage(
    State(state): State<LineageApiState>,
    Json(req): Json<SpawnLineageRequest>,
) -> Result<Json<ApiResponse<SpawnLineageResponse>>, ApiError> {
    info!(
        "👶 Spawning child lineage of type {} from parent {}",
        req.service_type, req.parent_lineage
    );

    // Look up parent lineage mapping
    let (parent_chain_id, parent_node_id) = state
        .get_lineage(&req.parent_lineage)
        .ok_or_else(|| {
            ApiError::bad_request(&format!(
                "Parent lineage not found: {}. Create genesis first with /lineage/create",
                req.parent_lineage
            ))
        })?;

    // Generate child node ID
    let child_node_id = format!("{}-{}", req.service_type, uuid::Uuid::new_v4());

    // Add child to the parent's chain
    let child_node = state
        .chain_manager
        .add_child(&parent_chain_id, &parent_node_id, child_node_id.clone(), req.metadata)
        .await
        .map_err(|e| ApiError::internal(format!("Failed to spawn child: {}", e)))?;

    // Create LineageID for child (same chain as parent)
    let child_lineage_id = LineageID::format(&req.service_type, &parent_chain_id, &child_node_id);

    // Store mapping for child
    state.store_lineage(child_lineage_id.as_str(), &parent_chain_id, &child_node_id);
    
    // Update current lineage to the child
    state.set_current_lineage(child_lineage_id.as_str());

    // Generate proof for child
    let proof = state
        .proof_manager
        .generate_proof(&parent_chain_id, &child_node_id)
        .map_err(|e| ApiError::internal(format!("Failed to generate proof: {}", e)))?;

    let response = SpawnLineageResponse {
        lineage_id: child_lineage_id.to_string(),
        proof,
    };

    info!("✅ Child lineage spawned: {} (depth: {})", child_lineage_id, child_node.depth);
    Ok(Json(ApiResponse::success(response)))
}

/// POST /lineage/sign - Sign lineage (generate proof)
async fn sign_lineage(
    State(state): State<LineageApiState>,
    Json(req): Json<SignLineageRequest>,
) -> Result<Json<ApiResponse<LineageProof>>, ApiError> {
    info!("🔐 Signing lineage: {}", req.lineage_id);

    // Look up lineage mapping
    let (chain_id, node_id) = state
        .get_lineage(&req.lineage_id)
        .ok_or_else(|| {
            ApiError::not_found(&format!(
                "Lineage not found: {}. Lineage must be created or spawned first",
                req.lineage_id
            ))
        })?;

    // Generate proof
    let proof = state
        .proof_manager
        .generate_proof(&chain_id, &node_id)
        .map_err(|e| ApiError::internal(format!("Failed to sign lineage: {}", e)))?;

    info!("✅ Lineage signed successfully");
    Ok(Json(ApiResponse::success(proof)))
}

/// POST /lineage/same_family - Check if two lineages share same genesis
async fn check_same_family(
    State(state): State<LineageApiState>,
    Json(req): Json<SameFamilyRequest>,
) -> Result<Json<ApiResponse<SameFamilyResponse>>, ApiError> {
    info!(
        "🔍 Checking family relationship: {} vs {}",
        req.lineage_a, req.lineage_b
    );

    // Look up both lineages
    let (chain_id_a, node_id_a) = state
        .get_lineage(&req.lineage_a)
        .ok_or_else(|| ApiError::not_found(&format!("Lineage not found: {}", req.lineage_a)))?;

    let (chain_id_b, node_id_b) = state
        .get_lineage(&req.lineage_b)
        .ok_or_else(|| ApiError::not_found(&format!("Lineage not found: {}", req.lineage_b)))?;

    // Check if same chain (simple same-family check)
    let same_family = chain_id_a == chain_id_b;

    // If same chain, find common ancestor
    let common_ancestor = if same_family {
        state
            .proof_manager
            .get_common_ancestor(&chain_id_a, &node_id_a, &node_id_b)
    } else {
        None
    };

    let response = SameFamilyResponse {
        same_family,
        common_ancestor,
    };

    info!("✅ Family check complete: {}", same_family);
    Ok(Json(ApiResponse::success(response)))
}

/// GET /lineage/current - Get current lineage state
async fn get_current_lineage(
    State(state): State<LineageApiState>,
) -> Result<Json<ApiResponse<CurrentLineageResponse>>, ApiError> {
    info!("📊 Querying current lineage state");

    // Get current lineage ID
    let current_lineage_id = state
        .get_current_lineage()
        .ok_or_else(|| {
            ApiError::not_found(
                "No current lineage set. Create a genesis lineage with /lineage/create first"
            )
        })?;

    // Look up lineage details
    let (chain_id, node_id) = state
        .get_lineage(&current_lineage_id)
        .ok_or_else(|| ApiError::internal("Current lineage mapping corrupted"))?;

    // Get chain to extract details
    let chain = state
        .chain_manager
        .get_chain(&chain_id)
        .ok_or_else(|| ApiError::internal("Chain not found for current lineage"))?;

    // Get current node
    let current_node = chain
        .nodes
        .get(&node_id)
        .ok_or_else(|| ApiError::internal("Node not found in chain"))?;

    // Get children of current node
    let descendants = state.chain_manager.get_descendants(&chain_id, &node_id);
    let direct_children: Vec<String> = descendants
        .iter()
        .filter(|desc| desc.parent_id.as_ref() == Some(&node_id))
        .map(|desc| desc.node_id.clone())
        .collect();

    let response = CurrentLineageResponse {
        lineage_id: current_lineage_id,
        genesis: chain.root_node.node_id.clone(),
        children: direct_children,
        depth: current_node.depth,
    };

    info!("✅ Current lineage retrieved: {} (depth: {})", response.lineage_id, response.depth);
    Ok(Json(ApiResponse::success(response)))
}

/// POST /lineage/proof/generate - Generate lineage proof for node
async fn generate_proof(
    State(state): State<LineageApiState>,
    Json(req): Json<GenerateProofRequest>,
) -> Result<Json<ApiResponse<LineageProof>>, ApiError> {
    info!(
        "🔐 Lineage proof generation request for node {}",
        req.node_id
    );

    // Generate proof using LineageProofManager
    let proof = state
        .proof_manager
        .generate_proof(&req.chain_id, &req.node_id)
        .map_err(|e| {
            warn!("Failed to generate proof: {}", e);
            ApiError::internal(format!("Proof generation failed: {}", e))
        })?;

    info!("✅ Lineage proof generated successfully");
    Ok(Json(ApiResponse::success(proof)))
}

/// POST /lineage/proof/verify - Verify lineage proof (Enhanced for biomeOS)
async fn verify_proof(
    State(state): State<LineageApiState>,
    Json(req): Json<VerifyProofRequest>,
) -> Result<Json<ApiResponse<VerificationResult>>, ApiError> {
    info!(
        "🔍 Lineage proof verification request for node {}",
        req.proof.node_id
    );

    // Look up chain_id from root_id using our reverse mapping
    let chain_id = state
        .get_chain_from_root(&req.proof.root_id)
        .ok_or_else(|| {
            ApiError::bad_request(&format!(
                "Unknown lineage root: {}. Create a genesis lineage first with /lineage/create",
                req.proof.root_id
            ))
        })?;

    // Verify proof using LineageProofManager
    let result = state
        .proof_manager
        .verify_proof(&req.proof, &chain_id)
        .map_err(|e| {
            warn!("Failed to verify proof: {}", e);
            ApiError::internal(format!("Proof verification failed: {}", e))
        })?;

    // Check if same genesis as current lineage (for biomeOS integration)
    // Strategy: Compare the verified proof's chain_id with current lineage's chain_id
    let same_genesis = if result.valid {
        if let Some(current_lineage_id) = state.get_current_lineage() {
            if let Some((current_chain_id, _)) = state.get_lineage(&current_lineage_id) {
                // Same genesis if they share the same chain_id
                // (chain_id is unique per genesis lineage)
                chain_id == current_chain_id
            } else {
                // Current lineage not in map - assume different for safety
                false
            }
        } else {
            // No current lineage set - first verification, treat as same_genesis
            true
        }
    } else {
        // Invalid proof - not same genesis
        false
    };

    // Build response
    let response = VerificationResult {
        valid: result.valid,
        same_genesis,
        message: result.failure_reason,
    };

    if result.valid {
        info!(
            "✅ Lineage proof verified successfully (depth: {}, same_genesis: {})",
            result.depth, same_genesis
        );
    } else {
        warn!("❌ Lineage proof verification failed");
    }

    Ok(Json(ApiResponse::success(response)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

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
            same_genesis: true,
            message: Some("Success".into()),
        };

        assert!(result.valid);
        assert!(result.same_genesis);
        assert_eq!(result.message, Some("Success".into()));
    }

    #[test]
    fn test_verify_proof_request_serialization() {
        let proof = LineageProof {
            node_id: "node-456".into(),
            root_id: "root-789".into(),
            path: vec!["root-789".into(), "node-456".into()],
            proof_chain: vec![],
            merkle_root: vec![0; 32],
            generated_at: Utc::now(),
        };

        let req = VerifyProofRequest { proof };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("node-456"));
        assert!(json.contains("root-789"));
    }

    #[test]
    fn test_verification_result_with_no_message() {
        let result = VerificationResult {
            valid: false,
            same_genesis: false,
            message: None,
        };

        assert!(!result.valid);
        assert!(!result.same_genesis);
        assert!(result.message.is_none());
    }

    #[test]
    fn test_verification_result_with_failure_message() {
        let result = VerificationResult {
            valid: false,
            same_genesis: false,
            message: Some("Root mismatch".into()),
        };

        assert!(!result.valid);
        assert!(!result.same_genesis);
        assert_eq!(result.message, Some("Root mismatch".into()));
    }

    #[test]
    fn test_generate_proof_request_roundtrip() {
        let req = GenerateProofRequest {
            node_id: "test-node".into(),
            chain_id: "test-chain".into(),
        };

        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GenerateProofRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(req.node_id, deserialized.node_id);
        assert_eq!(req.chain_id, deserialized.chain_id);
    }

    #[test]
    fn test_lineage_proof_structure() {
        let proof = LineageProof {
            node_id: "child".into(),
            root_id: "root".into(),
            path: vec!["root".into(), "parent".into(), "child".into()],
            proof_chain: vec![],
            merkle_root: vec![1, 2, 3, 4],
            generated_at: Utc::now(),
        };

        assert_eq!(proof.path.len(), 3);
        assert_eq!(proof.node_id, "child");
        assert_eq!(proof.root_id, "root");
        assert_eq!(proof.merkle_root.len(), 4);
    }

    #[test]
    fn test_empty_proof_chain() {
        let proof = LineageProof {
            node_id: "node".into(),
            root_id: "root".into(),
            path: vec![],
            proof_chain: vec![],
            merkle_root: vec![],
            generated_at: Utc::now(),
        };

        assert!(proof.proof_chain.is_empty());
        assert!(proof.path.is_empty());
    }

    #[test]
    fn test_verification_success_result() {
        let result = VerificationResult {
            valid: true,
            same_genesis: true,
            message: Some("Proof verified successfully".into()),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("true"));
        assert!(json.contains("successfully"));
    }

    #[test]
    fn test_generate_request_with_empty_ids() {
        let req = GenerateProofRequest {
            node_id: "".into(),
            chain_id: "".into(),
        };

        // Should serialize even with empty strings
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("node_id"));
        assert!(json.contains("chain_id"));
    }

    #[tokio::test]
    async fn test_lineage_api_state_creation() {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = Arc::new(LineageProofManager::new(chain_manager.clone()));

        let state = LineageApiState::new(chain_manager, proof_manager);

        assert!(state.lineage_map.read().is_empty());
        assert!(state.current_lineage.read().is_none());
    }

    #[tokio::test]
    async fn test_store_and_retrieve_lineage() {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = Arc::new(LineageProofManager::new(chain_manager.clone()));
        let state = LineageApiState::new(chain_manager, proof_manager);

        // Store lineage
        state.store_lineage("lineage-1", "chain-1", "node-1");

        // Retrieve
        let map = state.lineage_map.read();
        let entry = map.get("lineage-1").unwrap();
        assert_eq!(entry.0, "chain-1");
        assert_eq!(entry.1, "node-1");
    }

    #[tokio::test]
    async fn test_set_and_get_current_lineage() {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = Arc::new(LineageProofManager::new(chain_manager.clone()));
        let state = LineageApiState::new(chain_manager, proof_manager);

        // Set current
        *state.current_lineage.write() = Some("lineage-current".to_string());

        // Get current
        let current = state.current_lineage.read();
        assert_eq!(current.as_ref().unwrap(), "lineage-current");
    }

    #[tokio::test]
    async fn test_lineage_api_state_concurrent_access() {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = Arc::new(LineageProofManager::new(chain_manager.clone()));
        let state = Arc::new(LineageApiState::new(chain_manager, proof_manager));

        // Spawn multiple tasks writing to lineage map
        let mut handles = vec![];
        for i in 0..100 {
            let state = Arc::clone(&state);
            handles.push(tokio::spawn(async move {
                state.store_lineage(
                    &format!("lineage-{}", i),
                    &format!("chain-{}", i),
                    &format!("node-{}", i),
                );
            }));
        }

        futures::future::join_all(handles).await;

        // Verify all entries
        let map = state.lineage_map.read();
        assert_eq!(map.len(), 100);
    }

    #[test]
    fn test_create_lineage_request_serialization() {
        let req = CreateLineageRequest {
            service_type: "tower".to_string(),
            metadata: Some(vec![("key".to_string(), "value".to_string())]),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("tower"));
        assert!(json.contains("key"));
        assert!(json.contains("value"));
    }

    #[test]
    fn test_spawn_lineage_request_serialization() {
        let req = SpawnLineageRequest {
            parent_lineage: "lineage:parent:123".to_string(),
            service_type: "songbird".to_string(),
            metadata: None,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("parent"));
        assert!(json.contains("songbird"));
    }

    #[test]
    fn test_same_family_request_serialization() {
        let req = SameFamilyRequest {
            lineage_a: "lineage:a:123".to_string(),
            lineage_b: "lineage:b:456".to_string(),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("lineage_a"));
        assert!(json.contains("lineage_b"));
    }

    #[test]
    fn test_same_family_response_serialization() {
        let resp = SameFamilyResponse {
            same_family: true,
            common_ancestor: Some("lineage:root:000".to_string()),
        };

        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("same_family"));
        assert!(json.contains("common_ancestor"));
        assert!(json.contains("root"));
    }

    #[test]
    fn test_current_lineage_response() {
        let resp = CurrentLineageResponse {
            lineage_id: "lineage:test:123:abc:node1".to_string(),
            genesis: "node0".to_string(),
            children: vec!["child1".to_string(), "child2".to_string()],
            depth: 2,
        };

        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("lineage:test:123"));
        assert!(json.contains("node0"));
        assert!(json.contains("child1"));
        assert_eq!(resp.depth, 2);
    }
}
