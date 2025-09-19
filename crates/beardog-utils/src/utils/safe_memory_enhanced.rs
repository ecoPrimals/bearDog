// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use std::sync::Arc;
use tokio::sync::Mutex;

pub mod buffer_sizes {
    pub const SMALL: usize = 1024;
    pub const MEDIUM: usize = 4096;
    pub const LARGE: usize = 16384;
}

pub struct SafePinnedBuffer {
    data: Vec<u8>,
    size: usize,
}

impl SafePinnedBuffer {
    /// Creates a new instance
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
            size,
        }
    }

    pub fn named(size: usize, _name: &str) -> Self {
        Self::new(size)
    }

    pub fn size(&self) -> usize {
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
    pub fn new() -> Self {
        Self {
            buffer: SafePinnedBuffer::new(SIZE),
        }
    }

    pub fn buffer(&self) -> &SafePinnedBuffer {
        &self.buffer
    }

    /// Returns mutable reference to buffer
    pub fn buffer_mut(&mut self) -> &mut SafePinnedBuffer {
        &mut self.buffer
    }

    pub fn size(&self) -> usize {
        SIZE
    }
}

#[derive(Debug, Clone)]
pub struct BufferPoolMetrics {
    /// Number of total_requests
    pub total_requests: u64,
    /// Number of cache_hits
    pub cache_hits: u64,
    /// Number of cache_misses
    pub cache_misses: u64,
}

impl Default for BufferPoolMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferPoolMetrics {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    fn test_buffer_pool_operations() {
        let pool: SafeBufferPool<1024> = SafeBufferPool::new(2);
        let _buffer1 = pool.get_buffer();
        let _buffer2 = pool.get_buffer();
        assert!(pool.metrics().total_requests >= 2);
    }

    #[tokio::test]
    fn test_enhanced_memory_pools() {
        let pools = EnhancedMemoryPools::new();
        let small = pools.get_small();
        let medium = pools.get_medium();
        let large = pools.get_large();
        assert_eq!(small.size(), buffer_sizes::SMALL);
        assert_eq!(medium.size(), buffer_sizes::MEDIUM);
        assert_eq!(large.size(), buffer_sizes::LARGE);
        let metrics = pools.all_metrics();
        assert!(metrics.total_requests >= 3);
    }

    #[tokio::test]
    fn test_safe_pinned_buffer_operations() {
        let mut buffer = SafePinnedBuffer::new(1024);

        buffer.with_mut_slice(|slice| {
            slice[0] = 0xAA;
            slice[1] = 0xBB;
        });

        let values = buffer.with_slice(|slice| (slice[0], slice[1]));
        assert_eq!(values, (0xAA, 0xBB));
    }
}
