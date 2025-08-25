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


/// Human Entropy Classification System
///
/// This module classifies HSMs based on their human entropy capabilities, specifically
/// for ephemeral seed creation, which influences tier elevation. HSMs that can create
/// human entropy ephemeral seeds get elevated to premium tiers.

use super::*;
use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, error, info, warn};
// Import canonical types from beardog-types
use beardog_types::canonical::hsm::capabilities::HumanEntropyCapabilities;
use beardog_types::CapabilityRequirements;
/// Human entropy classifier for HSM tier elevation
#[derive(Debug)]
pub struct HumanEntropyClassifier {
    quality_assessor: EntropyQualityAssessor,
    method_evaluator: HumanEntropyMethodEvaluator,
    tier_elevation_criteria: TierElevationCriteria,
}
impl HumanEntropyClassifier {}


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
    /// Assess the quality and capabilities of human entropy features
    pub async fn assess_human_entropy_capabilities(
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
        // Evaluate entropy collection methods
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
            .await?;
        // Evaluate collection efficiency
        let collection_efficiency = self
            .quality_assessor
            .evaluate_collection_efficiency(entropy_caps)
        // Assess biometric integration
        let biometric_integration = self
            .assess_biometric_integration(entropy_caps)
        // Verification strength (using supports_ephemeral_seeds as proxy)
        let verification_strength = if entropy_caps.supports_ephemeral_seeds {
            0.8
            0.3
        };
        // Calculate overall quality rating
        let quality_rating = self
            .calculate_quality_rating(entropy_caps)
        // Calculate overall entropy score
        let overall_score = self
            .calculate_overall_entropy_score(
                collection_method_score,
                collection_efficiency,
                biometric_integration,
                verification_strength,
                quality_rating,
                entropy_caps,
        // Determine recommended tier
        let recommended_tier = self.determine_recommended_tier(overall_score).await?;
        // Check if meets tier elevation criteria
        let meets_criteria = self
            .evaluate_tier_elevation(&HumanEntropyAssessment {
                overall_score,
                supports_ephemeral_seeds: entropy_caps.supports_ephemeral_seeds,
                quality_rating: quality_rating.clone(),
                biometric_integration_score: biometric_integration,
                real_time_capability: entropy_caps.supports_ephemeral_seeds, // Use available field
                method_diversity_score: collection_method_score,
                temporal_analysis_capability: entropy_caps.supports_human_entropy, // Use available field
                meets_tier_elevation_criteria: true, // Temporary for recursion avoidance
                recommended_tier,
            })
        // Final classification result
        Ok(HumanEntropyAssessment {
            overall_score,
            supports_ephemeral_seeds: entropy_caps.supports_ephemeral_seeds,
            quality_rating,
            collection_efficiency,
            biometric_integration_score: biometric_integration,
            real_time_capability: entropy_caps.supports_ephemeral_seeds, // Use available field
            verification_strength,
            method_diversity_score: collection_method_score,
            temporal_analysis_capability: entropy_caps.supports_human_entropy, // Use available field
            meets_tier_elevation_criteria: meets_criteria,
            recommended_tier,
    /// Evaluate if HSM meets criteria for tier elevation
    pub async fn evaluate_tier_elevation(
        assessment: &HumanEntropyAssessment,
        debug!("📊 Evaluating tier elevation criteria");
        // Must support ephemeral seeds
        if !assessment.supports_ephemeral_seeds {
            debug!("❌ No ephemeral seed support");
            return Ok(false);
        // Must meet minimum overall score
        if assessment.overall_score < self.tier_elevation_criteria.min_overall_score {
            debug!(
                "❌ Overall score too low: {} < {}",
                assessment.overall_score, self.tier_elevation_criteria.min_overall_score
            );
        // Must have sufficient method diversity
        if assessment.method_diversity_score < self.tier_elevation_criteria.min_method_diversity {
                "❌ Method diversity too low: {} < {}",
                assessment.method_diversity_score,
                self.tier_elevation_criteria.min_method_diversity
        // Must support real-time generation if required
        if self.tier_elevation_criteria.require_real_time && !assessment.real_time_capability {
            debug!("❌ Real-time capability required but not supported");
        // Must support entropy verification if required
        if self.tier_elevation_criteria.require_verification
            && assessment.verification_strength < 0.7
        {
                "❌ Verification strength insufficient: {}",
                assessment.verification_strength
        // Must meet quality rating threshold
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
            EntropyQualityRating::Unknown => Ok(false),
    async fn calculate_overall_entropy_score(
        method_diversity: f64,
        collection_efficiency: f64,
        biometric_integration: f64,
        verification_strength: f64,
        quality_rating: EntropyQualityRating,
        entropy_caps: &HumanEntropyCapabilities,
    ) -> BearDogResult<f64> {
        // Use available fields for scoring
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


    async fn determine_recommended_tier(&self, overall_score: f64) -> BearDogResult<HsmTier> {
        if overall_score >= 0.9 {
            Ok(HsmTier::HumanEntropyPremium)
        } else if overall_score >= 0.7 {
            Ok(HsmTier::HighSecurity)
        } else if overall_score >= 0.5 {
            Ok(HsmTier::CertifiedHardware)
        } else if overall_score >= 0.3 {
            Ok(HsmTier::BasicHardware)
            Ok(HsmTier::Software)
    /// Get ranked entropy collection methods by effectiveness
    pub async fn get_ranked_entropy_methods(
        methods: &[EntropyCollectionMethod],
    ) -> BearDogResult<Vec<(EntropyCollectionMethod, f64)>> {
        let mut ranked = Vec::new();
        for method in methods {
            let score = self
                .method_evaluator
                .evaluate_method_effectiveness(method)
                .await?;
            ranked.push((method.clone(), score));
        // Sort by score (highest first)
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        Ok(ranked)
    /// Update tier elevation criteria}


    pub fn update_criteria(&mut self, criteria: TierElevationCriteria) {
        self.tier_elevation_criteria = criteria;
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
/// Quality rating for entropy collection capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntropyQualityRating {
    /// No entropy available
    None,
    /// Low entropy quality (< 25%)
    Low,
    /// Medium entropy quality (25-50%)
    Medium,
    /// High entropy quality (50-75%)
    High,
    /// Premium entropy quality (75-90%)
    Premium,
    /// Excellent entropy quality (>= 90%)
    Excellent,
    /// Entropy quality assessment failed
    Failed,
    /// Entropy quality unknown or not assessed
    Unknown,
/// Criteria for tier elevation based on human entropy}


pub struct TierElevationCriteria {
    pub min_overall_score: f64,
    pub min_method_diversity: f64,
    pub require_real_time: bool,
    pub require_verification: bool,
    pub require_biometric_integration: bool,
    pub min_collection_efficiency: f64,
    pub policies: TierElevationPolicies,}


impl Default for TierElevationCriteria {}


    fn default() -> Self {
        Self {
            min_overall_score: 0.6,      // 60% minimum score
            min_method_diversity: 0.5,   // 50% method diversity
            require_real_time: false,    // Real-time not required by default
            require_verification: false, // Verification not required by default
            require_biometric_integration: false,
            min_collection_efficiency: 0.4,
            policies: TierElevationPolicies::default(),
/// Policies for different types of tier elevation
pub struct TierElevationPolicies {
    pub mobile_hsm_bonus: f64,          // Bonus for mobile HSMs
    pub hardware_hsm_bonus: f64,        // Bonus for hardware HSMs
    pub biometric_entropy_weight: f64,  // Weight for biometric entropy
    pub behavioral_entropy_weight: f64, // Weight for behavioral entropy
    pub temporal_entropy_weight: f64,   // Weight for temporal entropy}


impl Default for TierElevationPolicies {
            mobile_hsm_bonus: 0.1,           // 10% bonus for mobile
            hardware_hsm_bonus: 0.05,        // 5% bonus for hardware
            biometric_entropy_weight: 0.3,   // 30% weight
            behavioral_entropy_weight: 0.25, // 25% weight
            temporal_entropy_weight: 0.15,   // 15% weight
/// Entropy quality assessor}


pub struct EntropyQualityAssessor;
impl EntropyQualityAssessor {
        Ok(Self)
    pub async fn evaluate_collection_efficiency(
        let mut efficiency = 0.0;
        // Base efficiency from number of methods
        let method_count = entropy_caps.entropy_collection_methods.len() as f64;
        efficiency += (method_count / 8.0).min(0.4); // Max 40% from method count
        // Real-time generation bonus
        if entropy_caps.real_time_entropy_generation {
            efficiency += 0.2;
        // Quality assessment capability
        if entropy_caps.entropy_quality_assessment {
        // User interaction entropy
        if entropy_caps.user_interaction_entropy {
            efficiency += 0.1;
        // Temporal collection
        if entropy_caps.temporal_entropy_collection {
        Ok(efficiency.min(1.0))}


    pub async fn assess_biometric_integration(
        // Use available biometric entropy fields
        let biometric_score = if entropy_caps.supports_biometric_entropy {
            0.2
        Ok(biometric_score)
    pub async fn calculate_quality_rating(
    ) -> BearDogResult<EntropyQualityRating> {
        // Calculate quality rating based on available fields
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
        Ok(quality_score)
/// Human entropy method evaluator
pub struct HumanEntropyMethodEvaluator;
impl HumanEntropyMethodEvaluator {
    pub async fn evaluate_collection_methods(
        let mut total_score = 0.0;
        let mut method_count = 0;
        for method_str in methods {
            method_count += 1;
            // Convert string to score based on method type
            let method_score = match method_str.as_str() {
                "TouchPatterns" => 75.0,
                "TouchPatternsAdvanced" => 80.0,
                "BiometricVariation" => 85.0,
                "KeyboardTiming" => 70.0,
                "DeviceMotion" => 60.0,
                "EnvironmentalSensors" => 50.0,
                _ => 40.0, // Unknown method
            };
            total_score += method_score;
        let average_score = if method_count > 0 {
            total_score / method_count as f64
            0.0
        Ok(average_score / 100.0) // Normalize to 0.0-1.0
    pub async fn evaluate_method_effectiveness(
        method: &EntropyCollectionMethod,
        let base_score = match method {
            EntropyCollectionMethod::BiometricVariation => {
                // Template noise analysis improves biometric variation quality
                0.8 // Default score for biometric variation
            EntropyCollectionMethod::TouchPatterns { pressure_sensitive } => {
                if *pressure_sensitive {
                    0.8
                } else {
                    0.5
                }
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => {
                // Advanced touch patterns with timing analysis
                0.6 // Default score for advanced touch patterns
            EntropyCollectionMethod::DeviceMotion => {
                // Device motion with velocity analysis
                0.5 // Default score for device motion
            EntropyCollectionMethod::EnvironmentalSensors { .. } => {
                // Environmental sensors with quantum randomness and ambient noise
                0.85 // Default high score for environmental sensors
            EntropyCollectionMethod::KeyboardTiming => 68.0,
            EntropyCollectionMethod::TouchPatterns { .. } => 75.0,
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => 80.0,
            EntropyCollectionMethod::DeviceMotion => 60.0,
            EntropyCollectionMethod::EnvironmentalSensors { .. } => 50.0,
        Ok(base_score)
    fn get_method_category(&self, method: &EntropyCollectionMethod) -> &'static str {
        match method {
            EntropyCollectionMethod::BiometricVariation => "biometric_variation",
            EntropyCollectionMethod::TouchPatterns { .. } => "touch",
            EntropyCollectionMethod::KeyboardTiming => "keyboard_timing",
            EntropyCollectionMethod::DeviceMotion => "device_motion",
            EntropyCollectionMethod::EnvironmentalSensors { .. } => "environmental",
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => "touch_advanced",
    /// Analyze capability requirements for collection method
    pub async fn analyze_capability_requirements(
    ) -> BearDogResult<CapabilityRequirements> {
        debug!("🔍 Analyzing capability requirements for: {:?}", method);
        let requirements = match method {
            EntropyCollectionMethod::TouchPatterns { .. } => CapabilityRequirements::default(),
            EntropyCollectionMethod::DeviceMotion => CapabilityRequirements::default(),
            EntropyCollectionMethod::BiometricVariation => CapabilityRequirements::default(),
            EntropyCollectionMethod::KeyboardTiming => CapabilityRequirements::default(),
                CapabilityRequirements::default()
            _ => CapabilityRequirements::default(),
        Ok(requirements)
    pub fn new() -> Self {
        Self
    /// Calculate entropy quality for collection method}


    pub async fn calculate_entropy_quality(
        debug!("📊 Calculating entropy quality for: {:?}", method);
        let quality = match method {
            EntropyCollectionMethod::BiometricVariation => 95.0,
            EntropyCollectionMethod::KeyboardTiming => 65.0,
            EntropyCollectionMethod::TouchPatterns { .. } => 70.0,
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => 75.0,
            EntropyCollectionMethod::EnvironmentalSensors { .. } => 60.0,
        debug!("✅ Entropy quality calculated: {:.2}", quality);
        Ok(quality)
    /// Get method description string
    fn get_method_description(&self, method: &EntropyCollectionMethod) -> &'static str {
            EntropyCollectionMethod::TouchPatterns { .. } => "touch_patterns",
            EntropyCollectionMethod::TouchPatternsAdvanced { .. } => "touch_patterns_advanced",
            EntropyCollectionMethod::EnvironmentalSensors { .. } => "environmental_sensors",
