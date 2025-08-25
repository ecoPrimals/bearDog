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


/// Genesis Spawning System
///
/// The foundational genetic spawning mechanism that creates the first generation
/// of genetic algorithms and manages the evolutionary process for `BearDog`.

use super::types::{SpawnRequest, SpawnResult};
use beardog_auth::auth::{`BearDog`Genetics, SecurityClearance};
use crate::genetics::types::{GeneticsCapability, genetics_to_node_capabilities};
use beardog_errors::BearDogResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;
use beardog_errors::{BearDogError, BearDogResult};
/// Genesis spawning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisConfig {
    /// Initial population size for first generation
    pub initial_population_size: usize,
    /// Minimum genetic diversity required
    pub min_genetic_diversity: f64,
    /// Maximum generations allowed
    pub max_generations: u32,
    /// Mutation rate for genetic evolution
    pub mutation_rate: f64,
    /// Crossover rate for genetic recombination
    pub crossover_rate: f64,
    /// Elite preservation percentage
    pub elite_preservation: f64,
    /// Fitness threshold for spawning
    pub fitness_threshold: f64,
}
impl Default for GenesisConfig {}


    fn default() -> Self {
        Self {
            initial_population_size: 10,
            min_genetic_diversity: 0.3,
            max_generations: 100,
            mutation_rate: 0.1,
            crossover_rate: 0.8,
            elite_preservation: 0.2,
            fitness_threshold: 0.7,
        }
    }
/// Genesis spawning engine
pub struct GenesisSpawningEngine {
    config: GenesisConfig,
    population: Vec<`BearDog`Genetics>,
    generation: u32,
    stats: GenesisStats,
/// Statistics for Genesis spawning
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GenesisStats {
    pub total_spawned: u64,
    pub successful_spawns: u64,
    pub failed_spawns: u64,
    pub current_generation: u32,
    pub average_fitness: f64,
    pub best_fitness: f64,
    pub genetic_diversity: f64,
    pub last_spawn_time: Option<DateTime<Utc>>,
/// Genesis spawning result with detailed information
pub struct GenesisSpawnResult {
    /// Generated genetics
    pub genetics: `BearDog`Genetics,
    /// Generation number
    pub generation: u32,
    /// Genetic lineage information
    pub lineage: GeneticLineage,
    /// Spawning success status
    pub success: bool,
    /// Performance metrics
    pub metrics: HashMap<String, f64>,
    /// Any messages or warnings
    pub messages: Vec<String>,
/// Genetic lineage tracking
pub struct GeneticLineage {
    /// Parent genetics IDs
    pub parents: Vec<String>,
    /// Mutation events applied
    pub mutations: Vec<MutationEvent>,
    /// Crossover events applied
    pub crossovers: Vec<CrossoverEvent>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
/// Mutation event record
pub struct MutationEvent {
    /// Type of mutation
    pub mutation_type: String,
    /// Strength of mutation (0.0 - 1.0)
    pub strength: f64,
    /// Affected genetic component
    pub component: String,
    /// Timestamp of mutation
    pub timestamp: DateTime<Utc>,
/// Crossover event record
pub struct CrossoverEvent {
    /// Parent A genetics ID
    pub parent_a: String,
    /// Parent B genetics ID
    pub parent_b: String,
    /// Crossover points used
    pub crossover_points: Vec<usize>,
    /// Timestamp of crossover}


impl GenesisSpawningEngine {
    /// Create new Genesis spawning engine}


    pub fn new() -> Self {
            config: GenesisConfig::default(),
            population: Vec::new(),
            generation: 0,
            stats: GenesisStats::default(),
    /// Create Genesis engine with custom configuration}


    pub fn with_config(config: GenesisConfig) -> Self {
            config,
    /// Initialize the first generation (Genesis)
    pub async fn initialize_genesis(&mut self) -> BearDogResult<Vec<GenesisSpawnResult>> {
        info!("🌱 Initializing Genesis - First Generation");
        let mut results = Vec::new();
        let mut created_genetics = Vec::new();
        for i in 0..self.config.initial_population_size {
            let genetics_id = format!("genesis_gen0_{:04}", i);
            // Create foundational genetics with high diversity
            let genetics = self
                .create_foundational_genetics(genetics_id.clone(), i)
                .await?;
            // Calculate initial fitness
            let fitness = self.calculate_genesis_fitness(&genetics).await?;
            let lineage = GeneticLineage {
                parents: vec![], // Genesis has no parents
                generation: 0,
                mutations: vec![],
                crossovers: vec![],
                created_at: Utc::now(),
            };
            let result = GenesisSpawnResult {
                genetics: genetics.clone(),
                lineage,
                success: fitness >= self.config.fitness_threshold,
                metrics: self.collect_genesis_metrics(&genetics).await,
                messages: vec![format!("Genesis genetics created: {}", genetics_id)],
            if result.success {
                created_genetics.push(genetics);
                self.stats.successful_spawns += 1;
            } else {
                self.stats.failed_spawns += 1;
            }
            results.push(result);
            self.stats.total_spawned += 1;
        // Store successful genetics in population
        self.population = created_genetics;
        self.generation = 0;
        self.stats.current_generation = 0;
        self.stats.last_spawn_time = Some(Utc::now());
        // Update diversity and fitness statistics
        self.update_population_stats().await?;
        info!(
            "✅ Genesis initialization complete: {} genetics created",
            self.population.len()
        );
        Ok(results)
    /// Evolve to next generation
    pub async fn evolve_generation(&mut self) -> BearDogResult<Vec<GenesisSpawnResult>> {
        if self.population.is_empty() {
            return Err(beardog_errors::BearDogError::Storage {
                message: "Cannot evolve: no genetics in population".to_string(),
            });
        self.generation += 1;
        info!("🧬 Evolving to generation {}", self.generation);
        // Select parents for reproduction
        let parents = self.select_parents().await?;
        // Create next generation through crossover and mutation
        let mut next_generation = Vec::new();
        let target_size = self.config.initial_population_size;
        let elite_count = (target_size as f64 * self.config.elite_preservation) as usize;
        // Preserve elite genetics - handle potential NaN fitness scores gracefully
        let mut elite_genetics = self.population.clone();
        elite_genetics.sort_by(|a, b| {
            b.fitness_score
                .partial_cmp(&a.fitness_score)
                .unwrap_or(std::cmp::Ordering::Equal) // Handle NaN by treating as equal
        });
        for (i, genetics) in elite_genetics.iter().take(elite_count).enumerate() {
                generation: self.generation,
                lineage: GeneticLineage {
                    parents: vec![genetics.id.clone()],
                    generation: self.generation,
                    mutations: vec![],
                    crossovers: vec![],
                    created_at: Utc::now(),
                },
                success: true,
                metrics: HashMap::new(),
                messages: vec![format!("Elite genetics preserved: rank {}", i + 1)],
            next_generation.push(genetics.clone());
        // Generate offspring to fill remaining population
        while next_generation.len() < target_size {
            let offspring_result = self.create_offspring(&parents).await?;
            if offspring_result.success {
                next_generation.push(offspring_result.genetics.clone());
            results.push(offspring_result);
        // Update population and stats
        self.population = next_generation;
        self.stats.current_generation = self.generation;
            "✅ Generation {} evolution complete: {} genetics",
            self.generation,
    /// Create foundational genetics for Genesis
    async fn create_foundational_genetics(
        &self,
        id: String,
        index: usize,
    ) -> GeneticsResult<`BearDog`Genetics> {
        // Generate diverse base capabilities
        let base_capabilities = self.generate_diverse_capabilities(index).await?;
        // Assign varying security clearances
        let security_clearance = match index % 4 {
            0 => SecurityClearance::Basic,
            1 => SecurityClearance::Medium,
            2 => SecurityClearance::High,
            _ => SecurityClearance::Maximum,
        };
        let mut genetics = `BearDog`Genetics {
            id: id.clone(),
            capabilities: genetics_to_node_capabilities(&base_capabilities),
            security_clearance,
            fitness_score: 0.0, // Will be calculated
            ..Default::default()
        // Add genetic diversity factors
        genetics.mutation_count = 0;
        genetics.generation = 0;
        genetics.created_at = Some(Utc::now());
        Ok(genetics)
    /// Generate diverse capabilities for genetic variety
    async fn generate_diverse_capabilities(
    ) -> BearDogResult<Vec<GeneticsCapability>> {
        let all_capabilities = vec![
            GeneticsCapability::new("Computation".to_string(), 0.7, 0.8),
            GeneticsCapability::new("Storage".to_string(), 0.7, 0.8),
            GeneticsCapability::new("Networking".to_string(), 0.7, 0.8),
            GeneticsCapability::new("Monitoring".to_string(), 0.7, 0.8),
            GeneticsCapability::new("Security".to_string(), 0.7, 0.8),
        ];
        // Create different capability combinations for diversity
        let capabilities = match index % 5 {
            0 => vec![GeneticsCapability::new("Computation".to_string(), 0.7, 0.8), GeneticsCapability::new("Security".to_string(), 0.7, 0.8)],
            1 => vec![GeneticsCapability::new("Storage".to_string(), 0.7, 0.8), GeneticsCapability::new("Networking".to_string(), 0.7, 0.8)],
            2 => vec![GeneticsCapability::new("Monitoring".to_string(), 0.7, 0.8), GeneticsCapability::new("Computation".to_string(), 0.7, 0.8)],
            3 => vec![GeneticsCapability::new("Security".to_string(), 0.7, 0.8), GeneticsCapability::new("Storage".to_string(), 0.7, 0.8)],
            _ => vec![GeneticsCapability::new("Networking".to_string(), 0.7, 0.8), GeneticsCapability::new("Monitoring".to_string(), 0.7, 0.8)],
        Ok(capabilities)
    /// Calculate fitness for Genesis genetics}


    async fn calculate_genesis_fitness(&self, genetics: &`BearDog`Genetics) -> BearDogResult<f64> {
        let mut fitness = 0.0;
        // Base fitness from capabilities diversity
        fitness += genetics.capabilities.len() as f64 * 0.2;
        // Security clearance bonus
        fitness += match genetics.security_clearance {
            SecurityClearance::Basic => 0.1,
            SecurityClearance::Standard => 0.2,
            SecurityClearance::Enhanced => 0.3,
            SecurityClearance::Maximum => 0.4,
        // Genetic health factor
        fitness += 0.3; // Genesis genetics start healthy
        // Normalize to 0.0-1.0 range
        Ok(fitness.min(1.0))
    /// Select parents for reproduction
    async fn select_parents(&self) -> BearDogResult<Vec<`BearDog`Genetics>> {
        if self.population.len() < 2 {
            return Err(beardog_errors::BearDogError::Genetics {
                message: "Not enough genetics for parent selection".to_string(),
        // Tournament selection - handle potential NaN fitness scores gracefully
        let mut parents = Vec::new();
        let tournament_size = 3; // Number of individuals in each tournament
        let parent_count = (self.population.len() / 2).max(2); // Number of parents to select
        let mut rng = rand::thread_rng();
        for _ in 0..parent_count {
            let mut tournament = Vec::new();
            for _ in 0..tournament_size {
                if let Some(genetics) = self.population.choose(&mut rng) {
                    tournament.push(genetics.clone());
                }
            // Sort by fitness, handling NaN values gracefully
            tournament.sort_by(|a, b| {
                b.fitness_score
                    .partial_cmp(&a.fitness_score)
                    .unwrap_or(std::cmp::Ordering::Equal) // Handle NaN by treating as equal
            if let Some(winner) = tournament.first() {
                parents.push(winner.clone());
        Ok(parents)
    /// Create offspring through crossover and mutation
    async fn create_offspring(
        parents: &[`BearDog`Genetics],
    ) -> BearDogResult<GenesisSpawnResult> {
        if parents.len() < 2 {
                message: "Need at least 2 parents for crossover".to_string(),
        // Select two parents randomly
        let parent_a = &parents[rand::random::<usize>() % parents.len()];
        let parent_b = &parents[rand::random::<usize>() % parents.len()];
        let offspring_id = format!("offspring_gen{}_{}", self.generation, Uuid::new_v4());
        // Perform genetic crossover
        let mut offspring = self
            .crossover(parent_a, parent_b, offspring_id.clone())
            .await?;
        // Apply mutations
        let mutations = self.mutate(&mut offspring).await?;
        // Calculate fitness
        offspring.fitness_score = self.calculate_genesis_fitness(&offspring).await?;
        offspring.generation = self.generation;
        let lineage = GeneticLineage {
            parents: vec![parent_a.id.clone(), parent_b.id.clone()],
            generation: self.generation,
            mutations,
            crossovers: vec![CrossoverEvent {
                parent_a: parent_a.id.clone(),
                parent_b: parent_b.id.clone(),
                crossover_points: vec![1], // Simple single-point crossover
                timestamp: Utc::now(),
            }],
            created_at: Utc::now(),
        let success = offspring.fitness_score >= self.config.fitness_threshold;
        if success {
            self.stats.successful_spawns += 1;
        } else {
            self.stats.failed_spawns += 1;
        Ok(GenesisSpawnResult {
            genetics: offspring,
            lineage,
            success,
            metrics: HashMap::new(),
            messages: vec![format!(
                "Offspring created from parents: {} x {}",
                parent_a.id, parent_b.id
            )],
        })
    /// Perform genetic crossover between two parents
    async fn crossover(
        parent_a: &`BearDog`Genetics,
        parent_b: &`BearDog`Genetics,
        offspring_id: String,
    ) -> BearDogResult<`BearDog`Genetics> {
        let mut offspring = `BearDog`Genetics {
            id: offspring_id,
            created_at: Some(Utc::now()),
        // Crossover capabilities (take from both parents)
        let mut capabilities = parent_a.capabilities.clone();
        for cap in &parent_b.capabilities {
            if !capabilities.contains(cap) && rand::random::<f64>() < self.config.crossover_rate {
                capabilities.push(cap.clone());
        offspring.capabilities = capabilities;
        // Inherit security clearance from fitter parent
        offspring.security_clearance = if parent_a.fitness_score >= parent_b.fitness_score {
            parent_a.security_clearance.clone()
            parent_b.security_clearance.clone()
        Ok(offspring)
    /// Apply mutations to genetics}


    async fn mutate(&self, genetics: &mut `BearDog`Genetics) -> BearDogResult<Vec<MutationEvent>> {
        let mut mutations = Vec::new();
        // Capability mutation
        if rand::random::<f64>() < self.config.mutation_rate {
            let new_capability = match rand::random::<usize>() % 5 {
                0 => GeneticsCapability::new("Computation".to_string(), 0.7, 0.8),
                1 => GeneticsCapability::new("Storage".to_string(), 0.7, 0.8),
                2 => GeneticsCapability::new("Networking".to_string(), 0.7, 0.8),
                3 => GeneticsCapability::new("Monitoring".to_string(), 0.7, 0.8),
                _ => GeneticsCapability::new("Security".to_string(), 0.7, 0.8),
            let node_capability = new_capability.to_node_capability();
            if !genetics.capabilities.contains(&node_capability) {
                genetics.capabilities.push(node_capability);
                genetics.mutation_count += 1;
                mutations.push(MutationEvent {
                    mutation_type: "capability_addition".to_string(),
                    strength: self.config.mutation_rate,
                    component: format!("{:?}", new_capability),
                    timestamp: Utc::now(),
                });
        // Security clearance mutation (rare)
        if rand::random::<f64>() < self.config.mutation_rate * 0.1 {
            genetics.security_clearance = match rand::random::<usize>() % 4 {
                0 => SecurityClearance::Basic,
                1 => SecurityClearance::Standard,
                2 => SecurityClearance::Enhanced,
                _ => SecurityClearance::Maximum,
            mutations.push(MutationEvent {
                mutation_type: "security_clearance_change".to_string(),
                strength: self.config.mutation_rate * 0.1,
                component: "security_clearance".to_string(),
        Ok(mutations)
    /// Update population statistics
    async fn update_population_stats(&mut self) -> BearDogResult<()> {
            return Ok(());
        // Calculate average and best fitness
        let total_fitness: f64 = self.population.iter().map(|g| g.fitness_score).sum();
        self.stats.average_fitness = total_fitness / self.population.len() as f64;
        self.stats.best_fitness = self
            .population
            .iter()
            .map(|g| g.fitness_score)
            .fold(0.0, f64::max);
        // Calculate genetic diversity (simplified)
        let unique_capability_sets: std::collections::HashSet<_> = self
            .map(|g| g.capabilities.clone())
            .collect();
        self.stats.genetic_diversity =
            unique_capability_sets.len() as f64 / self.population.len() as f64;
        debug!(
            "Population stats updated: avg_fitness={:.3}, best_fitness={:.3}, diversity={:.3}",
            self.stats.average_fitness, self.stats.best_fitness, self.stats.genetic_diversity
        Ok(())
    /// Collect metrics for Genesis genetics
    async fn collect_genesis_metrics(&self, genetics: &`BearDog`Genetics) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        metrics.insert(
            "capability_count".to_string(),
            genetics.capabilities.len() as f64,
        metrics.insert("generation".to_string(), genetics.generation as f64);
        metrics.insert("fitness_score".to_string(), genetics.fitness_score);
        metrics.insert("mutation_count".to_string(), genetics.mutation_count as f64);
        let security_score = match genetics.security_clearance {
            SecurityClearance::Basic => 1.0,
            SecurityClearance::Standard => 2.0,
            SecurityClearance::Enhanced => 3.0,
            SecurityClearance::Maximum => 4.0,
        metrics.insert("security_score".to_string(), security_score);
        metrics
    /// Get current Genesis statistics}


    pub fn get_stats(&self) -> &GenesisStats {
        &self.stats
    /// Get current generation number
    pub fn current_generation(&self) -> u32 {
        self.generation
    /// Get population size}


    pub fn population_size(&self) -> usize {
        self.population.len()
    /// Check if evolution should continue
    pub fn should_continue_evolution(&self) -> bool {
        self.generation < self.config.max_generations
            && self.stats.genetic_diversity >= self.config.min_genetic_diversity
            && self.stats.best_fitness < 0.95 // Stop if near-perfect fitness achieved
    /// Export current population for analysis}


    pub fn export_population(&self) -> Vec<`BearDog`Genetics> {
        self.population.clone()
impl Default for GenesisSpawningEngine {
        Self::new()
