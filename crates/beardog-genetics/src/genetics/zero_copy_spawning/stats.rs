

use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ZeroCopyGeneticsStats {

    pub total_spawns: AtomicU64,

    pub allocations_avoided: AtomicU64,

    pub memory_saved: AtomicU64,

    pub fitness_cache_hits: AtomicU64,

    pub fitness_cache_misses: AtomicU64,

    pub total_processing_time_us: AtomicU64,
}
impl ZeroCopyGeneticsStats {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_spawn(&self) {
        self.total_spawns.fetch_add(1, Ordering::Relaxed);

    pub fn record_allocation_avoided(&self, bytes: u64) {
        self.allocations_avoided.fetch_add(1, Ordering::Relaxed);
        self.memory_saved.fetch_add(bytes, Ordering::Relaxed);

    pub fn record_cache_hit(&self) {
        self.fitness_cache_hits.fetch_add(1, Ordering::Relaxed);

    pub fn record_cache_miss(&self) {
        self.fitness_cache_misses.fetch_add(1, Ordering::Relaxed);

    pub fn record_processing_time(&self, microseconds: u64) {
        self.total_processing_time_us.fetch_add(microseconds, Ordering::Relaxed);

    pub fn get_cache_hit_rate(&self) -> f64 {
        let hits = self.fitness_cache_hits.load(Ordering::Relaxed);
        let misses = self.fitness_cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;
        
        if total == 0 {
            0.0
        } else {
            (hits as f64 / total as f64) * 100.0
        }

    pub fn get_average_processing_time_us(&self) -> f64 {
        let total_time = self.total_processing_time_us.load(Ordering::Relaxed);
        let total_spawns = self.total_spawns.load(Ordering::Relaxed);
        if total_spawns == 0 {
            total_time as f64 / total_spawns as f64

    pub fn get_memory_saved_mb(&self) -> f64 {
        let bytes_saved = self.memory_saved.load(Ordering::Relaxed);
        bytes_saved as f64 / (1024.0 * 1024.0)

    pub fn reset(&self) {
        self.total_spawns.store(0, Ordering::Relaxed);
        self.allocations_avoided.store(0, Ordering::Relaxed);
        self.memory_saved.store(0, Ordering::Relaxed);
        self.fitness_cache_hits.store(0, Ordering::Relaxed);
        self.fitness_cache_misses.store(0, Ordering::Relaxed);
        self.total_processing_time_us.store(0, Ordering::Relaxed);

    pub fn get_summary(&self) -> StatsSummary {
        StatsSummary {
            total_spawns: self.total_spawns.load(Ordering::Relaxed),
            allocations_avoided: self.allocations_avoided.load(Ordering::Relaxed),
            memory_saved_mb: self.get_memory_saved_mb(),
            cache_hit_rate: self.get_cache_hit_rate(),
            average_processing_time_us: self.get_average_processing_time_us(),

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsSummary {

    pub total_spawns: u64,

    pub allocations_avoided: u64,

    pub memory_saved_mb: f64,

    pub cache_hit_rate: f64,

    pub average_processing_time_us: f64,
} 
