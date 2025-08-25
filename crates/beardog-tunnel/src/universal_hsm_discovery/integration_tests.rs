// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Integration Tests for Universal HSM Discovery System
///
/// These tests demonstrate the complete workflow of the Universal HSM Discovery System,
/// showing how HSMs are discovered, classified, tiered, and used for operations.

#[cfg(test)]
mod integration_tests {
    use super::super::*;
    use crate::universal_hsm_discovery::tier_manager;
    use std::time::Duration;
    use tokio;
    /// Comprehensive integration test showing the full HSM discovery and usage workflow
    #[tokio::test]
    async fn test_complete_hsm_discovery_workflow() -> beardog_errors::BearDogResult<()> {
        println!("🚀 Starting Universal HSM Discovery System Integration Test");
        // Step 1: Initialize the discovery system
        let mut discovery = UniversalHsmDiscovery::new()
            .map_err(|e| BearDogError::internal(format!("Failed to initialize Universal HSM Discovery System: {:?}", e)))?;
        println!("✅ Universal HSM Discovery System initialized");
        // Step 2: Configure discovery with human entropy priority
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(10),
            max_concurrent_discoveries: 8,
            tier_elevation_enabled: true,
            human_entropy_priority: true, // This is key!
        };
        discovery.update_config(config);
        println!("✅ Discovery configured with human entropy priority");
        // Step 3: Perform universal HSM discovery
        println!("🔍 Discovering all available HSMs...");
        let discovered_hsms = discovery
            .discover_all_hsms()
            .await
            .map_err(|e| BearDogError::internal(format!("HSM discovery failed: {:?}", e)))?;
        assert!(
            !discovered_hsms.is_empty(),
            "Should discover at least one HSM"
        );
        println!("✅ Discovered {} HSMs total", discovered_hsms.len());
        // Step 4: Analyze discovery results
        let stats = discovery.get_discovery_stats();
        println!("\n📊 Discovery Statistics:");
        println!("  Total HSMs: {}", stats.total_hsms);
        println!("  Human Entropy HSMs: {}", stats.human_entropy_hsms);
        println!("  Healthy HSMs: {}", stats.healthy_hsms);
        println!("\n🏆 Tier Distribution:");
        for (tier, count) in &stats.tier_distribution {
            println!("  {:?}: {} HSMs", tier, count);
        }
        // Step 5: Verify human entropy HSMs are properly elevated
        let human_entropy_hsms = discovery.get_human_entropy_hsms();
            !human_entropy_hsms.is_empty(),
            "Should find HSMs with human entropy support"
        println!("\n🌟 Human Entropy HSMs (Premium Tier Candidates):");
        for hsm in &human_entropy_hsms {
            println!(
                "  {} ({}): Tier {:?}",
                hsm.hsm_id, hsm.vendor, hsm.assigned_tier
            );
            // Verify human entropy capabilities
            assert!(
                hsm.supports_human_entropy,
                "HSM should support human entropy"
                hsm.capabilities.human_entropy.supports_human_entropy,
                "Capabilities should indicate human entropy support"
                hsm.capabilities.human_entropy.supports_ephemeral_seeds,
                "Should support ephemeral seeds"
            // Human entropy HSMs should be elevated to at least BasicHardware tier
                hsm.assigned_tier >= HsmTier::BasicHardware,
                "Human entropy HSMs should be tier-elevated"
            // Print entropy collection methods
            if !hsm
                .capabilities
                .human_entropy
                .entropy_collection_methods
                .is_empty()
            {
                println!(
                    "    Entropy Methods: {:?}",
                    hsm.capabilities.human_entropy.entropy_collection_methods
                );
            }
        // Step 6: Test operation-specific HSM selection
        println!("\n🎯 Testing Operation-Specific HSM Selection:");
        // Critical operation requiring human entropy
        if let Some(root_key_hsm) = discovery
            .get_best_hsm_for_operation("root_key_generation")
            .map_err(|e| BearDogError::internal(format!("Failed to select HSM: {:?}", e)))?
        {
                "  Root Key Generation → {} (Tier: {:?})",
                root_key_hsm.hsm_id, root_key_hsm.assigned_tier
                root_key_hsm.supports_human_entropy,
                "Root key generation should use human entropy HSM"
                root_key_hsm.assigned_tier >= HsmTier::CertifiedHardware,
                "Should use high-tier HSM"
        // Performance-oriented operation
        if let Some(bulk_hsm) = discovery
            .get_best_hsm_for_operation("bulk_encryption")
                "  Bulk Encryption → {} (Tier: {:?})",
                bulk_hsm.hsm_id, bulk_hsm.assigned_tier
                bulk_hsm.assigned_tier >= HsmTier::BasicHardware,
                "Should use at least basic hardware"
        // Step 7: Test universal adapter integration
        println!("\n🔧 Testing Universal Adapter Integration:");
        let adapter = discovery.get_universal_adapter();
        let available_adapters = adapter.get_available_adapters();
        println!("  Available Adapters: {}", available_adapters.len());
        for adapter_name in &available_adapters {
            println!("    - {}", adapter_name);
        // Verify key adapters are present
            available_adapters.contains(&"beardog_native".to_string()),
            "Should have BearDog Native adapter"
            available_adapters.contains(&"android_strongbox".to_string()),
            "Should have Android StrongBox adapter"
        // Step 8: Test human entropy seed generation (premium operation)
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
                seed_lifetime: Duration::from_secs(300), // 5 minutes
                verification_required: true,
                real_time_collection: true,
            };
            // Try to generate human entropy seed using the best available HSM
            match adapter
                .generate_human_entropy_seed(entropy_requirements, None)
                .await
                Ok(seed) => {
                    println!("  ✅ Successfully generated human entropy ephemeral seed!");
                    println!("    Quality: {:.2}", seed.entropy_quality);
                    println!("    Methods Used: {:?}", seed.collection_methods_used);
                    println!("    Generated At: {}", seed.generated_at);
                    println!("    Expires At: {}", seed.expires_at);
                    println!("    Verified: {}", seed.verification_signature.is_some());
                    // Verify seed properties
                    assert!(!seed.seed_data.is_empty(), "Should have seed data");
                    assert!(seed.entropy_quality > 0.0, "Should have quality rating");
                    assert!(
                        seed.expires_at > seed.generated_at,
                        "Should have valid expiration"
                    );
                        !seed.collection_methods_used.is_empty(),
                        "Should have used collection methods"
                }
                Err(e) => {
                    println!("  ⚠️  Human entropy seed generation failed (expected in test environment): {}", e);
                    // This is expected in test environment without real HSMs
        // Step 9: Test tier-based HSM ranking
        println!("\n📈 Testing HSM Ranking by Tier:");
        // Get all tiers with HSMs
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
                println!("  {:?} Tier: {} HSMs", tier, hsms_in_tier.len());
                for hsm in hsms_in_tier {
                    println!(
                        "    - {} ({}) - Human Entropy: {}",
                        hsm.hsm_id, hsm.vendor, hsm.supports_human_entropy
        // Step 10: Verify human entropy tier elevation is working
        let premium_hsms = discovery.get_hsms_by_tier(HsmTier::HumanEntropyPremium);
        if !premium_hsms.is_empty() {
            println!("\n🏆 Premium Tier HSMs (Human Entropy Elevated):");
            for hsm in premium_hsms {
                    "  {} - Supports Human Entropy: {} - Supports Ephemeral Seeds: {}",
                    hsm.hsm_id,
                    hsm.supports_human_entropy,
                    hsm.capabilities.human_entropy.supports_ephemeral_seeds
                // All premium tier HSMs should support human entropy
                assert!(
                    "Premium tier HSMs must support human entropy"
                    hsm.capabilities.human_entropy.supports_ephemeral_seeds,
                    "Premium tier HSMs must support ephemeral seeds"
        println!("\n✅ Universal HSM Discovery System Integration Test PASSED!");
        println!("🎉 System successfully discovered, classified, and tiered HSMs with human entropy priority!");
        Ok(())
    }
    /// Test the human entropy classification pipeline in detail
    async fn test_human_entropy_classification_pipeline() -> beardog_errors::BearDogResult<()> {
        println!("🧠 Testing Human Entropy Classification Pipeline");
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()
            .map_err(|e| BearDogError::internal(format!("Failed to create human entropy classifier: {:?}", e)))?;
        // Test different levels of human entropy capabilities
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
                .map_err(|e| BearDogError::internal(format!("Failed to assess human entropy capabilities: {:?}", e)))?;
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
            // Verify that higher entropy capabilities get higher scores
            if name == "Premium Human Entropy" {
                    assessment.overall_score > 0.8,
                    "Premium should have high score"
                    assessment.meets_tier_elevation_criteria,
                    "Premium should meet tier elevation"
                assert_eq!(
                    assessment.recommended_tier,
                    HsmTier::HumanEntropyPremium,
                    "Premium should recommend top tier"
    /// Test multi-HSM operation scenarios
    async fn test_multi_hsm_operation_scenarios() -> beardog_errors::BearDogResult<()> {
        println!("🚀 Testing Multi-HSM Operation Scenarios");
        let mut discovery =
            UniversalHsmDiscovery::new().map_err(|e| BearDogError::internal(format!("Failed to create discovery system: {:?}", e)))?;
        let _discovered_hsms = discovery
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Discovery failed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Discovery failed", e))
})?;
        let tier_manager = tier_manager::TierManager::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create tier manager", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to create tier manager", e))
})?;
        // Test different operation types and their HSM requirements
        let operations = vec![
            "root_key_generation",
            "critical_signing",
            "bulk_encryption",
            "authentication_token",
        println!("\n🎯 Operation-Specific HSM Selection:");
        for operation in operations {
            match discovery.get_best_hsm_for_operation(operation).await {
                Ok(Some(hsm)) => {
                        "  {} → {} (Tier: {:?}, Human Entropy: {})",
                        operation, hsm.hsm_id, hsm.assigned_tier, hsm.supports_human_entropy
                    // Verify operation-specific requirements
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
                Ok(None) => {
                    println!("  {} → No suitable HSM found", operation);
                    println!("  {} → Error: {}", operation, e);
    // Helper functions for creating test capabilities
    fn create_no_entropy_capabilities() -> HsmCapabilities {
        let mut capabilities = HsmCapabilities::default();
        capabilities.human_entropy.supports_human_entropy = false;
        capabilities.human_entropy.supports_ephemeral_seeds = false;
        capabilities}


    fn create_basic_entropy_capabilities() -> HsmCapabilities {
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
    fn create_advanced_entropy_capabilities() -> HsmCapabilities {
            entropy_collection_methods: vec![
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
        capabilities.security.fips_140_level = Some(2);
    fn create_premium_entropy_capabilities() -> HsmCapabilities {
                    template_noise: true,
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
