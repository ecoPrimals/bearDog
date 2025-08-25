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


/// Universal Human Entropy System - Modular Architecture
///
/// **REFACTORED ENTROPY MANAGEMENT SYSTEM**
/// This module provides a clean, modular organization of the universal human entropy system
/// that was previously in a single 938-line file. The refactoring improves maintainability
/// while preserving all entropy collection capabilities.
/// ## Module Organization
/// - **`collector`**: Main human entropy collector with multi-method collection
/// - **`quality_assessor`**: Entropy quality validation and scoring
/// - **`tier_elevation`**: HSM provider scoring and recommendation engine
/// - **`config`**: Configuration types and default values
/// - **`stats`**: Collection statistics and metrics
/// ## Entropy Collection Methods
/// - **Biometric**: Fingerprint, voice, facial recognition
/// - **Behavioral**: Typing patterns, mouse movement, touch patterns
/// - **Environmental**: Ambient sound, light sensors, accelerometer
/// - **Interactive**: User interaction timing, random choices
/// - **Hybrid**: Combination of multiple methods for maximum entropy

use beardog_errors::{BearDogError, BearDogResult};
use chrono::Utc;
use std::collections::HashMap;
use tracing::{debug, info, warn};
// Import traits from parent module
use super::traits::{
    EphemeralSeed, HumanEntropyCapabilities, HumanEntropyData, HumanEntropyMethod,
};
// Sub-modules
pub mod collector;
// Re-export main components
pub use collector::HumanEntropyCollector;
/// Configuration for entropy collection
#[derive(Debug, Clone)]
pub struct EntropyCollectionConfig {
    /// Minimum quality score required (0.0 - 1.0)
    pub min_quality_score: f64,
    /// Maximum collection time in milliseconds
    pub max_collection_time_ms: u64,
    /// Minimum entropy bits required
    pub min_entropy_bits: f64,
    /// Enable biometric integration
    pub enable_biometric: bool,
    /// Enable behavioral pattern collection
    pub enable_behavioral: bool,
    /// Enable environmental entropy
    pub enable_environmental: bool,
    /// Enable interactive entropy
    pub enable_interactive: bool,
}
impl Default for EntropyCollectionConfig {}


    fn default() -> Self {
        Self {
            min_quality_score: 0.7,
            max_collection_time_ms: 5000,
            min_entropy_bits: 128.0,
            enable_biometric: true,
            enable_behavioral: true,
            enable_environmental: true,
            enable_interactive: true,
        }
    }
/// Statistics for entropy collection
#[derive(Debug, Default)]
pub struct EntropyCollectionStats {
    /// Total collections performed
    pub total_collections: u64,
    /// Successful collections
    pub successful_collections: u64,
    /// Failed collections
    pub failed_collections: u64,
    /// Average quality score
    pub average_quality: f64,
    /// Average collection time
    pub average_collection_time_ms: f64,
    /// Quality scores by method
    pub quality_by_method: HashMap<String, f64>,
    /// Collection counts by method
    pub collections_by_method: HashMap<String, u64>,}


impl EntropyCollectionStats {
    /// Update statistics with new collection data}


    pub fn update_collection_stats(&mut self, quality_score: f64, collection_time: std::time::Duration) {
        self.total_collections += 1;
        self.successful_collections += 1;
        // Update average quality (running average)
        let total_quality = self.average_quality * (self.total_collections - 1) as f64 + quality_score;
        self.average_quality = total_quality / self.total_collections as f64;
        // Update average collection time
        let collection_time_ms = collection_time.as_millis() as f64;
        let total_time = self.average_collection_time_ms * (self.total_collections - 1) as f64 + collection_time_ms;
        self.average_collection_time_ms = total_time / self.total_collections as f64;
    /// Record a failed collection}


    pub fn record_failed_collection(&mut self) {
        self.failed_collections += 1;
    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_collections == 0 {
            0.0
        } else {
            (self.successful_collections as f64 / self.total_collections as f64) * 100.0
/// Entropy quality assessor for validating collected entropy
#[derive(Debug)]
pub struct EntropyQualityAssessor {
    config: EntropyCollectionConfig,}


impl EntropyQualityAssessor {
    /// Create new quality assessor}


    pub fn new(config: EntropyCollectionConfig) -> Self {
        Self { config }
    /// Assess the quality of collected entropy
    pub fn assess_entropy_quality(&self, entropy_data: &HumanEntropyData) -> BearDogResult<f64> {
        debug!("🔍 Assessing entropy quality for {} method", 
            match entropy_data.collection_method {
                HumanEntropyMethod::Biometric => "biometric",
                HumanEntropyMethod::Behavioral => "behavioral", 
                HumanEntropyMethod::Environmental => "environmental",
                HumanEntropyMethod::Interactive => "interactive",
                HumanEntropyMethod::Hybrid => "hybrid",
            }
        );
        let mut quality_score = 0.0;
        // Base score from entropy bits
        let entropy_ratio = entropy_data.entropy_bits / 256.0; // Normalize to 256 bits max
        quality_score += entropy_ratio.min(1.0) * 0.4; // 40% weight
        // Data size score
        let data_size_score = (entropy_data.raw_data.len() as f64 / 1024.0).min(1.0); // Normalize to 1KB
        quality_score += data_size_score * 0.2; // 20% weight
        // Method-specific scoring
        let method_score = match entropy_data.collection_method {
            HumanEntropyMethod::Biometric => 0.9,     // High trust
            HumanEntropyMethod::Behavioral => 0.7,    // Medium trust
            HumanEntropyMethod::Environmental => 0.5, // Lower trust
            HumanEntropyMethod::Interactive => 0.6,   // Variable trust
            HumanEntropyMethod::Hybrid => 0.8,        // Combined trust
        };
        quality_score += method_score * 0.3; // 30% weight
        // Quality indicators score
        let indicator_score = self.assess_quality_indicators(&entropy_data.quality_indicators);
        quality_score += indicator_score * 0.1; // 10% weight
        // Ensure score is within bounds
        quality_score = quality_score.max(0.0).min(1.0);
        debug!("✅ Entropy quality assessment: {:.3}", quality_score);
        Ok(quality_score)
    /// Update assessor configuration
    pub fn update_config(&mut self, new_config: EntropyCollectionConfig) {
        self.config = new_config;
    /// Assess quality indicators}


    fn assess_quality_indicators(&self, indicators: &HashMap<String, f64>) -> f64 {
        if indicators.is_empty() {
            return 0.5; // Default score if no indicators
        let sum: f64 = indicators.values().sum();
        let average = sum / indicators.len() as f64;
        average.max(0.0).min(1.0)
/// Tier elevation engine for promoting HSM providers based on human entropy capabilities
pub struct TierElevationEngine {
    /// Provider performance scores
    provider_scores: HashMap<String, f64>,
    /// Collection success counts by method
    method_success_counts: HashMap<String, u64>,
    /// Total collections by method
    method_total_counts: HashMap<String, u64>,}


impl TierElevationEngine {
    /// Create new tier elevation engine}


    pub fn new() -> Self {
            provider_scores: HashMap::new(),
            method_success_counts: HashMap::new(),
            method_total_counts: HashMap::new(),
    /// Record a successful entropy collection}


    pub fn record_successful_collection(&mut self, method: &HumanEntropyMethod, quality_score: f64) {
        let method_name = format!("{:?}", method);
        
        // Update success counts
        *self.method_success_counts.entry(method_name.clone()).or_insert(0) += 1;
        *self.method_total_counts.entry(method_name.clone()).or_insert(0) += 1;
        // Update provider scores based on method performance
        let current_score = self.provider_scores.get(&method_name).unwrap_or(&0.5);
        let new_score = (current_score + quality_score) / 2.0; // Running average
        self.provider_scores.insert(method_name, new_score);
        debug!("📈 Updated provider score for {}: {:.3}", method_name, new_score);
    /// Record a failed entropy collection
    pub fn record_failed_collection(&mut self, method: &HumanEntropyMethod) {
        // Decrease provider score for failures
        let penalty = 0.1;
        let new_score = (current_score - penalty).max(0.0);
        warn!("📉 Decreased provider score for {}: {:.3}", method_name, new_score);
    /// Get current provider scores
    pub fn get_provider_scores(&self) -> HashMap<String, f64> {
        self.provider_scores.clone()
    /// Get success rate for a method}


    pub fn get_method_success_rate(&self, method: &HumanEntropyMethod) -> f64 {
        let successes = self.method_success_counts.get(&method_name).unwrap_or(&0);
        let total = self.method_total_counts.get(&method_name).unwrap_or(&0);
        if *total == 0 {
            (*successes as f64 / *total as f64) * 100.0
    /// Get tier elevation recommendations
    pub fn get_tier_recommendations(&self) -> Vec<(String, f64, String)> {
        let mut recommendations = Vec::new();
        for (method, score) in &self.provider_scores {
            let recommendation = if *score >= 0.9 {
                "Promote to Tier 1 (Excellent)"
            } else if *score >= 0.8 {
                "Promote to Tier 2 (Good)"
            } else if *score >= 0.7 {
                "Maintain Current Tier (Acceptable)"
            } else if *score >= 0.5 {
                "Monitor Performance (Below Average)"
            } else {
                "Consider Demotion (Poor Performance)"
            };
            recommendations.push((method.clone(), *score, recommendation.to_string()));
        // Sort by score (highest first)
        recommendations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        recommendations
impl Default for TierElevationEngine {
        Self::new()
/// Utility functions for entropy management}


pub mod utils {
    use super::*;
    /// Create a test entropy collector with default configuration
    pub fn create_test_collector() -> HumanEntropyCollector {
        HumanEntropyCollector::new(EntropyCollectionConfig::default())
    /// Create test entropy capabilities with all methods enabled}


    pub fn create_test_capabilities() -> HumanEntropyCapabilities {
        HumanEntropyCapabilities {
            biometric_available: true,
            behavioral_available: true,
            environmental_available: true,
            interactive_available: true,
            fingerprint_available: true,
            voice_available: true,
            face_available: true,
            typing_pattern_available: true,
            mouse_pattern_available: true,
            touch_pattern_available: true,
            ambient_sound_available: true,
            light_sensor_available: true,
            accelerometer_available: true,
    /// Validate entropy data meets minimum requirements
    pub fn validate_entropy_data(data: &HumanEntropyData, min_bits: f64) -> bool {
        data.entropy_bits >= min_bits && !data.raw_data.is_empty()
    /// Calculate entropy diversity score from multiple sources}


    pub fn calculate_entropy_diversity(sources: &[HumanEntropyMethod]) -> f64 {
        let unique_sources = sources.len();
        match unique_sources {
            1 => 0.25,
            2 => 0.50,
            3 => 0.75,
            4 => 0.90,
            _ => 1.0,
#[cfg(test)]}


mod tests {
    #[test]
    fn test_entropy_config_default() -> beardog_errors::BearDogResult<()> {
        let config = EntropyCollectionConfig::default();
        assert_eq!(config.min_quality_score, 0.7);
        assert_eq!(config.min_entropy_bits, 128.0);
        assert!(config.enable_biometric);
        Ok(())}


    fn test_collection_stats() -> beardog_errors::BearDogResult<()> {
        let mut stats = EntropyCollectionStats::default();
        stats.update_collection_stats(0.8, std::time::Duration::from_millis(100));
        stats.update_collection_stats(0.9, std::time::Duration::from_millis(150));
        assert_eq!(stats.total_collections, 2);
        assert_eq!(stats.successful_collections, 2);
        assert_eq!(stats.success_rate(), 100.0);
        assert!((stats.average_quality - 0.85).abs() < 0.01);
    fn test_quality_assessor() -> beardog_errors::BearDogResult<()> {
        let assessor = EntropyQualityAssessor::new(config);
        let entropy_data = HumanEntropyData {
            raw_data: vec![0u8; 256],
            entropy_bits: 128.0,
            collection_method: HumanEntropyMethod::Biometric,
            timestamp: Utc::now(),
            quality_indicators: HashMap::new(),
        let quality = assessor.assess_entropy_quality(&entropy_data).map_err(|e| {
    tracing::error!("Operation failed: {e:?}");
    beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
})?;
        assert!(quality > 0.0 && quality <= 1.0);
    fn test_tier_elevation_engine() -> beardog_errors::BearDogResult<()> {
        let mut engine = TierElevationEngine::new();
        engine.record_successful_collection(&HumanEntropyMethod::Biometric, 0.9);
        engine.record_successful_collection(&HumanEntropyMethod::Biometric, 0.8);
        let scores = engine.get_provider_scores();
        assert!(scores.contains_key("Biometric"));
        let success_rate = engine.get_method_success_rate(&HumanEntropyMethod::Biometric);
        assert_eq!(success_rate, 100.0);}


    fn test_utility_functions() -> beardog_errors::BearDogResult<()> {
        let collector = utils::create_test_collector();
        assert!(collector.get_statistics().total_collections == 0);
        let capabilities = utils::create_test_capabilities();
        assert!(capabilities.biometric_available);
        assert!(capabilities.behavioral_available);
            raw_data: vec![0u8; 64],
            collection_method: HumanEntropyMethod::Hybrid,
        assert!(utils::validate_entropy_data(&entropy_data, 100.0));
        assert!(!utils::validate_entropy_data(&entropy_data, 200.0));
        let diversity = utils::calculate_entropy_diversity(&[
            HumanEntropyMethod::Biometric,
            HumanEntropyMethod::Behavioral,
            HumanEntropyMethod::Environmental,
        ]);
        assert_eq!(diversity, 0.75);
} 
