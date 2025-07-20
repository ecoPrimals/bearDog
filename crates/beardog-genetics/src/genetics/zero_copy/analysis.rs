//! Genetic Analysis Components
//!
//! Provides fitness analysis and population analysis functionality.

use beardog_auth::auth::NodeCapability;
use std::collections::HashMap;

/// Cached fitness analysis to avoid recomputation
#[derive(Debug, Clone)]
pub struct CachedFitnessAnalysis {
    pub fitness_score: f64,
    pub generation: u32,
    pub capabilities: Vec<NodeCapability>,
    pub genetic_hash: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Analysis results for a chunk of genetic data
#[derive(Debug, Default)]
pub struct ChunkAnalysis {
    pub nodes_analyzed: usize,
    pub fitness_sum: f64,
    pub generation_sum: u64,
    pub max_generation: u32,
    pub capabilities_distribution: HashMap<NodeCapability, u32>,
}

/// Complete population analysis results
#[derive(Debug, Default)]
pub struct PopulationAnalysis {
    pub total_nodes: usize,
    pub average_fitness: f64,
    pub average_generation: f64,
    pub max_generation: u32,
    pub diversity_index: f64,
    pub capabilities_distribution: HashMap<NodeCapability, u32>,
}

impl ChunkAnalysis {
    /// Create new empty chunk analysis
    pub fn new() -> Self {
        Self::default()
    }

    /// Merge this analysis with another chunk
    pub fn merge(&mut self, other: &ChunkAnalysis) {
        self.nodes_analyzed += other.nodes_analyzed;
        self.fitness_sum += other.fitness_sum;
        self.generation_sum += other.generation_sum;
        self.max_generation = self.max_generation.max(other.max_generation);

        // Merge capability distributions
        for (capability, count) in &other.capabilities_distribution {
            *self
                .capabilities_distribution
                .entry(capability.clone())
                .or_insert(0) += count;
        }
    }
}

impl PopulationAnalysis {
    /// Create new population analysis from chunk analyses
    pub fn from_chunks(chunks: &[ChunkAnalysis]) -> Self {
        let total_nodes: usize = chunks.iter().map(|c| c.nodes_analyzed).sum();
        let fitness_sum: f64 = chunks.iter().map(|c| c.fitness_sum).sum();
        let generation_sum: u64 = chunks.iter().map(|c| c.generation_sum).sum();
        let max_generation = chunks.iter().map(|c| c.max_generation).max().unwrap_or(0);

        let mut capabilities_distribution = HashMap::new();
        for chunk in chunks {
            for (capability, count) in &chunk.capabilities_distribution {
                *capabilities_distribution
                    .entry(capability.clone())
                    .or_insert(0) += count;
            }
        }

        // Calculate diversity index based on capability distribution
        let diversity_index =
            Self::calculate_diversity_index(&capabilities_distribution, total_nodes);

        Self {
            total_nodes,
            average_fitness: if total_nodes > 0 {
                fitness_sum / total_nodes as f64
            } else {
                0.0
            },
            average_generation: if total_nodes > 0 {
                generation_sum as f64 / total_nodes as f64
            } else {
                0.0
            },
            max_generation,
            diversity_index,
            capabilities_distribution,
        }
    }

    /// Calculate genetic diversity index
    fn calculate_diversity_index(distribution: &HashMap<NodeCapability, u32>, total: usize) -> f64 {
        if total == 0 {
            return 0.0;
        }

        // Shannon diversity index
        let mut diversity = 0.0;
        for count in distribution.values() {
            if *count > 0 {
                let p = *count as f64 / total as f64;
                diversity -= p * p.ln();
            }
        }
        diversity
    }
}
