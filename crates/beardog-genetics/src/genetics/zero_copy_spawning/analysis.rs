//! Genetic Analysis for Zero-Copy Operations
//!
//! Efficient genetic analysis with caching and streaming processing
//! to minimize allocations during fitness calculations and population analysis.

use std::collections::HashMap;
use std::time::Instant;
use serde::{Deserialize, Serialize};

/// Cached fitness analysis results to avoid recomputation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFitnessAnalysis {
    /// Overall fitness score
    pub fitness_score: f64,
    /// Performance-specific fitness score
    pub performance_score: f64,
    /// Security-specific fitness score
    pub security_score: f64,
    /// Adaptability and flexibility score
    pub adaptability_score: f64,
    /// Timestamp when this analysis was calculated
    pub calculated_at: Instant,
    /// Hash of the genetic data this analysis corresponds to
    pub genetic_hash: u64,
}

impl CachedFitnessAnalysis {
    /// Create new cached fitness analysis
    pub fn new(
        fitness_score: f64,
        performance_score: f64,
        security_score: f64,
        adaptability_score: f64,
        genetic_hash: u64,
    ) -> Self {
        Self {
            fitness_score,
            performance_score,
            security_score,
            adaptability_score,
            calculated_at: Instant::now(),
            genetic_hash,
        }
    }

    /// Check if this cached analysis is still valid
    pub fn is_valid(&self, max_age_seconds: u64) -> bool {
        self.calculated_at.elapsed().as_secs() < max_age_seconds
    }

    /// Get combined fitness score
    pub fn get_combined_score(&self) -> f64 {
        (self.fitness_score + self.performance_score + self.security_score + self.adaptability_score) / 4.0
    }
}

/// Analysis of genetic data chunks for streaming processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkAnalysis {
    /// Chunk identifier
    pub chunk_id: String,
    /// Size of the chunk in bytes
    pub chunk_size: usize,
    /// Number of genetic entities in this chunk
    pub entity_count: u32,
    /// Average fitness of entities in this chunk
    pub average_fitness: f64,
    /// Processing time for this chunk in microseconds
    pub processing_time_us: u64,
}

impl ChunkAnalysis {
    /// Create new chunk analysis
    pub fn new(chunk_id: String, chunk_size: usize, entity_count: u32) -> Self {
        Self {
            chunk_id,
            chunk_size,
            entity_count,
            average_fitness: 0.0,
            processing_time_us: 0,
        }
    }

    /// Update fitness score for this chunk
    pub fn set_average_fitness(&mut self, average_fitness: f64) {
        self.average_fitness = average_fitness;
    }

    /// Record processing time
    pub fn set_processing_time(&mut self, processing_time_us: u64) {
        self.processing_time_us = processing_time_us;
    }

    /// Get processing efficiency (entities per microsecond)
    pub fn get_processing_efficiency(&self) -> f64 {
        if self.processing_time_us == 0 {
            0.0
        } else {
            self.entity_count as f64 / self.processing_time_us as f64
        }
    }
}

/// Population-level analysis for genetic optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationAnalysis {
    /// Total population size
    pub population_size: u32,
    /// Average fitness across the population
    pub average_fitness: f64,
    /// Best (maximum) fitness in population
    pub best_fitness: f64,
    /// Worst (minimum) fitness in population
    pub worst_fitness: f64,
    /// Standard deviation of fitness scores
    pub fitness_std_dev: f64,
    /// Genetic diversity measure (0.0 to 1.0)
    pub genetic_diversity: f64,
    /// Analysis timestamp
    pub analyzed_at: Instant,
}

impl PopulationAnalysis {
    /// Create new population analysis
    pub fn new(population_size: u32) -> Self {
        Self {
            population_size,
            average_fitness: 0.0,
            best_fitness: f64::NEG_INFINITY,
            worst_fitness: f64::INFINITY,
            fitness_std_dev: 0.0,
            genetic_diversity: 0.0,
            analyzed_at: Instant::now(),
        }
    }

    /// Update fitness statistics
    pub fn update_fitness_stats(
        &mut self,
        average: f64,
        best: f64,
        worst: f64,
        std_dev: f64,
    ) {
        self.average_fitness = average;
        self.best_fitness = best;
        self.worst_fitness = worst;
        self.fitness_std_dev = std_dev;
    }

    /// Set genetic diversity measure
    pub fn set_genetic_diversity(&mut self, diversity: f64) {
        self.genetic_diversity = diversity.max(0.0).min(1.0);
    }

    /// Get fitness range
    pub fn get_fitness_range(&self) -> f64 {
        self.best_fitness - self.worst_fitness
    }

    /// Check if population is converging (low diversity)
    pub fn is_converging(&self, diversity_threshold: f64) -> bool {
        self.genetic_diversity < diversity_threshold
    }

    /// Get population health score (0.0 to 1.0)
    pub fn get_health_score(&self) -> f64 {
        // Combine fitness and diversity metrics
        let fitness_score = if self.best_fitness > 0.0 {
            self.average_fitness / self.best_fitness
        } else {
            0.0
        };
        
        // Balance between fitness and diversity
        (fitness_score * 0.7 + self.genetic_diversity * 0.3).max(0.0).min(1.0)
    }
}

/// Fitness cache for avoiding redundant calculations
#[derive(Debug)]
pub struct FitnessCache {
    /// Cache storage mapping genetic hash to fitness analysis
    cache: HashMap<u64, CachedFitnessAnalysis>,
    /// Maximum cache size
    max_size: usize,
    /// Cache hit count
    hits: u64,
    /// Cache miss count
    misses: u64,
}

impl FitnessCache {
    /// Create new fitness cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }

    /// Get cached fitness analysis
    pub fn get(&mut self, genetic_hash: u64, max_age_seconds: u64) -> Option<&CachedFitnessAnalysis> {
        if let Some(analysis) = self.cache.get(&genetic_hash) {
            if analysis.is_valid(max_age_seconds) {
                self.hits += 1;
                return Some(analysis);
            } else {
                // Remove expired entry
                self.cache.remove(&genetic_hash);
            }
        }
        self.misses += 1;
        None
    }

    /// Store fitness analysis in cache
    pub fn store(&mut self, genetic_hash: u64, analysis: CachedFitnessAnalysis) {
        // If cache is full, remove oldest entries
        if self.cache.len() >= self.max_size {
            self.evict_oldest();
        }
        
        self.cache.insert(genetic_hash, analysis);
    }

    /// Evict oldest cache entries
    fn evict_oldest(&mut self) {
        if self.cache.is_empty() {
            return;
        }

        // Simple eviction: remove first entry (in practice, would use LRU)
        if let Some(key) = self.cache.keys().next().copied() {
            self.cache.remove(&key);
        }
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> (u64, u64, f64, usize) {
        let total_requests = self.hits + self.misses;
        let hit_rate = if total_requests == 0 {
            0.0
        } else {
            self.hits as f64 / total_requests as f64 * 100.0
        };
        
        (self.hits, self.misses, hit_rate, self.cache.len())
    }

    /// Clear all cache entries
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }
} 