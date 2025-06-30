//! BearDog Genetic Spawning Demonstration
//!
//! Shows how BearDog nodes can "reproduce" by combining their cryptographic genetics
//! to spawn new task-specific or permanent instances.

use std::sync::Arc;
use tokio;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use beardog::{
    cross_node_auth::{
        BearDogGeneticsEngine, BearDogWorkflowType, CrossNodeAuthConfig, CrossNodeAuthEngine,
        ResourceLimits, SpawnPurpose, SpawnStatus, TaskType,
    },
    genetics_engine::{DefaultBearDogGeneticsEngine, GeneticsConfig, InMemoryGeneticsStore},
    node_registry::{InMemoryNodeRegistry, RegistryConfig},
    proof_verifier::DefaultProofVerifier,
    workflows::{MultiPartyWorkflowEngine, WorkflowConfig},
    BearDogError, BearDogResult,
};

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    info!("🧬 Starting BearDog Genetic Spawning Demonstration");

    // Create the genetic spawning ecosystem
    let ecosystem = GeneticEcosystem::new().await?;

    // Demonstrate various spawning scenarios
    demonstrate_zero_touch_spawning(&ecosystem).await?;
    demonstrate_human_approved_spawning(&ecosystem).await?;
    demonstrate_emergency_spawning(&ecosystem).await?;
    demonstrate_genetic_diversity(&ecosystem).await?;

    info!("✅ Genetic spawning demonstration complete!");
    Ok(())
}

/// BearDog genetic ecosystem for demonstrations
struct GeneticEcosystem {
    parent_node_alpha: Arc<CrossNodeAuthEngine>,
    parent_node_beta: Arc<CrossNodeAuthEngine>,
    parent_node_gamma: Arc<CrossNodeAuthEngine>,
    genetics_engine: Arc<DefaultBearDogGeneticsEngine>,
}

impl GeneticEcosystem {
    async fn new() -> BearDogResult<Self> {
        info!("🌱 Creating BearDog genetic ecosystem");

        // Create shared components
        let genetics_store = Arc::new(InMemoryGeneticsStore::new());
        let genetics_config = GeneticsConfig {
            base_mutation_rate: 0.1,
            max_genetic_diversity: 0.9,
            min_security_threshold: 0.8,
            capability_inheritance_weight: 0.85,
            trait_blending_factor: 0.7,
            enable_directed_evolution: true,
        };
        let genetics_engine = Arc::new(DefaultBearDogGeneticsEngine::new(
            genetics_store.clone(),
            genetics_config,
        ));

        // Create parent nodes with different characteristics
        let parent_alpha = Self::create_parent_node(
            "beardog-alpha-security-focused",
            genetics_engine.clone(),
            ParentProfile::SecurityFocused,
        )
        .await?;

        let parent_beta = Self::create_parent_node(
            "beardog-beta-performance-focused",
            genetics_engine.clone(),
            ParentProfile::PerformanceFocused,
        )
        .await?;

        let parent_gamma = Self::create_parent_node(
            "beardog-gamma-collaboration-focused",
            genetics_engine.clone(),
            ParentProfile::CollaborationFocused,
        )
        .await?;

        Ok(Self {
            parent_node_alpha: parent_alpha,
            parent_node_beta: parent_beta,
            parent_node_gamma: parent_gamma,
            genetics_engine,
        })
    }

    async fn create_parent_node(
        node_id: &str,
        genetics_engine: Arc<DefaultBearDogGeneticsEngine>,
        profile: ParentProfile,
    ) -> BearDogResult<Arc<CrossNodeAuthEngine>> {
        info!(
            "🧬 Creating parent node: {} with profile: {:?}",
            node_id, profile
        );

        // Create node registry and other required components
        let node_registry = Arc::new(InMemoryNodeRegistry::new(RegistryConfig::default()));

        // Create auth config based on profile
        let auth_config = CrossNodeAuthConfig {
            node_id: node_id.to_string(),
            signing_key: b"demo_signing_key_32_bytes_long!!".to_vec(),
            max_authorization_ttl: chrono::Duration::days(30),
            require_explicit_permissions: true,
            allow_permission_delegation: true,
            enable_proof_caching: true,
        };

        // Create cross-node auth engine (simplified for demo)
        // In a real implementation, this would use proper proof generators/verifiers
        let auth_engine = Arc::new(
            CrossNodeAuthEngine::new(
                auth_config,
                node_registry.clone(),
                Arc::new(DemoProofGenerator {}),
                Arc::new(DemoProofVerifier {}),
                Arc::new(DemoAuthStore {}),
                genetics_engine.clone(),
            )
            .await?,
        );

        // Customize genetics based on profile
        let genetics = genetics_engine.get_node_genetics(node_id).await?;
        // In real implementation, we'd modify genetics based on profile

        Ok(auth_engine)
    }
}

#[derive(Debug, Clone)]
enum ParentProfile {
    SecurityFocused,
    PerformanceFocused,
    CollaborationFocused,
}

/// Demonstrate zero-touch automated spawning
async fn demonstrate_zero_touch_spawning(ecosystem: &GeneticEcosystem) -> BearDogResult<()> {
    info!("🤖 === ZERO-TOUCH AUTOMATED SPAWNING ===");

    // Alpha and Beta nodes detect they need a specialized compute node
    let spawn_purpose = SpawnPurpose::TaskSpecific {
        task_type: TaskType::ComputeOffload,
        max_duration: chrono::Duration::hours(8),
        resource_limits: ResourceLimits {
            max_cpu_cores: 16,
            max_memory_gb: 32,
            max_storage_gb: 500,
            max_network_mbps: 1000,
            max_crypto_operations_per_second: 10000,
        },
    };

    let workflow_type = BearDogWorkflowType::AutomatedConsensus {
        participating_nodes: vec![
            "beardog-alpha-security-focused".to_string(),
            "beardog-beta-performance-focused".to_string(),
        ],
        consensus_threshold: 0.8,
        max_decision_time: chrono::Duration::minutes(5),
    };

    info!("🤖 Alpha node requesting automated spawn for compute offload...");

    // This would normally go through the full workflow
    let child_genetics = ecosystem
        .genetics_engine
        .recombine_genetics(
            "beardog-alpha-security-focused",
            &["beardog-beta-performance-focused".to_string()],
            &spawn_purpose,
        )
        .await?;

    info!("🧬 Generated child genetics:");
    info!("   - Genome ID: {}", child_genetics.genome_id);
    info!("   - Generation: {}", child_genetics.generation);
    info!("   - Parent nodes: {:?}", child_genetics.parent_nodes);
    info!(
        "   - Crypto chromosomes: {}",
        child_genetics.crypto_chromosomes.len()
    );
    info!(
        "   - Capability genes: {}",
        child_genetics.capability_genes.len()
    );
    info!(
        "   - Security traits: paranoia={:.2}, cooperation={:.2}",
        child_genetics.security_traits.paranoia_level,
        child_genetics.security_traits.cooperation_tendency
    );
    info!("   - Can spawn: {}", child_genetics.can_spawn);
    info!("   - Max offspring: {}", child_genetics.max_offspring);

    info!("✅ Zero-touch spawning demonstration complete\n");
    Ok(())
}

/// Demonstrate human-approved spawning for sensitive operations
async fn demonstrate_human_approved_spawning(ecosystem: &GeneticEcosystem) -> BearDogResult<()> {
    info!("👤 === HUMAN-APPROVED SPAWNING ===");

    // Request spawn for sensitive compliance audit
    let spawn_purpose = SpawnPurpose::TaskSpecific {
        task_type: TaskType::ComplianceAudit,
        max_duration: chrono::Duration::days(7),
        resource_limits: ResourceLimits {
            max_cpu_cores: 8,
            max_memory_gb: 16,
            max_storage_gb: 200,
            max_network_mbps: 100,
            max_crypto_operations_per_second: 5000,
        },
    };

    let workflow_type = BearDogWorkflowType::HumanApprovalRequired {
        approver_roles: vec![
            "compliance_officer".to_string(),
            "security_lead".to_string(),
        ],
        min_approvals: 2,
        approval_timeout: chrono::Duration::hours(24),
    };

    info!("👤 Requesting human approval for compliance audit spawn...");
    info!("   - Required approvers: compliance_officer, security_lead");
    info!("   - Minimum approvals: 2");
    info!("   - Approval timeout: 24 hours");

    // Simulate human approval process
    let child_genetics = ecosystem
        .genetics_engine
        .recombine_genetics(
            "beardog-alpha-security-focused",
            &["beardog-gamma-collaboration-focused".to_string()],
            &spawn_purpose,
        )
        .await?;

    info!("🧬 Child genetics after human approval:");
    info!(
        "   - Enhanced compliance strictness: {:.2}",
        child_genetics.security_traits.compliance_strictness
    );
    info!(
        "   - Specialized compliance genes: {}",
        child_genetics
            .capability_genes
            .iter()
            .filter(|g| matches!(
                g.capability,
                beardog::cross_node_auth::NodeCapability::ComplianceMonitoring { .. }
            ))
            .count()
    );

    info!("✅ Human-approved spawning demonstration complete\n");
    Ok(())
}

/// Demonstrate emergency spawning for incident response
async fn demonstrate_emergency_spawning(ecosystem: &GeneticEcosystem) -> BearDogResult<()> {
    info!("🚨 === EMERGENCY SPAWNING ===");

    let spawn_purpose = SpawnPurpose::EmergencyResponse {
        incident_type: "Advanced Persistent Threat Detected".to_string(),
        urgency_level: 9,
        auto_terminate_when_resolved: true,
    };

    let workflow_type = BearDogWorkflowType::HybridApproval {
        automated_checks: vec![
            beardog::cross_node_auth::AutomatedCheck::ThreatAssessment {
                max_risk_level: 0.9,
            },
            beardog::cross_node_auth::AutomatedCheck::ResourceAvailability {
                min_resources: ResourceLimits {
                    max_cpu_cores: 4,
                    max_memory_gb: 8,
                    max_storage_gb: 50,
                    max_network_mbps: 500,
                    max_crypto_operations_per_second: 2000,
                },
            },
        ],
        human_oversight: true,
        escalation_conditions: vec![
            beardog::cross_node_auth::EscalationCondition::HighRiskOperation,
            beardog::cross_node_auth::EscalationCondition::AnomalousPattern,
        ],
    };

    info!("🚨 Emergency spawn request: APT detected!");
    info!("   - Urgency level: 9/10");
    info!("   - Auto-terminate when resolved: true");
    info!("   - Hybrid approval with automated checks");

    // All three nodes contribute to emergency response
    let child_genetics = ecosystem
        .genetics_engine
        .recombine_genetics(
            "beardog-alpha-security-focused",
            &[
                "beardog-beta-performance-focused".to_string(),
                "beardog-gamma-collaboration-focused".to_string(),
            ],
            &spawn_purpose,
        )
        .await?;

    info!("🧬 Emergency response child genetics:");
    info!("   - Triple-parent inheritance from Alpha, Beta, Gamma");
    info!(
        "   - Enhanced threat sensitivity: {:.2}",
        child_genetics.security_traits.threat_sensitivity
    );
    info!(
        "   - Incident response capabilities: {}",
        child_genetics
            .capability_genes
            .iter()
            .filter(|g| matches!(
                g.capability,
                beardog::cross_node_auth::NodeCapability::IncidentResponse { .. }
            ))
            .count()
    );
    info!("   - Expected lifespan: Emergency duration only");

    info!("✅ Emergency spawning demonstration complete\n");
    Ok(())
}

/// Demonstrate genetic diversity across multiple generations
async fn demonstrate_genetic_diversity(ecosystem: &GeneticEcosystem) -> BearDogResult<()> {
    info!("🌈 === GENETIC DIVERSITY DEMONSTRATION ===");

    // Create multiple child generations to show genetic evolution
    let mut current_generation = Vec::new();

    // Generation 1: Basic task-specific children
    for task_type in &[
        TaskType::DataMigration,
        TaskType::NetworkExpansion,
        TaskType::KeyRecovery,
    ] {
        let spawn_purpose = SpawnPurpose::TaskSpecific {
            task_type: task_type.clone(),
            max_duration: chrono::Duration::hours(12),
            resource_limits: ResourceLimits {
                max_cpu_cores: 4,
                max_memory_gb: 8,
                max_storage_gb: 100,
                max_network_mbps: 200,
                max_crypto_operations_per_second: 1000,
            },
        };

        let child_genetics = ecosystem
            .genetics_engine
            .recombine_genetics(
                "beardog-alpha-security-focused",
                &["beardog-beta-performance-focused".to_string()],
                &spawn_purpose,
            )
            .await?;

        current_generation.push(child_genetics);
    }

    info!(
        "🧬 Generation 1 created: {} specialized children",
        current_generation.len()
    );

    // Show genetic diversity metrics
    let avg_paranoia = current_generation
        .iter()
        .map(|g| g.security_traits.paranoia_level)
        .sum::<f64>()
        / current_generation.len() as f64;

    let avg_cooperation = current_generation
        .iter()
        .map(|g| g.security_traits.cooperation_tendency)
        .sum::<f64>()
        / current_generation.len() as f64;

    info!("📊 Genetic diversity metrics:");
    info!("   - Average paranoia level: {:.2}", avg_paranoia);
    info!("   - Average cooperation tendency: {:.2}", avg_cooperation);
    info!("   - Unique genome IDs: {}", current_generation.len());
    info!(
        "   - Capability gene variations: {}",
        current_generation
            .iter()
            .map(|g| g.capability_genes.len())
            .sum::<usize>()
    );

    // Demonstrate mutations
    if let Some(first_child) = current_generation.first() {
        let mutated = ecosystem
            .genetics_engine
            .mutate_capabilities(first_child, 0.2)
            .await?;

        info!("🔬 Mutation demonstration:");
        info!("   - Original genome: {}", first_child.genome_id);
        info!("   - Mutated genome: {}", mutated.genome_id);
        info!(
            "   - Mutation history entries: {}",
            mutated
                .capability_genes
                .iter()
                .map(|g| g.mutation_history.len())
                .sum::<usize>()
        );
    }

    info!("✅ Genetic diversity demonstration complete\n");
    Ok(())
}

// Demo implementations for required traits
struct DemoProofGenerator;
struct DemoProofVerifier;
struct DemoAuthStore;

#[async_trait::async_trait]
impl beardog::cross_node_auth::ProofGenerator for DemoProofGenerator {
    async fn sign_authorization(
        &self,
        auth: beardog::cross_node_auth::CrossNodeAuthorization,
    ) -> BearDogResult<beardog::cross_node_auth::CrossNodeAuthorization> {
        Ok(auth) // Simplified for demo
    }

    async fn generate_operation_proof(
        &self,
        auth: beardog::cross_node_auth::CrossNodeAuthorization,
        op: &beardog::cross_node_auth::CrossNodeOperation,
    ) -> BearDogResult<beardog::cross_node_auth::AuthorizationProof> {
        Ok(beardog::cross_node_auth::AuthorizationProof {
            authorization: auth,
            operation: op.clone(),
            request_timestamp: chrono::Utc::now(),
            requester_signature: vec![0; 64], // Demo signature
        })
    }
}

#[async_trait::async_trait]
impl beardog::cross_node_auth::ProofVerifier for DemoProofVerifier {
    async fn verify_authorization_proof(
        &self,
        _proof: &beardog::cross_node_auth::AuthorizationProof,
    ) -> BearDogResult<bool> {
        Ok(true) // Always approve for demo
    }
}

#[async_trait::async_trait]
impl beardog::cross_node_auth::CrossNodeAuthStore for DemoAuthStore {
    async fn store_authorization(
        &self,
        _auth: &beardog::cross_node_auth::CrossNodeAuthorization,
    ) -> BearDogResult<()> {
        Ok(())
    }
    async fn get_authorization(
        &self,
        _id: &str,
    ) -> BearDogResult<Option<beardog::cross_node_auth::CrossNodeAuthorization>> {
        Ok(None)
    }
    async fn get_authorization_for_node(
        &self,
        _node_id: &str,
    ) -> BearDogResult<Option<beardog::cross_node_auth::CrossNodeAuthorization>> {
        Ok(None)
    }
    async fn list_active_authorizations(
        &self,
    ) -> BearDogResult<Vec<beardog::cross_node_auth::CrossNodeAuthorization>> {
        Ok(vec![])
    }
    async fn revoke_authorization(&self, _id: &str) -> BearDogResult<()> {
        Ok(())
    }
}
