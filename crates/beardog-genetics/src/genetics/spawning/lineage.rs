// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Genetic Lineage Tracking
///
/// This module handles lineage tracking, diversity calculation, and cryptographic
/// proof of genetic lineage for spawned nodes.

use beardog_auth::auth::BearDogGenetics;
use beardog_errors::BearDogResult;
use chrono::Utc;
use tracing::info;
// Import types from parent module
use super::{GeneticLineage, LineageProof, ParentSignature, SpawnRequest, WitnessSignature};
use crate::genetics::spawning::engine::GeneticSpawningEngine;
use beardog_errors::{BearDogError, BearDogResult};
/// Create a lineage record for a spawned child
pub async fn create_lineage_record(
    _engine: &GeneticSpawningEngine,
    request: &SpawnRequest,
    child_node_id: &str,
    child_genetics: &BearDogGenetics,
) -> GeneticsResult<GeneticLineage> {
    info!("Creating lineage record for child: {}", child_node_id);
    let mut parent_signatures = Vec::new();
    // Create signatures for each parent genetics in the request
    for parent_genetics in request.parent_genetics.iter() {
        let parent_id = &parent_genetics.id;
        let lineage_signature = format!("lineage_{parent_id}_{child_node_id}");
        parent_signatures.push(ParentSignature {
            parent_id: parent_id.clone(),
            signature: lineage_signature.as_bytes().to_vec(),
            timestamp: Utc::now(),
        });
    }
    // Create witness signatures
    let mut witness_signatures = Vec::new();
    let witness_signature_bytes = format!("witness_{:?}_{}", request.spawn_purpose, child_node_id);
    witness_signatures.push(WitnessSignature {
        witness_id: "genetic-witness".to_string(),
        signature: witness_signature_bytes.as_bytes().to_vec(),
        timestamp: Utc::now(),
    });
    // Calculate genetic diversity score
    let diversity_score =
        calculate_genetic_diversity_score(&request.parent_genetics, child_genetics).await?;
    let child_genetics_hash = format!("hash_{}", child_genetics.id);
    // Create the genetic lineage record
    let lineage = GeneticLineage {
        lineage_id: format!("lineage_{}", uuid::Uuid::new_v4()),
        child_id: child_node_id.to_string(),
        parent_ids: request
            .parent_genetics
            .iter()
            .map(|g| g.id.clone())
            .collect(),
        generation: child_genetics.generation,
        lineage_proof: LineageProof {
            lineage_id: format!("proof_{}", uuid::Uuid::new_v4()),
            parent_signatures,
            witness_signatures,
            genetic_hash: child_genetics_hash,
        },
        genetic_diversity_score: diversity_score,
        created_at: Utc::now(),
    };
    info!("Lineage record created for child: {}", child_node_id);
    Ok(lineage)
}
/// Calculate genetic diversity score based on parent genetics
pub async fn calculate_genetic_diversity_score(
    parent_genetics: &[BearDogGenetics],
) -> GeneticsResult<f64> {
    let mut diversity_factors = Vec::new();
    // Load parent genetics for comparison
    let mut parent_genetics_data = Vec::new();
    for parent_genetics in parent_genetics {
        parent_genetics_data.push(parent_genetics.clone());
    if parent_genetics_data.is_empty() {
        return Ok(0.5); // Default diversity score
    // Calculate chromosome diversity
    let chromosome_diversity = calculate_chromosome_diversity(
        &child_genetics.crypto_chromosomes,
        &parent_genetics_data
            .map(|g| &g.crypto_chromosomes)
            .collect::<Vec<_>>(),
    );
    diversity_factors.push(chromosome_diversity);
    // Calculate capability diversity
    let capability_diversity = calculate_capability_diversity(
        &child_genetics.capabilities,
            .map(|g| &g.capabilities)
    diversity_factors.push(capability_diversity);
    // Calculate trait diversity
    let trait_diversity = calculate_trait_diversity(
        &child_genetics.security_traits,
            .map(|g| &g.security_traits)
    diversity_factors.push(trait_diversity);
    // Calculate generation diversity (higher generation = more diverse)
    let generation_diversity = (child_genetics.generation as f64 / 10.0).min(1.0);
    diversity_factors.push(generation_diversity);
    // Calculate weighted average
    let total_diversity = diversity_factors.iter().sum::<f64>() / diversity_factors.len() as f64;
    Ok(total_diversity.clamp(0.0, 1.0))
/// Calculate chromosome diversity between child and parents}


fn calculate_chromosome_diversity(
    child_chromosomes: &[beardog_auth::auth::CryptoChromosome],
    parent_chromosomes: &[&Vec<beardog_auth::auth::CryptoChromosome>],
) -> f64 {
    if child_chromosomes.is_empty() || parent_chromosomes.is_empty() {
        return 0.5;
    let mut diversity_score = 0.0;
    let mut comparisons = 0;
    for child_chromosome in child_chromosomes {
        for parent_chromosome_set in parent_chromosomes {
            for parent_chromosome in parent_chromosome_set.iter() {
                if std::mem::discriminant(&child_chromosome.algorithm_family)
                    == std::mem::discriminant(&parent_chromosome.algorithm_family)
                {
                    // Compare similar algorithm families
                    let strength_diff = (child_chromosome.strength_bits as f64
                        - parent_chromosome.strength_bits as f64)
                        .abs()
                        / 1024.0;
                    let performance_diff = (child_chromosome.performance_factor
                        - parent_chromosome.performance_factor)
                        .abs();
                    let compatibility_diff = (child_chromosome.compatibility_score
                        - parent_chromosome.compatibility_score)
                    let chromosome_diversity =
                        (strength_diff + performance_diff + compatibility_diff) / 3.0;
                    diversity_score += chromosome_diversity;
                    comparisons += 1;
                }
            }
        }
    if comparisons > 0 {
        (diversity_score / comparisons as f64).clamp(0.0, 1.0)
    } else {
        0.8 // High diversity if no comparable chromosomes
/// Calculate capability diversity between child and parents
fn calculate_capability_diversity(
    child_capabilities: &[beardog_auth::auth::NodeCapability],
    parent_capabilities: &[&Vec<beardog_auth::auth::NodeCapability>],
    if child_capabilities.is_empty() || parent_capabilities.is_empty() {
    let mut total_parent_capabilities = std::collections::HashSet::new();
    for parent_cap_set in parent_capabilities {
        for cap in parent_cap_set.iter() {
            total_parent_capabilities.insert(cap.clone());
    let child_cap_set: std::collections::HashSet<_> = child_capabilities.iter().cloned().collect();
    // Calculate Jaccard diversity index
    let intersection_size = child_cap_set
        .intersection(&total_parent_capabilities)
        .count();
    let union_size = child_cap_set.union(&total_parent_capabilities).count();
    if union_size == 0 {
        0.5
        1.0 - (intersection_size as f64 / union_size as f64)
/// Calculate trait diversity between child and parents}


fn calculate_trait_diversity(
    child_traits: &beardog_auth::auth::SecurityTraits,
    parent_traits: &[&beardog_auth::auth::SecurityTraits],
    if parent_traits.is_empty() {
    let mut trait_differences = Vec::new();
    for parent_trait in parent_traits {
        let trust_diff = (child_traits.trust_threshold - parent_trait.trust_threshold).abs();
        let paranoia_diff =
            (child_traits.paranoia_level as f64 - parent_trait.paranoia_level as f64).abs() / 10.0;
        let isolation_diff =
            (child_traits.isolation_preference - parent_trait.isolation_preference).abs();
        let audit_diff =
            (child_traits.audit_frequency as f64 - parent_trait.audit_frequency as f64).abs()
                / 100.0;
        let trait_diversity = (trust_diff + paranoia_diff + isolation_diff + audit_diff) / 4.0;
        trait_differences.push(trait_diversity);
    let avg_diversity = trait_differences.iter().sum::<f64>() / trait_differences.len() as f64;
    avg_diversity.clamp(0.0, 1.0)
