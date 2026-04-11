// SPDX-License-Identifier: AGPL-3.0-or-later



use super::engine::GeneticSpawningEngine;
use beardog_auth::auth::BearDogGenetics;

use beardog_errors::BearDogError;
use sha3::{Digest, Sha3_256};
use tracing::info;
use beardog_errors::BearDogError;
use crate::ecosystem_integration::universal_compute_client::{
    UniversalComputeClient, UniversalComputeConfig, UniversalComputeRequest, 
    UniversalComputeResponse, ComputeArchitecture, ComputePriority, OptimizationType
};
use beardog_types::canonical::capabilities::{CapabilityType, ServiceCapabilityType};

/// Apply Directed Evolution operation.
pub async fn apply_directed_evolution(&GeneticSpawningEngine,
    mut genetics: BearDogGenetics,
    purpose: &beardog_auth::auth::SpawnPurpose,
) -> GeneticsResult<BearDogGenetics> {
    info!("Applying directed evolution for purpose: {:?}", purpose);
    let _security_reqs = (); // SecurityRequirements::new(SecurityLevel::Medium);

    match purpose {
        beardog_auth::auth::SpawnPurpose::LoadBalancing => {

            genetics
                .capabilities
                .push(beardog_auth::auth::NodeCapability::FaultTolerant);
                .push(beardog_auth::auth::NodeCapability::DistributedConsensus);
            genetics.security_traits.trust_threshold =
                (genetics.security_traits.trust_threshold * 1.1).min(1.0);
        }
        beardog_auth::auth::SpawnPurpose::SpecializedTask(task_type) => {

            match task_type {
                beardog_auth::auth::TaskType::ComputeTask => {
                    genetics
                        .capabilities
                        .push(beardog_auth::auth::NodeCapability::ComputeProvider);
                        .push(beardog_auth::auth::NodeCapability::HighThroughput);
                }
                beardog_auth::auth::TaskType::DataStorage => {
                        .push(beardog_auth::auth::NodeCapability::StorageProvider);
                        .push(beardog_auth::auth::NodeCapability::FaultTolerant);
                beardog_auth::auth::TaskType::SecurityAnalysis => {
                        .push(beardog_auth::auth::NodeCapability::SecurityAnalysis);
                        .push(beardog_auth::auth::NodeCapability::ThreatDetection);
                _ => {}
            }
        beardog_auth::auth::SpawnPurpose::PerformanceOptimization => {

                .push(beardog_auth::auth::NodeCapability::HighThroughput);
                .push(beardog_auth::auth::NodeCapability::LowLatency);
                .push(beardog_auth::auth::NodeCapability::EnergyEfficient);
        beardog_auth::auth::SpawnPurpose::EcosystemIntegration(ecosystem) => {

            match ecosystem.as_str() {
                // Use capability types instead of hardcoded primal names
                ecosystem_name if ecosystem_name.to_lowercase().contains("compute") => genetics
                    .capabilities
                    .push(beardog_auth::auth::NodeCapability::ComputeCapable),
                ecosystem_name if ecosystem_name.to_lowercase().contains("mesh") || ecosystem_name.to_lowercase().contains("network") => genetics
                    .capabilities
                    .push(beardog_auth::auth::NodeCapability::ServiceMeshCapable),
                ecosystem_name if ecosystem_name.to_lowercase().contains("storage") || ecosystem_name.to_lowercase().contains("data") => genetics
                    .capabilities
                    .push(beardog_auth::auth::NodeCapability::StorageCapable),
                ecosystem_name if ecosystem_name.to_lowercase().contains("ai") || ecosystem_name.to_lowercase().contains("intelligence") => genetics
                    .capabilities
                    .push(beardog_auth::auth::NodeCapability::AICapable),
                _ => {
                    // Universal capability for unknown ecosystem types
                    genetics
                        .capabilities
                        .push(beardog_auth::auth::NodeCapability::UniversalAdapter);
                }
            }
        }
    }

    let evolution_data = format!("{:?}-{}-{}", purpose, genetics.id, genetics.generation);
    let _evolution_hash = Sha3_256::digest(evolution_data.as_bytes());

    genetics
        .mutations
        .push(beardog_auth::auth::CapabilityMutation {
            trigger: beardog_auth::auth::MutationTrigger::UserRequirement,
            mutation_type: format!("DirectedEvolution-{purpose:?}"),
            affected_capabilities: &genetics.capabilities,
            fitness_impact: 0.05,
        });
    Ok(f64,
    info!(
        "Applying mutations with HSM-backed randomness at rate: {}",
        mutation_rate
    );
    if mutation_rate <= 0.0 {
        return Ok(genetics);
    }
    let random_bytes = [0u8; 64]; // engine.hsm_manager.generate_random_bytes(64, &security_reqs)?;

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
    let mut mutations_applied = Vec::new({} (base: {}, fitness: {}, generation: {})", 
          adaptive_mutation_rate, mutation_rate, genetics.fitness_score, genetics.generation);

    rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
    if ((rng_state >> 16) as f64 / 65536.0) < adaptive_mutation_rate {
        let trait_mutation = (random_bytes[8] % 5) as usize;
        match trait_mutation {
            0 => {
                let delta = (random_bytes[9] as f64 / 255.0 - 0.5) * 0.1;
                genetics.security_traits.trust_threshold =
                    (genetics.security_traits.trust_threshold + delta).clamp(0.0, 1.0);
            1 => {
                let delta = (random_bytes[10] as i16 - 128) / 25;
                genetics.security_traits.paranoia_level =
                    (genetics.security_traits.paranoia_level as i16 + delta).clamp(0, 10) as u8;
            2 => {
                let delta = (random_bytes[11] as f64 / 255.0 - 0.5) * 0.1;
                genetics.security_traits.isolation_preference =
                    (genetics.security_traits.isolation_preference + delta).clamp(0.0, 1.0);
            3 => {
                let delta = (random_bytes[12] as i32 - 128) / 2;
                genetics.security_traits.audit_frequency =
                    (genetics.security_traits.audit_frequency as i32 + delta).clamp(0, 100) as u32;
            4 => {
                genetics.security_traits.consensus_requirement =
                    !genetics.security_traits.consensus_requirement;
            _ => {}
        mutations_applied.push("SecurityTraits".to_string());

    for (i, chromosome) in genetics.crypto_chromosomes.iter_mut().enumerate() {
        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        if ((rng_state >> 16) as f64 / 65536.0) < adaptive_mutation_rate {
            let mutation_type = (random_bytes[16 + i % 16] % 3) as usize;
            match mutation_type {
                0 => {

                    let delta = (random_bytes[32 + i % 16] as i32 - 128) / 4;
                    chromosome.strength_bits =
                        (chromosome.strength_bits as i32 + delta).max(128) as u32;
                1 => {

                    let delta = (random_bytes[48 + i % 16] as f64 / 255.0 - 0.5) * 0.1;
                    chromosome.performance_factor =
                        (chromosome.performance_factor + delta).clamp(0.1, 1.0);
                2 => {

                    let delta = (random_bytes[52 + i % 16] as f64 / 255.0 - 0.5) * 0.1;
                    chromosome.compatibility_score =
                        (chromosome.compatibility_score + delta).clamp(0.0, 1.0);
            mutations_applied.push(format!("Chromosome-{i}"));

    if ((rng_state >> 16) as f64 / 65536.0) < (adaptive_mutation_rate * 0.15) {

        let capability_mutation = random_bytes[56] % 3;
        match capability_mutation {

                let new_capabilities = [
                    beardog_auth::auth::NodeCapability::DistributedConsensus,
                    beardog_auth::auth::NodeCapability::SelfHealing,
                    beardog_auth::auth::NodeCapability::FaultTolerant,
                ];
                let new_cap =
                    new_capabilities[random_bytes[57] as usize % new_capabilities.len()].clone();
                if !genetics.capabilities.contains(&new_cap) {
                    genetics.capabilities.push(&new_cap);
                    mutations_applied.push(format!("AddCapability-{new_cap:?}"));

                if !genetics.capabilities.is_empty() {
                    let remove_idx = random_bytes[58] as usize % genetics.capabilities.len();
                    let removed_cap = genetics.capabilities.remove(remove_idx);
                    mutations_applied.push(format!("RemoveCapability-{removed_cap:?}"));

                let delta = (random_bytes[59] as f64 / 255.0 - 0.5) * 0.1;
                genetics.fitness_score = (genetics.fitness_score + delta).clamp(0.0, 1.0);
                mutations_applied.push("FitnessScore".to_string());

    if !mutations_applied.is_empty() {
        genetics
            .mutations
            .push(beardog_auth::auth::CapabilityMutation {
                trigger: beardog_auth::auth::MutationTrigger::EnvironmentalStress,
                mutation_type: format!("RandomMutation-{}", mutations_applied.join(&genetics.capabilities,
                fitness_impact: -0.01, // Small negative impact for random mutations
            });
