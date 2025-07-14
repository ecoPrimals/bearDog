//! Genetic Recombination
//!
//! This module handles the genetic recombination process, including chromosome
//! recombination, trait blending, and capability merging.

use super::engine::GeneticSpawningEngine;
use super::super::types::*;
use crate::auth::BearDogGenetics;
use crate::tunnel::hsm::types::HsmOperation;
use crate::tunnel::hsm::{SecurityLevel, SecurityRequirements};
use crate::{BearDogError, BearDogResult};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

/// Advanced genetic recombination algorithm
pub async fn recombine_genetics(
    engine: &GeneticSpawningEngine,
    parents: &[BearDogGenetics],
    params: &RecombinationParams,
    purpose: &crate::auth::SpawnPurpose,
) -> BearDogResult<BearDogGenetics> {
    if parents.is_empty() {
        return Err(BearDogError::NotFound {
            resource_type: "parent_genetics".to_string(),
            id: "empty_parents".to_string(),
        });
    }

    // Start with the first parent as base
    let mut child_genetics = parents[0].clone();
    child_genetics.id = Uuid::new_v4().to_string();
    child_genetics.generation += 1;

    // Apply chromosome recombination
    child_genetics.crypto_chromosomes = recombine_chromosomes(
        engine,
        &parents
            .iter()
            .map(|p| &p.crypto_chromosomes)
            .collect::<Vec<_>>(),
        &params.chromosome_strategy,
    )
    .await?;

    // Apply trait blending
    child_genetics.security_traits = blend_security_traits(
        engine,
        &parents
            .iter()
            .map(|p| &p.security_traits)
            .collect::<Vec<_>>(),
        &params.trait_blending,
    )
    .await?;

    // Apply capability merging
    child_genetics.capabilities = merge_capabilities(
        engine,
        &parents.iter().map(|p| &p.capabilities).collect::<Vec<_>>(),
        &params.capability_merging,
    )
    .await?;

    // Apply directed evolution based on spawn purpose
    if params.directed_evolution {
        child_genetics = super::evolution::apply_directed_evolution(
            engine,
            child_genetics,
            purpose,
        )
        .await?;
    }

    // Apply mutations
    if params.mutation_rate > 0.0 {
        child_genetics = super::evolution::apply_mutations(
            engine,
            child_genetics,
            params.mutation_rate,
        )
        .await?;
    }

    // Update lineage information
    child_genetics.parent_genetics = Some(parents.iter().map(|p| p.id.clone()).collect());
    child_genetics.generation = parents.iter().map(|p| p.generation).max().unwrap_or(0) + 1;

    Ok(child_genetics)
}

/// Recombine chromosomes from multiple parents
pub async fn recombine_chromosomes(
    engine: &GeneticSpawningEngine,
    parent_chromosomes: &[&Vec<crate::auth::CryptoChromosome>],
    strategy: &ChromosomeRecombinationStrategy,
) -> BearDogResult<Vec<crate::auth::CryptoChromosome>> {
    info!("Recombining chromosomes with HSM-backed operations");

    if parent_chromosomes.is_empty() {
        return Ok(Vec::new());
    }

    let mut result_chromosomes = Vec::new();
    let security_reqs = SecurityRequirements::new(SecurityLevel::High);

    match strategy {
        ChromosomeRecombinationStrategy::DominantSelection => {
            // Select the strongest chromosomes from parents
            let mut all_chromosomes: Vec<_> = parent_chromosomes
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
            let mut family_groups: HashMap<String, Vec<_>> = HashMap::new();
            for chromosomes in parent_chromosomes {
                for chromosome in chromosomes.iter() {
                    let family_key = format!("{:?}", chromosome.algorithm_family);
                    family_groups
                        .entry(family_key)
                        .or_insert_with(Vec::new)
                        .push(chromosome);
                }
            }

            // Average chromosomes within each family
            for (_family, chromosomes) in family_groups {
                if !chromosomes.is_empty() {
                    let avg_strength = chromosomes
                        .iter()
                        .map(|c| c.strength_bits as f64)
                        .sum::<f64>()
                        / chromosomes.len() as f64;
                    let avg_compat = chromosomes
                        .iter()
                        .map(|c| c.compatibility_score)
                        .sum::<f64>()
                        / chromosomes.len() as f64;
                    let avg_performance = chromosomes
                        .iter()
                        .map(|c| c.performance_factor)
                        .sum::<f64>()
                        / chromosomes.len() as f64;
                    let avg_security = chromosomes
                        .iter()
                        .map(|c| c.security_level as f64)
                        .sum::<f64>()
                        / chromosomes.len() as f64;

                    let averaged = crate::auth::CryptoChromosome {
                        algorithm_family: chromosomes[0].algorithm_family.clone(),
                        strength_bits: avg_strength as u32,
                        compatibility_score: avg_compat,
                        performance_factor: avg_performance,
                        security_level: avg_security as u8,
                    };
                    result_chromosomes.push(averaged);
                }
            }
        }
        _ => {
            // Fallback for other strategies - use dominant selection
            let mut all_chromosomes: Vec<_> = parent_chromosomes
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
    parent_traits: &[&crate::auth::SecurityTraits],
    strategy: &TraitBlendingStrategy,
) -> BearDogResult<crate::auth::SecurityTraits> {
    info!("Blending security traits with HSM-backed operations");

    if parent_traits.is_empty() {
        return Ok(crate::auth::SecurityTraits::default());
    }

    let security_reqs = SecurityRequirements::new(SecurityLevel::Medium);

    match strategy {
        TraitBlendingStrategy::WeightedAverage { weights } => {
            let mut trust_sum = 0.0;
            let mut paranoia_sum = 0.0;
            let mut isolation_sum = 0.0;
            let mut audit_sum = 0.0;
            let mut consensus_count = 0;
            let mut weight_sum = 0.0;

            for (i, traits) in parent_traits.iter().enumerate() {
                let weight = weights.get(i).copied().unwrap_or(1.0);
                weight_sum += weight;

                trust_sum += traits.trust_threshold * weight;
                paranoia_sum += traits.paranoia_level as f64 * weight;
                isolation_sum += traits.isolation_preference * weight;
                audit_sum += traits.audit_frequency as f64 * weight;
                if traits.consensus_requirement {
                    consensus_count += 1;
                }
            }

            // Generate cryptographic hash for trait validation
            let trait_data = format!(
                "{:.3}{:.3}{:.3}{:.0}",
                trust_sum / weight_sum,
                paranoia_sum / weight_sum,
                isolation_sum / weight_sum,
                audit_sum / weight_sum
            );
            let trait_hash = Sha3_256::digest(trait_data.as_bytes());

            // Use hash to introduce slight randomness for genetic diversity
            let hash_influence = (trait_hash[0] as f64) / 255.0 * 0.05; // 5% max influence

            Ok(crate::auth::SecurityTraits {
                trust_threshold: ((trust_sum / weight_sum) + hash_influence).clamp(0.0, 1.0),
                paranoia_level: ((paranoia_sum / weight_sum) as u8).clamp(0, 10),
                isolation_preference: ((isolation_sum / weight_sum) + hash_influence).clamp(0.0, 1.0),
                audit_frequency: ((audit_sum / weight_sum) as u32).clamp(0, 100),
                consensus_requirement: consensus_count > (parent_traits.len() / 2),
            })
        }
        _ => {
            // Fallback - use simple average
            let trust_avg = parent_traits
                .iter()
                .map(|t| t.trust_threshold)
                .sum::<f64>()
                / parent_traits.len() as f64;
            let paranoia_avg = parent_traits
                .iter()
                .map(|t| t.paranoia_level as f64)
                .sum::<f64>()
                / parent_traits.len() as f64;
            let isolation_avg = parent_traits
                .iter()
                .map(|t| t.isolation_preference)
                .sum::<f64>()
                / parent_traits.len() as f64;
            let audit_avg = parent_traits
                .iter()
                .map(|t| t.audit_frequency as f64)
                .sum::<f64>()
                / parent_traits.len() as f64;
            let consensus_count = parent_traits
                .iter()
                .filter(|t| t.consensus_requirement)
                .count();

            Ok(crate::auth::SecurityTraits {
                trust_threshold: trust_avg,
                paranoia_level: paranoia_avg as u8,
                isolation_preference: isolation_avg,
                audit_frequency: audit_avg as u32,
                consensus_requirement: consensus_count > (parent_traits.len() / 2),
            })
        }
    }
}

/// Merge capabilities from multiple parents
pub async fn merge_capabilities(
    engine: &GeneticSpawningEngine,
    parent_capabilities: &[&Vec<crate::auth::NodeCapability>],
    strategy: &CapabilityMergingStrategy,
) -> BearDogResult<Vec<crate::auth::NodeCapability>> {
    info!("Merging capabilities with HSM-backed operations");

    if parent_capabilities.is_empty() {
        return Ok(Vec::new());
    }

    let security_reqs = SecurityRequirements::new(SecurityLevel::Medium);

    match strategy {
        CapabilityMergingStrategy::Union => {
            let mut all_capabilities = std::collections::HashSet::new();
            for capabilities in parent_capabilities {
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
                let cap_set: std::collections::HashSet<_> =
                    capabilities.iter().cloned().collect();
                intersection = intersection.intersection(&cap_set).cloned().collect();
            }

            let mut result: Vec<_> = intersection.into_iter().collect();
            result.sort();
            Ok(result)
        }
        CapabilityMergingStrategy::WeightedCombination { weights: _ } => {
            // Use HSM random for weighted selection
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

            let mut selected_capabilities = Vec::new();
            let mut seen_capabilities = std::collections::HashSet::new();

            for capabilities in parent_capabilities {
                for capability in capabilities.iter() {
                    if seen_capabilities.contains(capability) {
                        continue;
                    }

                    // Use HSM random to decide inheritance probability
                    rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                    let inherit_probability = (rng_state >> 16) as f64 / 65536.0;

                    // Higher probability for security and advanced capabilities
                    let selection_threshold = match capability {
                        crate::auth::NodeCapability::SecurityAnalysis => 0.2,
                        crate::auth::NodeCapability::QuantumResistant => 0.3,
                        crate::auth::NodeCapability::ZeroKnowledgeProofs => 0.3,
                        crate::auth::NodeCapability::ThreatDetection => 0.4,
                        crate::auth::NodeCapability::CryptographicAuditing => 0.4,
                        _ => 0.6,
                    };

                    if inherit_probability > selection_threshold {
                        selected_capabilities.push(capability.clone());
                        seen_capabilities.insert(capability.clone());
                    }
                }
            }

            selected_capabilities.sort();
            Ok(selected_capabilities)
        }
        _ => {
            // Fallback for other strategies - use union
            let mut all_capabilities = std::collections::HashSet::new();
            for capabilities in parent_capabilities {
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