// SPDX-License-Identifier: AGPL-3.0-or-later

//! Coverage: entropy hierarchy engine and validation.

use chrono::Utc;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════
// genetics/entropy_hierarchy/engine.rs - Additional coverage
// ═══════════════════════════════════════════════════════════════════

mod engine_tests {
    use super::{HashMap, Utc};
    use crate::genetics::entropy_hierarchy::*;

    pub(super) fn make_human_entropy() -> EntropyClass {
        EntropyClass::HumanLivedExperience {
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
        }
    }

    pub(super) fn make_machine_entropy() -> EntropyClass {
        EntropyClass::StoreBoughtMachine {
            quality_score: 0.8,
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

    pub(super) fn make_supervised_entropy() -> EntropyClass {
        EntropyClass::HumanSupervisedMachine {
            quality_score: 0.85,
            human_validator: HumanIdentity {
                identity_id: "validator-001".to_string(),
                identity_hash: vec![1; 32],
                verification_level: VerificationLevel::Maximum,
                verified_at: Utc::now(),
            },
            machine_source: MachineEntropySource {
                source_type: MachineSourceType::HRNG {
                    device_type: "TPM".to_string(),
                    entropy_rate: 1.0,
                },
                algorithm: "AES-CTR-DRBG".to_string(),
                seed_source: "Hardware".to_string(),
                quality_metrics: HashMap::new(),
            },
            validation_timestamp: Utc::now(),
        }
    }

    #[test]
    fn test_engine_default() {
        let manager = EntropyHierarchyManager::default();
        assert!(manager.active_seeds.is_empty());
    }

    #[test]
    fn test_create_and_validate_seed() {
        let config = EntropyHierarchyConfig::default();
        let mut manager = EntropyHierarchyManager::new(config);
        let entropy_class = make_human_entropy();

        let seed_id = manager
            .create_human_seed(entropy_class, vec![1, 2, 3, 4])
            .unwrap();

        assert!(manager.validate_seed(seed_id).unwrap());
    }

    #[test]
    fn test_use_seed_and_track_usage() {
        let config = EntropyHierarchyConfig::default();
        let mut manager = EntropyHierarchyManager::new(config);
        let entropy_class = make_human_entropy();

        let seed_id = manager
            .create_human_seed(entropy_class, vec![1, 2, 3, 4])
            .unwrap();

        let derived = manager.use_seed(seed_id, "test-operation").unwrap();
        assert!(!derived.is_empty());

        // Usage count should be incremented
        let info = manager.get_seed_info(seed_id).unwrap();
        assert_eq!(info.metadata.usage_count, 1);
    }

    #[test]
    fn test_use_seed_nonexistent() {
        let mut manager = EntropyHierarchyManager::default();
        let fake_id = uuid::Uuid::new_v4();
        assert!(manager.use_seed(fake_id, "test").is_err());
    }

    #[test]
    fn test_validate_seed_nonexistent() {
        let mut manager = EntropyHierarchyManager::default();
        let fake_id = uuid::Uuid::new_v4();
        assert!(manager.validate_seed(fake_id).is_err());
    }

    #[test]
    fn test_get_seed_info_nonexistent() {
        let manager = EntropyHierarchyManager::default();
        assert!(manager.get_seed_info(uuid::Uuid::new_v4()).is_none());
    }

    #[test]
    fn test_list_active_seeds() {
        let mut manager = EntropyHierarchyManager::default();
        assert!(manager.list_active_seeds().is_empty());

        let entropy_class = make_human_entropy();
        let seed_id = manager
            .create_human_seed(entropy_class, vec![1, 2, 3])
            .unwrap();

        let seeds = manager.list_active_seeds();
        assert_eq!(seeds.len(), 1);
        assert!(seeds.contains(&seed_id));
    }

    #[test]
    fn test_cleanup_expired_seeds() {
        let mut manager = EntropyHierarchyManager::default();
        let entropy_class = make_human_entropy();

        let seed_id = manager
            .create_human_seed(entropy_class, vec![1, 2, 3])
            .unwrap();

        let removed = manager.cleanup_expired_seeds().unwrap();
        assert_eq!(removed, 0);
        assert!(manager.get_seed_info(seed_id).is_some());
    }

    #[test]
    fn test_get_performance_metrics() {
        let mut manager = EntropyHierarchyManager::default();
        let entropy_class = make_human_entropy();

        manager
            .create_human_seed(entropy_class, vec![1, 2, 3, 4])
            .unwrap();

        let metrics = manager.get_performance_metrics();
        assert_eq!(metrics.active_seeds_count, 1);
        assert!(metrics.total_entropy_generated > 0);
        assert!(metrics.average_quality_score > 0.0);
    }

    #[test]
    fn test_get_performance_metrics_empty() {
        let manager = EntropyHierarchyManager::default();
        let metrics = manager.get_performance_metrics();
        assert_eq!(metrics.active_seeds_count, 0);
        assert_eq!(metrics.average_quality_score, 0.0);
    }

    #[test]
    fn test_calculate_average_quality_mixed() {
        let mut manager = EntropyHierarchyManager::default();

        manager
            .create_human_seed(make_human_entropy(), vec![1, 2, 3])
            .unwrap();
        manager
            .create_human_seed(make_machine_entropy(), vec![4, 5, 6])
            .unwrap();
        manager
            .create_human_seed(make_supervised_entropy(), vec![7, 8, 9])
            .unwrap();

        let metrics = manager.get_performance_metrics();
        assert!(metrics.average_quality_score > 0.0);
        assert!(metrics.average_quality_score < 1.0);
        assert_eq!(metrics.active_seeds_count, 3);
    }

    #[test]
    fn test_initialize_and_shutdown() {
        let mut manager = EntropyHierarchyManager::default();
        let entropy = make_human_entropy();
        manager.create_human_seed(entropy, vec![1, 2, 3]).unwrap();

        assert!(manager.initialize().is_ok());
        assert_eq!(manager.active_seeds.len(), 1);

        assert!(manager.shutdown().is_ok());
        assert!(manager.active_seeds.is_empty());
    }

    #[test]
    fn test_seed_multiple_uses() {
        let config = EntropyHierarchyConfig::default();
        let mut manager = EntropyHierarchyManager::new(config);
        let entropy = make_human_entropy();

        let seed_id = manager
            .create_human_seed(entropy, vec![1, 2, 3, 4])
            .unwrap();

        // Use the seed multiple times
        for i in 0..5 {
            let derived = manager.use_seed(seed_id, &format!("op-{i}")).unwrap();
            assert!(!derived.is_empty());
        }

        let info = manager.get_seed_info(seed_id).unwrap();
        assert_eq!(info.metadata.usage_count, 5);
    }
}

// ═══════════════════════════════════════════════════════════════════
// genetics/entropy_hierarchy/validation.rs - Additional coverage
// ═══════════════════════════════════════════════════════════════════

mod validation_tests {
    use super::{HashMap, Utc};
    use crate::genetics::entropy_hierarchy::*;

    fn strict_config() -> EntropyHierarchyConfig {
        EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        }
    }

    fn lenient_config() -> EntropyHierarchyConfig {
        EntropyHierarchyConfig {
            min_human_quality: 0.1,
            min_machine_quality: 0.1,
            max_entropy_age_hours: 8760,
            require_biometric_verification: false,
            require_ownership_proof: false,
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn make_valid_human_entropy() -> EntropyClass {
        let mut biometric_hash = vec![0u8; 32];
        for (i, byte) in biometric_hash.iter_mut().enumerate() {
            *byte = (i as u8).wrapping_mul(7).wrapping_add(13);
        }
        let mut sig = vec![0u8; 64];
        for (i, byte) in sig.iter_mut().enumerate() {
            *byte = (i as u8).wrapping_mul(3).wrapping_add(5);
        }

        EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: biometric_hash,
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: sig,
                timestamp: Utc::now(),
            },
        }
    }

    #[test]
    fn test_validate_supervised_machine_quality_pass() {
        let entropy = super::engine_tests::make_supervised_entropy();
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        assert!(v.validate_entropy_quality(&entropy).unwrap());
    }

    #[test]
    fn test_validate_supervised_machine_quality_fail() {
        let entropy = EntropyClass::HumanSupervisedMachine {
            quality_score: 0.1,
            human_validator: HumanIdentity {
                identity_id: "val-001".to_string(),
                identity_hash: vec![1; 32],
                verification_level: VerificationLevel::Maximum,
                verified_at: Utc::now(),
            },
            machine_source: MachineEntropySource {
                source_type: MachineSourceType::HRNG {
                    device_type: "TPM".to_string(),
                    entropy_rate: 1.0,
                },
                algorithm: "AES-CTR-DRBG".to_string(),
                seed_source: "Hardware".to_string(),
                quality_metrics: HashMap::new(),
            },
            validation_timestamp: Utc::now(),
        };
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        assert!(v.validate_entropy_quality(&entropy).is_err());
    }

    #[test]
    fn test_validate_supervised_machine_age() {
        let entropy = EntropyClass::HumanSupervisedMachine {
            quality_score: 0.8,
            human_validator: HumanIdentity {
                identity_id: "val-002".to_string(),
                identity_hash: vec![1; 32],
                verification_level: VerificationLevel::Maximum,
                verified_at: Utc::now(),
            },
            machine_source: MachineEntropySource {
                source_type: MachineSourceType::HRNG {
                    device_type: "TPM".to_string(),
                    entropy_rate: 1.0,
                },
                algorithm: "AES-CTR-DRBG".to_string(),
                seed_source: "Hardware".to_string(),
                quality_metrics: HashMap::new(),
            },
            validation_timestamp: Utc::now(),
        };
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        assert!(v.validate_entropy_age(&entropy).unwrap());
    }

    #[test]
    fn test_validate_biometric_disabled() {
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = make_valid_human_entropy();
        assert!(v.validate_biometric_verification(&entropy).unwrap());
    }

    #[test]
    fn test_validate_biometric_empty_hash() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_biometric_verification(&entropy).is_err());
    }

    #[test]
    fn test_validate_biometric_empty_ownership_proof() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1; 32],
                ownership_proof: vec![],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_biometric_verification(&entropy).is_err());
    }

    #[test]
    fn test_validate_biometric_short_hash() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1; 16],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_biometric_verification(&entropy).is_err());
    }

    #[test]
    fn test_validate_biometric_zeroed_hash() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![0; 32],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_biometric_verification(&entropy).is_err());
    }

    #[test]
    fn test_validate_biometric_supervised_machine() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = super::engine_tests::make_supervised_entropy();
        assert!(v.validate_biometric_verification(&entropy).unwrap());
    }

    #[test]
    fn test_validate_biometric_store_bought() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = super::engine_tests::make_machine_entropy();
        assert!(v.validate_biometric_verification(&entropy).unwrap());
    }

    #[test]
    fn test_validate_ownership_disabled() {
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = make_valid_human_entropy();
        assert!(v.validate_ownership_proof(&entropy).unwrap());
    }

    #[test]
    fn test_validate_ownership_empty_proof_data() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1; 32],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_ownership_proof(&entropy).is_err());
    }

    #[test]
    fn test_validate_ownership_empty_signature() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1; 32],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_ownership_proof(&entropy).is_err());
    }

    #[test]
    fn test_validate_ownership_zeroed_signature() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1; 32],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![0; 64],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_ownership_proof(&entropy).is_err());
    }

    #[test]
    fn test_validate_ownership_old_proof() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let old_timestamp = Utc::now() - chrono::Duration::hours(48);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1; 32],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![1; 64],
                timestamp: old_timestamp,
            },
        };
        assert!(v.validate_ownership_proof(&entropy).is_err());
    }

    #[test]
    fn test_validate_ownership_supervised_machine() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = super::engine_tests::make_supervised_entropy();
        assert!(v.validate_ownership_proof(&entropy).unwrap());
    }

    #[test]
    fn test_validate_ownership_store_bought() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = super::engine_tests::make_machine_entropy();
        assert!(v.validate_ownership_proof(&entropy).unwrap());
    }

    #[test]
    fn test_full_validate_pass() {
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = make_valid_human_entropy();
        assert!(v.validate_entropy(&entropy).is_ok());
    }

    #[test]
    fn test_full_validate_machine_pass() {
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = super::engine_tests::make_machine_entropy();
        assert!(v.validate_entropy(&entropy).is_ok());
    }

    #[test]
    fn test_full_validate_supervised_pass() {
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = super::engine_tests::make_supervised_entropy();
        assert!(v.validate_entropy(&entropy).is_ok());
    }
}
