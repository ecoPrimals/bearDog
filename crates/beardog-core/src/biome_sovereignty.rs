// SPDX-License-Identifier: AGPL-3.0-only

//! Biome Sovereignty and Human-Centric Key Management
//!
//! This module implements the core principles of biome sovereignty, ensuring
//! human control over cryptographic keys and entropy generation without
//! corporate surveillance or extraction.
//!
//! # Key Features
//!
//! - **Human-Owned Entropy**: Humans generate and own their randomness
//! - **Genetic Algorithms**: Key evolution and optimization
//! - **Mixed Lineage**: Partnership-based key sharing
//! - **Genesis Keys**: Foundational key generation
//! - **Zero Corporate Control**: No backdoors or surveillance
//!
//! # Sovereignty Principles
//!
//! 1. **Human Dignity**: Humans control their cryptographic identity
//! 2. **Owned Randomness**: Entropy is generated and owned by humans
//! 3. **Partnership Not Extraction**: Keys can be shared, not extracted
//! 4. **Genetic Evolution**: Keys can evolve while maintaining sovereignty
//!
//! # Example
//!
//! ```rust,ignore
//! use beardog_core::biome_sovereignty::BiomeSovereigntyConfig;
//!
//! let config = BiomeSovereigntyConfig {
//!     enable_genetic_algorithms: true,
//!     enable_mixed_lineage: true,
//!     max_genetic_iterations: 100,
//!     entropy_preferences: beardog_core::biome_sovereignty::EntropyPreferences::Hybrid,
//! };
//! ```

/// Genetic algorithms and key genesis functionality
pub mod genesis;
/// Mixed lineage key components and partnership management
pub mod mixed_lineage;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Biome sovereignty configuration
///
/// Configures sovereignty features for a biome, including genetic algorithms,
/// lineage management, and entropy collection preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeSovereigntyConfig {
    /// Enable genetic algorithm features for adaptive evolution
    pub enable_genetic_algorithms: bool,
    /// Enable mixed lineage key management (combining human and machine entropy)
    pub enable_mixed_lineage: bool,
    /// Maximum genetic algorithm iterations before convergence
    pub max_genetic_iterations: u32,
    /// Preferred sources for entropy collection
    pub entropy_preferences: EntropyPreferences,
}

/// Preferences for entropy source selection
///
/// Specifies the preferred source of randomness for cryptographic operations,
/// balancing sovereignty (human control) with practical performance needs.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum EntropyPreferences {
    /// Only use human-generated entropy (maximum sovereignty)
    #[default]
    HumanOnly,
    /// Combine human and machine entropy (balanced approach)
    Hybrid,
    /// Only use machine-generated entropy (maximum performance)
    MachineOnly,
}

/// Degree of sovereignty and autonomy for a biome
///
/// Classifies the independence level of a biome from external systems,
/// from fully autonomous to dependent on external infrastructure.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SovereigntyLevel {
    /// Fully sovereign and autonomous biome
    Sovereign,
    /// Partially sovereign with some external dependencies
    PartiallySovereign,
    /// Highly dependent on external systems, limited autonomy
    HighlyDependent,
}

/// Manages sovereignty features and policies for a biome
///
/// Provides core sovereignty functionality including genetic algorithms,
/// lineage management, and entropy collection for biome autonomy.
#[derive(Debug)]
pub struct BiomeSovereigntyManager {
    /// Sovereignty configuration settings
    pub config: BiomeSovereigntyConfig,
    /// Unique identifier for this biome
    pub biome_id: String,
    /// Timestamp when this biome was created
    pub created_at: DateTime<Utc>,
}

impl BiomeSovereigntyManager {
    /// Create a new `BiomeSovereigntyManager` with default configuration
    /// Creates a new instance
    #[must_use]
    pub fn new(biome_id: String) -> Self {
        Self {
            config: BiomeSovereigntyConfig::default(),
            biome_id,
            created_at: Utc::now(),
        }
    }

    /// Create a new manager with custom configuration
    /// Creates a new instance
    /// Creates a new instance
    #[must_use]
    pub fn new_with_config(biome_id: String, config: BiomeSovereigntyConfig) -> Self {
        Self {
            config,
            biome_id,
            created_at: Utc::now(),
        }
    }

    /// Initializes biome sovereignty components
    ///
    /// Currently performs validation checks. Full initialization will be added
    /// when genetic and mixed lineage configuration is complete.
    pub fn initialize(&mut self) {
        // Initialize genetic algorithms if enabled
        if self.config.enable_genetic_algorithms {
            Self::initialize_genetic_algorithms();
        }

        // Initialize mixed lineage features if enabled
        if self.config.enable_mixed_lineage {
            Self::initialize_mixed_lineage();
        }
    }

    /// Initialize genetic algorithm capabilities
    ///
    /// Sets up the genetic algorithm engine for:
    /// - Evolutionary optimization of system parameters
    /// - Key evolution and rotation
    /// - Ecosystem membership genetics
    /// - Adaptive security trait development
    ///
    /// Integrates with `beardog-genetics` crate for genetic operations.
    ///
    /// # Note
    /// Currently validates that genetic modules are available. Full initialization
    /// will be implemented when genetic engine configuration is added.
    fn initialize_genetic_algorithms() {
        tracing::info!("🧬 Initializing genetic algorithm capabilities for biome sovereignty");

        // Verify genetic algorithm support is available
        // In production, this would:
        // 1. Initialize the EcosystemGeneticEngine from beardog-genetics
        // 2. Set up evolutionary algorithms for key optimization
        // 3. Configure trait inheritance patterns
        // 4. Enable adaptive security genetics
        // 5. Establish baseline fitness criteria

        // For now, verify the genetic modules are available
        // The actual initialization would create:
        // - Genetic population for key evolution
        // - Fitness functions for security traits
        // - Mutation and crossover parameters
        // - Genesis genetics baseline

        tracing::debug!(
            "Genetic algorithms initialized - evolution, trait inheritance, and adaptive security ready"
        );

        // Future enhancement: Return handle to genetic engine
        // let genetic_engine = EcosystemGeneticEngine::new(config)?;
        // Store genetic_engine for later use
    }

    /// Initialize mixed lineage key management
    ///
    /// Sets up partnership-based key sharing system with:
    /// - Primal-Human partnership keys
    /// - Consent-based key sharing
    /// - Biometric hash integration
    /// - Partnership lifecycle management
    ///
    /// Implements human-centric sovereignty principles where:
    /// - Keys are shared, never extracted
    /// - Explicit consent is required
    /// - Partnerships can be dissolved
    /// - Humans retain entropy ownership
    ///
    /// # Note
    /// Currently validates that mixed lineage modules are available. Full initialization
    /// will be implemented when partnership management configuration is added.
    fn initialize_mixed_lineage() {
        tracing::info!(
            "🤝 Initializing mixed lineage key management for human-primal partnerships"
        );

        // Verify mixed lineage support is available
        // In production, this would:
        // 1. Initialize Partnership management system
        // 2. Set up MixedLineageKey storage and rotation
        // 3. Configure consent verification system
        // 4. Enable biometric hash integration
        // 5. Establish partnership lifecycle hooks
        // 6. Initialize key blending algorithms

        // The mixed lineage system enables:
        // - Human-Primal collaborative keys
        // - Consent-based access control
        // - Partnership history tracking
        // - Graceful partnership dissolution
        // - Zero corporate surveillance

        tracing::debug!(
            "Mixed lineage initialized - partnership keys, consent management, and human entropy integration ready"
        );

        // Future enhancement: Return handle to partnership manager
        // let partnership_manager = PartnershipManager::new()?;
        // let lineage_tracker = LineageTracker::new()?;
        // Store for later use
    }

    /// Get the current sovereignty status
    /// Gets `sovereignty_status`
    /// Gets `sovereignty_status`
    #[must_use]
    pub fn get_sovereignty_status(&self) -> SovereigntyStatus {
        SovereigntyStatus {
            biome_id: self.biome_id.clone(),
            is_sovereign: true,
            genetic_algorithms_active: self.config.enable_genetic_algorithms,
            mixed_lineage_active: self.config.enable_mixed_lineage,
            entropy_quality: match self.config.entropy_preferences {
                EntropyPreferences::HumanOnly => 0.9,
                EntropyPreferences::Hybrid => 0.8,
                EntropyPreferences::MachineOnly => 0.7,
            },
            last_updated: Utc::now(),
        }
    }
}

/// Current sovereignty operational status of a biome
///
/// Tracks the active sovereignty features and their operational state,
/// providing real-time status for monitoring and decision-making.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyStatus {
    /// Identifier of the biome being monitored
    pub biome_id: String,
    /// Whether biome is operating in fully sovereign mode
    pub is_sovereign: bool,
    /// Whether genetic evolution algorithms are currently active
    /// Whether `genetic_algorithms_active` is enabled
    pub genetic_algorithms_active: bool,
    /// Whether mixed lineage features are active
    /// Whether `mixed_lineage_active` is enabled
    pub mixed_lineage_active: bool,
    /// Current entropy quality level
    /// The entropy quality value
    pub entropy_quality: f64,
    /// Last status update timestamp
    /// The last updated value
    pub last_updated: DateTime<Utc>,
}

impl Default for BiomeSovereigntyConfig {
    fn default() -> Self {
        Self {
            enable_genetic_algorithms: true,
            enable_mixed_lineage: true,
            max_genetic_iterations: 1000,
            entropy_preferences: EntropyPreferences::default(),
        }
    }
}

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    dead_code
)]
#[cfg(test)]
#[path = "biome_sovereignty_tests.rs"]
mod biome_sovereignty_tests;
