

use beardog::auth::{BearDogGenetics, SpawnPurpose, TaskType};
use beardog::genetics::{
    BearDogWorkflowType, CapabilityMergingStrategy, ChromosomeRecombinationStrategy, GeneticsAPI,
    GeneticsConfig, InMemoryGeneticsStore, RecombinationParams, ResourceLimits, SpawnRequest,
    TraitBlendingStrategy,
};
use beardog::tunnel::hsm::manager::HsmManager;
use beardog::tunnel::hsm::types::{
    HsmTier, SecureEnclaveType, SmartphoneType, StrongBoxImplementation,
};
use beardog::tunnel::hsm::{SecurityLevel, SecurityRequirements};
use beardog_errors::BearDogError;
use chrono::{Duration, Utc};
use std::sync::Arc;
use tokio;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    println!("🧬 BearDog Genetic Spawning with HSM Integration Demo");
    println!("=====================================================");

    let hsm_manager = setup_hsm_manager()?;

    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics_config = GeneticsConfig {
        base_mutation_rate: 0.08,
        max_genetic_diversity: 0.85,
        min_security_threshold: 0.75,
        capability_inheritance_weight: 0.85,
        trait_blending_factor: 0.7,
        enable_directed_evolution: true,
    };

    let genetics_api =
        GeneticsAPI::with_hsm_manager(genetics_store.clone(), hsm_manager.clone(), genetics_config);

    println!("\n📱 Demo 1: Genesis Node Creation with HSM");
    println!("==========================================");

    let genesis_node_id = "beardog-genesis-001";
    let genesis_genetics = genetics_api.create_genesis_node(genesis_node_id).await?;
    println!("✅ Created genesis node: {}", genesis_node_id);
    println!("   Generation: {}", genesis_genetics.generation);
    println!("   Capabilities: {}", genesis_genetics.capabilities.len());
    println!(
        "   Chromosomes: {}",
        genesis_genetics.crypto_chromosomes.len()
    );
    println!("   Fitness Score: {:.3}", genesis_genetics.fitness_score);

    println!("\n🔒 Demo 2: Security Response Spawning");
    println!("=====================================");

    let security_spawn_request = SpawnRequest {
        request_id: Uuid::new_v4().to_string(),
        requesting_parent: genesis_node_id.to_string(),
        co_parents: vec![],
        purpose: SpawnPurpose::SecurityResponse,
        resource_requirements: ResourceLimits {
            max_cpu_percent: 90.0,
            max_memory_mb: 4096,
            max_storage_gb: 20,
            max_network_mbps: 1000,
            allowed_jurisdictions: vec!["US".to_string(), "EU".to_string()],
            temporal_windows: vec![],
        },
        workflow_type: BearDogWorkflowType::AutomatedConsensus {
            participating_nodes: vec![genesis_node_id.to_string()],
            consensus_threshold: 0.6,
            max_decision_time: Duration::minutes(10),
        },
        created_at: Utc::now(),
        expires_at: Utc::now() + Duration::hours(24),
        metadata: [
            ("spawn_reason".to_string(), "security_incident".to_string()),
            ("priority".to_string(), "high".to_string()),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let security_result = genetics_api.spawn_node(security_spawn_request).await?;
    println!(
        "✅ Security spawn result: {}",
        if security_result.approved {
            "APPROVED"
        } else {
            "REJECTED"
        }
    );
    println!("   Decision reason: {}", security_result.decision_reason);
    println!(
        "   Processing time: {} participants",
        security_result.decision_participants.len()
    );

    if let Some(ref child_genetics) = security_result.child_genetics {
        println!("   Child generation: {}", child_genetics.generation);
        println!(
            "   Enhanced capabilities: {}",
            child_genetics.capabilities.len()
        );
        println!("   Security level: {:?}", child_genetics.security_clearance);
        println!(
            "   Paranoia level: {}",
            child_genetics.security_traits.paranoia_level
        );
    }

    println!("\n⚡ Demo 3: Specialized Task Spawning");
    println!("===================================");

    let task_spawn_request = SpawnRequest {
        request_id: Uuid::new_v4().to_string(),
        requesting_parent: genesis_node_id.to_string(),
        co_parents: vec![],
        purpose: SpawnPurpose::SpecializedTask(TaskType::ComputeTask),
        resource_requirements: ResourceLimits {
            max_cpu_percent: 95.0,
            max_memory_mb: 8192,
            max_storage_gb: 100,
            max_network_mbps: 10000,
            allowed_jurisdictions: vec!["US".to_string()],
            temporal_windows: vec![],
        },
        workflow_type: BearDogWorkflowType::HybridApproval {
            automated_checks: vec![
                beardog::genetics::AutomatedCheck::TrustScore { min_score: 0.7 },
                beardog::genetics::AutomatedCheck::ThreatAssessment {
                    max_risk_level: 0.3,
                },
            ],
            human_oversight: false,
            escalation_conditions: vec![
                beardog::genetics::EscalationCondition::HighResourceUsage { threshold: 0.8 },
            ],
        },
        created_at: Utc::now(),
        expires_at: Utc::now() + Duration::hours(12),
        metadata: [
            (
                "task_type".to_string(),
                "high_performance_compute".to_string(),
            ),
            ("optimization_target".to_string(), "throughput".to_string()),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let task_result = genetics_api.spawn_node(task_spawn_request).await?;
    println!(
        "✅ Task-specialized spawn result: {}",
        if task_result.approved {
            "APPROVED"
        } else {
            "REJECTED"
        }
    );
    println!("   Decision reason: {}", task_result.decision_reason);

    if let Some(ref child_genetics) = task_result.child_genetics {
        println!("   Specialized for: Compute Tasks");
        println!(
            "   Performance capabilities: {}",
            child_genetics
                .capabilities
                .iter()
                .filter(|c| matches!(
                    c,
                    beardog::auth::NodeCapability::ComputeProvider
                        | beardog::auth::NodeCapability::HighThroughput
                        | beardog::auth::NodeCapability::LowLatency
                ))
                .count()
        );
    }

    println!("\n👥 Demo 4: Multi-Parent Genetic Recombination");
    println!("===============================================");

    let parent_a_id = "beardog-parent-a";
    let parent_a_genetics = genetics_api.create_genesis_node(parent_a_id).await?;

    let parent_b_id = "beardog-parent-b";
    let parent_b_genetics = genetics_api.create_genesis_node(parent_b_id).await?;

    let multi_parent_request = SpawnRequest {
        request_id: Uuid::new_v4().to_string(),
        requesting_parent: parent_a_id.to_string(),
        co_parents: vec![parent_b_id.to_string()],
        purpose: SpawnPurpose::EcosystemIntegration("ToadStool".to_string()),
        resource_requirements: ResourceLimits {
            max_cpu_percent: 70.0,
            max_memory_mb: 2048,
            max_storage_gb: 50,
            max_network_mbps: 500,
            allowed_jurisdictions: vec!["US".to_string(), "EU".to_string()],
            temporal_windows: vec![],
        },
        workflow_type: BearDogWorkflowType::AutomatedConsensus {
            participating_nodes: vec![parent_a_id.to_string(), parent_b_id.to_string()],
            consensus_threshold: 0.75,
            max_decision_time: Duration::minutes(15),
        },
        created_at: Utc::now(),
        expires_at: Utc::now() + Duration::hours(48),
        metadata: [
            (
                "recombination_strategy".to_string(),
                "crossover".to_string(),
            ),
            ("ecosystem_target".to_string(), "ToadStool".to_string()),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let multi_parent_result = genetics_api.spawn_node(multi_parent_request).await?;
    println!(
        "✅ Multi-parent spawn result: {}",
        if multi_parent_result.approved {
            "APPROVED"
        } else {
            "REJECTED"
        }
    );
    println!("   Parent A: {}", parent_a_id);
    println!("   Parent B: {}", parent_b_id);
    println!(
        "   Decision reason: {}",
        multi_parent_result.decision_reason
    );

    if let Some(ref child_genetics) = multi_parent_result.child_genetics {
        println!(
            "   Combined genetics generation: {}",
            child_genetics.generation
        );
        println!(
            "   Inherited capabilities: {}",
            child_genetics.capabilities.len()
        );
        println!(
            "   Ecosystem integration: {}",
            child_genetics
                .capabilities
                .iter()
                .any(|c| matches!(c, beardog::auth::NodeCapability::ToadStoolCompute))
        );
    }

    println!("\n🔐 Demo 5: HSM-Backed Cryptographic Operations");
    println!("==============================================");

    println!("Testing HSM operations for genetic spawning:");

    let security_reqs = SecurityRequirements::new(SecurityLevel::Medium);

    let random_bytes = hsm_manager
        .generate_random_bytes(32, &security_reqs)
        .await?;
    println!(
        "✅ Generated {} random bytes for genetic mutations",
        random_bytes.len()
    );

    let test_data = b"genetic-lineage-proof-test-data";
    let lineage_signature = hsm_manager
        .sign_data(
            "test-genetic-lineage",
            test_data,
            &security_reqs,
            &beardog::tunnel::hsm::types::HsmOperation::LineageProof,
        )
        .await?;
    println!(
        "✅ Generated lineage proof signature: {} bytes",
        lineage_signature.len()
    );

    println!("\n📊 Demo 6: Genetic Diversity Analysis");
    println!("=====================================");

    let diversity_metrics =
        analyze_genetic_diversity(&genetics_api, &[genesis_node_id, parent_a_id, parent_b_id])
            .await?;

    println!("Genetic diversity analysis:");
    println!(
        "  Average fitness: {:.3}",
        diversity_metrics.average_fitness
    );
    println!(
        "  Generation spread: {}-{}",
        diversity_metrics.min_generation, diversity_metrics.max_generation
    );
    println!(
        "  Capability diversity: {:.3}",
        diversity_metrics.capability_diversity
    );
    println!(
        "  Security trait variance: {:.3}",
        diversity_metrics.security_variance
    );

    println!("\n🏥 Demo 7: HSM Health Status");
    println!("============================");

    let hsm_health = hsm_manager.get_health_status().await?;
    println!("HSM Health Status:");
    if let Some((provider_id, status)) = hsm_health.iter().next() {
        println!("  Provider: {}", provider_id);
        println!(
            "  Overall health: {}",
            if status.healthy {
                "HEALTHY"
            } else {
                "UNHEALTHY"
            }
        );
        println!(
            "  Operations/sec: {:.1}",
            status.performance_metrics.operations_per_second
        );
        println!(
            "  Average latency: {:.2}ms",
            status.performance_metrics.average_latency_ms
        );
        println!(
            "  Error rate: {:.3}%",
            status.performance_metrics.error_rate * 100.0
        );
        println!(
            "  Availability: {:.1}%",
            status.performance_metrics.availability_percentage
        );
    } else {
        println!("  No HSM providers available");
    }

    println!("\n🎉 Genetic Spawning Demo Complete!");
    println!("===================================");
    println!("Successfully demonstrated:");
    println!("  ✅ HSM-backed genetic operations");
    println!("  ✅ Multi-parent genetic recombination");
    println!("  ✅ Cryptographic lineage proofs");
    println!("  ✅ Secure mutation operations");
    println!("  ✅ Directed evolution for specialized tasks");
    println!("  ✅ Automated consensus workflows");
    println!("  ✅ HSM health monitoring");

    Ok(())
}

fn setup_hsm_manager() -> Result<Arc<HsmManager, BearDogError>> {
    let hsm_manager = HsmManager::new();

    let primary_tier = HsmTier::SmartphoneHsm {
        device_type: SmartphoneType::Android {
            manufacturer: "Google".to_string(),
            model: "Pixel 8a".to_string(),
            android_version: "14".to_string(),
            strongbox_version: Some("2.0".to_string()),
        },
        secure_enclave: SecureEnclaveType::AndroidStrongBox {
            implementation: StrongBoxImplementation::TitanM {
                version: "3.0".to_string(),
                security_level: "StrongBox".to_string(),
            },
            hardware_backed: true,
            key_attestation: true,
        },
        attestation_level: beardog::tunnel::hsm::types::AttestationLevel::CertifiedHardware,
        user_presence_required: true,
    };

    let fallback_tier = HsmTier::SoftwareHsm {
        implementation: beardog::tunnel::hsm::types::SoftwareHsmType::RustSoftwareHsm,
        key_storage: beardog::tunnel::hsm::types::KeyStorageType::EncryptedFile,
        encryption_at_rest: true,
        memory_protection: beardog::tunnel::hsm::types::MemoryProtectionLevel::High,
    };

    Ok(Arc::new(hsm_manager))
}

struct GeneticDiversityMetrics {
    average_fitness: f64,
    min_generation: u32,
    max_generation: u32,
    capability_diversity: f64,
    security_variance: f64,
}

async fn analyze_genetic_diversity(
    genetics_api: &GeneticsAPI,
    node_ids: &[&str],
) -> Result<GeneticDiversityMetrics, BearDogError> {
    let mut genetics_samples = Vec::new();

    for node_id in node_ids {
        let genetics = genetics_api.get_node_genetics(node_id).await?;
        genetics_samples.push(genetics);
    }

    let average_fitness = genetics_samples
        .iter()
        .map(|g| g.fitness_score)
        .sum::<f64>()
        / genetics_samples.len() as f64;

    let min_generation = genetics_samples
        .iter()
        .map(|g| g.generation)
        .min()
        .unwrap_or(0);

    let max_generation = genetics_samples
        .iter()
        .map(|g| g.generation)
        .max()
        .unwrap_or(0);

    let mut all_capabilities = std::collections::HashSet::new();
    let mut total_capabilities = 0;

    for genetics in &genetics_samples {
        total_capabilities += genetics.capabilities.len();
        for capability in &genetics.capabilities {
            all_capabilities.insert(capability.clone());
        }
    }

    let capability_diversity = if total_capabilities > 0 {
        all_capabilities.len() as f64 / total_capabilities as f64
    } else {
        0.0
    };

    let trust_levels: Vec<f64> = genetics_samples
        .iter()
        .map(|g| g.security_traits.trust_threshold)
        .collect();

    let mean_trust = trust_levels.iter().sum::<f64>() / trust_levels.len() as f64;
    let security_variance = trust_levels
        .iter()
        .map(|t| (t - mean_trust).powi(2))
        .sum::<f64>()
        / trust_levels.len() as f64;

    Ok(GeneticDiversityMetrics {
        average_fitness,
        min_generation,
        max_generation,
        capability_diversity,
        security_variance,
    })
}
