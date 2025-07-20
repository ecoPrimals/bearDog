//! Entropy Source Management and Mixing
//!
//! This module handles entropy source management and provides the mixing engine
//! for combining different entropy sources while preserving the hierarchy.

use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use sha3::{Digest, Sha3_256};

/// Engine for mixing different entropy sources while preserving hierarchy
#[derive(Debug, Clone)]
pub struct EntropyMixingEngine {
    config: EntropyHierarchyConfig,
}

impl EntropyMixingEngine {
    /// Create a new entropy mixing engine
    pub fn new(config: EntropyHierarchyConfig) -> Self {
        Self { config }
    }

    /// Mix multiple entropy sources following hierarchy rules
    pub fn mix_entropy_sources(&self, sources: Vec<EntropyClass>) -> BearDogResult<EntropyClass> {
        if sources.is_empty() {
            return Err(BearDogError::InvalidInput {
                message: "Cannot mix empty list of entropy sources".to_string(),
            });
        }

        if sources.len() == 1 {
            return Ok(sources
                .into_iter()
                .next()
                .expect("Vector length is 1, so next() should return Some"));
        }

        // Find the highest tier entropy source (human dominance principle)
        let highest_tier = sources
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .ok_or_else(|| beardog_errors::BearDogError::Internal {
                message: "No entropy sources available for classification".to_string(),
            })?;

        // The mixed result inherits the highest tier classification
        match highest_tier {
            EntropyClass::HumanLivedExperience { .. } => {
                // Human entropy dominates - preserve the human characteristics
                Ok(highest_tier.clone())
            }
            EntropyClass::HumanSupervisedMachine { .. } => {
                // Human-supervised entropy - blend with other supervised sources
                self.mix_human_supervised_sources(&sources)
            }
            EntropyClass::StoreBoughtMachine { .. } => {
                // All machine entropy - apply standard mixing
                self.mix_machine_sources(&sources)
            }
        }
    }

    /// Mix human-supervised entropy sources
    fn mix_human_supervised_sources(
        &self,
        sources: &[EntropyClass],
    ) -> BearDogResult<EntropyClass> {
        // Find all human-supervised sources
        let supervised_sources: Vec<_> = sources
            .iter()
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
                Err(BearDogError::Internal {
                    message: "Unexpected entropy class type".to_string(),
                })
            }
        } else {
            Err(BearDogError::Internal {
                message: "No human-supervised sources found".to_string(),
            })
        }
    }

    /// Mix machine entropy sources
    fn mix_machine_sources(&self, sources: &[EntropyClass]) -> BearDogResult<EntropyClass> {
        // Calculate average reproducibility
        let mut total_reproducibility = 0.0f64;
        let mut machine_count = 0;

        for source in sources {
            if let EntropyClass::StoreBoughtMachine {
                reproducibility_index,
                ..
            } = source
            {
                total_reproducibility += reproducibility_index;
                machine_count += 1;
            }
        }

        let avg_reproducibility = if machine_count > 0 {
            total_reproducibility / machine_count as f64
        } else {
            0.8 // Default reproducibility
        };

        // Use the first machine source as template
        if let Some(EntropyClass::StoreBoughtMachine { source_type, .. }) = sources
            .iter()
            .find(|s| matches!(s, EntropyClass::StoreBoughtMachine { .. }))
        {
            Ok(EntropyClass::StoreBoughtMachine {
                source_type: source_type.clone(),
                generation_timestamp: chrono::Utc::now(),
                reproducibility_index: avg_reproducibility,
            })
        } else {
            // Fallback to CSPRNG
            Ok(EntropyClass::StoreBoughtMachine {
                source_type: MachineEntropySource::CSPRNG {
                    algorithm: "ChaCha20".to_string(),
                    seed_source: "Mixed".to_string(),
                    state_size: 256,
                },
                generation_timestamp: chrono::Utc::now(),
                reproducibility_index: avg_reproducibility,
            })
        }
    }

    /// Validate entropy quality based on configuration
    pub fn validate_entropy_quality(&self, entropy_class: &EntropyClass) -> BearDogResult<f64> {
        let quality_score = match entropy_class {
            EntropyClass::HumanLivedExperience { source_type, .. } => {
                self.calculate_human_entropy_quality(source_type)
            }
            EntropyClass::HumanSupervisedMachine { .. } => {
                // Human supervision adds significant quality
                0.8
            }
            EntropyClass::StoreBoughtMachine {
                reproducibility_index,
                ..
            } => {
                // Lower reproducibility means higher quality
                1.0 - reproducibility_index
            }
        };

        if quality_score < self.config.min_entropy_quality {
            return Err(BearDogError::InvalidInput {
                message: format!(
                    "Entropy quality {} below minimum threshold {}",
                    quality_score, self.config.min_entropy_quality
                ),
            });
        }

        Ok(quality_score)
    }

    /// Calculate quality score for human entropy sources
    fn calculate_human_entropy_quality(&self, source: &HumanEntropySource) -> f64 {
        match source {
            HumanEntropySource::MultiModalHuman {
                confidence_score, ..
            } => *confidence_score,
            HumanEntropySource::Biometric { quality_score, .. } => *quality_score,
            HumanEntropySource::Microphone {
                spectral_features, ..
            } => {
                // Quality based on spectral diversity
                let diversity = spectral_features.len() as f64 / 100.0; // Normalize
                diversity.clamp(0.5, 1.0) // Clamp between 0.5 and 1.0
            }
            HumanEntropySource::Camera {
                lighting_variations,
                ..
            } => {
                // Quality based on lighting variation
                let variation = lighting_variations.iter().sum::<f32>() as f64
                    / lighting_variations.len() as f64;
                variation.clamp(0.6, 1.0)
            }
            HumanEntropySource::Haptic {
                motion_patterns, ..
            } => {
                // Quality based on motion complexity
                let complexity = motion_patterns.len() as f64 / 50.0; // Normalize
                complexity.clamp(0.7, 1.0)
            }
        }
    }

    /// Generate entropy commitment for cryptographic proof
    pub fn generate_entropy_commitment(&self, entropy_data: &[u8]) -> BearDogResult<Vec<u8>> {
        let mut hasher = Sha3_256::new();
        hasher.update(entropy_data);
        hasher.update(b"entropy_commitment");
        hasher.update(chrono::Utc::now().timestamp().to_le_bytes());
        Ok(hasher.finalize().to_vec())
    }

    /// Mix entropy bytes from multiple sources
    pub fn mix_entropy_bytes(&self, entropy_sources: &[(Vec<u8>, f64)]) -> BearDogResult<Vec<u8>> {
        if entropy_sources.is_empty() {
            return Err(BearDogError::InvalidInput {
                message: "Cannot mix empty entropy sources".to_string(),
            });
        }

        let mut hasher = Sha3_256::new();

        // Add each entropy source with its weight
        for (entropy_bytes, weight) in entropy_sources {
            hasher.update(entropy_bytes);
            hasher.update(weight.to_le_bytes());
        }

        // Add mixing metadata
        hasher.update(b"entropy_mixing");
        hasher.update(chrono::Utc::now().timestamp().to_le_bytes());

        Ok(hasher.finalize().to_vec())
    }

    /// Calculate weighted entropy score
    pub fn calculate_weighted_entropy_score(&self, entropy_class: &EntropyClass) -> f64 {
        match entropy_class {
            EntropyClass::HumanLivedExperience { .. } => self.config.human_entropy_weight * 1.0,
            EntropyClass::HumanSupervisedMachine { .. } => {
                (self.config.human_entropy_weight + self.config.machine_entropy_weight) / 2.0 * 0.8
            }
            EntropyClass::StoreBoughtMachine { .. } => self.config.machine_entropy_weight * 0.5,
        }
    }

    /// Determine if entropy mixing should prefer human sources
    pub fn should_prefer_human_entropy(&self) -> bool {
        self.config.hierarchy_enforcement == "strict"
            && self.config.human_entropy_weight > self.config.machine_entropy_weight
    }

    /// Get mixing recommendations for entropy optimization
    pub fn get_mixing_recommendations(&self, sources: &[EntropyClass]) -> Vec<String> {
        let mut recommendations = Vec::new();

        let human_count = sources
            .iter()
            .filter(|s| matches!(s, EntropyClass::HumanLivedExperience { .. }))
            .count();

        let supervised_count = sources
            .iter()
            .filter(|s| matches!(s, EntropyClass::HumanSupervisedMachine { .. }))
            .count();

        let machine_count = sources
            .iter()
            .filter(|s| matches!(s, EntropyClass::StoreBoughtMachine { .. }))
            .count();

        if human_count == 0 {
            recommendations
                .push("Consider adding human entropy sources for higher security".to_string());
        }

        if supervised_count == 0 && machine_count > 0 {
            recommendations
                .push("Consider human supervision for machine entropy sources".to_string());
        }

        if sources.len() < 2 {
            recommendations
                .push("Consider mixing multiple entropy sources for better security".to_string());
        }

        if sources.len() > 5 {
            recommendations.push(
                "Too many entropy sources may not improve security significantly".to_string(),
            );
        }

        recommendations
    }

    /// Check if entropy combination is cryptographically sound
    pub fn validate_entropy_combination(&self, sources: &[EntropyClass]) -> BearDogResult<()> {
        // Check minimum entropy requirements
        if sources.is_empty() {
            return Err(BearDogError::InvalidInput {
                message: "At least one entropy source required".to_string(),
            });
        }

        // Check for conflicting entropy types
        let has_human = sources
            .iter()
            .any(|s| matches!(s, EntropyClass::HumanLivedExperience { .. }));
        let has_machine = sources
            .iter()
            .any(|s| matches!(s, EntropyClass::StoreBoughtMachine { .. }));

        if self.config.hierarchy_enforcement == "strict" && has_human && has_machine {
            // In strict mode, warn about mixing human and machine entropy
            // but don't fail - human entropy should dominate
        }

        // Validate each source quality
        for source in sources {
            self.validate_entropy_quality(source)?;
        }

        Ok(())
    }
}
