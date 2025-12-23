//! Human Entropy Classifier
//!
//! This module classifies HSMs based on their human entropy capabilities
//! and determines tier elevation for ephemeral seed creation.

use super::{HumanEntropyCapabilities, HumanEntropyMethod, UniversalHsmCapabilities};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::{debug, info};

/// Human entropy classifier for HSM tier elevation
pub struct HumanEntropyClassifier {
    quality_assessor: EntropyQualityAssessor,
    method_evaluator: HumanEntropyMethodEvaluator,
    elevation_criteria: TierElevationCriteria,
}

/// Entropy quality assessor
pub struct EntropyQualityAssessor {
    #[allow(dead_code)] // Future implementation
    min_entropy_bits: f64,
    #[allow(dead_code)] // Future implementation
    scoring_algorithms: Vec<EntropyQualityAlgorithm>,
}

/// Human entropy method evaluator
pub struct HumanEntropyMethodEvaluator {
    method_weights: HashMap<HumanEntropyMethod, f64>,
    quality_multipliers: HashMap<HumanEntropyMethod, f64>,
}

/// Tier elevation criteria
#[derive(Debug, Clone)]
pub struct TierElevationCriteria {
    pub min_quality_score: f64,
    pub require_realtime: bool,
    pub require_biometric: bool,
    pub require_quality_assessment: bool,
}

/// Entropy quality algorithms
#[derive(Debug, Clone)]
pub enum EntropyQualityAlgorithm {
    Shannon,
    MinEntropy,
    Compression,
    Statistical,
    Behavioral,
}

/// Human entropy assessment result
#[derive(Debug, Clone)]
pub struct HumanEntropyAssessment {
    pub supports_ephemeral_seeds: bool,
    pub quality_score: f64,
    pub method_scores: HashMap<HumanEntropyMethod, f64>,
    pub collection_efficiency: f64,
    pub realtime_capability: bool,
    pub biometric_integration_quality: f64,
    pub recommended_tier_elevation: bool,
    pub assessed_at: chrono::DateTime<chrono::Utc>,
}

impl Default for TierElevationCriteria {
    fn default() -> Self {
        Self {
            min_quality_score: 0.75,
            require_realtime: true,
            require_biometric: false,
            require_quality_assessment: true,
        }
    }
}

impl HumanEntropyClassifier {
    /// Creates a new HumanEntropyClassifier instance
    ///
    /// # Errors
    /// Returns an error if component initialization fails.
    pub fn new() -> Result<Self, BearDogError> {
        info!("🧠 Initializing Human Entropy Classifier");

        let quality_assessor = EntropyQualityAssessor::new()?;
        let method_evaluator = HumanEntropyMethodEvaluator::new()?;
        let elevation_criteria = TierElevationCriteria::default();

        Ok(Self {
            quality_assessor,
            method_evaluator,
            elevation_criteria,
        })
    }

    /// Creates instance with custom criteria
    ///
    /// # Errors
    /// Returns an error if initialization fails.
    pub fn with_criteria(criteria: TierElevationCriteria) -> Result<Self, BearDogError> {
        info!("🧠 Initializing Human Entropy Classifier with custom criteria");

        let mut classifier = Self::new()?;
        classifier.elevation_criteria = criteria;

        Ok(classifier)
    }

    /// Classifies human entropy support
    pub fn classify_human_entropy_support(
        &self,
        capabilities: &UniversalHsmCapabilities,
    ) -> Result<bool, BearDogError> {
        debug!("🧠 Classifying human entropy support");

        let assessment = self.assess_human_entropy_capabilities(capabilities)?;
        let supports_entropy = self.evaluate_tier_elevation(&assessment)?;

        if supports_entropy {
            info!(
                "✅ HSM supports human entropy ephemeral seeds (quality: {:.2})",
                assessment.quality_score
            );
        } else {
            debug!(
                "❌ HSM does not meet human entropy criteria (quality: {:.2})",
                assessment.quality_score
            );
        }

        Ok(supports_entropy)
    }

    /// Assesses human entropy capabilities
    fn assess_human_entropy_capabilities(
        &self,
        capabilities: &UniversalHsmCapabilities,
    ) -> Result<HumanEntropyAssessment, BearDogError> {
        let human_entropy = &capabilities.human_entropy;

        let method_scores = self
            .method_evaluator
            .evaluate_entropy_methods(&human_entropy.collection_methods)?;

        let quality_score = self
            .quality_assessor
            .calculate_quality_score(&method_scores, human_entropy)?;

        let collection_efficiency = self.assess_collection_efficiency(human_entropy)?;
        let biometric_integration_quality = self.assess_biometric_integration(human_entropy)?;

        Ok(HumanEntropyAssessment {
            supports_ephemeral_seeds: human_entropy.ephemeral_seed_creation,
            quality_score,
            method_scores,
            collection_efficiency,
            realtime_capability: human_entropy.realtime_entropy,
            biometric_integration_quality,
            recommended_tier_elevation: false,
            assessed_at: chrono::Utc::now(),
        })
    }

    /// Evaluates tier elevation criteria
    fn evaluate_tier_elevation(
        &self,
        assessment: &HumanEntropyAssessment,
    ) -> Result<bool, BearDogError> {
        debug!("🧠 Evaluating tier elevation criteria");

        if !assessment.supports_ephemeral_seeds {
            debug!("❌ No ephemeral seed creation support");
            return Ok(false);
        }

        if assessment.method_scores.len() < 2 {
            debug!(
                "❌ Insufficient entropy collection methods: {} < 2",
                assessment.method_scores.len()
            );
            return Ok(false);
        }

        if assessment.quality_score < self.elevation_criteria.min_quality_score {
            debug!(
                "❌ Quality score below threshold: {:.2} < {:.2}",
                assessment.quality_score, self.elevation_criteria.min_quality_score
            );
            return Ok(false);
        }

        if self.elevation_criteria.require_realtime && !assessment.realtime_capability {
            debug!("❌ Real-time entropy generation required but not available");
            return Ok(false);
        }

        if self.elevation_criteria.require_biometric
            && assessment.biometric_integration_quality < 0.5
        {
            debug!(
                "❌ Biometric integration required but insufficient quality: {:.2}",
                assessment.biometric_integration_quality
            );
            return Ok(false);
        }

        info!("✅ HSM qualifies for human entropy tier elevation");
        Ok(true)
    }

    /// Assesses collection efficiency
    fn assess_collection_efficiency(
        &self,
        human_entropy: &HumanEntropyCapabilities,
    ) -> Result<f64, BearDogError> {
        let mut efficiency_score = 0.0;

        let method_diversity = human_entropy.collection_methods.len() as f64 / 8.0;
        efficiency_score += method_diversity * 0.4;

        if human_entropy.realtime_entropy {
            efficiency_score += 0.3;
        }

        if human_entropy.entropy_quality_assessment {
            efficiency_score += 0.2;
        }

        if human_entropy.behavioral_entropy {
            efficiency_score += 0.1;
        }

        Ok(efficiency_score.min(1.0))
    }

    /// Assesses biometric integration
    fn assess_biometric_integration(
        &self,
        human_entropy: &HumanEntropyCapabilities,
    ) -> Result<f64, BearDogError> {
        if !human_entropy.biometric_entropy {
            return Ok(0.0);
        }

        let biometric_methods = human_entropy
            .collection_methods
            .iter()
            .filter(|&method| {
                matches!(
                    method,
                    HumanEntropyMethod::BiometricVariations
                        | HumanEntropyMethod::VoicePatterns
                        | HumanEntropyMethod::TouchPatterns
                )
            })
            .count();

        let biometric_score = (biometric_methods as f64 / 3.0) * 0.6 + 0.4;
        Ok(biometric_score.min(1.0))
    }

    /// Gets ranked entropy methods
    pub fn get_ranked_methods(
        &self,
        assessment: &HumanEntropyAssessment,
    ) -> Result<Vec<(HumanEntropyMethod, f64)>, BearDogError> {
        let mut ranked_methods: Vec<(HumanEntropyMethod, f64)> =
            assessment.method_scores.clone().into_iter().collect();

        ranked_methods.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        Ok(ranked_methods)
    }

    /// Updates tier elevation criteria
    pub fn update_criteria(&mut self, criteria: TierElevationCriteria) {
        info!("🧠 Updating human entropy tier elevation criteria");
        self.elevation_criteria = criteria;
    }
}

impl EntropyQualityAssessor {
    /// Creates a new EntropyQualityAssessor
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            min_entropy_bits: 128.0,
            scoring_algorithms: vec![
                EntropyQualityAlgorithm::Shannon,
                EntropyQualityAlgorithm::MinEntropy,
                EntropyQualityAlgorithm::Behavioral,
            ],
        })
    }

    /// Calculates quality score
    pub fn calculate_quality_score(
        &self,
        method_scores: &HashMap<HumanEntropyMethod, f64>,
        human_entropy: &HumanEntropyCapabilities,
    ) -> Result<f64, BearDogError> {
        let mut total_score = 0.0;
        let mut weight_sum = 0.0;

        for (method, score) in method_scores {
            let weight = self.get_method_weight(method);
            total_score += score * weight;
            weight_sum += weight;
        }

        let base_score = if weight_sum > 0.0 {
            total_score / weight_sum
        } else {
            0.0
        };

        let mut final_score = base_score;

        if human_entropy.entropy_quality_assessment {
            final_score *= 1.2;
        }

        if human_entropy.realtime_entropy {
            final_score *= 1.1;
        }

        if human_entropy.biometric_entropy {
            final_score *= 1.1;
        }

        Ok(final_score.min(1.0))
    }

    /// Gets method weight
    fn get_method_weight(&self, method: &HumanEntropyMethod) -> f64 {
        match method {
            HumanEntropyMethod::BiometricVariations => 0.9,
            HumanEntropyMethod::KeystrokeDynamics => 0.8,
            HumanEntropyMethod::MouseMovement => 0.7,
            HumanEntropyMethod::TouchPatterns => 0.8,
            HumanEntropyMethod::VoicePatterns => 0.8,
            HumanEntropyMethod::BehavioralTiming => 0.7,
            HumanEntropyMethod::CameraEntropy => 0.6,
            HumanEntropyMethod::CustomInput => 0.5,
        }
    }
}

impl HumanEntropyMethodEvaluator {
    /// Creates a new HumanEntropyMethodEvaluator
    pub fn new() -> Result<Self, BearDogError> {
        let mut method_weights = HashMap::with_capacity(8);
        let mut quality_multipliers = HashMap::with_capacity(8);

        method_weights.insert(HumanEntropyMethod::BiometricVariations, 1.0);
        method_weights.insert(HumanEntropyMethod::KeystrokeDynamics, 0.9);
        method_weights.insert(HumanEntropyMethod::TouchPatterns, 0.9);
        method_weights.insert(HumanEntropyMethod::VoicePatterns, 0.8);
        method_weights.insert(HumanEntropyMethod::MouseMovement, 0.7);
        method_weights.insert(HumanEntropyMethod::BehavioralTiming, 0.8);
        method_weights.insert(HumanEntropyMethod::CameraEntropy, 0.6);
        method_weights.insert(HumanEntropyMethod::CustomInput, 0.7);

        quality_multipliers.insert(HumanEntropyMethod::BiometricVariations, 1.2);
        quality_multipliers.insert(HumanEntropyMethod::KeystrokeDynamics, 1.1);
        quality_multipliers.insert(HumanEntropyMethod::TouchPatterns, 1.1);
        quality_multipliers.insert(HumanEntropyMethod::VoicePatterns, 1.0);
        quality_multipliers.insert(HumanEntropyMethod::MouseMovement, 0.9);
        quality_multipliers.insert(HumanEntropyMethod::BehavioralTiming, 1.0);
        quality_multipliers.insert(HumanEntropyMethod::CameraEntropy, 0.8);
        quality_multipliers.insert(HumanEntropyMethod::CustomInput, 0.9);

        Ok(Self {
            method_weights,
            quality_multipliers,
        })
    }

    /// Evaluates entropy methods
    pub fn evaluate_entropy_methods(
        &self,
        methods: &[HumanEntropyMethod],
    ) -> Result<HashMap<HumanEntropyMethod, f64>, BearDogError> {
        let mut scores = HashMap::with_capacity(methods.len());

        for method in methods {
            let base_weight = self.method_weights.get(method).unwrap_or(&0.5);
            let quality_multiplier = self.quality_multipliers.get(method).unwrap_or(&1.0);

            let score = base_weight * quality_multiplier;
            scores.insert(method.clone(), score);
        }

        Ok(scores)
    }
}

/// Policy presets for tier elevation
pub mod policies {
    use super::*;

    /// High security policy
    pub fn high_security_policy() -> TierElevationCriteria {
        TierElevationCriteria {
            min_quality_score: 0.85,
            require_realtime: true,
            require_biometric: true,
            require_quality_assessment: true,
        }
    }

    /// Balanced policy
    pub fn balanced_policy() -> TierElevationCriteria {
        TierElevationCriteria {
            min_quality_score: 0.65,
            require_realtime: true,
            require_biometric: false,
            require_quality_assessment: true,
        }
    }

    /// Permissive policy
    pub fn permissive_policy() -> TierElevationCriteria {
        TierElevationCriteria {
            min_quality_score: 0.5,
            require_realtime: false,
            require_biometric: false,
            require_quality_assessment: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classifier_creation() -> Result<(), Box<dyn std::error::Error>> {
        let classifier = HumanEntropyClassifier::new();
        assert!(classifier.is_ok());
        Ok(())
    }

    #[test]
    fn test_policy_presets() -> Result<(), Box<dyn std::error::Error>> {
        let high = policies::high_security_policy();
        assert_eq!(high.min_quality_score, 0.85);
        assert!(high.require_biometric);

        let balanced = policies::balanced_policy();
        assert_eq!(balanced.min_quality_score, 0.65);
        assert!(!balanced.require_biometric);
        Ok(())
    }
}
