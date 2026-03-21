// SPDX-License-Identifier: AGPL-3.0-only

// Entropy Hierarchy Engine
//
// This module provides the main entropy hierarchy management engine,
// coordinating entropy collection, validation, and seed management.

use super::monitoring::{EntropyMonitor, PerformanceMetrics};
use super::sources::EntropyMixingEngine;
use super::types::{EntropyClass, EntropyHierarchyConfig, EntropySeed};
use super::validation::EntropyValidator;

use beardog_errors::BearDogError;
use std::collections::HashMap;
use uuid::Uuid;

/// Main entropy hierarchy manager
#[derive(Debug, Clone)]
pub struct EntropyHierarchyManager {
    /// Thresholds and feature flags governing entropy acceptance.
    pub config: EntropyHierarchyConfig,
    /// Mapping of active seeds
    pub active_seeds: HashMap<Uuid, EntropySeed>,
    /// The mixing engine value
    pub mixing_engine: EntropyMixingEngine,
    /// Policy checks for quality, freshness, and optional biometric proof.
    pub validator: EntropyValidator,
    /// The monitor value
    pub monitor: EntropyMonitor,
}

impl EntropyHierarchyManager {
    /// Create new entropy hierarchy manager
    /// Creates a new instance
    #[must_use]
    pub fn new(config: EntropyHierarchyConfig) -> Self {
        let mixing_engine = EntropyMixingEngine::new(&config);
        let validator = EntropyValidator::new(config.clone());
        let monitor = EntropyMonitor::new(&config);

        Self {
            config,
            active_seeds: HashMap::with_capacity(1000),
            mixing_engine,
            validator,
            monitor,
        }
    }

    /// Create human entropy seed
    /// Creates `human_seed`
    /// Creates `human_seed`
    pub fn create_human_seed(
        &mut self,
        entropy_class: EntropyClass,
        entropy_data: Vec<u8>,
    ) -> Result<Uuid, BearDogError> {
        // Validate entropy quality
        if !self.validator.validate_entropy_quality(&entropy_class)? {
            return Err(BearDogError::invalid_input(
                "Entropy quality below threshold",
            ));
        }

        // Create seed
        let seed = self
            .mixing_engine
            .create_entropy_seed(entropy_data, entropy_class)?;
        let seed_id = seed.seed_id;

        self.active_seeds.insert(seed_id, seed);

        Ok(seed_id)
    }

    /// Consumes one logical use of a seed: enforces `max_usage`, updates counters, binds output to `operation`.
    pub fn use_seed(&mut self, seed_id: Uuid, operation: &str) -> Result<Vec<u8>, BearDogError> {
        let seed = self
            .active_seeds
            .get_mut(&seed_id)
            .ok_or_else(|| BearDogError::invalid_input("Seed not found"))?;

        // Check usage limits
        if let Some(max_usage) = seed.metadata.max_usage {
            if seed.metadata.usage_count >= max_usage {
                return Err(BearDogError::invalid_input("Seed usage limit exceeded"));
            }
        }

        // Update usage count
        seed.metadata.usage_count += 1;

        // Generate operation-specific entropy
        let mut result = seed.entropy_data.clone();
        result.extend_from_slice(operation.as_bytes());

        Ok(result)
    }

    /// Validate entropy age and quality
    /// Validates seed
    /// Validates seed
    pub fn validate_seed(&self, seed_id: Uuid) -> Result<bool, BearDogError> {
        let seed = self
            .active_seeds
            .get(&seed_id)
            .ok_or_else(|| BearDogError::invalid_input("Seed not found"))?;

        // Check age
        if !self.validator.validate_entropy_age(&seed.entropy_class)? {
            return Ok(false);
        }

        // Check quality
        self.validator.validate_entropy_quality(&seed.entropy_class)
    }

    /// Gets `seed_info`
    /// Gets `seed_info`
    #[must_use]
    pub fn get_seed_info(&self, seed_id: Uuid) -> Option<&EntropySeed> {
        self.active_seeds.get(&seed_id)
    }

    /// List all active seeds
    #[must_use]
    pub fn list_active_seeds(&self) -> Vec<Uuid> {
        self.active_seeds.keys().copied().collect()
    }

    /// Remove expired seeds
    /// Cleans up `expired_seeds`
    /// Cleans up `expired_seeds`
    pub fn cleanup_expired_seeds(&mut self) -> Result<usize, BearDogError> {
        let mut removed_count = 0;
        let mut to_remove = Vec::new();

        for (seed_id, seed) in &self.active_seeds {
            if let Some(expires_at) = seed.metadata.expires_at {
                if chrono::Utc::now() > expires_at {
                    to_remove.push(*seed_id);
                }
            }
        }

        for seed_id in to_remove {
            self.active_seeds.remove(&seed_id);
            removed_count += 1;
        }

        Ok(removed_count)
    }

    /// Snapshot of operational counters used for observability and tuning (timings are placeholders until wired).
    #[must_use]
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        PerformanceMetrics {
            active_seeds_count: self.active_seeds.len(),
            total_entropy_generated: self
                .active_seeds
                .values()
                .map(|s| s.entropy_data.len())
                .sum::<usize>() as u64,
            average_quality_score: self.calculate_average_quality(),
            seed_creation_time_ms: 0.0,  // Would be tracked in production
            entropy_mixing_time_ms: 0.0, // Would be tracked in production
            validation_time_ms: 0.0,     // Would be tracked in production
            total_operations: self
                .active_seeds
                .values()
                .map(|s| s.metadata.usage_count)
                .sum(),
            successful_operations: self
                .active_seeds
                .values()
                .map(|s| s.metadata.usage_count)
                .sum(), // Simplified - assume all successful for now
            failed_operations: 0, // Would be tracked in production
        }
    }

    /// Calculate average quality score across all seeds
    fn calculate_average_quality(&self) -> f64 {
        if self.active_seeds.is_empty() {
            return 0.0;
        }

        let total_quality: f64 = self
            .active_seeds
            .values()
            .map(|seed| match &seed.entropy_class {
                EntropyClass::HumanLivedExperience { quality_score, .. } => *quality_score,
                EntropyClass::HumanSupervisedMachine { quality_score, .. } => *quality_score,
                EntropyClass::StoreBoughtMachine { quality_score, .. } => *quality_score,
            })
            .sum();

        #[expect(
            clippy::cast_precision_loss,
            reason = "mean quality over seed count; usize to f64 acceptable for analytics"
        )]
        let divisor = self.active_seeds.len() as f64;
        total_quality / divisor
    }

    /// Initialize the manager
    /// Initializes componentialize
    /// Initializes componentialize
    pub const fn initialize(&self) -> Result<(), BearDogError> {
        // Perform any necessary initialization
        Ok(())
    }

    /// Shutdown the manager
    pub fn shutdown(&mut self) -> Result<(), BearDogError> {
        // Clear sensitive data
        self.active_seeds.clear();
        Ok(())
    }
}

impl Default for EntropyHierarchyManager {
    fn default() -> Self {
        Self::new(EntropyHierarchyConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::{BiometricHash, OwnershipProof};
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_entropy_hierarchy_manager_creation() {
        let config = EntropyHierarchyConfig::default();
        let _manager = EntropyHierarchyManager::new(config);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_create_human_seed() -> Result<(), Box<dyn std::error::Error>> {
        let config = EntropyHierarchyConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        let mut manager = EntropyHierarchyManager::new(config);

        let entropy_class = EntropyClass::HumanLivedExperience {
            quality_score: 0.9,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };

        let seed_id = manager.create_human_seed(entropy_class, vec![1, 2, 3, 4])?;
        assert!(manager.get_seed_info(seed_id).is_some());
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_seed_usage() -> Result<(), Box<dyn std::error::Error>> {
        let config = EntropyHierarchyConfig::default();
        let mut manager = EntropyHierarchyManager::new(config);

        let entropy_class = EntropyClass::HumanLivedExperience {
            quality_score: 0.9,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };

        let seed_id = manager.create_human_seed(entropy_class, vec![1, 2, 3, 4])?;
        let result = manager.use_seed(seed_id, "test_operation")?;

        assert!(!result.is_empty());
        assert_eq!(
            manager
                .get_seed_info(seed_id)
                .ok_or("seed not found")?
                .metadata
                .usage_count,
            1
        );
        Ok(())
    }
}
