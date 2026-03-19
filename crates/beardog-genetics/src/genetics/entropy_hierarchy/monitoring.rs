// SPDX-License-Identifier: AGPL-3.0-only

// Entropy Hierarchy Monitoring
//
// This module provides monitoring and analytics capabilities for entropy hierarchy operations.

use super::types::{EntropyClass, EntropyHierarchyConfig, EntropySeed};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct EntropyMonitor {
    #[allow(dead_code)] // Used for configuration but not yet fully implemented
    config: EntropyMonitoringConfig,
    metrics: PerformanceMetrics,
}

impl EntropyMonitor {
    /// Create new entropy monitor
    /// Creates a new instance
    #[must_use]
    pub fn new(config: &EntropyHierarchyConfig) -> Self {
        Self {
            config: EntropyMonitoringConfig {
                enable_detailed_analytics: true,
                cleanup_interval_seconds: 3600, // 1 hour
                max_seed_age_days: 30,
                quality_threshold: config.min_human_quality,
            },
            metrics: PerformanceMetrics::default(),
        }
    }

    /// Clean up expired seeds
    /// Cleans up `expired_seeds`
    /// Cleans up `expired_seeds`
    pub fn cleanup_expired_seeds(&mut self, active_seeds: &mut HashMap<Uuid, EntropySeed>) {
        let now = Utc::now();
        let mut expired_seeds = Vec::new();

        for (seed_id, seed) in active_seeds.iter() {
            if let Some(expires_at) = seed.metadata.expires_at {
                if now > expires_at {
                    expired_seeds.push(*seed_id);
                }
            }
        }

        for seed_id in expired_seeds {
            active_seeds.remove(&seed_id);
        }
    }

    /// Get entropy hierarchy statistics
    /// Gets statistics
    /// Gets statistics
    #[must_use]
    pub fn get_statistics(
        &self,
        active_seeds: &HashMap<Uuid, EntropySeed>,
    ) -> EntropyHierarchyStats {
        let mut human_entropy_seeds = 0;
        let mut human_supervised_seeds = 0;
        let mut machine_entropy_seeds = 0;

        for seed in active_seeds.values() {
            match &seed.entropy_class {
                EntropyClass::HumanLivedExperience { .. } => human_entropy_seeds += 1,
                EntropyClass::HumanSupervisedMachine { .. } => human_supervised_seeds += 1,
                EntropyClass::StoreBoughtMachine { .. } => machine_entropy_seeds += 1,
            }
        }

        EntropyHierarchyStats {
            total_seeds: active_seeds.len(),
            human_entropy_seeds,
            human_supervised_seeds,
            machine_entropy_seeds,
            event_seeds: 0,          // Not implemented in simplified version
            self_sovereign_seeds: 0, // Not implemented in simplified version
        }
    }

    /// Get analytics data
    /// Gets analytics
    /// Gets analytics
    #[must_use]
    pub fn get_analytics(&self, active_seeds: &HashMap<Uuid, EntropySeed>) -> EntropyAnalytics {
        let stats = self.get_statistics(active_seeds);
        let mut quality_scores = Vec::new();

        for seed in active_seeds.values() {
            let quality_score = match &seed.entropy_class {
                EntropyClass::HumanLivedExperience { quality_score, .. } => *quality_score,
                EntropyClass::HumanSupervisedMachine { quality_score, .. } => *quality_score,
                EntropyClass::StoreBoughtMachine { quality_score, .. } => *quality_score,
            };
            quality_scores.push(quality_score);
        }

        let average_quality = if quality_scores.is_empty() {
            0.0
        } else {
            quality_scores.iter().sum::<f64>() / quality_scores.len() as f64
        };

        EntropyAnalytics {
            stats,
            average_quality,
            quality_distribution: self.calculate_quality_distribution(&quality_scores),
            age_distribution: self.calculate_age_distribution(active_seeds),
        }
    }

    /// Get health status
    /// Gets `health_status`
    /// Gets `health_status`
    #[must_use]
    pub fn get_health_status(&self) -> EntropyHealthStatus {
        EntropyHealthStatus {
            overall_health: "Healthy".to_string(),
            quality_score: 0.9, // Would be calculated from actual metrics
            performance_metrics: self.metrics.clone(),
            last_updated: Utc::now(),
        }
    }

    /// Calculate quality distribution
    fn calculate_quality_distribution(&self, quality_scores: &[f64]) -> HashMap<String, u32> {
        let mut distribution = HashMap::new();

        for &score in quality_scores {
            let category = if score >= 0.9 {
                "excellent"
            } else if score >= 0.8 {
                "good"
            } else if score >= 0.7 {
                "acceptable"
            } else {
                "poor"
            };

            *distribution.entry(category.to_string()).or_insert(0) += 1;
        }

        distribution
    }

    /// Calculate age distribution
    fn calculate_age_distribution(
        &self,
        active_seeds: &HashMap<Uuid, EntropySeed>,
    ) -> HashMap<String, u32> {
        let mut distribution = HashMap::new();
        let now = Utc::now();

        for seed in active_seeds.values() {
            let age_hours = (now - seed.metadata.created_at).num_hours();
            let category = if age_hours < 1 {
                "very_new"
            } else if age_hours < 24 {
                "new"
            } else if age_hours < 168 {
                // 1 week
                "recent"
            } else {
                "old"
            };

            *distribution.entry(category.to_string()).or_insert(0) += 1;
        }

        distribution
    }

    /// Updates metrics
    /// Updates metrics
    pub fn update_metrics(&mut self, metrics: PerformanceMetrics) {
        self.metrics = metrics;
    }

    #[must_use]
    pub const fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }
}

/// Entropy hierarchy statistics
#[derive(Debug, Clone)]
pub struct EntropyHierarchyStats {
    /// Number of `total_seeds`
    pub total_seeds: usize,
    /// Number of `human_entropy_seeds`
    pub human_entropy_seeds: u32,
    /// Number of `human_supervised_seeds`
    pub human_supervised_seeds: u32,
    /// Number of `machine_entropy_seeds`
    pub machine_entropy_seeds: u32,
    /// Number of `event_seeds`
    pub event_seeds: u32,
    /// Number of `self_sovereign_seeds`
    pub self_sovereign_seeds: u32,
}

/// Entropy analytics data
#[derive(Debug, Clone)]
pub struct EntropyAnalytics {
    /// The stats value
    pub stats: EntropyHierarchyStats,
    /// The average quality value
    pub average_quality: f64,
    /// Mapping of quality distribution
    pub quality_distribution: HashMap<String, u32>,
    /// Mapping of age distribution
    pub age_distribution: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
pub struct EntropyHealthStatus {
    /// The overall health value
    pub overall_health: String,
    /// The quality score value
    pub quality_score: f64,
    pub performance_metrics: PerformanceMetrics,
    /// The last updated value
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// Number of `active_seeds`
    pub active_seeds_count: usize,
    /// Number of `total_entropy_generated`
    pub total_entropy_generated: u64,
    /// The average quality score value
    pub average_quality_score: f64,
    pub seed_creation_time_ms: f64,
    pub entropy_mixing_time_ms: f64,
    pub validation_time_ms: f64,
    /// Number of `total_operations`
    pub total_operations: u64,
    /// Number of `successful_operations`
    pub successful_operations: u64,
    /// Number of `failed_operations`
    pub failed_operations: u64,
}

/// Monitoring configuration
#[derive(Debug, Clone, Default)]
pub struct EntropyMonitoringConfig {
    /// Whether `enable_detailed_analytics` is enabled
    pub enable_detailed_analytics: bool,
    /// Number of `cleanup_interval_seconds`
    pub cleanup_interval_seconds: u64,
    /// Number of `max_seed_age_days`
    pub max_seed_age_days: u32,
    /// The quality threshold value
    pub quality_threshold: f64,
}

#[cfg(test)]
mod tests {
    use super::super::types::{
        BiometricHash, HumanIdentity, MachineEntropySource, MachineSourceType, OwnershipProof,
        SeedMetadata, VerificationLevel,
    };
    use super::*;
    use chrono::Duration;

    fn create_test_config() -> EntropyHierarchyConfig {
        EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        }
    }

    fn create_test_entropy_seed(
        entropy_class: EntropyClass,
        created_at: DateTime<Utc>,
        expires_at: Option<DateTime<Utc>>,
    ) -> EntropySeed {
        EntropySeed {
            seed_id: Uuid::new_v4(),
            entropy_class,
            entropy_data: vec![1, 2, 3, 4, 5],
            metadata: SeedMetadata {
                created_at,
                expires_at,
                usage_count: 0,
                max_usage: Some(100),
            },
        }
    }

    fn create_human_lived_experience_class(quality_score: f64) -> EntropyClass {
        EntropyClass::HumanLivedExperience {
            quality_score,
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
        }
    }

    fn create_human_supervised_machine_class(quality_score: f64) -> EntropyClass {
        EntropyClass::HumanSupervisedMachine {
            quality_score,
            machine_source: MachineEntropySource {
                source_type: MachineSourceType::CSPRNG {
                    algorithm: "ChaCha20".to_string(),
                    seed_source: "OS-RNG".to_string(),
                },
                algorithm: "ChaCha20".to_string(),
                seed_source: "OS-RNG".to_string(),
                quality_metrics: HashMap::new(),
            },
            human_validator: HumanIdentity {
                identity_id: "validator-1".to_string(),
                identity_hash: vec![1, 2, 3],
                verification_level: VerificationLevel::Enhanced,
                verified_at: Utc::now(),
            },
            validation_timestamp: Utc::now(),
        }
    }

    fn create_store_bought_machine_class(quality_score: f64) -> EntropyClass {
        EntropyClass::StoreBoughtMachine {
            quality_score,
            source_type: MachineEntropySource {
                source_type: MachineSourceType::HRNG {
                    device_type: "TPM".to_string(),
                    entropy_rate: 1.0,
                },
                algorithm: "AES-CTR-DRBG".to_string(),
                seed_source: "Hardware".to_string(),
                quality_metrics: HashMap::new(),
            },
            generation_timestamp: Utc::now(),
            reproducibility_index: 0.3,
        }
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_entropy_monitor_creation() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        assert_eq!(monitor.metrics.active_seeds_count, 0);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_entropy_monitor_creation_with_default_config() {
        let config = EntropyHierarchyConfig::default();
        let monitor = EntropyMonitor::new(&config);
        assert_eq!(monitor.config.cleanup_interval_seconds, 3600);
        assert_eq!(monitor.config.max_seed_age_days, 30);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_cleanup_expired_seeds_empty() {
        let config = create_test_config();
        let mut monitor = EntropyMonitor::new(&config);
        let mut active_seeds = HashMap::new();

        let initial_count = active_seeds.len();
        monitor.cleanup_expired_seeds(&mut active_seeds);
        assert_eq!(active_seeds.len(), initial_count);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_cleanup_expired_seeds_none_expired() {
        let config = create_test_config();
        let mut monitor = EntropyMonitor::new(&config);
        let mut active_seeds = HashMap::new();

        // Create seeds with future expiry
        let future_expiry = Utc::now() + Duration::hours(24);
        let seed1 = create_test_entropy_seed(
            create_human_lived_experience_class(0.9),
            Utc::now(),
            Some(future_expiry),
        );
        let seed2 = create_test_entropy_seed(
            create_human_supervised_machine_class(0.8),
            Utc::now(),
            Some(future_expiry),
        );

        active_seeds.insert(seed1.seed_id, seed1);
        active_seeds.insert(seed2.seed_id, seed2);

        monitor.cleanup_expired_seeds(&mut active_seeds);
        assert_eq!(active_seeds.len(), 2);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_cleanup_expired_seeds_removes_expired() {
        let config = create_test_config();
        let mut monitor = EntropyMonitor::new(&config);
        let mut active_seeds = HashMap::new();

        // Create expired seed
        let past_expiry = Utc::now() - Duration::hours(1);
        let expired_seed = create_test_entropy_seed(
            create_human_lived_experience_class(0.9),
            Utc::now() - Duration::hours(2),
            Some(past_expiry),
        );

        // Create valid seed
        let future_expiry = Utc::now() + Duration::hours(24);
        let valid_seed = create_test_entropy_seed(
            create_human_supervised_machine_class(0.8),
            Utc::now(),
            Some(future_expiry),
        );

        let expired_id = expired_seed.seed_id;
        let valid_id = valid_seed.seed_id;

        active_seeds.insert(expired_id, expired_seed);
        active_seeds.insert(valid_id, valid_seed);

        assert_eq!(active_seeds.len(), 2);
        monitor.cleanup_expired_seeds(&mut active_seeds);
        assert_eq!(active_seeds.len(), 1);
        assert!(active_seeds.contains_key(&valid_id));
        assert!(!active_seeds.contains_key(&expired_id));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_cleanup_expired_seeds_no_expiry() {
        let config = create_test_config();
        let mut monitor = EntropyMonitor::new(&config);
        let mut active_seeds = HashMap::new();

        // Create seed with no expiry
        let seed =
            create_test_entropy_seed(create_human_lived_experience_class(0.9), Utc::now(), None);

        active_seeds.insert(seed.seed_id, seed);
        monitor.cleanup_expired_seeds(&mut active_seeds);
        assert_eq!(active_seeds.len(), 1);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_get_statistics_empty() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let active_seeds = HashMap::new();

        let stats = monitor.get_statistics(&active_seeds);
        assert_eq!(stats.total_seeds, 0);
        assert_eq!(stats.human_entropy_seeds, 0);
        assert_eq!(stats.human_supervised_seeds, 0);
        assert_eq!(stats.machine_entropy_seeds, 0);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_get_statistics_mixed_seeds() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let mut active_seeds = HashMap::new();

        // Add human lived experience seeds
        let seed1 =
            create_test_entropy_seed(create_human_lived_experience_class(0.95), Utc::now(), None);
        let seed2 =
            create_test_entropy_seed(create_human_lived_experience_class(0.92), Utc::now(), None);

        // Add human supervised machine seeds
        let seed3 = create_test_entropy_seed(
            create_human_supervised_machine_class(0.85),
            Utc::now(),
            None,
        );

        // Add store bought machine seeds
        let seed4 =
            create_test_entropy_seed(create_store_bought_machine_class(0.65), Utc::now(), None);
        let seed5 =
            create_test_entropy_seed(create_store_bought_machine_class(0.60), Utc::now(), None);

        active_seeds.insert(seed1.seed_id, seed1);
        active_seeds.insert(seed2.seed_id, seed2);
        active_seeds.insert(seed3.seed_id, seed3);
        active_seeds.insert(seed4.seed_id, seed4);
        active_seeds.insert(seed5.seed_id, seed5);

        let stats = monitor.get_statistics(&active_seeds);
        assert_eq!(stats.total_seeds, 5);
        assert_eq!(stats.human_entropy_seeds, 2);
        assert_eq!(stats.human_supervised_seeds, 1);
        assert_eq!(stats.machine_entropy_seeds, 2);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_get_analytics_empty() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let active_seeds = HashMap::new();

        let analytics = monitor.get_analytics(&active_seeds);
        assert_eq!(analytics.stats.total_seeds, 0);
        assert_eq!(analytics.average_quality, 0.0);
        assert!(analytics.quality_distribution.is_empty());
        assert!(analytics.age_distribution.is_empty());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_get_analytics_with_seeds() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let mut active_seeds = HashMap::new();

        // Add seeds with different quality scores
        let seed1 =
            create_test_entropy_seed(create_human_lived_experience_class(0.95), Utc::now(), None);
        let seed2 = create_test_entropy_seed(
            create_human_supervised_machine_class(0.85),
            Utc::now(),
            None,
        );
        let seed3 =
            create_test_entropy_seed(create_store_bought_machine_class(0.75), Utc::now(), None);

        active_seeds.insert(seed1.seed_id, seed1);
        active_seeds.insert(seed2.seed_id, seed2);
        active_seeds.insert(seed3.seed_id, seed3);

        let analytics = monitor.get_analytics(&active_seeds);
        assert_eq!(analytics.stats.total_seeds, 3);
        assert!((analytics.average_quality - 0.85).abs() < 0.01);
        assert!(!analytics.quality_distribution.is_empty());
        assert!(!analytics.age_distribution.is_empty());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_get_health_status() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);

        let health = monitor.get_health_status();
        assert_eq!(health.overall_health, "Healthy");
        assert_eq!(health.quality_score, 0.9);
        assert_eq!(
            health.performance_metrics.active_seeds_count,
            monitor.metrics.active_seeds_count
        );
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_calculate_quality_distribution_excellent() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let quality_scores = vec![0.95, 0.92, 0.91];

        let distribution = monitor.calculate_quality_distribution(&quality_scores);
        assert_eq!(distribution.get("excellent"), Some(&3));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_calculate_quality_distribution_good() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let quality_scores = vec![0.85, 0.82, 0.88];

        let distribution = monitor.calculate_quality_distribution(&quality_scores);
        assert_eq!(distribution.get("good"), Some(&3));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_calculate_quality_distribution_acceptable() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let quality_scores = vec![0.75, 0.72, 0.78];

        let distribution = monitor.calculate_quality_distribution(&quality_scores);
        assert_eq!(distribution.get("acceptable"), Some(&3));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_calculate_quality_distribution_poor() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let quality_scores = vec![0.65, 0.55, 0.45];

        let distribution = monitor.calculate_quality_distribution(&quality_scores);
        assert_eq!(distribution.get("poor"), Some(&3));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_calculate_quality_distribution_mixed() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let quality_scores = vec![0.95, 0.85, 0.75, 0.65];

        let distribution = monitor.calculate_quality_distribution(&quality_scores);
        assert_eq!(distribution.get("excellent"), Some(&1));
        assert_eq!(distribution.get("good"), Some(&1));
        assert_eq!(distribution.get("acceptable"), Some(&1));
        assert_eq!(distribution.get("poor"), Some(&1));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_calculate_quality_distribution_empty() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let quality_scores: Vec<f64> = vec![];

        let distribution = monitor.calculate_quality_distribution(&quality_scores);
        assert!(distribution.is_empty());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_calculate_age_distribution_very_new() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let mut active_seeds = HashMap::new();

        // Create very new seeds (< 1 hour old)
        let now = Utc::now();
        let seed = create_test_entropy_seed(
            create_human_lived_experience_class(0.9),
            now - Duration::minutes(30),
            None,
        );

        active_seeds.insert(seed.seed_id, seed);

        let distribution = monitor.calculate_age_distribution(&active_seeds);
        assert_eq!(distribution.get("very_new"), Some(&1));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_calculate_age_distribution_new() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let mut active_seeds = HashMap::new();

        // Create new seeds (1-24 hours old)
        let now = Utc::now();
        let seed = create_test_entropy_seed(
            create_human_lived_experience_class(0.9),
            now - Duration::hours(12),
            None,
        );

        active_seeds.insert(seed.seed_id, seed);

        let distribution = monitor.calculate_age_distribution(&active_seeds);
        assert_eq!(distribution.get("new"), Some(&1));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_calculate_age_distribution_recent() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let mut active_seeds = HashMap::new();

        // Create recent seeds (24-168 hours old)
        let now = Utc::now();
        let seed = create_test_entropy_seed(
            create_human_lived_experience_class(0.9),
            now - Duration::hours(72),
            None,
        );

        active_seeds.insert(seed.seed_id, seed);

        let distribution = monitor.calculate_age_distribution(&active_seeds);
        assert_eq!(distribution.get("recent"), Some(&1));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_calculate_age_distribution_old() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let mut active_seeds = HashMap::new();

        // Create old seeds (> 168 hours old)
        let now = Utc::now();
        let seed = create_test_entropy_seed(
            create_human_lived_experience_class(0.9),
            now - Duration::hours(200),
            None,
        );

        active_seeds.insert(seed.seed_id, seed);

        let distribution = monitor.calculate_age_distribution(&active_seeds);
        assert_eq!(distribution.get("old"), Some(&1));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_calculate_age_distribution_mixed() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let mut active_seeds = HashMap::new();

        let now = Utc::now();

        // Very new
        let seed1 = create_test_entropy_seed(
            create_human_lived_experience_class(0.9),
            now - Duration::minutes(30),
            None,
        );
        // New
        let seed2 = create_test_entropy_seed(
            create_human_supervised_machine_class(0.8),
            now - Duration::hours(12),
            None,
        );
        // Recent
        let seed3 = create_test_entropy_seed(
            create_store_bought_machine_class(0.7),
            now - Duration::hours(72),
            None,
        );
        // Old
        let seed4 = create_test_entropy_seed(
            create_human_lived_experience_class(0.9),
            now - Duration::hours(200),
            None,
        );

        active_seeds.insert(seed1.seed_id, seed1);
        active_seeds.insert(seed2.seed_id, seed2);
        active_seeds.insert(seed3.seed_id, seed3);
        active_seeds.insert(seed4.seed_id, seed4);

        let distribution = monitor.calculate_age_distribution(&active_seeds);
        assert_eq!(distribution.get("very_new"), Some(&1));
        assert_eq!(distribution.get("new"), Some(&1));
        assert_eq!(distribution.get("recent"), Some(&1));
        assert_eq!(distribution.get("old"), Some(&1));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_update_metrics() {
        let config = create_test_config();
        let mut monitor = EntropyMonitor::new(&config);

        let new_metrics = PerformanceMetrics {
            active_seeds_count: 10,
            total_entropy_generated: 1000,
            average_quality_score: 0.85,
            seed_creation_time_ms: 5.0,
            entropy_mixing_time_ms: 3.0,
            validation_time_ms: 2.0,
            total_operations: 100,
            successful_operations: 95,
            failed_operations: 5,
        };

        monitor.update_metrics(new_metrics.clone());
        assert_eq!(monitor.metrics.active_seeds_count, 10);
        assert_eq!(monitor.metrics.total_entropy_generated, 1000);
        assert_eq!(monitor.metrics.average_quality_score, 0.85);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_get_performance_metrics() {
        let config = create_test_config();
        let mut monitor = EntropyMonitor::new(&config);

        let new_metrics = PerformanceMetrics {
            active_seeds_count: 5,
            total_entropy_generated: 500,
            average_quality_score: 0.92,
            seed_creation_time_ms: 4.5,
            entropy_mixing_time_ms: 2.5,
            validation_time_ms: 1.5,
            total_operations: 50,
            successful_operations: 48,
            failed_operations: 2,
        };

        monitor.update_metrics(new_metrics.clone());
        let retrieved_metrics = monitor.get_performance_metrics();
        assert_eq!(retrieved_metrics.active_seeds_count, 5);
        assert_eq!(retrieved_metrics.total_entropy_generated, 500);
        assert_eq!(retrieved_metrics.successful_operations, 48);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_quality_distribution_boundary_excellent_good() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let quality_scores = vec![0.9, 0.89];

        let distribution = monitor.calculate_quality_distribution(&quality_scores);
        assert_eq!(distribution.get("excellent"), Some(&1));
        assert_eq!(distribution.get("good"), Some(&1));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_quality_distribution_boundary_good_acceptable() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let quality_scores = vec![0.8, 0.79];

        let distribution = monitor.calculate_quality_distribution(&quality_scores);
        assert_eq!(distribution.get("good"), Some(&1));
        assert_eq!(distribution.get("acceptable"), Some(&1));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_quality_distribution_boundary_acceptable_poor() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let quality_scores = vec![0.7, 0.69];

        let distribution = monitor.calculate_quality_distribution(&quality_scores);
        assert_eq!(distribution.get("acceptable"), Some(&1));
        assert_eq!(distribution.get("poor"), Some(&1));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_age_distribution_boundary_very_new_new() {
        let config = create_test_config();
        let monitor = EntropyMonitor::new(&config);
        let mut active_seeds = HashMap::new();

        let now = Utc::now();

        // Just under 1 hour
        let seed1 = create_test_entropy_seed(
            create_human_lived_experience_class(0.9),
            now - Duration::minutes(59),
            None,
        );
        // Just over 1 hour
        let seed2 = create_test_entropy_seed(
            create_human_supervised_machine_class(0.8),
            now - Duration::hours(1) - Duration::minutes(1),
            None,
        );

        active_seeds.insert(seed1.seed_id, seed1);
        active_seeds.insert(seed2.seed_id, seed2);

        let distribution = monitor.calculate_age_distribution(&active_seeds);
        assert_eq!(distribution.get("very_new"), Some(&1));
        assert_eq!(distribution.get("new"), Some(&1));
    }
}
