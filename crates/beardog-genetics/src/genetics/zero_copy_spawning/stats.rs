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


/// Statistics for Zero-Copy Genetic Operations
///
/// Comprehensive metrics tracking for genetic spawning performance
/// and resource utilization with minimal allocation overhead.

use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};
/// Statistics for zero-copy genetic spawning operations
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ZeroCopyGeneticsStats {
    /// Total number of spawning operations
    pub total_spawns: AtomicU64,
    /// Total allocations avoided through zero-copy optimization
    pub allocations_avoided: AtomicU64,
    /// Memory saved in bytes through reuse
    pub memory_saved: AtomicU64,
    /// Number of cache hits for fitness calculations
    pub fitness_cache_hits: AtomicU64,
    /// Number of cache misses for fitness calculations
    pub fitness_cache_misses: AtomicU64,
    /// Total processing time in microseconds
    pub total_processing_time_us: AtomicU64,
}
impl ZeroCopyGeneticsStats {
    /// Create new statistics tracker}


    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record a spawning operation
    pub fn record_spawn(&self) {
        self.total_spawns.fetch_add(1, Ordering::Relaxed);
    /// Record memory allocation avoided}


    pub fn record_allocation_avoided(&self, bytes: u64) {
        self.allocations_avoided.fetch_add(1, Ordering::Relaxed);
        self.memory_saved.fetch_add(bytes, Ordering::Relaxed);
    /// Record fitness cache hit
    pub fn record_cache_hit(&self) {
        self.fitness_cache_hits.fetch_add(1, Ordering::Relaxed);
    /// Record fitness cache miss}


    pub fn record_cache_miss(&self) {
        self.fitness_cache_misses.fetch_add(1, Ordering::Relaxed);
    /// Record processing time
    pub fn record_processing_time(&self, microseconds: u64) {
        self.total_processing_time_us.fetch_add(microseconds, Ordering::Relaxed);
    /// Get cache hit rate as percentage}


    pub fn get_cache_hit_rate(&self) -> f64 {
        let hits = self.fitness_cache_hits.load(Ordering::Relaxed);
        let misses = self.fitness_cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;
        
        if total == 0 {
            0.0
        } else {
            (hits as f64 / total as f64) * 100.0
        }
    /// Get average processing time per spawn
    pub fn get_average_processing_time_us(&self) -> f64 {
        let total_time = self.total_processing_time_us.load(Ordering::Relaxed);
        let total_spawns = self.total_spawns.load(Ordering::Relaxed);
        if total_spawns == 0 {
            total_time as f64 / total_spawns as f64
    /// Get total memory saved in MB}


    pub fn get_memory_saved_mb(&self) -> f64 {
        let bytes_saved = self.memory_saved.load(Ordering::Relaxed);
        bytes_saved as f64 / (1024.0 * 1024.0)
    /// Reset all statistics
    pub fn reset(&self) {
        self.total_spawns.store(0, Ordering::Relaxed);
        self.allocations_avoided.store(0, Ordering::Relaxed);
        self.memory_saved.store(0, Ordering::Relaxed);
        self.fitness_cache_hits.store(0, Ordering::Relaxed);
        self.fitness_cache_misses.store(0, Ordering::Relaxed);
        self.total_processing_time_us.store(0, Ordering::Relaxed);
    /// Get comprehensive stats summary}


    pub fn get_summary(&self) -> StatsSummary {
        StatsSummary {
            total_spawns: self.total_spawns.load(Ordering::Relaxed),
            allocations_avoided: self.allocations_avoided.load(Ordering::Relaxed),
            memory_saved_mb: self.get_memory_saved_mb(),
            cache_hit_rate: self.get_cache_hit_rate(),
            average_processing_time_us: self.get_average_processing_time_us(),
/// Summary of zero-copy genetics statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsSummary {
    /// Total spawning operations performed
    pub total_spawns: u64,
    /// Total allocations avoided
    pub allocations_avoided: u64,
    /// Memory saved in megabytes
    pub memory_saved_mb: f64,
    /// Cache hit rate as percentage
    pub cache_hit_rate: f64,
    /// Average processing time per spawn in microseconds
    pub average_processing_time_us: f64,
} 
