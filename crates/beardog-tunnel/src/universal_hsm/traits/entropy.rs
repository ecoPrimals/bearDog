

use super::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HumanEntropyMethod {

    MouseMovement,

    KeystrokeTiming,

    TouchGestures,

    VoicePattern,

    BiometricPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyData {

    pub entropy_bytes: Vec<u8>,

    pub collection_method: HumanEntropyMethod,

    pub estimated_entropy_bits: f64,

    pub collected_at: DateTime<Utc>,

    pub collection_duration_ms: u64,

    pub quality_score: f64,

pub struct HumanEntropyCapabilities {

    pub supports_ephemeral_seeds: bool,

    pub collection_methods: Vec<HumanEntropyMethod>,

    pub realtime_entropy: bool,

    pub quality_assessment: bool,

    pub biometric_integration: bool,

    pub min_entropy_bits: f64,

    pub max_collection_rate: f64,

pub struct EphemeralSeed {

    pub seed_bytes: Vec<u8>,

    pub source_entropy: Vec<u8>,

    pub expires_at: DateTime<Utc>,

    pub seed_id: String,

    pub metadata: HashMap<String, serde_json::Value>,

    pub initial_seed: Vec<u8>,
    pub entropy_source_types: Vec<String>,

pub struct EntropyQualityAssessment {

    pub overall_score: f64,

    pub randomness_score: f64,

    pub uniqueness_score: f64,

    pub timing_score: f64,

    pub method_effectiveness_score: f64,

    pub detailed_metrics: HashMap<String, f64>,

    pub assessed_at: DateTime<Utc>,}

impl EntropyQualityAssessment {

    pub fn new(
        randomness_score: f64,
        uniqueness_score: f64,
        timing_score: f64,
        method_effectiveness_score: f64,
    ) -> Self {
        let overall_score =
            (randomness_score + uniqueness_score + timing_score + method_effectiveness_score) / 4.0;
        Self {
            overall_score,
            randomness_score,
            uniqueness_score,
            timing_score,
            method_effectiveness_score,
            detailed_metrics: HashMap::with_capacity(16),
            assessed_at: Utc::now(),
        }
    }

    pub fn with_metric(mut self, name: &str, value: f64) -> Self {
        self.detailed_metrics.insert(name, value);
        self

    pub fn meets_threshold(&self, threshold: f64) -> bool {
        self.overall_score >= threshold

pub struct EntropyCollectionConfig {

    pub preferred_methods: Vec<HumanEntropyMethod>,

    pub min_quality_threshold: f64,

    pub max_collection_time_ms: u64,

    pub target_entropy_bits: f64,

    pub provide_feedback: bool,

    pub custom_parameters: HashMap<String, String>,}

impl Default for EntropyCollectionConfig {}

    fn default() -> Self {
            preferred_methods: vec![
                HumanEntropyMethod::TouchGestures,
                HumanEntropyMethod::KeystrokeTiming,
                HumanEntropyMethod::MouseMovement,
            ],
            min_quality_threshold: 0.7,
            max_collection_time_ms: 30000, // 30 seconds
            target_entropy_bits: 256.0,
            provide_feedback: true,
            custom_parameters: HashMap::with_capacity(16),
#[cfg(test)]
mod tests {
    use super::*;
    #[test]}

    fn test_human_entropy_capabilities_default() -> beardog_errors::BearDogResult<()> {
        let caps = HumanEntropyCapabilities::default();
        assert!(!caps.supports_ephemeral_seeds);
        assert!(caps.collection_methods.is_empty());
        assert_eq!(caps.min_entropy_bits, 0.0);
        Ok(())
    fn test_human_entropy_method_display() -> beardog_errors::BearDogResult<()> {
        assert_eq!(
            HumanEntropyMethod::TouchGestures.to_string(),
            "Touch Interaction"
        );
            HumanEntropyMethod::KeystrokeTiming.to_string(),
            "Keyboard Timing"
            HumanEntropyMethod::Custom("test".to_string()).to_string(),
            "Custom: test"}

    fn test_human_entropy_data_creation() -> beardog_errors::BearDogResult<()> {
        let entropy_data = HumanEntropyData::new(
            vec![1, 2, 3, 4, 5],
            HumanEntropyMethod::TouchGestures,
            128.0,
            0.8,
            1000,
        assert_eq!(entropy_data.size_bytes(), 5);
        assert_eq!(entropy_data.estimated_entropy_bits, 128.0);
        assert!(entropy_data.meets_quality_threshold(0.7));
        assert!(!entropy_data.meets_quality_threshold(0.9));
    fn test_ephemeral_seed_creation() -> beardog_errors::BearDogResult<()> {
        let seed = EphemeralSeed::new(vec![1, 2, 3, 4], 0.9, entropy_data, None);
        assert_eq!(seed.size_bytes(), 4);
        assert!(!seed.is_expired());
        assert!(seed.estimated_entropy_bits() > 0.0);}

    fn test_entropy_quality_assessment() -> beardog_errors::BearDogResult<()> {
        let assessment = EntropyQualityAssessment::new(0.8, 0.9, 0.7, 0.85);
        assert!(assessment.overall_score > 0.8);
        assert!(assessment.meets_threshold(0.7));
        assert!(!assessment.meets_threshold(0.9));
    fn test_entropy_collection_config_default() -> beardog_errors::BearDogResult<()> {
        let config = EntropyCollectionConfig::default();
        assert!(!config.preferred_methods.is_empty());
        assert_eq!(config.min_quality_threshold, 0.7);
        assert!(config.provide_feedback);
