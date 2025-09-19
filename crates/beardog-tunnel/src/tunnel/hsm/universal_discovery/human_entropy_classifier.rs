

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::{DiscoveryHsmCapabilities, HumanEntropyCapabilities, HumanEntropyMethod};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::{debug, info, warn};

pub struct HumanEntropyClassifier {

    quality_assessor: EntropyQualityAssessor,

    method_evaluator: HumanEntropyMethodEvaluator,

    elevation_criteria: TierElevationCriteria,
}

pub struct EntropyQualityAssessor {

    min_entropy_bits: f64,

    scoring_algorithms: Vec<EntropyQualityAlgorithm>,

pub struct HumanEntropyMethodEvaluator {

    method_weights: HashMap<HumanEntropyMethod, f64>,

    quality_multipliers: HashMap<HumanEntropyMethod, f64>,

#[derive(Debug, Clone)]
    /// The min quality score value
    pub min_quality_score: f64,


    pub require_realtime: bool,

    /// Whether require_biometric is enabled
    pub require_biometric: bool,

    /// Whether require_quality_assessment is enabled
    pub require_quality_assessment: bool,

pub enum EntropyQualityAlgorithm {


    /// Represents shannon variant
    Shannon,


    /// Represents min entropy variant
    MinEntropy,


    /// Represents compression variant
    Compression,


    /// Represents statistical variant
    Statistical,


    /// Represents behavioral variant
    Behavioral,

pub struct HumanEntropyAssessment {

    /// Whether supports_ephemeral_seeds is enabled
    pub supports_ephemeral_seeds: bool,

    /// The quality score value
    pub quality_score: f64,

    /// Mapping of method scores
    pub method_scores: HashMap<HumanEntropyMethod, f64>,

    /// The collection efficiency value
    pub collection_efficiency: f64,


    pub realtime_capability: bool,

    /// The biometric integration quality value
    pub biometric_integration_quality: f64,

    /// Whether recommended_tier_elevation is enabled
    pub recommended_tier_elevation: bool,

    /// The assessed at value
    pub assessed_at: chrono::DateTime<chrono::Utc>,}
    pub assessed_at: chrono::DateTime<chrono::Utc>,}
    pub assessed_at: chrono::DateTime<chrono::Utc>,}

impl Default for TierElevationCriteria {}

    fn default(2,
            min_quality_score: 0.75,
            require_realtime: true,
            require_biometric: false,
            require_quality_assessment: true,
        }
    }
impl HumanEntropyClassifier {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new() -> Result<Self, BearDogError> {
        info!("🧠 Initializing Human Entropy Classifier");
        let quality_assessor = EntropyQualityAssessor::new()?;
        let method_evaluator = HumanEntropyMethodEvaluator::new()?;
        let elevation_criteria = TierElevationCriteria::default();
        Ok(Self {
            quality_assessor,
            method_evaluator,
            elevation_criteria,
        })

/// With Criteria operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates instance with criteria
    pub fn with_criteria(criteria: TierElevationCriteria) -> Result<Self, BearDogError> {
        info!("🧠 Initializing Human Entropy Classifier with custom criteria");
            elevation_criteria: criteria,

/// Classify Human Entropy Support operation.
    pub fn classify_human_entropy_support(&DiscoveryHsmCapabilities,
    ) -> Result<bool, BearDogError> {
        debug!("🧠 Classifying human entropy support");
        let assessment = self.assess_human_entropy_capabilities(capabilities)?;
        
        let supports_entropy = self.evaluate_tier_elevation(&assessment)?;
        if supports_entropy {
            info!("✅ HSM supports human entropy ephemeral seeds (quality: {:.2})", 
                 assessment.quality_score);
        } else {
            debug!("❌ HSM does not meet human entropy criteria (quality: {:.2})", 
                  assessment.quality_score);
        Ok(human_entropy.ephemeral_seed_creation,
            quality_score,
            method_scores,
            collection_efficiency,
            realtime_capability: human_entropy.realtime_entropy,
            biometric_integration_quality,
            recommended_tier_elevation: false, // Will be set below
            assessed_at: chrono::Utc::now(&HumanEntropyAssessment,
        debug!("🧠 Evaluating tier elevation criteria");

        if !assessment.supports_ephemeral_seeds {
            debug!("❌ No ephemeral seed creation support");
            return Ok({} < {}", 
                  assessment.method_scores.len({:.2} < {:.2}", 
                  assessment.quality_score, 
                  self.elevation_criteria.min_quality_score);

        if self.elevation_criteria.require_realtime && !assessment.realtime_capability {
            debug!("❌ Real-time entropy generation required but not available");

        if self.elevation_criteria.require_biometric && assessment.biometric_integration_quality < 0.5 {
            debug!("❌ Biometric integration required but insufficient quality: {:.2}", 
                  assessment.biometric_integration_quality);
        info!("✅ HSM qualifies for human entropy tier elevation");
        Ok(&HumanEntropyCapabilities,
    ) -> Result<f64, BearDogError> {
        let mut efficiency_score = 0.0;

        let method_diversity = human_entropy.collection_methods.len() as f64 / 8.0; // Max 8 methods
        efficiency_score += method_diversity * 0.4;

        if human_entropy.realtime_entropy {
            efficiency_score += 0.3;

        if human_entropy.entropy_quality_assessment {
            efficiency_score += 0.2;

        if human_entropy.behavioral_entropy {
            efficiency_score += 0.1;
        Ok(efficiency_score.min(1.0))


    fn assess_biometric_integration(
        let mut biometric_score = 0.0;
        if !human_entropy.biometric_entropy {
            return Ok(0.0);

        let biometric_methods = human_entropy.collection_methods.iter()
            .filter(|&method| matches!(method, 
                HumanEntropyMethod::BiometricVariations |
                HumanEntropyMethod::VoicePatterns |
                HumanEntropyMethod::TouchPatterns
            ))
            .count(Vec<(HumanEntropyMethod, f64)> = assessment.method_scores
            .into_iter()
            .collect();
        ranked_methods.sort_by(|a, b| b.1.partial_cmp(&a.1).map_err(|e| {
    tracing::error!("Operation failed: {e:?}");
    beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
})?);
        Ok(ranked_methods)

/// Update Criteria operation.
    /// Updates criteria
    /// Updates criteria
    pub fn update_criteria(&mut self, criteria: TierElevationCriteria) {
        info!("🧠 Updating human entropy tier elevation criteria");
        self.elevation_criteria = criteria;
impl EntropyQualityAssessor {

            min_entropy_bits: 128.0,
            scoring_algorithms: vec![
                EntropyQualityAlgorithm::Shannon,
                EntropyQualityAlgorithm::MinEntropy,
                EntropyQualityAlgorithm::Behavioral,
            ],

/// Calculate Quality Score operation.
    pub fn calculate_quality_score(&HashMap<HumanEntropyMethod, f64>,
        let mut total_score = 0.0;
        let mut weight_sum = 0.0;

        for (method, score) in method_scores {
            let weight = self.get_method_weight(method);
            total_score += score * weight;
            weight_sum += weight;

        let base_score = if weight_sum > 0.0 {
            total_score / weight_sum
            0.0

        let mut final_score = base_score;

            final_score *= 1.2;

            final_score *= 1.1;

        if human_entropy.biometric_entropy {
        Ok(final_score.min(1.0))

    /// Gets method_weight
    fn get_method_weight(&self, method: &HumanEntropyMethod) -> f64 {
        match method {
            HumanEntropyMethod::BiometricVariations => 0.9,
            HumanEntropyMethod::KeystrokeDynamics => 0.8,
            HumanEntropyMethod::MouseMovement => 0.7,
            HumanEntropyMethod::TouchPatterns => 0.8,
            HumanEntropyMethod::VoicePatterns => 0.8,
            HumanEntropyMethod::BehavioralTiming => 0.7,
            HumanEntropyMethod::CameraEntropy => 0.6,
            HumanEntropyMethod::CustomInput => 0.5,}

impl HumanEntropyMethodEvaluator {

        let mut method_weights = HashMap::with_capacity(16);
        let mut quality_multipliers = HashMap::with_capacity(16);

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
            method_weights,
            quality_multipliers,

/// Evaluate Entropy Methods operation.
    pub fn evaluate_entropy_methods(&[HumanEntropyMethod],
    ) -> Result<HashMap<HumanEntropyMethod, f64, BearDogError>> {
        let mut scores = HashMap::with_capacity(16);
        for method in methods {
            let base_weight = self.method_weights.get(method).unwrap_or(&0.5);
            let quality_multiplier = self.quality_multipliers.get(method).unwrap_or(&1.0);
            
            let score = base_weight * quality_multiplier;
            scores.insert(method.clone(), score);
        Ok(scores)

pub mod policies {
    use super::*;

/// High Security Policy operation.
    pub fn high_security_policy(3,
            min_quality_score: 0.85,
            require_biometric: true,

/// Balanced Policy operation.
    pub fn balanced_policy(1,
            min_quality_score: 0.5,
            require_realtime: false,
            require_quality_assessment: false,
} 
