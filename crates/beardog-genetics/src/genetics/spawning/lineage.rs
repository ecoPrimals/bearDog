

use beardog_auth::auth::BearDogGenetics;
use beardog_errors::BearDogResult;
use chrono::Utc;
use tracing::info;

use super::{GeneticLineage, LineageProof, ParentSignature, SpawnRequest, WitnessSignature};
use crate::genetics::spawning::engine::GeneticSpawningEngine;
use beardog_errors::{BearDogError, BearDogResult};

pub async fn create_lineage_record(
    _engine: &GeneticSpawningEngine,
    request: &SpawnRequest,
    child_node_id: &str,
    child_genetics: &BearDogGenetics,
) -> GeneticsResult<GeneticLineage> {
    info!("Creating lineage record for child: {}", child_node_id);
    let mut parent_signatures = Vec::new();

    for parent_genetics in request.parent_genetics.iter() {
        let parent_id = &parent_genetics.id;
        let lineage_signature = format!("lineage_{parent_id}_{child_node_id}");
        parent_signatures.push(ParentSignature {
            parent_id: parent_id.clone(),
            signature: lineage_signature.as_bytes().to_vec(),
            timestamp: Utc::now(),
        });
    }

    let mut witness_signatures = Vec::new();
    let witness_signature_bytes = format_args!("witness_{:?}_{}", request.spawn_purpose, child_node_id).to_string();
    witness_signatures.push(WitnessSignature {
        witness_id: "genetic-witness".to_string(),
        signature: witness_signature_bytes.as_bytes().to_vec(),
        timestamp: Utc::now(),
    });

    let diversity_score =
        calculate_genetic_diversity_score(&request.parent_genetics, child_genetics).await?;
    let child_genetics_hash = format_args!("hash_{}", child_genetics.id).to_string();

    let lineage = GeneticLineage {
        lineage_id: format_args!("lineage_{}", uuid::Uuid::new_v4().to_string()),
        child_id: child_node_id.to_string(),
        parent_ids: request
            .parent_genetics
            .iter()
            .map(|g| g.id.clone())
            .collect(),
        generation: child_genetics.generation,
        lineage_proof: LineageProof {
            lineage_id: format_args!("proof_{}", uuid::Uuid::new_v4().to_string()),
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

pub async fn calculate_genetic_diversity_score(
    parent_genetics: &[BearDogGenetics],
) -> GeneticsResult<f64> {
    let mut diversity_factors = Vec::new();

    let mut parent_genetics_data = Vec::new();
    for parent_genetics in parent_genetics {
        parent_genetics_data.push(parent_genetics.clone());
    if parent_genetics_data.is_empty() {
        return Ok(0.5); // Default diversity score

    let chromosome_diversity = calculate_chromosome_diversity(
        &child_genetics.crypto_chromosomes,
        &parent_genetics_data
            .map(|g| &g.crypto_chromosomes)
            .collect::<Vec<_>>(),
    );
    diversity_factors.push(chromosome_diversity);

    let capability_diversity = calculate_capability_diversity(
        &child_genetics.capabilities,
            .map(|g| &g.capabilities)
    diversity_factors.push(capability_diversity);

    let trait_diversity = calculate_trait_diversity(
        &child_genetics.security_traits,
            .map(|g| &g.security_traits)
    diversity_factors.push(trait_diversity);

    let generation_diversity = (child_genetics.generation as f64 / 10.0).min(1.0);
    diversity_factors.push(generation_diversity);

    let total_diversity = diversity_factors.iter().sum::<f64>() / diversity_factors.len() as f64;
    Ok(total_diversity.clamp(0.0, 1.0))

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

fn calculate_capability_diversity(
    child_capabilities: &[beardog_auth::auth::NodeCapability],
    parent_capabilities: &[&Vec<beardog_auth::auth::NodeCapability>],
    if child_capabilities.is_empty() || parent_capabilities.is_empty() {
    let mut total_parent_capabilities = std::collections::HashSet::new();
    for parent_cap_set in parent_capabilities {
        for cap in parent_cap_set.iter() {
            total_parent_capabilities.insert(cap.clone());
    let child_cap_set: std::collections::HashSet<_> = child_capabilities.iter().cloned().collect();

    let intersection_size = child_cap_set
        .intersection(&total_parent_capabilities)
        .count();
    let union_size = child_cap_set.union(&total_parent_capabilities).count();
    if union_size == 0 {
        0.5
        1.0 - (intersection_size as f64 / union_size as f64)

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
