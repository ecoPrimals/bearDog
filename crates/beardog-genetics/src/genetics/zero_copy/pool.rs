

use beardog_auth::auth::{BearDogGenetics, CryptoChromosome, NodeCapability};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    RwLock,
};

use crate::genetics::peer_to_peer_genetics::GeneticsPoolStats;

#[derive(Debug)]
pub struct GeneticsPool {

    genetics_pool: RwLock<Vec<BearDogGenetics>>,

    chromosome_pool: RwLock<Vec<Vec<CryptoChromosome>>>,

    capability_pool: RwLock<Vec<Vec<NodeCapability>>>,

    stats: GeneticsPoolStats,
}
impl Default for GeneticsPool {}

    fn default() -> Self {
        Self::new()
    }
impl GeneticsPool {

    pub fn new() -> Self {
        Self {
            genetics_pool: RwLock::new(Vec::new()),
            chromosome_pool: RwLock::new(Vec::new()),
            capability_pool: RwLock::new(Vec::new()),
            stats: GeneticsPoolStats::default(),
        }

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

    pub async fn return_genetics(&self, mut genetics: BearDogGenetics) {

        genetics.crypto_chromosomes.clear();
        genetics.capabilities.clear();
        genetics.mutations.clear();

                tracing::error!("Failed to acquire write lock on genetics pool for recycling");
                return; // Fail gracefully by not recycling
        if pool.len() < self.stats.pool_capacity && pool.len() < 100 {
            pool.push(genetics);

    pub async fn get_chromosomes(&self) -> Option<Vec<CryptoChromosome>> {
        let mut pool = match self.chromosome_pool.write() {
            Err(poisoned) => {
                tracing::warn!("Chromosome pool mutex poisoned, recovering gracefully");
                poisoned.into_inner()
        if let Some(chromosomes) = pool.pop() {
                .chromosomes_reused
            Some(chromosomes)
            None

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
                    reason: format_args!("Operation failed ({}): {:?}", "Genetics capability pool lock is poisoned - this indicates a critical bug", e).to_string(),
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

    pub async fn get_capability_vector(&self, capacity: usize) -> Vec<NodeCapability> {
        let mut pool = match self.capability_pool.write() {
                tracing::warn!("Capability pool mutex poisoned, recovering gracefully");
        if let Some(mut capabilities) = pool.pop() {
            capabilities.clear();
            capabilities.reserve(capacity);
            capabilities
            Vec::with_capacity(capacity)

    pub async fn return_capabilities(&self, capabilities: Vec<NodeCapability>) -> BearDogResult<()> {
        if capabilities.capacity() >= 5 && capabilities.capacity() <= 100 {
            let mut pool = self.capability_pool.write().map_err(|_| {
                BearDogError::internal("Failed to acquire write lock on capability pool")
            })?;
            pool.push(capabilities);
        }
        Ok(())
    }

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
