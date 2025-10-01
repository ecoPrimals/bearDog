

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_traits::unified::HsmProvider;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use tracing::debug;

#[derive(Debug, Clone)]
    /// Whether key_attestation is enabled
    pub key_attestation: bool,

    /// Whether user_authentication is enabled
    pub user_authentication: bool,

    /// Whether rollback_resistance is enabled
    pub rollback_resistance: bool,

    /// Collection of supported key sizes
    pub supported_key_sizes: Vec<u32>,

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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum HumanEntropyMethod {


    /// Represents mouse movement variant
    MouseMovement,


    /// Currently keyboardtiming
    KeyboardTiming,


    /// Represents touch patterns variant
    TouchPatterns,


    /// Represents biometric variant
    Biometric,


    /// Represents voice variant
    Voice,


    /// Represents voice patterns variant
    VoicePatterns,


    /// Represents camera variant
    Camera,


    /// Represents behavioral patterns variant
    BehavioralPatterns,


    /// Represents environmental sensors variant
    EnvironmentalSensors,

    HardwareEntropy { source_type: String },
    HardwareEntropy { source_type: String },
    HardwareEntropy { source_type: String },

    Custom(HsmProvider + Send + Sync {


    fn collect_human_entropy(
        &self,
        method: &HumanEntropyMethod,
        target_bits: u32,
    ) -> Result<HumanEntropyData, BearDogError>;

    /// Gets human_entropy_capabilities
    fn get_human_entropy_capabilities(&HumanEntropyData,
        seed_length: u32,
    ) -> Result<EphemeralSeed, BearDogError>;


    fn assess_entropy_quality(Vec<u8>,

    /// The method value
    pub method: HumanEntropyMethod,

    /// The entropy bits value
    pub entropy_bits: f64,

    /// The collected at value
    pub collected_at: DateTime<Utc>,

    /// Number of collection_duration_ms
    pub collection_duration_ms: u64,

    /// Mapping of quality indicators
    pub quality_indicators: HashMap<String, f64>,

#[derive(Debug, Clone)]
    /// The created at value
    pub created_at: DateTime<Utc>,

    /// The expires at value
    pub expires_at: DateTime<Utc>,

    /// The quality score value
    pub quality_score: f64,


    pub seed_id: String,

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntropyQualityReport {

    /// The shannon entropy value
    pub shannon_entropy: f64,

    /// The min entropy value
    pub min_entropy: f64,

    /// The compression ratio value
    pub compression_ratio: f64,

    /// Mapping of statistical tests
    pub statistical_tests: HashMap<String, f64>,

    /// The assessed at value
    pub assessed_at: DateTime<Utc>,

    /// Collection of recommendations
    pub recommendations: Vec<String>,

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]

pub struct UnifiedProviderRegistry {


    pub providers: HashMap<String, Box<dyn UnifiedHsmProvider>>,


    pub human_entropy_providers: Vec<String>,}

impl Default for UnifiedProviderRegistry {}

    fn default() -> Self {
        Self::new()
    }
impl UnifiedProviderRegistry {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            providers: HashMap::with_capacity(16),
            human_entropy_providers: Vec::new(&str,
        provider: Box<dyn UnifiedHsmProvider>,
    ) -> Result<(), BearDogError> {
        self.providers.insert(instance_id, provider);
        Ok(())

/// Get Provider operation.
    /// Gets provider
    /// Gets provider
    pub fn get_provider(&self, instance_id: &str) -> Option<&dyn UnifiedHsmProvider> {
        self.providers.get(instance_id).map(|p| p.as_ref())

/// Get Best Entropy Provider operation.
    /// Gets best_entropy_provider
    /// Gets best_entropy_provider
    pub fn get_best_entropy_provider(
    ) -> Result<Option<&dyn UnifiedHsmProvider>, BearDogError>> {
        let mut best_provider = None;
        let mut best_score = 0.0;
        for provider in self.providers.values() {
            let capabilities = provider.get_human_entropy_capabilities()?;

            let mut score = 0.0;
            if capabilities.realtime_entropy {
                score += 10.0;
            }
            if capabilities.biometric_integration {
                score += 15.0;
            if capabilities.quality_assessment {
            score += capabilities.min_entropy_bits * 0.1;
            score += capabilities.max_collection_rate * 0.001;
            score += capabilities.collection_methods.len() as f64 * 5.0;
            if score > best_score {
                best_score = score;
                best_provider = Some(provider.as_ref());
        Ok(best_provider)

pub struct HumanEntropyQualityAssessor;
impl HumanEntropyQualityAssessor {

/// Assess Quality operation.
    pub fn assess_quality(
    ) -> Result<EntropyQualityReport, BearDogError> {
        debug!("🧠 Assessing human entropy quality");

        let shannon_entropy = Self::calculate_shannon_entropy(&entropy_data.data);

        let min_entropy = Self::estimate_min_entropy(&entropy_data.data);

        let compression_ratio = Self::compression_test(&entropy_data.data);

        let mut statistical_tests = HashMap::with_capacity(16);
        statistical_tests.insert(
            "frequency_test".to_string(),
            Self::frequency_test(&entropy_data.data),
        );
        statistical_tests.insert("runs_test".to_string(), Self::runs_test(&entropy_data.data));
            "autocorrelation_test".to_string(),
            Self::autocorrelation_test(&entropy_data.data),

        let quality_score = Self::calculate_quality_score(
            shannon_entropy,
            min_entropy,
            compression_ratio,
            &statistical_tests,

        let recommendations = Self::generate_recommendations(quality_score, &statistical_tests);
        Ok(EntropyQualityReport {
            quality_score,
            statistical_tests,
            assessed_at: Utc::now(),
            recommendations,
        })
    fn calculate_shannon_entropy(data: &[u8]) -> f64 {
        let mut counts = [0u32; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        let len = data.len() as f64;
        let mut entropy = 0.0;
        for count in counts {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
        entropy}


    fn estimate_min_entropy(data: &[u8]) -> f64 {

        let max_count = counts.iter().max().unwrap_or(&0);
        if *max_count == 0 {
            return 0.0;
        let p_max = *max_count as f64 / data.len() as f64;
        -p_max.log2()}


    fn compression_test(data: &[u8]) -> f64 {

        if data.is_empty() {
            return 1.0;
        let mut compressed_size = 0;
        let mut current = data[0];
        let mut _count = 1;
        for &byte in &data[1..] {
            if byte == current {
                _count += 1;
            } else {
                compressed_size += 2; // byte + count
                current = byte;
                _count = 1;
        compressed_size += 2; // final run
        data.len() as f64 / compressed_size as f64
    fn frequency_test(data: &[u8]) -> f64 {

        let expected = data.len() as f64 / 256.0;
        let mut chi_square = 0.0;
            let diff = count as f64 - expected;
            chi_square += diff * diff / expected;

        1.0 - (chi_square / 255.0).min(1.0)}


    fn runs_test(data: &[u8]) -> f64 {

        if data.len() < 2 {
        let mut runs = 1;
        for i in 1..data.len() {
            if (data[i] > 127) != (data[i - 1] > 127) {
                runs += 1;
        let n = data.len() as f64;
        let expected_runs = n / 2.0 + 1.0;
        let variance = (n - 1.0) / 4.0;
        if variance <= 0.0 {
        let z = (runs as f64 - expected_runs).abs() / variance.sqrt();
        1.0 - (z / 3.0).min(1.0) // Simplified p-value}


    fn autocorrelation_test(data: &[u8]) -> f64 {

        if data.len(f64,
        min_entropy: f64,
        compression_ratio: f64,
        statistical_tests: &HashMap<&str, f64>,
    ) -> f64 {
        let mut score = 0.0;

        score += (shannon_entropy / 8.0) * 25.0;

        score += (min_entropy / 8.0) * 25.0;

        score += (compression_ratio.min(2.0) / 2.0) * 25.0;

        let avg_test_score: f64 =
            statistical_tests.values().sum::<f64>() / statistical_tests.len(f64,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();
        if quality_score < 0.7 {
            recommendations.push("Consider collecting more entropy data".to_string());
        if let Some(freq_score) = statistical_tests.get("frequency_test") {
            if freq_score < 0.5 {
                recommendations.push("Frequency distribution appears non-uniform".to_string());
        if let Some(runs_score) = statistical_tests.get("runs_test") {
            if runs_score < 0.5 {
                recommendations.push(
                    "Data may have patterns - consider different collection method");
        if recommendations.is_empty() {
            recommendations.push("Entropy quality is acceptable".to_string());
        recommendations
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    fn test_entropy_quality_assessment() -> Result<(), BearDogError> {

        let entropy_data = HumanEntropyData {
            data: (0..1000).map(HumanEntropyMethod::TouchPatterns,
            entropy_bits: 800.0,
            collected_at: Utc::now(1000,
            quality_indicators: HashMap::with_capacity(16),
        };
        let report = HumanEntropyQualityAssessor::assess_quality(&entropy_data)
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        assert!(report.quality_score > 0.0);
        assert!(report.shannon_entropy > 0.0);
    #[test]
    fn test_unified_provider_registry() -> Result<(), BearDogError> {
        let registry = UnifiedProviderRegistry::new();
        assert_eq!(registry.providers.len(), 0);
