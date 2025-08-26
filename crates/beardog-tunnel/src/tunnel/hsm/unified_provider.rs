

use beardog_errors::BearDogResult;
use beardog_traits::canonical::HsmProvider;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use tracing::debug;

#[derive(Debug, Clone)]
pub struct UnifiedHumanEntropyCapabilities {

    pub hardware_backed: bool,

    pub key_attestation: bool,

    pub user_authentication: bool,

    pub rollback_resistance: bool,

    pub supported_key_sizes: Vec<u32>,

    pub supports_ephemeral_seeds: bool,

    pub collection_methods: Vec<HumanEntropyMethod>,

    pub realtime_entropy: bool,

    pub quality_assessment: bool,

    pub biometric_integration: bool,

    pub min_entropy_bits: f64,

    pub max_collection_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum HumanEntropyMethod {

    MouseMovement,

    KeyboardTiming,

    TouchPatterns,

    Biometric,

    Voice,

    VoicePatterns,

    Camera,

    BehavioralPatterns,

    EnvironmentalSensors,

    HardwareEntropy { source_type: String },

    Custom(String),

#[allow(async_fn_in_trait)]
pub trait UnifiedHsmProvider: HsmProvider + Send + Sync {

    async fn collect_human_entropy(
        &self,
        method: &HumanEntropyMethod,
        target_bits: u32,
    ) -> BearDogResult<HumanEntropyData>;

    async fn get_human_entropy_capabilities(
    ) -> BearDogResult<UnifiedHumanEntropyCapabilities>;

    async fn create_ephemeral_seed(
        entropy_data: &HumanEntropyData,
        seed_length: u32,
    ) -> BearDogResult<EphemeralSeed>;

    async fn assess_entropy_quality(
    ) -> BearDogResult<EntropyQualityReport>;

    async fn get_tier_recommendation(&self) -> BearDogResult<HsmTier>;

pub struct HumanEntropyData {

    pub data: Vec<u8>,

    pub method: HumanEntropyMethod,

    pub entropy_bits: f64,

    pub collected_at: DateTime<Utc>,

    pub collection_duration_ms: u64,

    pub quality_indicators: HashMap<String, f64>,

#[derive(Debug)]
pub struct EphemeralSeed {

        pub(crate) seed_data: Vec<u8>,

    pub created_at: DateTime<Utc>,

    pub expires_at: DateTime<Utc>,

    pub quality_score: f64,

    pub seed_id: String,

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntropyQualityReport {

    pub shannon_entropy: f64,

    pub min_entropy: f64,

    pub compression_ratio: f64,

    pub statistical_tests: HashMap<String, f64>,

    pub assessed_at: DateTime<Utc>,

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

    pub fn new() -> Self {
        Self {
            providers: HashMap::with_capacity(16),
            human_entropy_providers: Vec::new(),
        }

    pub async fn register_provider(
        &mut self,
        instance_id: &str,
        provider: Box<dyn UnifiedHsmProvider>,
    ) -> BearDogResult<()> {
        self.providers.insert(instance_id, provider);
        Ok(())

    pub fn get_provider(&self, instance_id: &str) -> Option<&dyn UnifiedHsmProvider> {
        self.providers.get(instance_id).map(|p| p.as_ref())

    pub async fn get_best_entropy_provider(
    ) -> BearDogResult<Option<&dyn UnifiedHsmProvider>> {
        let mut best_provider = None;
        let mut best_score = 0.0;
        for provider in self.providers.values() {
            let capabilities = provider.get_human_entropy_capabilities().await?;

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

    pub async fn assess_quality(
    ) -> BearDogResult<EntropyQualityReport> {
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

        if data.len() < 10 {
        let lag = (data.len() / 10).max(1);
        let mut correlation = 0.0;
        let mut count = 0;
        for i in lag..data.len() {
            correlation += (data[i] as f64) * (data[i - lag] as f64);
            count += 1;
        if count == 0 {
        correlation /= count as f64;
        let normalized = (correlation / (127.5 * 127.5) - 1.0).abs();
        1.0 - normalized.min(1.0)}

    fn calculate_quality_score(
        shannon_entropy: f64,
        min_entropy: f64,
        compression_ratio: f64,
        statistical_tests: &HashMap<&str, f64>,
    ) -> f64 {
        let mut score = 0.0;

        score += (shannon_entropy / 8.0) * 25.0;

        score += (min_entropy / 8.0) * 25.0;

        score += (compression_ratio.min(2.0) / 2.0) * 25.0;

        let avg_test_score: f64 =
            statistical_tests.values().sum::<f64>() / statistical_tests.len() as f64;
        score += avg_test_score * 25.0;
        score.min(100.0) / 100.0}

    fn generate_recommendations(
        quality_score: f64,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();
        if quality_score < 0.7 {
            recommendations.push("Consider collecting more entropy data".to_string());
        if let Some(&freq_score) = statistical_tests.get("frequency_test") {
            if freq_score < 0.5 {
                recommendations.push("Frequency distribution appears non-uniform".to_string());
        if let Some(&runs_score) = statistical_tests.get("runs_test") {
            if runs_score < 0.5 {
                recommendations.push(
                    "Data may have patterns - consider different collection method".to_string(),
                );
        if recommendations.is_empty() {
            recommendations.push("Entropy quality is acceptable".to_string());
        recommendations
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_entropy_quality_assessment() -> beardog_errors::BearDogResult<()> {

        let entropy_data = HumanEntropyData {
            data: (0..1000).map(|i| (i % 256) as u8).collect(),
            method: HumanEntropyMethod::TouchPatterns,
            entropy_bits: 800.0,
            collected_at: Utc::now(),
            collection_duration_ms: 1000,
            quality_indicators: HashMap::with_capacity(16),
        };
        let report = HumanEntropyQualityAssessor::assess_quality(&entropy_data)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        assert!(report.quality_score > 0.0);
        assert!(report.shannon_entropy > 0.0);
    #[test]
    fn test_unified_provider_registry() -> beardog_errors::BearDogResult<()> {
        let registry = UnifiedProviderRegistry::new();
        assert_eq!(registry.providers.len(), 0);
