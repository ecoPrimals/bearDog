// Tests for sovereign entropy migration module
//
// Comprehensive tests for migration configuration, phases, statistics,
// and migration manager behavior.

use super::sovereign_entropy_migration::{
    MigrationPhase, MigrationStatistics, SovereignEntropyMigrationConfig,
};
use std::collections::HashMap;

// =============================================================================
// SovereignEntropyMigrationConfig Tests
// =============================================================================

#[test]
fn test_migration_config_default() {
    let config = SovereignEntropyMigrationConfig::default();

    assert!(
        config.enable_migration,
        "Migration should be enabled by default"
    );
    assert_eq!(config.default_system_identity, "beardog_system");
    assert!(config.enable_rollback, "Rollback should be enabled");
    assert!(
        !config.enabled_phases.is_empty(),
        "Should have default phases"
    );
    assert!(
        !config.operation_tier_requirements.is_empty(),
        "Should have tier requirements"
    );
}

#[test]
fn test_migration_config_default_phases() {
    let config = SovereignEntropyMigrationConfig::default();

    assert!(config
        .enabled_phases
        .contains(&MigrationPhase::CryptographicKeys));
    assert!(config
        .enabled_phases
        .contains(&MigrationPhase::NeuralNetworkWeights));
    assert!(config
        .enabled_phases
        .contains(&MigrationPhase::RandomDataGeneration));
    assert_eq!(
        config.enabled_phases.len(),
        3,
        "Should have 3 default phases"
    );
}

#[test]
fn test_migration_config_tier_requirements() {
    let config = SovereignEntropyMigrationConfig::default();

    // High security operations (tier 3)
    assert_eq!(
        config.operation_tier_requirements.get("key_generation"),
        Some(&3)
    );
    assert_eq!(
        config
            .operation_tier_requirements
            .get("signature_generation"),
        Some(&3)
    );
    assert_eq!(
        config.operation_tier_requirements.get("encryption_keys"),
        Some(&3)
    );

    // Supervised operations (tier 2)
    assert_eq!(
        config.operation_tier_requirements.get("neural_weights"),
        Some(&2)
    );
    assert_eq!(
        config.operation_tier_requirements.get("ai_training"),
        Some(&2)
    );
    assert_eq!(
        config.operation_tier_requirements.get("nonce_generation"),
        Some(&2)
    );
    assert_eq!(
        config.operation_tier_requirements.get("salt_generation"),
        Some(&2)
    );

    // Lower security operations (tier 1)
    assert_eq!(
        config.operation_tier_requirements.get("uuid_generation"),
        Some(&1)
    );
}

#[test]
fn test_migration_config_custom_configuration() {
    let mut tier_requirements = HashMap::new();
    tier_requirements.insert("custom_operation".to_string(), 4);

    let config = SovereignEntropyMigrationConfig {
        enable_migration: false,
        default_system_identity: "custom_system".to_string(),
        operation_tier_requirements: tier_requirements.clone(),
        enable_rollback: false,
        enabled_phases: vec![MigrationPhase::GeneticOperations],
    };

    assert!(!config.enable_migration);
    assert_eq!(config.default_system_identity, "custom_system");
    assert!(!config.enable_rollback);
    assert_eq!(config.enabled_phases.len(), 1);
    assert_eq!(
        config.operation_tier_requirements.get("custom_operation"),
        Some(&4)
    );
}

#[test]
fn test_migration_config_serialization() {
    let config = SovereignEntropyMigrationConfig::default();

    let serialized = serde_json::to_string(&config).expect("Should serialize");
    let deserialized: SovereignEntropyMigrationConfig =
        serde_json::from_str(&serialized).expect("Should deserialize");

    assert_eq!(config.enable_migration, deserialized.enable_migration);
    assert_eq!(
        config.default_system_identity,
        deserialized.default_system_identity
    );
    assert_eq!(config.enable_rollback, deserialized.enable_rollback);
    assert_eq!(
        config.enabled_phases.len(),
        deserialized.enabled_phases.len()
    );
}

// =============================================================================
// MigrationPhase Tests
// =============================================================================

#[test]
fn test_migration_phase_equality() {
    assert_eq!(
        MigrationPhase::CryptographicKeys,
        MigrationPhase::CryptographicKeys
    );
    assert_ne!(
        MigrationPhase::CryptographicKeys,
        MigrationPhase::NeuralNetworkWeights
    );
}

#[test]
fn test_migration_phase_all_variants() {
    let phases = vec![
        MigrationPhase::CryptographicKeys,
        MigrationPhase::NeuralNetworkWeights,
        MigrationPhase::RandomDataGeneration,
        MigrationPhase::GeneticOperations,
        MigrationPhase::UniversalAdapterOperations,
        MigrationPhase::FullEcosystemMigration,
    ];

    assert_eq!(phases.len(), 6, "Should have all 6 migration phases");
}

#[test]
fn test_migration_phase_serialization() {
    let phase = MigrationPhase::CryptographicKeys;

    let serialized = serde_json::to_string(&phase).expect("Should serialize");
    let deserialized: MigrationPhase =
        serde_json::from_str(&serialized).expect("Should deserialize");

    assert_eq!(phase, deserialized);
}

#[test]
fn test_migration_phase_clone() {
    let phase = MigrationPhase::NeuralNetworkWeights;
    let cloned = phase.clone();

    assert_eq!(phase, cloned);
}

// =============================================================================
// MigrationStatistics Tests
// =============================================================================

#[test]
fn test_migration_statistics_default() {
    let stats = MigrationStatistics::default();

    assert_eq!(stats.total_calls_migrated, 0);
    assert!(stats.calls_by_tier.is_empty());
    assert_eq!(stats.success_rate, 0.0);
    assert_eq!(stats.average_quality_score, 0.0);
    assert_eq!(stats.active_human_identities, 0);
    assert_eq!(stats.cross_primal_sharing_events, 0);
}

#[test]
fn test_migration_statistics_with_data() {
    let mut calls_by_tier = HashMap::new();
    calls_by_tier.insert(1, 100);
    calls_by_tier.insert(2, 250);
    calls_by_tier.insert(3, 50);

    let stats = MigrationStatistics {
        total_calls_migrated: 400,
        calls_by_tier,
        success_rate: 0.98,
        average_quality_score: 0.95,
        active_human_identities: 25,
        cross_primal_sharing_events: 150,
    };

    assert_eq!(stats.total_calls_migrated, 400);
    assert_eq!(stats.calls_by_tier.get(&1), Some(&100));
    assert_eq!(stats.calls_by_tier.get(&2), Some(&250));
    assert_eq!(stats.calls_by_tier.get(&3), Some(&50));
    assert_eq!(stats.success_rate, 0.98);
    assert_eq!(stats.average_quality_score, 0.95);
    assert_eq!(stats.active_human_identities, 25);
    assert_eq!(stats.cross_primal_sharing_events, 150);
}

#[test]
fn test_migration_statistics_serialization() {
    let mut stats = MigrationStatistics::default();
    stats.total_calls_migrated = 100;
    stats.success_rate = 0.95;

    let serialized = serde_json::to_string(&stats).expect("Should serialize");
    let deserialized: MigrationStatistics =
        serde_json::from_str(&serialized).expect("Should deserialize");

    assert_eq!(
        stats.total_calls_migrated,
        deserialized.total_calls_migrated
    );
    assert_eq!(stats.success_rate, deserialized.success_rate);
}

#[test]
fn test_migration_statistics_clone() {
    let stats = MigrationStatistics {
        total_calls_migrated: 500,
        calls_by_tier: HashMap::new(),
        success_rate: 0.99,
        average_quality_score: 0.92,
        active_human_identities: 10,
        cross_primal_sharing_events: 75,
    };

    let cloned = stats.clone();

    assert_eq!(stats.total_calls_migrated, cloned.total_calls_migrated);
    assert_eq!(stats.success_rate, cloned.success_rate);
    assert_eq!(stats.average_quality_score, cloned.average_quality_score);
}

#[test]
fn test_migration_statistics_tier_distribution() {
    let mut calls_by_tier = HashMap::new();
    calls_by_tier.insert(1, 50); // 25% tier 1 (low security)
    calls_by_tier.insert(2, 100); // 50% tier 2 (supervised)
    calls_by_tier.insert(3, 50); // 25% tier 3 (high security)

    let stats = MigrationStatistics {
        total_calls_migrated: 200,
        calls_by_tier,
        success_rate: 1.0,
        average_quality_score: 0.90,
        active_human_identities: 5,
        cross_primal_sharing_events: 20,
    };

    // Verify tier distribution
    let tier1 = stats.calls_by_tier.get(&1).copied().unwrap_or(0);
    let tier2 = stats.calls_by_tier.get(&2).copied().unwrap_or(0);
    let tier3 = stats.calls_by_tier.get(&3).copied().unwrap_or(0);

    assert_eq!(tier1 + tier2 + tier3, stats.total_calls_migrated);
    assert!(tier2 > tier1, "Most calls should be supervised (tier 2)");
    assert!(tier2 > tier3, "More supervised calls than high-security");
}

#[test]
fn test_migration_statistics_perfect_success() {
    let stats = MigrationStatistics {
        total_calls_migrated: 1000,
        calls_by_tier: HashMap::new(),
        success_rate: 1.0,
        average_quality_score: 1.0,
        active_human_identities: 50,
        cross_primal_sharing_events: 500,
    };

    assert_eq!(stats.success_rate, 1.0, "Perfect success rate");
    assert_eq!(stats.average_quality_score, 1.0, "Perfect quality score");
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_migration_workflow_simulation() {
    // Simulate a typical migration workflow
    let config = SovereignEntropyMigrationConfig::default();

    // Phase 1: Verify configuration
    assert!(config.enable_migration);
    assert!(config.enable_rollback);

    // Phase 2: Check enabled phases
    let enabled_phases = &config.enabled_phases;
    assert!(enabled_phases.contains(&MigrationPhase::CryptographicKeys));

    // Phase 3: Verify tier requirements
    let key_gen_tier = config.operation_tier_requirements.get("key_generation");
    assert_eq!(key_gen_tier, Some(&3), "Key generation requires tier 3");

    // Phase 4: Initialize statistics tracking
    let mut stats = MigrationStatistics::default();
    stats.total_calls_migrated = 100;
    stats.success_rate = 0.98;

    assert!(stats.success_rate > 0.95, "High success rate expected");
}

#[test]
fn test_progressive_phase_enablement() {
    let mut phases_enabled = vec![MigrationPhase::CryptographicKeys];
    assert_eq!(phases_enabled.len(), 1);

    // Gradually enable more phases
    phases_enabled.push(MigrationPhase::NeuralNetworkWeights);
    assert_eq!(phases_enabled.len(), 2);

    phases_enabled.push(MigrationPhase::RandomDataGeneration);
    assert_eq!(phases_enabled.len(), 3);

    phases_enabled.push(MigrationPhase::GeneticOperations);
    assert_eq!(phases_enabled.len(), 4);

    phases_enabled.push(MigrationPhase::UniversalAdapterOperations);
    assert_eq!(phases_enabled.len(), 5);

    phases_enabled.push(MigrationPhase::FullEcosystemMigration);
    assert_eq!(phases_enabled.len(), 6, "All phases enabled");
}

#[test]
fn test_tier_requirement_validation() {
    let config = SovereignEntropyMigrationConfig::default();

    // Validate that all tier requirements are between 1 and 4
    for (operation, tier) in &config.operation_tier_requirements {
        assert!(
            *tier >= 1 && *tier <= 4,
            "Tier for {} should be 1-4, got {}",
            operation,
            tier
        );
    }
}
