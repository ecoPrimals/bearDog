// SPDX-License-Identifier: AGPL-3.0-or-later

//! Entropy hierarchy: classify sources, mix seeds, monitor health, and validate policy.

/// Runtime manager coordinating validation, mixing, and seed lifecycle.
pub mod engine;
/// Validates that entropy feeds are live human input rather than scripted or replayed data.
pub mod live_feed_validator;
/// Metrics, analytics, and cleanup for active entropy seeds.
pub mod monitoring;
/// Fusion and mixing of multiple entropy buffers into classified seeds.
pub mod sources;
/// Core types: [`EntropyClass`], seeds, identities, and configuration.
pub mod types;
/// Quality, age, biometric, and ownership checks on entropy material.
pub mod validation;

// Re-export all types
pub use engine::EntropyHierarchyManager;
pub use live_feed_validator::{LiveFeedConfig, LiveFeedValidationResult, LiveFeedValidator};
pub use monitoring::{
    EntropyAnalytics, EntropyHealthStatus, EntropyHierarchyStats, EntropyMonitor,
    EntropyMonitoringConfig, PerformanceMetrics,
};
pub use sources::{EntropyMixingEngine, EntropySourceManager};
pub use types::{
    BiometricHash, EntropyClass, EntropyHierarchyConfig, EntropySeed, FusionAlgorithm,
    HumanEntropySource, HumanEntropyType, HumanIdentity, MachineEntropySource, MachineSourceType,
    MixingStrategy, OwnershipProof, SeedMetadata, VerificationLevel,
};
pub use validation::EntropyValidator;

// Note: BearDogError, Deserialize, and Serialize imports removed as they are unused in this module
// They can be re-added when needed for future implementations

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    // ========================================================================
    // Manager Creation Tests
    // ========================================================================

    #[test]
    fn test_entropy_hierarchy_creation() {
        let config = EntropyHierarchyConfig::default();
        let _manager = EntropyHierarchyManager::new(config);
    }

    #[test]
    fn test_entropy_hierarchy_with_custom_config() {
        let config = EntropyHierarchyConfig {
            min_human_quality: 0.9,
            min_machine_quality: 0.7,
            max_entropy_age_hours: 48,
            require_biometric_verification: false,
            require_ownership_proof: false,
        };
        let manager = EntropyHierarchyManager::new(config);

        assert_eq!(manager.config.min_human_quality, 0.9);
        assert_eq!(manager.config.min_machine_quality, 0.7);
        assert_eq!(manager.config.max_entropy_age_hours, 48);
    }

    #[test]
    fn test_entropy_hierarchy_has_empty_seeds_initially() {
        let config = EntropyHierarchyConfig::default();
        let manager = EntropyHierarchyManager::new(config);

        assert_eq!(manager.active_seeds.len(), 0);
    }

    // ========================================================================
    // EntropyClass Tests
    // ========================================================================

    #[test]
    fn test_entropy_class_human_lived_experience() {
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.95,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash::new(vec![1, 2, 3], vec![4, 5, 6]),
            ownership_proof: OwnershipProof::new(vec![7, 8, 9], vec![10, 11, 12]),
        };

        let EntropyClass::HumanLivedExperience { quality_score, .. } = entropy else {
            panic!("Expected HumanLivedExperience variant");
        };
        assert_eq!(quality_score, 0.95);
    }

    #[test]
    fn test_entropy_class_human_supervised_machine() {
        let human_id = HumanIdentity {
            identity_id: "test-human-123".to_string(),
            identity_hash: vec![1, 2, 3, 4],
            verification_level: VerificationLevel::Enhanced,
            verified_at: Utc::now(),
        };

        let machine_source = MachineEntropySource {
            source_type: MachineSourceType::CSPRNG {
                algorithm: "ChaCha20".to_string(),
                seed_source: "hardware".to_string(),
            },
            algorithm: "ChaCha20".to_string(),
            seed_source: "hardware".to_string(),
            quality_metrics: Default::default(),
        };

        let entropy = EntropyClass::HumanSupervisedMachine {
            quality_score: 0.85,
            machine_source,
            human_validator: human_id,
            validation_timestamp: Utc::now(),
        };

        let EntropyClass::HumanSupervisedMachine { quality_score, .. } = entropy else {
            panic!("Expected HumanSupervisedMachine variant");
        };
        assert_eq!(quality_score, 0.85);
    }

    #[test]
    fn test_entropy_class_store_bought_machine() {
        let machine_source = MachineEntropySource {
            source_type: MachineSourceType::CSPRNG {
                algorithm: "MT19937".to_string(),
                seed_source: "time".to_string(),
            },
            algorithm: "MT19937".to_string(),
            seed_source: "time".to_string(),
            quality_metrics: Default::default(),
        };

        let entropy = EntropyClass::StoreBoughtMachine {
            quality_score: 0.65,
            source_type: machine_source,
            generation_timestamp: Utc::now(),
            reproducibility_index: 0.3,
        };

        let EntropyClass::StoreBoughtMachine {
            quality_score,
            reproducibility_index,
            ..
        } = entropy
        else {
            panic!("Expected StoreBoughtMachine variant");
        };
        assert_eq!(quality_score, 0.65);
        assert_eq!(reproducibility_index, 0.3);
    }

    // ========================================================================
    // EntropyClass Ordering Tests
    // ========================================================================

    #[test]
    fn test_entropy_class_ordering_human_highest() {
        let human = EntropyClass::HumanLivedExperience {
            quality_score: 0.95,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash::new(vec![1], vec![2]),
            ownership_proof: OwnershipProof::new(vec![3], vec![4]),
        };

        let machine_source = MachineEntropySource {
            source_type: MachineSourceType::CSPRNG {
                algorithm: "test".to_string(),
                seed_source: "test".to_string(),
            },
            algorithm: "test".to_string(),
            seed_source: "test".to_string(),
            quality_metrics: Default::default(),
        };

        let supervised = EntropyClass::HumanSupervisedMachine {
            quality_score: 0.85,
            machine_source,
            human_validator: HumanIdentity {
                identity_id: "test".to_string(),
                identity_hash: vec![1],
                verification_level: VerificationLevel::Basic,
                verified_at: Utc::now(),
            },
            validation_timestamp: Utc::now(),
        };

        assert!(human > supervised);
    }

    #[test]
    fn test_entropy_class_ordering_supervised_higher_than_store_bought() {
        let machine_source = MachineEntropySource {
            source_type: MachineSourceType::CSPRNG {
                algorithm: "test".to_string(),
                seed_source: "test".to_string(),
            },
            algorithm: "test".to_string(),
            seed_source: "test".to_string(),
            quality_metrics: Default::default(),
        };

        let supervised = EntropyClass::HumanSupervisedMachine {
            quality_score: 0.85,
            machine_source: machine_source.clone(),
            human_validator: HumanIdentity {
                identity_id: "test".to_string(),
                identity_hash: vec![1],
                verification_level: VerificationLevel::Basic,
                verified_at: Utc::now(),
            },
            validation_timestamp: Utc::now(),
        };

        let store_bought = EntropyClass::StoreBoughtMachine {
            quality_score: 0.65,
            source_type: machine_source,
            generation_timestamp: Utc::now(),
            reproducibility_index: 0.3,
        };

        assert!(supervised > store_bought);
    }

    // ========================================================================
    // BiometricHash Tests
    // ========================================================================

    #[test]
    fn test_biometric_hash_creation() {
        let hash = vec![1, 2, 3, 4, 5];
        let proof = vec![6, 7, 8, 9, 10];

        let biometric = BiometricHash::new(hash.clone(), proof.clone());

        assert_eq!(biometric.hash, hash);
        assert_eq!(biometric.ownership_proof, proof);
    }

    #[test]
    fn test_biometric_hash_equality() {
        let bio1 = BiometricHash::new(vec![1, 2, 3], vec![4, 5, 6]);
        let bio2 = BiometricHash::new(vec![1, 2, 3], vec![4, 5, 6]);

        assert_eq!(bio1, bio2);
    }

    #[test]
    fn test_biometric_hash_inequality() {
        let bio1 = BiometricHash::new(vec![1, 2, 3], vec![4, 5, 6]);
        let bio2 = BiometricHash::new(vec![1, 2, 3], vec![4, 5, 7]); // Different proof

        assert_ne!(bio1, bio2);
    }

    // ========================================================================
    // OwnershipProof Tests
    // ========================================================================

    #[test]
    fn test_ownership_proof_creation() {
        let proof_data = vec![1, 2, 3];
        let signature = vec![4, 5, 6];

        let proof = OwnershipProof::new(proof_data.clone(), signature.clone());

        assert_eq!(proof.proof_data, proof_data);
        assert_eq!(proof.signature, signature);
    }

    #[test]
    fn test_ownership_proof_has_timestamp() {
        let proof = OwnershipProof::new(vec![1], vec![2]);
        let now = Utc::now();

        // Timestamp should be very recent
        let diff = (now - proof.timestamp).num_seconds().abs();
        assert!(diff < 2, "Timestamp should be within 2 seconds of now");
    }

    // ========================================================================
    // HumanIdentity Tests
    // ========================================================================

    #[test]
    fn test_human_identity_verification_levels() {
        let basic = HumanIdentity {
            identity_id: "user1".to_string(),
            identity_hash: vec![1, 2, 3],
            verification_level: VerificationLevel::Basic,
            verified_at: Utc::now(),
        };

        assert_eq!(basic.verification_level, VerificationLevel::Basic);

        let enhanced = HumanIdentity {
            identity_id: "user2".to_string(),
            identity_hash: vec![4, 5, 6],
            verification_level: VerificationLevel::Enhanced,
            verified_at: Utc::now(),
        };

        assert_eq!(enhanced.verification_level, VerificationLevel::Enhanced);

        let maximum = HumanIdentity {
            identity_id: "user3".to_string(),
            identity_hash: vec![7, 8, 9],
            verification_level: VerificationLevel::Maximum,
            verified_at: Utc::now(),
        };

        assert_eq!(maximum.verification_level, VerificationLevel::Maximum);
    }

    // ========================================================================
    // MachineSourceType Tests
    // ========================================================================

    #[test]
    fn test_machine_source_type_csprng() {
        let source = MachineSourceType::CSPRNG {
            algorithm: "ChaCha20".to_string(),
            seed_source: "hardware".to_string(),
        };

        let MachineSourceType::CSPRNG {
            algorithm,
            seed_source,
        } = source
        else {
            panic!("Expected CSPRNG variant");
        };
        assert_eq!(algorithm, "ChaCha20");
        assert_eq!(seed_source, "hardware");
    }

    #[test]
    fn test_machine_source_type_hrng() {
        let source = MachineSourceType::HRNG {
            device_type: "TPM2.0".to_string(),
            entropy_rate: 0.95,
        };

        let MachineSourceType::HRNG {
            device_type,
            entropy_rate,
        } = source
        else {
            panic!("Expected HRNG variant");
        };
        assert_eq!(device_type, "TPM2.0");
        assert_eq!(entropy_rate, 0.95);
    }

    #[test]
    fn test_machine_source_type_trng() {
        let source = MachineSourceType::TRNG {
            source_type: "quantum".to_string(),
            randomness_tests: vec!["diehard".to_string(), "nist".to_string()],
        };

        let MachineSourceType::TRNG {
            source_type,
            randomness_tests,
        } = source
        else {
            panic!("Expected TRNG variant");
        };
        assert_eq!(source_type, "quantum");
        assert_eq!(randomness_tests.len(), 2);
    }

    // ========================================================================
    // HumanEntropyType Tests
    // ========================================================================

    #[test]
    fn test_human_entropy_type_biometric() {
        let entropy = HumanEntropyType::Biometric {
            biometric_type: "fingerprint".to_string(),
            quality_score: 0.92,
        };

        let HumanEntropyType::Biometric {
            biometric_type,
            quality_score,
        } = entropy
        else {
            panic!("Expected Biometric variant");
        };
        assert_eq!(biometric_type, "fingerprint");
        assert_eq!(quality_score, 0.92);
    }

    #[test]
    fn test_human_entropy_type_behavioral() {
        let entropy = HumanEntropyType::Behavioral {
            pattern_type: "typing_rhythm".to_string(),
            complexity_score: 0.88,
        };

        let HumanEntropyType::Behavioral {
            pattern_type,
            complexity_score,
        } = entropy
        else {
            panic!("Expected Behavioral variant");
        };
        assert_eq!(pattern_type, "typing_rhythm");
        assert_eq!(complexity_score, 0.88);
    }

    #[test]
    fn test_human_entropy_type_creative() {
        let entropy = HumanEntropyType::Creative {
            expression_type: "drawing".to_string(),
            uniqueness_score: 0.94,
        };

        let HumanEntropyType::Creative {
            expression_type,
            uniqueness_score,
        } = entropy
        else {
            panic!("Expected Creative variant");
        };
        assert_eq!(expression_type, "drawing");
        assert_eq!(uniqueness_score, 0.94);
    }

    // ========================================================================
    // MixingStrategy Tests
    // ========================================================================

    #[test]
    fn test_mixing_strategy_xor() {
        let strategy = MixingStrategy::XorMix;

        assert!(matches!(strategy, MixingStrategy::XorMix));
    }

    #[test]
    fn test_mixing_strategy_hash() {
        let strategy = MixingStrategy::HashMix {
            hash_algorithm: "SHA3-512".to_string(),
        };

        let MixingStrategy::HashMix { hash_algorithm } = strategy else {
            panic!("Expected HashMix variant");
        };
        assert_eq!(hash_algorithm, "SHA3-512");
    }

    #[test]
    fn test_mixing_strategy_crypto() {
        let strategy = MixingStrategy::CryptoMix {
            cipher: "AES-256-GCM".to_string(),
        };

        let MixingStrategy::CryptoMix { cipher } = strategy else {
            panic!("Expected CryptoMix variant");
        };
        assert_eq!(cipher, "AES-256-GCM");
    }

    // ========================================================================
    // SeedMetadata Tests
    // ========================================================================

    #[test]
    fn test_seed_metadata_no_expiry() {
        let metadata = SeedMetadata {
            created_at: Utc::now(),
            expires_at: None,
            usage_count: 0,
            max_usage: None,
        };

        assert!(metadata.expires_at.is_none());
        assert!(metadata.max_usage.is_none());
        assert_eq!(metadata.usage_count, 0);
    }

    #[test]
    fn test_seed_metadata_with_limits() {
        let metadata = SeedMetadata {
            created_at: Utc::now(),
            expires_at: Some(Utc::now()),
            usage_count: 5,
            max_usage: Some(100),
        };

        assert!(metadata.expires_at.is_some());
        assert_eq!(metadata.max_usage, Some(100));
        assert_eq!(metadata.usage_count, 5);
    }

    // ========================================================================
    // Config Tests
    // ========================================================================

    #[test]
    fn test_entropy_config_default_values() {
        let config = EntropyHierarchyConfig::default();

        assert_eq!(config.min_human_quality, 0.8);
        assert_eq!(config.min_machine_quality, 0.6);
        assert_eq!(config.max_entropy_age_hours, 24);
        assert!(config.require_biometric_verification);
        assert!(config.require_ownership_proof);
    }

    #[test]
    fn test_entropy_config_custom_quality_thresholds() {
        let config = EntropyHierarchyConfig {
            min_human_quality: 0.95,
            min_machine_quality: 0.75,
            max_entropy_age_hours: 12,
            require_biometric_verification: false,
            require_ownership_proof: true,
        };

        assert_eq!(config.min_human_quality, 0.95);
        assert_eq!(config.min_machine_quality, 0.75);
        assert!(!config.require_biometric_verification);
    }

    // ========================================================================
    // Serialization Tests
    // ========================================================================

    #[test]
    fn test_entropy_class_serialization() {
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.95,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash::new(vec![1, 2, 3], vec![4, 5, 6]),
            ownership_proof: OwnershipProof::new(vec![7, 8, 9], vec![10, 11, 12]),
        };

        let serialized = serde_json::to_string(&entropy);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = EntropyHierarchyConfig::default();

        let serialized = serde_json::to_string(&config);
        assert!(serialized.is_ok());

        if let Ok(json) = serialized {
            let deserialized: Result<EntropyHierarchyConfig, _> = serde_json::from_str(&json);
            assert!(deserialized.is_ok());
        }
    }
}
