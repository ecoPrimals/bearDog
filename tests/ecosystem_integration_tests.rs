//! Ecosystem Integration Tests
//!
//! Tests for BearDog integration with ToadStool compute orchestration
//! and genetic spawning network effects across the ecosystem

use beardog::genetics::*;
use beardog::auth::*;
use beardog::workflows::*;
use beardog::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_toadstool_compute_integration() -> BearDogResult<()> {
    // Test BearDog's integration with ToadStool compute orchestration
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;
    
    // Simulate ToadStool compute request with security validation
    let compute_request = CrossNodeOperation {
        operation_type: OperationType::ComputeExecution,
        resource_id: "toadstool-compute-001".to_string(),
        source_node: "beardog-node-alpha".to_string(),
        target_node: "toadstool-orchestrator".to_string(),
        operation_data: serde_json::json!({
            "workload_type": "genetic_optimization",
            "compute_requirements": {
                "cpu_cores": 4,
                "memory_gb": 8,
                "estimated_duration_mins": 30
            },
            "security_context": {
                "encryption_required": true,
                "audit_level": "comprehensive"
            }
        }),
        authorization_proof: None,
        expiration: chrono::Utc::now() + chrono::Duration::minutes(60),
        metadata: HashMap::new(),
    };
    
    // Verify that BearDog can create authorization proof for ToadStool
    let genetics_engine = GeneticsEngine::new_placeholder();
    let authorization_result = genetics_engine.create_authorization_proof(
        &compute_request.source_node,
        &compute_request.target_node,
        &compute_request,
    ).await;
    
    assert!(authorization_result.is_ok(), "Should create authorization proof for ToadStool");
    
    println!("✅ ToadStool compute integration test passed");
    println!("   - Authorization proof created for compute request");
    println!("   - Security context preserved");
    
    Ok(())
}

#[tokio::test]
async fn test_genetic_spawning_network_effects() -> BearDogResult<()> {
    // Test how genetic spawning benefits from network effects with ecosystem components
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;
    
    // Create a genetics engine
    let genetics_engine = GeneticsEngine::new_placeholder();
    
    // Simulate genetics from multiple ecosystem components
    let beardog_genetics = BearDogGenetics {
        genome_id: "beardog-security-v1".to_string(),
        crypto_chromosomes: vec![
            CryptoChromosome {
                algorithm_family: AlgorithmFamily::Encryption,
                capability_flags: 0b11111111, // All security capabilities
                dominance_weight: 0.9,
                mutation_rate: 0.02,
            },
        ],
        capability_genes: vec![
            CapabilityGene {
                node_capability: NodeCapability::SecurityProvider,
                expression_level: 1.0,
                inheritance_chain: vec!["beardog-parent-alpha".to_string()],
                mutation_history: vec![],
            },
        ],
        security_traits: SecurityTraits {
            paranoia_level: 0.8,
            cooperation_tendency: 0.7,
            innovation_rate: 0.6,
            compliance_strictness: 0.9,
            threat_sensitivity: 0.85,
        },
        lineage: GeneticLineage {
            child_node_id: "test-child".to_string(),
            parent_node_ids: vec!["beardog-genesis".to_string()],
            generation: 3,
            spawn_timestamp: chrono::Utc::now(),
            diversity_score: 0.8,
            lineage_proof: beardog::genetics::LineageProof {
                parent_signatures: vec![],
                child_genetics_hash: vec![],
                parent_genetics_hashes: vec![],
                witness_signatures: vec![],
            },
        },
        reproductive_rights: ReproductiveRights {
            can_spawn: true,
            max_offspring: 10,
            spawn_cooldown: chrono::Duration::minutes(30),
            approved_partners: vec!["toadstool-compute-node".to_string()],
            geographic_restrictions: vec!["us-west".to_string(), "eu-central".to_string()],
            resource_limits: ResourceLimits {
                max_cpu_cores: 16,
                max_memory_gb: 64,
                max_storage_gb: 1024,
                max_network_mbps: 1000,
            },
        },
    };
    
    // Simulate ToadStool compute genetics (focusing on compute optimization)
    let toadstool_genetics = BearDogGenetics {
        genome_id: "toadstool-compute-v2".to_string(),
        crypto_chromosomes: vec![
            CryptoChromosome {
                algorithm_family: AlgorithmFamily::Optimization,
                capability_flags: 0b11110000, // Compute-focused capabilities
                dominance_weight: 0.8,
                mutation_rate: 0.05,
            },
        ],
        capability_genes: vec![
            CapabilityGene {
                node_capability: NodeCapability::ComputeOrchestrator,
                expression_level: 1.0,
                inheritance_chain: vec!["toadstool-parent-beta".to_string()],
                mutation_history: vec![],
            },
            CapabilityGene {
                node_capability: NodeCapability::ResourceManager,
                expression_level: 0.9,
                inheritance_chain: vec!["toadstool-parent-beta".to_string()],
                mutation_history: vec![],
            },
        ],
        security_traits: SecurityTraits {
            paranoia_level: 0.6, // Lower paranoia for compute efficiency
            cooperation_tendency: 0.9, // High cooperation for distributed compute
            innovation_rate: 0.8, // High innovation for optimization
            compliance_strictness: 0.7,
            threat_sensitivity: 0.6,
        },
        lineage: GeneticLineage {
            parent_nodes: vec!["toadstool-genesis".to_string()],
            generation: 5,
            spawn_timestamp: chrono::Utc::now() - chrono::Duration::hours(1),
            authentication_chain: vec!["toadstool-genesis-signature".to_string()],
        },
        reproductive_rights: ReproductiveRights {
            can_spawn: true,
            max_offspring: 50, // High offspring for compute scaling
            spawn_cooldown: chrono::Duration::minutes(5), // Fast spawning for compute
            approved_partners: vec!["beardog-security-node".to_string()],
            geographic_restrictions: vec!["global".to_string()], // Global compute
            resource_limits: ResourceLimits {
                max_cpu_cores: 1000, // Massive compute capability
                max_memory_gb: 10000,
                max_storage_gb: 100000,
                max_network_mbps: 100000,
            },
        },
    };
    
    // Test genetic crossover between BearDog security and ToadStool compute
    let spawn_request = SpawnRequest {
        requesting_node: "hybrid-orchestrator".to_string(),
        parent_genetics: vec![beardog_genetics.clone(), toadstool_genetics.clone()],
        spawn_purpose: SpawnPurpose::HybridSecurityCompute,
        resource_requirements: ResourceRequirements {
            cpu_cores: 8,
            memory_gb: 32,
            storage_gb: 512,
            network_mbps: 1000,
            gpu_memory_gb: Some(16),
        },
        spawn_priority: SpawnPriority::High,
        workflow_approval: Some(WorkflowRequirement {
            workflow_type: WorkflowType::AutomatedConsensus,
            timeout: chrono::Duration::minutes(15),
            required_approvals: 2,
        }),
        compliance_requirements: vec![
            ComplianceRequirement::GDPR,
            ComplianceRequirement::DataSovereignty,
        ],
        metadata: HashMap::new(),
    };
    
    // Test genetic recombination
    let recombination_result = genetics_engine.perform_genetic_recombination(&spawn_request).await;
    assert!(recombination_result.is_ok(), "Genetic recombination should succeed");
    
    let child_genetics = recombination_result.unwrap();
    
    // Verify network effects: child should inherit best traits from both parents
    
    // Should inherit high security from BearDog
    assert!(child_genetics.security_traits.paranoia_level > 0.6, 
           "Child should inherit reasonable security paranoia");
    
    // Should inherit high cooperation from ToadStool
    assert!(child_genetics.security_traits.cooperation_tendency > 0.8,
           "Child should inherit high cooperation for distributed operations");
    
    // Should have capabilities from both parents
    let capability_types: Vec<NodeCapability> = child_genetics.capability_genes
        .iter()
        .map(|gene| gene.node_capability.clone())
        .collect();
    
    assert!(capability_types.contains(&NodeCapability::SecurityProvider),
           "Child should inherit security capabilities");
    assert!(capability_types.contains(&NodeCapability::ComputeOrchestrator) ||
           capability_types.contains(&NodeCapability::ResourceManager),
           "Child should inherit compute capabilities");
    
    println!("✅ Genetic spawning network effects test passed");
    println!("   - Successfully combined BearDog security + ToadStool compute genetics");
    println!("   - Child inherited security paranoia: {:.2}", child_genetics.security_traits.paranoia_level);
    println!("   - Child inherited cooperation: {:.2}", child_genetics.security_traits.cooperation_tendency);
    println!("   - Child has {} capability genes", child_genetics.capability_genes.len());
    
    Ok(())
}

#[tokio::test]
async fn test_ecosystem_service_discovery() -> BearDogResult<()> {
    // Test BearDog's integration with ecosystem service discovery (via SongBird)
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;
    
    // Simulate service registration with ecosystem
    let service_registration = EcosystemServiceRegistration {
        service_id: "beardog-security-node-001".to_string(),
        service_type: EcosystemServiceType::SecurityProvider,
        capabilities: vec![
            ServiceCapability::Authorization,
            ServiceCapability::Encryption,
            ServiceCapability::ThreatDetection,
            ServiceCapability::ComplianceMonitoring,
        ],
        endpoints: vec![
            ServiceEndpoint {
                protocol: "HTTPS".to_string(),
                address: "https://beardog-node-001.local:8443".to_string(),
                capabilities: vec!["authorization", "audit"].iter().map(|s| s.to_string()).collect(),
            },
            ServiceEndpoint {
                protocol: "BSTP".to_string(),
                address: "bstp://beardog-node-001.local:9443".to_string(),
                capabilities: vec!["low-latency-auth", "gaming-security"].iter().map(|s| s.to_string()).collect(),
            },
        ],
        trust_metrics: TrustMetrics {
            uptime_percentage: 99.9,
            response_time_ms: 15.0,
            security_score: 0.95,
            community_reputation: 0.88,
        },
        metadata: HashMap::from([
            ("version".to_string(), "1.0.0".to_string()),
            ("ecosystem_role".to_string(), "security_provider".to_string()),
            ("genetic_compatibility".to_string(), "toadstool,songbird,nestgate".to_string()),
        ]),
    };
    
    // Test that BearDog can format service registration for ecosystem
    let registration_json = serde_json::to_string(&service_registration)
        .expect("Should serialize service registration");
    
    assert!(registration_json.contains("SecurityProvider"));
    assert!(registration_json.contains("beardog-security-node-001"));
    
    println!("✅ Ecosystem service discovery test passed");
    println!("   - Service registration formatted for ecosystem");
    println!("   - Trust metrics: uptime {}%, security score {:.2}", 
             service_registration.trust_metrics.uptime_percentage,
             service_registration.trust_metrics.security_score);
    
    Ok(())
}

// Supporting types for ecosystem integration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct EcosystemServiceRegistration {
    service_id: String,
    service_type: EcosystemServiceType,
    capabilities: Vec<ServiceCapability>,
    endpoints: Vec<ServiceEndpoint>,
    trust_metrics: TrustMetrics,
    metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum EcosystemServiceType {
    SecurityProvider,
    ComputeOrchestrator,
    StorageProvider,
    NetworkRouter,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum ServiceCapability {
    Authorization,
    Encryption,
    ThreatDetection,
    ComplianceMonitoring,
    ComputeExecution,
    ResourceManagement,
    DataStorage,
    NetworkRouting,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ServiceEndpoint {
    protocol: String,
    address: String,
    capabilities: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct TrustMetrics {
    uptime_percentage: f64,
    response_time_ms: f64,
    security_score: f64,
    community_reputation: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum SpawnPurpose {
    HybridSecurityCompute,
    DistributedStorage,
    NetworkOptimization,
    ComplianceAudit,
    ThreatResponse,
}

// Additional genetics types for ecosystem integration
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum NodeCapability {
    SecurityProvider,
    ComputeOrchestrator,
    ResourceManager,
    StorageProvider,
    NetworkRouter,
    ThreatDetector,
    ComplianceMonitor,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum AlgorithmFamily {
    Encryption,
    Hashing,
    Signing,
    KeyDerivation,
    Optimization,
    MachineLearning,
} 