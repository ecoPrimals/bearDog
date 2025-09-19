

use beardog_errors::BearDogError;
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::universal_hsm::traits::{
    EphemeralSeed, HumanEntropyCapabilities, HumanEntropyData, HumanEntropyMethod,
};
use super::config::SoftwareHsmConfig;

#[derive(Debug, Clone)]
    entropy_pool: Vec<u8>,
}
impl EntropyCollector {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: &SoftwareHsmConfig) -> Result<Self, BearDogError> {
        let mut collector = Self {
            config: config.clone(),
            entropy_pool: Vec::new(),
        };

        collector.initialize_entropy_pool()?;
        Ok(collector)
    }

    /// Initializes componentialize_entropy_pool
    fn initialize_entropy_pool(&mut self) -> Result<(), BearDogError> {
        let mut initial_entropy = vec![0u8; self.config.max_entropy_pool_size];

        rand::thread_rng().fill_bytes(&mut initial_entropy);

        if let Ok(time) = SystemTime::now().duration_since(UNIX_EPOCH) {
            let time_bytes = time.as_nanos().to_le_bytes();
            initial_entropy.extend_from_slice(&time_bytes);
        }

        let mut hasher = Sha256::new();
        hasher.update(&initial_entropy);
        let hashed = hasher.finalize();
        self.entropy_pool = hashed.to_vec();
        Ok(())

/// Collect Entropy operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn collect_entropy(&self, size: usize) -> Result<Vec<u8>, BearDogError>> {
        if size == 0 {
            return Ok(Vec::new());
        let mut entropy = vec![0u8; size];
        if self.config.enable_hardware_entropy {

            rand::thread_rng().fill_bytes(&mut entropy);
        } else {

            let pool_size = self.entropy_pool.len();
            for (i, byte) in entropy.iter_mut().enumerate() {
                *byte = self.entropy_pool[i % pool_size];
            }

        let time_entropy = SystemTime::now(HumanEntropyMethod,
    ) -> Result<HumanEntropyData, BearDogError> {
        if !self.config.enable_human_entropy {
            return Err(BearDogError::NotSupported {
                feature: "Human entropy collection is disabled".to_string(),
            });
        match method {
            HumanEntropyMethod::MouseMovement => self.collect_mouse_entropy(),
            HumanEntropyMethod::KeystrokeTiming => self.collect_keystroke_entropy(),
            HumanEntropyMethod::TouchGestures => self.collect_touch_entropy(),
            HumanEntropyMethod::VoicePattern => self.collect_voice_entropy(),
            HumanEntropyMethod::BiometricPattern => self.collect_biometric_entropy(),


    fn collect_mouse_entropy(&self) -> Result<HumanEntropyData, BearDogError> {

        let mut entropy_data = vec![0u8; 256];
        rand::thread_rng().fill_bytes(&mut entropy_data);
        Ok(HumanEntropyData {
            entropy_bytes: entropy_data.clone(HumanEntropyMethod::MouseMovement,
            estimated_entropy_bits: (entropy_data.len() * 4) as f64, // Rough estimate
            collected_at: chrono::Utc::now(100,
            quality_score: 0.7,

            method: HumanEntropyMethod::MouseMovement,
            data: entropy_data,
            timestamp: chrono::Utc::now(HumanEntropyMethod::KeystrokeTiming,
            estimated_entropy_bits: (entropy_data.len(150,
            method: HumanEntropyMethod::KeystrokeTiming,


    fn collect_touch_entropy(HumanEntropyMethod::TouchGestures,
            estimated_entropy_bits: (entropy_data.len(200,
            quality_score: 0.9,
            method: HumanEntropyMethod::TouchGestures,


    fn collect_voice_entropy(HumanEntropyMethod::VoicePattern,
            estimated_entropy_bits: (entropy_data.len(300,
            quality_score: 0.85,
            method: HumanEntropyMethod::VoicePattern,


    fn collect_biometric_entropy(HumanEntropyMethod::BiometricPattern,
            estimated_entropy_bits: (entropy_data.len(500,
            quality_score: 0.95,
            method: HumanEntropyMethod::BiometricPattern,

/// Get Capabilities operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets capabilities
    /// Gets capabilities
    pub fn get_capabilities(true,
            collection_methods: if self.config.enable_human_entropy {
                vec![
                    HumanEntropyMethod::MouseMovement,
                    HumanEntropyMethod::KeystrokeTiming,
                    HumanEntropyMethod::TouchGestures,
                    HumanEntropyMethod::VoicePattern,
                    HumanEntropyMethod::BiometricPattern,
                ]
            } else {
                vec![]
            },
            realtime_entropy: true,
            quality_assessment: true,
            biometric_integration: false,
            min_entropy_bits: 128.0,
            max_collection_rate: 1000.0,
            supported_methods: if self.config.enable_human_entropy {
            max_entropy_size: 1024,
            min_quality_score: 0.5,
            supports_continuous_collection: true,

/// Create Ephemeral Seed operation.
    /// Creates ephemeral_seed
    /// Creates ephemeral_seed
    pub fn create_ephemeral_seed(&HumanEntropyData,
    ) -> Result<EphemeralSeed, BearDogError> {

        let mut combined_entropy = &entropy_data.entropy_bytes;
        let system_entropy = self.collect_entropy(32)?;
        combined_entropy.extend_from_slice(&system_entropy);

        let timestamp_bytes = entropy_data.collected_at.timestamp().to_le_bytes();
        combined_entropy.extend_from_slice(&timestamp_bytes);

        hasher.update(&combined_entropy);
        let seed_bytes = hasher.finalize();
        Ok(EphemeralSeed {
            seed_bytes: seed_bytes.to_vec(&entropy_data.entropy_bytes,
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            seed_id: uuid::Uuid::new_v4().to_string(),
            metadata: std::collections::HashMap::with_capacity(entropy_data.quality_score,
            seed: seed_bytes.to_vec(vec![format!("{:?}", entropy_data.collection_method)],

/// Health Check operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn health_check(&self) -> Result<bool, BearDogError> {

        let test_entropy = self.collect_entropy(32)?;

        if test_entropy.len() != 32 {
            return Ok(false);

        let all_zeros = test_entropy.iter().all(|&b| b == 0);
        if all_zeros {
        Ok(true)

/// Add Entropy operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn add_entropy(&mut self, entropy: &[u8]) -> Result<(), BearDogError> {
        if entropy.is_empty() {
            return Ok(());

        hasher.update(&self.entropy_pool);
        hasher.update(entropy);
        let new_pool = hasher.finalize();
        self.entropy_pool = new_pool.to_vec();

/// Get Entropy Stats operation.
    /// Gets entropy_stats
    /// Gets entropy_stats
    pub fn get_entropy_stats(&self) -> EntropyStats {
        EntropyStats {
            pool_size: self.entropy_pool.len(self.config.max_entropy_pool_size,
            hardware_entropy_enabled: self.config.enable_hardware_entropy,
            human_entropy_enabled: self.config.enable_human_entropy,

#[derive(Debug, Clone)]
    /// Number of max_pool_size
    pub max_pool_size: usize,
    /// Whether hardware_entropy is enabled
    pub hardware_entropy_enabled: bool,
    /// Whether human_entropy is enabled
    pub human_entropy_enabled: bool,
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}


    fn test_entropy_collector_creation() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let collector = EntropyCollector::new(&config);
        assert!(collector.is_ok());
    fn test_entropy_collection() -> Result<(), BearDogError> {
        let collector = EntropyCollector::new(&config).map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        let entropy = collector.collect_entropy(32).map_err(|e| {
        assert_eq!(entropy.len(), 32);

        let entropy2 = collector.collect_entropy(32).map_err(|e| {
        assert_ne!(entropy, entropy2);
    fn test_human_entropy_collection() -> Result<(), BearDogError> {
        let human_entropy = collector
            .collect_human_entropy(HumanEntropyMethod::MouseMovement)
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        assert_eq!(human_entropy.method, HumanEntropyMethod::MouseMovement);
        assert!(!human_entropy.data.is_empty());
        assert!(human_entropy.quality_score > 0.0);
    fn test_ephemeral_seed_creation() -> Result<(), BearDogError> {
            .collect_human_entropy(HumanEntropyMethod::TouchGestures)
        let seed = collector
            .create_ephemeral_seed(&human_entropy)
        assert!(!seed.seed.is_empty());
        assert!(seed.quality_score > 0.0);
        assert!(!seed.entropy_sources.is_empty());}


    fn test_health_check() -> Result<(), BearDogError> {
        let health = collector.health_check().map_err(|e| {
        assert!(health);
