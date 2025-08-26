

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisConfig {

    pub initial_population_size: usize,

    pub min_genetic_diversity: f64,

    pub max_generations: u32,

    pub mutation_rate: f64,

    pub crossover_rate: f64,

    pub elite_preservation: f64,

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

pub struct GenesisSpawningEngine {
    config: GenesisConfig,
    population: Vec<`BearDog`Genetics>,
    generation: u32,
    stats: GenesisStats,

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

pub struct GenesisSpawnResult {

    pub genetics: `BearDog`Genetics,

    pub generation: u32,

    pub lineage: GeneticLineage,

    pub success: bool,

    pub metrics: HashMap<String, f64>,

    pub messages: Vec<String>,

pub struct GeneticLineage {

    pub parents: Vec<String>,

    pub mutations: Vec<MutationEvent>,

    pub crossovers: Vec<CrossoverEvent>,

    pub created_at: DateTime<Utc>,

pub struct MutationEvent {

    pub mutation_type: String,

    pub strength: f64,

    pub component: String,

    pub timestamp: DateTime<Utc>,

pub struct CrossoverEvent {

    pub parent_a: String,

    pub parent_b: String,

    pub crossover_points: Vec<usize>,

impl GenesisSpawningEngine {

    pub fn new() -> Self {
            config: GenesisConfig::default(),
            population: Vec::new(),
            generation: 0,
            stats: GenesisStats::default(),

    pub fn with_config(config: GenesisConfig) -> Self {
            config,

    pub async fn initialize_genesis(&mut self) -> BearDogResult<Vec<GenesisSpawnResult>> {
        info!("🌱 Initializing Genesis - First Generation");
        let mut results = Vec::new();
        let mut created_genetics = Vec::new();
        for i in 0..self.config.initial_population_size {
            let genetics_id = format_args!("genesis_gen0_{:04}", i).to_string();

            let genetics = self
                .create_foundational_genetics(genetics_id.clone(), i)
                .await?;

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
                messages: vec![format_args!("Genesis genetics created: {}", genetics_id).to_string()],
            if result.success {
                created_genetics.push(genetics);
                self.stats.successful_spawns += 1;
            } else {
                self.stats.failed_spawns += 1;
            }
            results.push(result);
            self.stats.total_spawned += 1;

        self.population = created_genetics;
        self.generation = 0;
        self.stats.current_generation = 0;
        self.stats.last_spawn_time = Some(Utc::now());

        self.update_population_stats().await?;
        info!(
            "✅ Genesis initialization complete: {} genetics created",
            self.population.len()
        );
        Ok(results)

    pub async fn evolve_generation(&mut self) -> BearDogResult<Vec<GenesisSpawnResult>> {
        if self.population.is_empty() {
            return Err(beardog_errors::BearDogError::Storage {
                message: "Cannot evolve: no genetics in population".to_string(),
            });
        self.generation += 1;
        info!("🧬 Evolving to generation {}", self.generation);

        let parents = self.select_parents().await?;

        let mut next_generation = Vec::new();
        let target_size = self.config.initial_population_size;
        let elite_count = (target_size as f64 * self.config.elite_preservation) as usize;

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
                metrics: HashMap::with_capacity(16),
                messages: vec![format_args!("Elite genetics preserved: rank {}", i + 1).to_string()],
            next_generation.push(genetics.clone());

        while next_generation.len() < target_size {
            let offspring_result = self.create_offspring(&parents).await?;
            if offspring_result.success {
                next_generation.push(offspring_result.genetics.clone());
            results.push(offspring_result);

        self.population = next_generation;
        self.stats.current_generation = self.generation;
            "✅ Generation {} evolution complete: {} genetics",
            self.generation,

    async fn create_foundational_genetics(
        &self,
        id: &str,
        index: usize,
    ) -> GeneticsResult<`BearDog`Genetics> {

        let base_capabilities = self.generate_diverse_capabilities(index).await?;

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

        genetics.mutation_count = 0;
        genetics.generation = 0;
        genetics.created_at = Some(Utc::now());
        Ok(genetics)

    async fn generate_diverse_capabilities(
    ) -> BearDogResult<Vec<GeneticsCapability>> {
        let all_capabilities = vec![
            GeneticsCapability::new("Computation".to_string(), 0.7, 0.8),
            GeneticsCapability::new("Storage".to_string(), 0.7, 0.8),
            GeneticsCapability::new("Networking".to_string(), 0.7, 0.8),
            GeneticsCapability::new("Monitoring".to_string(), 0.7, 0.8),
            GeneticsCapability::new("Security".to_string(), 0.7, 0.8),
        ];

        let capabilities = match index % 5 {
            0 => vec![GeneticsCapability::new("Computation".to_string(), 0.7, 0.8), GeneticsCapability::new("Security".to_string(), 0.7, 0.8)],
            1 => vec![GeneticsCapability::new("Storage".to_string(), 0.7, 0.8), GeneticsCapability::new("Networking".to_string(), 0.7, 0.8)],
            2 => vec![GeneticsCapability::new("Monitoring".to_string(), 0.7, 0.8), GeneticsCapability::new("Computation".to_string(), 0.7, 0.8)],
            3 => vec![GeneticsCapability::new("Security".to_string(), 0.7, 0.8), GeneticsCapability::new("Storage".to_string(), 0.7, 0.8)],
            _ => vec![GeneticsCapability::new("Networking".to_string(), 0.7, 0.8), GeneticsCapability::new("Monitoring".to_string(), 0.7, 0.8)],
        Ok(capabilities)

    async fn calculate_genesis_fitness(&self, genetics: &`BearDog`Genetics) -> BearDogResult<f64> {
        let mut fitness = 0.0;

        fitness += genetics.capabilities.len() as f64 * 0.2;

        fitness += match genetics.security_clearance {
            SecurityClearance::Basic => 0.1,
            SecurityClearance::Standard => 0.2,
            SecurityClearance::Enhanced => 0.3,
            SecurityClearance::Maximum => 0.4,

        fitness += 0.3; // Genesis genetics start healthy

        Ok(fitness.min(1.0))

    async fn select_parents(&self) -> BearDogResult<Vec<`BearDog`Genetics>> {
        if self.population.len() < 2 {
            return Err(beardog_errors::BearDogError::Genetics {
                message: "Not enough genetics for parent selection".to_string(),

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

            tournament.sort_by(|a, b| {
                b.fitness_score
                    .partial_cmp(&a.fitness_score)
                    .unwrap_or(std::cmp::Ordering::Equal) // Handle NaN by treating as equal
            if let Some(winner) = tournament.first() {
                parents.push(winner.clone());
        Ok(parents)

    async fn create_offspring(
        parents: &[`BearDog`Genetics],
    ) -> BearDogResult<GenesisSpawnResult> {
        if parents.len() < 2 {
                message: "Need at least 2 parents for crossover".to_string(),

        let parent_a = &parents[rand::random::<usize>() % parents.len()];
        let parent_b = &parents[rand::random::<usize>() % parents.len()];
        let offspring_id = format_args!("offspring_gen{}_{}", self.generation, Uuid::new_v4().to_string());

        let mut offspring = self
            .crossover(parent_a, parent_b, offspring_id.clone())
            .await?;

        let mutations = self.mutate(&mut offspring).await?;

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
            metrics: HashMap::with_capacity(16),
            messages: vec![format!(
                "Offspring created from parents: {} x {}",
                parent_a.id, parent_b.id
            )],
        })

    async fn crossover(
        parent_a: &`BearDog`Genetics,
        parent_b: &`BearDog`Genetics,
        offspring_id: &str,
    ) -> BearDogResult<`BearDog`Genetics> {
        let mut offspring = `BearDog`Genetics {
            id: offspring_id,
            created_at: Some(Utc::now()),

        let mut capabilities = parent_a.capabilities.clone();
        for cap in &parent_b.capabilities {
            if !capabilities.contains(cap) && rand::random::<f64>() < self.config.crossover_rate {
                capabilities.push(cap.clone());
        offspring.capabilities = capabilities;

        offspring.security_clearance = if parent_a.fitness_score >= parent_b.fitness_score {
            parent_a.security_clearance.clone()
            parent_b.security_clearance.clone()
        Ok(offspring)

    async fn mutate(&self, genetics: &mut `BearDog`Genetics) -> BearDogResult<Vec<MutationEvent>> {
        let mut mutations = Vec::new();

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
                    component: format_args!("{:?}", new_capability).to_string(),
                    timestamp: Utc::now(),
                });

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

    async fn update_population_stats(&mut self) -> BearDogResult<()> {
            return Ok(());

        let total_fitness: f64 = self.population.iter().map(|g| g.fitness_score).sum();
        self.stats.average_fitness = total_fitness / self.population.len() as f64;
        self.stats.best_fitness = self
            .population
            .iter()
            .map(|g| g.fitness_score)
            .fold(0.0, f64::max);

        let unique_capability_sets: std::collections::HashSet<_> = self
            .map(|g| g.capabilities.clone())
            .collect();
        self.stats.genetic_diversity =
            unique_capability_sets.len() as f64 / self.population.len() as f64;
        debug!(
            "Population stats updated: avg_fitness={:.3}, best_fitness={:.3}, diversity={:.3}",
            self.stats.average_fitness, self.stats.best_fitness, self.stats.genetic_diversity
        Ok(())

    async fn collect_genesis_metrics(&self, genetics: &`BearDog`Genetics) -> HashMap<String, f64> {
        let mut metrics = HashMap::with_capacity(16);
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

    pub fn get_stats(&self) -> &GenesisStats {
        &self.stats

    pub fn current_generation(&self) -> u32 {
        self.generation

    pub fn population_size(&self) -> usize {
        self.population.len()

    pub fn should_continue_evolution(&self) -> bool {
        self.generation < self.config.max_generations
            && self.stats.genetic_diversity >= self.config.min_genetic_diversity
            && self.stats.best_fitness < 0.95 // Stop if near-perfect fitness achieved

    pub fn export_population(&self) -> Vec<`BearDog`Genetics> {
        self.population.clone()
impl Default for GenesisSpawningEngine {
        Self::new()
