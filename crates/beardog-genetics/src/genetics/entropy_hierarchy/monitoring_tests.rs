// SPDX-License-Identifier: AGPL-3.0-only

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

#[test]
fn test_entropy_monitor_creation() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    assert_eq!(monitor.metrics.active_seeds_count, 0);
}

#[test]
fn test_entropy_monitor_creation_with_default_config() {
    let config = EntropyHierarchyConfig::default();
    let monitor = EntropyMonitor::new(&config);
    assert_eq!(monitor._config.cleanup_interval_seconds, 3600);
    assert_eq!(monitor._config.max_seed_age_days, 30);
}

#[test]
fn test_cleanup_expired_seeds_empty() {
    let config = create_test_config();
    let mut monitor = EntropyMonitor::new(&config);
    let mut active_seeds = HashMap::new();

    let initial_count = active_seeds.len();
    monitor.cleanup_expired_seeds(&mut active_seeds);
    assert_eq!(active_seeds.len(), initial_count);
}

#[test]
fn test_cleanup_expired_seeds_none_expired() {
    let config = create_test_config();
    let mut monitor = EntropyMonitor::new(&config);
    let mut active_seeds = HashMap::new();

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

#[test]
fn test_cleanup_expired_seeds_removes_expired() {
    let config = create_test_config();
    let mut monitor = EntropyMonitor::new(&config);
    let mut active_seeds = HashMap::new();

    let past_expiry = Utc::now() - Duration::hours(1);
    let expired_seed = create_test_entropy_seed(
        create_human_lived_experience_class(0.9),
        Utc::now() - Duration::hours(2),
        Some(past_expiry),
    );

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

#[test]
fn test_cleanup_expired_seeds_no_expiry() {
    let config = create_test_config();
    let mut monitor = EntropyMonitor::new(&config);
    let mut active_seeds = HashMap::new();

    let seed = create_test_entropy_seed(create_human_lived_experience_class(0.9), Utc::now(), None);

    active_seeds.insert(seed.seed_id, seed);
    monitor.cleanup_expired_seeds(&mut active_seeds);
    assert_eq!(active_seeds.len(), 1);
}

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

#[test]
fn test_get_statistics_mixed_seeds() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let mut active_seeds = HashMap::new();

    let seed1 =
        create_test_entropy_seed(create_human_lived_experience_class(0.95), Utc::now(), None);
    let seed2 =
        create_test_entropy_seed(create_human_lived_experience_class(0.92), Utc::now(), None);
    let seed3 = create_test_entropy_seed(
        create_human_supervised_machine_class(0.85),
        Utc::now(),
        None,
    );
    let seed4 = create_test_entropy_seed(create_store_bought_machine_class(0.65), Utc::now(), None);
    let seed5 = create_test_entropy_seed(create_store_bought_machine_class(0.60), Utc::now(), None);

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

#[test]
fn test_get_analytics_with_seeds() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let mut active_seeds = HashMap::new();

    let seed1 =
        create_test_entropy_seed(create_human_lived_experience_class(0.95), Utc::now(), None);
    let seed2 = create_test_entropy_seed(
        create_human_supervised_machine_class(0.85),
        Utc::now(),
        None,
    );
    let seed3 = create_test_entropy_seed(create_store_bought_machine_class(0.75), Utc::now(), None);

    active_seeds.insert(seed1.seed_id, seed1);
    active_seeds.insert(seed2.seed_id, seed2);
    active_seeds.insert(seed3.seed_id, seed3);

    let analytics = monitor.get_analytics(&active_seeds);
    assert_eq!(analytics.stats.total_seeds, 3);
    assert!((analytics.average_quality - 0.85).abs() < 0.01);
    assert!(!analytics.quality_distribution.is_empty());
    assert!(!analytics.age_distribution.is_empty());
}

#[test]
fn test_get_health_status() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);

    let health = monitor.get_health_status();
    assert_eq!(health.overall_health, "Healthy");
    assert_eq!(health.quality_score, 0.0);
    assert_eq!(
        health.performance_metrics.active_seeds_count,
        monitor.metrics.active_seeds_count
    );
}

#[test]
fn test_calculate_quality_distribution_excellent() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let quality_scores = vec![0.95, 0.92, 0.91];

    let distribution = monitor.calculate_quality_distribution(&quality_scores);
    assert_eq!(distribution.get("excellent"), Some(&3));
}

#[test]
fn test_calculate_quality_distribution_good() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let quality_scores = vec![0.85, 0.82, 0.88];

    let distribution = monitor.calculate_quality_distribution(&quality_scores);
    assert_eq!(distribution.get("good"), Some(&3));
}

#[test]
fn test_calculate_quality_distribution_acceptable() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let quality_scores = vec![0.75, 0.72, 0.78];

    let distribution = monitor.calculate_quality_distribution(&quality_scores);
    assert_eq!(distribution.get("acceptable"), Some(&3));
}

#[test]
fn test_calculate_quality_distribution_poor() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let quality_scores = vec![0.65, 0.55, 0.45];

    let distribution = monitor.calculate_quality_distribution(&quality_scores);
    assert_eq!(distribution.get("poor"), Some(&3));
}

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

#[test]
fn test_calculate_quality_distribution_empty() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let quality_scores: Vec<f64> = vec![];

    let distribution = monitor.calculate_quality_distribution(&quality_scores);
    assert!(distribution.is_empty());
}

#[test]
fn test_calculate_age_distribution_very_new() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let mut active_seeds = HashMap::new();

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

#[test]
fn test_calculate_age_distribution_new() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let mut active_seeds = HashMap::new();

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

#[test]
fn test_calculate_age_distribution_recent() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let mut active_seeds = HashMap::new();

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

#[test]
fn test_calculate_age_distribution_old() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let mut active_seeds = HashMap::new();

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

#[test]
fn test_calculate_age_distribution_mixed() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let mut active_seeds = HashMap::new();

    let now = Utc::now();

    let seed1 = create_test_entropy_seed(
        create_human_lived_experience_class(0.9),
        now - Duration::minutes(30),
        None,
    );
    let seed2 = create_test_entropy_seed(
        create_human_supervised_machine_class(0.8),
        now - Duration::hours(12),
        None,
    );
    let seed3 = create_test_entropy_seed(
        create_store_bought_machine_class(0.7),
        now - Duration::hours(72),
        None,
    );
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

#[test]
fn test_quality_distribution_boundary_excellent_good() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let quality_scores = vec![0.9, 0.89];

    let distribution = monitor.calculate_quality_distribution(&quality_scores);
    assert_eq!(distribution.get("excellent"), Some(&1));
    assert_eq!(distribution.get("good"), Some(&1));
}

#[test]
fn test_quality_distribution_boundary_good_acceptable() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let quality_scores = vec![0.8, 0.79];

    let distribution = monitor.calculate_quality_distribution(&quality_scores);
    assert_eq!(distribution.get("good"), Some(&1));
    assert_eq!(distribution.get("acceptable"), Some(&1));
}

#[test]
fn test_quality_distribution_boundary_acceptable_poor() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let quality_scores = vec![0.7, 0.69];

    let distribution = monitor.calculate_quality_distribution(&quality_scores);
    assert_eq!(distribution.get("acceptable"), Some(&1));
    assert_eq!(distribution.get("poor"), Some(&1));
}

#[test]
fn test_age_distribution_boundary_very_new_new() {
    let config = create_test_config();
    let monitor = EntropyMonitor::new(&config);
    let mut active_seeds = HashMap::new();

    let now = Utc::now();

    let seed1 = create_test_entropy_seed(
        create_human_lived_experience_class(0.9),
        now - Duration::minutes(59),
        None,
    );
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
