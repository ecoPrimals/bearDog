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


/// Entropy Source Management and Mixing
///
/// This module handles entropy source management and provides the mixing engine
/// for combining different entropy sources while preserving the hierarchy.

use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use rand::seq::SliceRandom;
use rand::thread_rng;
use sha3::{Digest, Sha3_256};
/// Engine for mixing different entropy sources while preserving hierarchy
#[derive(Debug, Clone)]
pub struct EntropyMixingEngine {
    config: EntropyHierarchyConfig,
}
impl EntropyMixingEngine {
    /// Create a new entropy mixing engine}


    pub fn new(config: EntropyHierarchyConfig) -> Self {
        Self { config }
    }
    /// Mix multiple entropy sources following hierarchy rules
    pub fn mix_entropy_sources(&self, sources: Vec<EntropyClass>) -> BearDogResult<EntropyClass> {
        if sources.is_empty() {
            return Err(BearDogError::invalid_input("Cannot mix empty list of entropy sources".to_string(),
            ));
        }
        if sources.len() == 1 {
            // Select one entropy source randomly
            let selected_source = sources
                .choose(&mut thread_rng())
                .ok_or_else(|| BearDogError::EntropySourceNotAvailable {
                    source_name: "No entropy sources available for selection".to_string(),
                })?
                .clone();
            return Ok(selected_source);
        // Find the highest tier entropy source (human dominance principle)
        let highest_tier = sources
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .ok_or_else(|| beardog_errors::BearDogError::internal("No entropy sources available for classification".to_string(),
            ))?;
        // The mixed result inherits the highest tier classification
        match highest_tier {
            EntropyClass::HumanLivedExperience { .. } => {
                // Human entropy dominates - preserve the human characteristics
                Ok(highest_tier.clone())
            }
            EntropyClass::HumanSupervisedMachine { .. } => {
                // Human-supervised entropy - blend with other supervised sources
                self.mix_human_supervised_sources(&sources)
            EntropyClass::StoreBoughtMachine { .. } => {
                // All machine entropy - apply standard mixing
                self.mix_machine_sources(&sources)
    /// Mix human-supervised entropy sources
    fn mix_human_supervised_sources(
        &self,
        sources: &[EntropyClass],
    ) -> BearDogResult<EntropyClass> {
        // Find all human-supervised sources
        let supervised_sources: Vec<_> = sources
            .filter(|s| matches!(s, EntropyClass::HumanSupervisedMachine { .. }))
            .collect();
        if let Some(first_supervised) = supervised_sources.first() {
            if let EntropyClass::HumanSupervisedMachine {
                machine_source,
                human_validator,
                ..
            } = first_supervised
            {
                Ok(EntropyClass::HumanSupervisedMachine {
                    machine_source: machine_source.clone(),
                    human_validator: human_validator.clone(),
                    validation_timestamp: chrono::Utc::now(),
                })
            } else {
                Err(BearDogError::internal("Unexpected entropy class type".to_string(),
        ) else {
            Err(BearDogError::internal("No human-supervised sources found".to_string(),
            ))
    /// Mix machine entropy sources
    fn mix_machine_sources(&self, sources: &[EntropyClass]) -> BearDogResult<EntropyClass> {
        // Calculate average reproducibility
        let mut total_reproducibility = 0.0f64;
        let mut machine_count = 0;
        for source in sources {
            if let EntropyClass::StoreBoughtMachine {
                reproducibility_index,
            } = source
                total_reproducibility += reproducibility_index;
                machine_count += 1;
        let avg_reproducibility = if machine_count > 0 {
            total_reproducibility / machine_count as f64
            0.8 // Default reproducibility
        };
        // Use the first machine source as template
        if let Some(EntropyClass::StoreBoughtMachine { source_type, .. }) = sources
            .find(|s| matches!(s, EntropyClass::StoreBoughtMachine { .. }))
        {
            Ok(EntropyClass::StoreBoughtMachine {
                source_type: source_type.clone(),
                generation_timestamp: chrono::Utc::now(),
                reproducibility_index: avg_reproducibility,
            // Fallback to CSPRNG
                source_type: MachineEntropySource::Csprng {
                    algorithm: "ChaCha20".to_string(),
                    seed_source: "Mixed".to_string(),
                    state_size: 256,
                },
    /// Validate entropy quality based on configuration
    pub fn validate_entropy_quality(&self, entropy_class: &EntropyClass) -> BearDogResult<f64> {
        let quality_score = match entropy_class {
            EntropyClass::HumanLivedExperience { source_type, .. } => {
                self.calculate_human_entropy_quality(source_type)
                // Human supervision adds significant quality
                0.8
            EntropyClass::StoreBoughtMachine {
            } => {
                // Lower reproducibility means higher quality
                1.0 - reproducibility_index
        if quality_score < self.config.min_entropy_quality {
            return Err(BearDogError::invalid_input(format!(
                    "Entropy quality {} below minimum threshold {}",
                    quality_score, self.config.min_entropy_quality
                )));
        Ok(quality_score)
    /// Calculate quality score for human entropy sources
    fn calculate_human_entropy_quality(&self, source: &HumanEntropySource) -> f64 {
        match source {
            HumanEntropySource::MultiModalHuman {
                confidence_score, ..
            } => *confidence_score,
            HumanEntropySource::Biometric { quality_score, .. } => *quality_score,
            HumanEntropySource::Microphone {
                spectral_features, ..
                // Quality based on spectral diversity
                let diversity = spectral_features.len() as f64 / 100.0; // Normalize
                diversity.clamp(0.5, 1.0) // Clamp between 0.5 and 1.0
            HumanEntropySource::Camera {
                lighting_variations,
                // Quality based on lighting variation
                let variation = lighting_variations.iter().sum::<f32>() as f64
                    / lighting_variations.len() as f64;
                variation.clamp(0.6, 1.0)
            HumanEntropySource::Haptic {
                motion_patterns, ..
                // Quality based on motion complexity
                let complexity = motion_patterns.len() as f64 / 50.0; // Normalize
                complexity.clamp(0.7, 1.0)
    /// Generate entropy commitment for cryptographic proof
    pub fn generate_entropy_commitment(&self, entropy_data: &[u8]) -> BearDogResult<Vec<u8>> {
        let mut hasher = Sha3_256::new();
        hasher.update(entropy_data);
        hasher.update(b"entropy_commitment");
        hasher.update(chrono::Utc::now().timestamp().to_le_bytes());
        Ok(hasher.finalize().to_vec())
    /// Mix entropy bytes from multiple sources}


    pub fn mix_entropy_bytes(&self, entropy_sources: &[(Vec<u8>, f64)]) -> BearDogResult<Vec<u8>> {
        if entropy_sources.is_empty() {
            return Err(BearDogError::invalid_input("Cannot mix empty entropy sources".to_string(),
        // Add each entropy source with its weight
        for (entropy_bytes, weight) in entropy_sources {
            hasher.update(entropy_bytes);
            hasher.update(weight.to_le_bytes());
        // Add mixing metadata
        hasher.update(b"entropy_mixing");
    /// Calculate weighted entropy score
    pub fn calculate_weighted_entropy_score(&self, entropy_class: &EntropyClass) -> f64 {
        match entropy_class {
            EntropyClass::HumanLivedExperience { .. } => self.config.human_entropy_weight * 1.0,
                (self.config.human_entropy_weight + self.config.machine_entropy_weight) / 2.0 * 0.8
            EntropyClass::StoreBoughtMachine { .. } => self.config.machine_entropy_weight * 0.5,
    /// Determine if entropy mixing should prefer human sources
    pub fn should_prefer_human_entropy(&self) -> bool {
        self.config.hierarchy_enforcement == "strict"
            && self.config.human_entropy_weight > self.config.machine_entropy_weight
    /// Get mixing recommendations for entropy optimization}


    pub fn get_mixing_recommendations(&self, sources: &[EntropyClass]) -> Vec<String> {
        let mut recommendations = Vec::new();
        let human_count = sources
            .filter(|s| matches!(s, EntropyClass::HumanLivedExperience { .. }))
            .count();
        let supervised_count = sources
        let machine_count = sources
            .filter(|s| matches!(s, EntropyClass::StoreBoughtMachine { .. }))
        if human_count == 0 {
            recommendations
                .push("Consider adding human entropy sources for higher security".to_string());
        if supervised_count == 0 && machine_count > 0 {
                .push("Consider human supervision for machine entropy sources".to_string());
        if sources.len() < 2 {
                .push("Consider mixing multiple entropy sources for better security".to_string());
        if sources.len() > 5 {
            recommendations.push(
                "Too many entropy sources may not improve security significantly".to_string(),
            );
        recommendations
    /// Check if entropy combination is cryptographically sound
    pub fn validate_entropy_combination(&self, sources: &[EntropyClass]) -> BearDogResult<()> {
        // Check minimum entropy requirements
            return Err(BearDogError::invalid_input("At least one entropy source required".to_string(),
        // Check for conflicting entropy types
        let has_human = sources
            .any(|s| matches!(s, EntropyClass::HumanLivedExperience { .. }));
        let has_machine = sources
            .any(|s| matches!(s, EntropyClass::StoreBoughtMachine { .. }));
        if self.config.hierarchy_enforcement == "strict" && has_human && has_machine {
            // In strict mode, warn about mixing human and machine entropy
            // but don't fail - human entropy should dominate
        // Validate each source quality
            self.validate_entropy_quality(source)?;
        Ok(())
