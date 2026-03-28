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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    #[test]
    fn evolution_metrics_new_default_and_clone() {
        let m = EvolutionMetrics::new();
        m.total_generations.store(7, Ordering::Relaxed);
        m.successful_mutations.store(2, Ordering::Relaxed);
        let c = m.clone();
        assert_eq!(c.total_generations.load(Ordering::Relaxed), 7);
        assert_eq!(c.successful_mutations.load(Ordering::Relaxed), 2);
        assert_eq!(m.total_generations.load(Ordering::Relaxed), 7);

        let d: EvolutionMetrics = EvolutionMetrics::default();
        assert_eq!(d.total_generations.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn evolution_metrics_all_counters_increment() {
        let m = EvolutionMetrics::new();
        m.total_generations.fetch_add(1, Ordering::Relaxed);
        m.successful_mutations.fetch_add(1, Ordering::Relaxed);
        m.successful_crossovers.fetch_add(1, Ordering::Relaxed);
        m.convergence_events.fetch_add(1, Ordering::Relaxed);
        m.diversity_events.fetch_add(1, Ordering::Relaxed);
        m.fitness_improvements.fetch_add(1, Ordering::Relaxed);
        assert_eq!(m.total_generations.load(Ordering::Relaxed), 1);
        assert_eq!(m.fitness_improvements.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn diversity_metrics_new_default_clone() {
        let m = DiversityMetrics::new();
        m.genetic_diversity.store(3, Ordering::Relaxed);
        let c = m.clone();
        assert_eq!(c.genetic_diversity.load(Ordering::Relaxed), 3);
        assert_eq!(m.genetic_diversity.load(Ordering::Relaxed), 3);
        let d = DiversityMetrics::default();
        assert_eq!(d.phenotypic_diversity.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn diversity_metrics_all_fields_roundtrip_via_clone() {
        let m = DiversityMetrics::new();
        m.genetic_diversity.store(10, Ordering::Relaxed);
        m.phenotypic_diversity.store(11, Ordering::Relaxed);
        m.behavioral_diversity.store(12, Ordering::Relaxed);
        m.specialization_spread.store(13, Ordering::Relaxed);
        let c = m.clone();
        assert_eq!(c.specialization_spread.load(Ordering::Relaxed), 13);
        assert_eq!(m.behavioral_diversity.load(Ordering::Relaxed), 12);
    }
}
