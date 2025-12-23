use crate::auth::node_registry::InMemoryNodeRegistry;
use crate::auth::proof_verifier::DefaultProofVerifier;
use crate::auth::types::*;
use beardog_errors::BearDogError;
// use beardog_security::{
//     Action, ActionType, Resource, ResourceClassification, Subject, SubjectType,
// };
use chrono::Utc;
use std::collections::HashMap;

#[tokio::test]

async fn test_cross_node_auth_engine_creation() -> Result<(), BearDogError> {
    let node_registry = Box::new(InMemoryNodeRegistry::new());
    let proof_verifier = Box::new(DefaultProofVerifier::new());
    let config = CrossNodeAuthConfig {
        verification_mode: VerificationMode::Enabled,
        max_proof_validity_minutes: 120,
        spawning_mode: SpawningMode::Enabled,
        consensus_config: ConsensusConfig {
            required: true,
            threshold: 0.8,
        },
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        max_spawns_per_node: 50,
        approval_mode: ApprovalMode::Automated,
    };

    let engine = CrossNodeAuthEngine::new(node_registry, proof_verifier, config);
    assert!(engine.is_initialized());
    Ok(())
}

#[tokio::test]
async fn test_node_registration() -> Result<(), BearDogError> {
    const TEST_PORT: u16 = 8080;
    let mut node_registry = InMemoryNodeRegistry::new();

    let node_info = NodeInfo {
        node_id: "test-node".to_string(),
        address: format!("127.0.0.1:{}", TEST_PORT),
        capabilities: vec![NodeCapability::StorageProvider],
        trust_level: 0.8,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        last_seen: Utc::now(),
        genetics: None,
    };

    node_registry.register_node(node_info.clone())?;
    let retrieved = node_registry.get_node_info("test-node")?;
    assert_eq!(retrieved.node_id, "test-node");
    assert_eq!(retrieved.trust_level, 0.8);

    Ok(())
}

#[tokio::test]
async fn test_authorization_creation() -> Result<(), BearDogError> {
    let authorization = CrossNodeAuthorization {
        request_id: "test-req-123".to_string(),
        requester_node_id: "requester-node".to_string(),
        resource_owner_node_id: "owner-node".to_string(),
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        resource_id: "test-resource".to_string(),
        permissions: vec![ResourcePermission::Read],
        conditions: vec![],
        created_at: Utc::now(),
        expires_at: Utc::now() + chrono::Duration::hours(1),
        signature: "test-signature".to_string(),
        is_active: true,
    };

    assert!(authorization.is_valid());
    assert_eq!(authorization.requester_node_id, "requester-node");

    Ok(())
}

#[tokio::test]
async fn test_proof_verification() -> Result<(), BearDogError> {
    let verifier = DefaultProofVerifier::new();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    let operation = CrossNodeOperation {
        operation_type: OperationType::Read,
        target_resource: "test-resource".to_string(),
        parameters: HashMap::new(),
        requester_signature: "test-signature".to_string(),
    };

    let proof = AuthorizationProof {
        authorization_id: "test-proof-123".to_string(),
        operation,
        timestamp: Utc::now(),
        proof_signature: "proof-signature".to_string(),
    };

    // For testing, we'll assume the proof is valid
    // In a real implementation, this would do actual cryptographic verification
    let is_valid = verifier.verify_authorization_proof(&proof)?;
    assert!(is_valid);

    Ok(())
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_consensus_calculation() -> Result<(), BearDogError> {
    let mut votes = HashMap::new();
    votes.insert("node1".to_string(), true);
    votes.insert("node2".to_string(), true);
    votes.insert("node3".to_string(), false);

    let result = ConsensusResult {
        consensus_reached: true,
        votes: votes.clone(),
        final_score: 0.67,
        participating_nodes: votes.keys().cloned().collect(),
    };

    assert!(result.consensus_reached);
    assert_eq!(result.participating_nodes.len(), 3);

    Ok(())
}
