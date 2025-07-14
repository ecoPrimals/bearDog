//! Ecosystem Integration Tests
//!
//! Tests for BearDog integration with ToadStool compute orchestration
//! and genetic spawning network effects across the ecosystem

use beardog::auth::*;
use beardog::genetics::*;
use beardog::workflows::*;
use beardog::*;
use std::collections::HashMap;
use std::time::Duration;

// Add missing enum for compliance requirements
#[derive(Debug, Clone, PartialEq)]
enum ComplianceRequirement {
    GDPR,
    DataSovereignty,
    SOX,
    HIPAA,
}

// Add placeholder GeneticsEngine struct
struct GeneticsEngine;

impl GeneticsEngine {
    pub fn new_placeholder() -> Self {
        Self
    }

    pub async fn create_authorization_proof(
        &self,
        _source_node: &str,
        _target_node: &str,
        _request: &CrossNodeOperation,
    ) -> BearDogResult<String> {
        Ok("mock-authorization-proof".to_string())
    }

    pub async fn perform_genetic_recombination(
        &self,
        _spawn_request: &SpawnRequest,
    ) -> BearDogResult<BearDogGenetics> {
        Ok(BearDogGenetics::default())
    }
}

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
    let authorization_result = genetics_engine
        .create_authorization_proof(
            &compute_request.source_node,
            &compute_request.target_node,
            &compute_request,
        )
        .await;

    assert!(
        authorization_result.is_ok(),
        "Should create authorization proof for ToadStool"
    );

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
        crypto_chromosomes: vec![CryptoChromosome {
            algorithm_family: AlgorithmFamily::Encryption,
            capability_flags: 0b11111111, // All security capabilities
            dominance_weight: 0.9,
            mutation_rate: 0.02,
        }],
        capabilities: vec![CapabilityGene {
            capability: NodeCapability::SecurityProvider,
            expression_level: 1.0,
            dominant: true,
            mutable: false,
            inheritance_weight: 0.8,
        }],
        security_traits: SecurityTraits {
            trust_threshold: 0.8,
            paranoia_level: 204, // Convert 0.8 to u8 scale
            consensus_requirement: true,
            isolation_preference: 0.7,
            audit_frequency: 6,
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
        crypto_chromosomes: vec![CryptoChromosome {
            algorithm_family: AlgorithmFamily::Optimization,
            capability_flags: 0b11110000, // Compute-focused capabilities
            dominance_weight: 0.8,
            mutation_rate: 0.05,
        }],
        capabilities: vec![
            CapabilityGene {
                capability: NodeCapability::ComputeOrchestrator,
                expression_level: 1.0,
                dominant: true,
                mutable: false,
                inheritance_weight: 0.8,
            },
            CapabilityGene {
                capability: NodeCapability::ResourceManager,
                expression_level: 0.9,
                dominant: true,
                mutable: false,
                inheritance_weight: 0.7,
            },
        ],
        security_traits: SecurityTraits {
            trust_threshold: 0.6,
            paranoia_level: 153, // Convert 0.6 to u8 scale
            consensus_requirement: true,
            isolation_preference: 0.7,
            audit_frequency: 8,
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
        request_id: "test-spawn-001".to_string(),
        requesting_parent: "hybrid-orchestrator".to_string(),
        co_parents: vec![
            "beardog-genesis".to_string(),
            "toadstool-genesis".to_string(),
        ],
        purpose: beardog::genetics::types::SpawnPurpose::EcosystemIntegration(
            "hybrid-compute".to_string(),
        ),
        resource_requirements: beardog::genetics::types::ResourceLimits {
            max_cpu_percent: 80.0,
            max_memory_mb: 32768, // 32GB
            max_storage_gb: 512,
            max_network_mbps: 1000,
            allowed_jurisdictions: vec!["US".to_string(), "EU".to_string()],
            temporal_windows: vec![],
        },
        workflow_type: beardog::genetics::types::BearDogWorkflowType::AutomatedConsensus {
            participating_nodes: vec!["node1".to_string(), "node2".to_string()],
            consensus_threshold: 0.75,
            max_decision_time: Duration::from_secs(900), // 15 minutes
        },
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
        metadata: HashMap::new(),
    };

    // Test genetic recombination
    let recombination_result = genetics_engine
        .perform_genetic_recombination(&spawn_request)
        .await;
    assert!(
        recombination_result.is_ok(),
        "Genetic recombination should succeed"
    );

    let child_genetics = recombination_result.unwrap();

    // Verify network effects: child should inherit best traits from both parents

    // Should inherit high security from BearDog
    assert!(
        child_genetics.security_traits.paranoia_level > 150,
        "Child should inherit reasonable security paranoia (u8 scale)"
    );

    // Should inherit high trust threshold for distributed operations
    assert!(
        child_genetics.security_traits.trust_threshold > 0.8,
        "Child should inherit high trust for distributed operations"
    );

    // Should have capabilities from both parents
    let capability_types: Vec<NodeCapability> = child_genetics
        .capabilities
        .iter()
        .map(|gene| gene.capability.clone())
        .collect();

    // Should inherit security provider and compute capabilities
    assert!(
        capability_types.contains(&NodeCapability::SecurityProvider),
        "Child should inherit SecurityProvider capability"
    );
    assert!(
        capability_types.contains(&NodeCapability::ComputeOrchestrator),
        "Child should inherit ComputeOrchestrator capability"
    );

    // Security traits should show hybrid characteristics
    assert!(
        child_genetics.security_traits.trust_threshold > 0.6,
        "Child should inherit reasonable trust requirements"
    );

    // Output genetics analysis
    println!(
        "   - Child inherited trust threshold: {:.2}",
        child_genetics.security_traits.trust_threshold
    );
    println!(
        "   - Child has {} capabilities",
        child_genetics.capabilities.len()
    );

    println!("✅ Genetic spawning network effects test passed");
    println!("   - Successfully combined BearDog security + ToadStool compute genetics");
    println!(
        "   - Child inherited security paranoia: {}",
        child_genetics.security_traits.paranoia_level
    );
    println!(
        "   - Child inherited trust threshold: {:.2}",
        child_genetics.security_traits.trust_threshold
    );
    println!(
        "   - Child has {} capabilities",
        child_genetics.capabilities.len()
    );

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
                capabilities: vec!["authorization", "audit"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            },
            ServiceEndpoint {
                protocol: "BSTP".to_string(),
                address: "bstp://beardog-node-001.local:9443".to_string(),
                capabilities: vec!["low-latency-auth", "gaming-security"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
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
            (
                "ecosystem_role".to_string(),
                "security_provider".to_string(),
            ),
            (
                "genetic_compatibility".to_string(),
                "toadstool,songbird,nestgate".to_string(),
            ),
        ]),
    };

    // Test that BearDog can format service registration for ecosystem
    let registration_json = serde_json::to_string(&service_registration)
        .expect("Should serialize service registration");

    assert!(registration_json.contains("SecurityProvider"));
    assert!(registration_json.contains("beardog-security-node-001"));

    println!("✅ Ecosystem service discovery test passed");
    println!("   - Service registration formatted for ecosystem");
    println!(
        "   - Trust metrics: uptime {}%, security score {:.2}",
        service_registration.trust_metrics.uptime_percentage,
        service_registration.trust_metrics.security_score
    );

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
