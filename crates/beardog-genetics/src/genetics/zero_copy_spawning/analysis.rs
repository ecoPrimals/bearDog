

use std::collections::HashMap;
use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFitnessAnalysis {

    pub fitness_score: f64,

    pub performance_score: f64,

    pub security_score: f64,

    pub adaptability_score: f64,

    pub calculated_at: Instant,

    pub genetic_hash: u64,
}
impl CachedFitnessAnalysis {

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

    pub fn is_valid(&self, max_age_seconds: u64) -> bool {
        self.calculated_at.elapsed().as_secs() < max_age_seconds

    pub fn get_combined_score(&self) -> f64 {
        (self.fitness_score + self.performance_score + self.security_score + self.adaptability_score) / 4.0

pub struct ChunkAnalysis {

    pub chunk_id: String,

    pub chunk_size: usize,

    pub entity_count: u32,

    pub average_fitness: f64,

    pub processing_time_us: u64,}

impl ChunkAnalysis {

    pub fn new(chunk_id: &str, chunk_size: usize, entity_count: u32) -> Self {
            chunk_id,
            chunk_size,
            entity_count,
            average_fitness: 0.0,
            processing_time_us: 0,

    pub fn set_average_fitness(&mut self, average_fitness: f64) {
        self.average_fitness = average_fitness;

    pub fn set_processing_time(&mut self, processing_time_us: u64) {
        self.processing_time_us = processing_time_us;

    pub fn get_processing_efficiency(&self) -> f64 {
        if self.processing_time_us == 0 {
            0.0
        } else {
            self.entity_count as f64 / self.processing_time_us as f64

pub struct PopulationAnalysis {

    pub population_size: u32,

    pub best_fitness: f64,

    pub worst_fitness: f64,

    pub fitness_std_dev: f64,

    pub genetic_diversity: f64,

    pub analyzed_at: Instant,}

impl PopulationAnalysis {

    pub fn new(population_size: u32) -> Self {
            population_size,
            best_fitness: f64::NEG_INFINITY,
            worst_fitness: f64::INFINITY,
            fitness_std_dev: 0.0,
            genetic_diversity: 0.0,
            analyzed_at: Instant::now(),

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

    pub fn set_genetic_diversity(&mut self, diversity: f64) {
        self.genetic_diversity = diversity.max(0.0).min(1.0);

    pub fn get_fitness_range(&self) -> f64 {
        self.best_fitness - self.worst_fitness

    pub fn is_converging(&self, diversity_threshold: f64) -> bool {
        self.genetic_diversity < diversity_threshold

    pub fn get_health_score(&self) -> f64 {

        let fitness_score = if self.best_fitness > 0.0 {
            self.average_fitness / self.best_fitness
        };

        (fitness_score * 0.7 + self.genetic_diversity * 0.3).max(0.0).min(1.0)

#[derive(Debug)]
pub struct FitnessCache {

    cache: HashMap<u64, CachedFitnessAnalysis>,

    max_size: usize,

    hits: u64,

    misses: u64,}

impl FitnessCache {

    pub fn new(max_size: usize) -> Self {
            cache: HashMap::with_capacity(16),
            max_size,
            hits: 0,
            misses: 0,

    pub fn get(&mut self, genetic_hash: u64, max_age_seconds: u64) -> Option<&CachedFitnessAnalysis> {
        if let Some(analysis) = self.cache.get(&genetic_hash) {
            if analysis.is_valid(max_age_seconds) {
                self.hits += 1;
                return Some(analysis);
            } else {

                self.cache.remove(&genetic_hash);
            }
        self.misses += 1;
        None

    pub fn store(&mut self, genetic_hash: u64, analysis: CachedFitnessAnalysis) {

        if self.cache.len() >= self.max_size {
            self.evict_oldest();
        self.cache.insert(genetic_hash, analysis);

    fn evict_oldest(&mut self) {
        if self.cache.is_empty() {
            return;

        if let Some(key) = self.cache.keys().next().copied() {
            self.cache.remove(&key);

    pub fn get_stats(&self) -> (u64, u64, f64, usize) {
        let total_requests = self.hits + self.misses;
        let hit_rate = if total_requests == 0 {
            self.hits as f64 / total_requests as f64 * 100.0
        (self.hits, self.misses, hit_rate, self.cache.len())

    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
} 
