

use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashMap;
use tracing::{debug, info, warn};

use super::traits::{
    EphemeralSeed, HumanEntropyCapabilities, HumanEntropyData, HumanEntropyMethod,
};

pub mod collector;

pub use collector::HumanEntropyCollector;

#[derive(Debug, Clone)]
pub struct EntropyCollectionConfig {

    pub min_quality_score: f64,

    pub max_collection_time_ms: u64,

    pub min_entropy_bits: f64,

    pub enable_biometric: bool,

    pub enable_behavioral: bool,

    pub enable_environmental: bool,

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

#[derive(Debug, Default)]
pub struct EntropyCollectionStats {

    pub total_collections: u64,

    pub successful_collections: u64,

    pub failed_collections: u64,

    pub average_quality: f64,

    pub average_collection_time_ms: f64,

    pub quality_by_method: HashMap<String, f64>,

    pub collections_by_method: HashMap<String, u64>,}

impl EntropyCollectionStats {

    pub fn update_collection_stats(&mut self, quality_score: f64, collection_time: std::time::Duration) {
        self.total_collections += 1;
        self.successful_collections += 1;

        let total_quality = self.average_quality * (self.total_collections - 1) as f64 + quality_score;
        self.average_quality = total_quality / self.total_collections as f64;

        let collection_time_ms = collection_time.as_millis() as f64;
        let total_time = self.average_collection_time_ms * (self.total_collections - 1) as f64 + collection_time_ms;
        self.average_collection_time_ms = total_time / self.total_collections as f64;

    pub fn record_failed_collection(&mut self) {
        self.failed_collections += 1;

    pub fn success_rate(&self) -> f64 {
        if self.total_collections == 0 {
            0.0
        } else {
            (self.successful_collections as f64 / self.total_collections as f64) * 100.0

#[derive(Debug)]
pub struct EntropyQualityAssessor {
    config: EntropyCollectionConfig,}

impl EntropyQualityAssessor {

    pub fn new(config: EntropyCollectionConfig) -> Self {
        Self { config }

    pub fn assess_entropy_quality(&self, entropy_data: &HumanEntropyData) -> Result<f64, BearDogError> {
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

        let entropy_ratio = entropy_data.entropy_bits / 256.0; // Normalize to 256 bits max
        quality_score += entropy_ratio.min(1.0) * 0.4; // 40% weight

        let data_size_score = (entropy_data.raw_data.len() as f64 / 1024.0).min(1.0); // Normalize to 1KB
        quality_score += data_size_score * 0.2; // 20% weight

        let method_score = match entropy_data.collection_method {
            HumanEntropyMethod::Biometric => 0.9,     // High trust
            HumanEntropyMethod::Behavioral => 0.7,    // Medium trust
            HumanEntropyMethod::Environmental => 0.5, // Lower trust
            HumanEntropyMethod::Interactive => 0.6,   // Variable trust
            HumanEntropyMethod::Hybrid => 0.8,        // Combined trust
        };
        quality_score += method_score * 0.3; // 30% weight

        let indicator_score = self.assess_quality_indicators(&entropy_data.quality_indicators);
        quality_score += indicator_score * 0.1; // 10% weight

        quality_score = quality_score.max(0.0).min(1.0);
        debug!("✅ Entropy quality assessment: {:.3}", quality_score);
        Ok(quality_score)

    pub fn update_config(&mut self, new_config: EntropyCollectionConfig) {
        self.config = new_config;

    fn assess_quality_indicators(&self, indicators: &HashMap<&str, f64>) -> f64 {
        if indicators.is_empty() {
            return 0.5; // Default score if no indicators
        let sum: f64 = indicators.values().sum();
        let average = sum / indicators.len() as f64;
        average.max(0.0).min(1.0)

pub struct TierElevationEngine {

    provider_scores: HashMap<String, f64>,

    method_success_counts: HashMap<String, u64>,

    method_total_counts: HashMap<String, u64>,}

impl TierElevationEngine {

    pub fn new() -> Self {
            provider_scores: HashMap::with_capacity(16),
            method_success_counts: HashMap::with_capacity(16),
            method_total_counts: HashMap::with_capacity(16),

    pub fn record_successful_collection(&mut self, method: &HumanEntropyMethod, quality_score: f64) {
        let method_name = format_args!("{:?}", method).to_string();

        *self.method_success_counts.entry(method_name.clone()).or_insert(0) += 1;
        *self.method_total_counts.entry(method_name.clone()).or_insert(0) += 1;

        let current_score = self.provider_scores.get(&method_name).unwrap_or(&0.5);
        let new_score = (current_score + quality_score) / 2.0; // Running average
        self.provider_scores.insert(method_name, new_score);
        debug!("📈 Updated provider score for {}: {:.3}", method_name, new_score);

    pub fn record_failed_collection(&mut self, method: &HumanEntropyMethod) {

        let penalty = 0.1;
        let new_score = (current_score - penalty).max(0.0);
        warn!("📉 Decreased provider score for {}: {:.3}", method_name, new_score);

    pub fn get_provider_scores(&self) -> HashMap<String, f64> {
        self.provider_scores.clone()

    pub fn get_method_success_rate(&self, method: &HumanEntropyMethod) -> f64 {
        let successes = self.method_success_counts.get(&method_name).unwrap_or(&0);
        let total = self.method_total_counts.get(&method_name).unwrap_or(&0);
        if *total == 0 {
            (*successes as f64 / *total as f64) * 100.0

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

        recommendations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        recommendations
impl Default for TierElevationEngine {
        Self::new()

pub mod utils {
    use super::*;

    pub fn create_test_collector() -> HumanEntropyCollector {
        HumanEntropyCollector::new(EntropyCollectionConfig::default())

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

    pub fn validate_entropy_data(data: &HumanEntropyData, min_bits: f64) -> bool {
        data.entropy_bits >= min_bits && !data.raw_data.is_empty()

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
    fn test_entropy_config_default() -> Result<(), BearDogError> {
        let config = EntropyCollectionConfig::default();
        assert_eq!(config.min_quality_score, 0.7);
        assert_eq!(config.min_entropy_bits, 128.0);
        assert!(config.enable_biometric);
        Ok(())}

    fn test_collection_stats() -> Result<(), BearDogError> {
        let mut stats = EntropyCollectionStats::default();
        stats.update_collection_stats(0.8, std::time::Duration::from_millis(100));
        stats.update_collection_stats(0.9, std::time::Duration::from_millis(150));
        assert_eq!(stats.total_collections, 2);
        assert_eq!(stats.successful_collections, 2);
        assert_eq!(stats.success_rate(), 100.0);
        assert!((stats.average_quality - 0.85).abs() < 0.01);
    fn test_quality_assessor() -> Result<(), BearDogError> {
        let assessor = EntropyQualityAssessor::new(config);
        let entropy_data = HumanEntropyData {
            raw_data: vec![0u8; 256],
            entropy_bits: 128.0,
            collection_method: HumanEntropyMethod::Biometric,
            timestamp: Utc::now(),
            quality_indicators: HashMap::with_capacity(16),
        let quality = assessor.assess_entropy_quality(&entropy_data).map_err(|e| {
    tracing::error!("Operation failed: {e:?}");
    beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
})?;
        assert!(quality > 0.0 && quality <= 1.0);
    fn test_tier_elevation_engine() -> Result<(), BearDogError> {
        let mut engine = TierElevationEngine::new();
        engine.record_successful_collection(&HumanEntropyMethod::Biometric, 0.9);
        engine.record_successful_collection(&HumanEntropyMethod::Biometric, 0.8);
        let scores = engine.get_provider_scores();
        assert!(scores.contains_key("Biometric"));
        let success_rate = engine.get_method_success_rate(&HumanEntropyMethod::Biometric);
        assert_eq!(success_rate, 100.0);}

    fn test_utility_functions() -> Result<(), BearDogError> {
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
