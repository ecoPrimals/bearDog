//! Genetics Pool for Zero-Copy Operations
//!
//! Pool for reusing genetic data structures to minimize allocations
//! during genetic spawning, recombination, and analysis operations.

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

impl Default for GeneticsPool {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneticsPool {
    /// Create a new genetics pool
    pub fn new() -> Self {
        Self {
            genetics_pool: RwLock::new(Vec::new()),
            chromosome_pool: RwLock::new(Vec::new()),
            capability_pool: RwLock::new(Vec::new()),
            stats: GeneticsPoolStats::default(),
        }
    }

    /// Get genetics from the reuse pool
    pub async fn get_genetics(&self) -> Option<BearDogGenetics> {
        let mut pool = self.genetics_pool.write().unwrap();

        if let Some(genetics) = pool.pop() {
            self.stats.genetics_reused.fetch_add(1, Ordering::Relaxed);
            Some(genetics)
        } else {
            self.stats
                .genetics_allocated
                .fetch_add(1, Ordering::Relaxed);
            None // Caller needs to create new
        }
    }

    /// Return genetics to the reuse pool
    pub async fn return_genetics(&self, mut genetics: BearDogGenetics) {
        // Clear sensitive data before reuse
        genetics.crypto_chromosomes.clear();
        genetics.capabilities.clear();
        genetics.mutations.clear();

        // Add genetics back to pool if not full
        let mut pool = self.genetics_pool.write().unwrap();
        if pool.len() < self.stats.pool_capacity && pool.len() < 100 {
            pool.push(genetics);
            self.stats.genetics_reused.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Get chromosomes from the reuse pool
    pub async fn get_chromosomes(&self) -> Option<Vec<CryptoChromosome>> {
        let mut pool = self.chromosome_pool.write().unwrap();

        if let Some(chromosomes) = pool.pop() {
            self.stats
                .chromosomes_reused
                .fetch_add(1, Ordering::Relaxed);
            Some(chromosomes)
        } else {
            None
        }
    }

    /// Get capabilities from the reuse pool
    pub async fn get_capabilities(&self) -> Option<Vec<NodeCapability>> {
        let mut pool = self.capability_pool.write().unwrap();
        if let Some(capabilities) = pool.pop() {
            self.stats
                .capabilities_reused
                .fetch_add(1, Ordering::Relaxed);
            Some(capabilities)
        } else {
            self.stats
                .capabilities_allocated
                .fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Return chromosome vector to pool
    pub async fn return_chromosomes(&self, chromosomes: Vec<CryptoChromosome>) {
        if chromosomes.capacity() >= 10 && chromosomes.capacity() <= 1000 {
            let mut pool = self.chromosome_pool.write().unwrap();
            if pool.len() < 20 {
                pool.push(chromosomes);
            }
        }
    }

    /// Get reusable capability vector
    pub async fn get_capability_vector(&self, capacity: usize) -> Vec<NodeCapability> {
        let mut pool = self.capability_pool.write().unwrap();
        if let Some(mut capabilities) = pool.pop() {
            capabilities.clear();
            capabilities.reserve(capacity);
            self.stats
                .capabilities_reused
                .fetch_add(1, Ordering::Relaxed);
            capabilities
        } else {
            self.stats
                .capabilities_allocated
                .fetch_add(1, Ordering::Relaxed);
            Vec::with_capacity(capacity)
        }
    }

    /// Return capability vector to pool
    pub async fn return_capabilities(&self, capabilities: Vec<NodeCapability>) {
        if capabilities.capacity() >= 5 && capabilities.capacity() <= 100 {
            let mut pool = self.capability_pool.write().unwrap();
            if pool.len() < 20 {
                pool.push(capabilities);
            }
        }
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
            ),
            chromosomes_reused: AtomicU64::new(
                self.stats.chromosomes_reused.load(Ordering::Relaxed),
            ),
            capabilities_allocated: AtomicU64::new(
                self.stats.capabilities_allocated.load(Ordering::Relaxed),
            ),
            capabilities_reused: AtomicU64::new(
                self.stats.capabilities_reused.load(Ordering::Relaxed),
            ),
            pool_capacity: 100, // Default pool capacity
        }
    }
}
