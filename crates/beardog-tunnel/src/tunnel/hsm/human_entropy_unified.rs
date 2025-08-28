

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tracing::{debug, info};
use super::unified_provider::{HsmTier, HumanEntropyMethod, UnifiedHumanEntropyCapabilities};

pub struct UnifiedHumanEntropyClassifier {

    quality_assessor: EntropyQualityAssessor,

    method_evaluator: HumanEntropyMethodEvaluator,

    tier_criteria: TierElevationCriteria,
}
impl UnifiedHumanEntropyClassifier {

    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            quality_assessor: EntropyQualityAssessor::new()?,
            method_evaluator: HumanEntropyMethodEvaluator::new()?,
            tier_criteria: TierElevationCriteria::default(),
        })
    }

    pub fn with_criteria(criteria: TierElevationCriteria) -> Result<Self, BearDogError> {
            tier_criteria: criteria,

    pub async fn classify_for_tier_elevation(
        &self,
        capabilities: &UnifiedHumanEntropyCapabilities,
    ) -> Result<HumanEntropyClassification, BearDogError> {
        info!("🧠 Classifying `HSM` for human entropy tier elevation");

        let assessment = self.perform_comprehensive_assessment(capabilities).await?;

        let qualifies_for_elevation = self.evaluate_tier_elevation(&assessment).await?;

        let recommended_tier = if qualifies_for_elevation {
            self.calculate_recommended_tier(&assessment).await?
        } else {
            HsmTier::BasicHardware
        };
        let classification = HumanEntropyClassification {
            qualifies_for_elevation,
            recommended_tier,
            assessment: assessment.clone(),
            classified_at: Utc::now(),
            classification_confidence: self.calculate_confidence(&assessment).await?,
        if qualifies_for_elevation {
            info!(
                "✅ `HSM` qualifies for human entropy tier elevation to {:?}",
                recommended_tier
            );
            info!("❌ `HSM` does not meet human entropy tier elevation criteria");
        }
        Ok(classification)

    async fn perform_comprehensive_assessment(
    ) -> Result<HumanEntropyAssessment, BearDogError> {
        debug!("🔍 Performing comprehensive human entropy assessment");

        let method_scores = self
            .method_evaluator
            .evaluate_methods(&capabilities.collection_methods)
            .await?;

        let quality_score = self
            .quality_assessor
            .assess_capability_quality(capabilities)

        let realtime_score = self.evaluate_realtime_capabilities(capabilities).await?;

        let biometric_score = self.evaluate_biometric_integration(capabilities).await?;

        let overall_score = self
            .calculate_overall_score(
                &method_scores,
                quality_score,
                realtime_score,
                biometric_score,
            )
        Ok(HumanEntropyAssessment {
            supports_ephemeral_seeds: capabilities.supports_ephemeral_seeds,
            method_scores,
            quality_score,
            realtime_score,
            biometric_score,
            overall_score,
            entropy_rate: capabilities.max_collection_rate,
            min_entropy_bits: capabilities.min_entropy_bits,
            assessed_at: Utc::now(),

    async fn evaluate_tier_elevation(
        assessment: &HumanEntropyAssessment,
    ) -> Result<bool, BearDogError> {
        debug!("📊 Evaluating tier elevation eligibility");

        if !assessment.supports_ephemeral_seeds {
            debug!("❌ `HSM` does not support ephemeral seed creation");
            return Ok(false);

        if assessment.method_scores.len() < self.tier_criteria.min_entropy_methods {
            debug!(
                "❌ Insufficient entropy methods: {} < {}",
                assessment.method_scores.len(),
                self.tier_criteria.min_entropy_methods

        if assessment.overall_score < self.tier_criteria.min_overall_score {
                "❌ Insufficient overall score: {} < {}",
                assessment.overall_score, self.tier_criteria.min_overall_score

        if self.tier_criteria.require_realtime && assessment.realtime_score < 0.7 {
            debug!("❌ Real-time capability requirement not met");

        if self.tier_criteria.require_biometric && assessment.biometric_score < 0.5 {
            debug!("❌ Biometric integration requirement not met");

        if assessment.min_entropy_bits < self.tier_criteria.min_entropy_bits {
                "❌ Insufficient entropy bits: {} < {}",
                assessment.min_entropy_bits, self.tier_criteria.min_entropy_bits
        debug!("✅ `HSM` meets all tier elevation criteria");
        Ok(true)

    async fn calculate_recommended_tier(
    ) -> Result<HsmTier, BearDogError> {
        let score = assessment.overall_score;
        let tier = if score >= 0.95
            && assessment.biometric_score >= 0.8
            && assessment.realtime_score >= 0.9
        {
            HsmTier::Premium
        } else if score >= 0.85 && assessment.method_scores.len() >= 3 {
            HsmTier::EnhancedHardware
        } else if score >= 0.7 && assessment.supports_ephemeral_seeds {
            HsmTier::Software
        debug!("📊 Recommended tier: {:?} (score: {:.3})", tier, score);
        Ok(tier)

    async fn calculate_confidence(
    ) -> Result<f64, BearDogError> {
        let mut confidence = 0.0;

        confidence += assessment.overall_score * 0.4;

        confidence += (assessment.method_scores.len() as f64 * 0.1).min(0.3);

        confidence += assessment.quality_score * 0.2;

        confidence += assessment.realtime_score * 0.1;
        Ok(confidence.min(1.0))}

    async fn evaluate_realtime_capabilities(
        let mut score: f32 = 0.0;
        if capabilities.realtime_entropy {
            score += 0.5;

        if capabilities.max_collection_rate > 1000.0 {
            score += 0.3;
        } else if capabilities.max_collection_rate > 100.0 {
            score += 0.2;
        } else if capabilities.max_collection_rate > 10.0 {
            score += 0.1;

        if capabilities.quality_assessment {
        Ok(score.min(1.0).into())
    async fn evaluate_biometric_integration(
        let mut score = 0.0;
        if capabilities.biometric_integration {
            score += 0.6;

        let biometric_methods = capabilities
            .collection_methods
            .iter()
            .filter(|method| matches!(method, HumanEntropyMethod::Biometric))
            .count();
        score += (biometric_methods as f64 * 0.2).min(0.4);
        Ok(score.min(1.0))}

    async fn calculate_overall_score(
        method_scores: &HashMap<HumanEntropyMethod, f64>,
        quality_score: f64,
        realtime_score: f64,
        biometric_score: f64,
        let method_avg = if method_scores.is_empty() {
            0.0
            method_scores.values().sum::<f64>() / method_scores.len() as f64
        let overall = (method_avg * 0.4)
            + (quality_score * 0.3)
            + (realtime_score * 0.2)
            + (biometric_score * 0.1);
        Ok(overall.min(1.0))

#[derive(Debug, Clone)]
pub struct HumanEntropyAssessment {

    pub supports_ephemeral_seeds: bool,

    pub method_scores: HashMap<HumanEntropyMethod, f64>,

    pub quality_score: f64,

    pub realtime_score: f64,

    pub biometric_score: f64,

    pub overall_score: f64,

    pub entropy_rate: f64,

    pub min_entropy_bits: f64,

    pub assessed_at: DateTime<Utc>,

pub struct HumanEntropyClassification {

    pub qualifies_for_elevation: bool,

    pub recommended_tier: HsmTier,

    pub assessment: HumanEntropyAssessment,

    pub classified_at: DateTime<Utc>,

    pub classification_confidence: f64,

#[derive(Debug, Clone, Default)]
pub struct TierElevationCriteria {

    pub min_entropy_methods: usize,

    pub min_overall_score: f64,

    pub require_realtime: bool,

    pub require_biometric: bool,

#[derive(Default)]
pub struct EntropyQualityAssessor;
impl EntropyQualityAssessor {
        Ok(Self)

    pub async fn assess_capability_quality(

            score += 0.4;

        if capabilities.min_entropy_bits >= 256.0 {
        } else if capabilities.min_entropy_bits >= 128.0 {
        } else if capabilities.min_entropy_bits >= 64.0 {

pub struct HumanEntropyMethodEvaluator {

        method_weights: HashMap<&str, f64>,}

impl HumanEntropyMethodEvaluator {
        let mut method_weights = HashMap::with_capacity(16);

        method_weights.insert("TouchPatterns".to_string(), 0.8);
        method_weights.insert("Biometric".to_string(), 0.9);
        method_weights.insert("VoicePatterns".to_string(), 0.7);
        method_weights.insert("BehavioralPatterns".to_string(), 0.6);
        method_weights.insert("EnvironmentalSensors".to_string(), 0.5);
        method_weights.insert("HardwareEntropy".to_string(), 0.9);
        Ok(Self { method_weights })

    pub async fn evaluate_methods(
        methods: &[HumanEntropyMethod],
    ) -> Result<HashMap<HumanEntropyMethod, f64, BearDogError>> {
        let mut scores = HashMap::with_capacity(16);
        for method in methods {
            let score = self.evaluate_single_method(method).await?;
            scores.insert(method.clone(), score);
        Ok(scores)}

    async fn evaluate_single_method(&self, method: &HumanEntropyMethod) -> Result<f64, BearDogError> {
        let base_score = match method {
            HumanEntropyMethod::TouchPatterns => 0.8,
            HumanEntropyMethod::Biometric => 0.9,
            HumanEntropyMethod::VoicePatterns => 0.7,
            HumanEntropyMethod::BehavioralPatterns => 0.6,
            HumanEntropyMethod::EnvironmentalSensors => 0.5,
            HumanEntropyMethod::MouseMovement => 0.5,
            HumanEntropyMethod::KeyboardTiming => 0.6,
            HumanEntropyMethod::Voice => 0.8,
            HumanEntropyMethod::Camera => 0.7,
            HumanEntropyMethod::Custom(_) => 0.4,
            HumanEntropyMethod::HardwareEntropy { source_type } => match source_type.as_str() {
                "accelerometer" => 0.6,
                "gyroscope" => 0.6,
                "magnetometer" => 0.5,
                "camera_noise" => 0.8,
                "microphone_noise" => 0.7,
                _ => 0.5,
            },
        Ok(base_score)
impl Default for UnifiedHumanEntropyClassifier {}

    fn default() -> Self {
        match Self::new() {
            Ok(classifier) => classifier,
            Err(e) => {
                tracing::error!(
                    "Failed to create default UnifiedHumanEntropyClassifier: {:?}",
                    e
                );

                Self {
                    quality_assessor: Default::default(),
                    method_evaluator: Default::default(),
                    tier_criteria: TierElevationCriteria::default(),
                }
            }
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_unified_classifier_creation() -> Result<(), BearDogError> {
        let classifier = UnifiedHumanEntropyClassifier::new().map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        assert_eq!(classifier.tier_criteria.min_entropy_methods, 2);
        Ok(())
    async fn test_tier_elevation_classification() -> Result<(), BearDogError> {
        let capabilities = UnifiedHumanEntropyCapabilities {
            hardware_backed: true,
            key_attestation: true,
            user_authentication: true,
            rollback_resistance: true,
            supported_key_sizes: vec![256, 384, 521],
            supports_ephemeral_seeds: true,
            realtime_entropy: true,
            collection_methods: vec![
                HumanEntropyMethod::TouchPatterns,
                HumanEntropyMethod::Biometric,
                HumanEntropyMethod::KeyboardTiming,
                HumanEntropyMethod::MouseMovement, // Add fourth method to ensure EnhancedHardware tier
            ],
            quality_assessment: true,
            biometric_integration: true,
            min_entropy_bits: 256.0,
            max_collection_rate: 1000.0,
        let classification = classifier
            .classify_for_tier_elevation(&capabilities)
            .await
            .map_err(|e| {
                    "Operation failed ({}): {:?}",
                    "Classification should succeed",
                beardog_errors::BearDogError::internal(format!(
                    "Classification should succeed", e
                ))
            })?;
        println!(
            "Classification result: {:?}",
            classification.recommended_tier
        );
            "Overall score: {:.3}",
            classification.assessment.overall_score
            "Method count: {}",
            classification.assessment.method_scores.len()
        assert!(classification.qualifies_for_elevation);

        assert!(matches!(
            classification.recommended_tier,
            HsmTier::Premium | HsmTier::EnhancedHardware | HsmTier::BasicHardware
        ));
    #[test]
    fn test_tier_elevation_criteria_default() -> Result<(), BearDogError> {
        let criteria = TierElevationCriteria::default();
        assert_eq!(criteria.min_entropy_methods, 2);
        assert_eq!(criteria.min_overall_score, 0.7);
        assert!(!criteria.require_realtime);
