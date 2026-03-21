// SPDX-License-Identifier: AGPL-3.0-only

//! Atomic counters for evolution and diversity tracking.

use std::sync::atomic::{AtomicU64, Ordering};

/// Evolution metrics
#[derive(Debug)]
pub struct EvolutionMetrics {
    /// Total generations
    pub total_generations: AtomicU64,

    /// Successful mutations
    pub successful_mutations: AtomicU64,

    /// Successful crossovers
    pub successful_crossovers: AtomicU64,

    /// Convergence events
    pub convergence_events: AtomicU64,

    /// Diversity events
    pub diversity_events: AtomicU64,

    /// Fitness improvements
    pub fitness_improvements: AtomicU64,
}

impl EvolutionMetrics {
    /// Create new metrics
    pub fn new() -> Self {
        Self {
            total_generations: AtomicU64::new(0),
            successful_mutations: AtomicU64::new(0),
            successful_crossovers: AtomicU64::new(0),
            convergence_events: AtomicU64::new(0),
            diversity_events: AtomicU64::new(0),
            fitness_improvements: AtomicU64::new(0),
        }
    }
}

impl Clone for EvolutionMetrics {
    fn clone(&self) -> Self {
        Self {
            total_generations: AtomicU64::new(self.total_generations.load(Ordering::Relaxed)),
            successful_mutations: AtomicU64::new(self.successful_mutations.load(Ordering::Relaxed)),
            successful_crossovers: AtomicU64::new(
                self.successful_crossovers.load(Ordering::Relaxed),
            ),
            convergence_events: AtomicU64::new(self.convergence_events.load(Ordering::Relaxed)),
            diversity_events: AtomicU64::new(self.diversity_events.load(Ordering::Relaxed)),
            fitness_improvements: AtomicU64::new(self.fitness_improvements.load(Ordering::Relaxed)),
        }
    }
}

impl Default for EvolutionMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Diversity metrics
#[derive(Debug)]
pub struct DiversityMetrics {
    /// Genetic diversity
    pub genetic_diversity: AtomicU64,

    /// Phenotypic diversity
    pub phenotypic_diversity: AtomicU64,

    /// Behavioral diversity
    pub behavioral_diversity: AtomicU64,

    /// Specialization spread
    pub specialization_spread: AtomicU64,
}

impl DiversityMetrics {
    /// Create new metrics
    pub fn new() -> Self {
        Self {
            genetic_diversity: AtomicU64::new(0),
            phenotypic_diversity: AtomicU64::new(0),
            behavioral_diversity: AtomicU64::new(0),
            specialization_spread: AtomicU64::new(0),
        }
    }
}

impl Clone for DiversityMetrics {
    fn clone(&self) -> Self {
        Self {
            genetic_diversity: AtomicU64::new(self.genetic_diversity.load(Ordering::Relaxed)),
            phenotypic_diversity: AtomicU64::new(self.phenotypic_diversity.load(Ordering::Relaxed)),
            behavioral_diversity: AtomicU64::new(self.behavioral_diversity.load(Ordering::Relaxed)),
            specialization_spread: AtomicU64::new(
                self.specialization_spread.load(Ordering::Relaxed),
            ),
        }
    }
}

impl Default for DiversityMetrics {
    fn default() -> Self {
        Self::new()
    }
}
