//! # Migration Module
//!
//! Handles migrations and upgrades across BearDog systems, including the revolutionary
//! migration to human-owned entropy and sovereign key management.
//!
//! ## Overview
//!
//! This module provides comprehensive migration capabilities for transitioning between
//! different BearDog system configurations, particularly focusing on migrating from
//! traditional corporate-controlled entropy sources to human-owned, sovereign entropy.
//!
//! ## Key Components
//!
//! - `SovereignEntropyMigrationManager` - Manages the migration process
//! - `MigrationPhase` - Tracks current phase of migration
//! - `MigrationStatistics` - Provides migration progress metrics
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::migration::{SovereignEntropyMigrationManager, SovereignEntropyMigrationConfig};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let config = SovereignEntropyMigrationConfig::default();
//! let manager = SovereignEntropyMigrationManager::new(config)?;
//!
//! // Execute migration
//! let stats = manager.execute_migration().await?;
//! println!("Migration progress: {:.1}%", stats.progress_percent());
//! # Ok(())
//! # }
//! ```

/// Sovereign entropy migration system
///
/// Provides tools and functionality for migrating from corporate-controlled
/// entropy sources to human-owned, sovereign entropy generation.
pub mod sovereign_entropy_migration;

pub use sovereign_entropy_migration::{
    MigrationPhase, MigrationStatistics, SovereignEntropyMigrationConfig,
    SovereignEntropyMigrationManager,
};

#[cfg(test)]
mod tests;
