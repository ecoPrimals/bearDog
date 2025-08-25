// MODERNIZED: Removed async_trait - now uses native async fn in trait

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


/// # Unified `HSM` Provider System
///
/// **FRAGMENTATION ELIMINATION - SINGLE SOURCE OF TRUTH**
/// This module consolidates all fragmented `HSM` provider implementations:
/// - ❌ hsm_foundation/providers/manager.rs (REPLACED)
/// - ❌ tunnel/hsm/types/providers.rs (REPLACED)
/// - ❌ Various scattered provider fragments (REPLACED)
/// - ✅ This unified system (CANONICAL)

use beardog_errors::BearDogResult;
use beardog_traits::canonical::HsmProvider;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
// Cleaned up during modernization
use tracing::debug;
/// Unified Human Entropy Capabilities structure
#[derive(Debug, Clone)]
pub struct UnifiedHumanEntropyCapabilities {
    /// Whether the implementation is hardware-backed
    pub hardware_backed: bool,
    /// Whether key attestation is supported
    pub key_attestation: bool,
    /// Whether user authentication is supported
    pub user_authentication: bool,
    /// Whether rollback resistance is supported
    pub rollback_resistance: bool,
    /// Supported key sizes in bits
    pub supported_key_sizes: Vec<u32>,
    /// Whether this `HSM` supports ephemeral seed creation from human entropy
    pub supports_ephemeral_seeds: bool,
    /// Available entropy collection methods
    pub collection_methods: Vec<HumanEntropyMethod>,
    /// Real-time entropy collection capability
    pub realtime_entropy: bool,
    /// Quality assessment capabilities
    pub quality_assessment: bool,
    /// Biometric integration support
    pub biometric_integration: bool,
    /// Minimum entropy bits supported
    pub min_entropy_bits: f64,
    /// Maximum entropy collection rate (bits/second)
    pub max_collection_rate: f64,
}
/// Human entropy collection methods
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum HumanEntropyMethod {
    /// Mouse movement patterns
    MouseMovement,
    /// Keyboard timing patterns
    KeyboardTiming,
    /// Touch screen patterns
    TouchPatterns,
    /// Biometric patterns
    Biometric,
    /// Voice patterns
    Voice,
    /// Voice patterns (legacy name)
    VoicePatterns,
    /// Camera-based entropy
    Camera,
    /// Behavioral patterns (typing rhythm, etc.)
    BehavioralPatterns,
    /// Environmental sensor data
    EnvironmentalSensors,
    /// Hardware entropy sources
    HardwareEntropy { source_type: String },
    /// Custom entropy method
    Custom(String),
/// **Unified `HSM` Provider Trait**
/// This trait extends the canonical HsmProvider with tunnel-specific capabilities,
/// particularly human entropy collection which is critical for `BearDog`'s security model.
/// 
/// **ZERO-COST ASYNC OPTIMIZATION** - Native async methods eliminate boxing overhead
/// This trait now uses native async fn in trait definitions (available in Rust 1.75+)
/// which eliminates the Box<dyn Future> allocation overhead from async_trait.
#[allow(async_fn_in_trait)]
pub trait UnifiedHsmProvider: HsmProvider + Send + Sync {
    /// **Human Entropy Collection - Core `BearDog` Feature**
    ///
    /// Collect human entropy for ephemeral seed generation.
    /// This is the key differentiator for tier elevation in `BearDog`'s security model.
    async fn collect_human_entropy(
        &self,
        method: &HumanEntropyMethod,
        target_bits: u32,
    ) -> BearDogResult<HumanEntropyData>;
    /// **Get Human Entropy Capabilities**
    /// Returns the human entropy capabilities of this `HSM` provider.
    async fn get_human_entropy_capabilities(
    ) -> BearDogResult<UnifiedHumanEntropyCapabilities>;
    /// **Create Ephemeral Seed from Human Entropy**
    /// Creates an ephemeral cryptographic seed from collected human entropy.
    /// This is used for tier elevation and enhanced security.
    async fn create_ephemeral_seed(
        entropy_data: &HumanEntropyData,
        seed_length: u32,
    ) -> BearDogResult<EphemeralSeed>;
    /// **Assess Entropy Quality**
    /// Performs quality assessment on collected entropy data.
    async fn assess_entropy_quality(
    ) -> BearDogResult<EntropyQualityReport>;
    /// **Get Provider Tier Recommendation**
    /// Based on capabilities, recommend the appropriate `HSM` tier for this provider.
    async fn get_tier_recommendation(&self) -> BearDogResult<HsmTier>;
/// **Human Entropy Data**
/// Container for collected human entropy with metadata.
pub struct HumanEntropyData {
    /// Raw entropy bytes
    pub data: Vec<u8>,
    /// Collection method used
    pub method: HumanEntropyMethod,
    /// Estimated entropy bits
    pub entropy_bits: f64,
    /// Collection timestamp
    pub collected_at: DateTime<Utc>,
    /// Collection duration in milliseconds
    pub collection_duration_ms: u64,
    /// Quality indicators
    pub quality_indicators: HashMap<String, f64>,
/// **Ephemeral Seed**
/// Cryptographically secure seed derived from human entropy.
#[derive(Debug)]
pub struct EphemeralSeed {
    /// Seed data (never logged or serialized)
    #[allow(dead_code)]
    pub(crate) seed_data: Vec<u8>,
    /// Seed creation timestamp
    pub created_at: DateTime<Utc>,
    /// Expiration time
    pub expires_at: DateTime<Utc>,
    /// Source entropy quality score
    pub quality_score: f64,
    /// Unique seed ID
    pub seed_id: String,
/// **Entropy Quality Report**
/// Detailed analysis of entropy quality.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntropyQualityReport {
    /// Overall quality score (0.0 to 1.0)
    /// Shannon entropy estimate
    pub shannon_entropy: f64,
    /// Min-entropy estimate
    pub min_entropy: f64,
    /// Compression ratio test result
    pub compression_ratio: f64,
    /// Statistical test results
    pub statistical_tests: HashMap<String, f64>,
    /// Quality assessment timestamp
    pub assessed_at: DateTime<Utc>,
    /// Recommendations for improvement
    pub recommendations: Vec<String>,
/// **`HSM` Tier Classification**
/// Unified tier system for `HSM` providers.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
/// Unified Provider Registry for managing `HSM` providers
pub struct UnifiedProviderRegistry {
    /// Map of provider instances
    pub providers: HashMap<String, Box<dyn UnifiedHsmProvider>>,
    /// List of providers that support human entropy
    pub human_entropy_providers: Vec<String>,}


impl Default for UnifiedProviderRegistry {}


    fn default() -> Self {
        Self::new()
    }
impl UnifiedProviderRegistry {
    /// Create a new provider registry}


    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            human_entropy_providers: Vec::new(),
        }
    /// Register a new provider
    pub async fn register_provider(
        &mut self,
        instance_id: String,
        provider: Box<dyn UnifiedHsmProvider>,
    ) -> BearDogResult<()> {
        self.providers.insert(instance_id, provider);
        Ok(())
    /// Get a provider by ID}


    pub fn get_provider(&self, instance_id: &str) -> Option<&dyn UnifiedHsmProvider> {
        self.providers.get(instance_id).map(|p| p.as_ref())
    /// Get best provider for human entropy collection
    pub async fn get_best_entropy_provider(
    ) -> BearDogResult<Option<&dyn UnifiedHsmProvider>> {
        let mut best_provider = None;
        let mut best_score = 0.0;
        for provider in self.providers.values() {
            let capabilities = provider.get_human_entropy_capabilities().await?;
            // Calculate capability score
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
/// **Human Entropy Quality Assessor**
/// Unified entropy quality assessment system.
pub struct HumanEntropyQualityAssessor;
impl HumanEntropyQualityAssessor {
    /// Assess the quality of collected human entropy
    pub async fn assess_quality(
    ) -> BearDogResult<EntropyQualityReport> {
        debug!("🧠 Assessing human entropy quality");
        // Shannon entropy calculation
        let shannon_entropy = Self::calculate_shannon_entropy(&entropy_data.data);
        // Min-entropy estimation
        let min_entropy = Self::estimate_min_entropy(&entropy_data.data);
        // Compression test
        let compression_ratio = Self::compression_test(&entropy_data.data);
        // Statistical tests
        let mut statistical_tests = HashMap::new();
        statistical_tests.insert(
            "frequency_test".to_string(),
            Self::frequency_test(&entropy_data.data),
        );
        statistical_tests.insert("runs_test".to_string(), Self::runs_test(&entropy_data.data));
            "autocorrelation_test".to_string(),
            Self::autocorrelation_test(&entropy_data.data),
        // Calculate overall quality score
        let quality_score = Self::calculate_quality_score(
            shannon_entropy,
            min_entropy,
            compression_ratio,
            &statistical_tests,
        // Generate recommendations
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
        // Simple min-entropy estimation using most common value
        let max_count = counts.iter().max().unwrap_or(&0);
        if *max_count == 0 {
            return 0.0;
        let p_max = *max_count as f64 / data.len() as f64;
        -p_max.log2()}


    fn compression_test(data: &[u8]) -> f64 {
        // Simple compression ratio using run-length encoding
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
        // Chi-square test for uniform distribution
        let expected = data.len() as f64 / 256.0;
        let mut chi_square = 0.0;
            let diff = count as f64 - expected;
            chi_square += diff * diff / expected;
        // Return p-value approximation (simplified)
        1.0 - (chi_square / 255.0).min(1.0)}


    fn runs_test(data: &[u8]) -> f64 {
        // Test for independence (simplified runs test)
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
        // Test for autocorrelation (simplified)
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
        statistical_tests: &HashMap<String, f64>,
    ) -> f64 {
        let mut score = 0.0;
        // Shannon entropy component (0-25 points)
        score += (shannon_entropy / 8.0) * 25.0;
        // Min-entropy component (0-25 points)
        score += (min_entropy / 8.0) * 25.0;
        // Compression ratio component (0-25 points)
        score += (compression_ratio.min(2.0) / 2.0) * 25.0;
        // Statistical tests component (0-25 points)
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
        // Test with high-quality random data
        let entropy_data = HumanEntropyData {
            data: (0..1000).map(|i| (i % 256) as u8).collect(),
            method: HumanEntropyMethod::TouchPatterns,
            entropy_bits: 800.0,
            collected_at: Utc::now(),
            collection_duration_ms: 1000,
            quality_indicators: HashMap::new(),
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
