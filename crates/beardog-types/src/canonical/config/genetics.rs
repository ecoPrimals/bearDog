//! Canonical Genetics Configuration
//!
//! Configuration for genetic algorithms in the BearDog ecosystem.
//!
//! # Overview
//!
//! `CanonicalGeneticsConfig` provides configuration for genetic algorithms including:
//! - Population size and generation limits
//! - Mutation and crossover rates
//! - Enable/disable toggle for evolutionary optimization
//!
//! # Quick Start
//!
//! ```rust
//! use beardog_types::canonical::config::genetics::CanonicalGeneticsConfig;
//!
//! // Create basic genetics configuration
//! let config = CanonicalGeneticsConfig {
//!     enabled: true,
//!     population_size: 100,
//!     mutation_rate: 0.01,       // 1% mutation
//!     crossover_rate: 0.7,       // 70% crossover
//!     max_generations: 1000,
//! };
//!
//! assert!(config.enabled);
//! assert_eq!(config.population_size, 100);
//! ```
//!
//! # Production Configuration
//!
//! ```rust
//! use beardog_types::canonical::config::genetics::CanonicalGeneticsConfig;
//!
//! // Production genetic algorithm with larger population
//! let config = CanonicalGeneticsConfig {
//!     enabled: true,
//!     population_size: 500,      // Larger for better diversity
//!     mutation_rate: 0.005,      // Lower mutation for stability
//!     crossover_rate: 0.8,       // Higher crossover for convergence
//!     max_generations: 5000,     // More generations for convergence
//! };
//! ```
//!
//! # Tuning Guidelines
//!
//! ```rust
//! use beardog_types::canonical::config::genetics::CanonicalGeneticsConfig;
//!
//! // Exploration-focused (early optimization)
//! let exploration_config = CanonicalGeneticsConfig {
//!     enabled: true,
//!     population_size: 200,
//!     mutation_rate: 0.05,       // High mutation for exploration
//!     crossover_rate: 0.6,       // Moderate crossover
//!     max_generations: 2000,
//! };
//!
//! // Exploitation-focused (fine-tuning)
//! let exploitation_config = CanonicalGeneticsConfig {
//!     enabled: true,
//!     population_size: 100,
//!     mutation_rate: 0.001,      // Low mutation for stability
//!     crossover_rate: 0.9,       // High crossover for convergence
//!     max_generations: 10000,
//! };
//! ```
//!
//! # Performance Considerations
//!
//! - **Population Size**: Larger populations provide better diversity but increase computation
//! - **Mutation Rate**: 0.001-0.05 typical range; higher for exploration, lower for exploitation
//! - **Crossover Rate**: 0.6-0.9 typical range; higher rates speed convergence
//! - **Max Generations**: Balance between solution quality and computation time
//!
//! # Use Cases
//!
//! - **Network Optimization**: Evolving optimal network topologies
//! - **Security Parameters**: Optimizing security configurations
//! - **Resource Allocation**: Finding optimal resource distribution
//! - **Pattern Recognition**: Evolving detection patterns
//!
//! # Design Principles
//!
//! - **Configurability**: All parameters exposed for fine-tuning
//! - **Flexibility**: Can be disabled for deterministic behavior
//! - **Performance**: Parameters balance exploration and exploitation
//! - **Testability**: Can be disabled for testing

use serde::{Deserialize, Serialize};

/// Canonical genetics algorithm configuration
///
/// Configuration for genetic algorithms including population management,
/// mutation/crossover rates, and generation limits.
///
/// # Fields
///
/// * `enabled` - Whether genetic algorithms are active
/// * `population_size` - Number of individuals in each generation
/// * `mutation_rate` - Probability of mutation (0.0-1.0)
/// * `crossover_rate` - Probability of crossover (0.0-1.0)
/// * `max_generations` - Maximum generations to evolve
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::genetics::CanonicalGeneticsConfig;
///
/// let config = CanonicalGeneticsConfig {
///     enabled: true,
///     population_size: 100,
///     mutation_rate: 0.01,
///     crossover_rate: 0.7,
///     max_generations: 1000,
/// };
/// ```
///
/// # Tuning Recommendations
///
/// | Parameter | Exploration | Balanced | Exploitation |
/// |-----------|-------------|----------|--------------|
/// | `population_size` | 200-500 | 100-200 | 50-100 |
/// | `mutation_rate` | 0.05-0.1 | 0.01-0.05 | 0.001-0.01 |
/// | `crossover_rate` | 0.6-0.7 | 0.7-0.8 | 0.8-0.9 |
/// | `max_generations` | 1000-2000 | 2000-5000 | 5000-10000 |
///
/// # Performance Notes
///
/// - Larger populations improve solution quality but increase computation
/// - Higher mutation rates promote exploration, lower rates promote convergence
/// - Crossover rate typically 0.6-0.9 for best results
/// - Balance `max_generations` against available computation time
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalGeneticsConfig {
    /// Whether genetic algorithms are enabled
    ///
    /// When `false`, genetic algorithm optimization is skipped.
    /// Useful for testing or when deterministic behavior is required.
    pub enabled: bool,

    /// Size of the genetic algorithm population
    ///
    /// Number of individuals (candidate solutions) in each generation.
    /// Larger populations provide better diversity but increase computation.
    /// **Typical range**: 50-500 depending on problem complexity.
    pub population_size: usize,

    /// Mutation rate (probability 0.0-1.0)
    ///
    /// Probability that a gene will randomly mutate during reproduction.
    /// Higher rates promote exploration, lower rates promote exploitation.
    /// **Typical range**: 0.001-0.05
    pub mutation_rate: f64,

    /// Crossover rate (probability 0.0-1.0)
    ///
    /// Probability that two parents will exchange genetic material.
    /// Higher rates speed convergence, lower rates maintain diversity.
    /// **Typical range**: 0.6-0.9
    pub crossover_rate: f64,

    /// Maximum number of generations to run
    ///
    /// Stops evolution after this many generations to limit computation.
    /// Balance solution quality against available time and resources.
    /// **Typical range**: 1000-10000 depending on problem complexity.
    pub max_generations: u32,
}

pub type GeneticsConfig = CanonicalGeneticsConfig;
