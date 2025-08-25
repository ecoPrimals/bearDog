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


/// Human Entropy Collection Traits - Universal Interface

use super::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
/// Human entropy collection methods
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HumanEntropyMethod {
    /// Mouse movement patterns
    MouseMovement,
    /// Keystroke timing patterns  
    KeystrokeTiming,
    /// Touch gesture patterns
    TouchGestures,
    /// Voice pattern analysis
    VoicePattern,
    /// Biometric pattern analysis
    BiometricPattern,
}
/// Human entropy data collected from user interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyData {
    /// Raw entropy bytes
    pub entropy_bytes: Vec<u8>,
    /// Collection method used
    pub collection_method: HumanEntropyMethod,
    /// Estimated entropy bits
    pub estimated_entropy_bits: f64,
    /// When the entropy was collected
    pub collected_at: DateTime<Utc>,
    /// Collection duration in milliseconds
    pub collection_duration_ms: u64,
    /// Quality score (0.0 to 1.0)
    pub quality_score: f64,
/// Human entropy collection capabilities
pub struct HumanEntropyCapabilities {
    /// Whether ephemeral seeds are supported
    pub supports_ephemeral_seeds: bool,
    /// Available collection methods
    pub collection_methods: Vec<HumanEntropyMethod>,
    /// Real-time entropy collection support
    pub realtime_entropy: bool,
    /// Quality assessment capabilities
    pub quality_assessment: bool,
    /// Biometric integration support
    pub biometric_integration: bool,
    /// Minimum entropy bits per collection
    pub min_entropy_bits: f64,
    /// Maximum collection rate (Hz)
    pub max_collection_rate: f64,
/// Ephemeral seed created from human entropy
pub struct EphemeralSeed {
    /// Seed bytes
    pub seed_bytes: Vec<u8>,
    /// Source entropy data
    pub source_entropy: Vec<u8>,
    /// When the seed expires
    pub expires_at: DateTime<Utc>,
    /// Unique seed identifier
    pub seed_id: String,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Quality score of the seed
    // Modern entropy configuration
    pub initial_seed: Vec<u8>,
    pub entropy_source_types: Vec<String>,
/// Entropy Quality Assessment
///
/// Metrics for assessing the quality of collected human entropy.
pub struct EntropyQualityAssessment {
    /// Overall quality score (0.0 to 1.0)
    pub overall_score: f64,
    /// Randomness score based on statistical tests
    pub randomness_score: f64,
    /// Uniqueness score (how different from previous collections)
    pub uniqueness_score: f64,
    /// Timing consistency score
    pub timing_score: f64,
    /// Collection method effectiveness score
    pub method_effectiveness_score: f64,
    /// Detailed quality metrics
    pub detailed_metrics: HashMap<String, f64>,
    /// Quality assessment timestamp
    pub assessed_at: DateTime<Utc>,}


impl EntropyQualityAssessment {
    /// Create a new quality assessment}


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
            detailed_metrics: HashMap::new(),
            assessed_at: Utc::now(),
        }
    }
    /// Add a detailed metric
    pub fn with_metric(mut self, name: String, value: f64) -> Self {
        self.detailed_metrics.insert(name, value);
        self
    /// Check if the assessment meets a quality threshold}


    pub fn meets_threshold(&self, threshold: f64) -> bool {
        self.overall_score >= threshold
/// Entropy Collection Configuration
/// Configuration for human entropy collection operations.
pub struct EntropyCollectionConfig {
    /// Preferred collection methods (in order of preference)
    pub preferred_methods: Vec<HumanEntropyMethod>,
    /// Minimum quality threshold
    pub min_quality_threshold: f64,
    /// Maximum collection time in milliseconds
    pub max_collection_time_ms: u64,
    /// Target entropy bits to collect
    pub target_entropy_bits: f64,
    /// Whether to provide real-time feedback to user
    pub provide_feedback: bool,
    /// Custom collection parameters
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
            custom_parameters: HashMap::new(),
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
