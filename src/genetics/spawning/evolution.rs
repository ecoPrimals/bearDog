//! Genetic Evolution and Mutation
//!
//! This module handles directed evolution based on spawn purpose and applies
//! mutations to introduce genetic diversity.

use super::engine::GeneticSpawningEngine;
use super::super::types::*;
use crate::auth::BearDogGenetics;
use crate::tunnel::hsm::types::HsmOperation;
use crate::tunnel::hsm::{SecurityLevel, SecurityRequirements};
use crate::{BearDogError, BearDogResult};
use sha3::{Digest, Sha3_256};
use tracing::info;

/// Apply directed evolution based on spawn purpose
pub async fn apply_directed_evolution(
    engine: &GeneticSpawningEngine,
    mut genetics: BearDogGenetics,
    purpose: &crate::auth::SpawnPurpose,
) -> BearDogResult<BearDogGenetics> {
    info!("Applying directed evolution for purpose: {:?}", purpose);

    let security_reqs = SecurityRequirements::new(SecurityLevel::High);

    // Evolve genetics based on spawn purpose
    match purpose {
        crate::auth::SpawnPurpose::LoadBalancing => {
            // Enhance reliability and compatibility
            genetics
                .capabilities
                .push(crate::auth::NodeCapability::FaultTolerant);
            genetics
                .capabilities
                .push(crate::auth::NodeCapability::DistributedConsensus);
            genetics.security_traits.trust_threshold = 
                (genetics.security_traits.trust_threshold * 1.1).min(1.0);
        }
        crate::auth::SpawnPurpose::SpecializedTask(task_type) => {
            // Evolve based on task specialization
            match task_type {
                crate::auth::TaskType::ComputeTask => {
                    genetics
                        .capabilities
                        .push(crate::auth::NodeCapability::ComputeProvider);
                    genetics
                        .capabilities
                        .push(crate::auth::NodeCapability::HighThroughput);
                }
                crate::auth::TaskType::DataStorage => {
                    genetics
                        .capabilities
                        .push(crate::auth::NodeCapability::StorageProvider);
                    genetics
                        .capabilities
                        .push(crate::auth::NodeCapability::FaultTolerant);
                }
                crate::auth::TaskType::SecurityAnalysis => {
                    genetics
                        .capabilities
                        .push(crate::auth::NodeCapability::SecurityAnalysis);
                    genetics
                        .capabilities
                        .push(crate::auth::NodeCapability::ThreatDetection);
                }
                _ => {}
            }
        }
        crate::auth::SpawnPurpose::PerformanceOptimization => {
            // Enhance performance traits
            genetics
                .capabilities
                .push(crate::auth::NodeCapability::HighThroughput);
            genetics
                .capabilities
                .push(crate::auth::NodeCapability::LowLatency);
            genetics
                .capabilities
                .push(crate::auth::NodeCapability::EnergyEfficient);
        }
        crate::auth::SpawnPurpose::EcosystemIntegration(ecosystem) => {
            // Add ecosystem-specific capabilities
            match ecosystem.as_str() {
                "ToadStool" => genetics
                    .capabilities
                    .push(crate::auth::NodeCapability::ToadStoolCompute),
                "SongBird" => genetics
                    .capabilities
                    .push(crate::auth::NodeCapability::SongBirdDiscovery),
                "NestGate" => genetics
                    .capabilities
                    .push(crate::auth::NodeCapability::NestGateStorage),
                "Squirrel" => genetics
                    .capabilities
                    .push(crate::auth::NodeCapability::SquirrelPlugins),
                _ => {}
            }
        }
        _ => {
            // General evolution - slightly improve traits
            genetics.fitness_score = (genetics.fitness_score * 1.05).min(1.0);
        }
    }

    // Generate cryptographic proof of evolution
    let evolution_data = format!("{:?}-{}-{}", purpose, genetics.id, genetics.generation);
    let evolution_hash = Sha3_256::digest(evolution_data.as_bytes());

    // Create evolution signature using HSM
    let evolution_signature = engine
        .hsm_manager
        .sign_data(
            "genetics-evolution",
            &evolution_hash,
            &security_reqs,
            &HsmOperation::GeneticEvolution,
        )
        .await?;

    // Add evolution record to genetics
    genetics.mutations.push(crate::auth::CapabilityMutation {
        trigger: crate::auth::MutationTrigger::UserRequirement,
        mutation_type: format!("DirectedEvolution-{:?}", purpose),
        affected_capabilities: genetics.capabilities.clone(),
        fitness_impact: 0.05,
    });

    Ok(genetics)
}

/// Apply mutations to introduce genetic diversity
pub async fn apply_mutations(
    engine: &GeneticSpawningEngine,
    mut genetics: BearDogGenetics,
    mutation_rate: f64,
) -> BearDogResult<BearDogGenetics> {
    info!(
        "Applying mutations with HSM-backed randomness at rate: {}",
        mutation_rate
    );

    if mutation_rate <= 0.0 {
        return Ok(genetics);
    }

    let security_reqs = SecurityRequirements::new(SecurityLevel::Medium);
    let random_bytes = engine
        .hsm_manager
        .generate_random_bytes(64, &security_reqs)
        .await?;

    let mut rng_state = u64::from_le_bytes([
        random_bytes[0],
        random_bytes[1],
        random_bytes[2],
        random_bytes[3],
        random_bytes[4],
        random_bytes[5],
        random_bytes[6],
        random_bytes[7],
    ]);

    let mut mutations_applied = Vec::new();

    // Mutate security traits
    rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
    if ((rng_state >> 16) as f64 / 65536.0) < mutation_rate {
        let trait_mutation = (random_bytes[8] % 5) as usize;
        match trait_mutation {
            0 => {
                let delta = (random_bytes[9] as f64 / 255.0 - 0.5) * 0.1;
                genetics.security_traits.trust_threshold =
                    (genetics.security_traits.trust_threshold + delta).clamp(0.0, 1.0);
            }
            1 => {
                let delta = (random_bytes[10] as i16 - 128) / 25;
                genetics.security_traits.paranoia_level = (genetics.security_traits.paranoia_level
                    as i16
                    + delta)
                    .clamp(0, 10) as u8;
            }
            2 => {
                let delta = (random_bytes[11] as f64 / 255.0 - 0.5) * 0.1;
                genetics.security_traits.isolation_preference =
                    (genetics.security_traits.isolation_preference + delta).clamp(0.0, 1.0);
            }
            3 => {
                let delta = (random_bytes[12] as i32 - 128) / 2;
                genetics.security_traits.audit_frequency = (genetics.security_traits.audit_frequency
                    as i32
                    + delta)
                    .clamp(0, 100) as u32;
            }
            4 => {
                genetics.security_traits.consensus_requirement =
                    !genetics.security_traits.consensus_requirement;
            }
            _ => {}
        }
        mutations_applied.push("SecurityTraits".to_string());
    }

    // Mutate chromosomes
    for (i, chromosome) in genetics.crypto_chromosomes.iter_mut().enumerate() {
        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        if ((rng_state >> 16) as f64 / 65536.0) < mutation_rate {
            let mutation_type = (random_bytes[16 + i % 16] % 3) as usize;
            match mutation_type {
                0 => {
                    // Strength mutation
                    let delta = (random_bytes[32 + i % 16] as i32 - 128) / 4;
                    chromosome.strength_bits =
                        (chromosome.strength_bits as i32 + delta).max(128) as u32;
                }
                1 => {
                    // Performance mutation
                    let delta = (random_bytes[48 + i % 16] as f64 / 255.0 - 0.5) * 0.1;
                    chromosome.performance_factor =
                        (chromosome.performance_factor + delta).clamp(0.1, 1.0);
                }
                2 => {
                    // Compatibility mutation
                    let delta = (random_bytes[52 + i % 16] as f64 / 255.0 - 0.5) * 0.1;
                    chromosome.compatibility_score =
                        (chromosome.compatibility_score + delta).clamp(0.0, 1.0);
                }
                _ => {}
            }
            mutations_applied.push(format!("Chromosome-{}", i));
        }
    }

    // Capability mutations - rarely add new capabilities
    rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
    if ((rng_state >> 16) as f64 / 65536.0) < (mutation_rate * 0.1) {
        // 10% of mutation rate for capability changes
        let capability_mutation = random_bytes[56] % 3;
        match capability_mutation {
            0 => {
                // Add a new capability
                let new_capabilities = [
                    crate::auth::NodeCapability::DistributedConsensus,
                    crate::auth::NodeCapability::SelfHealing,
                    crate::auth::NodeCapability::FaultTolerant,
                ];
                let new_cap = new_capabilities[random_bytes[57] as usize % new_capabilities.len()].clone();
                if !genetics.capabilities.contains(&new_cap) {
                    genetics.capabilities.push(new_cap.clone());
                    mutations_applied.push(format!("AddCapability-{:?}", new_cap));
                }
            }
            1 => {
                // Remove a capability (rarely)
                if !genetics.capabilities.is_empty() {
                    let remove_idx = random_bytes[58] as usize % genetics.capabilities.len();
                    let removed_cap = genetics.capabilities.remove(remove_idx);
                    mutations_applied.push(format!("RemoveCapability-{:?}", removed_cap));
                }
            }
            2 => {
                // Fitness score mutation
                let delta = (random_bytes[59] as f64 / 255.0 - 0.5) * 0.1;
                genetics.fitness_score = (genetics.fitness_score + delta).clamp(0.0, 1.0);
                mutations_applied.push("FitnessScore".to_string());
            }
            _ => {}
        }
    }

    // Record mutation in genetics
    if !mutations_applied.is_empty() {
        genetics.mutations.push(crate::auth::CapabilityMutation {
            trigger: crate::auth::MutationTrigger::EnvironmentalStress,
            mutation_type: format!("RandomMutation-{}", mutations_applied.join(",")),
            affected_capabilities: genetics.capabilities.clone(),
            fitness_impact: -0.01, // Small negative impact for random mutations
        });
    }

    Ok(genetics)
} 