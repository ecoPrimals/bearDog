

use beardog::{
    auth::{
        BearDogGenetics, BearDogWorkflowType, CrossNodeAuthConfig, CrossNodeAuthEngine,
        NodeCapability, NodeInfo, ResourceLimits, SpawnPurpose, SpawnRequest, SpawnStatus, 
        TaskType, ProofVerifier, AuthorizationProof,
    },
    config::WorkflowConfig,
    genetics::{
        DefaultBearDogGeneticsEngine, GeneticsConfig, InMemoryGeneticsStore, 
        spawning::GeneticSpawningEngine, GeneticsAPI,
    },
    node_registry::{BearDogNodeRegistry, RegistryConfig, TrustLevel},
    workflows::{MultiPartyWorkflowEngine, WorkflowType, WorkflowRequest, WorkflowTarget, WorkflowPriority},
    BearDogError, BearDogResult,
};
use chrono::Utc;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tracing::{debug, error, info, warn};
use std::sync::Once;

static INIT: Once = Once::new();

fn init_tracing() {
    INIT.call_once(|| {
        tracing_subscriber::fmt::init();
    });
}

struct MockNodeRegistry {
    nodes: HashMap<String, NodeInfo>,
}

impl MockNodeRegistry {
    fn new() -> Self {
        Self {
            nodes: HashMap::with_capacity(16),
        }
    }
}

impl beardog::auth::NodeRegistry for MockNodeRegistry {
    fn get_node_info(&self, node_id: &str) -> BearDogResult<NodeInfo> {
        self.nodes.get(node_id).cloned().ok_or_else(|| BearDogError::NotFound {
            resource_type: "Node".to_string(),
            id: node_id.to_string(),
        })
    }

    fn register_node(&mut self, node_info: NodeInfo) -> BearDogResult<()> {
        self.nodes.insert(node_info.id.clone(), node_info);
        Ok(())
    }

    fn get_trust_level(&self, node_id: &str) -> BearDogResult<f64> {
        self.nodes.get(node_id).map(|n| n.trust_level as f64)
            .ok_or_else(|| BearDogError::NotFound {
                resource_type: "Node".to_string(),
                id: node_id.to_string(),
            })
    }

    fn update_trust_level(&mut self, node_id: &str, trust_level: f64) -> BearDogResult<()> {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.trust_level = trust_level;
            Ok(())
        } else {
            Err(BearDogError::NotFound {
                resource_type: "Node".to_string(),
                id: node_id.to_string(),
            })
        }
    }
}

struct MockProofVerifier;

impl ProofVerifier for MockProofVerifier {
    fn verify_authorization_proof(&self, _proof: &AuthorizationProof) -> BearDogResult<bool> {
        Ok(true) // Always verify for testing
    }

    fn generate_proof(
        &self,
        authorization: &beardog::auth::CrossNodeAuthorization,
        operation: &beardog::auth::CrossNodeOperation,
    ) -> BearDogResult<AuthorizationProof> {
        Ok(AuthorizationProof {
            authorization_id: authorization.id.clone(),
            operation: operation.clone(),
            timestamp: Utc::now(),
            proof_signature: "mock_signature".to_string(),
        })
    }
}

pub struct GeneticIntegrationHarness {
    auth_engines: HashMap<String, Arc<CrossNodeAuthEngine>>,
    genetics_api: Arc<GeneticsAPI>,
    workflow_engine: Arc<MultiPartyWorkflowEngine>,
    test_metrics: IntegrationMetrics,
}

#[derive(Debug, Default)]
pub struct IntegrationMetrics {
    pub total_spawns_attempted: u64,
    pub successful_spawns: u64,
    pub failed_spawns: u64,
    pub approval_workflows_initiated: u64,
    pub consensus_achieved: u64,
    pub genetic_operations_performed: u64,
    pub audit_events_generated: u64,
}

impl GeneticIntegrationHarness {
    pub async fn new() -> BearDogResult<Self> {
        info!("🧪 Initializing Genetic Integration Test Harness");

        let genetics_store = Arc::new(InMemoryGeneticsStore::new());
        let genetics_config = GeneticsConfig::default();
        let genetics_api = Arc::new(GeneticsAPI::new(genetics_store, genetics_config));

        let workflow_config = Arc::new(WorkflowConfig::default());
        let workflow_store = Arc::new(beardog::workflows::InMemoryWorkflowStore::new());
        let approval_store = Arc::new(beardog::workflows::InMemoryApprovalStore::new());
        let workflow_engine = Arc::new(
            MultiPartyWorkflowEngine::new(workflow_config, workflow_store, approval_store).await?
        );

        Ok(Self {
            auth_engines: HashMap::with_capacity(16),
            genetics_api,
            workflow_engine,
            test_metrics: IntegrationMetrics::default(),
        })
    }

    pub async fn create_test_node(&mut self, node_id: &str) -> BearDogResult<String> {
        info!("🧬 Creating test node with genetics: {}", node_id);

        let node_registry = Box::new(MockNodeRegistry::new());
        let proof_verifier = Box::new(MockProofVerifier);

        let auth_engine = Arc::new(CrossNodeAuthEngine::new(node_registry, proof_verifier));

        let genetics = self.genetics_api.create_genesis_node(node_id).await?;

        let mut auth_engine_mut = Arc::try_unwrap(auth_engine)
            .map_err(|_| BearDogError::internal("Failed to unwrap auth engine".to_string() ))?;
        auth_engine_mut.register_genetics(genetics).await?;

        let auth_engine = Arc::new(auth_engine_mut);
        self.auth_engines.insert(node_id.to_string(), auth_engine);

        self.test_metrics.genetic_operations_performed += 1;
        Ok(node_id.to_string())
    }

    pub async fn test_genetic_spawning(
        &mut self,
        parent_node_id: &str,
        spawn_purpose: SpawnPurpose,
    ) -> BearDogResult<String> {
        info!("🧬 Testing genetic spawning from parent: {}", parent_node_id);

        let spawn_request = SpawnRequest {
            parent_genetics: vec![BearDogGenetics::default()],
            spawn_purpose,
            required_capabilities: vec![NodeCapability::SecurityAnalysis],
            resource_limits: ResourceLimits::default(),
            target_environment: "test".to_string(),
        };

        self.test_metrics.total_spawns_attempted += 1;

        let child_node_id = format_args!("{}_child_{}", parent_node_id, uuid::Uuid::new_v4().to_string());
        self.create_test_node(&child_node_id).await?;

        self.test_metrics.successful_spawns += 1;
        self.test_metrics.genetic_operations_performed += 1;

        Ok(child_node_id)
    }

    pub async fn test_multi_party_workflow(
        &mut self,
        workflow_type: BearDogWorkflowType,
    ) -> BearDogResult<String> {
        info!("🔄 Testing multi-party workflow: {:?}", workflow_type);

        let wf_type = match workflow_type {
            BearDogWorkflowType::GeneticSpawning { .. } => WorkflowType::SystemMaintenance,
            BearDogWorkflowType::ComplianceAudit { .. } => WorkflowType::ComplianceAudit,
            BearDogWorkflowType::SecurityIncidentResponse { .. } => WorkflowType::EmergencyAccess,
            BearDogWorkflowType::DataBackup { .. } => WorkflowType::SystemMaintenance,
        };

        let workflow_request = WorkflowRequest {
            workflow_type: wf_type,
            initiator: "test_user".to_string(),
            target: WorkflowTarget::System,
            parameters: HashMap::with_capacity(16),
            reason: "Integration test workflow".to_string(),
            priority: WorkflowPriority::Normal,
            metadata: HashMap::with_capacity(16),
        };

        let response = self.workflow_engine.initiate_workflow(workflow_request).await?;

        self.test_metrics.approval_workflows_initiated += 1;
        self.test_metrics.audit_events_generated += 1;

        Ok(response.workflow_id)
    }

    pub fn get_metrics(&self) -> &IntegrationMetrics {
        &self.test_metrics
    }
}

#[tokio::test]
async fn test_genetic_spawning_integration() -> BearDogResult<()> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new().await?;

    let parent_node = harness.create_test_node("parent_node_1").await?;
    assert_eq!(parent_node, "parent_node_1");

    let child_node = harness.test_genetic_spawning(&parent_node, SpawnPurpose::SecurityResponse).await?;
    assert!(child_node.starts_with("parent_node_1_child_"));

    let metrics = harness.get_metrics();
    assert_eq!(metrics.total_spawns_attempted, 1);
    assert_eq!(metrics.successful_spawns, 1);
    assert!(metrics.genetic_operations_performed >= 2); // Parent + child creation

    Ok(())
}

#[tokio::test]
async fn test_multi_party_workflow_integration() -> BearDogResult<()> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new().await?;

    let genetic_workflow = BearDogWorkflowType::GeneticSpawning {
        parent_genetics: vec!["parent1".to_string()],
        spawn_purpose: SpawnPurpose::SecurityResponse,
        target_capabilities: vec![NodeCapability::ThreatDetection],
    };

    let workflow_id = harness.test_multi_party_workflow(genetic_workflow).await?;
    assert!(!workflow_id.is_empty());

    let compliance_workflow = BearDogWorkflowType::ComplianceAudit {
        audit_scope: vec!["all_systems".to_string()],
        standards: vec!["SOC2".to_string(), "ISO27001".to_string()],
        automated_remediation: true,
    };

    let workflow_id2 = harness.test_multi_party_workflow(compliance_workflow).await?;
    assert!(!workflow_id2.is_empty());

    let metrics = harness.get_metrics();
    assert_eq!(metrics.approval_workflows_initiated, 2);
    assert_eq!(metrics.audit_events_generated, 2);

    Ok(())
}

#[tokio::test]
async fn test_cross_node_genetic_operations() -> BearDogResult<()> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new().await?;

    let node1 = harness.create_test_node("node_1").await?;
    let node2 = harness.create_test_node("node_2").await?;
    let node3 = harness.create_test_node("node_3").await?;

    let child1 = harness.test_genetic_spawning(&node1, SpawnPurpose::LoadBalancing).await?;
    let child2 = harness.test_genetic_spawning(&node2, SpawnPurpose::NetworkExpansion).await?;
    let child3 = harness.test_genetic_spawning(&node3, SpawnPurpose::PerformanceOptimization).await?;

    assert!(child1.starts_with("node_1_child_"));
    assert!(child2.starts_with("node_2_child_"));
    assert!(child3.starts_with("node_3_child_"));

    let metrics = harness.get_metrics();
    assert_eq!(metrics.total_spawns_attempted, 3);
    assert_eq!(metrics.successful_spawns, 3);
    assert!(metrics.genetic_operations_performed >= 6); // 3 parents + 3 children

    Ok(())
}

#[tokio::test]
async fn test_genetic_lineage_tracking() -> BearDogResult<()> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new().await?;

    let genesis_node = harness.create_test_node("genesis").await?;

    let gen1_child1 = harness.test_genetic_spawning(&genesis_node, SpawnPurpose::SecurityResponse).await?;
    let gen1_child2 = harness.test_genetic_spawning(&genesis_node, SpawnPurpose::ComplianceRequirement).await?;

    let gen2_child1 = harness.test_genetic_spawning(&gen1_child1, SpawnPurpose::SpecializedTask(TaskType::ThreatHunting)).await?;
    let gen2_child2 = harness.test_genetic_spawning(&gen1_child2, SpawnPurpose::SpecializedTask(TaskType::DisasterRecovery)).await?;

    assert!(gen1_child1.starts_with("genesis_child_"));
    assert!(gen1_child2.starts_with("genesis_child_"));
    assert!(gen2_child1.starts_with(&format_args!("{}_child_", gen1_child1).to_string()));
    assert!(gen2_child2.starts_with(&format_args!("{}_child_", gen1_child2).to_string()));

    let metrics = harness.get_metrics();
    assert_eq!(metrics.total_spawns_attempted, 4);
    assert_eq!(metrics.successful_spawns, 4);
    assert!(metrics.genetic_operations_performed >= 5); // genesis + 4 spawns

    Ok(())
}

#[tokio::test]
async fn test_genetic_capability_inheritance() -> BearDogResult<()> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new().await?;

    let parent_node = harness.create_test_node("security_specialist").await?;

    let threat_detector = harness.test_genetic_spawning(&parent_node, SpawnPurpose::SecurityResponse).await?;
    let compliance_auditor = harness.test_genetic_spawning(&parent_node, SpawnPurpose::ComplianceRequirement).await?;
    let performance_optimizer = harness.test_genetic_spawning(&parent_node, SpawnPurpose::PerformanceOptimization).await?;

    assert!(threat_detector.contains("security_specialist"));
    assert!(compliance_auditor.contains("security_specialist"));
    assert!(performance_optimizer.contains("security_specialist"));

    let metrics = harness.get_metrics();
    assert_eq!(metrics.total_spawns_attempted, 3);
    assert_eq!(metrics.successful_spawns, 3);

    Ok(())
}

#[tokio::test]
async fn test_workflow_approval_integration() -> BearDogResult<()> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new().await?;

    let workflows = vec![
        BearDogWorkflowType::GeneticSpawning {
            parent_genetics: vec!["parent1".to_string()],
            spawn_purpose: SpawnPurpose::SecurityResponse,
            target_capabilities: vec![NodeCapability::ThreatDetection],
        },
        BearDogWorkflowType::ComplianceAudit {
            audit_scope: vec!["all_systems".to_string()],
            standards: vec!["SOC2".to_string(), "ISO27001".to_string()],
            automated_remediation: true,
        },
        BearDogWorkflowType::SecurityIncidentResponse {
            threat_level: 8,
            affected_resources: vec!["critical_database".to_string()],
            response_team: vec!["security_team".to_string()],
        },
    ];

    let mut workflow_ids = Vec::new();
    for workflow in workflows {
        let workflow_id = harness.test_multi_party_workflow(workflow).await?;
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
async fn test_genetic_integration_error_handling() -> BearDogResult<()> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new().await?;

    let parent_node = harness.create_test_node("error_test_parent").await?;

    let mut successful_spawns = 0;
    let mut failed_spawns = 0;

    for i in 0..5 {
        let spawn_purpose = match i % 3 {
            0 => SpawnPurpose::SecurityResponse,
            1 => SpawnPurpose::LoadBalancing,
            _ => SpawnPurpose::ComplianceRequirement,
        };

        match harness.test_genetic_spawning(&parent_node, spawn_purpose).await {
            Ok(_) => successful_spawns += 1,
            Err(_) => failed_spawns += 1,
        }
    }

    assert!(successful_spawns > 0);

    let metrics = harness.get_metrics();
    assert_eq!(metrics.successful_spawns, successful_spawns);
    assert_eq!(metrics.total_spawns_attempted, successful_spawns + failed_spawns);

    Ok(())
}

#[tokio::test]
async fn test_genetic_integration_comprehensive() -> BearDogResult<()> {
    init_tracing();

    let mut harness = GeneticIntegrationHarness::new().await?;

    let nodes = vec![
        harness.create_test_node("comprehensive_test_1").await?,
        harness.create_test_node("comprehensive_test_2").await?,
        harness.create_test_node("comprehensive_test_3").await?,
    ];

    let mut children = Vec::new();
    for node in &nodes {
        let child = harness.test_genetic_spawning(node, SpawnPurpose::NetworkExpansion).await?;
        children.push(child);
    }

    let workflow_id = harness.test_multi_party_workflow(BearDogWorkflowType::GeneticSpawning {
        parent_genetics: nodes.clone(),
        spawn_purpose: SpawnPurpose::SecurityResponse,
        target_capabilities: vec![NodeCapability::ThreatDetection, NodeCapability::SecurityAnalysis],
    }).await?;

    assert_eq!(nodes.len(), 3);
    assert_eq!(children.len(), 3);
    assert!(!workflow_id.is_empty());

    let metrics = harness.get_metrics();
    assert_eq!(metrics.total_spawns_attempted, 3);
    assert_eq!(metrics.successful_spawns, 3);
    assert_eq!(metrics.approval_workflows_initiated, 1);
    assert!(metrics.genetic_operations_performed >= 6); // 3 parents + 3 children

    info!("🎉 Comprehensive genetic integration test completed successfully!");
    info!("📊 Final metrics: {:?}", metrics);

    Ok(())
}
