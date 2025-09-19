

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::*;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, error, info, warn};

use beardog_types::canonical::hsm::capabilities::HumanEntropyCapabilities;
use beardog_types::CapabilityRequirements;

#[derive(Debug, Clone)]
    method_evaluator: HumanEntropyMethodEvaluator,
    tier_elevation_criteria: TierElevationCriteria,
}
impl HumanEntropyClassifier {}

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            quality_assessor: EntropyQualityAssessor::new()?,
            method_evaluator: HumanEntropyMethodEvaluator::new()?,
            tier_elevation_criteria: TierElevationCriteria::default(&HsmCapabilities,
    ) -> Result<bool, BearDogError> {
        debug!("🧠 Classifying human entropy support");
        let assessment = self.assess_human_entropy_capabilities(0.0,
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

        let collection_method_score = self
            .method_evaluator
            .evaluate_collection_methods(
                &entropy_caps
                    .available_methods
                    .iter()
                    .map(|method| match method.as_str() {
                        "TouchPatterns" => EntropyCollectionMethod::TouchPatterns {
                            pressure_sensitive: false,
                        },
                        "TouchPatternsAdvanced" => EntropyCollectionMethod::TouchPatternsAdvanced {
                            pressure_sensitive: true,
                        "BiometricVariation" => EntropyCollectionMethod::BiometricVariation,
                        "KeyboardTiming" => EntropyCollectionMethod::KeyboardTiming,
                        "DeviceMotion" => EntropyCollectionMethod::DeviceMotion,
                        _ => EntropyCollectionMethod::EnvironmentalSensors {
                            sensor_types: vec![],
                    })
                    .collect::<Vec<_>>(),
            )
            ?;

        let collection_efficiency = self
            .quality_assessor
            .evaluate_collection_efficiency(entropy_caps.supports_ephemeral_seeds,
                quality_rating: quality_rating.clone(biometric_integration,
                real_time_capability: entropy_caps.supports_ephemeral_seeds, // Use available field
                method_diversity_score: collection_method_score,
                temporal_analysis_capability: entropy_caps.supports_human_entropy, // Use available field
                meets_tier_elevation_criteria: true, // Temporary for recursion avoidance
                recommended_tier,
            })

        Ok(entropy_caps.supports_ephemeral_seeds,
            quality_rating,
            collection_efficiency,
            biometric_integration_score: biometric_integration,
            real_time_capability: entropy_caps.supports_ephemeral_seeds, // Use available field
            verification_strength,
            method_diversity_score: collection_method_score,
            temporal_analysis_capability: entropy_caps.supports_human_entropy, // Use available field
            meets_tier_elevation_criteria: meets_criteria,
            recommended_tier,

/// Evaluate Tier Elevation operation.
    pub fn evaluate_tier_elevation(&HumanEntropyAssessment,
        debug!("📊 Evaluating tier elevation criteria");

        if !assessment.supports_ephemeral_seeds {
            debug!("❌ No ephemeral seed support");
            return Ok({} < {}",
                assessment.overall_score, self.tier_elevation_criteria.min_overall_score
            );

        if assessment.method_diversity_score < self.tier_elevation_criteria.min_method_diversity {
                "❌ Method diversity too low: {} < {}",
                assessment.method_diversity_score,
                self.tier_elevation_criteria.min_method_diversity

        if self.tier_elevation_criteria.require_real_time && !assessment.real_time_capability {
            debug!("❌ Real-time capability required but not supported");

        if self.tier_elevation_criteria.require_verification
            && assessment.verification_strength < 0.7
        {
                "❌ Verification strength insufficient: {}",
                assessment.verification_strength

        match assessment.quality_rating {
            EntropyQualityRating::None | EntropyQualityRating::Low => {
                debug!("❌ Quality rating too low: {:?}", assessment.quality_rating);
                Ok(false)
            }
            EntropyQualityRating::Medium => {
                Ok(assessment.overall_score >= 0.8) // Higher bar for medium quality
            EntropyQualityRating::High | EntropyQualityRating::Premium => Ok(true),
            EntropyQualityRating::Excellent
            | EntropyQualityRating::Good
            | EntropyQualityRating::Fair
            | EntropyQualityRating::Poor
            | EntropyQualityRating::Failed => Ok(false),
            EntropyQualityRating::Unknown => Ok(f64,
        collection_efficiency: f64,
        biometric_integration: f64,
        verification_strength: f64,
        quality_rating: EntropyQualityRating,
        entropy_caps: &HumanEntropyCapabilities,
    ) -> Result<f64, BearDogError> {

        let mut score: f64 = 0.0;
        if entropy_caps.supports_ephemeral_seeds {
            score += 0.3;
        if entropy_caps.entropy_quality_score > 0.0 {
            score += 0.2;
        if entropy_caps.supports_biometric_entropy {
        if entropy_caps.supports_behavioral_entropy {
        if !entropy_caps.entropy_collection_methods.is_empty() {
            score += 0.1;
        Ok(score.min(1.0))}


    fn determine_recommended_tier(&self, overall_score: f64) -> Result<HsmTier, BearDogError> {
        if overall_score >= 0.9 {
            Ok(HsmTier::HumanEntropyPremium)
        } else if overall_score >= 0.7 {
            Ok(HsmTier::HighSecurity)
        } else if overall_score >= 0.5 {
            Ok(HsmTier::CertifiedHardware)
        } else if overall_score >= 0.3 {
            Ok(HsmTier::BasicHardware)
            Ok(HsmTier::Software)

/// Get Ranked Entropy Methods operation.
    /// Gets ranked_entropy_methods
    /// Gets ranked_entropy_methods
    pub fn get_ranked_entropy_methods(&[EntropyCollectionMethod],
    ) -> Result<Vec<(EntropyCollectionMethod, f64)>> {
        let mut ranked = Vec::new();
        for method in methods {
            let score = self
                .method_evaluator
                .evaluate_method_effectiveness(method)
                ?;
            ranked.push((method.clone(), score));

        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        Ok(ranked)

/// Update Criteria operation.
    /// Updates criteria
    /// Updates criteria
    pub fn update_criteria(&mut self, criteria: TierElevationCriteria) {
        self.tier_elevation_criteria = criteria;

#[derive(Debug, Clone)]
    /// Whether supports_ephemeral_seeds is enabled
    pub supports_ephemeral_seeds: bool,
    /// The quality rating value
    pub quality_rating: EntropyQualityRating,
    /// The collection efficiency value
    pub collection_efficiency: f64,
    /// The biometric integration score value
    pub biometric_integration_score: f64,
    pub real_time_capability: bool,
    /// The verification strength value
    pub verification_strength: f64,
    /// The method diversity score value
    pub method_diversity_score: f64,
    /// Whether temporal_analysis_capability is enabled
    pub temporal_analysis_capability: bool,
    /// Whether meets_tier_elevation_criteria is enabled
    pub meets_tier_elevation_criteria: bool,
    /// The recommended tier value
    pub recommended_tier: HsmTier,

#[derive(Debug, Clone)]
    /// The min method diversity value
    pub min_method_diversity: f64,
    pub require_real_time: bool,
    /// Whether require_verification is enabled
    pub require_verification: bool,
    /// Whether require_biometric_integration is enabled
    pub require_biometric_integration: bool,
    /// The min collection efficiency value
    pub min_collection_efficiency: f64,
    /// The policies value
    pub policies: TierElevationPolicies,}

impl Default for TierElevationCriteria {}

    fn default(0.6,      // 60% minimum score
            min_method_diversity: 0.5,   // 50% method diversity
            require_real_time: false,    // Real-time not required by default
            require_verification: false, // Verification not required by default
            require_biometric_integration: false,
            min_collection_efficiency: 0.4,
            policies: TierElevationPolicies::default(f64,          // Bonus for mobile HSMs
    /// The hardware hsm bonus value
    pub hardware_hsm_bonus: f64,        // Bonus for hardware HSMs
    /// The biometric entropy weight value
    pub biometric_entropy_weight: f64,  // Weight for biometric entropy
    /// The behavioral entropy weight value
    pub behavioral_entropy_weight: f64, // Weight for behavioral entropy
    /// The temporal entropy weight value
    pub temporal_entropy_weight: f64,   // Weight for temporal entropy}

impl Default for TierElevationPolicies {
            mobile_hsm_bonus: 0.1,           // 10% bonus for mobile
            hardware_hsm_bonus: 0.05,        // 5% bonus for hardware
            biometric_entropy_weight: 0.3,   // 30% weight
            behavioral_entropy_weight: 0.25, // 25% weight
            temporal_entropy_weight: 0.15,   // 15% weight

pub struct EntropyQualityAssessor;
impl EntropyQualityAssessor {
        Ok(Self)
/// Evaluate Collection Efficiency operation.
    pub fn evaluate_collection_efficiency(
        let mut efficiency = 0.0;

        let method_count = entropy_caps.entropy_collection_methods.len() as f64;
        efficiency += (method_count / 8.0).min(0.4); // Max 40% from method count

        if entropy_caps.real_time_entropy_generation {
            efficiency += 0.2;

        if entropy_caps.entropy_quality_assessment {

        if entropy_caps.user_interaction_entropy {
            efficiency += 0.1;

        if entropy_caps.temporal_entropy_collection {
        Ok(efficiency.min(1.0))}

/// Assess Biometric Integration operation.
    pub fn assess_biometric_integration(

        let biometric_score = if entropy_caps.supports_biometric_entropy {
            0.2
        Ok(biometric_score)
/// Calculate Quality Rating operation.
    pub fn calculate_quality_rating(
    ) -> Result<EntropyQualityRating, BearDogError> {

        let quality_score = if entropy_caps.supports_ephemeral_seeds {
            if entropy_caps.supports_biometric_entropy && entropy_caps.supports_behavioral_entropy {
                EntropyQualityRating::Excellent
            } else if entropy_caps.supports_biometric_entropy
                || entropy_caps.supports_behavioral_entropy
            {
                EntropyQualityRating::Good
            } else {
                EntropyQualityRating::Fair
        } else if entropy_caps.entropy_quality_score > 0.5 {
            EntropyQualityRating::Fair
            EntropyQualityRating::Poor
        Ok(&EntropyCollectionMethod,
        let base_score = match method {
            EntropyCollectionMethod::BiometricVariation => {

                0.8 // Default score for biometric variation
            EntropyCollectionMethod::TouchPatterns { pressure_sensitive } => {
                if *pressure_sensitive {
                    0.8
                } else {
                    0.5
                }
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => {

                0.6 // Default score for advanced touch patterns
            EntropyCollectionMethod::DeviceMotion => {

                0.5 // Default score for device motion
            EntropyCollectionMethod::EnvironmentalSensors { .. } => {

                0.85 // Default high score for environmental sensors
            EntropyCollectionMethod::KeyboardTiming => 68.0,
            EntropyCollectionMethod::TouchPatterns { .. } => 75.0,
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => 80.0,
            EntropyCollectionMethod::DeviceMotion => 60.0,
            EntropyCollectionMethod::EnvironmentalSensors { .. } => 50.0,
        Ok(base_score)
    /// Gets method_category
    fn get_method_category(&self, method: &EntropyCollectionMethod) -> &'static str {
        match method {
            EntropyCollectionMethod::BiometricVariation => "biometric_variation",
            EntropyCollectionMethod::TouchPatterns { .. } => "touch",
            EntropyCollectionMethod::KeyboardTiming => "keyboard_timing",
            EntropyCollectionMethod::DeviceMotion => "device_motion",
            EntropyCollectionMethod::EnvironmentalSensors { .. } => "environmental",
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => "touch_advanced",

/// Analyze Capability Requirements operation.
    pub fn analyze_capability_requirements({:?}", method);
        let requirements = match method {
            EntropyCollectionMethod::TouchPatterns { .. } => CapabilityRequirements::default(),
            EntropyCollectionMethod::DeviceMotion => CapabilityRequirements::default(),
            EntropyCollectionMethod::BiometricVariation => CapabilityRequirements::default(),
            EntropyCollectionMethod::KeyboardTiming => CapabilityRequirements::default(),
                CapabilityRequirements::default()
            _ => CapabilityRequirements::default({:?}", method);
        let quality = match method {
            EntropyCollectionMethod::BiometricVariation => 95.0,
            EntropyCollectionMethod::KeyboardTiming => 65.0,
            EntropyCollectionMethod::TouchPatterns { .. } => 70.0,
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => 75.0,
            EntropyCollectionMethod::EnvironmentalSensors { .. } => 60.0,
        debug!("✅ Entropy quality calculated: {:.2}", quality);
        Ok(quality)

    /// Gets method_description
    fn get_method_description(&self, method: &EntropyCollectionMethod) -> &'static str {
            EntropyCollectionMethod::TouchPatterns { .. } => "touch_patterns",
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => "touch_patterns_advanced",
            EntropyCollectionMethod::EnvironmentalSensors { .. } => "environmental_sensors",
