//! Genetic Spawning Module
//!
//! This module provides the complete genetic spawning system with multi-party
//! workflow processing, genetic recombination, and lineage tracking.
//!
//! ## Architecture
//!
//! The genetic spawning system is composed of several focused modules:
//!
//! - `engine`: Core spawning engine and orchestration
//! - `workflows`: Approval workflow processing (consensus, human, hybrid)
//! - `recombination`: Genetic recombination algorithms
//! - `evolution`: Directed evolution and mutation systems
//! - `validation`: Risk assessment and automated validation
//! - `lineage`: Genetic lineage tracking and diversity calculation
//!
//! ## Usage
//!
//! ```rust
//! use crate::genetics::spawning::{GeneticSpawningEngine, GeneticsConfig};
//! use crate::genetics::types::InMemoryGeneticsStore;
//! use crate::tunnel::hsm::HsmManager;
//! use std::sync::Arc;
//!
//! // Create the spawning engine
//! let genetics_store = Arc::new(InMemoryGeneticsStore::new());
//! let hsm_manager = Arc::new(HsmManager::new());
//! let config = GeneticsConfig::default();
//! 
//! let engine = GeneticSpawningEngine::new(genetics_store, hsm_manager, config);
//!
//! // Process spawn requests
//! let result = engine.process_spawn_request(request).await?;
//! ```

pub mod engine;
pub mod workflows;
pub mod recombination;
pub mod evolution;
pub mod validation;
pub mod lineage;

// Re-export all types from genetics/types that are needed
pub use super::types::*;

// Re-export the main engine and key components
pub use engine::GeneticSpawningEngine;

// Re-export workflow functions for advanced usage
pub use workflows::{
    process_automated_consensus,
    process_human_approval,
    process_hybrid_approval,
};

// Re-export recombination functions for advanced usage
pub use recombination::{
    recombine_genetics,
    recombine_chromosomes,
    blend_security_traits,
    merge_capabilities,
};

// Re-export evolution functions for advanced usage
pub use evolution::{
    apply_directed_evolution,
    apply_mutations,
};

// Re-export validation functions for advanced usage
pub use validation::{
    calculate_spawn_risk_score,
    run_automated_check,
    evaluate_escalation_condition,
};

// Re-export lineage functions for advanced usage
pub use lineage::{
    create_lineage_record,
    calculate_genetic_diversity_score,
};

/// Main spawning engine - provides the complete genetic spawning system
pub type SpawningEngine = GeneticSpawningEngine; 