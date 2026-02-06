// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use std::sync::Arc;
use tokio::sync::Mutex;

// Re-export buffer size constants from centralized location
pub use beardog_types::constants::domains::buffers::{
    BUFFER_SIZE_LARGE as LARGE, BUFFER_SIZE_MEDIUM as MEDIUM, BUFFER_SIZE_SMALL as SMALL,
};

pub mod buffer_sizes {
    // Re-export for backward compatibility
    pub use beardog_types::constants::domains::buffers::{
        BUFFER_SIZE_LARGE as LARGE, BUFFER_SIZE_MEDIUM as MEDIUM, BUFFER_SIZE_SMALL as SMALL,
    };
}

/// Global buffer pool manager for reusable memory buffers
///
/// This is a stub implementation. Full buffer pooling will be implemented in future.
pub struct GlobalBufferPools {
    _marker: std::marker::PhantomData<()>,
}

impl GlobalBufferPools {
    /// Creates a new buffer pool
    #[must_use]
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    /// Get a medium-sized buffer (4KB)
    #[must_use]
    pub fn get_medium(&self) -> SafePinnedBuffer {
        SafePinnedBuffer::new(4096)
    }

    /// Get a large buffer (64KB)
    #[must_use]
    pub fn get_large(&self) -> SafePinnedBuffer {
        SafePinnedBuffer::new(65536)
    }

    /// Get a small buffer (1KB)
    #[must_use]
    pub fn get_small(&self) -> SafePinnedBuffer {
        SafePinnedBuffer::new(1024)
    }
}

impl Default for GlobalBufferPools {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SafePinnedBuffer {
    data: Vec<u8>,
    size: usize,
}

impl SafePinnedBuffer {
    /// Creates a new instance
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
            size,
        }
    }

    /// Creates a buffer from existing Vec
    #[must_use]
    pub fn from_vec(data: Vec<u8>) -> Self {
        let size = data.len();
        Self { data, size }
    }

    /// Execute a closure with access to buffer contents
    pub fn with_buffer<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&[u8]) -> R,
    {
        f(&self.data)
    }

    #[must_use]
    pub fn named(size: usize, _name: &str) -> Self {
        Self::new(size)
    }

    #[must_use]
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Creates instance with slice
    pub fn with_slice<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&[u8]) -> R,
    {
        f(&self.data)
    }

    /// Creates instance with mut slice
    pub fn with_mut_slice<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        f(&mut self.data)
    }
}

pub struct SafePooledBuffer<const SIZE: usize> {
    buffer: SafePinnedBuffer,
}

impl<const SIZE: usize> Default for SafePooledBuffer<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize> SafePooledBuffer<SIZE> {
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            buffer: SafePinnedBuffer::new(SIZE),
        }
    }

    #[must_use]
    pub const fn buffer(&self) -> &SafePinnedBuffer {
        &self.buffer
    }

    /// Returns mutable reference to buffer
    pub fn buffer_mut(&mut self) -> &mut SafePinnedBuffer {
        &mut self.buffer
    }

    #[must_use]
    pub const fn size(&self) -> usize {
        SIZE
    }
}

#[derive(Debug, Clone)]
pub struct BufferPoolMetrics {
    /// Number of `total_requests`
    pub total_requests: u64,
    /// Number of `cache_hits`
    pub cache_hits: u64,
    /// Number of `cache_misses`
    pub cache_misses: u64,
}

impl Default for BufferPoolMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferPoolMetrics {
    /// Creates a new instance
    #[must_use]
    pub const fn new() -> Self {
        Self {
            total_requests: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    #[must_use]
    pub fn hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.cache_hits as f64 / self.total_requests as f64
        }
    }
}

pub struct SafeBufferPool<const SIZE: usize> {
    available: Arc<Mutex<Vec<SafePinnedBuffer>>>,
    metrics: Arc<Mutex<BufferPoolMetrics>>,
}

impl<const SIZE: usize> SafeBufferPool<SIZE> {
    /// Creates a new instance
    #[must_use]
    pub fn new(initial_capacity: usize) -> Self {
        let mut available = Vec::with_capacity(initial_capacity);
        for _ in 0..initial_capacity {
            available.push(SafePinnedBuffer::new(SIZE));
        }

        Self {
            available: Arc::new(Mutex::new(available)),
            metrics: Arc::new(Mutex::new(BufferPoolMetrics::new())),
        }
    }

    /// Gets buffer
    /// Gets buffer
    pub async fn get_buffer(&self) -> SafePooledBuffer<SIZE> {
        if let Some(_buffer) = self.available.lock().await.pop() {
            let mut metrics = self.metrics.lock().await;
            metrics.total_requests += 1;
            metrics.cache_hits += 1;
        } else {
            let mut metrics = self.metrics.lock().await;
            metrics.total_requests += 1;
            metrics.cache_misses += 1;
        }

        SafePooledBuffer::new()
    }

    pub async fn metrics(&self) -> BufferPoolMetrics {
        self.metrics.lock().await.clone()
    }
}

pub struct EnhancedMemoryPools {
    small_pool: SafeBufferPool<{ buffer_sizes::SMALL }>,
    medium_pool: SafeBufferPool<{ buffer_sizes::MEDIUM }>,
    large_pool: SafeBufferPool<{ buffer_sizes::LARGE }>,
}

impl Default for EnhancedMemoryPools {
    fn default() -> Self {
        Self::new()
    }
}

impl EnhancedMemoryPools {
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            small_pool: SafeBufferPool::new(10),
            medium_pool: SafeBufferPool::new(5),
            large_pool: SafeBufferPool::new(2),
        }
    }

    /// Gets small
    /// Gets small
    pub async fn get_small(&self) -> SafePooledBuffer<{ buffer_sizes::SMALL }> {
        self.small_pool.get_buffer().await
    }

    /// Gets medium
    /// Gets medium
    pub async fn get_medium(&self) -> SafePooledBuffer<{ buffer_sizes::MEDIUM }> {
        self.medium_pool.get_buffer().await
    }

    /// Gets large
    /// Gets large
    pub async fn get_large(&self) -> SafePooledBuffer<{ buffer_sizes::LARGE }> {
        self.large_pool.get_buffer().await
    }

    pub async fn all_metrics(&self) -> BufferPoolMetrics {
        let small_metrics = self.small_pool.metrics().await;
        let medium_metrics = self.medium_pool.metrics().await;
        let large_metrics = self.large_pool.metrics().await;

        BufferPoolMetrics {
            total_requests: small_metrics.total_requests
                + medium_metrics.total_requests
                + large_metrics.total_requests,
            cache_hits: small_metrics.cache_hits
                + medium_metrics.cache_hits
                + large_metrics.cache_hits,
            cache_misses: small_metrics.cache_misses
                + medium_metrics.cache_misses
                + large_metrics.cache_misses,
        }
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_buffer_pool_operations() {
        let pool: SafeBufferPool<1024> = SafeBufferPool::new(2);
        let _buffer1 = pool.get_buffer().await;
        let _buffer2 = pool.get_buffer().await;
        assert!(pool.metrics().await.total_requests >= 2);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    }

    #[tokio::test]
    async fn test_enhanced_memory_pools() {
        let pools = EnhancedMemoryPools::new();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let small = pools.get_small().await;
        let medium = pools.get_medium().await;
        let large = pools.get_large().await;
        assert_eq!(small.size(), buffer_sizes::SMALL);
        assert_eq!(medium.size(), buffer_sizes::MEDIUM);
        assert_eq!(large.size(), buffer_sizes::LARGE);
        let metrics = pools.all_metrics().await;
        assert!(metrics.total_requests >= 3);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_safe_pinned_buffer_operations() {
        let mut buffer = SafePinnedBuffer::new(1024);

        buffer.with_mut_slice(|slice| {
            slice[0] = 0xAA;
            slice[1] = 0xBB;
        });

        let values = buffer.with_slice(|slice| (slice[0], slice[1]));
        assert_eq!(values, (0xAA, 0xBB));
    }
}
