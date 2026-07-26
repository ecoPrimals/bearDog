// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use std::sync::Arc;

#[tokio::test]
async fn test_generate_and_verify_proof() -> Result<(), BearDogError> {
    // Setup
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());

    // Create lineage
    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-1".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "child-1", "grandchild-1".to_string(), None)?;

    // Generate proof for grandchild
    let proof = proof_manager.generate_proof(&chain.chain_id, "grandchild-1")?;

    assert_eq!(proof.node_id, "grandchild-1");
    assert_eq!(proof.root_id, "root");
    assert_eq!(proof.path, vec!["root", "child-1", "grandchild-1"]);
    assert_eq!(proof.proof_chain.len(), 2);

    // Verify proof
    let result = proof_manager.verify_proof(&proof, &chain.chain_id)?;
    assert!(result.valid);
    assert_eq!(result.depth, 2);
    assert!(result.failure_reason.is_none());

    Ok(())
}

#[tokio::test]
async fn test_is_descendant() -> Result<(), BearDogError> {
    // Setup
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());

    // Create lineage
    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-1".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "child-1", "grandchild-1".to_string(), None)?;

    // Test descendant check
    assert!(proof_manager.is_descendant(&chain.chain_id, "root", "child-1")?);
    assert!(proof_manager.is_descendant(&chain.chain_id, "root", "grandchild-1")?);
    assert!(proof_manager.is_descendant(&chain.chain_id, "child-1", "grandchild-1")?);
    assert!(!proof_manager.is_descendant(&chain.chain_id, "child-1", "root")?);

    Ok(())
}

#[tokio::test]
async fn test_generate_proof_chain_not_found() {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager);

    let result = proof_manager.generate_proof("nonexistent", "node-1");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Chain not found"));
}

#[tokio::test]
async fn test_generate_proof_node_not_found() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());

    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;

    let result = proof_manager.generate_proof(&chain.chain_id, "nonexistent");
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_verify_proof_chain_not_found() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());

    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-1".to_string(), None)?;

    let proof = proof_manager.generate_proof(&chain.chain_id, "child-1")?;

    // Verify against wrong chain ID
    let result = proof_manager.verify_proof(&proof, "nonexistent");
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_verify_proof_root_mismatch() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());

    // Create two separate chains
    let chain1 = chain_manager.generate_root_chain("root-1".to_string(), None)?;
    chain_manager.add_child(&chain1.chain_id, "root-1", "child-1".to_string(), None)?;

    let chain2 = chain_manager.generate_root_chain("root-2".to_string(), None)?;

    // Generate proof for chain1
    let proof = proof_manager.generate_proof(&chain1.chain_id, "child-1")?;

    // Verify against chain2 (root mismatch)
    let result = proof_manager.verify_proof(&proof, &chain2.chain_id)?;
    assert!(!result.valid);
    assert!(
        result
            .failure_reason
            .as_ref()
            .expect("invalid proof failure reason")
            .contains("Root mismatch")
    );
    Ok(())
}

#[tokio::test]
async fn test_verify_proof_merkle_mismatch() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());

    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-1".to_string(), None)?;

    let mut proof = proof_manager.generate_proof(&chain.chain_id, "child-1")?;

    // Tamper with merkle root
    proof.merkle_root = vec![0u8; 32];

    let result = proof_manager.verify_proof(&proof, &chain.chain_id)?;
    assert!(!result.valid);
    assert!(
        result
            .failure_reason
            .as_ref()
            .expect("invalid proof failure reason")
            .contains("Merkle root mismatch")
    );
    Ok(())
}

#[tokio::test]
async fn test_verify_proof_path_length_mismatch() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());

    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-1".to_string(), None)?;

    let mut proof = proof_manager.generate_proof(&chain.chain_id, "child-1")?;

    // Tamper with path to cause length mismatch
    proof.path.push("extra-node".to_string());

    let result = proof_manager.verify_proof(&proof, &chain.chain_id)?;
    assert!(!result.valid);
    assert!(
        result
            .failure_reason
            .as_ref()
            .expect("invalid proof failure reason")
            .contains("Path length mismatch")
    );
    Ok(())
}

#[tokio::test]
async fn test_is_descendant_no_relationship() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());

    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-1".to_string(), None)?;

    // child-1 is not an ancestor of root
    assert!(!proof_manager.is_descendant(&chain.chain_id, "child-1", "root")?);

    // non-existent node
    assert!(!proof_manager.is_descendant(&chain.chain_id, "root", "non-existent")?);
    Ok(())
}

#[tokio::test]
async fn test_common_ancestor_no_match() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());

    // Single chain, one node
    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;

    // Non-existent nodes
    assert!(
        proof_manager
            .get_common_ancestor(&chain.chain_id, "nonexistent-a", "nonexistent-b")
            .is_none()
    );
    Ok(())
}

#[tokio::test]
async fn test_common_ancestor() -> Result<(), BearDogError> {
    // Setup
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());

    // Create lineage with multiple branches
    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-1".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-2".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "child-1", "grandchild-1".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "child-2", "grandchild-2".to_string(), None)?;

    // Test common ancestor
    let ancestor =
        proof_manager.get_common_ancestor(&chain.chain_id, "grandchild-1", "grandchild-2");
    assert_eq!(ancestor, Some("root".to_string()));

    let ancestor = proof_manager.get_common_ancestor(&chain.chain_id, "grandchild-1", "child-1");
    assert_eq!(ancestor, Some("child-1".to_string()));

    Ok(())
}

#[tokio::test]
async fn test_verify_proof_unknown_chain_errors() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());
    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-1".to_string(), None)?;
    let proof = proof_manager.generate_proof(&chain.chain_id, "child-1")?;
    let err = proof_manager
        .verify_proof(&proof, "nonexistent-chain-id")
        .expect_err("unknown chain");
    assert!(err.to_string().contains("Chain not found") || err.to_string().contains("chain"));
    Ok(())
}

// ── Genetic distance tests ──────────────────────────────────

#[tokio::test]
async fn test_genetic_distance_self() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());
    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;

    assert_eq!(
        proof_manager.genetic_distance(&chain.chain_id, "root", "root"),
        Some(0)
    );
    Ok(())
}

#[tokio::test]
async fn test_genetic_distance_parent_child() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());
    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-1".to_string(), None)?;

    // Parent→child = distance 1
    assert_eq!(
        proof_manager.genetic_distance(&chain.chain_id, "root", "child-1"),
        Some(1)
    );
    // Symmetric
    assert_eq!(
        proof_manager.genetic_distance(&chain.chain_id, "child-1", "root"),
        Some(1)
    );
    Ok(())
}

#[tokio::test]
async fn test_genetic_distance_siblings() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());
    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-1".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-2".to_string(), None)?;

    // Siblings: depth 1 + depth 1 − 2*depth(root=0) = 2
    assert_eq!(
        proof_manager.genetic_distance(&chain.chain_id, "child-1", "child-2"),
        Some(2)
    );
    Ok(())
}

#[tokio::test]
async fn test_genetic_distance_cousins() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());
    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-1".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "root", "child-2".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "child-1", "gc-1".to_string(), None)?;
    chain_manager.add_child(&chain.chain_id, "child-2", "gc-2".to_string(), None)?;

    // Cousins: depth 2 + depth 2 − 2*depth(root=0) = 4
    assert_eq!(
        proof_manager.genetic_distance(&chain.chain_id, "gc-1", "gc-2"),
        Some(4)
    );

    // Uncle: gc-1 (depth 2) to child-2 (depth 1), common ancestor root (depth 0)
    // = 2 + 1 − 2*0 = 3
    assert_eq!(
        proof_manager.genetic_distance(&chain.chain_id, "gc-1", "child-2"),
        Some(3)
    );
    Ok(())
}

#[tokio::test]
async fn test_genetic_distance_nonexistent_node() -> Result<(), BearDogError> {
    let chain_manager = Arc::new(LineageChainManager::new());
    let proof_manager = LineageProofManager::new(chain_manager.clone());
    let chain = chain_manager.generate_root_chain("root".to_string(), None)?;

    assert_eq!(
        proof_manager.genetic_distance(&chain.chain_id, "root", "ghost"),
        None
    );
    Ok(())
}

// ── Enrollment tier tests ──────────────────────────────────

#[test]
fn test_classify_enrollment_tier_identity() {
    assert_eq!(
        LineageProofManager::classify_enrollment_tier(0),
        GeneticEnrollmentTier::Identity
    );
}

#[test]
fn test_classify_enrollment_tier_kin() {
    assert_eq!(
        LineageProofManager::classify_enrollment_tier(1),
        GeneticEnrollmentTier::Kin
    );
}

#[test]
fn test_classify_enrollment_tier_sibling() {
    assert_eq!(
        LineageProofManager::classify_enrollment_tier(2),
        GeneticEnrollmentTier::Sibling
    );
}

#[test]
fn test_classify_enrollment_tier_extended() {
    assert_eq!(
        LineageProofManager::classify_enrollment_tier(3),
        GeneticEnrollmentTier::Extended
    );
    assert_eq!(
        LineageProofManager::classify_enrollment_tier(4),
        GeneticEnrollmentTier::Extended
    );
}

#[test]
fn test_classify_enrollment_tier_distant() {
    assert_eq!(
        LineageProofManager::classify_enrollment_tier(5),
        GeneticEnrollmentTier::Distant
    );
    assert_eq!(
        LineageProofManager::classify_enrollment_tier(100),
        GeneticEnrollmentTier::Distant
    );
}

#[test]
fn test_enrollment_tier_auto_enroll() {
    assert!(GeneticEnrollmentTier::Identity.auto_enroll());
    assert!(GeneticEnrollmentTier::Kin.auto_enroll());
    assert!(GeneticEnrollmentTier::Sibling.auto_enroll());
    assert!(GeneticEnrollmentTier::Extended.auto_enroll());
    assert!(!GeneticEnrollmentTier::Distant.auto_enroll());
}

#[test]
fn test_enrollment_tier_wire_names() {
    assert_eq!(GeneticEnrollmentTier::Identity.wire_name(), "identity");
    assert_eq!(GeneticEnrollmentTier::Kin.wire_name(), "kin");
    assert_eq!(GeneticEnrollmentTier::Sibling.wire_name(), "sibling");
    assert_eq!(GeneticEnrollmentTier::Extended.wire_name(), "extended");
    assert_eq!(GeneticEnrollmentTier::Distant.wire_name(), "distant");
}

#[test]
fn test_enrollment_tier_ordering() {
    assert!(GeneticEnrollmentTier::Identity < GeneticEnrollmentTier::Kin);
    assert!(GeneticEnrollmentTier::Kin < GeneticEnrollmentTier::Sibling);
    assert!(GeneticEnrollmentTier::Sibling < GeneticEnrollmentTier::Extended);
    assert!(GeneticEnrollmentTier::Extended < GeneticEnrollmentTier::Distant);
}
