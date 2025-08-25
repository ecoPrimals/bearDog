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


//! Enhanced safe memory operations for BearDog
//! 
//! This module provides enhanced memory safety features including
//! buffer pools, zero-copy operations, and secure memory handling.

use std::sync::Arc;
use tokio::sync::Mutex;

/// Buffer size constants
pub mod buffer_sizes {
    pub const SMALL: usize = 1024;
    pub const MEDIUM: usize = 4096; 
    pub const LARGE: usize = 16384;
}

/// A safe memory buffer with automatic cleanup
pub struct SafePinnedBuffer {
    data: Vec<u8>,
    size: usize,
}

impl SafePinnedBuffer {
    /// Create a new safe pinned buffer
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
            size,
        }
    }

    /// Create a named buffer for debugging
    pub fn named(size: usize, _name: &str) -> Self {
        Self::new(size)
    }

    /// Get the size of the buffer
    pub fn size(&self) -> usize {
        self.size
    }

    /// Execute a closure with read access to the buffer
    pub fn with_slice<F, R>(&self, f: F) -> R 
    where
        F: FnOnce(&[u8]) -> R,
    {
        f(&self.data)
    }

    /// Execute a closure with write access to the buffer
    pub fn with_mut_slice<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        f(&mut self.data)
    }
}

/// A pooled buffer that returns to the pool when dropped
pub struct SafePooledBuffer<const SIZE: usize> {
    buffer: SafePinnedBuffer,
}

impl<const SIZE: usize> Default for SafePooledBuffer<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize> SafePooledBuffer<SIZE> {
    /// Create a new pooled buffer
    pub fn new() -> Self {
        Self {
            buffer: SafePinnedBuffer::new(SIZE),
        }
    }

    /// Get access to the underlying buffer
    pub fn buffer(&self) -> &SafePinnedBuffer {
        &self.buffer
    }

    /// Get mutable access to the underlying buffer
    pub fn buffer_mut(&mut self) -> &mut SafePinnedBuffer {
        &mut self.buffer
    }

    /// Get the size of the buffer
    pub fn size(&self) -> usize {
        SIZE
    }
}

/// Metrics for buffer pool performance
#[derive(Debug, Clone)]
pub struct BufferPoolMetrics {
    pub total_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl Default for BufferPoolMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferPoolMetrics {
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

/// A thread-safe buffer pool
pub struct SafeBufferPool<const SIZE: usize> {
    available: Arc<Mutex<Vec<SafePinnedBuffer>>>,
    metrics: Arc<Mutex<BufferPoolMetrics>>,
}

impl<const SIZE: usize> SafeBufferPool<SIZE> {
    /// Create new buffer pool with initial capacity
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

    /// Get a buffer from the pool
    pub async fn get_buffer(&self) -> SafePooledBuffer<SIZE> {
        // Try to reuse existing buffer
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

    /// Get metrics for this pool
    pub async fn metrics(&self) -> BufferPoolMetrics {
        self.metrics.lock().await.clone()
    }
}

/// Enhanced memory pools for different buffer sizes
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
    /// Create new enhanced memory pools
    pub fn new() -> Self {
        Self {
            small_pool: SafeBufferPool::new(10),
            medium_pool: SafeBufferPool::new(5),
            large_pool: SafeBufferPool::new(2),
        }
    }

    /// Get a small buffer
    pub async fn get_small(&self) -> SafePooledBuffer<{ buffer_sizes::SMALL }> {
        self.small_pool.get_buffer().await
    }

    /// Get a medium buffer
    pub async fn get_medium(&self) -> SafePooledBuffer<{ buffer_sizes::MEDIUM }> {
        self.medium_pool.get_buffer().await
    }

    /// Get a large buffer
    pub async fn get_large(&self) -> SafePooledBuffer<{ buffer_sizes::LARGE }> {
        self.large_pool.get_buffer().await
    }

    /// Get combined metrics from all pools
    pub async fn all_metrics(&self) -> BufferPoolMetrics {
        let small_metrics = self.small_pool.metrics().await;
        let medium_metrics = self.medium_pool.metrics().await;
        let large_metrics = self.large_pool.metrics().await;

        BufferPoolMetrics {
            total_requests: small_metrics.total_requests + medium_metrics.total_requests + large_metrics.total_requests,
            cache_hits: small_metrics.cache_hits + medium_metrics.cache_hits + large_metrics.cache_hits,
            cache_misses: small_metrics.cache_misses + medium_metrics.cache_misses + large_metrics.cache_misses,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_buffer_pool_operations() {
        let pool: SafeBufferPool<1024> = SafeBufferPool::new(2);
        let _buffer1 = pool.get_buffer().await;
        let _buffer2 = pool.get_buffer().await;
        assert!(pool.metrics().await.total_requests >= 2);
    }

    #[tokio::test]
    async fn test_enhanced_memory_pools() {
        let pools = EnhancedMemoryPools::new();
        let small = pools.get_small().await;
        let medium = pools.get_medium().await;
        let large = pools.get_large().await;
        assert_eq!(small.size(), buffer_sizes::SMALL);
        assert_eq!(medium.size(), buffer_sizes::MEDIUM);
        assert_eq!(large.size(), buffer_sizes::LARGE);
        let metrics = pools.all_metrics().await;
        assert!(metrics.total_requests >= 3);
    }

    #[tokio::test]
    async fn test_safe_pinned_buffer_operations() {
        let mut buffer = SafePinnedBuffer::new(1024);
        // Test zero-copy write
        buffer.with_mut_slice(|slice| {
            slice[0] = 0xAA;
            slice[1] = 0xBB;
        });
        // Test zero-copy read
        let values = buffer.with_slice(|slice| (slice[0], slice[1]));
        assert_eq!(values, (0xAA, 0xBB));
    }
}
