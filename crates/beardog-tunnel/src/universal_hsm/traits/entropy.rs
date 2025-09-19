// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

use super::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
    /// The collection method value
    pub collection_method: HumanEntropyMethod,

    /// The estimated entropy bits value
    pub estimated_entropy_bits: f64,

    /// The collected at value
    pub collected_at: DateTime<Utc>,

    /// Number of collection_duration_ms
    pub collection_duration_ms: u64,

    /// The quality score value
    pub quality_score: f64,

pub struct HumanEntropyCapabilities {

    /// Whether supports_ephemeral_seeds is enabled
    pub supports_ephemeral_seeds: bool,

    /// Collection of collection methods
    pub collection_methods: Vec<HumanEntropyMethod>,


    pub realtime_entropy: bool,

    /// Whether quality_assessment is enabled
    pub quality_assessment: bool,

    /// Whether biometric_integration is enabled
    pub biometric_integration: bool,

    /// The min entropy bits value
    pub min_entropy_bits: f64,

    /// The max collection rate value
    pub max_collection_rate: f64,

pub struct EphemeralSeed {

    /// Collection of seed bytes
    pub seed_bytes: Vec<u8>,

    /// Collection of source entropy
    pub source_entropy: Vec<u8>,

    /// The expires at value
    pub expires_at: DateTime<Utc>,


    pub seed_id: String,

    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,

    /// Collection of initial seed
    pub initial_seed: Vec<u8>,
    /// Collection of entropy source types
    pub entropy_source_types: Vec<String>,

pub struct EntropyQualityAssessment {

    /// The overall score value
    pub overall_score: f64,

    /// The randomness score value
    pub randomness_score: f64,

    /// The uniqueness score value
    pub uniqueness_score: f64,

    /// The timing score value
    pub timing_score: f64,

    /// The method effectiveness score value
    pub method_effectiveness_score: f64,

    /// Mapping of detailed metrics
    pub detailed_metrics: HashMap<String, f64>,

    /// The assessed at value
    pub assessed_at: DateTime<Utc>,}

impl EntropyQualityAssessment {

/// New operation.
    /// Creates a new instance
    pub fn new(f64,
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
            assessed_at: Utc::now(&str, value: f64) -> Self {
        self.detailed_metrics.insert(name, value);
        self

/// Meets Threshold operation.
    pub fn meets_threshold(&self, threshold: f64) -> bool {
        self.overall_score >= threshold

pub struct EntropyCollectionConfig {

    /// Collection of preferred methods
    pub preferred_methods: Vec<HumanEntropyMethod>,

    /// The min quality threshold value
    pub min_quality_threshold: f64,


    pub max_collection_time_ms: u64,

    /// The target entropy bits value
    pub target_entropy_bits: f64,


    pub provide_feedback: bool,

    /// Mapping of custom parameters
    pub custom_parameters: HashMap<String, String>,}

impl Default for EntropyCollectionConfig {}

    fn default(vec![
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


    fn test_human_entropy_capabilities_default() -> Result<(), BearDogError> {
        let caps = HumanEntropyCapabilities::default();
        assert!(!caps.supports_ephemeral_seeds);
        assert!(caps.collection_methods.is_empty());
        assert_eq!(caps.min_entropy_bits, 0.0);
        Ok(())
    fn test_human_entropy_method_display() -> Result<(), BearDogError> {
        assert_eq!(
            HumanEntropyMethod::TouchGestures.to_string(),
            "Touch Interaction"
        );
            HumanEntropyMethod::KeystrokeTiming.to_string(),
            "Keyboard Timing"
            HumanEntropyMethod::Custom("test".to_string()).to_string(),
            "Custom: test"}


    fn test_human_entropy_data_creation() -> Result<(), BearDogError> {
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
    fn test_ephemeral_seed_creation() -> Result<(), BearDogError> {
        let seed = EphemeralSeed::new(vec![1, 2, 3, 4], 0.9, entropy_data, None);
        assert_eq!(seed.size_bytes(), 4);
        assert!(!seed.is_expired());
        assert!(seed.estimated_entropy_bits() > 0.0);}


    fn test_entropy_quality_assessment() -> Result<(), BearDogError> {
        let assessment = EntropyQualityAssessment::new(0.8, 0.9, 0.7, 0.85);
        assert!(assessment.overall_score > 0.8);
        assert!(assessment.meets_threshold(0.7));
        assert!(!assessment.meets_threshold(0.9));
    fn test_entropy_collection_config_default() -> Result<(), BearDogError> {
        let config = EntropyCollectionConfig::default();
        assert!(!config.preferred_methods.is_empty());
        assert_eq!(config.min_quality_threshold, 0.7);
        assert!(config.provide_feedback);
