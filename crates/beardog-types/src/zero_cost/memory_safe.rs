// SPDX-License-Identifier: AGPL-3.0-or-later

// Safe Zero-Cost Memory Management for BearDog Types
//
// This module provides memory-efficient operations without unchecked memory patterns,
// using safe Rust patterns and high-performance data structures.

use crossbeam_queue::SegQueue;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Thread-safe size-classed buffer pool with alignment and allocation metrics.
pub struct SafeZeroCopyMemoryPool {
    pools: Arc<RwLock<HashMap<usize, SegQueue<Vec<u8>>>>>,
    alignment: usize,
    total_allocated: AtomicUsize,
    peak_usage: AtomicUsize,
    stats: SafeMemoryPoolMetrics,
}

/// Atomic counters mirroring [`SafeZeroCopyMemoryPool`] hit/miss statistics.
#[derive(Debug, Default)]
pub struct SafeMemoryPoolMetrics {
    /// Total number of memory allocations
    /// The allocations value
    pub allocations: AtomicUsize,
    /// Total number of memory deallocations
    /// The deallocations value
    pub deallocations: AtomicUsize,
    /// The cache hits value
    pub cache_hits: AtomicUsize,
    /// Number of cache misses requiring new allocation
    /// The cache misses value
    pub cache_misses: AtomicUsize,
    /// Peak memory usage in bytes
    /// The peak memory usage value
    pub peak_memory_usage: AtomicUsize,
}

/// Fixed-capacity `SegQueue`-backed ring for producer/consumer workloads.
pub struct SafeRingBuffer<T>
where
    T: Send + Sync,
{
    buffer: SegQueue<T>,
    capacity: usize,
    current_size: AtomicUsize,
}

impl SafeZeroCopyMemoryPool {
    /// Create a new safe memory pool with specified alignment
    /// Creates a new instance
    pub fn new(pool_sizes: &[usize], alignment: usize) -> Self {
        let mut pools = HashMap::new();

        // Initialize pools for each size class
        for &size in pool_sizes {
            pools.insert(size, SegQueue::new());
        }

        Self {
            pools: Arc::new(RwLock::new(pools)),
            alignment,
            total_allocated: AtomicUsize::new(0),
            peak_usage: AtomicUsize::new(0),
            stats: SafeMemoryPoolMetrics::default(),
        }
    }

    /// Allocate a buffer with the specified size
    pub fn allocate(&self, size: usize) -> Vec<u8> {
        let aligned_size = self.align_size(size);

        // Try to get from pool first
        {
            let pools = self.pools.read();
            if let Some(pool) = pools.get(&aligned_size)
                && let Some(mut buffer) = pool.pop()
            {
                buffer.clear();
                buffer.resize(size, 0);
                self.stats.cache_hits.fetch_add(1, Ordering::Relaxed);
                return buffer;
            }
        }

        // Allocate new buffer
        self.stats.cache_misses.fetch_add(1, Ordering::Relaxed);
        self.stats.allocations.fetch_add(1, Ordering::Relaxed);

        let current_allocated = self.total_allocated.fetch_add(size, Ordering::Relaxed) + size;
        let peak = self.peak_usage.load(Ordering::Relaxed);
        if current_allocated > peak {
            self.peak_usage.store(current_allocated, Ordering::Relaxed);
            self.stats
                .peak_memory_usage
                .store(current_allocated, Ordering::Relaxed);
        }

        vec![0u8; size]
    }

    /// Return a buffer to the pool for reuse, or drop it if the pool is full.
    pub fn deallocate(&self, buffer: Vec<u8>) {
        let capacity = buffer.capacity();
        let aligned_size = self.align_size(capacity);

        {
            let pools = self.pools.read();
            if let Some(pool) = pools.get(&aligned_size) {
                // Return to pool if it's not too full
                if pool.len() < 100 {
                    // Limit pool size to prevent memory bloat
                    pool.push(buffer);
                    self.stats.deallocations.fetch_add(1, Ordering::Relaxed);
                    self.total_allocated.fetch_sub(capacity, Ordering::Relaxed);
                    return;
                }
            }
        }

        // Just drop the buffer if pool is full
        self.total_allocated.fetch_sub(capacity, Ordering::Relaxed);
    }

    /// Get current memory usage statistics
    /// Gets metrics
    pub fn get_metrics(&self) -> SafeMemoryPoolMetrics {
        SafeMemoryPoolMetrics {
            allocations: AtomicUsize::new(self.stats.allocations.load(Ordering::Relaxed)),
            deallocations: AtomicUsize::new(self.stats.deallocations.load(Ordering::Relaxed)),
            cache_hits: AtomicUsize::new(self.stats.cache_hits.load(Ordering::Relaxed)),
            cache_misses: AtomicUsize::new(self.stats.cache_misses.load(Ordering::Relaxed)),
            peak_memory_usage: AtomicUsize::new(
                self.stats.peak_memory_usage.load(Ordering::Relaxed),
            ),
        }
    }

    const fn align_size(&self, size: usize) -> usize {
        (size + self.alignment - 1) & !(self.alignment - 1)
    }

    /// Clear all pools and reset metrics
    pub fn clear(&self) {
        let pools = self.pools.write();
        for pool in pools.values() {
            while pool.pop().is_some() {
                // Drain all pools
            }
        }

        self.total_allocated.store(0, Ordering::Relaxed);
        self.peak_usage.store(0, Ordering::Relaxed);
        self.stats.allocations.store(0, Ordering::Relaxed);
        self.stats.deallocations.store(0, Ordering::Relaxed);
        self.stats.cache_hits.store(0, Ordering::Relaxed);
        self.stats.cache_misses.store(0, Ordering::Relaxed);
        self.stats.peak_memory_usage.store(0, Ordering::Relaxed);
    }
}

impl<T> SafeRingBuffer<T>
where
    T: Send + Sync,
{
    /// Create a new thread-safe buffer pool
    #[must_use]
    pub const fn new(capacity: usize) -> Self {
        Self {
            buffer: SegQueue::new(),
            capacity,
            current_size: AtomicUsize::new(0),
        }
    }

    /// Push an item to the buffer (compatibility method)
    pub fn push(&self, item: T) -> bool {
        self.try_push(item).is_ok()
    }

    /// Try to push an item to the buffer
    ///
    /// # Errors
    ///
    /// Returns the item unchanged if the buffer is full.
    pub fn try_push(&self, item: T) -> Result<(), T> {
        let current_size = self.current_size.load(Ordering::Relaxed);
        if current_size >= self.capacity {
            return Err(item); // Buffer is full
        }

        self.buffer.push(item);
        self.current_size.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// Pop an item from the buffer pool
    pub fn pop(&self) -> Option<T> {
        self.buffer.pop().map_or_else(
            || None,
            |item| {
                self.current_size.fetch_sub(1, Ordering::Relaxed);
                Some(item)
            },
        )
    }

    /// Non-blocking pop; equivalent to [`Self::pop`] for API symmetry.
    pub fn try_pop(&self) -> Option<T> {
        self.pop()
    }

    /// Check if the buffer is empty
    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.current_size.load(Ordering::Relaxed) == 0
    }

    /// Check if the buffer is full
    /// Checks if full
    pub fn is_full(&self) -> bool {
        self.current_size.load(Ordering::Relaxed) >= self.capacity
    }

    /// Get the current length
    pub fn len(&self) -> usize {
        self.current_size.load(Ordering::Relaxed)
    }
}

impl<T> Default for SafeRingBuffer<T>
where
    T: Send + Sync,
{
    fn default() -> Self {
        Self::new(1024)
    }
}

/// Safe SIMD capabilities detection
#[derive(Debug, Clone)]
pub struct SafeSimdCapabilities {
    /// Whether AVX2 instructions are available
    /// Whether `avx2_available` is enabled
    pub avx2_available: bool,
    /// Whether SSE4.2 instructions are available
    /// Whether `sse42_available` is enabled
    pub sse42_available: bool,
    /// Native vector register width in bytes used for chunking
    pub vector_width: usize,
}

impl Default for SafeSimdCapabilities {
    fn default() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            Self {
                avx2_available: is_x86_feature_detected!("avx2"),
                sse42_available: is_x86_feature_detected!("sse4.2"),
                vector_width: if is_x86_feature_detected!("avx2") {
                    32
                } else {
                    16
                },
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            Self {
                avx2_available: false,
                sse42_available: false,
                vector_width: 16, // Default for ARM NEON
            }
        }
    }
}

impl SafeSimdCapabilities {
    /// Create new SIMD capabilities
    /// Creates a new instance
    pub fn new() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            Self {
                avx2_available: is_x86_feature_detected!("avx2"),
                sse42_available: is_x86_feature_detected!("sse4.2"),
                vector_width: if is_x86_feature_detected!("avx2") {
                    32
                } else if is_x86_feature_detected!("sse4.2") {
                    16
                } else {
                    8
                },
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            Self {
                avx2_available: false,
                sse42_available: false,
                vector_width: 16, // Default for ARM NEON
            }
        }
    }

    /// Detect SIMD capabilities (compatibility method)
    pub fn detect() -> Self {
        Self::new()
    }

    /// Vectorized hash function using safe operations
    pub fn vectorized_hash(&self, data: &[u8]) -> Vec<u8> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Safe vectorized processing
        let chunk_size = self.vector_width;
        let mut result = Vec::new();

        for chunk in data.chunks(chunk_size) {
            let mut hasher = DefaultHasher::new();
            chunk.hash(&mut hasher);
            let hash_value = hasher.finish();

            // Convert hash to bytes and extend result
            result.extend_from_slice(&hash_value.to_le_bytes());
        }

        // Ensure we have at least 32 bytes for compatibility
        if result.len() < 32 {
            result.resize(32, 0);
        }

        result
    }

    /// Check if AVX2 instructions are available
    #[must_use]
    pub const fn has_avx2(&self) -> bool {
        self.avx2_available
    }

    /// Check if SSE4.2 instructions are available
    #[must_use]
    pub const fn has_sse42(&self) -> bool {
        self.sse42_available
    }

    /// SIMD vector width in bytes (e.g. 16 for SSE, 32 for AVX2).
    #[must_use]
    pub const fn get_vector_width(&self) -> usize {
        self.vector_width
    }

    /// Suggested input chunk size for [`Self::vectorized_hash`] and similar.
    #[must_use]
    pub const fn optimal_chunk_size(&self) -> usize {
        self.vector_width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_memory_pool() {
        let pool = SafeZeroCopyMemoryPool::new(&[64, 128, 256], 8);

        let buffer1 = pool.allocate(64);
        assert_eq!(buffer1.len(), 64);

        let buffer2 = pool.allocate(128);
        assert_eq!(buffer2.len(), 128);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal

        // Return buffers to pool
        pool.deallocate(buffer1);
        pool.deallocate(buffer2);

        // Allocate again - should reuse from pool
        let buffer3 = pool.allocate(64);
        assert_eq!(buffer3.len(), 64);

        let metrics = pool.get_metrics();
        assert!(metrics.cache_hits.load(Ordering::Relaxed) > 0);
    }

    #[test]
    fn test_safe_ring_buffer() {
        let buffer = SafeRingBuffer::new(3);

        // Test push
        assert!(buffer.try_push("item1".to_string()).is_ok());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(buffer.try_push("item2".to_string()).is_ok());
        assert!(buffer.try_push("item3".to_string()).is_ok());

        // Buffer should be full
        assert!(buffer.try_push("item4".to_string()).is_err());

        // Test pop
        assert_eq!(buffer.try_pop(), Some("item1".to_string()));
        assert_eq!(buffer.try_pop(), Some("item2".to_string()));
        assert_eq!(buffer.try_pop(), Some("item3".to_string()));
        assert_eq!(buffer.try_pop(), None);
    }

    #[test]
    fn test_simd_capabilities() {
        let caps = SafeSimdCapabilities::new();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal

        assert!(caps.vector_width >= 16);
        assert!(caps.optimal_chunk_size() >= 16);

        // Should be able to detect some form of vectorization on modern systems
        println!("SIMD capabilities: {caps:?}");
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_memory_pool_alignment() {
        let pool = SafeZeroCopyMemoryPool::new(&[64], 16);

        let buffer = pool.allocate(50); // Should be aligned to 64
        assert_eq!(buffer.len(), 50);

        pool.deallocate(buffer);

        let metrics = pool.get_metrics();
        assert_eq!(metrics.allocations.load(Ordering::Relaxed), 1);
    }
}
