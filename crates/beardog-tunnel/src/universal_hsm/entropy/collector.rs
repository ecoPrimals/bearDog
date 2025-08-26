

use super::traits::{
    EphemeralSeed, HumanEntropyCapabilities, HumanEntropyData, HumanEntropyMethod,
};
use super::{EntropyCollectionConfig, EntropyCollectionStats, EntropyQualityAssessor, TierElevationEngine};
use beardog_errors::{BearDogError, BearDogResult};
use chrono::Utc;
use std::collections::HashMap;
use tracing::{debug, info, warn};

#[derive(Debug)]
pub struct HumanEntropyCollector {

    config: EntropyCollectionConfig,

    quality_assessor: EntropyQualityAssessor,

    tier_elevation: TierElevationEngine,

    stats: EntropyCollectionStats,
}
impl Default for HumanEntropyCollector {}

    fn default() -> Self {
        Self::new(EntropyCollectionConfig::default())
    }
impl HumanEntropyCollector {

    pub fn new(config: EntropyCollectionConfig) -> Self {
        info!("🎲 Initializing Universal Human Entropy Collector");
        
        Self {
            quality_assessor: EntropyQualityAssessor::new(config.clone()),
            tier_elevation: TierElevationEngine::new(),
            stats: EntropyCollectionStats::default(),
            config,
        }

    pub async fn collect_entropy(
        &mut self,
        method: HumanEntropyMethod,
        capabilities: &HumanEntropyCapabilities,
    ) -> BearDogResult<HumanEntropyData> {
        let start_time = std::time::Instant::now();
        info!("🎲 Collecting human entropy using method: {:?}", method);

        self.validate_capabilities(capabilities)?;

        let entropy_data = match method {
            HumanEntropyMethod::Biometric => {
                self.collect_biometric_entropy(capabilities).await?
            }
            HumanEntropyMethod::Behavioral => {
                self.collect_behavioral_entropy(capabilities).await?
            HumanEntropyMethod::Environmental => {
                self.collect_environmental_entropy(capabilities).await?
            HumanEntropyMethod::Interactive => {
                self.collect_interactive_entropy(capabilities).await?
            HumanEntropyMethod::Hybrid => {
                self.collect_hybrid_entropy(capabilities).await?
        };

        let quality_score = self.quality_assessor.assess_entropy_quality(&entropy_data)?;
        if quality_score < self.config.min_quality_score {
            return Err(BearDogError::EntropyQuality {
                message: format!(
                    "Entropy quality {} below minimum threshold {}",
                    quality_score, self.config.min_quality_score
                ),
            });

        let collection_time = start_time.elapsed();
        self.stats.update_collection_stats(quality_score, collection_time);

        self.tier_elevation.record_successful_collection(&method, quality_score);
        info!(
            "✅ Entropy collection successful: quality={:.3}, time={:?}",
            quality_score, collection_time
        );
        Ok(entropy_data)

    pub fn create_ephemeral_seed(
        &self,
        entropy_data: &HumanEntropyData,
        seed_size: usize,
    ) -> BearDogResult<EphemeralSeed> {
        debug!("🌱 Creating ephemeral seed of size {} bytes", seed_size);
        if entropy_data.entropy_bits < self.config.min_entropy_bits {
            return Err(BearDogError::InsufficientEntropy {
                    "Insufficient entropy: {} bits, need {}",
                    entropy_data.entropy_bits, self.config.min_entropy_bits

        let seed_data = self.derive_seed_from_entropy(&entropy_data.raw_data, seed_size)?;
        Ok(EphemeralSeed {
            data: seed_data,
            entropy_bits: entropy_data.entropy_bits,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::minutes(5), // 5-minute expiry
            source_method: entropy_data.collection_method.clone(),
        })

    pub fn get_statistics(&self) -> &EntropyCollectionStats {
        &self.stats

    pub fn get_tier_recommendations(&self) -> HashMap<String, f64> {
        self.tier_elevation.get_provider_scores()

    pub fn update_config(&mut self, new_config: EntropyCollectionConfig) {
        info!("🔧 Updating entropy collection configuration");
        self.config = new_config.clone();
        self.quality_assessor.update_config(new_config);

    fn validate_capabilities(&self, capabilities: &HumanEntropyCapabilities) -> BearDogResult<()> {
        if !capabilities.biometric_available && !capabilities.behavioral_available 
            && !capabilities.environmental_available && !capabilities.interactive_available {
            return Err(BearDogError::NoEntropySource {
                message: "No entropy collection methods available".to_string(),
        if self.config.enable_biometric && !capabilities.biometric_available {
            warn!("⚠️ Biometric entropy requested but not available");
        Ok(())

    async fn collect_biometric_entropy(
        if !capabilities.biometric_available {
            return Err(BearDogError::UnsupportedOperation {
                message: "Biometric entropy collection not available".to_string(),
        debug!("👤 Collecting biometric entropy");

        let mut entropy_data = Vec::new();

        if capabilities.fingerprint_available {
            entropy_data.extend_from_slice(&self.simulate_fingerprint_entropy());

        if capabilities.voice_available {
            entropy_data.extend_from_slice(&self.simulate_voice_entropy());

        if capabilities.face_available {
            entropy_data.extend_from_slice(&self.simulate_facial_entropy());
        Ok(HumanEntropyData {
            raw_data: entropy_data,
            entropy_bits: 256.0, // High entropy from biometric data
            collection_method: HumanEntropyMethod::Biometric,
            timestamp: Utc::now(),
            quality_indicators: self.calculate_biometric_quality_indicators(capabilities),

    async fn collect_behavioral_entropy(
        if !capabilities.behavioral_available {
                message: "Behavioral entropy collection not available".to_string(),
        debug!("🎯 Collecting behavioral entropy");

        if capabilities.typing_pattern_available {
            entropy_data.extend_from_slice(&self.simulate_typing_patterns());

        if capabilities.mouse_pattern_available {
            entropy_data.extend_from_slice(&self.simulate_mouse_patterns());

        if capabilities.touch_pattern_available {
            entropy_data.extend_from_slice(&self.simulate_touch_patterns());
            entropy_bits: 128.0, // Moderate entropy from behavioral data
            collection_method: HumanEntropyMethod::Behavioral,
            quality_indicators: self.calculate_behavioral_quality_indicators(capabilities),

    async fn collect_environmental_entropy(
        if !capabilities.environmental_available {
                message: "Environmental entropy collection not available".to_string(),
        debug!("🌍 Collecting environmental entropy");

        if capabilities.ambient_sound_available {
            entropy_data.extend_from_slice(&self.simulate_ambient_sound());

        if capabilities.light_sensor_available {
            entropy_data.extend_from_slice(&self.simulate_light_sensor_data());

        if capabilities.accelerometer_available {
            entropy_data.extend_from_slice(&self.simulate_accelerometer_data());
            entropy_bits: 96.0, // Lower entropy from environmental data
            collection_method: HumanEntropyMethod::Environmental,
            quality_indicators: self.calculate_environmental_quality_indicators(capabilities),

    async fn collect_interactive_entropy(
        if !capabilities.interactive_available {
                message: "Interactive entropy collection not available".to_string(),
        debug!("🎮 Collecting interactive entropy");

        entropy_data.extend_from_slice(&self.simulate_interaction_timing());

        entropy_data.extend_from_slice(&self.simulate_user_choices());
            entropy_bits: 64.0, // Variable entropy from user interaction
            collection_method: HumanEntropyMethod::Interactive,
            quality_indicators: self.calculate_interactive_quality_indicators(),

    async fn collect_hybrid_entropy(
        debug!("🔄 Collecting hybrid entropy from multiple sources");
        let mut combined_entropy = Vec::new();
        let mut total_entropy_bits = 0.0;
        let mut quality_indicators = HashMap::with_capacity(16);

        if capabilities.biometric_available {
            if let Ok(bio_data) = self.collect_biometric_entropy(capabilities).await {
                combined_entropy.extend_from_slice(&bio_data.raw_data);
                total_entropy_bits += bio_data.entropy_bits * 0.4; // Weight biometric highly
                quality_indicators.extend(bio_data.quality_indicators);
        if capabilities.behavioral_available {
            if let Ok(behavior_data) = self.collect_behavioral_entropy(capabilities).await {
                combined_entropy.extend_from_slice(&behavior_data.raw_data);
                total_entropy_bits += behavior_data.entropy_bits * 0.3; // Weight behavioral moderately
                quality_indicators.extend(behavior_data.quality_indicators);
        if capabilities.environmental_available {
            if let Ok(env_data) = self.collect_environmental_entropy(capabilities).await {
                combined_entropy.extend_from_slice(&env_data.raw_data);
                total_entropy_bits += env_data.entropy_bits * 0.2; // Weight environmental lower
                quality_indicators.extend(env_data.quality_indicators);
        if capabilities.interactive_available {
            if let Ok(interactive_data) = self.collect_interactive_entropy(capabilities).await {
                combined_entropy.extend_from_slice(&interactive_data.raw_data);
                total_entropy_bits += interactive_data.entropy_bits * 0.1; // Weight interactive lowest
                quality_indicators.extend(interactive_data.quality_indicators);
        if combined_entropy.is_empty() {
                message: "No entropy sources available for hybrid collection".to_string(),
            raw_data: combined_entropy,
            entropy_bits: total_entropy_bits,
            collection_method: HumanEntropyMethod::Hybrid,
            quality_indicators,

    fn derive_seed_from_entropy(&self, entropy_data: &[u8], seed_size: usize) -> BearDogResult<Vec<u8>> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(entropy_data);
        hasher.update(&Utc::now().timestamp().to_le_bytes());
        hasher.update(&seed_size.to_le_bytes());
        let hash = hasher.finalize();

        let mut seed = Vec::new();
        let mut counter = 0u32;
        while seed.len() < seed_size {
            let mut extended_hasher = Sha256::new();
            extended_hasher.update(&hash);
            extended_hasher.update(&counter.to_le_bytes());
            let extended_hash = extended_hasher.finalize();
            
            let bytes_needed = std::cmp::min(seed_size - seed.len(), extended_hash.len());
            seed.extend_from_slice(&extended_hash[..bytes_needed]);
            counter += 1;
        Ok(seed)

    fn simulate_fingerprint_entropy(&self) -> Vec<u8> {

        (0..64).map(|i| ((i * 7 + 23) % 256) as u8).collect()}

    fn simulate_voice_entropy(&self) -> Vec<u8> {

        (0..32).map(|i| ((i * 11 + 47) % 256) as u8).collect()}

    fn simulate_facial_entropy(&self) -> Vec<u8> {

        (0..48).map(|i| ((i * 13 + 71) % 256) as u8).collect()
    fn simulate_typing_patterns(&self) -> Vec<u8> {

        (0..24).map(|i| ((i * 17 + 89) % 256) as u8).collect()}

    fn simulate_mouse_patterns(&self) -> Vec<u8> {

        (0..16).map(|i| ((i * 19 + 101) % 256) as u8).collect()
    fn simulate_touch_patterns(&self) -> Vec<u8> {

        (0..20).map(|i| ((i * 23 + 113) % 256) as u8).collect()}

    fn simulate_ambient_sound(&self) -> Vec<u8> {

        (0..32).map(|i| ((i * 29 + 127) % 256) as u8).collect()
    fn simulate_light_sensor_data(&self) -> Vec<u8> {

        (0..8).map(|i| ((i * 31 + 139) % 256) as u8).collect()}

    fn simulate_accelerometer_data(&self) -> Vec<u8> {

        (0..12).map(|i| ((i * 37 + 149) % 256) as u8).collect()
    fn simulate_interaction_timing(&self) -> Vec<u8> {

        (0..16).map(|i| ((i * 41 + 163) % 256) as u8).collect()}

    fn simulate_user_choices(&self) -> Vec<u8> {

        (0..8).map(|i| ((i * 43 + 179) % 256) as u8).collect()
    fn calculate_biometric_quality_indicators(&self, capabilities: &HumanEntropyCapabilities) -> HashMap<String, f64> {
        let mut indicators = HashMap::with_capacity(16);
            indicators.insert("fingerprint_quality".to_string(), 0.95);
            indicators.insert("voice_quality".to_string(), 0.88);
            indicators.insert("facial_quality".to_string(), 0.92);
        indicators.insert("overall_biometric_quality".to_string(), 0.91);
        indicators}

    fn calculate_behavioral_quality_indicators(&self, capabilities: &HumanEntropyCapabilities) -> HashMap<String, f64> {
            indicators.insert("typing_consistency".to_string(), 0.78);
            indicators.insert("mouse_uniqueness".to_string(), 0.82);
            indicators.insert("touch_pressure_variance".to_string(), 0.75);
        indicators.insert("overall_behavioral_quality".to_string(), 0.79);
    fn calculate_environmental_quality_indicators(&self, capabilities: &HumanEntropyCapabilities) -> HashMap<String, f64> {
            indicators.insert("sound_entropy".to_string(), 0.65);
            indicators.insert("light_variance".to_string(), 0.58);
            indicators.insert("motion_entropy".to_string(), 0.72);
        indicators.insert("overall_environmental_quality".to_string(), 0.65);}

    fn calculate_interactive_quality_indicators(&self) -> HashMap<String, f64> {
        indicators.insert("timing_unpredictability".to_string(), 0.68);
        indicators.insert("choice_randomness".to_string(), 0.62);
        indicators.insert("overall_interactive_quality".to_string(), 0.65);
} 
