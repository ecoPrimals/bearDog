//! Auth module tests
//!
//! Comprehensive unit tests for the cross-node authorization functionality.

use super::*;
use crate::{BearDogError, BearDogResult};
use crate::auth::types::{NodeInfo, ResourcePermission};
use crate::auth::{
    BearDogGenetics, CryptoChromosome, SecurityTraits, AlgorithmFamily, 
    EncryptionFamily, NodeCapability, SpawnRequest, SpawnPurpose, ResourceLimits
};
use crate::security::types::{Subject, SubjectType, Resource, ResourceClassification, Action, ActionType};
use std::collections::HashMap;
use chrono::{Utc, Duration};
use uuid::Uuid;

// Mock implementations for testing
struct MockNodeRegistry {
    nodes: HashMap<String, NodeInfo>,
    trust_levels: HashMap<String, f64>,
}

impl MockNodeRegistry {
    fn new() -> Self {
        let mut registry = Self {
            nodes: HashMap::new(),
            trust_levels: HashMap::new(),
        };
        
        // Add some test nodes
        let test_node = NodeInfo {
            id: "test_node_1".to_string(),
            address: "127.0.0.1:8080".to_string(),
            capabilities: vec![NodeCapability::StorageProvider, NodeCapability::SecurityAnalysis],
            trust_level: 0.8,
            last_seen: Utc::now(),
            genetics: None,
        };
        
        registry.nodes.insert("test_node_1".to_string(), test_node);
        registry.trust_levels.insert("test_node_1".to_string(), 0.8);
        
        registry
    }
}

impl NodeRegistry for MockNodeRegistry {
    fn get_node_info(&self, node_id: &str) -> BearDogResult<NodeInfo> {
        self.nodes.get(node_id)
            .cloned()
            .ok_or_else(|| BearDogError::NotFound { 
                resource_type: "Node".to_string(), 
                id: node_id.to_string() 
            })
    }

    fn register_node(&mut self, node_info: NodeInfo) -> BearDogResult<()> {
        self.trust_levels.insert(node_info.id.clone(), node_info.trust_level);
        self.nodes.insert(node_info.id.clone(), node_info);
        Ok(())
    }

    fn get_trust_level(&self, node_id: &str) -> BearDogResult<f64> {
        self.trust_levels.get(node_id)
            .copied()
            .ok_or_else(|| BearDogError::NotFound { 
                resource_type: "TrustLevel".to_string(), 
                id: node_id.to_string() 
            })
    }

    fn update_trust_level(&mut self, node_id: &str, trust_level: f64) -> BearDogResult<()> {
        self.trust_levels.insert(node_id.to_string(), trust_level);
        Ok(())
    }
}

struct MockProofVerifier;

impl ProofVerifier for MockProofVerifier {
    fn verify_authorization_proof(&self, _proof: &AuthorizationProof) -> BearDogResult<bool> {
        Ok(true) // Always verify for testing
    }

    fn generate_proof(
        &self,
        authorization: &CrossNodeAuthorization,
        operation: &CrossNodeOperation,
    ) -> BearDogResult<AuthorizationProof> {
        Ok(AuthorizationProof {
            authorization_id: authorization.id.clone(),
            operation: operation.clone(),
            timestamp: Utc::now(),
            proof_signature: "mock_signature".to_string(),
        })
    }
}

#[tokio::test]
async fn test_auth_engine_creation() {
    let node_registry = Box::new(MockNodeRegistry::new());
    let proof_verifier = Box::new(MockProofVerifier);
    
    let engine = CrossNodeAuthEngine::new(node_registry, proof_verifier);
    assert!(engine.active_authorizations.is_empty());
    assert!(engine.spawned_beardogs.is_empty());
}

#[tokio::test]
async fn test_create_authorization() -> BearDogResult<()> {
    let node_registry = Box::new(MockNodeRegistry::new());
    let proof_verifier = Box::new(MockProofVerifier);
    
    let mut engine = CrossNodeAuthEngine::new(node_registry, proof_verifier);
    
    let subject = Subject {
        id: "test_node_1".to_string(),
        subject_type: SubjectType::System,
        attributes: HashMap::new(),
        roles: vec!["node".to_string()],
        clearance_level: Some(5),
    };
    
    let resource = Resource {
        id: "test_resource".to_string(),
        resource_type: "data".to_string(),
        classification: ResourceClassification::Internal,
        attributes: HashMap::new(),
        owner: Some("test_node_2".to_string()),
    };
    
    let action = Action {
        action_type: ActionType::Read,
        context: HashMap::new(),
        timestamp: Utc::now(),
        source_ip: Some("127.0.0.1".to_string()),
    };
    
    let auth = engine.create_authorization(&subject, &resource, &action, "read").await?;
    
    assert!(auth.permitted);
    
    Ok(())
}

#[tokio::test]
async fn test_authorization_with_insufficient_trust() -> BearDogResult<()> {
    let mut node_registry = MockNodeRegistry::new();
    // Add a low-trust node
    let low_trust_node = NodeInfo {
        id: "low_trust_node".to_string(),
        address: "127.0.0.1:8081".to_string(),
        capabilities: vec![NodeCapability::NetworkRelay],
        trust_level: 0.3, // Below threshold
        last_seen: Utc::now(),
        genetics: None,
    };
    node_registry.register_node(low_trust_node).unwrap();
    
    let mut engine = CrossNodeAuthEngine::new(Box::new(node_registry), Box::new(MockProofVerifier));
    
    let subject = Subject {
        id: "low_trust_node".to_string(),
        subject_type: SubjectType::System,
        attributes: HashMap::new(),
        roles: vec!["node".to_string()],
        clearance_level: Some(2),
    };
    
    let resource = Resource {
        id: "test_resource".to_string(),
        resource_type: "data".to_string(),
        classification: ResourceClassification::Confidential,
        attributes: HashMap::new(),
        owner: Some("test_node_1".to_string()),
    };
    
    let action = Action {
        action_type: ActionType::Read,
        context: HashMap::new(),
        timestamp: Utc::now(),
        source_ip: Some("127.0.0.1".to_string()),
    };
    
    let auth = engine.create_authorization(&subject, &resource, &action, "read").await?;
    
    // With low trust, authorization should be denied
    assert!(!auth.permitted);
    
    Ok(())
}

#[tokio::test]
async fn test_resource_permission_implications() {
    assert!(ResourcePermission::Admin.implies(&ResourcePermission::Read));
    assert!(ResourcePermission::Admin.implies(&ResourcePermission::Write));
    assert!(ResourcePermission::Admin.implies(&ResourcePermission::Delete));
    assert!(ResourcePermission::Write.implies(&ResourcePermission::Read));
    assert!(ResourcePermission::Delete.implies(&ResourcePermission::Write));
    assert!(!ResourcePermission::Read.implies(&ResourcePermission::Write));
}

#[tokio::test]
async fn test_permission_security_levels() {
    assert!(ResourcePermission::Admin.security_level() > ResourcePermission::Delete.security_level());
    assert!(ResourcePermission::Delete.security_level() > ResourcePermission::Write.security_level());
    assert!(ResourcePermission::Write.security_level() > ResourcePermission::Read.security_level());
    assert_eq!(ResourcePermission::Admin.security_level(), 15);
    assert_eq!(ResourcePermission::Read.security_level(), 1);
}

#[tokio::test]
async fn test_authorization_validity() {
    let now = Utc::now();
    let auth = CrossNodeAuthorization {
        id: "test_auth".to_string(),
        requester_node_id: "node1".to_string(),
        resource_owner_node_id: "node2".to_string(),
        resource_id: "resource1".to_string(),
        permissions: vec![ResourcePermission::Read],
        conditions: vec![],
        created_at: now,
        expires_at: now + Duration::minutes(60),
        signature: "signature".to_string(),
        is_active: true,
    };
    
    assert!(auth.is_valid());
    
    let expired_auth = CrossNodeAuthorization {
        expires_at: now - Duration::minutes(60), // Expired
        ..auth.clone()
    };
    
    assert!(!expired_auth.is_valid());
    
    let inactive_auth = CrossNodeAuthorization {
        is_active: false,
        ..auth
    };
    
    assert!(!inactive_auth.is_valid());
}

#[tokio::test]
async fn test_genetics_registration() {
    let node_registry = Box::new(MockNodeRegistry::new());
    let proof_verifier = Box::new(MockProofVerifier);
    
    let mut engine = CrossNodeAuthEngine::new(node_registry, proof_verifier);
    
    let genetics = BearDogGenetics {
        id: "test_genetics".to_string(),
        crypto_chromosomes: vec![],
        security_traits: SecurityTraits::default(),
        capabilities: vec![NodeCapability::StorageProvider],
        spawn_restrictions: vec![],
        generation: 0,
        parent_genetics: None,
        mutations: vec![],
        fitness_score: 0.8,
    };
    
    let result = engine.register_genetics(genetics.clone()).await;
    assert!(result.is_ok());
    assert!(engine.genetics_registry.contains_key(&genetics.id));
}

#[tokio::test]
async fn test_beardog_spawning() {
    let node_registry = Box::new(MockNodeRegistry::new());
    let proof_verifier = Box::new(MockProofVerifier);
    
    let mut engine = CrossNodeAuthEngine::new(node_registry, proof_verifier);
    
    let parent_genetics = BearDogGenetics {
        id: "parent_genetics".to_string(),
        crypto_chromosomes: vec![CryptoChromosome {
            algorithm_family: AlgorithmFamily::Encryption(EncryptionFamily::Aes),
            strength_bits: 256,
            compatibility_score: 0.9,
            performance_factor: 0.8,
            security_level: 8,
        }],
        security_traits: SecurityTraits::default(),
        capabilities: vec![NodeCapability::StorageProvider, NodeCapability::SecurityAnalysis],
        spawn_restrictions: vec![],
        generation: 0,
        parent_genetics: None,
        mutations: vec![],
        fitness_score: 0.8,
    };
    
    let spawn_request = SpawnRequest {
        parent_genetics: vec![parent_genetics],
        spawn_purpose: SpawnPurpose::LoadBalancing,
        required_capabilities: vec![NodeCapability::StorageProvider],
        resource_limits: ResourceLimits::default(),
        target_environment: "test_env".to_string(),
    };
    
    let spawned = engine.spawn_beardog(spawn_request, "test_node_1").await;
    assert!(spawned.is_ok());
    
    let spawned = spawned.unwrap();
    assert_eq!(spawned.parent_id, "test_node_1");
    assert_eq!(spawned.genetics.generation, 1); // Child generation
    assert!(spawned.genetics.capabilities.contains(&NodeCapability::StorageProvider));
}

#[tokio::test]
async fn test_spawn_limits() {
    let mut config = CrossNodeAuthConfig::default();
    config.max_spawns_per_node = 1; // Only allow 1 spawn
    
    let node_registry = Box::new(MockNodeRegistry::new());
    let proof_verifier = Box::new(MockProofVerifier);
    
    let mut engine = CrossNodeAuthEngine::with_config(config, node_registry, proof_verifier);
    
    let parent_genetics = BearDogGenetics {
        id: "parent_genetics".to_string(),
        crypto_chromosomes: vec![],
        security_traits: SecurityTraits::default(),
        capabilities: vec![NodeCapability::StorageProvider],
        spawn_restrictions: vec![],
        generation: 0,
        parent_genetics: None,
        mutations: vec![],
        fitness_score: 0.8,
    };
    
    let spawn_request = SpawnRequest {
        parent_genetics: vec![parent_genetics.clone()],
        spawn_purpose: SpawnPurpose::LoadBalancing,
        required_capabilities: vec![NodeCapability::StorageProvider],
        resource_limits: ResourceLimits::default(),
        target_environment: "test_env".to_string(),
    };
    
    // First spawn should succeed
    let first_spawn = engine.spawn_beardog(spawn_request.clone(), "test_node_1").await;
    assert!(first_spawn.is_ok());
    
    // Second spawn should fail due to limit
    let second_spawn = engine.spawn_beardog(spawn_request, "test_node_1").await;
    assert!(second_spawn.is_err());
    
    if let Err(BearDogError::ResourceExhaustion(msg)) = second_spawn {
        assert!(msg.contains("Maximum spawns per node exceeded"));
    } else {
        panic!("Expected ResourceExhaustion error");
    }
}

#[tokio::test]
async fn test_ecosystem_capabilities() {
    let node_registry = Box::new(MockNodeRegistry::new());
    let proof_verifier = Box::new(MockProofVerifier);
    
    let mut engine = CrossNodeAuthEngine::new(node_registry, proof_verifier);
    
    // Create spawned BearDog with ecosystem capabilities
    let genetics = BearDogGenetics {
        id: "ecosystem_genetics".to_string(),
        crypto_chromosomes: vec![],
        security_traits: SecurityTraits::default(),
        capabilities: vec![
            NodeCapability::ToadStoolCompute,
            NodeCapability::SongBirdDiscovery,
            NodeCapability::StorageProvider,
        ],
        spawn_restrictions: vec![],
        generation: 0,
        parent_genetics: None,
        mutations: vec![],
        fitness_score: 0.9,
    };
    
    let spawned = SpawnedBearDog {
        id: "ecosystem_spawn".to_string(),
        parent_id: "test_node_1".to_string(),
        genetics,
        spawn_purpose: SpawnPurpose::EcosystemIntegration("ToadStool".to_string()),
        task_assignment: vec![],
        resource_limits: ResourceLimits::default(),
        spawn_time: Utc::now(),
        expected_lifetime: None,
        current_status: SpawnStatus::Active,
        performance_metrics: HashMap::new(),
        trust_relationships: HashMap::new(),
        consensus_participation: false,
        ecosystem_connections: vec!["ToadStool".to_string(), "SongBird".to_string()],
    };
    
    engine.spawned_beardogs.insert("ecosystem_spawn".to_string(), spawned);
    
    let capabilities = engine.get_ecosystem_capabilities("test_node_1");
    assert!(capabilities.contains(&NodeCapability::ToadStoolCompute));
    assert!(capabilities.contains(&NodeCapability::SongBirdDiscovery));
    assert!(!capabilities.contains(&NodeCapability::StorageProvider)); // Not ecosystem capability
}

#[tokio::test]
async fn test_network_effects_evaluation() {
    let node_registry = Box::new(MockNodeRegistry::new());
    let proof_verifier = Box::new(MockProofVerifier);
    
    let engine = CrossNodeAuthEngine::new(node_registry, proof_verifier);
    
    // Test single component
    let single_component = vec![NodeCapability::ToadStoolCompute];
    let score = engine.evaluate_network_effects(&single_component).await.unwrap();
    assert!(score > 0.0);
    assert!(score <= 1.0);
    
    // Test multiple components (should have synergy bonus)
    let multi_component = vec![
        NodeCapability::ToadStoolCompute,
        NodeCapability::SongBirdDiscovery,
        NodeCapability::NestGateStorage,
    ];
    let multi_score = engine.evaluate_network_effects(&multi_component).await.unwrap();
    
    // Multi-component should have higher score due to synergy
    assert!(multi_score > score);
}

#[tokio::test]
async fn test_consensus_mechanism() {
    let node_registry = Box::new(MockNodeRegistry::new());
    let proof_verifier = Box::new(MockProofVerifier);
    
    let engine = CrossNodeAuthEngine::new(node_registry, proof_verifier);
    
    let participating_nodes = vec!["test_node_1".to_string()];
    let result = engine.perform_consensus("test_proposal", &participating_nodes, 0.5).await;
    
    assert!(result.is_ok());
    let consensus = result.unwrap();
    assert_eq!(consensus.participating_nodes.len(), 1);
    assert!(consensus.votes.contains_key("test_node_1"));
}

#[tokio::test]
async fn test_authorization_cleanup() {
    let node_registry = Box::new(MockNodeRegistry::new());
    let proof_verifier = Box::new(MockProofVerifier);
    
    let mut engine = CrossNodeAuthEngine::new(node_registry, proof_verifier);
    
    // Create expired authorization
    let expired_auth = CrossNodeAuthorization {
        id: "expired_auth".to_string(),
        requester_node_id: "test_node_1".to_string(),
        resource_owner_node_id: "test_node_1".to_string(),
        resource_id: "test_resource".to_string(),
        permissions: vec![ResourcePermission::Read],
        conditions: vec![],
        created_at: Utc::now() - Duration::hours(2),
        expires_at: Utc::now() - Duration::hours(1), // Expired
        signature: "signature".to_string(),
        is_active: true,
    };
    
    engine.active_authorizations.insert("expired_auth".to_string(), expired_auth);
    assert_eq!(engine.active_authorizations.len(), 1);
    
    engine.cleanup_expired_data().await.unwrap();
    assert_eq!(engine.active_authorizations.len(), 0); // Should be cleaned up
}

#[tokio::test]
async fn test_authorization_metrics() {
    let node_registry = Box::new(MockNodeRegistry::new());
    let proof_verifier = Box::new(MockProofVerifier);
    
    let mut engine = CrossNodeAuthEngine::new(node_registry, proof_verifier);
    
    // Add some test data
    let auth = CrossNodeAuthorization {
        id: "test_auth".to_string(),
        requester_node_id: "test_node_1".to_string(),
        resource_owner_node_id: "test_node_1".to_string(),
        resource_id: "test_resource".to_string(),
        permissions: vec![ResourcePermission::Read, ResourcePermission::Write],
        conditions: vec![],
        created_at: Utc::now(),
        expires_at: Utc::now() + Duration::hours(1),
        signature: "signature".to_string(),
        is_active: true,
    };
    
    engine.active_authorizations.insert("test_auth".to_string(), auth);
    
    let metrics = engine.get_authorization_metrics();
    assert_eq!(metrics.get("total_authorizations"), Some(&1));
    assert!(metrics.contains_key("permission_read"));
    assert!(metrics.contains_key("permission_write"));
}
