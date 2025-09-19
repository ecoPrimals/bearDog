// MODERNIZATION NOTE: This file contains vendor-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
use beardog_errors::BearDogError;

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::universal_hsm_discovery::tier_manager::TierManager;
    use crate::universal_hsm_discovery::universal_adapter::HsmAdapter;
    use std::time::Duration;
    use tokio;

    #[tokio::test]
    fn test_universal_hsm_discovery_creation() -> Result<(), BearDogError> {
        let discovery = UniversalHsmDiscovery::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create discovery system", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to create discovery system", e))
})?;

        assert!(discovery.discovered_hsms.is_empty());
        let stats = discovery.get_discovery_stats();
        assert_eq!(stats.total_hsms, 0);
        assert_eq!(stats.human_entropy_hsms, 0);
        Ok(())
    }


    fn test_hsm_discovery_process() -> Result<(), BearDogError> {
        let mut discovery =
            UniversalHsmDiscovery::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create discovery system", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to create discovery system", e))
})?;

        let discovered_hsms = discovery
            .discover_all_hsms()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Discovery failed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Discovery failed", e))
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


    fn test_human_entropy_classification() -> Result<(), BearDogError> {
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


    fn test_human_entropy_tier_elevation() -> Result<(), BearDogError> {
        let tier_manager = TierManager::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create tier manager", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to create tier manager", e))
})?;

        let basic_capabilities = create_basic_hsm_capabilities(false);
        let basic_tier = tier_manager
            .assign_tier(&basic_capabilities, false)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to assign tier", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to assign tier", e))
})?;

        let entropy_capabilities = create_basic_hsm_capabilities({:?}", basic_tier);
        println!("Human entropy HSM tier: {:?}", entropy_tier);


    fn test_capability_detection() -> Result<(), BearDogError> {
        let capability_detector = capability_detector::CapabilityDetector::new()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create capability detector", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to create capability detector", e))
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
                .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to detect capabilities", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to detect capabilities", e))
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


    fn test_human_entropy_classifier() -> Result<(), BearDogError> {
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create classifier", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to create classifier", e))
})?;

        let high_quality_capabilities = create_premium_human_entropy_capabilities();
        let assessment = classifier
            .assess_human_entropy_capabilities(&high_quality_capabilities)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to assess capabilities", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to assess capabilities", e))
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

        let low_quality_capabilities = create_basic_human_entropy_capabilities(score={:.2}, tier={:?}",
            assessment.overall_score, assessment.recommended_tier
            "Low quality assessment: score={:.2}, tier={:?}",
            low_assessment.overall_score, low_assessment.recommended_tier


    fn test_operation_hsm_selection() -> Result<(), BearDogError> {

        let root_key_hsm = discovery
            .get_best_hsm_for_operation("root_key_generation")
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to select HSM", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to select HSM", e))
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


    fn test_universal_adapter() -> Result<(), BearDogError> {
        let adapter =
            universal_adapter::UniversalAdapter::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create universal adapter", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to create universal adapter", e))
})?;

        let available_adapters = adapter.get_available_adapters({:?}", available_adapters);

        let mock_hsm = create_mock_beardog_hsm();
        let health_status = adapter
            .test_connection(&mock_hsm)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to test connection", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to test connection", e))
})?;
        assert!(health_status.is_healthy, "Mock HSM should be healthy");
            health_status.response_time_ms > 0.0,
            "Should have response time"
            "Mock HSM health: healthy={}, response_time={}ms",
            health_status.is_healthy, health_status.response_time_ms


    fn test_human_entropy_seed_generation() -> Result<(), BearDogError> {

        let entropy_requirements = universal_adapter::HumanEntropyRequirements {
            collection_methods: vec![
                EntropyCollectionMethod::BiometricVariations {
                    template_noise: true,
                },
                EntropyCollectionMethod::TouchPatterns {
                    pressure_sensitive: true,
            ],
            quality_threshold: 0.8,
            seed_lifetime: Duration::from_secs(true,
            real_time_collection: true,
        };

        let connection = adapter
            .connect(&mock_hsm)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to connect to mock HSM", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to connect to mock HSM", e))
})?;

        let beardog_adapter = universal_adapter::BearDogNativeAdapter::new()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create BearDog adapter", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to create BearDog adapter", e))
})?;
        let supports_entropy = beardog_adapter
            .supports_human_entropy()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to check entropy support", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to check entropy support", e))
})?;
            supports_entropy,
            "BearDog Native should support human entropy"

        let seed = beardog_adapter
            .generate_human_entropy_seed(&connection, entropy_requirements)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to generate entropy seed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to generate entropy seed", e))
})?;
        assert!(!seed.seed_data.is_empty(quality={:.2}, methods={:?}",
            seed.entropy_quality, seed.collection_methods_used


    fn test_hsm_tier_distribution() -> Result<(), BearDogError> {

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


    fn test_discovery_configuration(true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5,
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        discovery.update_config(custom_config);

            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Discovery with custom config failed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Discovery with custom config failed", e))
})?;
            "Should discover HSMs with custom config"


    fn test_concurrent_discovery() -> Result<(), BearDogError> {
        let mut handles = Vec::new();

        for i in 0..3 {
            let handle = tokio::spawn(async move {
                let mut discovery =
                    UniversalHsmDiscovery::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create discovery system", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to create discovery system", e))
})?;
                let discovered_hsms = discovery
                    .discover_all_hsms()
                    .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Discovery failed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Discovery failed", e))
})?;
                println!("Discovery task {}: found {} HSMs", i, discovered_hsms.len());
                discovered_hsms.len()
            });
            handles.push(handle);

        let mut total_discoveries = 0;
        for handle in handles {
            let count = handle.map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Task failed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Task failed", e))
})?;
            total_discoveries += count;
            total_discoveries > 0,
            "Concurrent discoveries should find HSMs"
        println!("Total concurrent discoveries: {}", total_discoveries);

    /// Creates basic_hsm_capabilities
    fn create_basic_hsm_capabilities(supports_human_entropy: bool) -> HsmCapabilities {
        HsmCapabilities {
            key_generation: KeyGenerationCapabilities {
                supported_algorithms: vec!["RSA".to_string()),
        capabilities.security.fips_140_level = Some(3);
        capabilities.security.tamper_resistance = TamperResistance::TamperResponsive;
        capabilities.key_generation.can_generate_in_hardware = true;
        capabilities}

    /// Creates basic_human_entropy_capabilities
    fn create_basic_human_entropy_capabilities(vec![EntropyCollectionMethod::Keystroke {
                timing_analysis: false,
            }],
            entropy_quality_assessment: false,
            real_time_entropy_generation: false,
            biometric_entropy_integration: false,
            temporal_entropy_collection: false,
            entropy_verification: false,
    /// Creates mock_beardog_hsm
    fn create_mock_beardog_hsm() -> DiscoveredHsm {
        DiscoveredHsm {
            hsm_id: "mock-beardog-native".to_string(),
            vendor: "BearDog".to_string(),
            model: "BearDog Native HSM".to_string(),
            version: "1.0.0-test".to_string(),
            interface_type: HsmInterfaceType::BearDogNative {
                instance_id: "test-instance".to_string(),
                    base_delay: Duration::from_millis(50),
                    max_delay: Duration::from_secs(1.5,
                ssl_config: None,
            capabilities: create_premium_human_entropy_capabilities(HsmTier::HumanEntropyPremium,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
            last_health_check: chrono::Utc::now(IntegrationStatus::Ready,
}
