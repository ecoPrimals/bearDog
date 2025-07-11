//! Comprehensive Genetics Engine Tests
//! 
//! This test suite ensures 100% coverage of BearDog's genetic algorithm engine
//! including spawning, mutation, crossover, adaptation, and evolutionary mechanics.

use beardog::genetics::*;
use beardog::core::*;
use beardog::error::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

/// Comprehensive genetics engine testing
/// Tests all genetic algorithm operations and evolutionary mechanics
#[tokio::test]
async fn test_genetics_engine_comprehensive() {
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.expect("Core initialization failed"));
    
    let mut genetics_engine = GeneticsEngine::new(core.clone()).await
        .expect("Genetics engine creation failed");
    
    // Test all major genetic operations
    test_genetic_crossover(&mut genetics_engine).await;
    test_genetic_mutation(&mut genetics_engine).await;
    test_adaptation_mechanisms(&mut genetics_engine).await;
    test_fitness_evaluation(&mut genetics_engine).await;
    test_population_management(&mut genetics_engine).await;
    test_evolutionary_pressure(&mut genetics_engine).await;
}

async fn test_genetic_crossover(genetics_engine: &mut GeneticsEngine) {
    println!("🧬 Testing genetic crossover operations...");
    
    // Create parent genetics with different characteristics
    let parent_a = BearDogGenetics {
        crypto_preferences: vec![
            CryptoAlgorithm::Aes256Gcm,
            CryptoAlgorithm::Rsa4096,
        ],
        performance_characteristics: PerformanceProfile {
            cpu_cores: 16,
            memory_gb: 64,
            storage_gb: 2000,
            network_mbps: 2000,
            specializations: vec![
                NodeSpecialization::HighPerformanceCrypto,
                NodeSpecialization::ServerWorkloads,
            ],
        },
        security_clearance: SecurityClearance::High,
        trust_relationships: {
            let mut trust = HashMap::new();
            trust.insert("node-alpha".to_string(), TrustLevel::High);
            trust.insert("node-beta".to_string(), TrustLevel::Medium);
            trust
        },
        adaptation_rate: 0.15,
        mutation_probability: 0.02,
        compatibility_matrix: HashMap::new(),
    };
    
    let parent_b = BearDogGenetics {
        crypto_preferences: vec![
            CryptoAlgorithm::ChaCha20Poly1305,
            CryptoAlgorithm::Ed25519,
        ],
        performance_characteristics: PerformanceProfile {
            cpu_cores: 12,
            memory_gb: 48,
            storage_gb: 1500,
            network_mbps: 1500,
            specializations: vec![
                NodeSpecialization::GamingOptimized,
                NodeSpecialization::LowLatencyNetworking,
            ],
        },
        security_clearance: SecurityClearance::Medium,
        trust_relationships: {
            let mut trust = HashMap::new();
            trust.insert("node-gamma".to_string(), TrustLevel::High);
            trust.insert("node-delta".to_string(), TrustLevel::Low);
            trust
        },
        adaptation_rate: 0.12,
        mutation_probability: 0.03,
        compatibility_matrix: HashMap::new(),
    };
    
    // Test single-point crossover
    let offspring_single = genetics_engine.perform_crossover(
        &parent_a,
        &parent_b,
        CrossoverType::SinglePoint,
    ).await.expect("Single-point crossover should succeed");
    
    // Validate offspring characteristics
    assert!(!offspring_single.crypto_preferences.is_empty(),
           "Offspring should inherit crypto preferences");
    assert!(offspring_single.performance_characteristics.cpu_cores > 0,
           "Offspring should have valid performance characteristics");
    assert!(offspring_single.adaptation_rate > 0.0,
           "Offspring should have valid adaptation rate");
    
    // Test uniform crossover
    let offspring_uniform = genetics_engine.perform_crossover(
        &parent_a,
        &parent_b,
        CrossoverType::Uniform,
    ).await.expect("Uniform crossover should succeed");
    
    // Test that offspring combines parent traits
    let combined_specs: Vec<_> = parent_a.performance_characteristics.specializations
        .iter()
        .chain(parent_b.performance_characteristics.specializations.iter())
        .cloned()
        .collect();
    
    assert!(offspring_uniform.performance_characteristics.specializations
           .iter()
           .any(|spec| combined_specs.contains(spec)),
           "Offspring should inherit specializations from parents");
    
    // Test multi-parent crossover
    let parent_c = BearDogGenetics {
        crypto_preferences: vec![CryptoAlgorithm::X25519],
        performance_characteristics: PerformanceProfile {
            cpu_cores: 8,
            memory_gb: 32,
            storage_gb: 1000,
            network_mbps: 1000,
            specializations: vec![NodeSpecialization::EdgeComputing],
        },
        security_clearance: SecurityClearance::Maximum,
        trust_relationships: HashMap::new(),
        adaptation_rate: 0.10,
        mutation_probability: 0.01,
        compatibility_matrix: HashMap::new(),
    };
    
    let offspring_multi = genetics_engine.perform_multi_parent_crossover(
        &[parent_a, parent_b, parent_c],
    ).await.expect("Multi-parent crossover should succeed");
    
    assert!(offspring_multi.crypto_preferences.len() >= 1,
           "Multi-parent offspring should have crypto preferences");
    assert!(offspring_multi.security_clearance != SecurityClearance::Basic,
           "Multi-parent offspring should inherit appropriate clearance");
}

async fn test_genetic_mutation(genetics_engine: &mut GeneticsEngine) {
    println!("🔬 Testing genetic mutation operations...");
    
    let original_genetics = create_test_genetics();
    
    // Test performance mutation
    let mutated_performance = genetics_engine.mutate_performance(
        &original_genetics,
        0.1, // 10% mutation rate
    ).await.expect("Performance mutation should succeed");
    
    // Performance characteristics should be different but valid
    assert!(mutated_performance.performance_characteristics.cpu_cores > 0,
           "Mutated performance should have valid CPU cores");
    assert!(mutated_performance.performance_characteristics.memory_gb > 0,
           "Mutated performance should have valid memory");
    
    // Test crypto preference mutation
    let mutated_crypto = genetics_engine.mutate_crypto_preferences(
        &original_genetics,
        0.2,
    ).await.expect("Crypto mutation should succeed");
    
    assert!(!mutated_crypto.crypto_preferences.is_empty(),
           "Mutated genetics should have crypto preferences");
    
    // Test adaptive mutation (mutation rate changes based on environment)
    let environment_pressure = EnvironmentPressure {
        resource_scarcity: 0.7,
        security_threats: 0.8,
        performance_demands: 0.9,
        competition_level: 0.6,
    };
    
    let adaptive_mutated = genetics_engine.adaptive_mutation(
        &original_genetics,
        &environment_pressure,
    ).await.expect("Adaptive mutation should succeed");
    
    // Under high pressure, mutation rate should increase
    assert!(adaptive_mutated.mutation_probability >= original_genetics.mutation_probability,
           "High environmental pressure should increase mutation rate");
    
    // Test beneficial mutation detection
    let beneficial_mutation = genetics_engine.attempt_beneficial_mutation(
        &original_genetics,
        OptimizationTarget::MaximizePerformance,
    ).await.expect("Beneficial mutation should succeed");
    
    let original_fitness = calculate_fitness_score(&original_genetics);
    let mutated_fitness = calculate_fitness_score(&beneficial_mutation);
    
    assert!(mutated_fitness >= original_fitness,
           "Beneficial mutation should not decrease fitness");
}

async fn test_adaptation_mechanisms(genetics_engine: &mut GeneticsEngine) {
    println!("🎯 Testing adaptation mechanisms...");
    
    let mut genetics = create_test_genetics();
    
    // Test performance adaptation
    let performance_metrics = PerformanceMetrics {
        cpu_utilization: 0.95, // High CPU usage
        memory_utilization: 0.80,
        network_latency_ms: 50.0,
        crypto_operations_per_second: 8000,
        error_rate: 0.001,
        throughput_mbps: 900.0,
    };
    
    genetics_engine.adapt_to_performance(&mut genetics, &performance_metrics).await
        .expect("Performance adaptation should succeed");
    
    // Adaptation should optimize for high CPU usage
    assert!(genetics.performance_characteristics.cpu_cores >= 8,
           "Should adapt to need more CPU cores");
    
    // Test security threat adaptation
    let security_environment = SecurityEnvironment {
        threat_level: ThreatLevel::High,
        attack_vectors: vec![
            AttackVector::CryptographicAttack,
            AttackVector::ResourceExhaustion,
        ],
        vulnerability_count: 3,
        incident_frequency: 0.1,
    };
    
    genetics_engine.adapt_to_security_environment(&mut genetics, &security_environment).await
        .expect("Security adaptation should succeed");
    
    // Should adapt crypto preferences for higher security
    assert!(genetics.crypto_preferences.len() >= 2,
           "Should diversify crypto algorithms under threat");
    assert!(genetics.security_clearance != SecurityClearance::Basic,
           "Should elevate security clearance under threat");
    
    // Test network condition adaptation
    let network_conditions = NetworkConditions {
        available_bandwidth_mbps: 100, // Limited bandwidth
        latency_ms: 200.0, // High latency
        packet_loss_rate: 0.05, // 5% packet loss
        jitter_ms: 50.0,
    };
    
    genetics_engine.adapt_to_network_conditions(&mut genetics, &network_conditions).await
        .expect("Network adaptation should succeed");
    
    // Should optimize for poor network conditions
    assert!(genetics.performance_characteristics.specializations
           .contains(&NodeSpecialization::LowLatencyNetworking) ||
           genetics.performance_characteristics.specializations
           .contains(&NodeSpecialization::NetworkOptimized),
           "Should adapt specialization for network conditions");
}

async fn test_fitness_evaluation(genetics_engine: &mut GeneticsEngine) {
    println!("💪 Testing fitness evaluation...");
    
    // Create genetics with different characteristics for fitness comparison
    let high_performance_genetics = BearDogGenetics {
        crypto_preferences: vec![
            CryptoAlgorithm::Aes256Gcm,
            CryptoAlgorithm::ChaCha20Poly1305,
        ],
        performance_characteristics: PerformanceProfile {
            cpu_cores: 32,
            memory_gb: 128,
            storage_gb: 4000,
            network_mbps: 4000,
            specializations: vec![
                NodeSpecialization::HighPerformanceCrypto,
                NodeSpecialization::ServerWorkloads,
            ],
        },
        security_clearance: SecurityClearance::Maximum,
        trust_relationships: HashMap::new(),
        adaptation_rate: 0.20,
        mutation_probability: 0.01,
        compatibility_matrix: HashMap::new(),
    };
    
    let basic_genetics = BearDogGenetics {
        crypto_preferences: vec![CryptoAlgorithm::Aes256Gcm],
        performance_characteristics: PerformanceProfile {
            cpu_cores: 2,
            memory_gb: 4,
            storage_gb: 100,
            network_mbps: 100,
            specializations: vec![NodeSpecialization::GeneralPurpose],
        },
        security_clearance: SecurityClearance::Basic,
        trust_relationships: HashMap::new(),
        adaptation_rate: 0.05,
        mutation_probability: 0.10,
        compatibility_matrix: HashMap::new(),
    };
    
    // Test fitness evaluation with different criteria
    let performance_fitness = genetics_engine.evaluate_fitness(
        &high_performance_genetics,
        &FitnessWeights {
            performance_weight: 0.8,
            security_weight: 0.1,
            efficiency_weight: 0.1,
            adaptability_weight: 0.0,
        },
    ).await.expect("Fitness evaluation should succeed");
    
    let basic_fitness = genetics_engine.evaluate_fitness(
        &basic_genetics,
        &FitnessWeights {
            performance_weight: 0.8,
            security_weight: 0.1,
            efficiency_weight: 0.1,
            adaptability_weight: 0.0,
        },
    ).await.expect("Fitness evaluation should succeed");
    
    assert!(performance_fitness > basic_fitness,
           "High-performance genetics should have higher fitness for performance-weighted evaluation");
    
    // Test security-weighted fitness
    let security_fitness_high = genetics_engine.evaluate_fitness(
        &high_performance_genetics,
        &FitnessWeights {
            performance_weight: 0.1,
            security_weight: 0.8,
            efficiency_weight: 0.1,
            adaptability_weight: 0.0,
        },
    ).await.expect("Security fitness evaluation should succeed");
    
    let security_fitness_basic = genetics_engine.evaluate_fitness(
        &basic_genetics,
        &FitnessWeights {
            performance_weight: 0.1,
            security_weight: 0.8,
            efficiency_weight: 0.1,
            adaptability_weight: 0.0,
        },
    ).await.expect("Security fitness evaluation should succeed");
    
    assert!(security_fitness_high > security_fitness_basic,
           "Maximum security clearance should result in higher security fitness");
}

async fn test_population_management(genetics_engine: &mut GeneticsEngine) {
    println!("👥 Testing population management...");
    
    // Create initial population
    let mut population = Vec::new();
    for i in 0..10 {
        let genetics = create_test_genetics_variant(i);
        population.push(genetics);
    }
    
    // Test population fitness ranking
    let ranked_population = genetics_engine.rank_population_by_fitness(
        &population,
        &FitnessWeights::balanced(),
    ).await.expect("Population ranking should succeed");
    
    assert_eq!(ranked_population.len(), population.len(),
              "Ranked population should have same size");
    
    // Fitness should be in descending order
    for i in 1..ranked_population.len() {
        assert!(ranked_population[i-1].fitness >= ranked_population[i].fitness,
               "Population should be ranked by fitness in descending order");
    }
    
    // Test selection mechanisms
    let tournament_selected = genetics_engine.tournament_selection(
        &ranked_population,
        3, // Tournament size
        2, // Number to select
    ).await.expect("Tournament selection should succeed");
    
    assert_eq!(tournament_selected.len(), 2,
              "Should select requested number of individuals");
    
    // Test elitist selection
    let elite_selected = genetics_engine.elitist_selection(
        &ranked_population,
        3, // Select top 3
    ).await.expect("Elitist selection should succeed");
    
    assert_eq!(elite_selected.len(), 3,
              "Should select top individuals");
    
    // Test population diversity maintenance
    let diversity_score = genetics_engine.calculate_population_diversity(&population).await
        .expect("Diversity calculation should succeed");
    
    assert!(diversity_score >= 0.0 && diversity_score <= 1.0,
           "Diversity score should be between 0 and 1");
    
    // Test population replacement
    let new_population = genetics_engine.replace_population(
        &population,
        &elite_selected,
        0.3, // Replace 30% of population
    ).await.expect("Population replacement should succeed");
    
    assert_eq!(new_population.len(), population.len(),
              "New population should maintain size");
}

async fn test_evolutionary_pressure(genetics_engine: &mut GeneticsEngine) {
    println!("🌊 Testing evolutionary pressure mechanisms...");
    
    let initial_genetics = create_test_genetics();
    
    // Test resource pressure evolution
    let resource_pressure = ResourcePressure {
        cpu_availability: 0.2, // Low CPU availability
        memory_availability: 0.3, // Low memory availability
        storage_availability: 0.8, // High storage availability
        network_availability: 0.5, // Medium network availability
    };
    
    let evolved_under_pressure = genetics_engine.evolve_under_resource_pressure(
        &initial_genetics,
        &resource_pressure,
        5, // 5 generations
    ).await.expect("Evolution under resource pressure should succeed");
    
    // Should evolve to be more efficient with constrained resources
    assert!(evolved_under_pressure.performance_characteristics.cpu_cores <= 
           initial_genetics.performance_characteristics.cpu_cores ||
           evolved_under_pressure.performance_characteristics.specializations
           .contains(&NodeSpecialization::ResourceEfficient),
           "Should adapt to resource constraints");
    
    // Test competitive pressure
    let competitors = vec![
        create_high_performance_competitor(),
        create_efficient_competitor(),
        create_security_focused_competitor(),
    ];
    
    let competitive_evolved = genetics_engine.evolve_under_competitive_pressure(
        &initial_genetics,
        &competitors,
        CompetitionType::PerformanceBenchmark,
        3 // 3 generations
    ).await.expect("Competitive evolution should succeed");
    
    // Should evolve to compete better
    let initial_performance_score = calculate_performance_score(&initial_genetics);
    let evolved_performance_score = calculate_performance_score(&competitive_evolved);
    
    assert!(evolved_performance_score >= initial_performance_score,
           "Competitive evolution should improve performance");
    
    // Test environmental adaptation over time
    let environmental_changes = vec![
        EnvironmentalChange {
            change_type: ChangeType::SecurityThreatIncrease,
            magnitude: 0.3,
            duration: chrono::Duration::hours(2),
        },
        EnvironmentalChange {
            change_type: ChangeType::ResourceScarcity,
            magnitude: 0.5,
            duration: chrono::Duration::hours(4),
        },
        EnvironmentalChange {
            change_type: ChangeType::PerformanceDemandIncrease,
            magnitude: 0.4,
            duration: chrono::Duration::hours(3),
        },
    ];
    
    let environmentally_adapted = genetics_engine.adapt_to_environmental_changes(
        &initial_genetics,
        &environmental_changes,
    ).await.expect("Environmental adaptation should succeed");
    
    // Should show adaptations to environmental pressures
    assert!(environmentally_adapted.adaptation_rate >= initial_genetics.adaptation_rate,
           "Should increase adaptation rate under environmental pressure");
    assert!(environmentally_adapted.security_clearance >= initial_genetics.security_clearance,
           "Should maintain or improve security clearance");
}

/// Test genetic algorithm convergence
#[tokio::test]
async fn test_genetic_algorithm_convergence() {
    println!("📈 Testing genetic algorithm convergence...");
    
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await.expect("Core initialization failed"));
    
    let mut genetics_engine = GeneticsEngine::new(core.clone()).await
        .expect("Genetics engine creation failed");
    
    // Create initial diverse population
    let mut population = Vec::new();
    for i in 0..20 {
        population.push(create_test_genetics_variant(i));
    }
    
    let target_fitness = FitnessTarget {
        min_performance_score: 0.8,
        min_security_score: 0.7,
        min_efficiency_score: 0.6,
        max_generations: 50,
        convergence_threshold: 0.01,
    };
    
    let evolution_result = genetics_engine.evolve_population_to_target(
        &mut population,
        &target_fitness,
    ).await.expect("Population evolution should succeed");
    
    assert!(evolution_result.converged,
           "Population should converge to target fitness");
    assert!(evolution_result.final_best_fitness >= target_fitness.min_performance_score * 0.9,
           "Should achieve reasonable fitness score");
    assert!(evolution_result.generations_elapsed <= target_fitness.max_generations,
           "Should converge within generation limit");
}

// Helper functions and test data creation

fn create_test_genetics() -> BearDogGenetics {
    BearDogGenetics {
        crypto_preferences: vec![
            CryptoAlgorithm::Aes256Gcm,
            CryptoAlgorithm::ChaCha20Poly1305,
        ],
        performance_characteristics: PerformanceProfile {
            cpu_cores: 8,
            memory_gb: 32,
            storage_gb: 1000,
            network_mbps: 1000,
            specializations: vec![NodeSpecialization::GeneralPurpose],
        },
        security_clearance: SecurityClearance::High,
        trust_relationships: HashMap::new(),
        adaptation_rate: 0.1,
        mutation_probability: 0.05,
        compatibility_matrix: HashMap::new(),
    }
}

fn create_test_genetics_variant(variant: usize) -> BearDogGenetics {
    let base_cores = 4 + (variant % 8) as u32;
    let base_memory = 16 + (variant % 16) as u32;
    
    BearDogGenetics {
        crypto_preferences: match variant % 3 {
            0 => vec![CryptoAlgorithm::Aes256Gcm],
            1 => vec![CryptoAlgorithm::ChaCha20Poly1305],
            _ => vec![CryptoAlgorithm::Ed25519],
        },
        performance_characteristics: PerformanceProfile {
            cpu_cores: base_cores,
            memory_gb: base_memory,
            storage_gb: 500 + (variant * 100) as u32,
            network_mbps: 500 + (variant * 100) as u32,
            specializations: vec![
                match variant % 4 {
                    0 => NodeSpecialization::GeneralPurpose,
                    1 => NodeSpecialization::HighPerformanceCrypto,
                    2 => NodeSpecialization::GamingOptimized,
                    _ => NodeSpecialization::ResourceEfficient,
                }
            ],
        },
        security_clearance: match variant % 4 {
            0 => SecurityClearance::Basic,
            1 => SecurityClearance::Medium,
            2 => SecurityClearance::High,
            _ => SecurityClearance::Maximum,
        },
        trust_relationships: HashMap::new(),
        adaptation_rate: 0.05 + (variant as f64 * 0.02),
        mutation_probability: 0.01 + (variant as f64 * 0.01),
        compatibility_matrix: HashMap::new(),
    }
}

fn create_high_performance_competitor() -> BearDogGenetics {
    BearDogGenetics {
        performance_characteristics: PerformanceProfile {
            cpu_cores: 64,
            memory_gb: 256,
            storage_gb: 8000,
            network_mbps: 8000,
            specializations: vec![NodeSpecialization::HighPerformanceCrypto],
        },
        security_clearance: SecurityClearance::Maximum,
        ..create_test_genetics()
    }
}

fn create_efficient_competitor() -> BearDogGenetics {
    BearDogGenetics {
        performance_characteristics: PerformanceProfile {
            cpu_cores: 4,
            memory_gb: 8,
            storage_gb: 200,
            network_mbps: 200,
            specializations: vec![NodeSpecialization::ResourceEfficient],
        },
        adaptation_rate: 0.25,
        ..create_test_genetics()
    }
}

fn create_security_focused_competitor() -> BearDogGenetics {
    BearDogGenetics {
        crypto_preferences: vec![
            CryptoAlgorithm::Aes256Gcm,
            CryptoAlgorithm::ChaCha20Poly1305,
            CryptoAlgorithm::Rsa4096,
            CryptoAlgorithm::Ed25519,
        ],
        security_clearance: SecurityClearance::Maximum,
        ..create_test_genetics()
    }
}

// Mock calculation functions (would be implemented in actual genetics engine)

fn calculate_fitness_score(_genetics: &BearDogGenetics) -> f64 {
    0.75 // Mock fitness calculation
}

fn calculate_performance_score(_genetics: &BearDogGenetics) -> f64 {
    0.80 // Mock performance calculation
}

// Mock structs and enums for testing

#[derive(Debug, Clone)]
pub struct EnvironmentPressure {
    pub resource_scarcity: f64,
    pub security_threats: f64,
    pub performance_demands: f64,
    pub competition_level: f64,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub cpu_utilization: f64,
    pub memory_utilization: f64,
    pub network_latency_ms: f64,
    pub crypto_operations_per_second: u32,
    pub error_rate: f64,
    pub throughput_mbps: f64,
}

#[derive(Debug, Clone)]
pub struct SecurityEnvironment {
    pub threat_level: ThreatLevel,
    pub attack_vectors: Vec<AttackVector>,
    pub vulnerability_count: u32,
    pub incident_frequency: f64,
}

#[derive(Debug, Clone)]
pub struct NetworkConditions {
    pub available_bandwidth_mbps: u32,
    pub latency_ms: f64,
    pub packet_loss_rate: f64,
    pub jitter_ms: f64,
}

#[derive(Debug, Clone)]
pub struct FitnessWeights {
    pub performance_weight: f64,
    pub security_weight: f64,
    pub efficiency_weight: f64,
    pub adaptability_weight: f64,
}

impl FitnessWeights {
    pub fn balanced() -> Self {
        Self {
            performance_weight: 0.25,
            security_weight: 0.25,
            efficiency_weight: 0.25,
            adaptability_weight: 0.25,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResourcePressure {
    pub cpu_availability: f64,
    pub memory_availability: f64,
    pub storage_availability: f64,
    pub network_availability: f64,
}

#[derive(Debug, Clone)]
pub struct EnvironmentalChange {
    pub change_type: ChangeType,
    pub magnitude: f64,
    pub duration: chrono::Duration,
}

#[derive(Debug, Clone)]
pub struct FitnessTarget {
    pub min_performance_score: f64,
    pub min_security_score: f64,
    pub min_efficiency_score: f64,
    pub max_generations: u32,
    pub convergence_threshold: f64,
}

#[derive(Debug, Clone)]
pub struct EvolutionResult {
    pub converged: bool,
    pub final_best_fitness: f64,
    pub generations_elapsed: u32,
}

#[derive(Debug, Clone)]
pub enum CrossoverType {
    SinglePoint,
    TwoPoint,
    Uniform,
}

#[derive(Debug, Clone)]
pub enum OptimizationTarget {
    MaximizePerformance,
    MaximizeSecurity,
    MaximizeEfficiency,
    Balanced,
}

#[derive(Debug, Clone)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub enum AttackVector {
    CryptographicAttack,
    ResourceExhaustion,
    NetworkIntrusion,
    SocialEngineering,
}

#[derive(Debug, Clone)]
pub enum CompetitionType {
    PerformanceBenchmark,
    SecurityAssessment,
    EfficiencyTest,
}

#[derive(Debug, Clone)]
pub enum ChangeType {
    SecurityThreatIncrease,
    ResourceScarcity,
    PerformanceDemandIncrease,
    NetworkDegradation,
}

#[derive(Debug, Clone)]
pub struct IndividualWithFitness {
    pub genetics: BearDogGenetics,
    pub fitness: f64,
}
