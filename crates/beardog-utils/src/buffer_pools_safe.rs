// Safe buffer pool implementation for BearDog
// Provides memory-efficient buffer management without unsafe code

use std::collections::HashMap;

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

pub struct SafeBufferPool {
    pools: HashMap<usize, Vec<Vec<u8>>>,
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

    /// Gets a buffer of the specified size
    /// Gets buffer
    /// Gets buffer
    pub fn get_buffer(&mut self, size: usize) -> Vec<u8> {
        let size_class = self.get_size_class(size);

        if let Some(pool) = self.pools.get_mut(&size_class) {
            if let Some(mut buffer) = pool.pop() {
                // Resize to requested size if needed
                buffer.resize(size, 0);
                self.stats.buffers_reused += 1;
                return buffer;
            }
        }

        self.stats.buffers_allocated += 1;
        self.stats.current_usage += 1;
        if self.stats.current_usage > self.stats.peak_usage {
            self.stats.peak_usage = self.stats.current_usage;
        }

        vec![0u8; size]
    }

    /// Returns a buffer to the pool
    pub fn return_buffer(&mut self, mut buffer: Vec<u8>) {
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
    /// Gets stats
    #[must_use]
    pub fn get_stats(&self) -> &PoolStats {
        &self.stats
    }

    /// Clears all pools
    pub fn clear(&mut self) {
        self.pools.clear();
        self.stats = PoolStats::default();
    }

    /// Gets `size_class`
    fn get_size_class(&self, size: usize) -> usize {
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

#[cfg(test)]
#[allow(clippy::unwrap_used)] // Tests are allowed to use unwrap for assertions
mod tests {
    use super::*;

    #[test]
    fn test_safe_buffer_pool_basic() {
        let mut pool = SafeBufferPool::new(5);

        let buffer = pool.get_buffer(1024);
        assert_eq!(buffer.len(), 1024);

        pool.return_buffer(buffer);

        let stats = pool.get_stats();
        assert_eq!(stats.buffers_allocated, 1);
        assert_eq!(stats.buffers_returned, 1);
    }

    #[test]
    fn test_buffer_reuse() {
        let mut pool = SafeBufferPool::new(5);

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
}
