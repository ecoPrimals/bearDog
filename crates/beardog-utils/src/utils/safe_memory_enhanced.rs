// SPDX-License-Identifier: AGPL-3.0-only

//! Async-safe pinned buffers and Tokio [`Mutex`] pools sized from canonical constants.
//!
//! Sensitive slabs use [`zeroize`] so backing bytes are cleared on drop; use [`SensitiveByteBuf`]
//! for small secrets that should use [`ZeroizeOnDrop`].

use std::sync::Arc;
use tokio::sync::Mutex;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Canonical small/medium/large buffer capacities from `beardog-types`.
pub use beardog_types::constants::domains::buffers::{
    BUFFER_SIZE_LARGE as LARGE, BUFFER_SIZE_MEDIUM as MEDIUM, BUFFER_SIZE_SMALL as SMALL,
};

/// Nested re-exports matching legacy `buffer_sizes::SMALL` paths.
pub mod buffer_sizes {
    pub use beardog_types::constants::domains::buffers::{
        BUFFER_SIZE_LARGE as LARGE, BUFFER_SIZE_MEDIUM as MEDIUM, BUFFER_SIZE_SMALL as SMALL,
    };
}

/// Global buffer pool manager for reusable memory buffers.
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

/// Vec-backed byte slab with explicit logical `size` (may equal `data.len()`).
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

    /// Same as [`Self::new`]; `_name` reserved for future diagnostics.
    #[must_use]
    pub fn named(size: usize, _name: &str) -> Self {
        Self::new(size)
    }

    /// Logical byte length tracked alongside `data`.
    #[must_use]
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Immutable read-only access to the full backing store.
    pub fn with_slice<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&[u8]) -> R,
    {
        f(&self.data)
    }

    /// Mutable access to the full backing store for in-place writes.
    pub fn with_mut_slice<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        f(&mut self.data)
    }
}

impl Drop for SafePinnedBuffer {
    fn drop(&mut self) {
        self.data.zeroize();
        self.size = 0;
    }
}

/// Heap-allocated sensitive bytes; cleared on drop via [`ZeroizeOnDrop`].
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SensitiveByteBuf {
    #[zeroize(skip)]
    label: String,
    data: Vec<u8>,
}

impl SensitiveByteBuf {
    /// Allocates `len` zero bytes under a diagnostic `label` (not secret; not zeroized).
    #[must_use]
    pub fn alloc_zeroized(len: usize, label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            data: vec![0u8; len],
        }
    }

    /// Borrows the secret slice.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Borrows the secret slice mutably.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Logical length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true when no bytes are allocated.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Non-secret label for logging/metrics.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// Fixed-capacity wrapper used when returning buffers from [`SafeBufferPool`].
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

    /// Borrow the inner pinned storage.
    #[must_use]
    pub const fn buffer(&self) -> &SafePinnedBuffer {
        &self.buffer
    }

    /// Mutable reference to the underlying [`SafePinnedBuffer`].
    pub fn buffer_mut(&mut self) -> &mut SafePinnedBuffer {
        &mut self.buffer
    }

    /// Compile-time capacity `SIZE`.
    #[must_use]
    pub const fn size(&self) -> usize {
        SIZE
    }
}

/// Hit/miss counters for a single [`SafeBufferPool`].
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

    /// `cache_hits / total_requests`, or `0.0` when idle.
    #[must_use]
    pub fn hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.cache_hits as f64 / self.total_requests as f64
        }
    }
}

/// Tokio mutex stack of [`SafePinnedBuffer`] of fixed `SIZE` with reuse metrics.
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

    /// Pops a buffer or allocates fresh; updates [`BufferPoolMetrics`].
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

    /// Clone of the live metrics (cheap snapshot).
    pub async fn metrics(&self) -> BufferPoolMetrics {
        self.metrics.lock().await.clone()
    }
}

/// Three-tier pools (small/medium/large) for common I/O buffer sizes.
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

    /// Acquires from the small pool ([`buffer_sizes::SMALL`] bytes).
    pub async fn get_small(&self) -> SafePooledBuffer<{ buffer_sizes::SMALL }> {
        self.small_pool.get_buffer().await
    }

    /// Acquires from the medium pool ([`buffer_sizes::MEDIUM`] bytes).
    pub async fn get_medium(&self) -> SafePooledBuffer<{ buffer_sizes::MEDIUM }> {
        self.medium_pool.get_buffer().await
    }

    /// Acquires from the large pool ([`buffer_sizes::LARGE`] bytes).
    pub async fn get_large(&self) -> SafePooledBuffer<{ buffer_sizes::LARGE }> {
        self.large_pool.get_buffer().await
    }

    /// Sums metrics across all three internal pools.
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

    #[test]
    fn safe_memory_global_buffer_pools_sizes() {
        let pools = GlobalBufferPools::new();
        assert_eq!(pools.get_medium().size(), 4096);
        assert_eq!(pools.get_large().size(), 65536);
        assert_eq!(pools.get_small().size(), 1024);
    }

    #[test]
    fn safe_memory_pinned_from_vec_named_and_with_buffer() {
        let buf = SafePinnedBuffer::from_vec(vec![1, 2, 3]);
        assert_eq!(buf.size(), 3);
        assert_eq!(buf.with_buffer(|s| s.to_vec()), vec![1, 2, 3]);
        let named = SafePinnedBuffer::named(16, "diag");
        assert_eq!(named.size(), 16);
    }

    #[test]
    fn safe_memory_sensitive_byte_buf() {
        let mut s = SensitiveByteBuf::alloc_zeroized(4, "k");
        assert_eq!(s.label(), "k");
        assert_eq!(s.len(), 4);
        assert!(!s.is_empty());
        s.as_mut_slice().copy_from_slice(&[9, 8, 7, 6]);
        assert_eq!(s.as_slice(), &[9, 8, 7, 6]);
        let empty = SensitiveByteBuf::alloc_zeroized(0, "e");
        assert!(empty.is_empty());
    }

    #[test]
    fn safe_memory_buffer_pool_metrics_hit_rate() {
        let idle = BufferPoolMetrics::new();
        assert_eq!(idle.hit_rate(), 0.0);
        let m = BufferPoolMetrics {
            total_requests: 4,
            cache_hits: 1,
            cache_misses: 3,
        };
        assert!((m.hit_rate() - 0.25).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn safe_memory_pool_miss_when_exhausted() {
        let pool: SafeBufferPool<256> = SafeBufferPool::new(1);
        let _a = pool.get_buffer().await;
        let _b = pool.get_buffer().await;
        let m = pool.metrics().await;
        assert_eq!(m.total_requests, 2);
        assert_eq!(m.cache_hits, 1);
        assert_eq!(m.cache_misses, 1);
    }

    #[tokio::test]
    async fn safe_memory_enhanced_pools_default_and_metrics() {
        let pools = EnhancedMemoryPools::default();
        let _ = pools.get_small().await;
        let m = pools.all_metrics().await;
        assert!(m.total_requests >= 1);
    }

    #[test]
    fn safe_memory_global_buffer_pools_default() {
        let pools = GlobalBufferPools::default();
        assert_eq!(pools.get_small().size(), 1024);
    }

    #[test]
    fn safe_memory_buffer_pool_metrics_default_matches_new() {
        assert_eq!(
            BufferPoolMetrics::default().total_requests,
            BufferPoolMetrics::new().total_requests
        );
        assert_eq!(
            BufferPoolMetrics::default().cache_hits,
            BufferPoolMetrics::new().cache_hits
        );
    }

    #[test]
    fn safe_memory_safe_pooled_buffer_buffer_and_mut() {
        let mut pooled: SafePooledBuffer<64> = SafePooledBuffer::default();
        assert_eq!(pooled.size(), 64);
        pooled.buffer_mut().with_mut_slice(|s| {
            s[0] = 0x01;
        });
        let b = pooled.buffer().with_slice(|s| s[0]);
        assert_eq!(b, 0x01);
    }

    #[tokio::test]
    async fn safe_memory_pool_zero_initial_capacity_all_misses() {
        let pool: SafeBufferPool<128> = SafeBufferPool::new(0);
        let _ = pool.get_buffer().await;
        let _ = pool.get_buffer().await;
        let m = pool.metrics().await;
        assert_eq!(m.total_requests, 2);
        assert_eq!(m.cache_hits, 0);
        assert_eq!(m.cache_misses, 2);
    }
}
