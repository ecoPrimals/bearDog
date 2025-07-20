//! Genetic Recombination - Simplified for Production
//!
//! This module handles genetic recombination operations with streamlined
//! algorithms suitable for production deployment.

use beardog_auth::auth::{BearDogGenetics, CryptoChromosome, NodeCapability, SpawnPurpose};
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use tracing::{debug, info};

// TODO: Re-implement recombination strategies after refactor
// use super::{CapabilityMergingStrategy, ChromosomeRecombinationStrategy, RecombinationParams, TraitBlendingStrategy};
use crate::genetics::spawning::engine::GeneticSpawningEngine;
// use beardog_tunnel::tunnel::hsm::{SecurityLevel, SecurityRequirements};

/// Simplified recombination parameters for production
#[derive(Debug, Clone)]
pub struct SimpleRecombinationParams {
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub preserve_best: bool,
}

impl Default for SimpleRecombinationParams {
    fn default() -> Self {
        Self {
            mutation_rate: 0.1,
            crossover_rate: 0.7,
            preserve_best: true,
        }
    }
}

/// Perform genetic recombination with simplified parameters
pub async fn recombine_genetics(
    _engine: &GeneticSpawningEngine,
    parents: &[BearDogGenetics],
    spawn_purpose: &SpawnPurpose,
) -> BearDogResult<BearDogGenetics> {
    info!(
        "🧬 Starting genetic recombination with {} parents",
        parents.len()
    );

    if parents.is_empty() {
        return Err(BearDogError::Validation {
            message: "At least one parent required for recombination".to_string(),
        });
    }

    let params = SimpleRecombinationParams::default();

    // Start with the first parent as base
    let mut offspring = parents[0].clone();
    offspring.id = uuid::Uuid::new_v4().to_string();
    offspring.generation += 1;

    // Combine capabilities from all parents
    let mut combined_capabilities = Vec::new();
    for parent in parents {
        combined_capabilities.extend(parent.capabilities.clone());
    }

    // Remove duplicates and take the most diverse set
    combined_capabilities.sort();
    combined_capabilities.dedup();
    offspring.capabilities = combined_capabilities;

    // Combine and optimize crypto chromosomes
    offspring.crypto_chromosomes = combine_chromosomes(parents, &params).await?;

    // Update parent genetics for lineage tracking
    offspring.parent_genetics = Some(parents.iter().map(|p| p.id.clone()).collect());

    // Calculate fitness based on purpose
    offspring.fitness_score = calculate_recombined_fitness(&offspring, spawn_purpose).await?;

    info!(
        "✅ Genetic recombination completed for offspring: {}",
        offspring.id
    );
    debug!(
        "Offspring capabilities: {} unique",
        offspring.capabilities.len()
    );

    Ok(offspring)
}

/// Combine crypto chromosomes from multiple parents
async fn combine_chromosomes(
    parents: &[BearDogGenetics],
    params: &SimpleRecombinationParams,
) -> BearDogResult<Vec<CryptoChromosome>> {
    debug!("Combining chromosomes from {} parents", parents.len());

    let mut all_chromosomes: Vec<CryptoChromosome> = parents
        .iter()
        .flat_map(|parent| parent.crypto_chromosomes.iter().cloned())
        .collect();

    // Sort by performance and security
    all_chromosomes.sort_by(|a, b| {
        let score_a = (a.security_level as f64) * a.performance_factor * a.compatibility_score;
        let score_b = (b.security_level as f64) * b.performance_factor * b.compatibility_score;
        score_b
            .partial_cmp(&score_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Take the best chromosomes, up to a reasonable limit
    let max_chromosomes = 8;
    all_chromosomes.truncate(max_chromosomes);

    // Apply mutation if specified
    if params.mutation_rate > 0.0 {
        apply_mutations(&mut all_chromosomes, params.mutation_rate).await?;
    }

    debug!(
        "Combined {} chromosomes after optimization",
        all_chromosomes.len()
    );
    Ok(all_chromosomes)
}

/// Apply random mutations to chromosomes
async fn apply_mutations(
    chromosomes: &mut [CryptoChromosome],
    mutation_rate: f64,
) -> BearDogResult<()> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for chromosome in chromosomes.iter_mut() {
        if rng.gen::<f64>() < mutation_rate {
            // Apply small mutations to performance factors
            chromosome.performance_factor *= rng.gen_range(0.9..1.1);
            chromosome.compatibility_score *= rng.gen_range(0.95..1.05);

            // Clamp values to reasonable ranges
            chromosome.performance_factor = chromosome.performance_factor.clamp(0.1, 2.0);
            chromosome.compatibility_score = chromosome.compatibility_score.clamp(0.0, 1.0);
        }
    }

    debug!("Applied mutations with rate {}", mutation_rate);
    Ok(())
}

/// Calculate fitness score for recombined genetics
async fn calculate_recombined_fitness(
    genetics: &BearDogGenetics,
    purpose: &SpawnPurpose,
) -> BearDogResult<f64> {
    let mut fitness = 0.5; // Base fitness

    // Bonus for capabilities
    fitness += genetics.capabilities.len() as f64 * 0.1;

    // Bonus for crypto chromosomes
    fitness += genetics.crypto_chromosomes.len() as f64 * 0.05;

    // Purpose-specific bonuses
    match purpose {
        SpawnPurpose::LoadBalancing => fitness += 0.1,
        SpawnPurpose::SecurityResponse => fitness += 0.15,
        SpawnPurpose::EmergencyResponse => fitness += 0.2,
        _ => {}
    }

    // Penalty for high generation (prevent runaway inheritance)
    fitness -= genetics.generation as f64 * 0.02;

    Ok(fitness.clamp(0.0, 1.0))
}
