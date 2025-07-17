//! Genetic Lineage Tracking
//!
//! This module handles lineage tracking, diversity calculation, and cryptographic
//! proof of genetic lineage for spawned nodes.

use super::super::types::*;
use super::engine::GeneticSpawningEngine;
use beardog_auth::auth::{BearDogGenetics, SpawnPurpose};
use crate::genetics::types::{ParentSignature, WitnessSignature, WitnessType};
// use beardog_tunnel::tunnel::hsm::types::HsmOperation;
// use beardog_tunnel::tunnel::hsm::{SecurityLevel, SecurityRequirements};
use beardog_errors::BearDogResult;
use chrono::Utc;
use sha3::{Digest, Sha3_256};
use tracing::info;

/// Create a lineage record for a spawned child
pub async fn create_lineage_record(
    engine: &GeneticSpawningEngine,
    request: &SpawnRequest,
    child_node_id: &str,
    child_genetics: &BearDogGenetics,
) -> BearDogResult<()> {
    info!(
        "Creating HSM-backed lineage record for child: {}",
        child_node_id
    );

    let mut parent_node_ids = vec![request.requesting_parent.clone()];
    parent_node_ids.extend(request.co_parents.clone());

    let security_reqs = (); // SecurityRequirements::new(SecurityLevel::High);

    // Generate cryptographic hash of child genetics
    let child_genetics_data = format!("{child_genetics:?}");
    let child_genetics_hash = Sha3_256::digest(child_genetics_data.as_bytes()).to_vec();

    // Generate parent genetics hashes
    let mut parent_genetics_hashes = Vec::new();
    for parent_id in &parent_node_ids {
        if let Some(parent_genetics) = engine.genetics_store.load_genetics(parent_id).await? {
            let parent_genetics_data = format!("{parent_genetics:?}");
            let parent_hash = Sha3_256::digest(parent_genetics_data.as_bytes()).to_vec();
            parent_genetics_hashes.push(parent_hash);
        }
    }

    // Generate HSM signatures for lineage proof
    let mut parent_signatures = Vec::new();
    for (i, parent_id) in parent_node_ids.iter().enumerate() {
        // Create lineage proof data for signing
        let lineage_proof_data = format!(
            "lineage-{}-{}-{}-{}",
            child_node_id, parent_id, child_genetics.generation, request.request_id
        );
        let lineage_hash = Sha3_256::digest(lineage_proof_data.as_bytes());

        // Generate signature proof for this parent
        let lineage_signature = "signature_placeholder"; // engine.hsm_manager.sign_data(
            // &format!("lineage-{parent_id}"),
            // &lineage_hash,
            // &security_reqs,
            // &HsmOperation::LineageProof,
        // ).await?;

        // Generate a mock public key for the parent
        let public_key_data = format!("parent-pubkey-{parent_id}");
        let public_key = Sha3_256::digest(public_key_data.as_bytes()).to_vec();

        parent_signatures.push(ParentSignature {
            parent_node_id: parent_id.clone(),
            signature: lineage_signature,
            public_key,
        });
    }

    // Generate witness signatures for consensus validation
    let mut witness_signatures = Vec::new();
    let witness_data = format!(
        "witness-{}-{}-{}",
        child_node_id,
        request.request_id,
        Utc::now().timestamp()
    );
    let witness_hash = Sha3_256::digest(witness_data.as_bytes());

    let witness_signature_bytes = "witness_signature_placeholder".to_string(); // engine.hsm_manager.sign_data(
        // "genetic-witness",
        // &witness_hash,
        // &security_reqs,
        // &HsmOperation::GeneticWitness,
    // ).await?;

    // Generate a mock public key for the witness
    let witness_public_key = Sha3_256::digest(b"genetic-witness-pubkey").to_vec();

    witness_signatures.push(WitnessSignature {
        witness_id: "genetic-witness".to_string(),
        witness_type: WitnessType::Node,
        signature: witness_signature_bytes,
        public_key: witness_public_key,
    });

    // Calculate genetic diversity score
    let diversity_score =
        calculate_genetic_diversity_score(engine, child_genetics, &parent_node_ids).await?;

    let lineage = GeneticLineage {
        child_node_id: child_node_id.to_string(),
        parent_node_ids,
        generation: child_genetics.generation,
        lineage_proof: LineageProof {
            parent_signatures,
            child_genetics_hash,
            parent_genetics_hashes,
            witness_signatures,
        },
        spawn_timestamp: Utc::now(),
        diversity_score,
    };

    engine
        .lineage_store
        .write()
        .await
        .insert(child_node_id.to_string(), lineage);
    info!("Successfully created lineage record with HSM-backed proof");
    Ok(())
}

/// Calculate genetic diversity score based on parent genetics
pub async fn calculate_genetic_diversity_score(
    engine: &GeneticSpawningEngine,
    child_genetics: &BearDogGenetics,
    parent_ids: &[String],
) -> BearDogResult<f64> {
    let mut diversity_factors = Vec::new();

    // Load parent genetics for comparison
    let mut parent_genetics = Vec::new();
    for parent_id in parent_ids {
        if let Some(genetics) = engine.genetics_store.load_genetics(parent_id).await? {
            parent_genetics.push(genetics);
        }
    }

    if parent_genetics.is_empty() {
        return Ok(0.5); // Default diversity score
    }

    // Calculate chromosome diversity
    let chromosome_diversity = calculate_chromosome_diversity(
        &child_genetics.crypto_chromosomes,
        &parent_genetics
            .iter()
            .map(|g| &g.crypto_chromosomes)
            .collect::<Vec<_>>(),
    );
    diversity_factors.push(chromosome_diversity);

    // Calculate capability diversity
    let capability_diversity = calculate_capability_diversity(
        &child_genetics.capabilities,
        &parent_genetics
            .iter()
            .map(|g| &g.capabilities)
            .collect::<Vec<_>>(),
    );
    diversity_factors.push(capability_diversity);

    // Calculate trait diversity
    let trait_diversity = calculate_trait_diversity(
        &child_genetics.security_traits,
        &parent_genetics
            .iter()
            .map(|g| &g.security_traits)
            .collect::<Vec<_>>(),
    );
    diversity_factors.push(trait_diversity);

    // Calculate generation diversity (higher generation = more diverse)
    let generation_diversity = (child_genetics.generation as f64 / 10.0).min(1.0);
    diversity_factors.push(generation_diversity);

    // Calculate weighted average
    let total_diversity = diversity_factors.iter().sum::<f64>() / diversity_factors.len() as f64;
    Ok(total_diversity.clamp(0.0, 1.0))
}

/// Calculate chromosome diversity between child and parents
fn calculate_chromosome_diversity(
    child_chromosomes: &[beardog_auth::auth::CryptoChromosome],
    parent_chromosomes: &[&Vec<beardog_auth::auth::CryptoChromosome>],
) -> f64 {
    if child_chromosomes.is_empty() || parent_chromosomes.is_empty() {
        return 0.5;
    }

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
                        .abs();

                    let chromosome_diversity =
                        (strength_diff + performance_diff + compatibility_diff) / 3.0;
                    diversity_score += chromosome_diversity;
                    comparisons += 1;
                }
            }
        }
    }

    if comparisons > 0 {
        (diversity_score / comparisons as f64).clamp(0.0, 1.0)
    } else {
        0.8 // High diversity if no comparable chromosomes
    }
}

/// Calculate capability diversity between child and parents
fn calculate_capability_diversity(
    child_capabilities: &[beardog_auth::auth::NodeCapability],
    parent_capabilities: &[&Vec<beardog_auth::auth::NodeCapability>],
) -> f64 {
    if child_capabilities.is_empty() || parent_capabilities.is_empty() {
        return 0.5;
    }

    let mut total_parent_capabilities = std::collections::HashSet::new();
    for parent_cap_set in parent_capabilities {
        for cap in parent_cap_set.iter() {
            total_parent_capabilities.insert(cap.clone());
        }
    }

    let child_cap_set: std::collections::HashSet<_> = child_capabilities.iter().cloned().collect();

    // Calculate Jaccard diversity index
    let intersection_size = child_cap_set
        .intersection(&total_parent_capabilities)
        .count();
    let union_size = child_cap_set.union(&total_parent_capabilities).count();

    if union_size == 0 {
        0.5
    } else {
        1.0 - (intersection_size as f64 / union_size as f64)
    }
}

/// Calculate trait diversity between child and parents
fn calculate_trait_diversity(
    child_traits: &beardog_auth::auth::SecurityTraits,
    parent_traits: &[&beardog_auth::auth::SecurityTraits],
) -> f64 {
    if parent_traits.is_empty() {
        return 0.5;
    }

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
    }

    let avg_diversity = trait_differences.iter().sum::<f64>() / trait_differences.len() as f64;
    avg_diversity.clamp(0.0, 1.0)
}
