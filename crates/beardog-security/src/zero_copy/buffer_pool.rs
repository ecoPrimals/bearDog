// SPDX-License-Identifier: AGPL-3.0-only



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use bytes::BytesMut;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::RwLock;
use tracing::trace;
use super::types::ZeroCopyConfig;
use beardog_errors::BearDogError;

pub struct BufferPool {

    pools: RwLock<HashMap<usize, Vec<BytesMut>>>,

    stats: BufferPoolStats,

    config: ZeroCopyConfig,
}

#[derive(Debug, Clone)]
    /// The deallocations value
    pub deallocations: AtomicU64,
    /// The cache hits value
    pub cache_hits: AtomicU64,
    /// The cache misses value
    pub cache_misses: AtomicU64,
    /// The peak buffers value
    pub peak_buffers: AtomicU64,}

impl BufferPool {

/// New operation.
    /// Creates a new instance
    pub fn new(config: ZeroCopyConfig) -> Self {
        Self {
            pools: RwLock::new(HashMap::with_capacity(16)),
            stats: BufferPoolStats::default(),
            config,
        }
    }

/// Get Buffer operation.
    /// Gets buffer
    pub fn get_buffer(&self, size: usize) -> BytesMut {
        let size_class = self.size_class(size);
        {
            let mut pools = self.pools.write();
            if let Some(pool) = pools.get_mut(&size_class) {
                if let Some(mut buffer) = pool.pop() {
                    buffer.clear();
                    buffer.reserve(size);
                    self.stats.cache_hits.fetch_add(1, Ordering::Relaxed);
                    trace!("Buffer pool hit for size {}", size);
                    return buffer;
                }
            }

        self.stats.cache_misses.fetch_add(1, Ordering::Relaxed);
        self.stats.allocations.fetch_add(1, Ordering::Relaxed);
        trace!("Buffer pool miss for size {}, allocating new", size);
        BytesMut::with_capacity(size_class)

/// Return Buffer operation.
    pub fn return_buffer(&self, buffer: BytesMut) {
        let capacity = buffer.capacity();
        let size_class = self.size_class(capacity);
        if capacity > 1024 * 1024 * 4 {

            self.stats.deallocations.fetch_add(1, Ordering::Relaxed);
            return;
        let mut pools = self.pools.write();
        let pool = pools.entry(size_class).or_insert_with(Vec::new);
        if pool.len() < self.config.buffer_pool_size {
            pool.push(buffer);
            trace!("Returned buffer to pool for size class {}", size_class);
        } else {


    fn size_class(&self, size: usize) -> usize {
        match size {
            0..=1024 => 1024,
            1025..=4096 => 4096,
            4097..=16384 => 16384,
            16385..=65536 => 65536,
            65537..=262144 => 262144,
            262145..=1048576 => 1048576,
            _ => size.div_ceil(1048576) * 1048576, // Round up to MB

/// Get Stats operation.
    /// Gets stats
    pub fn get_stats(&self) -> BufferPoolStats {
        BufferPoolStats {
            allocations: AtomicU64::new(self.stats.allocations.load(Ordering::Relaxed)),
            deallocations: AtomicU64::new(self.stats.deallocations.load(Ordering::Relaxed)),
            cache_hits: AtomicU64::new(self.stats.cache_hits.load(Ordering::Relaxed)),
            cache_misses: AtomicU64::new(self.stats.cache_misses.load(Ordering::Relaxed)),
            peak_buffers: AtomicU64::new(self.stats.peak_buffers.load(Ordering::Relaxed)),
impl Default for BufferPool {}

    fn default() -> Self {
        Self::new(ZeroCopyConfig::default())
