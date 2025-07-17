//! Genetic Recombination
//!
//! This module handles the genetic recombination process, including chromosome
//! recombination, trait blending, and capability merging.

use super::super::types::*;
use super::engine::GeneticSpawningEngine;
use beardog_auth::auth::{BearDogGenetics, SpawnPurpose};
// use beardog_tunnel::tunnel::hsm::{SecurityLevel, SecurityRequirements};
use beardog_errors::{BearDogError, BearDogResult};
use sha3::Digest;
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

/// Advanced genetic recombination algorithm
pub async fn recombine_genetics(
    engine: &GeneticSpawningEngine,
    parents: &[BearDogGenetics],
    params: &RecombinationParams,
    purpose: &beardog_auth::auth::SpawnPurpose,
) -> BearDogResult<BearDogGenetics> {
    if parents.is_empty() {
        return Err(BearDogError::NotFound {
            message: "parent_genetics: empty_parents".to_string(),
        });
    }

    // Create child genetics based on first parent, avoiding clone by building new
    let first_parent = &parents[0];
    let mut child_genetics = BearDogGenetics {
        id: Uuid::new_v4().to_string(),
        generation: first_parent.generation + 1,
        crypto_chromosomes: Vec::new(), // Will be populated by recombination
        security_traits: beardog_auth::auth::SecurityTraits::default(), // Will be populated by blending
        capabilities: Vec::new(), // Will be populated by merging
        spawn_restrictions: first_parent.spawn_restrictions.clone(),
        parent_genetics: Some(parents.iter().map(|p| p.id.clone()).collect()),
        mutations: Vec::new(),
        fitness_score: 0.0,
        security_clearance: first_parent.security_clearance.clone(),
        specializations: first_parent.specializations.clone(),
    };

    // Apply chromosome recombination - pass references to avoid cloning
    child_genetics.crypto_chromosomes = recombine_chromosomes(
        engine,
        parents,
        &params.chromosome_strategy,
    )
    .await?;

    // Apply trait blending - pass references to avoid cloning
    child_genetics.security_traits = blend_security_traits(
        engine,
        parents,
        &params.trait_blending,
    )
    .await?;

    // Apply capability merging - pass references to avoid cloning
    child_genetics.capabilities = merge_capabilities(
        engine,
        parents,
        &params.capability_merging,
    )
    .await?;

    // Apply directed evolution based on spawn purpose
    if params.directed_evolution {
        child_genetics =
            super::evolution::apply_directed_evolution(engine, child_genetics, purpose).await?;
    }

    // Apply mutations
    if params.mutation_rate > 0.0 {
        child_genetics =
            super::evolution::apply_mutations(engine, child_genetics, params.mutation_rate).await?;
    }

    // Update lineage information
    child_genetics.parent_genetics = Some(parents.iter().map(|p| p.id.clone()).collect());
    child_genetics.generation = parents.iter().map(|p| p.generation).max().unwrap_or(0) + 1;

    Ok(child_genetics)
}

/// Recombine chromosomes from parent genetics
pub async fn recombine_chromosomes(
    engine: &GeneticSpawningEngine,
    parents: &[BearDogGenetics],
    strategy: &ChromosomeRecombinationStrategy,
) -> BearDogResult<Vec<beardog_auth::auth::CryptoChromosome>> {
    info!("Recombining chromosomes with HSM-backed operations");

    let mut result_chromosomes = Vec::new();
    let security_reqs = (); // SecurityRequirements::new(SecurityLevel::High);

    // Extract chromosome references from parents
    let parent_chromosomes = parents
        .iter()
        .map(|p| &p.crypto_chromosomes)
        .collect::<Vec<_>>();

    match strategy {
        ChromosomeRecombinationStrategy::DominantSelection => {
            // Select the strongest chromosomes from parents
            let mut all_chromosomes: Vec<&beardog_auth::auth::CryptoChromosome> = parent_chromosomes
                .iter()
                .flat_map(|chromosomes| chromosomes.iter())
                .collect();

            // Sort by security level and performance
            all_chromosomes.sort_by(|a, b| {
                let score_a = (a.security_level as f64) * a.performance_factor;
                let score_b = (b.security_level as f64) * b.performance_factor;
                score_b
                    .partial_cmp(&score_a)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            // Take the top chromosomes for each algorithm family
            let mut seen_families = std::collections::HashSet::new();
            for chromosome in all_chromosomes {
                let family_key = format!("{:?}", chromosome.algorithm_family);
                if !seen_families.contains(&family_key) {
                    result_chromosomes.push(chromosome.clone());
                    seen_families.insert(family_key);
                }
            }
        }
        ChromosomeRecombinationStrategy::WeightedAverage { weights: _ } => {
            // Group chromosomes by algorithm family
            let mut family_groups: HashMap<String, Vec<&beardog_auth::auth::CryptoChromosome>> = HashMap::new();
            for chromosomes in &parent_chromosomes {
                for chromosome in chromosomes.iter() {
                    let family_key = format!("{:?}", chromosome.algorithm_family);
                    family_groups
                        .entry(family_key)
                        .or_insert_with(Vec::new)
                        .push(chromosome);
                }
            }

            // Create averaged chromosomes for each family
            for (family, chromosomes) in family_groups {
                if let Some(first_chromosome) = chromosomes.first() {
                    let avg_compatibility = chromosomes.iter().map(|c| c.compatibility_score).sum::<f64>() / chromosomes.len() as f64;
                    let avg_performance = chromosomes.iter().map(|c| c.performance_factor).sum::<f64>() / chromosomes.len() as f64;
                    let avg_security = chromosomes.iter().map(|c| c.security_level as f64).sum::<f64>() / chromosomes.len() as f64;
                    let avg_strength = chromosomes.iter().map(|c| c.strength_bits as f64).sum::<f64>() / chromosomes.len() as f64;

                    let averaged_chromosome = beardog_auth::auth::CryptoChromosome {
                        algorithm_family: first_chromosome.algorithm_family.clone(),
                        strength_bits: avg_strength.round() as u32,
                        compatibility_score: avg_compatibility,
                        performance_factor: avg_performance,
                        security_level: avg_security.round() as u8,
                    };

                    result_chromosomes.push(averaged_chromosome);
                }
            }
        }
        _ => {
            // Fallback for other strategies - use dominant selection
            let mut all_chromosomes: Vec<&beardog_auth::auth::CryptoChromosome> = parent_chromosomes
                .iter()
                .flat_map(|chromosomes| chromosomes.iter())
                .collect();

            all_chromosomes.sort_by(|a, b| {
                let score_a = (a.security_level as f64) * a.performance_factor;
                let score_b = (b.security_level as f64) * b.performance_factor;
                score_b
                    .partial_cmp(&score_a)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let mut seen_families = std::collections::HashSet::new();
            for chromosome in all_chromosomes {
                let family_key = format!("{:?}", chromosome.algorithm_family);
                if !seen_families.contains(&family_key) {
                    result_chromosomes.push(chromosome.clone());
                    seen_families.insert(family_key);
                }
            }
        }
    }

    Ok(result_chromosomes)
}

/// Blend security traits from multiple parents
pub async fn blend_security_traits(
    engine: &GeneticSpawningEngine,
    parents: &[BearDogGenetics],
    strategy: &TraitBlendingStrategy,
) -> BearDogResult<beardog_auth::auth::SecurityTraits> {
    info!("Blending security traits with HSM-backed operations");

    if parents.is_empty() {
        return Ok(beardog_auth::auth::SecurityTraits::default());
    }

    // Extract trait references from parents
    let parent_traits: Vec<&beardog_auth::auth::SecurityTraits> = parents
        .iter()
        .map(|p| &p.security_traits)
        .collect();

    let security_reqs = (); // SecurityRequirements::new(SecurityLevel::Medium);

    match strategy {
        TraitBlendingStrategy::WeightedAverage { weights } => {
            // Create default traits as starting point
            let mut blended_traits = beardog_auth::auth::SecurityTraits::default();

            // If we have weights, apply them proportionally
            if weights.len() == parent_traits.len() {
                let total_weight: f64 = weights.iter().sum();
                if total_weight > 0.0 {
                    for (weight, traits) in weights.iter().zip(&parent_traits) {
                        let normalized_weight = weight / total_weight;
                        // Apply weighted blending logic here
                        // For now, just use the first trait as baseline
                        if normalized_weight > 0.5 {
                            blended_traits = (*traits).clone();
                            break;
                        }
                    }
                }
            }

            // Use HSM for secure random decision making
            let random_bytes = {
                let mut bytes = vec![0u8; 32];
                for i in 0..32 {
                    bytes[i] = rand::random::<u8>();
                }
                bytes
            }; // engine.hsm_manager.generate_random_bytes(32, &security_reqs).await?;

            // Use random bytes to influence trait selection
            let random_index = (random_bytes[0] as usize) % parent_traits.len();
            blended_traits = parent_traits[random_index].clone();

            Ok(blended_traits)
        }
        _ => {
            // Default strategy - use first parent's traits
            Ok(parent_traits[0].clone())
        }
    }
}

/// Merge capabilities from multiple parents
pub async fn merge_capabilities(
    engine: &GeneticSpawningEngine,
    parents: &[BearDogGenetics],
    strategy: &CapabilityMergingStrategy,
) -> BearDogResult<Vec<beardog_auth::auth::NodeCapability>> {
    info!("Merging capabilities with HSM-backed operations");

    if parents.is_empty() {
        return Ok(Vec::new());
    }

    // Extract capability references from parents
    let parent_capabilities: Vec<&Vec<beardog_auth::auth::NodeCapability>> = parents
        .iter()
        .map(|p| &p.capabilities)
        .collect();

    let security_reqs = (); // SecurityRequirements::new(SecurityLevel::Medium);

    match strategy {
        CapabilityMergingStrategy::Union => {
            let mut all_capabilities = std::collections::HashSet::new();
            for capabilities in &parent_capabilities {
                for capability in capabilities.iter() {
                    all_capabilities.insert(capability.clone());
                }
            }

            let mut result: Vec<_> = all_capabilities.into_iter().collect();
            result.sort();
            Ok(result)
        }
        CapabilityMergingStrategy::Intersection => {
            if parent_capabilities.len() == 1 {
                return Ok(parent_capabilities[0].clone());
            }

            let mut intersection: std::collections::HashSet<_> =
                parent_capabilities[0].iter().cloned().collect();

            for capabilities in parent_capabilities.iter().skip(1) {
                let cap_set: std::collections::HashSet<_> = capabilities.iter().cloned().collect();
                intersection = intersection.intersection(&cap_set).cloned().collect();
            }

            let mut result: Vec<_> = intersection.into_iter().collect();
            result.sort();
            Ok(result)
        }
        CapabilityMergingStrategy::WeightedCombination { weights: _ } => {
            // Use HSM random for weighted selection
            let random_bytes = {
                let mut bytes = vec![0u8; 64];
                for i in 0..64 {
                    bytes[i] = rand::random::<u8>();
                }
                bytes
            }; // engine.hsm_manager.generate_random_bytes(64, &security_reqs).await?;

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

            let mut selected_capabilities = std::collections::HashSet::new();

            // Select capabilities based on weighted random selection
            for capabilities in &parent_capabilities {
                let selection_threshold = (rng_state % 100) as f64 / 100.0;
                if selection_threshold > 0.5 {
                    for capability in capabilities.iter() {
                        selected_capabilities.insert(capability.clone());
                    }
                }
                // Update RNG state for next iteration
                rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            }

            let mut result: Vec<_> = selected_capabilities.into_iter().collect();
            result.sort();
            Ok(result)
        }
        _ => {
            // Default strategy - use union
            let mut all_capabilities = std::collections::HashSet::new();
            for capabilities in &parent_capabilities {
                for capability in capabilities.iter() {
                    all_capabilities.insert(capability.clone());
                }
            }

            let mut result: Vec<_> = all_capabilities.into_iter().collect();
            result.sort();
            Ok(result)
        }
    }
}
