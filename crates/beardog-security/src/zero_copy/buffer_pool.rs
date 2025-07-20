//! Buffer Pool for Zero-Copy Operations
//!
//! Provides buffer pooling to reduce memory allocations in hot code paths.

use bytes::BytesMut;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::RwLock;
use tracing::trace;

use super::types::ZeroCopyConfig;

/// Buffer pool for reusing memory allocations
pub struct BufferPool {
    /// Pool of reusable buffers by size class
    pools: RwLock<HashMap<usize, Vec<BytesMut>>>,
    /// Statistics for buffer pool usage
    stats: BufferPoolStats,
    /// Configuration
    config: ZeroCopyConfig,
}

/// Statistics for buffer pool performance monitoring
#[derive(Debug, Default)]
pub struct BufferPoolStats {
    pub allocations: AtomicU64,
    pub deallocations: AtomicU64,
    pub cache_hits: AtomicU64,
    pub cache_misses: AtomicU64,
    pub peak_buffers: AtomicU64,
}

impl BufferPool {
    /// Create a new buffer pool
    pub fn new(config: ZeroCopyConfig) -> Self {
        Self {
            pools: RwLock::new(HashMap::new()),
            stats: BufferPoolStats::default(),
            config,
        }
    }

    /// Get a buffer of at least the specified size
    pub async fn get_buffer(&self, size: usize) -> BytesMut {
        let size_class = self.size_class(size);

        {
            let mut pools = self.pools.write().await;
            if let Some(pool) = pools.get_mut(&size_class) {
                if let Some(mut buffer) = pool.pop() {
                    buffer.clear();
                    buffer.reserve(size);
                    self.stats.cache_hits.fetch_add(1, Ordering::Relaxed);
                    trace!("Buffer pool hit for size {}", size);
                    return buffer;
                }
            }
        }

        // Cache miss - create new buffer
        self.stats.cache_misses.fetch_add(1, Ordering::Relaxed);
        self.stats.allocations.fetch_add(1, Ordering::Relaxed);
        trace!("Buffer pool miss for size {}, allocating new", size);
        BytesMut::with_capacity(size_class)
    }

    /// Return a buffer to the pool
    pub async fn return_buffer(&self, buffer: BytesMut) {
        let capacity = buffer.capacity();
        let size_class = self.size_class(capacity);

        if capacity > 1024 * 1024 * 4 {
            // Don't pool very large buffers to prevent excessive memory usage
            self.stats.deallocations.fetch_add(1, Ordering::Relaxed);
            return;
        }

        let mut pools = self.pools.write().await;
        let pool = pools.entry(size_class).or_insert_with(Vec::new);

        if pool.len() < self.config.buffer_pool_size {
            pool.push(buffer);
            trace!("Returned buffer to pool for size class {}", size_class);
        } else {
            self.stats.deallocations.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Determine size class for buffer pooling
    fn size_class(&self, size: usize) -> usize {
        match size {
            0..=1024 => 1024,
            1025..=4096 => 4096,
            4097..=16384 => 16384,
            16385..=65536 => 65536,
            65537..=262144 => 262144,
            262145..=1048576 => 1048576,
            _ => size.div_ceil(1048576) * 1048576, // Round up to MB
        }
    }

    /// Get buffer pool statistics
    pub fn get_stats(&self) -> BufferPoolStats {
        BufferPoolStats {
            allocations: AtomicU64::new(self.stats.allocations.load(Ordering::Relaxed)),
            deallocations: AtomicU64::new(self.stats.deallocations.load(Ordering::Relaxed)),
            cache_hits: AtomicU64::new(self.stats.cache_hits.load(Ordering::Relaxed)),
            cache_misses: AtomicU64::new(self.stats.cache_misses.load(Ordering::Relaxed)),
            peak_buffers: AtomicU64::new(self.stats.peak_buffers.load(Ordering::Relaxed)),
        }
    }
}

impl Default for BufferPool {
    fn default() -> Self {
        Self::new(ZeroCopyConfig::default())
    }
}
