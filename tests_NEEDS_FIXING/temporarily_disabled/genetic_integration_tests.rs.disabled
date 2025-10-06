use beardog_errors::BearDogError;

use beardog::{
    auth::{
        AuthorizationProof, BearDogGenetics, BearDogWorkflowType, CrossNodeAuthConfig,
        CrossNodeAuthEngine, NodeCapability, NodeInfo, ProofVerifier, ResourceLimits, SpawnPurpose,
        SpawnRequest, SpawnStatus, TaskType,
    },
    config::WorkflowConfig,
    genetics::{
        spawning::GeneticSpawningEngine, DefaultBearDogGeneticsEngine, GeneticsAPI, GeneticsConfig,
        InMemoryGeneticsStore,
    },
    node_registry::{BearDogNodeRegistry, RegistryConfig, TrustLevel},
    workflows::{
        MultiPartyWorkflowEngine, WorkflowPriority, WorkflowRequest, WorkflowTarget, WorkflowType,
    },
    BearDogError, BearDogResult,
};
use chrono::Utc;
use std::sync::Once;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tracing::{debug, error, info, warn};

static INIT: Once = Once::new();

fn init_tracing() {
    INIT.call_once(|| {
        tracing_subscriber::fmt::init();
    });
}

impl MockNodeRegistry {
    fn new() -> Self {
        Self {
            nodes: HashMap::with_capacity(16),
        }
    }
}

impl beardog::auth::NodeRegistry for MockNodeRegistry {
    fn get_node_info(&self, node_id: &str) -> Result<NodeInfo, BearDogError> {
        self.nodes
            .get(node_id)
            .cloned()
            .ok_or_else(|| BearDogError::NotFound {
                resource_type: "Node".to_string(),
                id: node_id.to_string(),
            })
    }

    fn register_node(&mut self, node_info: NodeInfo) -> Result<(), BearDogError> {
        self.nodes.insert(node_info.id.clone(), node_info);
        Ok(())
    }

    fn get_trust_level(&self, node_id: &str) -> Result<f64, BearDogError> {
        self.nodes
            .get(node_id)
            .map(|n| n.trust_level as f64)
            .ok_or_else(|| BearDogError::NotFound {
                resource_type:  "Node".to_string(),
                id: node_id.to_string(),
            })
    }

    fn set_trust_level(&mut self, node_id: &str, trust_level: f64) -> Result<(), BearDogError> {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.trust_level = trust_level;
            Ok(())
        } else {
            Err(BearDogError::NotFound {
                resource_type:  "Node".to_string(),
                id: node_id.to_string(),
            })
        }
    }

    fn validate_authorization(
        &self,
        _authorization: &beardog::auth::CrossNodeAuthorization,
        _operation: &beardog::auth::CrossNodeOperation,
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }

    fn create_authorization_proof(
        &self,
        authorization: &beardog::auth::CrossNodeAuthorization,
        operation: &beardog::auth::CrossNodeOperation,
    ) -> Result<AuthorizationProof, BearDogError> {
        Ok(AuthorizationProof {
            authorization_id: authorization.id.clone(),
            operation: operation.clone(),
            timestamp: Utc::now(),
            proof_signature:  "mock_signature".to_string(),
        })
    }

    pub fn create_test_node(&mut self, node_id: &str) -> Result<String, BearDogError> {
        info!("🧬 Creating test node with genetics: {}", node_id);

        let node_registry = Box::new(MockNodeRegistry::new());
        let proof_verifier = Box::new(MockProofVerifier);

        let auth_engine = Arc::new(CrossNodeAuthEngine::new(node_registry, proof_verifier));

        let genetics = self.genetics_api.create_genesis_node(node_id)?;

        let mut auth_engine_mut = Arc::try_unwrap(auth_engine)
            .map_err(|_| BearDogError::internal("Failed to unwrap auth engine".to_string()))?;
        auth_engine_mut.register_genetics(genetics)?;

        let auth_engine = Arc::new(auth_engine_mut);
        Ok(node_id.to_string())
    }

    pub fn spawn_child_node(
        &mut self,
        parent_node_id: &str,
        spawn_purpose: SpawnPurpose,
    ) -> Result<String, BearDogError> {
        info!(
            "🧬 Testing genetic spawning from parent: {}",
            parent_node_id
        );

        let spawn_request = SpawnRequest {
            parent_genetics: vec![BearDogGenetics::default()],
            resource_limits: ResourceLimits::default(),
            target_environment: "test".to_string(),
        };

        self.test_metrics.total_spawns_attempted += 1;

        let child_node_id = format!(
            "{}_child_{}",
            parent_node_id,
            uuid::Uuid::new_v4()
    ) -> Result<String, BearDogError> {
        info!("🔄 Testing multi-party workflow: {:?}", workflow_type);

        let wf_type = match workflow_type {
            BearDogWorkflowType::GeneticSpawning { .. } => WorkflowType::SystemMaintenance,
            BearDogWorkflowType::ComplianceAudit { .. } => WorkflowType::ComplianceAudit,
            BearDogWorkflowType::SecurityIncidentResponse { .. } => WorkflowType::EmergencyAccess,
            BearDogWorkflowType::DataBackup { .. } => WorkflowType::SystemMaintenance,
        };

        let workflow_request = WorkflowRequest {
            workflow_type: wf_type,
            initiator:  "test_user".to_string(),
            parameters: HashMap::with_capacity(16),
            reason:  "Integration test workflow".to_string(),
            metadata: HashMap::with_capacity(16),
        };

        let response = self
            .workflow_engine
            .initiate_workflow(workflow_request)
            ?;

        self.test_metrics.approval_workflows_initiated += 1;
        self.test_metrics.audit_events_generated += 1;

        Ok(response.workflow_id)
    }

    pub fn get_metrics(&self) -> &IntegrationMetrics {
        &self.test_metrics
    }
}

#[tokio::test]
async fn test_genetic_spawning_integration() -> Result<(), BearDogError> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new()?;

    let parent_node = harness.create_test_node( "parent_node_1")?;
    assert_eq!(parent_node,  "parent_node_1");

    let child_node = harness
        .test_genetic_spawning(&parent_node, SpawnPurpose::SecurityResponse)
        ?;
    assert!(child_node.starts_with( "parent_node_1_child_"));

    let metrics = harness.get_metrics();
    assert_eq!(metrics.total_spawns_attempted, 1);
    assert_eq!(metrics.successful_spawns, 1);
    assert!(metrics.genetic_operations_performed >= 2); // Parent + child creation

    Ok(())
}

#[tokio::test]
async fn test_multi_party_workflow_integration() -> Result<(), BearDogError> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new()?;

    let genetic_workflow = BearDogWorkflowType::GeneticSpawning {
        parent_genetics: vec![ "parent1".to_string()],
    };

    let workflow_id = harness.test_multi_party_workflow(genetic_workflow)?;
    assert!(!workflow_id.is_empty());

    let compliance_workflow = BearDogWorkflowType::ComplianceAudit {
        audit_scope: vec![ "all_systems".to_string()],
        standards: vec![ "SOC2".to_string()],
    };

    let workflow_id2 = harness
        .test_multi_party_workflow(compliance_workflow)
        ?;
    assert!(!workflow_id2.is_empty());

    let metrics = harness.get_metrics();
    assert_eq!(metrics.approval_workflows_initiated, 2);
    assert_eq!(metrics.audit_events_generated, 2);

    Ok(())
}

#[tokio::test]
async fn test_cross_node_genetic_operations() -> Result<(), BearDogError> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new()?;

    let node1 = harness.create_test_node( "node_1")?;
    let node2 = harness.create_test_node( "node_2")?;
    let node3 = harness.create_test_node( "node_3")?;

    let child1 = harness
        .test_genetic_spawning(&node1, SpawnPurpose::LoadBalancing)
        ?;
    let child2 = harness
        .test_genetic_spawning(&node2, SpawnPurpose::NetworkExpansion)
        ?;
    let child3 = harness
        .test_genetic_spawning(&node3, SpawnPurpose::PerformanceOptimization)
        ?;

    assert!(child1.starts_with( "node_1_child_"));
    assert!(child2.starts_with( "node_2_child_"));
    assert!(child3.starts_with( "node_3_child_"));

    let metrics = harness.get_metrics();
    assert_eq!(metrics.total_spawns_attempted, 3);
    assert_eq!(metrics.successful_spawns, 3);
    assert!(metrics.genetic_operations_performed >= 6); // 3 parents + 3 children

    Ok(())
}

#[tokio::test]
async fn test_genetic_lineage_tracking() -> Result<(), BearDogError> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new()?;

    let genesis_node = harness.create_test_node( "genesis")?;

    let gen1_child1 = harness
        .test_genetic_spawning(&genesis_node, SpawnPurpose::SecurityResponse)
        ?;
    let gen1_child2 = harness
        .test_genetic_spawning(&genesis_node, SpawnPurpose::ComplianceRequirement)
        ?;

    let gen2_child1 = harness
        .test_genetic_spawning(
            &gen1_child1,
            SpawnPurpose::SpecializedTask(TaskType::ThreatHunting),
        )
        ?;
    let gen2_child2 = harness
        .test_genetic_spawning(
            &gen1_child2,
            SpawnPurpose::SpecializedTask(TaskType::DisasterRecovery),
        )
        ?;

    assert!(gen1_child1.starts_with( "genesis_child_"));
    assert!(gen1_child2.starts_with( "genesis_child_"));
    assert!(gen2_child1.starts_with(&format!("{}_child_", gen1_child1)));
    assert!(gen2_child2.starts_with(&format!("{}_child_", gen1_child2)));

    let metrics = harness.get_metrics();
    assert_eq!(metrics.total_spawns_attempted, 4);
    assert_eq!(metrics.successful_spawns, 4);
    assert!(metrics.genetic_operations_performed >= 5); // genesis + 4 spawns

    Ok(())
}

#[tokio::test]
async fn test_genetic_capability_inheritance() -> Result<(), BearDogError> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new()?;

    let parent_node = harness.create_test_node( "security_specialist")?;

    let threat_detector = harness
        .test_genetic_spawning(&parent_node, SpawnPurpose::SecurityResponse)
        ?;
    let compliance_auditor = harness
        .test_genetic_spawning(&parent_node, SpawnPurpose::ComplianceRequirement)
        ?;
    let performance_optimizer = harness
        .test_genetic_spawning(&parent_node, SpawnPurpose::PerformanceOptimization)
        ?;

    assert!(threat_detector.contains( "security_specialist"));
    assert!(compliance_auditor.contains( "security_specialist"));
    assert!(performance_optimizer.contains( "security_specialist"));

    let metrics = harness.get_metrics();
    assert_eq!(metrics.total_spawns_attempted, 3);
    assert_eq!(metrics.successful_spawns, 3);

    Ok(())
}

#[tokio::test]
async fn test_workflow_approval_integration() -> Result<(), BearDogError> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new()?;

    let workflows = vec![
        BearDogWorkflowType::GeneticSpawning {
            parent_genetics: vec![ "parent1".to_string()],
            response_team: vec![ "security_team".to_string()],
        },
    ];

    let mut workflow_ids = Vec::new();
    for workflow in workflows {
        let workflow_id = harness.test_multi_party_workflow(workflow)?;
        workflow_ids.push(workflow_id);
    }

    assert_eq!(workflow_ids.len(), 3);
    for id in &workflow_ids {
        assert!(!id.is_empty());
    }

    let metrics = harness.get_metrics();
    assert_eq!(metrics.approval_workflows_initiated, 3);
    assert_eq!(metrics.audit_events_generated, 3);

    Ok(())
}

#[tokio::test]
async fn test_genetic_integration_error_handling() -> Result<(), BearDogError> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new()?;

    let parent_node = harness.create_test_node( "error_test_parent")?;

    let mut successful_spawns = 0;
    let mut failed_spawns = 0;

    for i in 0..5 {
        let spawn_purpose = match i % 3 {
            0 => SpawnPurpose::SecurityResponse,
            1 => SpawnPurpose::LoadBalancing,
            _ => SpawnPurpose::ComplianceRequirement,
        };

        match harness
            .test_genetic_spawning(&parent_node, spawn_purpose)
        {
            Ok(_) => successful_spawns += 1,
            Err(_) => failed_spawns += 1,
        }
    }

    assert!(successful_spawns > 0);

    let metrics = harness.get_metrics();
    assert_eq!(metrics.successful_spawns, successful_spawns);
    assert_eq!(
        metrics.total_spawns_attempted,
        successful_spawns + failed_spawns
    );

    Ok(())
}

#[tokio::test]
async fn test_genetic_integration_comprehensive() -> Result<(), BearDogError> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new()?;

    let nodes = vec![
        harness.create_test_node( "comprehensive_test_1")?,
        harness.create_test_node( "comprehensive_test_2")?,
        harness.create_test_node( "comprehensive_test_3")?,
    ];

    let mut children = Vec::new();
    for node in &nodes {
        let child = harness
            .test_genetic_spawning(node, SpawnPurpose::NetworkExpansion)
            ?;
        children.push(child);
    }

    let workflow_id = harness
        .test_multi_party_workflow(BearDogWorkflowType::GeneticSpawning {
            parent_genetics: nodes.clone(),
            target_capabilities: vec![
                NodeCapability::ThreatDetection,
                NodeCapability::SecurityAnalysis,
            ],
        })
        ?;

    assert_eq!(nodes.len(), metrics.total_nodes);

    Ok(())
}
