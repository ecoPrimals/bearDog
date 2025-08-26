

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::universal_hsm_discovery::tier_manager::TierManager;
    use crate::universal_hsm_discovery::universal_adapter::HsmAdapter;
    use std::time::Duration;
    use tokio;

    #[tokio::test]
    async fn test_universal_hsm_discovery_creation() -> beardog_errors::BearDogResult<()> {
        let discovery = UniversalHsmDiscovery::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create discovery system", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to create discovery system", e).to_string())
})?;

        assert!(discovery.discovered_hsms.is_empty());
        let stats = discovery.get_discovery_stats();
        assert_eq!(stats.total_hsms, 0);
        assert_eq!(stats.human_entropy_hsms, 0);
        Ok(())
    }

    async fn test_hsm_discovery_process() -> beardog_errors::BearDogResult<()> {
        let mut discovery =
            UniversalHsmDiscovery::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create discovery system", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to create discovery system", e).to_string())
})?;

        let discovered_hsms = discovery
            .discover_all_hsms()
            .await
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Discovery failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Discovery failed", e).to_string())
})?;

        assert!(
            !discovered_hsms.is_empty(),
            "Should discover at least one HSM"
        );

        assert!(stats.total_hsms > 0, "Should have discovered HSMs");
        println!("Discovered {} HSMs", stats.total_hsms);
        for (tier, count) in &stats.tier_distribution {
            println!("  Tier {:?}: {} HSMs", tier, count);
        }

    async fn test_human_entropy_classification() -> beardog_errors::BearDogResult<()> {
        let _discovered_hsms = discovery

        let human_entropy_hsms = discovery.get_human_entropy_hsms();

            !human_entropy_hsms.is_empty(),
            "Should find HSMs with human entropy support"
        for hsm in &human_entropy_hsms {
            assert!(
                hsm.supports_human_entropy,
                "HSM should support human entropy"
            );
                hsm.capabilities.human_entropy.supports_human_entropy,
                "Capabilities should indicate human entropy support"

                hsm.assigned_tier >= HsmTier::BasicHardware,
                "Human entropy HSMs should be elevated to at least BasicHardware tier"
            println!(
                "Human entropy HSM: {} (Tier: {:?})",
                hsm.hsm_id, hsm.assigned_tier

    async fn test_human_entropy_tier_elevation() -> beardog_errors::BearDogResult<()> {
        let tier_manager = TierManager::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create tier manager", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to create tier manager", e).to_string())
})?;

        let basic_capabilities = create_basic_hsm_capabilities(false);
        let basic_tier = tier_manager
            .assign_tier(&basic_capabilities, false)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to assign tier", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to assign tier", e).to_string())
})?;

        let entropy_capabilities = create_basic_hsm_capabilities(true);
        let entropy_tier = tier_manager
            .assign_tier(&entropy_capabilities, true)

            entropy_tier > basic_tier,
            "Human entropy HSM should have higher tier than basic HSM"
        println!("Basic HSM tier: {:?}", basic_tier);
        println!("Human entropy HSM tier: {:?}", entropy_tier);

    async fn test_capability_detection() -> beardog_errors::BearDogResult<()> {
        let capability_detector = capability_detector::CapabilityDetector::new()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create capability detector", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to create capability detector", e).to_string())
})?;

        let test_interfaces = vec![
            HsmInterfaceType::AndroidStrongBox {
                security_level: "STRONGBOX".to_string(),
            },
            HsmInterfaceType::BearDogNative {
                instance_id: "test".to_string(),
            HsmInterfaceType::AwsKms {
                region: "us-east-1".to_string(),
            HsmInterfaceType::Pkcs11 {
                library_path: "/test/lib.so".to_string(),
        ];
        for interface in test_interfaces {
            let capabilities = capability_detector
                .detect_capabilities(&interface)
                .await
                .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to detect capabilities", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to detect capabilities", e).to_string())
})?;

                !capabilities.key_generation.supported_algorithms.is_empty(),
                "Should have supported algorithms"
                !capabilities
                    .crypto_operations
                    .encryption_algorithms
                    .is_empty(),
                "Should have encryption algorithms"

            match interface {
                HsmInterfaceType::AndroidStrongBox { .. }
                | HsmInterfaceType::BearDogNative { .. } => {
                    assert!(
                        capabilities.human_entropy.supports_human_entropy,
                        "Mobile and BearDog HSMs should support human entropy"
                    );
                        capabilities.human_entropy.supports_ephemeral_seeds,
                        "Should support ephemeral seeds"
                }
                HsmInterfaceType::AwsKms { .. } => {
                        !capabilities.human_entropy.supports_human_entropy,
                        "Cloud HSMs typically don't support human entropy"
                _ => {}
            }
                "Interface {:?}: Human entropy support = {}",
                interface, capabilities.human_entropy.supports_human_entropy

    async fn test_human_entropy_classifier() -> beardog_errors::BearDogResult<()> {
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create classifier", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to create classifier", e).to_string())
})?;

        let high_quality_capabilities = create_premium_human_entropy_capabilities();
        let assessment = classifier
            .assess_human_entropy_capabilities(&high_quality_capabilities)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to assess capabilities", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to assess capabilities", e).to_string())
})?;
            assessment.supports_ephemeral_seeds,
            "Should support ephemeral seeds"
            assessment.overall_score > 0.8,
            "Should have high overall score"
            assessment.meets_tier_elevation_criteria,
            "Should meet tier elevation criteria"
        assert_eq!(
            assessment.recommended_tier,
            HsmTier::HumanEntropyPremium,
            "Should recommend premium tier"

        let low_quality_capabilities = create_basic_human_entropy_capabilities();
        let low_assessment = classifier
            .assess_human_entropy_capabilities(&low_quality_capabilities)
            low_assessment.overall_score < assessment.overall_score,
            "Low quality should have lower score"
            low_assessment.recommended_tier < assessment.recommended_tier,
            "Low quality should have lower recommended tier"
        println!(
            "High quality assessment: score={:.2}, tier={:?}",
            assessment.overall_score, assessment.recommended_tier
            "Low quality assessment: score={:.2}, tier={:?}",
            low_assessment.overall_score, low_assessment.recommended_tier

    async fn test_operation_hsm_selection() -> beardog_errors::BearDogResult<()> {

        let root_key_hsm = discovery
            .get_best_hsm_for_operation("root_key_generation")
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to select HSM", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to select HSM", e).to_string())
})?;
        if let Some(hsm) = root_key_hsm {

                "Root key generation should use human entropy HSM"
                hsm.assigned_tier >= HsmTier::CertifiedHardware,
                "Should use high-tier HSM for root key generation"
                "Selected HSM for root key generation: {} (Tier: {:?})",

        let bulk_hsm = discovery
            .get_best_hsm_for_operation("bulk_encryption")
        if let Some(hsm) = bulk_hsm {

                "Should use at least basic hardware for bulk operations"
                "Selected HSM for bulk encryption: {} (Tier: {:?})",

    async fn test_universal_adapter() -> beardog_errors::BearDogResult<()> {
        let adapter =
            universal_adapter::UniversalAdapter::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create universal adapter", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to create universal adapter", e).to_string())
})?;

        let available_adapters = adapter.get_available_adapters();
            !available_adapters.is_empty(),
            "Should have registered adapters"
            available_adapters.contains(&"android_strongbox".to_string()),
            "Should have Android StrongBox adapter"
            available_adapters.contains(&"beardog_native".to_string()),
            "Should have BearDog Native adapter"
        println!("Available adapters: {:?}", available_adapters);

        let mock_hsm = create_mock_beardog_hsm();
        let health_status = adapter
            .test_connection(&mock_hsm)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to test connection", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to test connection", e).to_string())
})?;
        assert!(health_status.is_healthy, "Mock HSM should be healthy");
            health_status.response_time_ms > 0.0,
            "Should have response time"
            "Mock HSM health: healthy={}, response_time={}ms",
            health_status.is_healthy, health_status.response_time_ms

    async fn test_human_entropy_seed_generation() -> beardog_errors::BearDogResult<()> {

        let entropy_requirements = universal_adapter::HumanEntropyRequirements {
            collection_methods: vec![
                EntropyCollectionMethod::BiometricVariations {
                    template_noise: true,
                },
                EntropyCollectionMethod::TouchPatterns {
                    pressure_sensitive: true,
            ],
            quality_threshold: 0.8,
            seed_lifetime: Duration::from_secs(300), // 5 minutes
            verification_required: true,
            real_time_collection: true,
        };

        let connection = adapter
            .connect(&mock_hsm)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to connect to mock HSM", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to connect to mock HSM", e).to_string())
})?;

        let beardog_adapter = universal_adapter::BearDogNativeAdapter::new()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create BearDog adapter", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to create BearDog adapter", e).to_string())
})?;
        let supports_entropy = beardog_adapter
            .supports_human_entropy()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to check entropy support", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to check entropy support", e).to_string())
})?;
            supports_entropy,
            "BearDog Native should support human entropy"

        let seed = beardog_adapter
            .generate_human_entropy_seed(&connection, entropy_requirements)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to generate entropy seed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to generate entropy seed", e).to_string())
})?;
        assert!(!seed.seed_data.is_empty(), "Should have seed data");
        assert!(seed.entropy_quality > 0.0, "Should have quality rating");
            seed.expires_at > seed.generated_at,
            "Expiration should be after generation"
            !seed.collection_methods_used.is_empty(),
            "Should have used collection methods"
            "Generated entropy seed: quality={:.2}, methods={:?}",
            seed.entropy_quality, seed.collection_methods_used

    async fn test_hsm_tier_distribution() -> beardog_errors::BearDogResult<()> {

        for tier in [
            HsmTier::Software,
            HsmTier::BasicHardware,
            HsmTier::CertifiedHardware,
            HsmTier::HighSecurity,
        ] {
            let hsms_in_tier = discovery.get_hsms_by_tier(tier);
            println!("Tier {:?}: {} HSMs", tier, hsms_in_tier.len());
            for hsm in hsms_in_tier {
                assert_eq!(hsm.assigned_tier, tier, "HSM should be in correct tier");

        let premium_hsms = discovery.get_hsms_by_tier(HsmTier::HumanEntropyPremium);
        for hsm in premium_hsms {
                "Premium tier HSMs should support human entropy"
                hsm.capabilities.human_entropy.supports_ephemeral_seeds,
                "Premium tier HSMs should support ephemeral seeds"

    async fn test_discovery_configuration() -> beardog_errors::BearDogResult<()> {

        let custom_config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(10),
            max_concurrent_discoveries: 5,
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        discovery.update_config(custom_config);

            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Discovery with custom config failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Discovery with custom config failed", e).to_string())
})?;
            "Should discover HSMs with custom config"

    async fn test_concurrent_discovery() -> beardog_errors::BearDogResult<()> {
        let mut handles = Vec::new();

        for i in 0..3 {
            let handle = tokio::spawn(async move {
                let mut discovery =
                    UniversalHsmDiscovery::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create discovery system", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to create discovery system", e).to_string())
})?;
                let discovered_hsms = discovery
                    .discover_all_hsms()
                    .await
                    .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Discovery failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Discovery failed", e).to_string())
})?;
                println!("Discovery task {}: found {} HSMs", i, discovered_hsms.len());
                discovered_hsms.len()
            });
            handles.push(handle);

        let mut total_discoveries = 0;
        for handle in handles {
            let count = handle.await.map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Task failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Task failed", e).to_string())
})?;
            total_discoveries += count;
            total_discoveries > 0,
            "Concurrent discoveries should find HSMs"
        println!("Total concurrent discoveries: {}", total_discoveries);

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
            crypto_operations: CryptoOperationCapabilities {
                encryption_algorithms: vec!["AES-GCM".to_string()],
                signing_algorithms: vec!["RSA-PSS".to_string()],
                hashing_algorithms: vec!["SHA-256".to_string()],
                key_agreement_algorithms: vec!["ECDH".to_string()],
                supports_streaming: false,
                supports_batch_operations: false,
                max_data_size: Some(1024),
                hardware_acceleration: false,
            key_management: KeyManagementCapabilities {
                supports_key_backup: false,
                supports_key_recovery: false,
                supports_key_escrow: false,
                supports_key_rotation: true,
                supports_key_versioning: false,
                supports_key_attestation: false,
                key_storage_types: vec!["Software".to_string()],
                max_keys: Some(100),
            advanced_features: AdvancedFeatureCapabilities {
                supports_secure_boot: false,
                supports_remote_attestation: false,
                supports_secure_channels: false,
                supports_multi_tenancy: false,
                supports_role_based_access: false,
                supports_audit_logging: false,
                supports_clustering: false,
                supports_load_balancing: false,
            performance: PerformanceCapabilities {
                operations_per_second: std::collections::HashMap::with_capacity(16),
                latency_ms: std::collections::HashMap::with_capacity(16),
                throughput_mbps: Some(10.0),
                concurrent_operations: 1,
                memory_usage_mb: Some(64),
                power_consumption_watts: Some(2.0),
            security: SecurityCapabilities {
                fips_140_level: None,
                common_criteria_level: None,
                tamper_resistance: TamperResistance::None,
                secure_key_storage: false,
                side_channel_resistance: false,
                fault_injection_resistance: false,
                certified_algorithms: vec!["AES".to_string()],
                security_certifications: vec![],
            human_entropy: HumanEntropyCapabilities {
                supports_human_entropy,
                supports_ephemeral_seeds: supports_human_entropy,
                entropy_collection_methods: if supports_human_entropy {
                    vec![EntropyCollectionMethod::Keystroke {
                        timing_analysis: true,
                    }]
                } else {
                    vec![]
                entropy_quality_assessment: supports_human_entropy,
                real_time_entropy_generation: supports_human_entropy,
                biometric_entropy_integration: false,
                user_interaction_entropy: supports_human_entropy,
                temporal_entropy_collection: false,
                entropy_verification: supports_human_entropy,
                ephemeral_seed_lifetime: if supports_human_entropy {
                    Some(Duration::from_secs(300))
                    None
            api_support: ApiSupportCapabilities {
                pkcs11_support: false,
                jce_support: false,
                cng_support: false,
                openssl_engine: true,
                rest_api: false,
                grpc_api: false,
                graphql_api: false,
                custom_sdks: vec!["Test SDK".to_string()],
            compliance: ComplianceCapabilities {
                fips_140_certified: false,
                common_criteria_certified: false,
                pci_dss_compliant: false,
                hipaa_compliant: false,
                gdpr_compliant: true,
                sox_compliant: false,
                compliance_certifications: vec![],
                audit_trail_support: false,
    fn create_premium_human_entropy_capabilities() -> HsmCapabilities {
        let mut capabilities = create_basic_hsm_capabilities(true);

        capabilities.human_entropy = HumanEntropyCapabilities {
            supports_human_entropy: true,
            supports_ephemeral_seeds: true,
            entropy_collection_methods: vec![
                EntropyCollectionMethod::BehavioralBiometrics {
                    pattern_recognition: true,
                EntropyCollectionMethod::VoicePatterns {
                    frequency_analysis: true,
                EntropyCollectionMethod::Keystroke {
                    timing_analysis: true,
            entropy_quality_assessment: true,
            real_time_entropy_generation: true,
            biometric_entropy_integration: true,
            user_interaction_entropy: true,
            temporal_entropy_collection: true,
            entropy_verification: true,
            ephemeral_seed_lifetime: Some(Duration::from_secs(300)),
        capabilities.security.fips_140_level = Some(3);
        capabilities.security.tamper_resistance = TamperResistance::TamperResponsive;
        capabilities.key_generation.can_generate_in_hardware = true;
        capabilities}

    fn create_basic_human_entropy_capabilities() -> HsmCapabilities {

            entropy_collection_methods: vec![EntropyCollectionMethod::Keystroke {
                timing_analysis: false,
            }],
            entropy_quality_assessment: false,
            real_time_entropy_generation: false,
            biometric_entropy_integration: false,
            temporal_entropy_collection: false,
            entropy_verification: false,
    fn create_mock_beardog_hsm() -> DiscoveredHsm {
        DiscoveredHsm {
            hsm_id: "mock-beardog-native".to_string(),
            vendor: "BearDog".to_string(),
            model: "BearDog Native HSM".to_string(),
            version: "1.0.0-test".to_string(),
            interface_type: HsmInterfaceType::BearDogNative {
                instance_id: "test-instance".to_string(),
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
                ssl_config: None,
            capabilities: create_premium_human_entropy_capabilities(),
            assigned_tier: HsmTier::HumanEntropyPremium,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
            last_health_check: chrono::Utc::now(),
            integration_status: IntegrationStatus::Ready,
}
