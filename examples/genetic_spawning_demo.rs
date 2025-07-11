//! # BearDog Genetic Spawning System Demo
//!
//! This example demonstrates the genetic spawning capabilities of BearDog, showcasing:
//! - Genesis node creation
//! - Multi-party spawning workflows
//! - Genetic recombination algorithms
//! - Resource constraint enforcement
//! - Cryptographic lineage verification
//!
//! Run with: `cargo run --example genetic_spawning_demo`

use beardog::{
    genetics::{
        GeneticsAPI, InMemoryGeneticsStore, GeneticsConfig, SpawnRequest, ResourceLimits,
        BearDogWorkflowType, quick_spawn_automated_consensus, quick_spawn_human_approval,
    },
    auth::SpawnPurpose,
    BearDogResult,
};
use std::sync::Arc;
use tracing::{info, error};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🧬 Starting BearDog Genetic Spawning System Demo");

    // Initialize the genetics system
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics_config = GeneticsConfig {
        base_mutation_rate: 0.05,
        max_genetic_diversity: 0.8,
        min_security_threshold: 0.7,
        capability_inheritance_weight: 0.8,
        trait_blending_factor: 0.6,
        enable_directed_evolution: true,
    };
    let genetics_api = GeneticsAPI::new(genetics_store, genetics_config);

    // Demo 1: Create genesis nodes (founding parents)
    info!("\n📚 Demo 1: Creating Genesis Nodes");
    let genesis_nodes = create_genesis_nodes(&genetics_api).await?;

    // Demo 2: Automated consensus spawning
    info!("\n🤖 Demo 2: Automated Consensus Spawning");
    let consensus_child = demonstrate_automated_consensus(&genetics_api, &genesis_nodes).await?;

    // Demo 3: Human approval spawning
    info!("\n👨‍💼 Demo 3: Human Approval Spawning");
    let approval_child = demonstrate_human_approval(&genetics_api, &genesis_nodes).await?;

    // Demo 4: Resource-constrained spawning
    info!("\n⚡ Demo 4: Resource-Constrained Spawning");
    demonstrate_resource_constraints(&genetics_api, &genesis_nodes).await?;

    // Demo 5: Show genetic lineage
    info!("\n🌳 Demo 5: Genetic Lineage Analysis");
    analyze_genetic_lineage(&genetics_api, &[consensus_child, approval_child]).await?;

    info!("\n✅ BearDog Genetic Spawning Demo completed successfully!");
    info!("🎯 Key capabilities demonstrated:");
    info!("   • Genesis node creation with secure genetics");
    info!("   • Multi-party workflow processing");
    info!("   • Genetic recombination algorithms");
    info!("   • Resource constraint enforcement");
    info!("   • Lineage tracking and verification");

    Ok(())
}

/// Create several genesis nodes to serve as founding parents
async fn create_genesis_nodes(genetics_api: &GeneticsAPI) -> BearDogResult<Vec<String>> {
    let mut genesis_nodes = Vec::new();

    for i in 1..=3 {
        let node_id = format!("genesis-node-{}", i);
        info!("Creating genesis node: {}", node_id);

        let genetics = genetics_api.create_genesis_node(&node_id).await?;
        
        info!("✅ Created {}: generation {}, {} chromosomes, {} capabilities",
              node_id, genetics.generation, genetics.crypto_chromosomes.len(), genetics.capability_genes.len());
        
        genesis_nodes.push(node_id);
    }

    Ok(genesis_nodes)
}

/// Demonstrate automated consensus spawning workflow
async fn demonstrate_automated_consensus(
    genetics_api: &GeneticsAPI,
    genesis_nodes: &[String],
) -> BearDogResult<String> {
    info!("Initiating automated consensus spawn...");

    let participating_nodes = vec![
        "consensus-node-1".to_string(),
        "consensus-node-2".to_string(),
        "consensus-node-3".to_string(),
    ];

    let result = quick_spawn_automated_consensus(
        genetics_api,
        &genesis_nodes[0], // First genesis node as parent
        vec![genesis_nodes[1].clone()], // Second genesis node as co-parent
        SpawnPurpose::EmergencyResponse,
        participating_nodes,
    ).await?;

    if result.approved {
        let child_id = result.child_node_id.as_ref().unwrap();
        info!("✅ Automated consensus successful!");
        info!("   Child node: {}", child_id);
        info!("   Decision: {}", result.decision_reason);
        info!("   Participants: {:?}", result.decision_participants);
        Ok(child_id.clone())
    } else {
        error!("❌ Automated consensus failed: {}", result.decision_reason);
        Err(beardog::BearDogError::InvalidInput { 
            message: "Consensus spawn failed".to_string() 
        })
    }
}

/// Demonstrate human approval spawning workflow
async fn demonstrate_human_approval(
    genetics_api: &GeneticsAPI,
    genesis_nodes: &[String],
) -> BearDogResult<String> {
    info!("Initiating human approval spawn...");

    let approver_roles = vec![
        "security_officer".to_string(),
        "compliance_lead".to_string(),
    ];

    let result = quick_spawn_human_approval(
        genetics_api,
        &genesis_nodes[1], // Second genesis node as parent
        vec![genesis_nodes[2].clone()], // Third genesis node as co-parent
        SpawnPurpose::ComplianceAudit,
        approver_roles,
        1, // Minimum 1 approval
    ).await?;

    if result.approved {
        let child_id = result.child_node_id.as_ref().unwrap();
        info!("✅ Human approval successful!");
        info!("   Child node: {}", child_id);
        info!("   Decision: {}", result.decision_reason);
        Ok(child_id.clone())
    } else {
        error!("❌ Human approval failed: {}", result.decision_reason);
        Err(beardog::BearDogError::InvalidInput { 
            message: "Human approval spawn failed".to_string() 
        })
    }
}

/// Demonstrate resource-constrained spawning
async fn demonstrate_resource_constraints(
    genetics_api: &GeneticsAPI,
    genesis_nodes: &[String],
) -> BearDogResult<()> {
    info!("Testing resource constraint enforcement...");

    // Create a spawn request with high resource requirements
    let high_resource_request = SpawnRequest {
        request_id: Uuid::new_v4().to_string(),
        requesting_parent: genesis_nodes[0].clone(),
        co_parents: vec![],
        purpose: SpawnPurpose::ComputeOffload,
        resource_requirements: ResourceLimits {
            max_cpu_percent: 95.0, // Very high CPU requirement
            max_memory_mb: 16384,  // 16GB RAM
            max_storage_gb: 1000,  // 1TB storage
            max_network_mbps: 10000, // 10Gbps network
            allowed_jurisdictions: vec!["US".to_string(), "EU".to_string(), "APAC".to_string()],
            temporal_windows: vec![],
        },
        workflow_type: BearDogWorkflowType::AutomatedConsensus {
            participating_nodes: vec!["resource-checker-1".to_string()],
            consensus_threshold: 1.0,
            max_decision_time: chrono::Duration::minutes(1),
        },
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
        metadata: {
            let mut meta = std::collections::HashMap::new();
            meta.insert("resource_intensive".to_string(), "true".to_string());
            meta
        },
    };

    let result = genetics_api.spawn_node(high_resource_request).await?;
    
    info!("High-resource spawn result: approved={}", result.approved);
    info!("Decision reason: {}", result.decision_reason);

    // Create a spawn request with reasonable resource requirements
    let reasonable_request = SpawnRequest {
        request_id: Uuid::new_v4().to_string(),
        requesting_parent: genesis_nodes[0].clone(),
        co_parents: vec![],
        purpose: SpawnPurpose::DataMigration,
        resource_requirements: ResourceLimits {
            max_cpu_percent: 25.0,
            max_memory_mb: 1024,
            max_storage_gb: 50,
            max_network_mbps: 100,
            allowed_jurisdictions: vec!["US".to_string()],
            temporal_windows: vec![],
        },
        workflow_type: BearDogWorkflowType::AutomatedConsensus {
            participating_nodes: vec!["resource-checker-1".to_string()],
            consensus_threshold: 1.0,
            max_decision_time: chrono::Duration::minutes(1),
        },
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
        metadata: std::collections::HashMap::new(),
    };

    let result = genetics_api.spawn_node(reasonable_request).await?;
    
    info!("Reasonable-resource spawn result: approved={}", result.approved);
    info!("Decision reason: {}", result.decision_reason);

    Ok(())
}

/// Analyze genetic lineage of spawned nodes
async fn analyze_genetic_lineage(
    genetics_api: &GeneticsAPI,
    child_nodes: &[String],
) -> BearDogResult<()> {
    info!("Analyzing genetic lineage...");

    for child_node in child_nodes {
        info!("\n🔍 Analyzing lineage for node: {}", child_node);
        
        match genetics_api.get_node_genetics(child_node).await {
            Ok(genetics) => {
                info!("   Genetics ID: {}", genetics.genome_id);
                info!("   Generation: {}", genetics.generation);
                info!("   Lineage depth: {}", genetics.lineage_depth);
                info!("   Parent genomes: {:?}", genetics.parent_genomes);
                info!("   Crypto chromosomes: {}", genetics.crypto_chromosomes.len());
                info!("   Capability genes: {}", genetics.capability_genes.len());
                info!("   Offspring count: {}", genetics.offspring_count);
                
                // Analyze genetic diversity
                let diversity_score = calculate_genetic_diversity(&genetics);
                info!("   Genetic diversity score: {:.3}", diversity_score);
                
                // Analyze security traits
                info!("   Security traits:");
                info!("     • Paranoia level: {:.3}", genetics.security_traits.paranoia_level);
                info!("     • Cooperation tendency: {:.3}", genetics.security_traits.cooperation_tendency);
                info!("     • Innovation rate: {:.3}", genetics.security_traits.innovation_rate);
                info!("     • Threat sensitivity: {:.3}", genetics.security_traits.threat_sensitivity);
            }
            Err(e) => {
                error!("❌ Failed to get genetics for {}: {}", child_node, e);
            }
        }
    }

    Ok(())
}

/// Calculate a simple genetic diversity score
fn calculate_genetic_diversity(genetics: &beardog::auth::BearDogGenetics) -> f64 {
    let mut diversity_factors = Vec::new();

    // Factor in chromosome variety
    let chromosome_variety = genetics.crypto_chromosomes.len() as f64 / 10.0; // Normalize
    diversity_factors.push(chromosome_variety.min(1.0));

    // Factor in capability variety
    let capability_variety = genetics.capability_genes.len() as f64 / 10.0; // Normalize
    diversity_factors.push(capability_variety.min(1.0));

    // Factor in generation depth (older lineages have more diversity)
    let generation_factor = (genetics.generation as f64).min(10.0) / 10.0;
    diversity_factors.push(generation_factor);

    // Factor in parent count (more parents = more diversity)
    let parent_factor = (genetics.parent_genomes.len() as f64).min(5.0) / 5.0;
    diversity_factors.push(parent_factor);

    // Average all factors
    diversity_factors.iter().sum::<f64>() / diversity_factors.len() as f64
}

/// Display spawn request summary
fn display_spawn_summary(request: &SpawnRequest) {
    info!("📋 Spawn Request Summary:");
    info!("   Request ID: {}", request.request_id);
    info!("   Parent: {}", request.requesting_parent);
    info!("   Co-parents: {:?}", request.co_parents);
    info!("   Purpose: {:?}", request.purpose);
    info!("   Resources: CPU {}%, RAM {}MB, Storage {}GB", 
          request.resource_requirements.max_cpu_percent,
          request.resource_requirements.max_memory_mb,
          request.resource_requirements.max_storage_gb);
    info!("   Workflow: {:?}", request.workflow_type);
}
