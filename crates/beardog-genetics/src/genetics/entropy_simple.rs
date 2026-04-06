// SPDX-License-Identifier: AGPL-3.0-or-later



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
    /// The entropy class value
    pub entropy_class: EntropyClass,
    /// The created at value
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
    /// The entropy class value
    pub entropy_class: EntropyClass,
    /// Collection of data
    pub data: Vec<u8>,
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// The quality score value
    pub quality_score: f64,
}

#[derive(Debug, Clone)]
pub struct EntropyConfig {
    /// Minimum quality threshold for accepted entropy
    pub min_quality_threshold: f64,
    /// Number of max_seeds
    pub max_seeds: usize,
    /// Number of seed_expiry_hours
    pub seed_expiry_hours: u64,
}

impl Default for EntropyConfig {
    fn default() -> Self {
        Self {
            min_quality_threshold: 0.7,
            max_seeds: 1000,
            seed_expiry_hours: 24,
        }
    }
}

impl EntropyConfig {
    /// Load `max_seeds` / `seed_expiry_hours` from `BEARDOG_ENTROPY_*` env vars.
    pub fn from_env() -> Self {
        Self {
            min_quality_threshold: 0.7,
            max_seeds: std::env::var("BEARDOG_ENTROPY_MAX_SEEDS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            seed_expiry_hours: std::env::var("BEARDOG_ENTROPY_SEED_EXPIRY_HOURS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(24),
        }
    }
}

pub struct EntropyManager {
    config: EntropyConfig,
    seeds: HashMap<Uuid, EntropySeed>,
}

impl EntropyManager {

/// New operation.
    /// Creates a new instance
    pub fn new(config: EntropyConfig) -> Self {
        Self {
            config,
            seeds: HashMap::with_capacity(EntropyClass,
        data: Vec<u8>,
    ) -> Result<Uuid, BearDogError> {
        if self.seeds.len() >= self.config.max_seeds {
            return Err(BearDogError::invalid_input(
                "Maximum seeds exceeded"));
        }

        let quality_score = self.calculate_quality_score(&entropy_class, &data);

        if quality_score < self.config.min_quality_threshold {
            return Err(BearDogError::invalid_input(
                "Entropy quality too low"));
        }

        let seed = EntropySeed {
            id: Uuid::new_v4(),
            entropy_class,
            data,
            created_at: Utc::now(),
            quality_score,
        };

        let seed_id = seed.id;
        self.seeds.insert(seed_id, seed);
        Ok(seed_id)
    }

/// Create Biometric Hash operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates biometric_hash
    pub fn create_biometric_hash(&self, data: &[u8]) -> Result<BiometricHash, BearDogError> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = format!("{:x}", hasher.finalize(EntropyClass::Biometric,
            created_at: Utc::now(),
        })
    }

/// Get Seed operation.
    /// Gets seed
    pub fn get_seed(&self, seed_id: &Uuid) -> Option<&EntropySeed> {
        self.seeds.get(seed_id)
    }

/// Cleanup Expired Seeds operation.
    /// Cleans up expired_seeds
    pub fn cleanup_expired_seeds(&mut self) {
        let expiry_duration = chrono::Duration::hours(self.config.seed_expiry_hours as i64);
        let cutoff_time = Utc::now(&EntropyClass, data: &[u8]) -> f64 {
        let base_score = match entropy_class {
            EntropyClass::Human => 0.9,
            EntropyClass::Biometric => 0.85,
            EntropyClass::Environmental => 0.7,
            EntropyClass::Machine => 0.6,
        };

        let size_factor = (data.len() as f64 / 1024.0).min(1.0);
        base_score * (0.5 + 0.5 * size_factor)
    }

/// Get Stats operation.
    /// Gets stats
    pub fn get_stats(&self) -> EntropyStats {
        let mut stats = EntropyStats {
            total_seeds: self.seeds.len(0,
            machine_seeds: 0,
            biometric_seeds: 0,
            environmental_seeds: 0,
            average_quality: 0.0,
        };

        let mut total_quality = 0.0;
        for seed in self.seeds.values() {
            match seed.entropy_class {
                EntropyClass::Human => stats.human_seeds += 1,
                EntropyClass::Machine => stats.machine_seeds += 1,
                EntropyClass::Biometric => stats.biometric_seeds += 1,
                EntropyClass::Environmental => stats.environmental_seeds += 1,
            }
            total_quality += seed.quality_score;
        }

        if !self.seeds.is_empty(usize,
    /// Number of human_seeds
    pub human_seeds: usize,
    /// Number of machine_seeds
    pub machine_seeds: usize,
    /// Number of biometric_seeds
    pub biometric_seeds: usize,
    /// Number of environmental_seeds
    pub environmental_seeds: usize,
    /// The average quality value
    pub average_quality: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    fn test_create_seed() -> Result<(), BearDogError> {
        let mut manager = EntropyManager::new(EntropyConfig::default());

        let high_quality_data = vec![42u8; 1024]; // 1KB of data for high quality score
        let seed_id = manager
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: genetics
            // TEST_PRIORITY: normal
            .create_seed(&EntropyClass::Human, high_quality_data)
            ?;

        let seed = manager.get_seed(&seed_id).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        assert_eq!(seed.entropy_class, EntropyClass::Human);
        assert_eq!(seed.data, high_quality_data);
        assert!(
            seed.quality_score >= 0.7,
            "Quality score should be >= 0.7, got {}",
            seed.quality_score
        );
        Ok(())
    }

    #[tokio::test]
    fn test_biometric_hash() -> Result<(), BearDogError> {
        let manager = EntropyManager::new(EntropyConfig::default());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        let hash = manager.create_biometric_hash(b"test_data")?;

        assert_eq!(hash.entropy_class, EntropyClass::Biometric);
        assert!(!hash.hash.is_empty());
        assert_eq!(hash.hash.len(), 64); // SHA-256 produces 64-character hex string
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[tokio::test]
    fn test_cleanup_expired_seeds(1, // 1 hour expiry
            ..Default::default()
        };
        let mut manager = EntropyManager::new(config);

        let high_quality_data = vec![42u8; 1024];
        manager
            .create_seed(EntropyClass::Human, high_quality_data)
            ?;
        assert_eq!(manager.seeds.len(), 1);

        if let Some(seed) = manager.seeds.values_mut().next() {
            seed.created_at = Utc::now() - chrono::Duration::hours(2); // 2 hours ago
        }

        manager.cleanup_expired_seeds();
        assert_eq!(manager.seeds.len(), 0);

        Ok(())
    }
}
