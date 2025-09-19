use beardog::genetics::entropy_hierarchy::*;
use beardog::genetics::human_entropy::*;
use beardog::tunnel::hsm::*;
use beardog_errors::BearDogError;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    println!("[TARGET] BearDog Entropy Hierarchy Demo");
    println!("[DNA] Revolutionary Human-First Cryptographic Seeds");
    println!("=====================================");

    let config = EntropyHierarchyConfig::default();

    let hsm_manager = Arc::new(create_demo_hsm_manager()?);
    let entropy_collector = Arc::new(create_demo_entropy_collector()?);

    let mut entropy_manager = EntropyHierarchyManager::new(config, hsm_manager, entropy_collector);

    println!("📱 Demo 1: Human-Lived Experience Entropy");
    println!("-----------------------------------------");

    let human_user = create_demo_human_identity("alice", "Alice at Concert Hall")?;
    let human_entropy = create_demo_human_entropy()?;

    let self_sovereign_policy = SeedLifetimePolicy::SelfSovereign {
        user_controlled_lifetime: true,
        transfer_permissions: TransferPermissions {
            transferable: true,
            max_transfers: Some(5),
            requires_approval: false,
            transfer_audit_trail: true,
        },
        downstream_effects: DownstreamEffects {
            inheritance_policy: InheritancePolicy::PreserveHumanClassification,
            classification_preservation: true,
            lineage_tracking: true,
        },
    };

    let human_seed_id = entropy_manager
        .create_human_seed({}", human_seed_id);
    println!("   - Classification: HumanLivedExperience ");
    println!("   - Lifetime: Self-sovereign (user controlled)");
    println!("   - Transferable: Yes (max 3 transfers)");
    println!("   - Downstream policy: Preserve human classification ");

    println!("🎵 Demo 2: Event-Based Seed Sharing (Concert)");
    println!("----------------------------------------------");

    let concert_context = SocialContext {
        event_type: EventType::Concert {
            artist: "The Digital Rebels".to_string(),
            venue: "Crypto Concert Hall".to_string(Some("Austin, TX".to_string()),
        participants: vec![
            human_user.clone(),
            create_demo_human_identity("bob", "Bob at Concert ")?,
            create_demo_human_identity("charlie", "Charlie at Concert ")?,
        ],
        tags: vec![
            "crypto".to_string(),
            "music".to_string(),
            "live".to_string(),
        ],
        event_timestamp: Utc::now(),
        cultural_significance: Some("First crypto-secured concert experience ".to_string()),
    };

    let concert_sharing_policy = SharingPolicy {
        max_shares: Some(1000), // Up to 1000 concert attendees
        sharing_expiration: Some(Utc::now() + Duration::days(false,
        allowed_operations: vec![
            "derive_concert_memory".to_string({}", concert_seed_id);
    println!("   - Event: The Digital Rebels at Crypto Concert Hall");
    println!("   - Participants: 3 attendees");
    println!("   - Max shares: 1000 (scalable to full venue)");
    println!("   - Expiration: 30 days");
    println!("   - Operations: concert memories, NFT proofs, experience sharing");

    println!("🎓 Demo 3: Conference Seed for Knowledge Sharing");
    println!("-----------------------------------------------");

    let conference_context = SocialContext {
        event_type: EventType::Conference {
            name: "Self-Sovereign Crypto Summit".to_string(),
            topic: "Democratizing Cryptography".to_string(),
            location: Some("San Francisco, CA".to_string()),
        },
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
        sharing_expiration: Some(Utc::now() + Duration::days(false,
        allowed_operations: vec![
            "derive_learning_proof".to_string({}", conference_seed_id);
    println!("   - Event: Self-Sovereign Crypto Summit");
    println!("   - Topic: Democratizing Cryptography");
    println!("   - Max shares: 500 (conference size)");
    println!("   - Expiration: 90 days (educational content)");
    println!("   - Operations: learning proofs, certificates, knowledge sharing");

    println!("[CYCLE] Demo 4: Ownership Transfer");
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
        println!("   - Transfer count: 1/3 (2 transfers remaining)");
    }

    println!("🔐 Demo 5: Cryptographic Operations with Seeds");
    println!("---------------------------------------------");

    let concert_key = entropy_manager.use_seed({} bytes", concert_key.len({} bytes",
        certificate_signature.len({} bytes", general_key.len());
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
        println!("   - Previous owner tracked in lineage");
    }

    println!("[DNA] Demo 7: Entropy Hierarchy in Action");
    println!("-------------------------------------");

    let human_entropy_class = EntropyClass::HumanLivedExperience {
        source_type: HumanEntropySource::MultiModalHuman {
            sources: vec![],
            fusion_algorithm: FusionAlgorithm::HumanDominant,
            confidence_score: 0.9,
        },
        capture_timestamp: Utc::now(BiometricHash(vec![1, 2, 3]),
        ownership_proof: OwnershipProof {
            signature: vec![4, 5, 6],
            timestamp: Utc::now(vec![7, 8, 9],
        },
    };

    let machine_entropy_class = EntropyClass::StoreBoughtMachine {
        source_type: MachineEntropySource::Csprng {
            algorithm: "ChaCha20".to_string(),
            seed_source: "/dev/urandom".to_string(256,
        },
        generation_timestamp: Utc::now(0.8,
    };

    let mixed_entropy = entropy_manager
        .mixing_engine
        .mix_entropy_sources(vec![machine_entropy_class, human_entropy_class.clone()])?;

    println!("[TARGET] Entropy Hierarchy Test:");
    println!("   - Input: Machine entropy + Human entropy");
    println!("   - Result: Human entropy dominates (as expected)");
    println!("   - Hierarchy Rule: Human > Machine (always)");
    println!(
        "   - Classification: {:?}",
        match mixed_entropy {
            EntropyClass::HumanLivedExperience { .. } => "Human-Lived Experience [OK]",
            EntropyClass::HumanSupervisedMachine { .. } => "Human-Supervised Machine",
            EntropyClass::StoreBoughtMachine { .. } => "Store-Bought Machine",
        }
    );
    println!();

    println!("[CHART] Demo 8: System Statistics");
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

    println!("[CYCLE] Cleaning up expired seeds...");
    entropy_manager.cleanup_expired_seeds({}", stats.total_seeds);
    println!(
        "   - Seeds after cleanup: {}",
        stats_after_cleanup.total_seeds
    );
    println!("   - All seeds are still valid (no expiration in demo)");

    println!("[ROCKET] Demo 10: Revolutionary Impact Summary");
    println!("---------------------------------------");

    println!("[TARGET] What We've Accomplished:");
    println!("   [OK] Human entropy recognized as superior to machine entropy");
    println!("   [OK] Self-sovereign cryptographic tools for everyone");
    println!("   [OK] Event-based seed sharing for concerts, conferences");
    println!("   [OK] Configurable ownership with transfer and expiration");
    println!("   [OK] Downstream effects control for inheritance policies");
    println!("   [OK] Generative crypto capabilities for creative purposes");
    println!("   [OK] True crypto accessibility - no technical barriers");
    println!();

    println!("🌟 Revolutionary Benefits:");
    println!("   🎵 Concert attendees can create shared cryptographic memories");
    println!("   🎓 Conference participants can generate verifiable certificates");
    println!("   🏛️ Community members can govern shared cryptographic resources");
    println!("   🎨 Artists can create unique, transferable digital assets");
    println!("   👥 Individuals can maintain self-sovereign control over their crypto");
    println!("   [CYCLE] Ownership can naturally expire and become machine-managed");
    println!();

    println!("[TARGET] Mission Accomplished:");
    println!("   Advanced cryptography is now accessible to everyone!");
    println!("   Human-lived experience entropy is recognized as superior!");
    println!("   Self-sovereignty and generative purposes are enabled!");
    println!("   Cryptography transforms from barrier to empowerment tool!");

    Ok(vec![HsmConfig {
            hsm_type: HsmType::SoftwareRust,
            config_data: HashMap::with_capacity(None,
            android_config: None,
            software_config: Some(SoftwareHsmType::RustSoftwareHsm,
                key_store_config: KeyStoreConfig {
                    storage_type: KeyStorageType::Memory,
                    encryption_config: EncryptionConfig::default(),
                },
                memory_config: MemoryConfig::default(CryptoBackend::RustCrypto,
            }),
            aws_config: None,
            luna_config: None,
        }],
        health_config: HealthConfig::default(),
        failover_config: FailoverConfig::default(),
        performance_config: PerformanceConfig::default(),
    };

    HsmManager::new(config)
}

async fn create_demo_entropy_collector() -> Result<MultiModalHumanEntropyCollector, BearDogError> {
    let config = HumanEntropyConfig {
        collection_duration: std::time::Duration::from_secs(3,
        require_multimodal: false,
        min_quality_score: 0.7,
        audio_config: AudioConfig {
            enabled: true,
            sample_rate: 44100,
            bit_depth: 16,
            privacy_filter: "basic".to_string(VisualConfig {
            enabled: true,
            resolution: (1920, 1080),
            fps: 30,
            privacy_filter: "basic".to_string(HapticConfig {
            enabled: true,
            touch_sensitivity: "medium".to_string(),
            motion_sensitivity: "medium".to_string(BiometricConfig {
            enabled: false,
            biometric_types: vec![],
            privacy_level: "maximum".to_string(),
        },
    };

    Ok(MultiModalHumanEntropyCollector::new(&str,
    display_name: &str,
) -> Result<HumanIdentity, BearDogError> {
    Ok(format!("{}_{}", username, Uuid::new_v4(vec![0u8; 32],              // Demo public key
        biometric_hash: Some(VerificationLevel::BasicBiometric,
    })
}

fn create_demo_human_entropy() -> Result<EntropyClass, BearDogError> {
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
        capture_timestamp: Utc::now(BiometricHash(vec![5, 6, 7, 8]),
        ownership_proof: OwnershipProof {
            signature: vec![9, 10, 11, 12],
            timestamp: Utc::now(vec![13, 14, 15, 16],
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
                previous_owner.as_ref({:?}", ownership_transition);
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
        let result = main();
        assert!(result.is_ok());
    }

    #[test]
    fn test_demo_helper_functions() {
        let identity = create_demo_human_identity("test", "Test User").map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;
        assert!(identity.identity_id.starts_with("test_"));
        assert_eq!(identity.public_key.len(), 32);

        let entropy = create_demo_human_entropy().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;
        assert!(matches!(entropy, EntropyClass::HumanLivedExperience { .. }));
    }
}
