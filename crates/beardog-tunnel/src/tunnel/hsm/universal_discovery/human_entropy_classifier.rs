//! # Human Entropy Classifier
//!
//! This module classifies HSMs based on their human entropy capabilities,
//! specifically for ephemeral seed creation from human randomness.

use super::{DiscoveryHsmCapabilities, HumanEntropyCapabilities, HumanEntropyMethod};
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Human Entropy Classifier
///
/// Analyzes HSM capabilities and determines if they support
/// human entropy ephemeral seed creation for tier elevation.
pub struct HumanEntropyClassifier {
    /// Entropy quality assessment engine
    quality_assessor: EntropyQualityAssessor,
    /// Human entropy method evaluator
    method_evaluator: HumanEntropyMethodEvaluator,
    /// Tier elevation criteria
    elevation_criteria: TierElevationCriteria,
}

/// Entropy quality assessment engine
pub struct EntropyQualityAssessor {
    /// Minimum entropy bits required
    min_entropy_bits: f64,
    /// Quality scoring algorithms
    scoring_algorithms: Vec<EntropyQualityAlgorithm>,
}

/// Human entropy method evaluator
pub struct HumanEntropyMethodEvaluator {
    /// Method scoring weights
    method_weights: HashMap<HumanEntropyMethod, f64>,
    /// Quality multipliers for different methods
    quality_multipliers: HashMap<HumanEntropyMethod, f64>,
}

/// Tier elevation criteria
#[derive(Debug, Clone)]
pub struct TierElevationCriteria {
    /// Minimum number of entropy methods required
    pub min_entropy_methods: usize,
    /// Minimum overall entropy quality score
    pub min_quality_score: f64,
    /// Required real-time entropy capability
    pub require_realtime: bool,
    /// Required biometric integration
    pub require_biometric: bool,
    /// Required entropy quality assessment
    pub require_quality_assessment: bool,
}

/// Entropy quality scoring algorithms
#[derive(Debug, Clone)]
pub enum EntropyQualityAlgorithm {
    /// Shannon entropy calculation
    Shannon,
    /// Min-entropy estimation
    MinEntropy,
    /// Compression-based entropy
    Compression,
    /// Statistical entropy tests
    Statistical,
    /// Behavioral entropy analysis
    Behavioral,
}

/// Human entropy assessment result
#[derive(Debug, Clone)]
pub struct HumanEntropyAssessment {
    /// Whether HSM supports human entropy ephemeral seeds
    pub supports_ephemeral_seeds: bool,
    /// Overall entropy quality score (0.0 - 1.0)
    pub quality_score: f64,
    /// Individual method scores
    pub method_scores: HashMap<HumanEntropyMethod, f64>,
    /// Entropy collection efficiency
    pub collection_efficiency: f64,
    /// Real-time generation capability
    pub realtime_capability: bool,
    /// Biometric integration quality
    pub biometric_integration_quality: f64,
    /// Recommended tier elevation
    pub recommended_tier_elevation: bool,
    /// Assessment timestamp
    pub assessed_at: chrono::DateTime<chrono::Utc>,
}

impl Default for TierElevationCriteria {
    fn default() -> Self {
        Self {
            min_entropy_methods: 2,
            min_quality_score: 0.75,
            require_realtime: true,
            require_biometric: false,
            require_quality_assessment: true,
        }
    }
}

impl HumanEntropyClassifier {
    /// Create a new human entropy classifier
    pub async fn new() -> BearDogResult<Self> {
        info!("🧠 Initializing Human Entropy Classifier");

        let quality_assessor = EntropyQualityAssessor::new().await?;
        let method_evaluator = HumanEntropyMethodEvaluator::new().await?;
        let elevation_criteria = TierElevationCriteria::default();

        Ok(Self {
            quality_assessor,
            method_evaluator,
            elevation_criteria,
        })
    }

    /// Create classifier with custom criteria
    pub async fn with_criteria(criteria: TierElevationCriteria) -> BearDogResult<Self> {
        info!("🧠 Initializing Human Entropy Classifier with custom criteria");

        let quality_assessor = EntropyQualityAssessor::new().await?;
        let method_evaluator = HumanEntropyMethodEvaluator::new().await?;

        Ok(Self {
            quality_assessor,
            method_evaluator,
            elevation_criteria: criteria,
        })
    }

    /// Classify human entropy support for an HSM
    pub async fn classify_human_entropy_support(
        &self,
        capabilities: &DiscoveryHsmCapabilities,
    ) -> BearDogResult<bool> {
        debug!("🧠 Classifying human entropy support");

        let assessment = self.assess_human_entropy_capabilities(capabilities).await?;
        
        let supports_entropy = self.evaluate_tier_elevation(&assessment).await?;

        if supports_entropy {
            info!("✅ HSM supports human entropy ephemeral seeds (quality: {:.2})", 
                 assessment.quality_score);
        } else {
            debug!("❌ HSM does not meet human entropy criteria (quality: {:.2})", 
                  assessment.quality_score);
        }

        Ok(supports_entropy)
    }

    /// Perform comprehensive human entropy assessment
    pub async fn assess_human_entropy_capabilities(
        &self,
        capabilities: &DiscoveryHsmCapabilities,
    ) -> BearDogResult<HumanEntropyAssessment> {
        debug!("🧠 Performing comprehensive human entropy assessment");

        let human_entropy = &capabilities.human_entropy;

        // Assess individual entropy methods
        let method_scores = self.method_evaluator
            .evaluate_entropy_methods(&human_entropy.collection_methods).await?;

        // Calculate overall quality score
        let quality_score = self.quality_assessor
            .calculate_quality_score(human_entropy, &method_scores).await?;

        // Evaluate collection efficiency
        let collection_efficiency = self.evaluate_collection_efficiency(human_entropy).await?;

        // Assess biometric integration quality
        let biometric_integration_quality = self.assess_biometric_integration(human_entropy).await?;

        // Determine tier elevation recommendation
        let assessment = HumanEntropyAssessment {
            supports_ephemeral_seeds: human_entropy.ephemeral_seed_creation,
            quality_score,
            method_scores,
            collection_efficiency,
            realtime_capability: human_entropy.realtime_entropy,
            biometric_integration_quality,
            recommended_tier_elevation: false, // Will be set below
            assessed_at: chrono::Utc::now(),
        };

        let recommended_tier_elevation = self.evaluate_tier_elevation(&assessment).await?;

        Ok(HumanEntropyAssessment {
            recommended_tier_elevation,
            ..assessment
        })
    }

    /// Evaluate if HSM qualifies for tier elevation
    async fn evaluate_tier_elevation(
        &self,
        assessment: &HumanEntropyAssessment,
    ) -> BearDogResult<bool> {
        debug!("🧠 Evaluating tier elevation criteria");

        // Check basic ephemeral seed support
        if !assessment.supports_ephemeral_seeds {
            debug!("❌ No ephemeral seed creation support");
            return Ok(false);
        }

        // Check minimum entropy methods
        if assessment.method_scores.len() < self.elevation_criteria.min_entropy_methods {
            debug!("❌ Insufficient entropy methods: {} < {}", 
                  assessment.method_scores.len(), 
                  self.elevation_criteria.min_entropy_methods);
            return Ok(false);
        }

        // Check minimum quality score
        if assessment.quality_score < self.elevation_criteria.min_quality_score {
            debug!("❌ Insufficient quality score: {:.2} < {:.2}", 
                  assessment.quality_score, 
                  self.elevation_criteria.min_quality_score);
            return Ok(false);
        }

        // Check real-time capability if required
        if self.elevation_criteria.require_realtime && !assessment.realtime_capability {
            debug!("❌ Real-time entropy generation required but not available");
            return Ok(false);
        }

        // Check biometric integration if required
        if self.elevation_criteria.require_biometric && assessment.biometric_integration_quality < 0.5 {
            debug!("❌ Biometric integration required but insufficient quality: {:.2}", 
                  assessment.biometric_integration_quality);
            return Ok(false);
        }

        info!("✅ HSM qualifies for human entropy tier elevation");
        Ok(true)
    }

    /// Evaluate entropy collection efficiency
    async fn evaluate_collection_efficiency(
        &self,
        human_entropy: &HumanEntropyCapabilities,
    ) -> BearDogResult<f64> {
        let mut efficiency_score = 0.0;

        // Base efficiency from method diversity
        let method_diversity = human_entropy.collection_methods.len() as f64 / 8.0; // Max 8 methods
        efficiency_score += method_diversity * 0.4;

        // Real-time collection bonus
        if human_entropy.realtime_entropy {
            efficiency_score += 0.3;
        }

        // Quality assessment capability
        if human_entropy.entropy_quality_assessment {
            efficiency_score += 0.2;
        }

        // Behavioral entropy patterns
        if human_entropy.behavioral_entropy {
            efficiency_score += 0.1;
        }

        Ok(efficiency_score.min(1.0))
    }

    /// Assess biometric integration quality
    async fn assess_biometric_integration(
        &self,
        human_entropy: &HumanEntropyCapabilities,
    ) -> BearDogResult<f64> {
        let mut biometric_score = 0.0;

        if !human_entropy.biometric_entropy {
            return Ok(0.0);
        }

        // Check for biometric entropy methods
        let biometric_methods = human_entropy.collection_methods.iter()
            .filter(|&method| matches!(method, 
                HumanEntropyMethod::BiometricVariations |
                HumanEntropyMethod::VoicePatterns |
                HumanEntropyMethod::TouchPatterns
            ))
            .count();

        biometric_score += (biometric_methods as f64 / 3.0) * 0.7;

        // Quality assessment integration
        if human_entropy.entropy_quality_assessment {
            biometric_score += 0.3;
        }

        Ok(biometric_score.min(1.0))
    }

    /// Get human entropy methods ranked by quality
    pub async fn get_ranked_entropy_methods(
        &self,
        capabilities: &DiscoveryHsmCapabilities,
    ) -> BearDogResult<Vec<(HumanEntropyMethod, f64)>> {
        let assessment = self.assess_human_entropy_capabilities(capabilities).await?;
        
        let mut ranked_methods: Vec<(HumanEntropyMethod, f64)> = assessment.method_scores
            .into_iter()
            .collect();
        
        ranked_methods.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        Ok(ranked_methods)
    }

    /// Update tier elevation criteria
    pub fn update_criteria(&mut self, criteria: TierElevationCriteria) {
        info!("🧠 Updating human entropy tier elevation criteria");
        self.elevation_criteria = criteria;
    }
}

impl EntropyQualityAssessor {
    /// Create a new entropy quality assessor
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            min_entropy_bits: 128.0,
            scoring_algorithms: vec![
                EntropyQualityAlgorithm::Shannon,
                EntropyQualityAlgorithm::MinEntropy,
                EntropyQualityAlgorithm::Behavioral,
            ],
        })
    }

    /// Calculate overall entropy quality score
    pub async fn calculate_quality_score(
        &self,
        human_entropy: &HumanEntropyCapabilities,
        method_scores: &HashMap<HumanEntropyMethod, f64>,
    ) -> BearDogResult<f64> {
        let mut total_score = 0.0;
        let mut weight_sum = 0.0;

        // Score based on method quality
        for (method, score) in method_scores {
            let weight = self.get_method_weight(method);
            total_score += score * weight;
            weight_sum += weight;
        }

        // Normalize base score
        let base_score = if weight_sum > 0.0 {
            total_score / weight_sum
        } else {
            0.0
        };

        // Apply capability bonuses
        let mut final_score = base_score;

        // Quality assessment capability bonus
        if human_entropy.entropy_quality_assessment {
            final_score *= 1.2;
        }

        // Real-time entropy bonus
        if human_entropy.realtime_entropy {
            final_score *= 1.1;
        }

        // Behavioral entropy bonus
        if human_entropy.behavioral_entropy {
            final_score *= 1.1;
        }

        // Biometric entropy bonus
        if human_entropy.biometric_entropy {
            final_score *= 1.1;
        }

        Ok(final_score.min(1.0))
    }

    /// Get scoring weight for entropy method
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
    /// Create a new method evaluator
    pub async fn new() -> BearDogResult<Self> {
        let mut method_weights = HashMap::new();
        let mut quality_multipliers = HashMap::new();

        // Initialize method weights (higher = better for tier elevation)
        method_weights.insert(HumanEntropyMethod::BiometricVariations, 1.0);
        method_weights.insert(HumanEntropyMethod::KeystrokeDynamics, 0.9);
        method_weights.insert(HumanEntropyMethod::TouchPatterns, 0.9);
        method_weights.insert(HumanEntropyMethod::VoicePatterns, 0.8);
        method_weights.insert(HumanEntropyMethod::MouseMovement, 0.7);
        method_weights.insert(HumanEntropyMethod::BehavioralTiming, 0.8);
        method_weights.insert(HumanEntropyMethod::CameraEntropy, 0.6);
        method_weights.insert(HumanEntropyMethod::CustomInput, 0.7);

        // Initialize quality multipliers
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

    /// Evaluate entropy collection methods
    pub async fn evaluate_entropy_methods(
        &self,
        methods: &[HumanEntropyMethod],
    ) -> BearDogResult<HashMap<HumanEntropyMethod, f64>> {
        let mut scores = HashMap::new();

        for method in methods {
            let base_weight = self.method_weights.get(method).unwrap_or(&0.5);
            let quality_multiplier = self.quality_multipliers.get(method).unwrap_or(&1.0);
            
            let score = base_weight * quality_multiplier;
            scores.insert(method.clone(), score);
        }

        Ok(scores)
    }
}

/// Specialized human entropy tier elevation policies
pub mod policies {
    use super::*;

    /// High-security policy for maximum entropy quality
    pub fn high_security_policy() -> TierElevationCriteria {
        TierElevationCriteria {
            min_entropy_methods: 3,
            min_quality_score: 0.85,
            require_realtime: true,
            require_biometric: true,
            require_quality_assessment: true,
        }
    }

    /// Balanced policy for general use
    pub fn balanced_policy() -> TierElevationCriteria {
        TierElevationCriteria {
            min_entropy_methods: 2,
            min_quality_score: 0.75,
            require_realtime: true,
            require_biometric: false,
            require_quality_assessment: true,
        }
    }

    /// Permissive policy for development/testing
    pub fn permissive_policy() -> TierElevationCriteria {
        TierElevationCriteria {
            min_entropy_methods: 1,
            min_quality_score: 0.5,
            require_realtime: false,
            require_biometric: false,
            require_quality_assessment: false,
        }
    }
} 