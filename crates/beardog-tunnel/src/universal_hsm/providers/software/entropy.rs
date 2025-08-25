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


/// # Entropy Collection and Management
///
/// Entropy collection for secure random number generation and human entropy integration.

use beardog_errors::{BearDogError, BearDogResult};
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::universal_hsm::traits::{
    EphemeralSeed, HumanEntropyCapabilities, HumanEntropyData, HumanEntropyMethod,
};
use super::config::SoftwareHsmConfig;
/// Entropy collector for secure random number generation
#[derive(Debug)]
pub struct EntropyCollector {
    config: SoftwareHsmConfig,
    entropy_pool: Vec<u8>,
}
impl EntropyCollector {
    /// Create a new entropy collector
    pub async fn new(config: &SoftwareHsmConfig) -> BearDogResult<Self> {
        let mut collector = Self {
            config: config.clone(),
            entropy_pool: Vec::new(),
        };
        // Initialize entropy pool
        collector.initialize_entropy_pool().await?;
        Ok(collector)
    }
    /// Initialize the entropy pool with system entropy
    async fn initialize_entropy_pool(&mut self) -> BearDogResult<()> {
        let mut initial_entropy = vec![0u8; self.config.max_entropy_pool_size];
        // Collect system entropy
        rand::thread_rng().fill_bytes(&mut initial_entropy);
        // Add timing entropy
        if let Ok(time) = SystemTime::now().duration_since(UNIX_EPOCH) {
            let time_bytes = time.as_nanos().to_le_bytes();
            initial_entropy.extend_from_slice(&time_bytes);
        }
        // Hash to uniform distribution
        let mut hasher = Sha256::new();
        hasher.update(&initial_entropy);
        let hashed = hasher.finalize();
        self.entropy_pool = hashed.to_vec();
        Ok(())
    /// Collect entropy of specified size
    pub async fn collect_entropy(&self, size: usize) -> BearDogResult<Vec<u8>> {
        if size == 0 {
            return Ok(Vec::new());
        let mut entropy = vec![0u8; size];
        if self.config.enable_hardware_entropy {
            // Use hardware RNG if available
            rand::thread_rng().fill_bytes(&mut entropy);
        } else {
            // Use entropy pool
            let pool_size = self.entropy_pool.len();
            for (i, byte) in entropy.iter_mut().enumerate() {
                *byte = self.entropy_pool[i % pool_size];
            }
        // Mix with current time for additional entropy
        let time_entropy = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        hasher.update(&entropy);
        hasher.update(time_entropy.to_le_bytes());
        let final_entropy = hasher.finalize();
        Ok(final_entropy[..size.min(32)].to_vec())
    /// Collect human entropy using various methods
    pub async fn collect_human_entropy(
        &self,
        method: HumanEntropyMethod,
    ) -> BearDogResult<HumanEntropyData> {
        if !self.config.enable_human_entropy {
            return Err(BearDogError::NotSupported {
                feature: "Human entropy collection is disabled".to_string(),
            });
        match method {
            HumanEntropyMethod::MouseMovement => self.collect_mouse_entropy().await,
            HumanEntropyMethod::KeystrokeTiming => self.collect_keystroke_entropy().await,
            HumanEntropyMethod::TouchGestures => self.collect_touch_entropy().await,
            HumanEntropyMethod::VoicePattern => self.collect_voice_entropy().await,
            HumanEntropyMethod::BiometricPattern => self.collect_biometric_entropy().await,
    /// Simulate mouse movement entropy collection
    async fn collect_mouse_entropy(&self) -> BearDogResult<HumanEntropyData> {
        // In a real implementation, this would collect actual mouse movement data
        let mut entropy_data = vec![0u8; 256];
        rand::thread_rng().fill_bytes(&mut entropy_data);
        Ok(HumanEntropyData {
            entropy_bytes: entropy_data.clone(),
            collection_method: HumanEntropyMethod::MouseMovement,
            estimated_entropy_bits: (entropy_data.len() * 4) as f64, // Rough estimate
            collected_at: chrono::Utc::now(),
            collection_duration_ms: 100,
            quality_score: 0.7,
            // Legacy compatibility fields
            method: HumanEntropyMethod::MouseMovement,
            data: entropy_data,
            timestamp: chrono::Utc::now(),
        })
    /// Simulate keystroke timing entropy collection
    async fn collect_keystroke_entropy(&self) -> BearDogResult<HumanEntropyData> {
        let mut entropy_data = vec![0u8; 128];
            collection_method: HumanEntropyMethod::KeystrokeTiming,
            estimated_entropy_bits: (entropy_data.len() * 6) as f64,
            collection_duration_ms: 150,
            method: HumanEntropyMethod::KeystrokeTiming,
    /// Simulate touch gesture entropy collection}


    async fn collect_touch_entropy(&self) -> BearDogResult<HumanEntropyData> {
        let mut entropy_data = vec![0u8; 192];
            collection_method: HumanEntropyMethod::TouchGestures,
            estimated_entropy_bits: (entropy_data.len() * 5) as f64,
            collection_duration_ms: 200,
            quality_score: 0.9,
            method: HumanEntropyMethod::TouchGestures,
    /// Simulate voice pattern entropy collection
    async fn collect_voice_entropy(&self) -> BearDogResult<HumanEntropyData> {
        let mut entropy_data = vec![0u8; 512];
            collection_method: HumanEntropyMethod::VoicePattern,
            estimated_entropy_bits: (entropy_data.len() * 4) as f64,
            collection_duration_ms: 300,
            quality_score: 0.85,
            method: HumanEntropyMethod::VoicePattern,
    /// Simulate biometric pattern entropy collection}


    async fn collect_biometric_entropy(&self) -> BearDogResult<HumanEntropyData> {
        let mut entropy_data = vec![0u8; 1024];
            collection_method: HumanEntropyMethod::BiometricPattern,
            estimated_entropy_bits: (entropy_data.len() * 7) as f64,
            collection_duration_ms: 500,
            quality_score: 0.95,
            method: HumanEntropyMethod::BiometricPattern,
    /// Get human entropy capabilities
    pub async fn get_capabilities(&self) -> BearDogResult<HumanEntropyCapabilities> {
        Ok(HumanEntropyCapabilities {
            supports_ephemeral_seeds: true,
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
    /// Create ephemeral seed from human entropy
    pub async fn create_ephemeral_seed(
        entropy_data: &HumanEntropyData,
    ) -> BearDogResult<EphemeralSeed> {
        // Combine human entropy with system entropy
        let mut combined_entropy = entropy_data.entropy_bytes.clone();
        let system_entropy = self.collect_entropy(32).await?;
        combined_entropy.extend_from_slice(&system_entropy);
        // Add timestamp for uniqueness
        let timestamp_bytes = entropy_data.collected_at.timestamp().to_le_bytes();
        combined_entropy.extend_from_slice(&timestamp_bytes);
        // Hash to create seed
        hasher.update(&combined_entropy);
        let seed_bytes = hasher.finalize();
        Ok(EphemeralSeed {
            seed_bytes: seed_bytes.to_vec(),
            source_entropy: entropy_data.entropy_bytes.clone(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            seed_id: uuid::Uuid::new_v4().to_string(),
            metadata: std::collections::HashMap::new(),
            quality_score: entropy_data.quality_score,
            seed: seed_bytes.to_vec(),
            entropy_sources: vec![format!("{:?}", entropy_data.collection_method)],
    /// Perform health check on entropy collector
    pub async fn health_check(&self) -> BearDogResult<bool> {
        // Test entropy collection
        let test_entropy = self.collect_entropy(32).await?;
        // Check that we got the requested amount
        if test_entropy.len() != 32 {
            return Ok(false);
        // Check that entropy is not all zeros (basic sanity check)
        let all_zeros = test_entropy.iter().all(|&b| b == 0);
        if all_zeros {
        Ok(true)
    /// Add entropy to the pool}


    pub async fn add_entropy(&mut self, entropy: &[u8]) -> BearDogResult<()> {
        if entropy.is_empty() {
            return Ok(());
        // Mix new entropy with existing pool
        hasher.update(&self.entropy_pool);
        hasher.update(entropy);
        let new_pool = hasher.finalize();
        self.entropy_pool = new_pool.to_vec();
    /// Get entropy pool statistics
    pub fn get_entropy_stats(&self) -> EntropyStats {
        EntropyStats {
            pool_size: self.entropy_pool.len(),
            max_pool_size: self.config.max_entropy_pool_size,
            hardware_entropy_enabled: self.config.enable_hardware_entropy,
            human_entropy_enabled: self.config.enable_human_entropy,
/// Entropy pool statistics
#[derive(Debug, Clone)]
pub struct EntropyStats {
    pub pool_size: usize,
    pub max_pool_size: usize,
    pub hardware_entropy_enabled: bool,
    pub human_entropy_enabled: bool,
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}


    async fn test_entropy_collector_creation() -> beardog_errors::BearDogResult<()> {
        let config = SoftwareHsmConfig::default();
        let collector = EntropyCollector::new(&config).await;
        assert!(collector.is_ok());
    async fn test_entropy_collection() -> beardog_errors::BearDogResult<()> {
        let collector = EntropyCollector::new(&config).await.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        let entropy = collector.collect_entropy(32).await.map_err(|e| {
        assert_eq!(entropy.len(), 32);
        // Test that subsequent calls return different values
        let entropy2 = collector.collect_entropy(32).await.map_err(|e| {
        assert_ne!(entropy, entropy2);
    async fn test_human_entropy_collection() -> beardog_errors::BearDogResult<()> {
        let human_entropy = collector
            .collect_human_entropy(HumanEntropyMethod::MouseMovement)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        assert_eq!(human_entropy.method, HumanEntropyMethod::MouseMovement);
        assert!(!human_entropy.data.is_empty());
        assert!(human_entropy.quality_score > 0.0);
    async fn test_ephemeral_seed_creation() -> beardog_errors::BearDogResult<()> {
            .collect_human_entropy(HumanEntropyMethod::TouchGestures)
        let seed = collector
            .create_ephemeral_seed(&human_entropy)
        assert!(!seed.seed.is_empty());
        assert!(seed.quality_score > 0.0);
        assert!(!seed.entropy_sources.is_empty());}


    async fn test_health_check() -> beardog_errors::BearDogResult<()> {
        let health = collector.health_check().await.map_err(|e| {
        assert!(health);
