

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
pub struct TierElevationCriteria {

    pub min_entropy_methods: usize,

    pub min_quality_score: f64,

    pub require_realtime: bool,

    pub require_biometric: bool,

    pub require_quality_assessment: bool,

pub enum EntropyQualityAlgorithm {

    Shannon,

    MinEntropy,

    Compression,

    Statistical,

    Behavioral,

pub struct HumanEntropyAssessment {

    pub supports_ephemeral_seeds: bool,

    pub quality_score: f64,

    pub method_scores: HashMap<HumanEntropyMethod, f64>,

    pub collection_efficiency: f64,

    pub realtime_capability: bool,

    pub biometric_integration_quality: f64,

    pub recommended_tier_elevation: bool,

    pub assessed_at: chrono::DateTime<chrono::Utc>,}

impl Default for TierElevationCriteria {}

    fn default() -> Self {
        Self {
            min_entropy_methods: 2,
            min_quality_score: 0.75,
            require_realtime: true,
            require_biometric: false,
            require_quality_assessment: true,
        }
    }
impl HumanEntropyClassifier {

    pub async fn new() -> Result<Self, BearDogError> {
        info!("🧠 Initializing Human Entropy Classifier");
        let quality_assessor = EntropyQualityAssessor::new().await?;
        let method_evaluator = HumanEntropyMethodEvaluator::new().await?;
        let elevation_criteria = TierElevationCriteria::default();
        Ok(Self {
            quality_assessor,
            method_evaluator,
            elevation_criteria,
        })

    pub async fn with_criteria(criteria: TierElevationCriteria) -> Result<Self, BearDogError> {
        info!("🧠 Initializing Human Entropy Classifier with custom criteria");
            elevation_criteria: criteria,

    pub async fn classify_human_entropy_support(
        &self,
        capabilities: &DiscoveryHsmCapabilities,
    ) -> Result<bool, BearDogError> {
        debug!("🧠 Classifying human entropy support");
        let assessment = self.assess_human_entropy_capabilities(capabilities).await?;
        
        let supports_entropy = self.evaluate_tier_elevation(&assessment).await?;
        if supports_entropy {
            info!("✅ HSM supports human entropy ephemeral seeds (quality: {:.2})", 
                 assessment.quality_score);
        } else {
            debug!("❌ HSM does not meet human entropy criteria (quality: {:.2})", 
                  assessment.quality_score);
        Ok(supports_entropy)

    pub async fn assess_human_entropy_capabilities(
    ) -> Result<HumanEntropyAssessment, BearDogError> {
        debug!("🧠 Performing comprehensive human entropy assessment");
        let human_entropy = &capabilities.human_entropy;

        let method_scores = self.method_evaluator
            .evaluate_entropy_methods(&human_entropy.collection_methods).await?;

        let quality_score = self.quality_assessor
            .calculate_quality_score(human_entropy, &method_scores).await?;

        let collection_efficiency = self.evaluate_collection_efficiency(human_entropy).await?;

        let biometric_integration_quality = self.assess_biometric_integration(human_entropy).await?;

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

    async fn evaluate_tier_elevation(
        assessment: &HumanEntropyAssessment,
        debug!("🧠 Evaluating tier elevation criteria");

        if !assessment.supports_ephemeral_seeds {
            debug!("❌ No ephemeral seed creation support");
            return Ok(false);

        if assessment.method_scores.len() < self.elevation_criteria.min_entropy_methods {
            debug!("❌ Insufficient entropy methods: {} < {}", 
                  assessment.method_scores.len(), 
                  self.elevation_criteria.min_entropy_methods);

        if assessment.quality_score < self.elevation_criteria.min_quality_score {
            debug!("❌ Insufficient quality score: {:.2} < {:.2}", 
                  assessment.quality_score, 
                  self.elevation_criteria.min_quality_score);

        if self.elevation_criteria.require_realtime && !assessment.realtime_capability {
            debug!("❌ Real-time entropy generation required but not available");

        if self.elevation_criteria.require_biometric && assessment.biometric_integration_quality < 0.5 {
            debug!("❌ Biometric integration required but insufficient quality: {:.2}", 
                  assessment.biometric_integration_quality);
        info!("✅ HSM qualifies for human entropy tier elevation");
        Ok(true)

    async fn evaluate_collection_efficiency(
        human_entropy: &HumanEntropyCapabilities,
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

    async fn assess_biometric_integration(
        let mut biometric_score = 0.0;
        if !human_entropy.biometric_entropy {
            return Ok(0.0);

        let biometric_methods = human_entropy.collection_methods.iter()
            .filter(|&method| matches!(method, 
                HumanEntropyMethod::BiometricVariations |
                HumanEntropyMethod::VoicePatterns |
                HumanEntropyMethod::TouchPatterns
            ))
            .count();
        biometric_score += (biometric_methods as f64 / 3.0) * 0.7;

            biometric_score += 0.3;
        Ok(biometric_score.min(1.0))

    pub async fn get_ranked_entropy_methods(
    ) -> Result<Vec<(HumanEntropyMethod, f64)>> {
        let mut ranked_methods: Vec<(HumanEntropyMethod, f64)> = assessment.method_scores
            .into_iter()
            .collect();
        ranked_methods.sort_by(|a, b| b.1.partial_cmp(&a.1).map_err(|e| {
    tracing::error!("Operation failed: {e:?}");
    beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
})?);
        Ok(ranked_methods)

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

    pub async fn calculate_quality_score(
        method_scores: &HashMap<HumanEntropyMethod, f64>,
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

    pub async fn evaluate_entropy_methods(
        methods: &[HumanEntropyMethod],
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

    pub fn high_security_policy() -> TierElevationCriteria {
        TierElevationCriteria {
            min_entropy_methods: 3,
            min_quality_score: 0.85,
            require_biometric: true,

    pub fn balanced_policy() -> TierElevationCriteria {

    pub fn permissive_policy() -> TierElevationCriteria {
            min_entropy_methods: 1,
            min_quality_score: 0.5,
            require_realtime: false,
            require_quality_assessment: false,
} 
