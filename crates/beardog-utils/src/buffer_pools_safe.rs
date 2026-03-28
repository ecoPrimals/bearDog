// SPDX-License-Identifier: AGPL-3.0-only

//! Size-classed buffer pools backed by [`bytes::BytesMut`] (no raw pointers).

use bytes::BytesMut;
use std::collections::HashMap;

/// Accounting for [`SafeBufferPool`] reuse and high-water marks.
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// Number of `buffers_allocated`
    pub buffers_allocated: u64,
    /// Number of `buffers_reused`
    pub buffers_reused: u64,
    /// Number of `buffers_returned`
    pub buffers_returned: u64,
    /// Number of `peak_usage`
    pub peak_usage: u64,
    /// Number of `current_usage`
    pub current_usage: u64,
}

/// Per–size-class stacks of [`BytesMut`] with a cap on pooled instances.
pub struct SafeBufferPool {
    pools: HashMap<usize, Vec<BytesMut>>,
    stats: PoolStats,
    max_pool_size: usize,
}

impl SafeBufferPool {
    /// Creates a new safe buffer pool
    /// Creates a new instance
    #[must_use]
    pub fn new(max_pool_size: usize) -> Self {
        Self {
            pools: HashMap::with_capacity(16),
            stats: PoolStats::default(),
            max_pool_size,
        }
    }

    /// Gets a zero-filled [`BytesMut`] of the requested logical length.
    pub fn get_buffer(&mut self, size: usize) -> BytesMut {
        let size_class = self.get_size_class(size);

        if let Some(pool) = self.pools.get_mut(&size_class)
            && let Some(mut buffer) = pool.pop()
        {
            buffer.clear();
            buffer.resize(size, 0);
            self.stats.buffers_reused += 1;
            return buffer;
        }

        self.stats.buffers_allocated += 1;
        self.stats.current_usage += 1;
        if self.stats.current_usage > self.stats.peak_usage {
            self.stats.peak_usage = self.stats.current_usage;
        }

        let mut b = BytesMut::with_capacity(size_class.max(size));
        b.resize(size, 0);
        b
    }

    /// Returns a buffer to the pool.
    pub fn return_buffer(&mut self, mut buffer: BytesMut) {
        buffer.clear();
        let capacity = buffer.capacity();
        let size_class = self.get_size_class(capacity);

        let pool = self.pools.entry(size_class).or_default();
        if pool.len() < self.max_pool_size {
            pool.push(buffer);
            self.stats.buffers_returned += 1;
        }

        if self.stats.current_usage > 0 {
            self.stats.current_usage -= 1;
        }
    }

    /// Gets statistics about pool usage
    /// Gets stats
    #[must_use]
    pub const fn get_stats(&self) -> &PoolStats {
        &self.stats
    }

    /// Clears all pools
    pub fn clear(&mut self) {
        self.pools.clear();
        self.stats = PoolStats::default();
    }

    /// Gets `size_class`
    const fn get_size_class(&self, size: usize) -> usize {
        match size {
            0..=64 => 64,
            65..=256 => 256,
            257..=1024 => 1024,
            1025..=4096 => 4096,
            4097..=16384 => 16384,
            _ => ((size / 16384) + 1) * 16384,
        }
    }
}

impl Default for SafeBufferPool {
    fn default() -> Self {
        Self::new(10)
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
#[allow(clippy::unwrap_used)] // Tests are allowed to use unwrap for assertions
mod tests {
    use super::*;

    #[test]
    fn test_safe_buffer_pool_basic() {
        let mut pool = SafeBufferPool::new(5);

        let buffer = pool.get_buffer(1024);
        assert_eq!(buffer.len(), 1024);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        pool.return_buffer(buffer);

        let stats = pool.get_stats();
        assert_eq!(stats.buffers_allocated, 1);
        assert_eq!(stats.buffers_returned, 1);
    }

    #[test]
    fn test_buffer_reuse() {
        let mut pool = SafeBufferPool::new(5);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        let mut buffers = Vec::new();
        for _ in 0..3 {
            buffers.push(pool.get_buffer(512));
        }

        for buffer in buffers {
            pool.return_buffer(buffer);
        }

        let _buffer = pool.get_buffer(512);
        let stats = pool.get_stats();
        assert_eq!(stats.buffers_reused, 1); // Only 1 reuse when getting the last buffer
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_buffer_manager() {
        let mut manager = SafeBufferPool::new(10);

        let buffer1 = manager.get_buffer(1024);
        let buffer2 = manager.get_buffer(2048);

        assert_eq!(buffer1.len(), 1024);
        assert_eq!(buffer2.len(), 2048);

        manager.return_buffer(buffer1);
        manager.return_buffer(buffer2);

        let stats = manager.get_stats();
        assert_eq!(stats.buffers_allocated, 2);
        assert_eq!(stats.buffers_returned, 2);
    }

    #[test]
    fn test_buffer_pool_default() {
        let pool = SafeBufferPool::default();
        assert_eq!(pool.get_stats().buffers_allocated, 0);
    }

    #[test]
    fn test_pool_clear() {
        let mut pool = SafeBufferPool::new(5);
        let buf = pool.get_buffer(256);
        pool.return_buffer(buf);
        assert_eq!(pool.get_stats().buffers_returned, 1);

        pool.clear();
        let stats = pool.get_stats();
        assert_eq!(stats.buffers_allocated, 0);
        assert_eq!(stats.buffers_returned, 0);
    }

    #[test]
    fn test_pool_peak_usage() {
        let mut pool = SafeBufferPool::new(5);
        let b1 = pool.get_buffer(64);
        let b2 = pool.get_buffer(64);
        assert_eq!(pool.get_stats().peak_usage, 2);

        pool.return_buffer(b1);
        assert_eq!(pool.get_stats().current_usage, 1);
        assert_eq!(pool.get_stats().peak_usage, 2);

        pool.return_buffer(b2);
        assert_eq!(pool.get_stats().current_usage, 0);
    }

    #[test]
    fn test_pool_exceeds_max_size() {
        let mut pool = SafeBufferPool::new(1); // max 1 buffer per pool
        let b1 = pool.get_buffer(64);
        let b2 = pool.get_buffer(64);

        pool.return_buffer(b1); // accepted
        pool.return_buffer(b2); // should be dropped (pool full)

        // Only 1 should be returned to pool
        assert_eq!(pool.get_stats().buffers_returned, 1);
    }

    #[test]
    fn test_size_classes() {
        let mut pool = SafeBufferPool::new(5);

        // Test all size class boundaries
        let _ = pool.get_buffer(32); // class: 64
        let _ = pool.get_buffer(128); // class: 256
        let _ = pool.get_buffer(512); // class: 1024
        let _ = pool.get_buffer(2048); // class: 4096
        let _ = pool.get_buffer(8192); // class: 16384
        let _ = pool.get_buffer(32768); // class: > 16384

        assert_eq!(pool.get_stats().buffers_allocated, 6);
    }
}
