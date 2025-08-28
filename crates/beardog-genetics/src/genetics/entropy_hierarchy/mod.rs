

pub mod engine;
pub mod monitoring;
pub mod seed;
pub mod sources;
pub mod types;
pub mod validation;

pub use engine::{EntropyHierarchyManager, EntropyQualityAssessment};
pub use monitoring::{EntropyAnalytics, EntropyHealthStatus, HealthLevel, PerformanceMetrics};
pub use sources::EntropyMixingEngine;
pub use types::{
    BiometricHash, ContributionProof, DownstreamEffects, EntropyClass, EntropyHierarchyConfig,
    EntropyHierarchyStats, EntropySeed, EventType, FusionAlgorithm, GovernanceModel,
    HumanEntropySource, HumanIdentity, InheritancePolicy, IrreproducibilityProof,
    MachineEntropySource, OwnershipProof, OwnershipTransition, SecretBytes, SeedLifetimePolicy,
    SeedOwnership, SeedUsageEvent, SeedUsagePolicy, SharingPolicy, SharingTerms, SocialContext,
    TransferPermissions, VerificationLevel,
};
pub use validation::EntropyValidator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genetics::human_entropy;
    use beardog_errors::BearDogError;
    use chrono::Utc;
    use std::sync::Arc;

    async fn create_test_manager() -> Result<EntropyHierarchyManager, BearDogError> {
        let config = EntropyHierarchyConfig::default();

        let human_config = human_entropy::create_default_config();
        let human_entropy_collector = Arc::new(
            human_entropy::MultiModalHumanEntropyCollector::new(human_config),
        );
        Ok(EntropyHierarchyManager::new(
            config,
            human_entropy_collector,
        ))
    }
    #[tokio::test]
    async fn test_entropy_hierarchy_creation() -> Result<(), BearDogError> {
        let manager = create_test_manager().await?;

        let stats = manager.get_statistics();
        assert_eq!(stats.total_seeds, 0);
        assert_eq!(stats.human_entropy_seeds, 0);

        let health = manager.get_health_status();
        assert!(matches!(
            health.overall_health,
            HealthLevel::Good | HealthLevel::Warning | HealthLevel::Poor
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_human_entropy_seed_creation() -> Result<(), BearDogError> {
        let mut manager = create_test_manager().await?;
        let owner = HumanIdentity {
            identity_id: "test_user".to_string(),
            public_key: vec![0u8; 32],
            biometric_hash: None,
            verification_level: VerificationLevel::BasicBiometric,
        };
        let entropy_class = EntropyClass::HumanLivedExperience {
            source_type: HumanEntropySource::Microphone {
                duration_ms: 8000,
                sample_rate: 44100,
                spectral_features: (0..100).map(|i| i as f32 * 0.01).collect(), // 100 features for quality > 0.7
            },
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash(vec![0u8; 32]),
            ownership_proof: OwnershipProof {
                signature: vec![0u8; 64],
                timestamp: Utc::now(),
                verification_key: vec![0u8; 32],
            },
        };
        let lifetime_policy = SeedLifetimePolicy::Persistent {
            ownership_transfer_allowed: true,
            ownership_expiration: None,
        };
        let seed_id = manager
            .create_human_seed(entropy_class, lifetime_policy, owner, vec![1, 2, 3, 4, 5])
            .await?;

        let seed = manager.get_seed(&seed_id).ok_or_else(|| {
            tracing::error!("Operation failed: seed not found");
            beardog_errors::BearDogError::system("Operation failed: seed not found")
        })?;
        
        assert!(matches!(
            seed.entropy_class,
            EntropyClass::HumanLivedExperience { .. }
        ));
        assert_eq!(manager.get_statistics().total_seeds, 1);
        assert_eq!(manager.get_statistics().human_entropy_seeds, 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_event_seed_creation() -> Result<(), BearDogError> {
        let owner = HumanIdentity {
            identity_id: "concert_attendee".to_string(),
            public_key: vec![0u8; 32],
            biometric_hash: None,
            verification_level: VerificationLevel::BasicBiometric,
        };
        let event_context = SocialContext {
            event_type: EventType::Concert {
                artist: "Artist Name".to_string(),
                venue: "Concert Hall".to_string(),
            },
            location: Some("City, State".to_string()),
            participants: vec![owner.clone()],
            tags: vec!["music".to_string(), "live".to_string()],
            event_timestamp: Utc::now(),
            cultural_significance: Some("First post-pandemic concert".to_string()),
        };
        let sharing_policy = SharingPolicy {
            max_shares: Some(100),
            sharing_expiration: Some(Utc::now() + chrono::Duration::days(30)),
            require_permission: false,
            allowed_operations: vec!["derive_key".to_string()],
        };
        let mut manager = create_test_manager().await?;
        let seed_id = manager
            .create_event_seed(event_context, sharing_policy, owner, vec![1, 2, 3, 4, 5])
            .await?;
        let seed = manager.get_seed(&seed_id).unwrap();
        assert!(seed.social_context.is_some());
        assert_eq!(manager.get_statistics().event_seeds, 1);
        Ok(())
    }

    #[test]
    fn test_entropy_hierarchy_precedence() -> Result<(), BearDogError> {
        let config = EntropyMixingConfig::default();
        let mixer = EntropyMixingEngine::new(config);
        let human_entropy = EntropyClass::HumanLivedExperience {
            source_type: HumanEntropySource::Microphone {
                duration_ms: 1000,
                sample_rate: 44100,
                spectral_features: vec![0.5],
            },
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash(vec![0u8; 32]),
            ownership_proof: OwnershipProof {
                signature: vec![0u8; 64],
                timestamp: Utc::now(),
                verification_key: vec![0u8; 32],
            },
        };
        let machine_entropy = EntropyClass::StoreBoughtMachine {
            source_type: MachineEntropySource::Csprng {
                algorithm: "ChaCha20".to_string(),
                seed_source: "OS".to_string(),
                state_size: 256,
            },
            generation_timestamp: Utc::now(),
            reproducibility_index: 0.8,
        };
        let mixed = mixer
            .mix_entropy_sources(vec![machine_entropy, human_entropy.clone()])
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::system(&format!("Operation failed: {:?}", e))
            })?;

        assert!(matches!(mixed, EntropyClass::HumanLivedExperience { .. }));
        Ok(())
    }

    #[tokio::test]
    async fn test_seed_usage_and_lifecycle() -> Result<(), BearDogError> {
        let owner = HumanIdentity {
            identity_id: "lifecycle_test".to_string(),
            public_key: vec![0u8; 32],
            biometric_hash: None,
            verification_level: VerificationLevel::CryptographicProof,
        };
        let entropy_class = EntropyClass::HumanLivedExperience {
            source_type: HumanEntropySource::Biometric {
                entropy_hash: vec![1, 2, 3],
                biometric_type: "fingerprint".to_string(),
                quality_score: 0.95,
            },
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash(vec![0u8; 32]),
            ownership_proof: OwnershipProof {
                signature: vec![0u8; 64],
                timestamp: Utc::now(),
                verification_key: vec![0u8; 32],
            },
        };
        let lifetime_policy = SeedLifetimePolicy::SelfSovereign {
            user_controlled_lifetime: true,
            transfer_permissions: TransferPermissions {
                transferable: true,
                max_transfers: Some(3),
                transfer_requires_approval: false,
                transfer_audit_trail: true,
            },
            downstream_effects: DownstreamEffects {
                inheritance_policy: InheritancePolicy::PreserveHumanClassification,
                classification_preservation: true,
                lineage_tracking: true,
            },
        };
        let mut manager = create_test_manager().await?;
        let seed_id = manager
            .create_human_seed(
                entropy_class,
                lifetime_policy,
                owner.clone(),
                vec![5, 4, 3, 2, 1],
            )
            .await?;

        let result = manager.use_seed(&seed_id, "key_derivation")?;
        assert!(!result.is_empty());

        let seed = manager.get_seed(&seed_id).unwrap();
        assert_eq!(seed.usage_history.len(), 1);
        assert_eq!(seed.get_entropy_tier(), 3); // Human-lived experience

        if let Some(current_owner) = seed.get_current_owner() {
            assert_eq!(current_owner.identity_id, "lifecycle_test");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_entropy_quality_assessment() -> Result<(), BearDogError> {
        let entropy_class = EntropyClass::HumanLivedExperience {
            source_type: HumanEntropySource::Microphone {
                duration_ms: 8000, // Increased duration for better quality
                sample_rate: 44100,
                spectral_features: (0..100).map(|i| i as f32 * 0.01).collect(),
            },
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash(vec![0u8; 32]),
            ownership_proof: OwnershipProof {
                signature: vec![0u8; 64],
                timestamp: Utc::now(),
                verification_key: vec![0u8; 32],
            },
        };
        let manager = create_test_manager().await?;
        let assessment = manager.assess_entropy_quality(&entropy_class)?;
        assert_eq!(assessment.entropy_tier, 3);
        assert!(assessment.quality_score > 0.7);
        assert!(!assessment.recommendations.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_health_monitoring() -> Result<(), BearDogError> {
        let owner = HumanIdentity {
            identity_id: "health_test_user".to_string(),
            public_key: vec![0u8; 32],
            biometric_hash: None,
            verification_level: VerificationLevel::MultiFactorBiometric,
        };
        let entropy_class = EntropyClass::HumanLivedExperience {
            source_type: HumanEntropySource::Haptic {
                duration_ms: 3000,
                touch_points: vec![(0.1, 0.2), (0.3, 0.4), (0.5, 0.6)],
                motion_patterns: vec![0.1, 0.2, 0.3, 0.4, 0.5],
            },
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash(vec![0u8; 32]),
            ownership_proof: OwnershipProof {
                signature: vec![0u8; 64],
                timestamp: Utc::now(),
                verification_key: vec![0u8; 32],
            },
        };
        let mut manager = create_test_manager().await?;
        let _seed1 = manager
            .create_human_seed(
                entropy_class,
                SeedLifetimePolicy::Persistent {
                    ownership_transfer_allowed: false,
                    ownership_expiration: None,
                },
                owner.clone(),
                vec![1, 2, 3],
            )
            .await?;

        let social_context = SocialContext {
            event_type: EventType::Community {
                group_name: "BearDog Test Community".to_string(),
                purpose: "Testing entropy diversity".to_string(),
            },
            location: Some("Test Environment".to_string()),
            participants: vec![owner.clone()],
            tags: vec!["test".to_string(), "entropy".to_string()],
            event_timestamp: Utc::now(),
            cultural_significance: Some("Testing entropy system".to_string()),
        };
        let sharing_policy = SharingPolicy {
            max_shares: Some(5),
            sharing_expiration: None,
            require_permission: true,
            allowed_operations: vec!["read".to_string(), "derive".to_string()],
        };
        let _seed2 = manager
            .create_event_seed(social_context, sharing_policy, owner.clone(), vec![4, 5, 6])
            .await?;

        let health = manager.get_health_status().await?;
        assert!(!matches!(health.overall_health, HealthLevel::Poor));

        let metrics = manager.get_performance_metrics();
        assert!(metrics.total_seeds >= 2);
        Ok(())
    }
}
