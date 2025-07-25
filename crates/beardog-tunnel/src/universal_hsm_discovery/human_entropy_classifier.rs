//! Human Entropy Classification System
//!
//! This module classifies HSMs based on their human entropy capabilities, specifically
//! for ephemeral seed creation, which influences tier elevation. HSMs that can create
//! human entropy ephemeral seeds get elevated to premium tiers.

use super::*;
use beardog_errors::BearDogResult;
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// Human entropy classifier for HSM tier elevation
#[derive(Debug)]
pub struct HumanEntropyClassifier {
    quality_assessor: EntropyQualityAssessor,
    method_evaluator: HumanEntropyMethodEvaluator,
    tier_elevation_criteria: TierElevationCriteria,
}

impl HumanEntropyClassifier {
    pub fn new() -> BearDogResult<Self> {
        Ok(Self {
            quality_assessor: EntropyQualityAssessor::new()?,
            method_evaluator: HumanEntropyMethodEvaluator::new()?,
            tier_elevation_criteria: TierElevationCriteria::default(),
        })
    }

    /// Classify if an HSM supports human entropy for tier elevation
    pub async fn classify_human_entropy_support(
        &self,
        capabilities: &HsmCapabilities,
    ) -> BearDogResult<bool> {
        debug!("🧠 Classifying human entropy support");

        let assessment = self.assess_human_entropy_capabilities(capabilities).await?;
        let supports_entropy = self.evaluate_tier_elevation(&assessment).await?;

        if supports_entropy {
            info!("✅ HSM qualifies for human entropy tier elevation");
        } else {
            info!("❌ HSM does not meet human entropy criteria");
        }

        Ok(supports_entropy)
    }

    /// Assess the quality and capabilities of human entropy features
    pub async fn assess_human_entropy_capabilities(
        &self,
        capabilities: &HsmCapabilities,
    ) -> BearDogResult<HumanEntropyAssessment> {
        debug!("🔍 Assessing human entropy capabilities");

        let entropy_caps = &capabilities.human_entropy;

        // Basic requirement check
        if !entropy_caps.supports_human_entropy || !entropy_caps.supports_ephemeral_seeds {
            return Ok(HumanEntropyAssessment {
                overall_score: 0.0,
                supports_ephemeral_seeds: false,
                quality_rating: EntropyQualityRating::None,
                collection_efficiency: 0.0,
                biometric_integration_score: 0.0,
                real_time_capability: false,
                verification_strength: 0.0,
                method_diversity_score: 0.0,
                temporal_analysis_capability: false,
                meets_tier_elevation_criteria: false,
                recommended_tier: HsmTier::Software,
            });
        }

        // Evaluate collection methods
        let method_diversity_score = self
            .method_evaluator
            .evaluate_collection_methods(&entropy_caps.entropy_collection_methods)
            .await?;

        // Assess collection efficiency
        let collection_efficiency = self
            .quality_assessor
            .evaluate_collection_efficiency(entropy_caps)
            .await?;

        // Assess biometric integration
        let biometric_integration_score = self
            .quality_assessor
            .assess_biometric_integration(entropy_caps)
            .await?;

        // Assess verification strength
        let verification_strength = if entropy_caps.entropy_verification {
            0.9
        } else {
            0.3
        };

        // Calculate overall quality rating
        let quality_rating = self
            .quality_assessor
            .calculate_quality_rating(entropy_caps)
            .await?;

        // Calculate overall score
        let overall_score = self
            .calculate_overall_entropy_score(
                method_diversity_score,
                collection_efficiency,
                biometric_integration_score,
                verification_strength,
                entropy_caps,
            )
            .await?;

        // Determine recommended tier
        let recommended_tier = self.determine_recommended_tier(overall_score).await?;

        // Check if meets tier elevation criteria
        let meets_criteria = self
            .evaluate_tier_elevation(&HumanEntropyAssessment {
                overall_score,
                supports_ephemeral_seeds: entropy_caps.supports_ephemeral_seeds,
                quality_rating: quality_rating.clone(),
                collection_efficiency,
                biometric_integration_score,
                real_time_capability: entropy_caps.real_time_entropy_generation,
                verification_strength,
                method_diversity_score,
                temporal_analysis_capability: entropy_caps.temporal_entropy_collection,
                meets_tier_elevation_criteria: true, // Temporary for recursion avoidance
                recommended_tier,
            })
            .await?;

        Ok(HumanEntropyAssessment {
            overall_score,
            supports_ephemeral_seeds: entropy_caps.supports_ephemeral_seeds,
            quality_rating,
            collection_efficiency,
            biometric_integration_score,
            real_time_capability: entropy_caps.real_time_entropy_generation,
            verification_strength,
            method_diversity_score,
            temporal_analysis_capability: entropy_caps.temporal_entropy_collection,
            meets_tier_elevation_criteria: meets_criteria,
            recommended_tier,
        })
    }

    /// Evaluate if HSM meets criteria for tier elevation
    pub async fn evaluate_tier_elevation(
        &self,
        assessment: &HumanEntropyAssessment,
    ) -> BearDogResult<bool> {
        debug!("📊 Evaluating tier elevation criteria");

        // Must support ephemeral seeds
        if !assessment.supports_ephemeral_seeds {
            debug!("❌ No ephemeral seed support");
            return Ok(false);
        }

        // Must meet minimum overall score
        if assessment.overall_score < self.tier_elevation_criteria.min_overall_score {
            debug!(
                "❌ Overall score too low: {} < {}",
                assessment.overall_score, self.tier_elevation_criteria.min_overall_score
            );
            return Ok(false);
        }

        // Must have sufficient method diversity
        if assessment.method_diversity_score < self.tier_elevation_criteria.min_method_diversity {
            debug!(
                "❌ Method diversity too low: {} < {}",
                assessment.method_diversity_score,
                self.tier_elevation_criteria.min_method_diversity
            );
            return Ok(false);
        }

        // Must support real-time generation if required
        if self.tier_elevation_criteria.require_real_time && !assessment.real_time_capability {
            debug!("❌ Real-time capability required but not supported");
            return Ok(false);
        }

        // Must support entropy verification if required
        if self.tier_elevation_criteria.require_verification
            && assessment.verification_strength < 0.7
        {
            debug!(
                "❌ Verification strength insufficient: {}",
                assessment.verification_strength
            );
            return Ok(false);
        }

        // Must meet quality rating threshold
        match assessment.quality_rating {
            EntropyQualityRating::None | EntropyQualityRating::Low => {
                debug!("❌ Quality rating too low: {:?}", assessment.quality_rating);
                Ok(false)
            }
            EntropyQualityRating::Medium => {
                Ok(assessment.overall_score >= 0.8) // Higher bar for medium quality
            }
            EntropyQualityRating::High | EntropyQualityRating::Premium => Ok(true),
        }
    }

    async fn calculate_overall_entropy_score(
        &self,
        method_diversity_score: f64,
        collection_efficiency: f64,
        biometric_integration_score: f64,
        verification_strength: f64,
        entropy_caps: &HumanEntropyCapabilities,
    ) -> BearDogResult<f64> {
        let mut score = 0.0;

        // Weight different factors
        score += method_diversity_score * 0.25; // 25% - diversity of collection methods
        score += collection_efficiency * 0.20; // 20% - efficiency of collection
        score += biometric_integration_score * 0.20; // 20% - biometric integration
        score += verification_strength * 0.15; // 15% - verification capabilities

        // Bonus factors
        if entropy_caps.real_time_entropy_generation {
            score += 0.10; // 10% bonus for real-time
        }
        if entropy_caps.temporal_entropy_collection {
            score += 0.05; // 5% bonus for temporal analysis
        }
        if entropy_caps.entropy_quality_assessment {
            score += 0.05; // 5% bonus for quality assessment
        }

        // Ephemeral seed lifetime bonus
        if let Some(lifetime) = entropy_caps.ephemeral_seed_lifetime {
            let lifetime_minutes = lifetime.as_secs() / 60;
            if lifetime_minutes >= 5 && lifetime_minutes <= 30 {
                score += 0.05; // Bonus for good lifetime range
            }
        }

        Ok(score.min(1.0)) // Cap at 1.0
    }

    async fn determine_recommended_tier(&self, overall_score: f64) -> BearDogResult<HsmTier> {
        if overall_score >= 0.9 {
            Ok(HsmTier::HumanEntropyPremium)
        } else if overall_score >= 0.7 {
            Ok(HsmTier::HighSecurity)
        } else if overall_score >= 0.5 {
            Ok(HsmTier::CertifiedHardware)
        } else if overall_score >= 0.3 {
            Ok(HsmTier::BasicHardware)
        } else {
            Ok(HsmTier::Software)
        }
    }

    /// Get ranked entropy collection methods by effectiveness
    pub async fn get_ranked_entropy_methods(
        &self,
        methods: &[EntropyCollectionMethod],
    ) -> BearDogResult<Vec<(EntropyCollectionMethod, f64)>> {
        let mut ranked = Vec::new();

        for method in methods {
            let score = self
                .method_evaluator
                .evaluate_method_effectiveness(method)
                .await?;
            ranked.push((method.clone(), score));
        }

        // Sort by score (highest first)
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        Ok(ranked)
    }

    /// Update tier elevation criteria
    pub fn update_criteria(&mut self, criteria: TierElevationCriteria) {
        self.tier_elevation_criteria = criteria;
    }
}

/// Assessment result for human entropy capabilities
#[derive(Debug, Clone)]
pub struct HumanEntropyAssessment {
    pub overall_score: f64,
    pub supports_ephemeral_seeds: bool,
    pub quality_rating: EntropyQualityRating,
    pub collection_efficiency: f64,
    pub biometric_integration_score: f64,
    pub real_time_capability: bool,
    pub verification_strength: f64,
    pub method_diversity_score: f64,
    pub temporal_analysis_capability: bool,
    pub meets_tier_elevation_criteria: bool,
    pub recommended_tier: HsmTier,
}

/// Quality rating for entropy collection
#[derive(Debug, Clone, PartialEq)]
pub enum EntropyQualityRating {
    None,
    Low,
    Medium,
    High,
    Premium,
}

/// Criteria for tier elevation based on human entropy
#[derive(Debug, Clone)]
pub struct TierElevationCriteria {
    pub min_overall_score: f64,
    pub min_method_diversity: f64,
    pub require_real_time: bool,
    pub require_verification: bool,
    pub require_biometric_integration: bool,
    pub min_collection_efficiency: f64,
    pub policies: TierElevationPolicies,
}

impl Default for TierElevationCriteria {
    fn default() -> Self {
        Self {
            min_overall_score: 0.6,      // 60% minimum score
            min_method_diversity: 0.5,   // 50% method diversity
            require_real_time: false,    // Real-time not required by default
            require_verification: false, // Verification not required by default
            require_biometric_integration: false,
            min_collection_efficiency: 0.4,
            policies: TierElevationPolicies::default(),
        }
    }
}

/// Policies for different types of tier elevation
#[derive(Debug, Clone)]
pub struct TierElevationPolicies {
    pub mobile_hsm_bonus: f64,          // Bonus for mobile HSMs
    pub hardware_hsm_bonus: f64,        // Bonus for hardware HSMs
    pub biometric_entropy_weight: f64,  // Weight for biometric entropy
    pub behavioral_entropy_weight: f64, // Weight for behavioral entropy
    pub temporal_entropy_weight: f64,   // Weight for temporal entropy
}

impl Default for TierElevationPolicies {
    fn default() -> Self {
        Self {
            mobile_hsm_bonus: 0.1,           // 10% bonus for mobile
            hardware_hsm_bonus: 0.05,        // 5% bonus for hardware
            biometric_entropy_weight: 0.3,   // 30% weight
            behavioral_entropy_weight: 0.25, // 25% weight
            temporal_entropy_weight: 0.15,   // 15% weight
        }
    }
}

/// Entropy quality assessor
#[derive(Debug)]
pub struct EntropyQualityAssessor;

impl EntropyQualityAssessor {
    pub fn new() -> BearDogResult<Self> {
        Ok(Self)
    }

    pub async fn evaluate_collection_efficiency(
        &self,
        entropy_caps: &HumanEntropyCapabilities,
    ) -> BearDogResult<f64> {
        let mut efficiency = 0.0;

        // Base efficiency from number of methods
        let method_count = entropy_caps.entropy_collection_methods.len() as f64;
        efficiency += (method_count / 8.0).min(0.4); // Max 40% from method count

        // Real-time generation bonus
        if entropy_caps.real_time_entropy_generation {
            efficiency += 0.2;
        }

        // Quality assessment capability
        if entropy_caps.entropy_quality_assessment {
            efficiency += 0.2;
        }

        // User interaction entropy
        if entropy_caps.user_interaction_entropy {
            efficiency += 0.1;
        }

        // Temporal collection
        if entropy_caps.temporal_entropy_collection {
            efficiency += 0.1;
        }

        Ok(efficiency.min(1.0))
    }

    pub async fn assess_biometric_integration(
        &self,
        entropy_caps: &HumanEntropyCapabilities,
    ) -> BearDogResult<f64> {
        if !entropy_caps.biometric_entropy_integration {
            return Ok(0.0);
        }

        let mut score: f64 = 0.3; // Base score for having biometric integration

        // Check for biometric-related collection methods
        for method in &entropy_caps.entropy_collection_methods {
            match method {
                EntropyCollectionMethod::BiometricVariation => {
                    score += 0.3;
                }
                EntropyCollectionMethod::KeyboardTiming => {
                    score += 0.2;
                }
                EntropyCollectionMethod::TouchPatterns { pressure_sensitive } => {
                    score += if *pressure_sensitive { 0.1 } else { 0.05 };
                }
                EntropyCollectionMethod::TouchPatternsAdvanced { pressure_sensitive } => {
                    score += if *pressure_sensitive { 0.15 } else { 0.1 };
                }
                EntropyCollectionMethod::DeviceMotion => {
                    score += 0.1;
                }
                EntropyCollectionMethod::EnvironmentalSensors { .. } => {
                    score += 0.05;
                }
                _ => {}
            }
        }

        Ok(score.min(1.0))
    }

    pub async fn calculate_quality_rating(
        &self,
        entropy_caps: &HumanEntropyCapabilities,
    ) -> BearDogResult<EntropyQualityRating> {
        let method_count = entropy_caps.entropy_collection_methods.len();
        let has_verification = entropy_caps.entropy_verification;
        let has_real_time = entropy_caps.real_time_entropy_generation;
        let has_biometric = entropy_caps.biometric_entropy_integration;
        let has_temporal = entropy_caps.temporal_entropy_collection;

        let advanced_features = [has_verification, has_real_time, has_biometric, has_temporal]
            .iter()
            .filter(|&&x| x)
            .count();

        match (method_count, advanced_features) {
            (0, _) => Ok(EntropyQualityRating::None),
            (1, 0..=1) => Ok(EntropyQualityRating::Low),
            (1..=2, 0..=2) => Ok(EntropyQualityRating::Low),
            (3..=4, 2..=3) => Ok(EntropyQualityRating::Medium),
            (5..=6, 3..=4) => Ok(EntropyQualityRating::High),
            (7.., 4) => Ok(EntropyQualityRating::Premium),
            _ => Ok(EntropyQualityRating::Medium),
        }
    }
}

/// Human entropy method evaluator
#[derive(Debug)]
pub struct HumanEntropyMethodEvaluator;

impl HumanEntropyMethodEvaluator {
    pub fn new() -> BearDogResult<Self> {
        Ok(Self)
    }

    pub async fn evaluate_collection_methods(
        &self,
        methods: &[EntropyCollectionMethod],
    ) -> BearDogResult<f64> {
        if methods.is_empty() {
            return Ok(0.0);
        }

        let mut total_score = 0.0;
        let mut unique_categories = std::collections::HashSet::new();

        for method in methods {
            let method_score = self.evaluate_method_effectiveness(method).await?;
            total_score += method_score;

            // Track unique categories for diversity bonus
            let category = self.get_method_category(method);
            unique_categories.insert(category);
        }

        // Average method score
        let avg_method_score = total_score / methods.len() as f64;

        // Diversity bonus (max 20% bonus for having methods from different categories)
        let diversity_bonus = (unique_categories.len() as f64 / 8.0) * 0.2;

        Ok((avg_method_score + diversity_bonus).min(1.0))
    }

    pub async fn evaluate_method_effectiveness(
        &self,
        method: &EntropyCollectionMethod,
    ) -> BearDogResult<f64> {
        let base_score = match method {
            EntropyCollectionMethod::BiometricVariation => {
                // Template noise analysis improves biometric variation quality
                0.8  // Default score for biometric variation
            }
            EntropyCollectionMethod::TouchPatterns { pressure_sensitive } => {
                if *pressure_sensitive {
                    0.8
                } else {
                    0.5
                }
            }
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => {
                // Advanced touch patterns with timing analysis
                0.6  // Default score for advanced touch patterns
            }
            EntropyCollectionMethod::DeviceMotion => {
                // Device motion with velocity analysis
                0.5  // Default score for device motion
            }
            EntropyCollectionMethod::EnvironmentalSensors { .. } => {
                // Environmental sensors with quantum randomness and ambient noise
                0.85  // Default high score for environmental sensors
            }
            EntropyCollectionMethod::KeyboardTiming => {
                68.0
            }
            EntropyCollectionMethod::TouchPatterns { .. } => {
                75.0  
            }
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => {
                80.0
            }
            EntropyCollectionMethod::DeviceMotion => {
                60.0
            }
            EntropyCollectionMethod::EnvironmentalSensors { .. } => {
                50.0
            }
        };

        Ok(base_score)
    }

    fn get_method_category(&self, method: &EntropyCollectionMethod) -> &'static str {
        match method {
            EntropyCollectionMethod::BiometricVariation => "biometric_variation",
            EntropyCollectionMethod::TouchPatterns { .. } => "touch",
            EntropyCollectionMethod::KeyboardTiming => "keyboard_timing",
            EntropyCollectionMethod::DeviceMotion => "device_motion",
            EntropyCollectionMethod::EnvironmentalSensors { .. } => "environmental",
        }
    }

    /// Analyze capability requirements for collection method
    pub async fn analyze_capability_requirements(&self, method: &EntropyCollectionMethod) -> BearDogResult<CapabilityRequirements> {
        debug!("🔍 Analyzing capability requirements for: {:?}", method);

        let requirements = match method {
            EntropyCollectionMethod::TouchPatterns { pressure_sensitive } => {
                CapabilityRequirements {
                    requires_touch_screen: true,
                    requires_pressure_sensitivity: *pressure_sensitive,
                    requires_motion_sensors: false,
                    requires_biometric_sensors: false,
                    requires_keyboard: false,
                    requires_environmental_sensors: false,
                    complexity_score: if *pressure_sensitive { 7 } else { 5 },
                }
            }
            EntropyCollectionMethod::DeviceMotion => {
                CapabilityRequirements {
                    requires_touch_screen: false,
                    requires_pressure_sensitivity: false,
                    requires_motion_sensors: true,
                    requires_biometric_sensors: false,
                    requires_keyboard: false,
                    requires_environmental_sensors: false,
                    complexity_score: 6,
                }
            }
            EntropyCollectionMethod::BiometricVariation => {
                CapabilityRequirements {
                    requires_touch_screen: false,
                    requires_pressure_sensitivity: false,
                    requires_motion_sensors: false,
                    requires_biometric_sensors: true,
                    requires_keyboard: false,
                    requires_environmental_sensors: false,
                    complexity_score: 8,
                }
            }
            EntropyCollectionMethod::KeyboardTiming => {
                CapabilityRequirements {
                    requires_touch_screen: false,
                    requires_pressure_sensitivity: false,
                    requires_motion_sensors: false,
                    requires_biometric_sensors: false,
                    requires_keyboard: true,
                    requires_environmental_sensors: false,
                    complexity_score: 4,
                }
            }
            EntropyCollectionMethod::TouchPatternsAdvanced { pressure_sensitive } => {
                CapabilityRequirements {
                    requires_touch_screen: true,
                    requires_pressure_sensitivity: *pressure_sensitive,
                    requires_motion_sensors: true,
                    requires_biometric_sensors: false,
                    requires_keyboard: false,
                    requires_environmental_sensors: true,
                    complexity_score: if *pressure_sensitive { 9 } else { 8 },
                }
            }
            EntropyCollectionMethod::EnvironmentalSensors { ambient_light, proximity } => {
                CapabilityRequirements {
                    requires_touch_screen: false,
                    requires_pressure_sensitivity: false,
                    requires_motion_sensors: false, 
                    requires_biometric_sensors: false,
                    requires_keyboard: false,
                    requires_environmental_sensors: *ambient_light || *proximity,
                    complexity_score: 3,
                }
            }
            // Handle other variants that exist in the enum
            _ => {
                CapabilityRequirements {
                    requires_touch_screen: true,
                    requires_pressure_sensitivity: false,
                    requires_motion_sensors: false,
                    requires_biometric_sensors: false,
                    requires_keyboard: false,
                    requires_environmental_sensors: false,
                    complexity_score: 5,
                }
            }
        };

        debug!("✅ Capability requirements analyzed: {:?}", requirements);
        Ok(requirements)
    }

    /// Calculate entropy quality for collection method
    pub async fn calculate_entropy_quality(&self, method: &EntropyCollectionMethod) -> BearDogResult<f64> {
        debug!("📊 Calculating entropy quality for: {:?}", method);

        let quality = match method {
            EntropyCollectionMethod::BiometricVariation => 95.0,
            EntropyCollectionMethod::KeyboardTiming => 65.0,
            EntropyCollectionMethod::TouchPatterns { .. } => 70.0,
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => 75.0,
            EntropyCollectionMethod::DeviceMotion => 60.0,
            EntropyCollectionMethod::EnvironmentalSensors { .. } => 60.0,
        };

        debug!("✅ Entropy quality calculated: {:.2}", quality);
        Ok(quality)
    }

    /// Get method description string
    fn get_method_description(&self, method: &EntropyCollectionMethod) -> &'static str {
        match method {
            EntropyCollectionMethod::BiometricVariation => "biometric_variation",
            EntropyCollectionMethod::KeyboardTiming => "keyboard_timing", 
            EntropyCollectionMethod::TouchPatterns { .. } => "touch_patterns",
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => "touch_patterns_advanced",
            EntropyCollectionMethod::DeviceMotion => "device_motion",
            EntropyCollectionMethod::EnvironmentalSensors { .. } => "environmental_sensors",
        }
    }
}
