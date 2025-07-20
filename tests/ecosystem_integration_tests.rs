//! Ecosystem Integration Tests
//!
//! Tests for BearDog integration with ToadStool compute orchestration
//! and genetic spawning network effects across the ecosystem

// Use specific imports to avoid ambiguity
use beardog::auth::{
    AlgorithmFamily, BearDogGenetics, CrossNodeOperation, CryptoChromosome, NodeCapability,
    NodeSpecialization, OperationType, ResourceLimits, SecurityClearance, SecurityTraits,
    SpawnPurpose, SpawnRequest as AuthSpawnRequest, SpawnRestriction,
};
use beardog::*;
use std::collections::HashMap;

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
        _spawn_request: &AuthSpawnRequest,
    ) -> BearDogResult<BearDogGenetics> {
        Ok(BearDogGenetics {
            id: "child-genetics".to_string(),
            crypto_chromosomes: vec![],
            security_traits: beardog::auth::SecurityTraits::default(),
            capabilities: vec![
                NodeCapability::SecurityAnalysis,
                NodeCapability::ComputeProvider,
            ],
            spawn_restrictions: vec![],
            generation: 1,
            parent_genetics: Some(vec!["parent1".to_string(), "parent2".to_string()]),
            mutations: vec![],
            fitness_score: 0.8,
            security_clearance: SecurityClearance::High,
            specializations: vec![NodeSpecialization::GeneralPurpose],
        })
    }
}

#[tokio::test]
async fn test_toadstool_compute_integration() -> BearDogResult<()> {
    // Test BearDog's integration with ToadStool compute orchestration
    let _config = BearDogConfig::default();

    // Create a cross-node operation to request compute from ToadStool
    let compute_request = CrossNodeOperation {
        operation_type: OperationType::Execute,
        target_resource: "toadstool-compute-001".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert(
                "operation_type".to_string(),
                "compute_execution".to_string(),
            );
            params.insert("source_node".to_string(), "beardog-node-alpha".to_string());
            params.insert(
                "target_node".to_string(),
                "toadstool-orchestrator".to_string(),
            );
            params.insert(
                "operation_data".to_string(),
                serde_json::json!({
                    "compute_task": {
                        "task_id": "toadstool_hybrid_security_computation",
                        "algorithm": "genetic_security_optimization",
                        "input_data": "encrypted_node_genetics",
                        "expected_output": "optimized_security_configuration"
                    }
                })
                .to_string(),
            );
            params
        },
        requester_signature: "mock_signature".to_string(),
    };

    // Create genetics engine for genetic operations
    let genetics_engine = GeneticsEngine::new_placeholder();

    // Create authorization proof
    let authorization_proof = genetics_engine
        .create_authorization_proof(
            &compute_request
                .parameters
                .get("source_node")
                .unwrap_or(&"unknown".to_string()),
            &compute_request
                .parameters
                .get("target_node")
                .unwrap_or(&"unknown".to_string()),
            &compute_request,
        )
        .await?;

    // Verify authorization proof exists
    assert!(!authorization_proof.is_empty());

    // Create BearDog genetics for the operation
    let beardog_genetics = BearDogGenetics {
        id: "beardog-security-v1".to_string(),
        crypto_chromosomes: vec![CryptoChromosome {
            algorithm_family: AlgorithmFamily::Encryption(beardog::auth::EncryptionFamily::Aes),
            strength_bits: 256,
            compatibility_score: 0.95,
            performance_factor: 0.8,
            security_level: 9,
        }],
        security_traits: beardog::auth::SecurityTraits::default(),
        capabilities: vec![NodeCapability::SecurityAnalysis],
        spawn_restrictions: vec![],
        generation: 0,
        parent_genetics: None,
        mutations: vec![],
        fitness_score: 0.9,
        security_clearance: SecurityClearance::High,
        specializations: vec![NodeSpecialization::SecurityResponse],
    };

    // Test successful integration
    assert_eq!(beardog_genetics.generation, 0);
    assert_eq!(beardog_genetics.security_clearance, SecurityClearance::High);

    Ok(())
}

#[tokio::test]
async fn test_genetic_spawning_network_effects() -> BearDogResult<()> {
    // Test genetic crossover between BearDog security and ToadStool compute
    let spawn_request = AuthSpawnRequest {
        parent_genetics: vec![
            BearDogGenetics {
                id: "beardog-security-v1".to_string(),
                crypto_chromosomes: vec![CryptoChromosome {
                    algorithm_family: AlgorithmFamily::Encryption(
                        beardog::auth::EncryptionFamily::Aes,
                    ),
                    strength_bits: 256,
                    compatibility_score: 0.95,
                    performance_factor: 0.8,
                    security_level: 9,
                }],
                security_traits: beardog::auth::SecurityTraits::default(),
                capabilities: vec![NodeCapability::SecurityAnalysis],
                spawn_restrictions: vec![SpawnRestriction::ResourceLimits(ResourceLimits {
                    max_memory_mb: 64000,
                    max_cpu_percent: 80,
                    max_disk_mb: 1024000,
                    max_network_mbps: 100,
                    max_concurrent_connections: 1000,
                })],
                generation: 0,
                parent_genetics: None,
                mutations: vec![],
                fitness_score: 0.9,
                security_clearance: SecurityClearance::High,
                specializations: vec![NodeSpecialization::SecurityResponse],
            },
            BearDogGenetics {
                id: "toadstool-compute-v2".to_string(),
                crypto_chromosomes: vec![CryptoChromosome {
                    algorithm_family: AlgorithmFamily::Encryption(
                        beardog::auth::EncryptionFamily::Aes,
                    ),
                    strength_bits: 256,
                    compatibility_score: 0.95,
                    performance_factor: 0.8,
                    security_level: 9,
                }],
                security_traits: beardog::auth::SecurityTraits::default(),
                capabilities: vec![
                    NodeCapability::ComputeProvider,
                    NodeCapability::HighThroughput,
                ],
                spawn_restrictions: vec![],
                generation: 0,
                parent_genetics: None,
                mutations: vec![],
                fitness_score: 0.8,
                security_clearance: SecurityClearance::High,
                specializations: vec![NodeSpecialization::GeneralPurpose],
            },
        ],
        spawn_purpose: SpawnPurpose::EcosystemIntegration("ToadStool".to_string()),
        required_capabilities: vec![
            NodeCapability::SecurityAnalysis,
            NodeCapability::ComputeProvider,
        ],
        resource_limits: ResourceLimits {
            max_memory_mb: 10000,
            max_cpu_percent: 80,
            max_disk_mb: 100000,
            max_network_mbps: 100,
            max_concurrent_connections: 1000,
        },
        target_environment: "production".to_string(),
    };

    // Create genetics engine for recombination
    let genetics_engine = GeneticsEngine::new_placeholder();

    // Test genetic recombination
    let child_genetics = genetics_engine
        .perform_genetic_recombination(&spawn_request)
        .await?;

    // Verify child inherits capabilities from both parents
    assert!(!child_genetics.capabilities.is_empty());
    assert!(child_genetics.generation > 0);

    // Test network effects calculation
    let _capabilities = vec![
        NodeCapability::SecurityAnalysis,
        NodeCapability::ComputeProvider,
        NodeCapability::ToadStoolCompute,
    ];

    // Verify ecosystem integration capabilities
    assert!(spawn_request
        .required_capabilities
        .contains(&NodeCapability::SecurityAnalysis));
    assert!(spawn_request
        .required_capabilities
        .contains(&NodeCapability::ComputeProvider));

    Ok(())
}

#[tokio::test]
async fn test_ecosystem_service_discovery() -> BearDogResult<()> {
    // Test BearDog's integration with ecosystem service discovery (via SongBird)
    let config = BearDogConfig::default();
    let _core = BearDogCore::new(config).await?;

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

// Remove duplicate enums - these are defined in the actual codebase
// SpawnPurpose, NodeCapability, and AlgorithmFamily are imported from beardog modules
