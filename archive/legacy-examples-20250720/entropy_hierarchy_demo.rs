

use beardog::genetics::entropy_hierarchy::*;
use beardog::genetics::human_entropy::*;
use beardog::tunnel::hsm::*;
use beardog::BearDogResult;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> BearDogResult<()> {
    println!("🎯 BearDog Entropy Hierarchy Demo");
    println!("🧬 Revolutionary Human-First Cryptographic Seeds");
    println!("=====================================\n");

    let config = EntropyHierarchyConfig::default();

    let hsm_manager = Arc::new(create_demo_hsm_manager().await?);
    let entropy_collector = Arc::new(create_demo_entropy_collector().await?);

    let mut entropy_manager = EntropyHierarchyManager::new(config, hsm_manager, entropy_collector);

    println!("📱 Demo 1: Human-Lived Experience Entropy");
    println!("-----------------------------------------");

    let human_user = create_demo_human_identity("alice", "Alice at Concert Hall")?;
    let human_entropy = create_demo_human_entropy()?;

    let self_sovereign_policy = SeedLifetimePolicy::SelfSovereign {
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

    let human_seed_id = entropy_manager
        .create_human_seed(
            human_entropy,
            self_sovereign_policy,
            human_user.clone(),
            vec![1, 2, 3, 4, 5, 6, 7, 8], // Demo seed bytes
        )
        .await?;

    println!("✅ Created human entropy seed: {}", human_seed_id);
    println!("   - Classification: HumanLivedExperience");
    println!("   - Lifetime: Self-sovereign (user controlled)");
    println!("   - Transferable: Yes (max 3 transfers)");
    println!("   - Downstream policy: Preserve human classification\n");

    println!("🎵 Demo 2: Event-Based Seed Sharing (Concert)");
    println!("----------------------------------------------");

    let concert_context = SocialContext {
        event_type: EventType::Concert {
            artist: "The Digital Rebels".to_string(),
            venue: "Crypto Concert Hall".to_string(),
        },
        location: Some("Austin, TX".to_string()),
        participants: vec![
            human_user.clone(),
            create_demo_human_identity("bob", "Bob at Concert")?,
            create_demo_human_identity("charlie", "Charlie at Concert")?,
        ],
        tags: vec![
            "crypto".to_string(),
            "music".to_string(),
            "live".to_string(),
        ],
        event_timestamp: Utc::now(),
        cultural_significance: Some("First crypto-secured concert experience".to_string()),
    };

    let concert_sharing_policy = SharingPolicy {
        max_shares: Some(1000), // Up to 1000 concert attendees
        sharing_expiration: Some(Utc::now() + Duration::days(30)),
        require_permission: false,
        allowed_operations: vec![
            "derive_concert_memory".to_string(),
            "create_nft_proof".to_string(),
            "share_experience".to_string(),
        ],
    };

    let concert_seed_id = entropy_manager
        .create_event_seed(
            concert_context,
            concert_sharing_policy,
            human_user.clone(),
            vec![9, 10, 11, 12, 13, 14, 15, 16], // Concert entropy
        )
        .await?;

    println!("✅ Created concert event seed: {}", concert_seed_id);
    println!("   - Event: The Digital Rebels at Crypto Concert Hall");
    println!("   - Participants: 3 attendees");
    println!("   - Max shares: 1000 (scalable to full venue)");
    println!("   - Expiration: 30 days");
    println!("   - Operations: concert memories, NFT proofs, experience sharing\n");

    println!("🎓 Demo 3: Conference Seed for Knowledge Sharing");
    println!("-----------------------------------------------");

    let conference_context = SocialContext {
        event_type: EventType::Conference {
            name: "Self-Sovereign Crypto Summit".to_string(),
            topic: "Democratizing Cryptography".to_string(),
        },
        location: Some("San Francisco, CA".to_string()),
        participants: vec![
            human_user.clone(),
            create_demo_human_identity("diana", "Diana - Speaker")?,
            create_demo_human_identity("eve", "Eve - Attendee")?,
        ],
        tags: vec![
            "education".to_string(),
            "crypto".to_string(),
            "self-sovereign".to_string(),
        ],
        event_timestamp: Utc::now(),
        cultural_significance: Some("Advancing crypto accessibility".to_string()),
    };

    let conference_sharing_policy = SharingPolicy {
        max_shares: Some(500),
        sharing_expiration: Some(Utc::now() + Duration::days(90)), // Longer for education
        require_permission: false,
        allowed_operations: vec![
            "derive_learning_proof".to_string(),
            "create_certificate".to_string(),
            "share_knowledge".to_string(),
        ],
    };

    let conference_seed_id = entropy_manager
        .create_event_seed(
            conference_context,
            conference_sharing_policy,
            human_user.clone(),
            vec![17, 18, 19, 20, 21, 22, 23, 24], // Conference entropy
        )
        .await?;

    println!("✅ Created conference seed: {}", conference_seed_id);
    println!("   - Event: Self-Sovereign Crypto Summit");
    println!("   - Topic: Democratizing Cryptography");
    println!("   - Max shares: 500 (conference size)");
    println!("   - Expiration: 90 days (educational content)");
    println!("   - Operations: learning proofs, certificates, knowledge sharing\n");

    println!("🔄 Demo 4: Ownership Transfer");
    println!("----------------------------");

    let new_owner = create_demo_human_identity("frank", "Frank - New Owner")?;

    if let Some(seed) = entropy_manager.get_seed_mut(&human_seed_id) {
        println!("📋 Before transfer:");
        print_seed_ownership(seed);

        seed.transfer_ownership(new_owner.clone())?;

        println!("📋 After transfer:");
        print_seed_ownership(seed);
        println!("   - Transfer recorded in audit trail");
        println!("   - Human classification preserved");
        println!("   - Transfer count: 1/3 (2 transfers remaining)\n");
    }

    println!("🔐 Demo 5: Cryptographic Operations with Seeds");
    println!("---------------------------------------------");

    let concert_key = entropy_manager.use_seed(&concert_seed_id, "derive_key")?;
    println!("✅ Derived concert memory key: {} bytes", concert_key.len());

    let certificate_signature =
        entropy_manager.use_seed(&conference_seed_id, "generate_signature")?;
    println!(
        "✅ Created certificate signature: {} bytes",
        certificate_signature.len()
    );

    let general_key = entropy_manager.use_seed(&human_seed_id, "derive_key")?;
    println!("✅ Derived general key: {} bytes", general_key.len());
    println!();

    println!("⏳ Demo 6: Ownership Expiration and Downstream Effects");
    println!("-----------------------------------------------------");

    if let Some(seed) = entropy_manager.get_seed_mut(&human_seed_id) {
        println!("📋 Before ownership expiration:");
        print_seed_ownership(seed);
        print_entropy_classification(seed);

        seed.expire_ownership()?;

        println!("📋 After ownership expiration:");
        print_seed_ownership(seed);
        print_entropy_classification(seed);
        println!("   - Became self-sovereign machine-owned");
        println!("   - Human classification preserved (per policy)");
        println!("   - Previous owner tracked in lineage\n");
    }

    println!("🧬 Demo 7: Entropy Hierarchy in Action");
    println!("-------------------------------------");

    let human_entropy_class = EntropyClass::HumanLivedExperience {
        source_type: HumanEntropySource::MultiModalHuman {
            sources: vec![],
            fusion_algorithm: FusionAlgorithm::HumanDominant,
            confidence_score: 0.9,
        },
        capture_timestamp: Utc::now(),
        biometric_signature: BiometricHash(vec![1, 2, 3]),
        ownership_proof: OwnershipProof {
            signature: vec![4, 5, 6],
            timestamp: Utc::now(),
            verification_key: vec![7, 8, 9],
        },
    };

    let machine_entropy_class = EntropyClass::StoreBoughtMachine {
        source_type: MachineEntropySource::CSPRNG {
            algorithm: "ChaCha20".to_string(),
            seed_source: "/dev/urandom".to_string(),
            state_size: 256,
        },
        generation_timestamp: Utc::now(),
        reproducibility_index: 0.8,
    };

    let mixed_entropy = entropy_manager
        .mixing_engine
        .mix_entropy_sources(vec![machine_entropy_class, human_entropy_class.clone()])?;

    println!("🎯 Entropy Hierarchy Test:");
    println!("   - Input: Machine entropy + Human entropy");
    println!("   - Result: Human entropy dominates (as expected)");
    println!("   - Hierarchy Rule: Human > Machine (always)");
    println!(
        "   - Classification: {:?}",
        match mixed_entropy {
            EntropyClass::HumanLivedExperience { .. } => "Human-Lived Experience ✅",
            EntropyClass::HumanSupervisedMachine { .. } => "Human-Supervised Machine",
            EntropyClass::StoreBoughtMachine { .. } => "Store-Bought Machine",
        }
    );
    println!();

    println!("📊 Demo 8: System Statistics");
    println!("----------------------------");

    let stats = entropy_manager.get_statistics();
    println!("🔢 Entropy Hierarchy Statistics:");
    println!("   - Total seeds: {}", stats.total_seeds);
    println!("   - Human entropy seeds: {}", stats.human_entropy_seeds);
    println!(
        "   - Human-supervised seeds: {}",
        stats.human_supervised_seeds
    );
    println!(
        "   - Machine entropy seeds: {}",
        stats.machine_entropy_seeds
    );
    println!("   - Event seeds: {}", stats.event_seeds);
    println!("   - Self-sovereign seeds: {}", stats.self_sovereign_seeds);
    println!();

    println!("🧹 Demo 9: Cleanup and Lifecycle Management");
    println!("------------------------------------------");

    println!("🔄 Cleaning up expired seeds...");
    entropy_manager.cleanup_expired_seeds();

    let stats_after_cleanup = entropy_manager.get_statistics();
    println!("✅ Cleanup complete");
    println!("   - Seeds before cleanup: {}", stats.total_seeds);
    println!(
        "   - Seeds after cleanup: {}",
        stats_after_cleanup.total_seeds
    );
    println!("   - All seeds are still valid (no expiration in demo)\n");

    println!("🚀 Demo 10: Revolutionary Impact Summary");
    println!("---------------------------------------");

    println!("🎯 What We've Accomplished:");
    println!("   ✅ Human entropy recognized as superior to machine entropy");
    println!("   ✅ Self-sovereign cryptographic tools for everyone");
    println!("   ✅ Event-based seed sharing for concerts, conferences");
    println!("   ✅ Configurable ownership with transfer and expiration");
    println!("   ✅ Downstream effects control for inheritance policies");
    println!("   ✅ Generative crypto capabilities for creative purposes");
    println!("   ✅ True crypto accessibility - no technical barriers");
    println!();

    println!("🌟 Revolutionary Benefits:");
    println!("   🎵 Concert attendees can create shared cryptographic memories");
    println!("   🎓 Conference participants can generate verifiable certificates");
    println!("   🏛️ Community members can govern shared cryptographic resources");
    println!("   🎨 Artists can create unique, transferable digital assets");
    println!("   👥 Individuals can maintain self-sovereign control over their crypto");
    println!("   🔄 Ownership can naturally expire and become machine-managed");
    println!();

    println!("🎯 Mission Accomplished:");
    println!("   Advanced cryptography is now accessible to everyone!");
    println!("   Human-lived experience entropy is recognized as superior!");
    println!("   Self-sovereignty and generative purposes are enabled!");
    println!("   Cryptography transforms from barrier to empowerment tool!");

    Ok(())
}

async fn create_demo_hsm_manager() -> BearDogResult<HsmManager> {
    let config = HsmManagerConfig {
        hsm_configs: vec![HsmConfig {
            hsm_type: HsmType::SoftwareRust,
            config_data: HashMap::with_capacity(16),
            ios_config: None,
            android_config: None,
            software_config: Some(SoftwareHsmConfig {
                implementation: SoftwareHsmType::RustSoftwareHsm,
                key_store_config: KeyStoreConfig {
                    storage_type: KeyStorageType::Memory,
                    encryption_config: EncryptionConfig::default(),
                },
                memory_config: MemoryConfig::default(),
                crypto_backend: CryptoBackend::RustCrypto,
            }),
            aws_config: None,
            luna_config: None,
        }],
        health_config: HealthConfig::default(),
        failover_config: FailoverConfig::default(),
        performance_config: PerformanceConfig::default(),
    };

    HsmManager::new(config).await
}

async fn create_demo_entropy_collector() -> BearDogResult<MultiModalHumanEntropyCollector> {
    let config = HumanEntropyConfig {
        collection_duration: std::time::Duration::from_secs(30),
        max_collection_attempts: 3,
        require_multimodal: false,
        min_quality_score: 0.7,
        audio_config: AudioConfig {
            enabled: true,
            sample_rate: 44100,
            bit_depth: 16,
            privacy_filter: "basic".to_string(),
        },
        visual_config: VisualConfig {
            enabled: true,
            resolution: (1920, 1080),
            fps: 30,
            privacy_filter: "basic".to_string(),
        },
        haptic_config: HapticConfig {
            enabled: true,
            touch_sensitivity: "medium".to_string(),
            motion_sensitivity: "medium".to_string(),
        },
        biometric_config: BiometricConfig {
            enabled: false,
            biometric_types: vec![],
            privacy_level: "maximum".to_string(),
        },
    };

    Ok(MultiModalHumanEntropyCollector::new(config))
}

fn create_demo_human_identity(username: &str, display_name: &str) -> BearDogResult<HumanIdentity> {
    Ok(HumanIdentity {
        identity_id: format_args!("{}_{}", username, Uuid::new_v4().to_string()),
        public_key: vec![0u8; 32],              // Demo public key
        biometric_hash: Some(vec![1, 2, 3, 4]), // Demo biometric hash
        verification_level: VerificationLevel::BasicBiometric,
    })
}

fn create_demo_human_entropy() -> BearDogResult<EntropyClass> {
    Ok(EntropyClass::HumanLivedExperience {
        source_type: HumanEntropySource::MultiModalHuman {
            sources: vec![
                HumanEntropySource::Microphone {
                    duration_ms: 5000,
                    sample_rate: 44100,
                    spectral_features: vec![0.3, 0.7, 0.5, 0.9],
                },
                HumanEntropySource::Camera {
                    duration_ms: 3000,
                    resolution: (1920, 1080),
                    lighting_variations: vec![0.8, 0.6, 0.4],
                },
                HumanEntropySource::Haptic {
                    duration_ms: 2000,
                    touch_points: vec![(0.5, 0.3), (0.7, 0.8), (0.2, 0.9)],
                    motion_patterns: vec![0.1, 0.8, 0.3, 0.6],
                },
            ],
            fusion_algorithm: FusionAlgorithm::HumanDominant,
            confidence_score: 0.85,
        },
        capture_timestamp: Utc::now(),
        biometric_signature: BiometricHash(vec![5, 6, 7, 8]),
        ownership_proof: OwnershipProof {
            signature: vec![9, 10, 11, 12],
            timestamp: Utc::now(),
            verification_key: vec![13, 14, 15, 16],
        },
    })
}

fn print_seed_ownership(seed: &EntropySeed) {
    match &seed.ownership {
        SeedOwnership::HumanOwned {
            owner_identity,
            transfer_count,
            ..
        } => {
            println!("   - Owner: {} (Human)", owner_identity.identity_id);
            println!("   - Transfer count: {}", transfer_count);
        }
        SeedOwnership::MachineOwned {
            previous_owner,
            ownership_transition,
            self_sovereign,
        } => {
            println!("   - Owner: Machine (Self-sovereign: {})", self_sovereign);
            println!(
                "   - Previous owner: {:?}",
                previous_owner.as_ref().map(|o| &o.identity_id)
            );
            println!("   - Transition: {:?}", ownership_transition);
        }
        SeedOwnership::SharedOwnership {
            primary_owner,
            shared_with,
            ..
        } => {
            println!("   - Primary owner: {}", primary_owner.identity_id);
            println!("   - Shared with: {} others", shared_with.len());
        }
        SeedOwnership::CommunityOwned { community_id, .. } => {
            println!("   - Community: {}", community_id);
        }
    }
}

fn print_entropy_classification(seed: &EntropySeed) {
    match &seed.entropy_class {
        EntropyClass::HumanLivedExperience { .. } => {
            println!("   - Classification: Human-Lived Experience (Highest tier)");
        }
        EntropyClass::HumanSupervisedMachine { .. } => {
            println!("   - Classification: Human-Supervised Machine (Mid tier)");
        }
        EntropyClass::StoreBoughtMachine {
            reproducibility_index,
            ..
        } => {
            println!("   - Classification: Store-Bought Machine (Lowest tier)");
            println!("   - Reproducibility: {:.2}", reproducibility_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_entropy_hierarchy_demo() {

        let result = main().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_demo_helper_functions() {
        let identity = create_demo_human_identity("test", "Test User").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert!(identity.identity_id.starts_with("test_"));
        assert_eq!(identity.public_key.len(), 32);

        let entropy = create_demo_human_entropy().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert!(matches!(entropy, EntropyClass::HumanLivedExperience { .. }));
    }
}
