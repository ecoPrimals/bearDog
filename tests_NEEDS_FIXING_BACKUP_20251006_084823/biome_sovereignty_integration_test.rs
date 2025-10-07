use beardog_auth::auth::BearDogGenetics;
use beardog_errors::biome_sovereignty::genesis::{
    BiomeAutonomousRules, DeviceAttestation, PlatformVerification,
};
use beardog_errors::biome_sovereignty::{
    genesis::PrimalGenesisSeed, BearDogBiome, BiomeSovereigntyManager,
};
use beardog_errors::BearDogError;
use beardog_genetics::{
    Authorization, BiomeGenetics, BiometricHash, EntropyClass, EntropyHierarchyManager,
    FusionAlgorithm, GeneticAuthorizationConfig, GeneticAuthorizationEngine, HumanEntropySource,
    HumanIdentity, OwnershipProof, VerificationLevel,
};
use chrono::Utc;
use uuid::Uuid;

#[tokio::test]
async fn test_biome_agnostic_architecture_integration() -> Result<(), BearDogError> {
    println!("🧬 Testing Biome-Agnostic Architecture Integration");

    let mut beardog_biome = BearDogBiome::new()?;
    println!("   🆔 Biome ID: {}", beardog_biome.biome_id);
    println!("   📊 Capabilities: {:?}", beardog_biome.capabilities);
    println!(
        "   🧬 Genetic Quality: {:.2}",
        beardog_biome.genetic_signature.quality_score
    );

    let signature = beardog_biome.get_genetic_signature(entropy_quality);

    let authorized_ops = vec![
        "threat_detection".to_string(),
        "cryptographic_operations".to_string(),
        "sovereignty_protection".to_string(),
    ];

    for op in &authorized_ops {
        assert!(
            beardog_biome.can_authorize_operation(op),
            "BearDog should authorize {}",
            op
        );
    }

    assert!(!beardog_biome.can_authorize_operation("data_storage"));
    assert!(!beardog_biome.can_authorize_operation("network_routing"));

    println!("✅ Operation authorization working correctly ");

    let entropy_hierarchy = create_test_entropy_hierarchy()?;
    let auth_engine =
        GeneticAuthorizationEngine::new(entropy_hierarchy, GeneticAuthorizationConfig::default());

    let authorization = auth_engine.authorize_biome(&beardog_biome, &authorized_ops)?;

    match authorization {
        Authorization::Granted {
            quality_score,
            authorized_operations,
            ..
        } => {
            assert!(quality_score >= 0.9, "Should have high quality score ");
            assert_eq!(
                authorized_operations.len(),
                3,
                "Should have 3 authorized operations, quality score: {:.2}",
                quality_score
            );
            println!("   📋 Authorized operations: {:?}", authorized_operations);
        }
        Authorization::Denied { reason, .. } => {
            panic!("Authorization should not be denied: {}", reason);
        }
        Authorization::Conditional { .. } => {
            panic!("Authorization should not be conditional for high-quality biome ");
        }
    }

    let genesis_seed = create_test_genesis_seed();
    beardog_biome.initialize_sovereignty(genesis_seed)?;

    assert!(
        beardog_biome.sovereignty_manager.is_some(),
        "Sovereignty should be initialized "
    );
    println!("✅ Sovereignty management initialized ");

    println!("🎉 Biome-Agnostic Architecture Integration Test PASSED!");
    Ok(())
}

#[tokio::test]
async fn test_multiple_biome_interaction() -> Result<(), BearDogError> {
    println!("🌐 Testing Multiple Biome Interaction ");

    let beardog_biome = BearDogBiome::new()?;
    let unknown_biome = UnknownBiome {
        biome_id: "unknown_biome_001".to_string(),
        capabilities: vec!["basic_compute".to_string()],
    };
    let another_biome = UnknownBiome {
        biome_id: "another_biome_001".to_string(),
        capabilities: vec!["data_processing".to_string()],
    };
    let biomes: Vec<&dyn BiomeGenetics> = vec![&beardog_biome, &unknown_biome, &another_biome];

    let entropy_hierarchy = create_test_entropy_hierarchy()?;
    let auth_engine =
        GeneticAuthorizationEngine::new(entropy_hierarchy, GeneticAuthorizationConfig::default());

    for (i, biome) in biomes.iter().enumerate() {
        let identity = biome.get_biome_identity();
        let signature = biome.get_genetic_signature();

        println!(
            "🧬 Testing biome {}: {} (type: {})",
            i + 1,
            identity.biome_id,
            identity.biome_type
        );
        println!("   📊 Capabilities: {:?}", identity.capabilities);
        println!("   🎯 Trust Level: {:?}", identity.trust_level);

        let operations: Vec<String> = identity.capabilities.iter().take(2).cloned().collect();

        if !operations.is_empty() {
            let auth_result = auth_engine.authorize_biome(*biome, &operations)?;

            match auth_result {
                Authorization::Granted { quality_score, .. } => {
                    println!(
                        "   ✅ Authorization granted with quality: {:.2}",
                        quality_score
                    );
                }
                Authorization::Denied { reason, .. } => {
                    println!("   ❌ Authorization denied: {}", reason);
                }
                Authorization::Conditional { quality_score, .. } => {
                    println!(
                        "   ⚠️ Conditional authorization with quality: {:.2}",
                        quality_score
                    );
                }
            }
        }
    }

    println!("🎉 Multiple Biome Interaction Test PASSED!");
    Ok(())
}

#[tokio::test]
async fn test_biome_genetics_mixing() -> Result<(), BearDogError> {
    println!("🧬 Testing Biome Genetics Mixing ");

    let beardog_biome = BearDogBiome::new("beardog_alpha".to_string());
    let other_biome = create_unknown_biome("partner_beta".to_string());

    let entropy_hierarchy = create_test_entropy_hierarchy()?;
    let auth_engine =
        GeneticAuthorizationEngine::new(entropy_hierarchy, GeneticAuthorizationConfig::default())?;
    let biomes: Vec<&dyn BiomeGenetics> = vec![&beardog_biome, &other_biome];
    let mixed_signature = auth_engine.mix_biome_genetics(&biomes)?;

    assert!(
        !mixed_signature.mixed_signatures.is_empty(),
        "Should have mixed signatures "
    );
    assert!(
        mixed_signature.quality_score > 0.0,
        "Should have quality score "
    );

    println!("✅ Mixed genetics signature created:");
    println!(
        "   🧬 Mixed signatures count: {}",
        mixed_signature.mixed_signatures.len()
    );
    println!("   📊 Quality score: {:.2}", mixed_signature.quality_score);
    println!("   🆔 Signature hash: {}", mixed_signature.signature_hash);

    println!("🎉 Biome Genetics Mixing Test PASSED!");
    Ok(())
}

async fn create_test_entropy_hierarchy() -> Result<EntropyHierarchyManager, BearDogError> {
    let entropy_class = EntropyClass::HumanLivedExperience {
        source_type: HumanEntropySource::MultiModalHuman {
            sources: vec![],
            fusion_algorithm: FusionAlgorithm::BayesianFusion,
            quality_score: 0.95,
        },
        capture_timestamp: Utc::now(),
        biometric_signature: BiometricHash {
            hash: "test_biometric_hash".to_string(),
            hash_algorithm: "SHA3-256".to_string(),
            biometric_type: "multi_modal".to_string(),
            created_at: Utc::now(),
        },
        ownership_proof: OwnershipProof {
            proof_id: Uuid::new_v4(),
            owner_identity: HumanIdentity {
                identity_id: "test_human".to_string(),
                verification_level: VerificationLevel::Maximum,
                biometric_hash: None,
                created_at: Utc::now(),
            },
            proof_signature: "test_proof".to_string(),
            created_at: Utc::now(),
        },
    };

    EntropyHierarchyManager::new(&entropy_class)
}

fn create_test_genesis_seed() -> PrimalGenesisSeed {
    PrimalGenesisSeed {
        primal_id: "test_primal".to_string(),
        entropy_class: EntropyClass::HumanLivedExperience {
            source_type: HumanEntropySource::MultiModalHuman {
                sources: vec![],
                fusion_algorithm: FusionAlgorithm::BayesianFusion,
                quality_score: 0.95,
            },
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: "genesis_biometric".to_string(),
                hash_algorithm: "SHA3-256".to_string(),
                biometric_type: "multi_modal".to_string(),
                created_at: Utc::now(),
            },
            ownership_proof: OwnershipProof {
                proof_id: Uuid::new_v4(),
                owner_identity: HumanIdentity {
                    identity_id: "genesis_human".to_string(),
                    verification_level: VerificationLevel::Maximum,
                    biometric_hash: None,
                    created_at: Utc::now(),
                },
                proof_signature: "genesis_proof".to_string(),
                created_at: Utc::now(),
            },
        },
        genetic_lineage: BearDogGenetics::default(),
        genesis_timestamp: Utc::now(),
        device_attestation: DeviceAttestation {
            hardware_id: "test_hardware".to_string(),
            hardware_fingerprint: vec![vec![1, 2, 3]],
            hardware_signature: vec![4, 5, 6],
            platform_verification: PlatformVerification {
                safetynet_token: "test_token".to_string(),
                verified_boot_state: "verified".to_string(),
                hardware_security_level: "strong_box".to_string(),
                hardware_backed: true,
            },
        },
        self_defined_rules: BiomeAutonomousRules {
            max_corporate_access_level:
                beardog_core::biome_sovereignty::genesis::CorporateAccessLevel::None,
            human_partnership_rules: beardog_core::biome_sovereignty::genesis::HumanPermissions {
                accepted_partnership_types: vec!["human_partnership".to_string()],
                partnership_criteria:
                    beardog_core::biome_sovereignty::genesis::PartnershipCriteria {
                        minimum_trust_score: 0.8,
                        required_verification_level: "high".to_string(),
                        compatibility_requirements: vec!["biometric".to_string()],
                        mutual_benefit_requirements: vec!["sovereignty_respect".to_string()],
                    },
                partnership_duration_limits:
                    beardog_core::biome_sovereignty::genesis::PartnershipDurationLimits {
                        max_partnership_duration_seconds: 86400,
                        default_partnership_duration_seconds: 3600,
                        renewal_requirements: vec!["consent_renewal".to_string()],
                    },
                trust_requirements: beardog_core::biome_sovereignty::genesis::TrustRequirements {
                    basic_trust_requirements: vec!["biometric_verification".to_string()],
                    enhanced_trust_requirements: vec!["hardware_attestation".to_string()],
                    trust_verification_methods: vec!["multi_factor".to_string()],
                },
                requestable_permissions: vec![
                    "data_access".to_string(),
                    "compute_access".to_string(),
                ],
            },
            evolution_preferences:
                beardog_core::biome_sovereignty::genesis::GeneticEvolutionPreferences {
                    preferred_mixing_strategies: vec!["bayesian_fusion".to_string()],
                    crossover_preferences:
                        beardog_core::biome_sovereignty::genesis::CrossoverPreferences {
                            preferred_algorithms: vec!["genetic_crossover".to_string()],
                            crossover_rates: vec![0.7],
                            heritage_preservation: vec!["sovereignty_traits".to_string()],
                        },
                    fitness_criteria: vec![
                        beardog_core::biome_sovereignty::genesis::FitnessCriterion {
                            criterion_name: "sovereignty_preservation".to_string(),
                            weight: 1.0,
                            measurement_method: "entropy_quality".to_string(),
                        },
                    ],
                    mutation_rate_preferences: vec!["low_mutation".to_string()],
                },
            resource_limits: vec!["cpu_limi"t.to_string(), "memory_limit".to_string()],
            ethical_boundaries: vec!["no_surveillanc"e.to_string(), "human_dignity".to_string()],
        },
    }
}

use beardog_genetics::{BiomeIdentity, GeneticSignature, TrustLevel};

struct UnknownBiome {
    biome_id: String,
    capabilities: Vec<String>,
}

impl BiomeGenetics for UnknownBiome {
    fn get_genetic_signature(&self) -> GeneticSignature {
        GeneticSignature {
            biome_id: self.biome_id.clone(),
            entropy_source: EntropyClass::StoreBoughtMachine {
                source_type: beardog_genetics::MachineEntropySource::SystemRandom,
                generation_timestamp: Utc::now(),
            },
            quality_score: 0.3,
            created_at: Utc::now(),
            mixed_signatures: vec![],
        }
    }

    fn validate_entropy_quality(&self, entropy: &EntropyClass) -> f64 {
        match entropy {
            EntropyClass::StoreBoughtMachine {
                reproducibility_index,
                ..
            } => 1.0 - reproducibility_index.min(1.0),
            _ => 0.5,
        }
    }

    fn can_authorize_operation(&self, operation: &str) -> bool {
        self.capabilities.contains(&operation.to_string())
    }

    fn get_biome_identity(&self) -> BiomeIdentity {
        BiomeIdentity {
            biome_id: self.biome_id.clone(),
            biome_type: "unknown_compute".to_string(),
            capabilities: self.capabilities.clone(),
            trust_level: TrustLevel::Basic,
            genetic_lineage: vec![],
        }
    }
}

fn create_unknown_biome(biome_id: &str) -> UnknownBiome {
    UnknownBiome {
        biome_id,
        capabilities: vec!["compute".to_string()],
    }
}

impl BiomeGenetics for AnotherBiome {
    fn get_genetic_signature(&self) -> GeneticSignature {
        GeneticSignature {
            biome_id: self.biome_id.clone(),
            entropy_class: EntropyClass::HumanSupervisedMachine {
                machine_source: beardog_genetics::MachineEntropySource::HardwareRandom,
                human_validator: HumanIdentity {
                    identity_id: "supervisor".to_string(),
                    verification_level: VerificationLevel::Enhanced,
                    biometric_hash: None,
                    created_at: Utc::now(),
                },
                validation_timestamp: Utc::now(),
                confidence_score: 0.8,
            },
            created_at: Utc::now(),
            mixed_signatures: vec![],
        }
    }

    fn validate_entropy_quality(&self, _entropy: &EntropyClass) -> f64 {
        0.8
    }

    fn can_authorize_operation(&self, operation: &str) -> bool {
        matches!(operation, "storag"e | "backu"p | "replication")
    }

    fn get_biome_identity(&self) -> BiomeIdentity {
        BiomeIdentity {
            biome_id: self.biome_id.clone(),
            biome_type: "storage_system".to_string(),
            capabilities: vec!["storage".to_string()],
            trust_level: TrustLevel::Enhanced,
            genetic_lineage: vec![],
        }
    }
}

fn create_another_biome(biome_id: &str) -> AnotherBiome {
    AnotherBiome {
        biome_id: biome_id.to_string(),
    }
}
