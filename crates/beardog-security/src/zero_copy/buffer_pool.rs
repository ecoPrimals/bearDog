// SPDX-License-Identifier: AGPL-3.0-only

use super::types::ZeroCopyConfig;
use bytes::BytesMut;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::RwLock;
use tracing::trace;

const BUFFER_POOL_MAX_RETURN_BYTES: usize = 1024 * 1024 * 4;

const BUFFER_SIZE_CLASS_1K: usize = 1024;
const BUFFER_SIZE_CLASS_4K: usize = 4096;
const BUFFER_SIZE_CLASS_16K: usize = 16384;
const BUFFER_SIZE_CLASS_64K: usize = 65536;

const ONE_MEBIBYTE: usize = 1024 * 1024;

#[derive(Debug, Clone)]
pub struct BufferPoolStats {
    pub allocations: AtomicU64,
    pub deallocations: AtomicU64,
    pub cache_hits: AtomicU64,
    pub cache_misses: AtomicU64,
    pub peak_buffers: AtomicU64,
}

impl Default for BufferPoolStats {
    fn default() -> Self {
        Self {
            allocations: AtomicU64::new(0),
            deallocations: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            peak_buffers: AtomicU64::new(0),
        }
    }
}

pub struct BufferPool {
    pools: RwLock<HashMap<usize, Vec<BytesMut>>>,
    stats: BufferPoolStats,
    config: ZeroCopyConfig,
}

impl BufferPool {
    pub fn new(config: ZeroCopyConfig) -> Self {
        Self {
            pools: RwLock::new(HashMap::with_capacity(16)),
            stats: BufferPoolStats::default(),
            config,
        }
    }

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
        }

        self.stats.cache_misses.fetch_add(1, Ordering::Relaxed);
        self.stats.allocations.fetch_add(1, Ordering::Relaxed);
        trace!("Buffer pool miss for size {}, allocating new", size);
        BytesMut::with_capacity(size_class)
    }

    pub fn return_buffer(&self, buffer: BytesMut) {
        let capacity = buffer.capacity();
        let size_class = self.size_class(capacity);
        if capacity > BUFFER_POOL_MAX_RETURN_BYTES {
            self.stats.deallocations.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let mut pools = self.pools.write();
        let pool = pools.entry(size_class).or_insert_with(Vec::new);
        if pool.len() < self.config.buffer_pool_size {
            pool.push(buffer);
            trace!("Returned buffer to pool for size class {}", size_class);
        } else {
            self.stats.deallocations.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn size_class(&self, size: usize) -> usize {
        match size {
            0..=BUFFER_SIZE_CLASS_1K => BUFFER_SIZE_CLASS_1K,
            (BUFFER_SIZE_CLASS_1K + 1)..=BUFFER_SIZE_CLASS_4K => BUFFER_SIZE_CLASS_4K,
            (BUFFER_SIZE_CLASS_4K + 1)..=BUFFER_SIZE_CLASS_16K => BUFFER_SIZE_CLASS_16K,
            (BUFFER_SIZE_CLASS_16K + 1)..=BUFFER_SIZE_CLASS_64K => BUFFER_SIZE_CLASS_64K,
            (BUFFER_SIZE_CLASS_64K + 1)..=262_144 => 262_144,
            262_145..=ONE_MEBIBYTE => ONE_MEBIBYTE,
            _ => size.div_ceil(ONE_MEBIBYTE) * ONE_MEBIBYTE,
        }
    }

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
