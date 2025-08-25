// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// High-Performance Genetic Algorithm Optimization
///
/// **VECTORIZED GENETIC OPERATIONS USING STABLE RUST**
/// This module provides high-performance implementations of genetic algorithm
/// operations using stable Rust features. All operations are designed to leverage
/// modern CPU capabilities through compiler auto-vectorization and explicit
/// loop unrolling while maintaining safety and compatibility.
/// ## Performance Targets:
/// - 8x improvement in population fitness evaluation through chunked processing
/// - 4x improvement in crossover operations through batch processing
/// - 6x improvement in mutation operations through vectorized math
/// - Zero allocation in hot paths using pre-allocated buffers

use beardog_errors::{BearDogError, BearDogResult};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};
/// High-performance genetic algorithm processor using stable Rust
#[derive(Debug)]
pub struct SimdGeneticsProcessor {
    /// Population size (aligned to 8 for optimal vectorization)
    population_size: usize,
    /// Chromosome length (aligned to 8 for optimal vectorization)
    chromosome_length: usize,
    /// CPU capability detection
    #[allow(dead_code)]
    cpu_features: CpuFeatures,
    /// Memory-aligned population buffer
    population_buffer: Vec<f64>,
    /// Memory-aligned fitness buffer
    fitness_buffer: Vec<f64>,
}
/// CPU feature detection for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuFeatures {
    pub avx2: bool,
    pub avx512: bool,
    pub fma: bool,
    pub sse4_1: bool,}


impl Default for CpuFeatures {}


    fn default() -> Self {
        Self {
            avx2: is_x86_feature_detected!("avx2"),
            avx512: is_x86_feature_detected!("avx512f"),
            fma: is_x86_feature_detected!("fma"),
            sse4_1: is_x86_feature_detected!("sse4.1"),
        }
    }
impl SimdGeneticsProcessor {
    /// Create new SIMD genetics processor with optimal alignment}


    pub fn new(population_size: usize, chromosome_length: usize) -> BearDogResult<Self> {
        if population_size == 0 || chromosome_length == 0 {
            return Err(BearDogError::invalid_input("Population size and chromosome length must be positive".to_string(),
            ));
        let cpu_features = CpuFeatures::default();
        // Align sizes to 8-element boundaries for optimal vectorization
        const VECTOR_WIDTH: usize = 8;
        let aligned_pop_size = population_size.div_ceil(VECTOR_WIDTH) * VECTOR_WIDTH;
        let aligned_chromosome_length = chromosome_length.div_ceil(VECTOR_WIDTH) * VECTOR_WIDTH;
        info!(
            "Initializing high-performance genetics processor: {}x{} (aligned), AVX2={}, AVX512={}",
            aligned_pop_size, aligned_chromosome_length, cpu_features.avx2, cpu_features.avx512
        );
        Ok(Self {
            population_size: aligned_pop_size,
            chromosome_length: aligned_chromosome_length,
            cpu_features,
            population_buffer: vec![0.0; aligned_pop_size * aligned_chromosome_length],
            fitness_buffer: vec![0.0; aligned_pop_size],
        })
    /// High-performance fitness evaluation using stable vectorization
    pub fn evaluate_population_fitness(
        &mut self,
        fitness_fn: impl Fn(&[f64]) -> f64,
    ) -> BearDogResult<&[f64]> {
        const CHUNK_SIZE: usize = 8; // Process 8 chromosomes at once for optimal cache usage
        // Process population in chunks for better cache locality and vectorization
        for chunk_start in (0..self.population_size).step_by(CHUNK_SIZE) {
            let chunk_end = (chunk_start + CHUNK_SIZE).min(self.population_size);
            // Parallel fitness evaluation within chunk
            for i in chunk_start..chunk_end {
                let chromosome_start = i * self.chromosome_length;
                let chromosome_end = chromosome_start + self.chromosome_length;
                let chromosome = &self.population_buffer[chromosome_start..chromosome_end];
                // Apply fitness function with penalty for constraint violations
                let base_fitness = fitness_fn(chromosome);
                let penalty = self.calculate_constraint_penalty_vectorized(chromosome);
                self.fitness_buffer[i] = base_fitness - penalty;
            }
        debug!("Evaluated fitness for {} individuals", self.population_size);
        Ok(&self.fitness_buffer[..self.population_size])
    /// Vectorized constraint penalty calculation using safe Rust
    #[inline]
    fn calculate_constraint_penalty_vectorized(&self, chromosome: &[f64]) -> f64 {
        const PENALTY_FACTOR: f64 = 0.01;
        // Process in chunks of 8 for optimal vectorization by compiler
        let mut total_penalty = 0.0;
        for chunk in chromosome.chunks_exact(8) {
            // Manual loop unrolling for predictable vectorization
            let penalty_sum = chunk[0].abs()
                + chunk[1].abs()
                + chunk[2].abs()
                + chunk[3].abs()
                + chunk[4].abs()
                + chunk[5].abs()
                + chunk[6].abs()
                + chunk[7].abs();
            total_penalty += penalty_sum * PENALTY_FACTOR;
        // Handle remaining elements
        for &value in chromosome.chunks_exact(8).remainder() {
            total_penalty += value.abs() * PENALTY_FACTOR;
        total_penalty
    /// High-performance crossover operation using batch processing}


    pub fn crossover_population(
        parent1_indices: &[usize],
        parent2_indices: &[usize],
        crossover_rate: f64,
    ) -> BearDogResult<()> {
        if parent1_indices.len() != parent2_indices.len() {
            return Err(BearDogError::invalid_input("Parent index arrays must have equal length".to_string(),
        // Use fast random number generation for crossover decisions
        use rand::prelude::*;
        let mut rng = thread_rng();
        // Process crossovers in batches for better cache performance
        const BATCH_SIZE: usize = 16;
        for batch in parent1_indices
            .chunks(BATCH_SIZE)
            .zip(parent2_indices.chunks(BATCH_SIZE))
        {
            let (p1_batch, p2_batch) = batch;
            for (&p1_idx, &p2_idx) in p1_batch.iter().zip(p2_batch.iter()) {
                if rng.gen::<f64>() < crossover_rate {
                    self.perform_uniform_crossover_vectorized(p1_idx, p2_idx, &mut rng)?;
                }
        Ok(())
    /// Vectorized uniform crossover implementation
    fn perform_uniform_crossover_vectorized(
        parent1_idx: usize,
        parent2_idx: usize,
        rng: &mut impl Rng,
        let p1_start = parent1_idx * self.chromosome_length;
        let p2_start = parent2_idx * self.chromosome_length;
        // Process genes in chunks for vectorization
        for chunk_offset in (0..self.chromosome_length).step_by(8) {
            let chunk_end = (chunk_offset + 8).min(self.chromosome_length);
            // Generate crossover mask for this chunk
            let crossover_mask: [bool; 8] = [
                rng.gen_bool(0.5),
            ];
            // Apply crossover mask efficiently
            for (i, &should_cross) in crossover_mask.iter().enumerate() {
                if chunk_offset + i >= chunk_end {
                    break;
                if should_cross {
                    let p1_pos = p1_start + chunk_offset + i;
                    let p2_pos = p2_start + chunk_offset + i;
                    self.population_buffer.swap(p1_pos, p2_pos);
    /// High-performance mutation using vectorized operations}


    pub fn mutate_population(
        mutation_rate: f64,
        mutation_strength: f64,
        use rand_distr::Normal;
        let normal_dist =
            Normal::new(0.0, mutation_strength).map_err(|e| BearDogError::internal(format!("Failed to create normal distribution: {e)"),
            })?;
        // Pre-generate mutation decisions for better performance
        let total_genes = self.population_size * self.chromosome_length;
        let mutations_needed = (total_genes as f64 * mutation_rate) as usize;
        // Generate random positions for mutations
        let mut mutation_positions: Vec<usize> = (0..mutations_needed)
            .map(|_| rng.gen_range(0..total_genes))
            .collect();
        // Sort positions for better cache locality
        mutation_positions.sort_unstable();
        // Apply mutations with vectorized noise generation
        for &pos in &mutation_positions {
            let mutation_value = rng.sample(normal_dist);
            self.population_buffer[pos] += mutation_value;
            // Apply bounds clamping
            self.population_buffer[pos] = self.population_buffer[pos].clamp(-10.0, 10.0);
        debug!("Applied {} mutations to population", mutations_needed);
    /// Get population statistics with vectorized computation
    #[must_use] pub fn get_population_stats(&self) -> PopulationStats {
        // Vectorized min/max/sum computation
        let mut min_fitness = f64::INFINITY;
        let mut max_fitness = f64::NEG_INFINITY;
        let mut sum_fitness = 0.0;
        // Process fitness values in chunks for vectorization
        for chunk in self.fitness_buffer[..self.population_size].chunks_exact(8) {
            // Manual unrolling for predictable vectorization
            for &fitness in chunk {
                min_fitness = min_fitness.min(fitness);
                max_fitness = max_fitness.max(fitness);
                sum_fitness += fitness;
        // Handle remainder
        for &fitness in self.fitness_buffer[..self.population_size]
            .chunks_exact(8)
            .remainder()
            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness += fitness;
        let avg_fitness = sum_fitness / self.population_size as f64;
        PopulationStats {
            min_fitness,
            max_fitness,
            avg_fitness,
            population_size: self.population_size,
    /// Get reference to population buffer for external processing
    #[must_use] pub fn population_buffer(&self) -> &[f64] {
        &self.population_buffer
    /// Get mutable reference to population buffer for initialization}


    pub fn population_buffer_mut(&mut self) -> &mut [f64] {
        &mut self.population_buffer
/// Population statistics
pub struct PopulationStats {
    pub min_fitness: f64,
    pub max_fitness: f64,
    pub avg_fitness: f64,
    pub population_size: usize,
/// SIMD genetic algorithm configuration
pub struct SimdGeneticsConfig {
    /// Enable SIMD optimizations
    pub enable_simd: bool,
    /// Force specific SIMD instruction set
    pub force_instruction_set: Option<String>,
    /// Memory alignment for optimal performance
    pub memory_alignment: usize,
    /// Population size (will be aligned to vector width)
    /// Chromosome length (will be aligned to vector width)
    pub chromosome_length: usize,}


impl Default for SimdGeneticsConfig {
            enable_simd: true,
            force_instruction_set: None,
            memory_alignment: 64, // 64-byte alignment for AVX-512
            population_size: 1000,
            chromosome_length: 100,}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cpu_feature_detection() {
        let features = CpuFeatures::default();
        println!("CPU Features: {features:?}");
        // Basic validation that features are detected
        assert!(features.avx2 || features.sse4_1); // At least one should be available on modern CPUs
    fn test_processor_creation() -> BearDogResult<()> {
        let processor = SimdGeneticsProcessor::new(100, 50)?;
        // Check alignment - should be rounded up to nearest multiple of 8
        assert_eq!(processor.population_size, 104); // 100 -> 104 (next multiple of 8)
        assert_eq!(processor.chromosome_length, 56); // 50 -> 56 (next multiple of 8)}


    fn test_fitness_evaluation() -> BearDogResult<()> {
        let mut processor = SimdGeneticsProcessor::new(8, 16)?;
        // Initialize population with test data
        let population_buffer = processor.population_buffer_mut();
        for (i, value) in population_buffer.iter_mut().enumerate() {
            *value = (i as f64) * 0.1; // Simple test pattern
        // Simple fitness function: sum of chromosome values
        let fitness_results =
            processor.evaluate_population_fitness(|chromosome| chromosome.iter().sum::<f64>())?;
        assert_eq!(fitness_results.len(), 8);
        // Fitness should be positive for our test data
        assert!(fitness_results.iter().all(|&f| f > 0.0));
    fn test_crossover_operation() -> BearDogResult<()> {
        let mut processor = SimdGeneticsProcessor::new(4, 8)?;
        // Initialize population with distinct patterns
        // Individual 0: all 1.0
        for item in population_buffer.iter_mut().take(8) {
            *item = 1.0;
        // Individual 1: all 2.0
        for item in population_buffer.iter_mut().take(16).skip(8) {
            *item = 2.0;
        // Individual 2: all 3.0
        for item in population_buffer.iter_mut().take(24).skip(16) {
            *item = 3.0;
        // Individual 3: all 4.0
        for item in population_buffer.iter_mut().take(32).skip(24) {
            *item = 4.0;
        processor.crossover_population(&[0, 1], &[2, 3], 1.0)?;
        // Verify that crossover occurred (values should have mixed)
        let buffer = processor.population_buffer();
        let individual_0: Vec<f64> = buffer[0..8].to_vec();
        let individual_2: Vec<f64> = buffer[16..24].to_vec();
        // At least some mixing should have occurred
        let has_mixed_0 = individual_0.iter().any(|&x| x != 1.0);
        let has_mixed_2 = individual_2.iter().any(|&x| x != 3.0);
        assert!(
            has_mixed_0 || has_mixed_2,
            "Crossover should have caused some mixing"}


    fn test_mutation_operation() -> GeneticsResult<()> {
        // Initialize with zeros
        for value in population_buffer.iter_mut() {
            *value = 0.0;
        processor.mutate_population(1.0, 0.1)?; // High mutation rate for testing
        // At least some genes should have mutated from 0
        let mutations_occurred = buffer.iter().any(|&gene| gene != 0.0);
        assert!(mutations_occurred, "Mutations should have occurred");
        // All genes should be in valid range
        assert!(buffer.iter().all(|&gene| (-10.0..=10.0).contains(&gene)));}


    fn test_population_stats() -> BearDogResult<()> {
        // Set known fitness values for testing - processor aligns to 8, so population_size is 8
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
