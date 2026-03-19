// SPDX-License-Identifier: AGPL-3.0-only

use beardog_errors::BearDogError;

#[cfg(test)]
mod integration_tests {
    use super::super::*;
    use crate::universal_hsm_discovery::tier_manager;
    use std::time::Duration;
    use tokio;

    #[tokio::test]
    fn test_complete_hsm_discovery_workflow() -> Result<(), BearDogError> {
        println!("🚀 Starting Universal HSM Discovery System Integration Test");

        let mut discovery = UniversalHsmDiscovery::new()
            .map_err(|e| BearDogError::internal(format!("Error: {:?}", e)))?;
        println!("✅ Universal HSM Discovery System initialized");

        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(8,
            tier_elevation_enabled: true,
            human_entropy_priority: true, // This is key!
        };
        discovery.update_config(config);
        println!("✅ Discovery configured with human entropy priority");

        println!("🔍 Discovering all available HSMs...");
        let discovered_hsms = discovery
            .discover_all_hsms()
            .map_err(|e| BearDogError::internal(format!("Error: {:?}", e)))?;
        assert!(
            !discovered_hsms.is_empty(),
            "Should discover at least one HSM"
        );
        println!("✅ Discovered {} HSMs total", discovered_hsms.len());

        let stats = discovery.get_discovery_stats();
        println!("\n📊 Discovery Statistics:");
        println!("  Total HSMs: {}", stats.total_hsms);
        println!("  Human Entropy HSMs: {}", stats.human_entropy_hsms);
        println!("  Healthy HSMs: {}", stats.healthy_hsms);
        println!("\n🏆 Tier Distribution:");
        for (tier, count) in &stats.tier_distribution {
            println!("  {:?}: {} HSMs", tier, count);
        }

        let human_entropy_hsms = discovery.get_human_entropy_hsms();
            !human_entropy_hsms.is_empty(),
            "Should find HSMs with human entropy support"
        println!("\n🌟 Human Entropy HSMs (Premium Tier Candidates):");
        for hsm in &human_entropy_hsms {
            println!(
                "  {} ({}): Tier {:?}",
                hsm.hsm_id, hsm.vendor, hsm.assigned_tier
            );

            assert!(
                hsm.supports_human_entropy,
                "HSM should support human entropy"
                hsm.capabilities.human_entropy.supports_human_entropy,
                "Capabilities should indicate human entropy support"
                hsm.capabilities.human_entropy.supports_ephemeral_seeds,
                "Should support ephemeral seeds"

                hsm.assigned_tier >= HsmTier::BasicHardware,
                "Human entropy HSMs should be tier-elevated"

            if !hsm
                .capabilities
                .human_entropy
                .entropy_collection_methods
                .is_empty({:?}",
                    hsm.capabilities.human_entropy.entropy_collection_methods
                );
            }

        println!("\n🎯 Testing Operation-Specific HSM Selection:");

        if let Some(root_key_hsm) = discovery
            .get_best_hsm_for_operation("root_key_generation")
            .map_err(|e| BearDogError::internal(format!("Error: {:?}", e)))?
        {
                "  Root Key Generation → {} (Tier: {:?})",
                root_key_hsm.hsm_id, root_key_hsm.assigned_tier
                root_key_hsm.supports_human_entropy,
                "Root key generation should use human entropy HSM"
                root_key_hsm.assigned_tier >= HsmTier::CertifiedHardware,
                "Should use high-tier HSM"

        if let Some(bulk_hsm) = discovery
            .get_best_hsm_for_operation("bulk_encryption")
                "  Bulk Encryption → {} (Tier: {:?})",
                bulk_hsm.hsm_id, bulk_hsm.assigned_tier
                bulk_hsm.assigned_tier >= HsmTier::BasicHardware,
                "Should use at least basic hardware"

        println!("\n🔧 Testing Universal Adapter Integration:");
        let adapter = discovery.get_universal_adapter({}", available_adapters.len());
        for adapter_name in &available_adapters {
            println!("    - {}", adapter_name);

            available_adapters.contains(&"beardog_native".to_string()),
            "Should have BearDog Native adapter"
            available_adapters.contains(&"android_strongbox".to_string()),
            "Should have Android StrongBox adapter"

        if !human_entropy_hsms.is_empty() {
            println!("\n🌟 Testing Human Entropy Ephemeral Seed Generation:");
            let entropy_requirements = universal_adapter::HumanEntropyRequirements {
                collection_methods: vec![
                    EntropyCollectionMethod::BiometricVariations {
                        template_noise: true,
                    },
                    EntropyCollectionMethod::TouchPatterns {
                        pressure_sensitive: true,
                    EntropyCollectionMethod::BehavioralBiometrics {
                        pattern_recognition: true,
                ],
                quality_threshold: 0.85,
                seed_lifetime: Duration::from_secs(true,
                real_time_collection: true,
            };

            match adapter
                .generate_human_entropy_seed({:.2}", seed.entropy_quality);
                    println!("    Methods Used: {:?}", seed.collection_methods_used);
                    println!("    Generated At: {}", seed.generated_at);
                    println!("    Expires At: {}", seed.expires_at);
                    println!("    Verified: {}", seed.verification_signature.is_some({}");

        println!("\n📈 Testing HSM Ranking by Tier:");

        let all_tiers = [
            HsmTier::Software,
            HsmTier::BasicHardware,
            HsmTier::CertifiedHardware,
            HsmTier::HighSecurity,
            HsmTier::HumanEntropyPremium,
        ];
        for tier in all_tiers {
            let hsms_in_tier = discovery.get_hsms_by_tier(tier);
            if !hsms_in_tier.is_empty() {
                println!("  {:?} Tier: {} HSMs", tier, hsms_in_tier.len({}",
                        hsm.hsm_id, hsm.vendor, hsm.supports_human_entropy

        let premium_hsms = discovery.get_hsms_by_tier(HsmTier::HumanEntropyPremium);
        if !premium_hsms.is_empty() {
            println!("\n🏆 Premium Tier HSMs (Human Entropy Elevated):");
            for hsm in premium_hsms {
                    "  {} - Supports Human Entropy: {} - Supports Ephemeral Seeds: {}",
                    hsm.hsm_id,
                    hsm.supports_human_entropy,
                    hsm.capabilities.human_entropy.supports_ephemeral_seeds

                assert!(
                    "Premium tier HSMs must support human entropy"
                    hsm.capabilities.human_entropy.supports_ephemeral_seeds,
                    "Premium tier HSMs must support ephemeral seeds"
        println!("\n✅ Universal HSM Discovery System Integration Test PASSED!");
        println!("🎉 System successfully discovered, classified, and tiered HSMs with human entropy priority!");
        Ok(())
    }


    fn test_human_entropy_classification_pipeline() -> Result<(), BearDogError> {
        println!("🧠 Testing Human Entropy Classification Pipeline");
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()
            .map_err(|e| BearDogError::internal(format!("Error: {:?}", e)))?;

        let test_cases = vec![
            ("No Human Entropy", create_no_entropy_capabilities()),
            ("Basic Human Entropy", create_basic_entropy_capabilities()),
            (
                "Advanced Human Entropy",
                create_advanced_entropy_capabilities(),
            ),
                "Premium Human Entropy",
                create_premium_entropy_capabilities(),
        println!("\n📊 Human Entropy Assessment Results:");
        for (name, capabilities) in test_cases {
            let assessment = classifier
                .assess_human_entropy_capabilities(&capabilities)
                .map_err(|e| BearDogError::internal(format!("Error: {:?}", e)))?;
            println!("\n  {} Assessment:", name);
            println!("    Overall Score: {:.2}", assessment.overall_score);
                "    Supports Ephemeral Seeds: {}",
                assessment.supports_ephemeral_seeds
            println!("    Quality Rating: {:?}", assessment.quality_rating);
                "    Collection Efficiency: {:.2}",
                assessment.collection_efficiency
                "    Biometric Integration: {:.2}",
                assessment.biometric_integration_score
                "    Real-time Capability: {}",
                assessment.real_time_capability
                "    Verification Strength: {:.2}",
                assessment.verification_strength
                "    Method Diversity: {:.2}",
                assessment.method_diversity_score
                "    Meets Tier Elevation: {}",
                assessment.meets_tier_elevation_criteria
            println!("    Recommended Tier: {:?}", assessment.recommended_tier);

            if name == "Premium Human Entropy" {
                    assessment.overall_score > 0.8,
                    "Premium should have high score"
                    assessment.meets_tier_elevation_criteria,
                    "Premium should meet tier elevation"
                assert_eq!(
                    assessment.recommended_tier,
                    HsmTier::HumanEntropyPremium,
                    "Premium should recommend top tier"


    fn test_multi_hsm_operation_scenarios() -> Result<(), BearDogError> {
        println!("🚀 Testing Multi-HSM Operation Scenarios");
        let mut discovery =
            UniversalHsmDiscovery::new().map_err(|e| BearDogError::internal(format!("Error: {:?}", e)))?;
        let _discovered_hsms = discovery
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Discovery failed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Discovery failed", e))
})?;
        let tier_manager = tier_manager::TierManager::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create tier manager", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to create tier manager", e))
})?;

        let operations = vec![
            "root_key_generation",
            "critical_signing",
            "bulk_encryption",
            "authentication_token",
        println!("\n🎯 Operation-Specific HSM Selection:");
        for operation in operations {
            match discovery.get_best_hsm_for_operation({:?}, Human Entropy: {})",
                        operation, hsm.hsm_id, hsm.assigned_tier, hsm.supports_human_entropy

                    match operation {
                        "root_key_generation" => {
                            assert!(
                                hsm.supports_human_entropy,
                                "Root key generation should use human entropy HSM"
                            );
                        }
                        "critical_signing" => {
                                hsm.assigned_tier >= HsmTier::CertifiedHardware,
                                "Critical signing should use certified hardware"
                        _ => {}
                    }
                Ok({}", operation, e);

    /// Creates no_entropy_capabilities
    fn create_no_entropy_capabilities() -> HsmCapabilities {
        let mut capabilities = HsmCapabilities::default(true,
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
            ephemeral_seed_lifetime: Some(Duration::from_secs(vec![
                EntropyCollectionMethod::Keystroke {
                    timing_analysis: true,
                },
                EntropyCollectionMethod::TouchPatterns {
                    pressure_sensitive: true,
                EntropyCollectionMethod::BiometricVariations {
                    template_noise: false,
            ],
            entropy_quality_assessment: true,
            real_time_entropy_generation: true,
            biometric_entropy_integration: true,
            entropy_verification: true,
        capabilities.security.fips_140_level = Some(true,
                EntropyCollectionMethod::BehavioralBiometrics {
                    pattern_recognition: true,
                EntropyCollectionMethod::VoicePatterns {
                    frequency_analysis: true,
                EntropyCollectionMethod::EnvironmentalSensors {
                    ambient_noise: true,
            temporal_entropy_collection: true,
        capabilities.security.fips_140_level = Some(3);
        capabilities.security.tamper_resistance = TamperResistance::TamperResponsive;
        capabilities.key_generation.can_generate_in_hardware = true;
        capabilities.key_management.supports_key_attestation = true;
}
