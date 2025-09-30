// Migration Module
//
// This module handles migrations and upgrades across BearDog systems,
// including the revolutionary migration to human-owned entropy.

/// Sovereign entropy migration system
pub mod sovereign_entropy_migration;

pub use sovereign_entropy_migration::{
    MigrationPhase, MigrationStatistics, SovereignEntropyMigrationConfig,
    SovereignEntropyMigrationManager,
};
