//! Comprehensive Test Suite for Universal HSM Discovery System
//!
//! This module provides thorough testing of the HSM discovery, classification,
//! tier management, and human entropy support features.

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::universal_hsm_discovery::tier_manager::TierManager;
    use crate::universal_hsm_discovery::universal_adapter::HsmAdapter;
    use std::time::Duration;
    use tokio;

    /// Test basic HSM discovery functionality
    #[tokio::test]
    async fn test_universal_hsm_discovery_creation() {
        let discovery = UniversalHsmDiscovery::new().expect("Failed to create discovery system");

        // Verify all components are initialized
        assert!(discovery.discovered_hsms.is_empty());

        let stats = discovery.get_discovery_stats();
        assert_eq!(stats.total_hsms, 0);
        assert_eq!(stats.human_entropy_hsms, 0);
    }

    /// Test HSM discovery process
    #[tokio::test]
    async fn test_hsm_discovery_process() {
        let mut discovery =
            UniversalHsmDiscovery::new().expect("Failed to create discovery system");

        // Test discovery with default config
        let discovered_hsms = discovery
            .discover_all_hsms()
            .await
            .expect("Discovery failed");

        // Should discover at least the BearDog native HSM and potentially others
        assert!(
            !discovered_hsms.is_empty(),
            "Should discover at least one HSM"
        );

        // Verify statistics are updated
        let stats = discovery.get_discovery_stats();
        assert!(stats.total_hsms > 0, "Should have discovered HSMs");

        println!("Discovered {} HSMs", stats.total_hsms);
        for (tier, count) in &stats.tier_distribution {
            println!("  Tier {:?}: {} HSMs", tier, count);
        }
    }

    /// Test human entropy HSM classification
    #[tokio::test]
    async fn test_human_entropy_classification() {
        let mut discovery =
            UniversalHsmDiscovery::new().expect("Failed to create discovery system");
        let _discovered_hsms = discovery
            .discover_all_hsms()
            .await
            .expect("Discovery failed");

        // Get HSMs that support human entropy
        let human_entropy_hsms = discovery.get_human_entropy_hsms();

        // Should find at least BearDog native HSM and potentially mobile HSMs
        assert!(
            !human_entropy_hsms.is_empty(),
            "Should find HSMs with human entropy support"
        );

        for hsm in &human_entropy_hsms {
            assert!(
                hsm.supports_human_entropy,
                "HSM should support human entropy"
            );
            assert!(
                hsm.capabilities.human_entropy.supports_human_entropy,
                "Capabilities should indicate human entropy support"
            );

            // Human entropy HSMs should be at least BasicHardware tier or higher
            assert!(
                hsm.assigned_tier >= HsmTier::BasicHardware,
                "Human entropy HSMs should be elevated to at least BasicHardware tier"
            );

            println!(
                "Human entropy HSM: {} (Tier: {:?})",
                hsm.hsm_id, hsm.assigned_tier
            );
        }
    }

    /// Test tier elevation for human entropy HSMs
    #[tokio::test]
    async fn test_human_entropy_tier_elevation() {
        let tier_manager = TierManager::new().expect("Failed to create tier manager");

        // Test HSM without human entropy - should get basic tier
        let basic_capabilities = create_basic_hsm_capabilities(false);
        let basic_tier = tier_manager
            .assign_tier(&basic_capabilities, false)
            .await
            .expect("Failed to assign tier");

        // Test HSM with human entropy - should get elevated tier
        let entropy_capabilities = create_basic_hsm_capabilities(true);
        let entropy_tier = tier_manager
            .assign_tier(&entropy_capabilities, true)
            .await
            .expect("Failed to assign tier");

        // Human entropy HSM should have higher tier
        assert!(
            entropy_tier > basic_tier,
            "Human entropy HSM should have higher tier than basic HSM"
        );

        println!("Basic HSM tier: {:?}", basic_tier);
        println!("Human entropy HSM tier: {:?}", entropy_tier);
    }

    /// Test capability detection for different HSM types
    #[tokio::test]
    async fn test_capability_detection() {
        let capability_detector = capability_detector::CapabilityDetector::new()
            .expect("Failed to create capability detector");

        // Test different interface types
        let test_interfaces = vec![
            HsmInterfaceType::AndroidStrongBox {
                security_level: "STRONGBOX".to_string(),
            },
            HsmInterfaceType::BearDogNative {
                instance_id: "test".to_string(),
            },
            HsmInterfaceType::AwsKms {
                region: "us-east-1".to_string(),
            },
            HsmInterfaceType::Pkcs11 {
                library_path: "/test/lib.so".to_string(),
            },
        ];

        for interface in test_interfaces {
            let capabilities = capability_detector
                .detect_capabilities(&interface)
                .await
                .expect("Failed to detect capabilities");

            // Verify basic capability structure
            assert!(
                !capabilities.key_generation.supported_algorithms.is_empty(),
                "Should have supported algorithms"
            );
            assert!(
                !capabilities
                    .crypto_operations
                    .encryption_algorithms
                    .is_empty(),
                "Should have encryption algorithms"
            );

            // Check human entropy support based on interface type
            match interface {
                HsmInterfaceType::AndroidStrongBox { .. }
                | HsmInterfaceType::BearDogNative { .. } => {
                    assert!(
                        capabilities.human_entropy.supports_human_entropy,
                        "Mobile and BearDog HSMs should support human entropy"
                    );
                    assert!(
                        capabilities.human_entropy.supports_ephemeral_seeds,
                        "Should support ephemeral seeds"
                    );
                }
                HsmInterfaceType::AwsKms { .. } => {
                    assert!(
                        !capabilities.human_entropy.supports_human_entropy,
                        "Cloud HSMs typically don't support human entropy"
                    );
                }
                _ => {}
            }

            println!(
                "Interface {:?}: Human entropy support = {}",
                interface, capabilities.human_entropy.supports_human_entropy
            );
        }
    }

    /// Test human entropy classifier detailed evaluation
    #[tokio::test]
    async fn test_human_entropy_classifier() {
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()
            .expect("Failed to create classifier");

        // Test high-quality human entropy capabilities
        let high_quality_capabilities = create_premium_human_entropy_capabilities();
        let assessment = classifier
            .assess_human_entropy_capabilities(&high_quality_capabilities)
            .await
            .expect("Failed to assess capabilities");

        assert!(
            assessment.supports_ephemeral_seeds,
            "Should support ephemeral seeds"
        );
        assert!(
            assessment.overall_score > 0.8,
            "Should have high overall score"
        );
        assert!(
            assessment.meets_tier_elevation_criteria,
            "Should meet tier elevation criteria"
        );
        assert_eq!(
            assessment.recommended_tier,
            HsmTier::HumanEntropyPremium,
            "Should recommend premium tier"
        );

        // Test low-quality human entropy capabilities
        let low_quality_capabilities = create_basic_human_entropy_capabilities();
        let low_assessment = classifier
            .assess_human_entropy_capabilities(&low_quality_capabilities)
            .await
            .expect("Failed to assess capabilities");

        assert!(
            low_assessment.overall_score < assessment.overall_score,
            "Low quality should have lower score"
        );
        assert!(
            low_assessment.recommended_tier < assessment.recommended_tier,
            "Low quality should have lower recommended tier"
        );

        println!(
            "High quality assessment: score={:.2}, tier={:?}",
            assessment.overall_score, assessment.recommended_tier
        );
        println!(
            "Low quality assessment: score={:.2}, tier={:?}",
            low_assessment.overall_score, low_assessment.recommended_tier
        );
    }

    /// Test operation-specific HSM selection
    #[tokio::test]
    async fn test_operation_hsm_selection() {
        let mut discovery =
            UniversalHsmDiscovery::new().expect("Failed to create discovery system");
        let _discovered_hsms = discovery
            .discover_all_hsms()
            .await
            .expect("Discovery failed");

        // Test selection for operation requiring human entropy
        let root_key_hsm = discovery
            .get_best_hsm_for_operation("root_key_generation")
            .await
            .expect("Failed to select HSM");

        if let Some(hsm) = root_key_hsm {
            // Root key generation should prefer human entropy HSMs
            assert!(
                hsm.supports_human_entropy,
                "Root key generation should use human entropy HSM"
            );
            assert!(
                hsm.assigned_tier >= HsmTier::CertifiedHardware,
                "Should use high-tier HSM for root key generation"
            );

            println!(
                "Selected HSM for root key generation: {} (Tier: {:?})",
                hsm.hsm_id, hsm.assigned_tier
            );
        }

        // Test selection for bulk operations (should prefer performance)
        let bulk_hsm = discovery
            .get_best_hsm_for_operation("bulk_encryption")
            .await
            .expect("Failed to select HSM");

        if let Some(hsm) = bulk_hsm {
            // Bulk encryption can use lower-tier HSMs for performance
            assert!(
                hsm.assigned_tier >= HsmTier::BasicHardware,
                "Should use at least basic hardware for bulk operations"
            );

            println!(
                "Selected HSM for bulk encryption: {} (Tier: {:?})",
                hsm.hsm_id, hsm.assigned_tier
            );
        }
    }

    /// Test universal adapter functionality
    #[tokio::test]
    async fn test_universal_adapter() {
        let adapter =
            universal_adapter::UniversalAdapter::new().expect("Failed to create universal adapter");

        // Test adapter registration
        let available_adapters = adapter.get_available_adapters();
        assert!(
            !available_adapters.is_empty(),
            "Should have registered adapters"
        );
        assert!(
            available_adapters.contains(&"android_strongbox".to_string()),
            "Should have Android StrongBox adapter"
        );
        assert!(
            available_adapters.contains(&"beardog_native".to_string()),
            "Should have BearDog Native adapter"
        );

        println!("Available adapters: {:?}", available_adapters);

        // Test connection to mock HSM
        let mock_hsm = create_mock_beardog_hsm();
        let health_status = adapter
            .test_connection(&mock_hsm)
            .await
            .expect("Failed to test connection");

        assert!(health_status.is_healthy, "Mock HSM should be healthy");
        assert!(
            health_status.response_time_ms > 0.0,
            "Should have response time"
        );

        println!(
            "Mock HSM health: healthy={}, response_time={}ms",
            health_status.is_healthy, health_status.response_time_ms
        );
    }

    /// Test human entropy seed generation
    #[tokio::test]
    async fn test_human_entropy_seed_generation() {
        let adapter =
            universal_adapter::UniversalAdapter::new().expect("Failed to create universal adapter");

        // Create requirements for human entropy seed
        let entropy_requirements = universal_adapter::HumanEntropyRequirements {
            collection_methods: vec![
                EntropyCollectionMethod::BiometricVariations {
                    template_noise: true,
                },
                EntropyCollectionMethod::TouchPatterns {
                    pressure_sensitive: true,
                },
            ],
            quality_threshold: 0.8,
            seed_lifetime: Duration::from_secs(300), // 5 minutes
            verification_required: true,
            real_time_collection: true,
        };

        // Test with BearDog Native HSM (supports human entropy)
        let mock_hsm = create_mock_beardog_hsm();
        let connection = adapter
            .connect(&mock_hsm)
            .await
            .expect("Failed to connect to mock HSM");

        // Since this is a mock test, we'll test the adapter's human entropy support check
        let beardog_adapter = universal_adapter::BearDogNativeAdapter::new()
            .expect("Failed to create BearDog adapter");
        let supports_entropy = beardog_adapter
            .supports_human_entropy()
            .await
            .expect("Failed to check entropy support");

        assert!(
            supports_entropy,
            "BearDog Native should support human entropy"
        );

        // Generate seed
        let seed = beardog_adapter
            .generate_human_entropy_seed(&connection, entropy_requirements)
            .await
            .expect("Failed to generate entropy seed");

        assert!(!seed.seed_data.is_empty(), "Should have seed data");
        assert!(seed.entropy_quality > 0.0, "Should have quality rating");
        assert!(
            seed.expires_at > seed.generated_at,
            "Expiration should be after generation"
        );
        assert!(
            !seed.collection_methods_used.is_empty(),
            "Should have used collection methods"
        );

        println!(
            "Generated entropy seed: quality={:.2}, methods={:?}",
            seed.entropy_quality, seed.collection_methods_used
        );
    }

    /// Test tier distribution and ranking
    #[tokio::test]
    async fn test_hsm_tier_distribution() {
        let mut discovery =
            UniversalHsmDiscovery::new().expect("Failed to create discovery system");
        let _discovered_hsms = discovery
            .discover_all_hsms()
            .await
            .expect("Discovery failed");

        // Test getting HSMs by tier
        for tier in [
            HsmTier::Software,
            HsmTier::BasicHardware,
            HsmTier::CertifiedHardware,
            HsmTier::HighSecurity,
            HsmTier::HumanEntropyPremium,
        ] {
            let hsms_in_tier = discovery.get_hsms_by_tier(tier);
            println!("Tier {:?}: {} HSMs", tier, hsms_in_tier.len());

            for hsm in hsms_in_tier {
                assert_eq!(hsm.assigned_tier, tier, "HSM should be in correct tier");
            }
        }

        // Verify human entropy HSMs are properly elevated
        let premium_hsms = discovery.get_hsms_by_tier(HsmTier::HumanEntropyPremium);
        for hsm in premium_hsms {
            assert!(
                hsm.supports_human_entropy,
                "Premium tier HSMs should support human entropy"
            );
            assert!(
                hsm.capabilities.human_entropy.supports_ephemeral_seeds,
                "Premium tier HSMs should support ephemeral seeds"
            );
        }
    }

    /// Test discovery configuration and customization
    #[tokio::test]
    async fn test_discovery_configuration() {
        let mut discovery =
            UniversalHsmDiscovery::new().expect("Failed to create discovery system");

        // Test custom configuration
        let custom_config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(10),
            max_concurrent_discoveries: 5,
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };

        discovery.update_config(custom_config);

        // Should still be able to discover with custom config
        let discovered_hsms = discovery
            .discover_all_hsms()
            .await
            .expect("Discovery with custom config failed");
        assert!(
            !discovered_hsms.is_empty(),
            "Should discover HSMs with custom config"
        );
    }

    /// Test concurrent discovery and thread safety
    #[tokio::test]
    async fn test_concurrent_discovery() {
        let mut handles = Vec::new();

        // Spawn multiple concurrent discovery tasks
        for i in 0..3 {
            let handle = tokio::spawn(async move {
                let mut discovery =
                    UniversalHsmDiscovery::new().expect("Failed to create discovery system");
                let discovered_hsms = discovery
                    .discover_all_hsms()
                    .await
                    .expect("Discovery failed");
                println!("Discovery task {}: found {} HSMs", i, discovered_hsms.len());
                discovered_hsms.len()
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        let mut total_discoveries = 0;
        for handle in handles {
            let count = handle.await.expect("Task failed");
            total_discoveries += count;
        }

        assert!(
            total_discoveries > 0,
            "Concurrent discoveries should find HSMs"
        );
        println!("Total concurrent discoveries: {}", total_discoveries);
    }

    // Helper functions for creating test data

    fn create_basic_hsm_capabilities(supports_human_entropy: bool) -> HsmCapabilities {
        HsmCapabilities {
            key_generation: KeyGenerationCapabilities {
                supported_algorithms: vec!["RSA".to_string(), "ECDSA".to_string()],
                key_sizes: vec![2048, 256],
                can_generate_in_hardware: false,
                supports_key_derivation: true,
                supports_secure_key_import: true,
                supports_key_wrapping: false,
                entropy_sources: vec!["Software RNG".to_string()],
                fips_compliant_generation: false,
            },
            crypto_operations: CryptoOperationCapabilities {
                encryption_algorithms: vec!["AES-GCM".to_string()],
                signing_algorithms: vec!["RSA-PSS".to_string()],
                hashing_algorithms: vec!["SHA-256".to_string()],
                key_agreement_algorithms: vec!["ECDH".to_string()],
                supports_streaming: false,
                supports_batch_operations: false,
                max_data_size: Some(1024),
                hardware_acceleration: false,
            },
            key_management: KeyManagementCapabilities {
                supports_key_backup: false,
                supports_key_recovery: false,
                supports_key_escrow: false,
                supports_key_rotation: true,
                supports_key_versioning: false,
                supports_key_attestation: false,
                key_storage_types: vec!["Software".to_string()],
                max_keys: Some(100),
            },
            advanced_features: AdvancedFeatureCapabilities {
                supports_secure_boot: false,
                supports_remote_attestation: false,
                supports_secure_channels: false,
                supports_multi_tenancy: false,
                supports_role_based_access: false,
                supports_audit_logging: false,
                supports_clustering: false,
                supports_load_balancing: false,
            },
            performance: PerformanceCapabilities {
                operations_per_second: std::collections::HashMap::new(),
                latency_ms: std::collections::HashMap::new(),
                throughput_mbps: Some(10.0),
                concurrent_operations: 1,
                memory_usage_mb: Some(64),
                power_consumption_watts: Some(2.0),
            },
            security: SecurityCapabilities {
                fips_140_level: None,
                common_criteria_level: None,
                tamper_resistance: TamperResistance::None,
                secure_key_storage: false,
                side_channel_resistance: false,
                fault_injection_resistance: false,
                certified_algorithms: vec!["AES".to_string()],
                security_certifications: vec![],
            },
            human_entropy: HumanEntropyCapabilities {
                supports_human_entropy,
                supports_ephemeral_seeds: supports_human_entropy,
                entropy_collection_methods: if supports_human_entropy {
                    vec![EntropyCollectionMethod::Keystroke {
                        timing_analysis: true,
                    }]
                } else {
                    vec![]
                },
                entropy_quality_assessment: supports_human_entropy,
                real_time_entropy_generation: supports_human_entropy,
                biometric_entropy_integration: false,
                user_interaction_entropy: supports_human_entropy,
                temporal_entropy_collection: false,
                entropy_verification: supports_human_entropy,
                ephemeral_seed_lifetime: if supports_human_entropy {
                    Some(Duration::from_secs(300))
                } else {
                    None
                },
            },
            api_support: ApiSupportCapabilities {
                pkcs11_support: false,
                jce_support: false,
                cng_support: false,
                openssl_engine: true,
                rest_api: false,
                grpc_api: false,
                graphql_api: false,
                custom_sdks: vec!["Test SDK".to_string()],
            },
            compliance: ComplianceCapabilities {
                fips_140_certified: false,
                common_criteria_certified: false,
                pci_dss_compliant: false,
                hipaa_compliant: false,
                gdpr_compliant: true,
                sox_compliant: false,
                compliance_certifications: vec![],
                audit_trail_support: false,
            },
        }
    }

    fn create_premium_human_entropy_capabilities() -> HsmCapabilities {
        let mut capabilities = create_basic_hsm_capabilities(true);

        // Enhance for premium tier
        capabilities.human_entropy = HumanEntropyCapabilities {
            supports_human_entropy: true,
            supports_ephemeral_seeds: true,
            entropy_collection_methods: vec![
                EntropyCollectionMethod::BiometricVariations {
                    template_noise: true,
                },
                EntropyCollectionMethod::BehavioralBiometrics {
                    pattern_recognition: true,
                },
                EntropyCollectionMethod::TouchPatterns {
                    pressure_sensitive: true,
                },
                EntropyCollectionMethod::VoicePatterns {
                    frequency_analysis: true,
                },
                EntropyCollectionMethod::Keystroke {
                    timing_analysis: true,
                },
            ],
            entropy_quality_assessment: true,
            real_time_entropy_generation: true,
            biometric_entropy_integration: true,
            user_interaction_entropy: true,
            temporal_entropy_collection: true,
            entropy_verification: true,
            ephemeral_seed_lifetime: Some(Duration::from_secs(300)),
        };

        capabilities.security.fips_140_level = Some(3);
        capabilities.security.tamper_resistance = TamperResistance::TamperResponsive;
        capabilities.key_generation.can_generate_in_hardware = true;

        capabilities
    }

    fn create_basic_human_entropy_capabilities() -> HsmCapabilities {
        let mut capabilities = create_basic_hsm_capabilities(true);

        // Basic human entropy support
        capabilities.human_entropy = HumanEntropyCapabilities {
            supports_human_entropy: true,
            supports_ephemeral_seeds: true,
            entropy_collection_methods: vec![EntropyCollectionMethod::Keystroke {
                timing_analysis: false,
            }],
            entropy_quality_assessment: false,
            real_time_entropy_generation: false,
            biometric_entropy_integration: false,
            user_interaction_entropy: true,
            temporal_entropy_collection: false,
            entropy_verification: false,
            ephemeral_seed_lifetime: Some(Duration::from_secs(300)),
        };

        capabilities
    }

    fn create_mock_beardog_hsm() -> DiscoveredHsm {
        DiscoveredHsm {
            hsm_id: "mock-beardog-native".to_string(),
            vendor: "BearDog".to_string(),
            model: "BearDog Native HSM".to_string(),
            version: "1.0.0-test".to_string(),
            interface_type: HsmInterfaceType::BearDogNative {
                instance_id: "test-instance".to_string(),
            },
            connection_info: HsmConnectionInfo {
                connection_type: ConnectionType::Local,
                authentication: AuthenticationMethod::None,
                endpoint: None,
                port: None,
                timeout: Duration::from_secs(30),
                retry_policy: RetryPolicy {
                    max_retries: 3,
                    base_delay: Duration::from_millis(50),
                    max_delay: Duration::from_secs(2),
                    backoff_multiplier: 1.5,
                },
                ssl_config: None,
            },
            capabilities: create_premium_human_entropy_capabilities(),
            assigned_tier: HsmTier::HumanEntropyPremium,
            supports_human_entropy: true,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
            last_health_check: chrono::Utc::now(),
            integration_status: IntegrationStatus::Ready,
        }
    }
}
