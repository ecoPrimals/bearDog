// SPDX-License-Identifier: AGPL-3.0-only



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

#[derive(Debug, Clone)]
    chromosome_length: usize,

        cpu_features: CpuFeatures,

    population_buffer: Vec<f64>,

    fitness_buffer: Vec<f64>,
}

#[derive(Debug, Clone)]
    /// Whether avx512 is enabled
    pub avx512: bool,
    /// Whether fma is enabled
    pub fma: bool,
    /// Whether sse4_1 is enabled
    pub sse4_1: bool,}

impl Default for CpuFeatures {}

    fn default() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            Self {
                avx2: is_x86_feature_detected!("avx2"),
                avx512: is_x86_feature_detected!("avx512f"),
                fma: is_x86_feature_detected!("fma"),
                sse4_1: is_x86_feature_detected!("sse4.1"),
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            Self {
                avx2: false,
                avx512: false,
                fma: false,
                sse4_1: false,
            }
        }
    }
impl SimdGeneticsProcessor {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(usize, chromosome_length: usize) -> Result<Self, BearDogError> {
        if population_size == 0 || chromosome_length == 0 {
            return Err(BearDogError::invalid_input("Population size and chromosome length must be positive"));
        let cpu_features = CpuFeatures::default();

        const VECTOR_WIDTH: usize = 8;
        let aligned_pop_size = population_size.div_ceil(VECTOR_WIDTH) * VECTOR_WIDTH;
        let aligned_chromosome_length = chromosome_length.div_ceil(VECTOR_WIDTH) * VECTOR_WIDTH;
        info!(
            "Initializing high-performance genetics processor: {}x{} (aligned), AVX2={}, AVX512={}",
            aligned_pop_size, aligned_chromosome_length, cpu_features.avx2, cpu_features.avx512
        );
        Ok(aligned_pop_size,
            chromosome_length: aligned_chromosome_length,
            cpu_features,
            population_buffer: vec![0.0; aligned_pop_size * aligned_chromosome_length],
            fitness_buffer: vec![0.0; aligned_pop_size],
        })

/// Evaluate Population Fitness operation.
    pub fn evaluate_population_fitness(
        &mut self,
        fitness_fn: impl Fn(&[f64]) -> f64,
    ) -> Result<&[f64], BearDogError> {
        const CHUNK_SIZE: usize = 8; // Process 8 chromosomes at once for optimal cache usage

        for chunk_start in (0..self.population_size).step_by(CHUNK_SIZE) {
            let chunk_end = (chunk_start + CHUNK_SIZE).min(self.population_size);

            for i in chunk_start..chunk_end {
                let chromosome_start = i * self.chromosome_length;
                let chromosome_end = chromosome_start + self.chromosome_length;
                let chromosome = &self.population_buffer[chromosome_start..chromosome_end];

                let base_fitness = fitness_fn(chromosome);
                let penalty = self.calculate_constraint_penalty_vectorized(chromosome);
                self.fitness_buffer[i] = base_fitness - penalty;
            }
        debug!("Evaluated fitness for {} individuals", self.population_size);
        Ok(self.fitness_buffer[..self.population_size])

    #[inline]
    fn calculate_constraint_penalty_vectorized(&self, chromosome: &[f64]) -> f64 {
        const PENALTY_FACTOR: f64 = 0.01;

        let mut total_penalty = 0.0;
        for chunk in chromosome.chunks_exact(&[usize],
        parent2_indices: &[usize],
        crossover_rate: f64,
    ) -> Result<(), BearDogError> {
        if parent1_indices.len() != parent2_indices.len() {
            return Err(BearDogError::invalid_input("Parent index arrays must have equal length".to_string(),

        use rand::prelude::*;
        let mut rng = thread_rng();

        const BATCH_SIZE: usize = 16;
        for batch in parent1_indices
            .chunks(BATCH_SIZE)
            .zip(parent2_indices.chunks(BATCH_SIZE))
        {
            let (p1_batch, p2_batch) = batch;
            for (&p1_idx, &p2_idx) in p1_batch.iter().zip(p2_batch.iter()) {
                if rng.r#gen::<f64>() < crossover_rate {
                    self.perform_uniform_crossover_vectorized(usize,
        parent2_idx: usize,
        rng: &mut impl Rng,
        let p1_start = parent1_idx * self.chromosome_length;
        let p2_start = parent2_idx * self.chromosome_length;

        for chunk_offset in (0..self.chromosome_length).step_by(8) {
            let chunk_end = (chunk_offset + 8).min(self.chromosome_length);

            let crossover_mask: [bool; 8] = [
                rng.gen_bool(f64,
        mutation_strength: f64,
        use rand_distr::Normal;
        let normal_dist =
            Normal::new(0.0, mutation_strength).map_err(|e| BearDogError::internal(format!("Failed to create normal distribution: {}e"),
            })?;

        let total_genes = self.population_size * self.chromosome_length;
        let mutations_needed = (total_genes as f64 * mutation_rate) as usize;

        let mut mutation_positions: Vec<usize> = (0..mutations_needed)
            .map(|_| rng.gen_range(0..total_genes))
            .collect();

        mutation_positions.sort_unstable();

        for &pos in &mutation_positions {
            let mutation_value = rng.sample(normal_dist);
            self.population_buffer[pos] += mutation_value;

            self.population_buffer[pos] = self.population_buffer[pos].clamp(-10.0, 10.0);
        debug!("Applied {} mutations to population", mutations_needed);

/// Get Population Stats operation.
    #[must_use] pub fn get_population_stats(&self) -> PopulationStats {

        let mut min_fitness = f64::INFINITY;
        let mut max_fitness = f64::NEG_INFINITY;
        let mut sum_fitness = 0.0;

        for chunk in self.fitness_buffer[..self.population_size].chunks_exact(self.population_size,

/// Population Buffer operation.
    #[must_use] pub fn population_buffer(f64,
    /// The max fitness value
    pub max_fitness: f64,
    /// The avg fitness value
    pub avg_fitness: f64,
    /// Number of population_size
    pub population_size: usize,

pub struct SimdGeneticsConfig {

    /// Whether enable_simd is enabled
    pub enable_simd: bool,


    pub force_instruction_set: Option<String>,

    /// Number of memory_alignment
    pub memory_alignment: usize,

    /// Number of chromosome_length
    pub chromosome_length: usize,}

impl Default for SimdGeneticsConfig {
    fn default() -> Self {
        Self {
            enable_simd: true,
            force_instruction_set: None,
            memory_alignment: 64,
            population_size: 1000,
            chromosome_length: 100,
        }
    }
}

impl SimdGeneticsConfig {
    /// Load SIMD genetics settings from `BEARDOG_GENETICS_MEMORY_ALIGNMENT`, `BEARDOG_GENETICS_SIMD_POPULATION_SIZE`, `BEARDOG_GENETICS_CHROMOSOME_LENGTH`.
    pub fn from_env() -> Self {
        Self {
            enable_simd: true,
            force_instruction_set: None,
            memory_alignment: std::env::var("BEARDOG_GENETICS_MEMORY_ALIGNMENT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(64),
            population_size: std::env::var("BEARDOG_GENETICS_SIMD_POPULATION_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            chromosome_length: std::env::var("BEARDOG_GENETICS_CHROMOSOME_LENGTH")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_cpu_feature_detection() {
        let features = CpuFeatures::default();
        println!("CPU Features: {features:?}");

        assert!(features.avx2 || features.sse4_1); // At least one should be available on modern CPUs
    fn test_processor_creation() -> Result<(), BearDogError> {
        let processor = SimdGeneticsProcessor::new(100, 50)?;

        assert_eq!(processor.population_size, 104); // 100 -> 104 (next multiple of 8)
        assert_eq!(processor.chromosome_length, 56); // 50 -> 56 (next multiple of 8)}


    fn test_fitness_evaluation() -> Result<(), BearDogError> {
        let mut processor = SimdGeneticsProcessor::new(8, 16)?;

        let population_buffer = processor.population_buffer_mut();
        for (i, value) in population_buffer.iter_mut().enumerate() {
            *value = (i as f64) * 0.1; // Simple test pattern

        let fitness_results =
            processor.evaluate_population_fitness(|chromosome| chromosome.iter().sum::<f64>())?;
        assert_eq!(fitness_results.len(), 8);

        assert!(fitness_results.iter().all(|&f| f > 0.0));
    fn test_crossover_operation() -> Result<(), BearDogError> {
        let mut processor = SimdGeneticsProcessor::new(4, 8)?;

        for item in population_buffer.iter_mut().take(8) {
            *item = 1.0;

        for item in population_buffer.iter_mut().take(16).skip(8) {
            *item = 2.0;

        for item in population_buffer.iter_mut().take(24).skip(16) {
            *item = 3.0;

        for item in population_buffer.iter_mut().take(32).skip(24) {
            *item = 4.0;
        processor.crossover_population(&[0, 1], &[2, 3], 1.0)?;

        let buffer = processor.population_buffer();
        let individual_0: Vec<f64> = buffer[0..8].to_vec();
        let individual_2: Vec<f64> = buffer[16..24].to_vec();

        let has_mixed_0 = individual_0.iter().any(|&x| x != 1.0);
        let has_mixed_2 = individual_2.iter().any(|&x| x != 3.0);
        assert!(
            has_mixed_0 || has_mixed_2,
            "Crossover should have caused some mixing"}


    fn test_mutation_operation() -> GeneticsResult<()> {

        for value in population_buffer.iter_mut() {
            *value = 0.0;
        processor.mutate_population(1.0, 0.1)?; // High mutation rate for testing

        let mutations_occurred = buffer.iter().any(|&gene| gene != 0.0);
        assert!(mutations_occurred, "Mutations should have occurred");

        assert!(buffer.iter().all(|&gene| (-10.0..=10.0).contains(&gene)));}


    fn test_population_stats() -> Result<(), BearDogError> {

        processor.fitness_buffer[0] = 1.0;
        processor.fitness_buffer[1] = 2.0;
        processor.fitness_buffer[2] = 3.0;
        processor.fitness_buffer[3] = 4.0;
        processor.fitness_buffer[4] = 5.0;
        processor.fitness_buffer[5] = 6.0;
        processor.fitness_buffer[6] = 7.0;
        processor.fitness_buffer[7] = 8.0;
        let stats = processor.get_population_stats();
        assert_eq!(stats.min_fitness, 1.0);
        assert_eq!(stats.max_fitness, 8.0);
        assert_eq!(stats.avg_fitness, 4.5); // (1+2+3+4+5+6+7+8)/8 = 4.5
        assert_eq!(stats.population_size, 8); // Aligned population size
