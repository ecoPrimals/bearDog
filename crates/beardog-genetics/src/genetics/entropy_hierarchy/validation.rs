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


/// Entropy Validation and Proof Generation
///
/// This module provides validation logic for entropy quality assessment
/// and cryptographic proof generation for ownership and irreproducibility.

use super::types::*;
// use beardog_tunnel::tunnel::hsm::{HsmManager, SecurityLevel};
use beardog_errors::{BearDogError, BearDogResult};
use chrono::Utc;
use sha3::{Digest, Sha3_256};
// use std::sync::Arc;
/// Validates entropy quality and generates cryptographic proofs
pub struct EntropyValidator {
    // hsm_manager: Arc<HsmManager>,
    config: EntropyHierarchyConfig,
}
impl EntropyValidator {
    /// Create a new entropy validator}


    pub fn new(/* hsm_manager: Arc<HsmManager>, */ config: EntropyHierarchyConfig) -> Self {
        Self {
            // hsm_manager,
            config,
        }
    }
    /// Generate ownership proof for human identity
    pub async fn generate_ownership_proof(
        &self,
        owner_identity: &HumanIdentity,
        entropy_data: &[u8],
    ) -> BearDogResult<OwnershipProof> {
        // Create proof data
        let proof_data = self.create_ownership_proof_data(owner_identity, entropy_data)?;
        // Generate signature using placeholder since HSM doesn't have generate_signature
        let signature = {
            let mut hasher = Sha3_256::new();
            hasher.update(&proof_data);
            hasher.update(b"ownership_signature");
            hasher.finalize().to_vec()
        };
        Ok(OwnershipProof {
            signature,
            timestamp: Utc::now(),
            verification_key: owner_identity.public_key.clone(),
        })
    /// Generate irreproducibility proof for entropy
    pub async fn generate_irreproducibility_proof(
        entropy_class: &EntropyClass,
    ) -> BearDogResult<IrreproducibilityProof> {
        // Generate entropy commitment
        let entropy_commitment = self.generate_entropy_commitment(entropy_data)?;
        // Generate temporal proof
        let temporal_proof = self.generate_temporal_proof(entropy_class).await?;
        // Generate uniqueness proof
        let uniqueness_proof = self
            .generate_uniqueness_proof(entropy_data, entropy_class)
            .await?;
        Ok(IrreproducibilityProof {
            entropy_commitment,
            temporal_proof,
            uniqueness_proof,
    /// Create data for ownership proof signature}


    fn create_ownership_proof_data(
    ) -> BearDogResult<Vec<u8>> {
        let mut hasher = Sha3_256::new();
        // Include owner identity
        hasher.update(owner_identity.identity_id.as_bytes());
        hasher.update(&owner_identity.public_key);
        // Include biometric hash if available
        if let Some(bio_hash) = &owner_identity.biometric_hash {
            hasher.update(bio_hash);
        // Include entropy data hash (not raw entropy for privacy)
        let entropy_hash = Sha3_256::digest(entropy_data);
        hasher.update(entropy_hash);
        // Include timestamp for freshness
        hasher.update(Utc::now().timestamp().to_le_bytes());
        // Include proof type identifier
        hasher.update(b"entropy_ownership_proof");
        Ok(hasher.finalize().to_vec())
    /// Generate entropy commitment
    fn generate_entropy_commitment(&self, entropy_data: &[u8]) -> BearDogResult<Vec<u8>> {
        hasher.update(entropy_data);
        hasher.update(b"entropy_commitment");
    /// Generate temporal proof showing when entropy was captured}


    async fn generate_temporal_proof(
        let timestamp = match entropy_class {
            EntropyClass::HumanLivedExperience {
                capture_timestamp, ..
            } => *capture_timestamp,
            EntropyClass::HumanSupervisedMachine {
                validation_timestamp,
                ..
            } => *validation_timestamp,
            EntropyClass::StoreBoughtMachine {
                generation_timestamp,
            } => *generation_timestamp,
        // Create temporal proof data
        hasher.update(timestamp.timestamp().to_le_bytes());
        hasher.update(b"temporal_proof");
        // Add entropy class type for additional binding
        let class_type = match entropy_class {
            EntropyClass::HumanLivedExperience { .. } => "human_lived_experience",
            EntropyClass::HumanSupervisedMachine { .. } => "human_supervised_machine",
            EntropyClass::StoreBoughtMachine { .. } => "store_bought_machine",
        hasher.update(class_type.as_bytes());
        let temporal_data = hasher.finalize().to_vec();
        // Use placeholder signature since HSM doesn't have generate_signature
            hasher.update(&temporal_data);
            hasher.update(b"temporal_signature");
        Ok(signature)
    /// Generate uniqueness proof showing entropy cannot be reproduced
    async fn generate_uniqueness_proof(
        // Include entropy data hash
        hasher.update(Sha3_256::digest(entropy_data));
        // Include source-specific uniqueness factors
        match entropy_class {
                source_type,
                biometric_signature,
            } => {
                hasher.update(b"human_lived_experience");
                // Add source-specific data
                match source_type {
                    HumanEntropySource::Microphone {
                        spectral_features, ..
                    } => {
                        hasher.update(b"microphone");
                        for feature in spectral_features {
                            hasher.update(feature.to_le_bytes());
                        }
                    }
                    HumanEntropySource::Camera {
                        lighting_variations,
                        ..
                        hasher.update(b"camera");
                        for variation in lighting_variations {
                            hasher.update(variation.to_le_bytes());
                    HumanEntropySource::Haptic {
                        motion_patterns, ..
                        hasher.update(b"haptic");
                        for pattern in motion_patterns {
                            hasher.update(pattern.to_le_bytes());
                    HumanEntropySource::Biometric { entropy_hash, .. } => {
                        hasher.update(b"biometric");
                        hasher.update(entropy_hash);
                    HumanEntropySource::MultiModalHuman { sources, .. } => {
                        hasher.update(b"multimodal");
                        hasher.update(sources.len().to_le_bytes());
                }
                // Add biometric signature
                hasher.update(&biometric_signature.0);
            }
                machine_source,
                human_validator,
                hasher.update(b"human_supervised_machine");
                hasher.update(human_validator.identity_id.as_bytes());
                // Add machine source specifics
                match machine_source {
                    MachineEntropySource::HardwareRNG { device_id, .. } => {
                        hasher.update(device_id.as_bytes());
                    MachineEntropySource::Csprng { algorithm, .. } => {
                        hasher.update(algorithm.as_bytes());
                    MachineEntropySource::DerivedFromHuman {
                        transition_timestamp,
                        hasher.update(transition_timestamp.timestamp().to_le_bytes());
                reproducibility_index,
                hasher.update(b"store_bought_machine");
                hasher.update(reproducibility_index.to_le_bytes());
                    MachineEntropySource::Csprng {
                        algorithm,
                        state_size,
                        hasher.update(state_size.to_le_bytes());
                    MachineEntropySource::DerivedFromHuman { .. } => {
                        hasher.update(b"derived_from_human");
        // Add current timestamp for freshness
        hasher.update(b"uniqueness_proof");
        let uniqueness_data = hasher.finalize().to_vec();
            hasher.update(&uniqueness_data);
            hasher.update(b"uniqueness_signature");
    /// Validate entropy quality based on source type and configuration
    pub fn validate_entropy_quality(&self, entropy_class: &EntropyClass) -> BearDogResult<f64> {
        let quality_score = match entropy_class {
            EntropyClass::HumanLivedExperience { source_type, .. } => {
                self.calculate_human_entropy_quality(source_type)?
            EntropyClass::HumanSupervisedMachine { .. } => {
                // Human supervision significantly improves quality
                0.8
                // Lower reproducibility means higher quality for machine sources
                1.0 - reproducibility_index
        if quality_score < self.config.min_entropy_quality {
            return Err(BearDogError::invalid_input(format!(
                    "Entropy quality {} below minimum threshold {}",
                    quality_score, self.config.min_entropy_quality
                )));
        Ok(quality_score)
    /// Calculate quality score for human entropy sources
    fn calculate_human_entropy_quality(&self, source: &HumanEntropySource) -> BearDogResult<f64> {
        let quality = match source {
            HumanEntropySource::MultiModalHuman {
                confidence_score,
                sources,
                // Multi-modal gets bonus for diversity
                let diversity_bonus = (sources.len() as f64 / 10.0).min(0.2);
                confidence_score + diversity_bonus
            HumanEntropySource::Biometric { quality_score, .. } => *quality_score,
            HumanEntropySource::Microphone {
                spectral_features,
                duration_ms,
                // Quality based on spectral diversity and duration
                let feature_diversity = spectral_features.len() as f64 / 100.0;
                let duration_factor = (*duration_ms as f64 / 10000.0).min(1.0); // 10 seconds max
                (feature_diversity * 0.7 + duration_factor * 0.3).clamp(0.5, 1.0)
            HumanEntropySource::Camera {
                lighting_variations,
                // Quality based on lighting variation and capture duration
                if lighting_variations.is_empty() {
                    return Ok(0.6); // Base quality for camera without variations
                let variation_score = lighting_variations.iter().sum::<f32>() as f64
                    / lighting_variations.len() as f64;
                let duration_factor = (*duration_ms as f64 / 5000.0).min(1.0); // 5 seconds max
                (variation_score * 0.8 + duration_factor * 0.2).clamp(0.6, 1.0)
            HumanEntropySource::Haptic {
                motion_patterns,
                touch_points,
                // Quality based on motion complexity and touch diversity
                let motion_complexity = motion_patterns.len() as f64 / 50.0;
                let touch_diversity = touch_points.len() as f64 / 20.0;
                (motion_complexity * 0.6 + touch_diversity * 0.4).clamp(0.7, 1.0)
        Ok(quality.min(1.0))
    /// Verify ownership proof (placeholder implementation)
    pub async fn verify_ownership_proof(
        _proof: &OwnershipProof,
        _owner_identity: &HumanIdentity,
        _entropy_data: &[u8],
    ) -> BearDogResult<bool> {
        // Placeholder verification - in real implementation would verify signature
        Ok(true)
    /// Verify irreproducibility proof (placeholder implementation)}


    pub async fn verify_irreproducibility_proof(
        proof: &IrreproducibilityProof,
        _entropy_class: &EntropyClass,
        // Verify entropy commitment
        let expected_commitment = self.generate_entropy_commitment(entropy_data)?;
        if proof.entropy_commitment != expected_commitment {
            return Ok(false);
        // Check that proofs are not empty (basic validation)
        if proof.temporal_proof.is_empty() || proof.uniqueness_proof.is_empty() {
    /// Validate seed usage policy}


    pub fn validate_usage_policy(
        policy: &SeedUsagePolicy,
    ) -> BearDogResult<()> {
        // Check if operations are appropriate for entropy class
            EntropyClass::HumanLivedExperience { .. } => {
                // Human entropy can be used for any operation
                Ok(())
                // Supervised entropy has some restrictions
                if policy
                    .allowed_operations
                    .contains(&"high_security_signing".to_string())
                    && !policy.requires_approval
                {
                    return Err(BearDogError::invalid_input("High security operations with supervised entropy require approval"
                                .to_string(),
                    ));
            EntropyClass::StoreBoughtMachine { .. } => {
                // Machine entropy has strict limitations
                if policy.max_uses.is_none() {
                    return Err(BearDogError::invalid_input("Machine entropy must have usage limits".to_string(),
                if !policy.requires_approval {
                    return Err(BearDogError::invalid_input("Machine entropy operations require approval".to_string(),
    /// Check if entropy meets minimum security requirements
    pub fn check_security_requirements(&self, entropy_class: &EntropyClass) -> BearDogResult<u8> {
        let security_level = match entropy_class {
            EntropyClass::HumanLivedExperience { .. } => 100, // SecurityLevel::Maximum,
            EntropyClass::HumanSupervisedMachine { .. } => 80, // SecurityLevel::High,
                if *reproducibility_index < 0.3 {
                    // SecurityLevel::High
                    80
                } else if *reproducibility_index < 0.7 {
                    // SecurityLevel::Medium
                    60
                } else {
                    // SecurityLevel::Basic // Changed from Minimal to Basic
                    40
        Ok(security_level)
