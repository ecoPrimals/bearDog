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


/// Genetics Pool for Zero-Copy Operations
///
/// Pool for reusing genetic data structures to minimize allocations
/// during genetic spawning, recombination, and analysis operations.

use beardog_auth::auth::{BearDogGenetics, CryptoChromosome, NodeCapability};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    RwLock,
};
// Import from peer_to_peer_genetics module
use crate::genetics::peer_to_peer_genetics::GeneticsPoolStats;
/// Pool for reusing genetic data structures
#[derive(Debug)]
pub struct GeneticsPool {
    /// Pool of reusable genetic structures by type
    genetics_pool: RwLock<Vec<BearDogGenetics>>,
    /// Pool of reusable chromosome vectors
    chromosome_pool: RwLock<Vec<Vec<CryptoChromosome>>>,
    /// Pool of reusable capability vectors
    capability_pool: RwLock<Vec<Vec<NodeCapability>>>,
    /// Pool statistics
    stats: GeneticsPoolStats,
}
impl Default for GeneticsPool {}


    fn default() -> Self {
        Self::new()
    }
impl GeneticsPool {
    /// Create a new genetics pool}


    pub fn new() -> Self {
        Self {
            genetics_pool: RwLock::new(Vec::new()),
            chromosome_pool: RwLock::new(Vec::new()),
            capability_pool: RwLock::new(Vec::new()),
            stats: GeneticsPoolStats::default(),
        }
    /// Get genetics from the reuse pool
    pub async fn get_genetics(&self) -> Option<BearDogGenetics> {
        let mut pool = match self.genetics_pool.write() {
            Ok(pool) => pool,
            Err(_) => {
                tracing::error!("Failed to acquire write lock on genetics pool");
                return None;
            }
        };
        if let Some(genetics) = pool.pop() {
            self.stats.genetics_reused.fetch_add(1, Ordering::Relaxed);
            Some(genetics)
        } else {
            self.stats
                .genetics_allocated
                .fetch_add(1, Ordering::Relaxed);
            None // Caller needs to create new
    /// Return genetics to the reuse pool
    pub async fn return_genetics(&self, mut genetics: BearDogGenetics) {
        // Clear sensitive data before reuse
        genetics.crypto_chromosomes.clear();
        genetics.capabilities.clear();
        genetics.mutations.clear();
        // Add genetics back to pool if not full
                tracing::error!("Failed to acquire write lock on genetics pool for recycling");
                return; // Fail gracefully by not recycling
        if pool.len() < self.stats.pool_capacity && pool.len() < 100 {
            pool.push(genetics);
    /// Get chromosomes from the reuse pool}


    pub async fn get_chromosomes(&self) -> Option<Vec<CryptoChromosome>> {
        let mut pool = match self.chromosome_pool.write() {
            Err(poisoned) => {
                tracing::warn!("Chromosome pool mutex poisoned, recovering gracefully");
                poisoned.into_inner()
        if let Some(chromosomes) = pool.pop() {
                .chromosomes_reused
            Some(chromosomes)
            None
    /// Get capabilities from the reuse pool
    pub async fn get_capabilities(&self) -> Option<Vec<NodeCapability>> {
        let mut pool = self
            .capability_pool
            .write()
            .map_err(|e| {
                tracing::error!(
                    "Operation failed ({}): {:?}",
                    "Genetics capability pool lock is poisoned - this indicates a critical bug",
                    e
                );
                beardog_errors::GeneticsError::InternalError { 
                    reason: format!("Operation failed ({}): {:?}", "Genetics capability pool lock is poisoned - this indicates a critical bug", e),
                    context: create_genetics_context(),
                    metadata: GeneticsMetadata::default(),
                    improvement: None 
                }
            })
            .ok()?;
        if let Some(capabilities) = pool.pop() {
                .capabilities_reused
            Some(capabilities)
                .capabilities_allocated
    /// Return chromosome vector to pool
    pub async fn return_chromosomes(&self, chromosomes: Vec<CryptoChromosome>) {
        if chromosomes.capacity() >= 10 && chromosomes.capacity() <= 1000 {
            let mut pool = match self.chromosome_pool.write() {
                Ok(pool) => pool,
                Err(poisoned) => {
                    tracing::warn!("Chromosome pool mutex poisoned, recovering gracefully");
                    poisoned.into_inner()
            };
            if pool.len() < 20 {
                pool.push(chromosomes);
    /// Get reusable capability vector
    pub async fn get_capability_vector(&self, capacity: usize) -> Vec<NodeCapability> {
        let mut pool = match self.capability_pool.write() {
                tracing::warn!("Capability pool mutex poisoned, recovering gracefully");
        if let Some(mut capabilities) = pool.pop() {
            capabilities.clear();
            capabilities.reserve(capacity);
            capabilities
            Vec::with_capacity(capacity)
    /// Return capability vector to pool}


    pub async fn return_capabilities(&self, capabilities: Vec<NodeCapability>) -> BearDogResult<()> {
        if capabilities.capacity() >= 5 && capabilities.capacity() <= 100 {
            let mut pool = self.capability_pool.write().map_err(|_| {
                BearDogError::internal("Failed to acquire write lock on capability pool")
            })?;
            pool.push(capabilities);
        }
        Ok(())
    }
    /// Get pool statistics
    pub fn get_stats(&self) -> GeneticsPoolStats {
        GeneticsPoolStats {
            genetics_allocated: AtomicU64::new(
                self.stats.genetics_allocated.load(Ordering::Relaxed),
            ),
            genetics_reused: AtomicU64::new(self.stats.genetics_reused.load(Ordering::Relaxed)),
            chromosomes_allocated: AtomicU64::new(
                self.stats.chromosomes_allocated.load(Ordering::Relaxed),
            chromosomes_reused: AtomicU64::new(
                self.stats.chromosomes_reused.load(Ordering::Relaxed),
            capabilities_allocated: AtomicU64::new(
                self.stats.capabilities_allocated.load(Ordering::Relaxed),
            capabilities_reused: AtomicU64::new(
                self.stats.capabilities_reused.load(Ordering::Relaxed),
            pool_capacity: 100, // Default pool capacity
