// SPDX-License-Identifier: AGPL-3.0-or-later

// Safe memory pools for BearDog
// Provides thread-safe memory management without unchecked memory patterns

use crossbeam::queue::ArrayQueue;
use std::sync::{Arc, Mutex};

/// Fixed-capacity [`ArrayQueue`] of boxed `T` with mutex-protected [`PoolStats`].
#[derive(Debug)]
pub struct SafeMemoryPool<T> {
    pool: Arc<ArrayQueue<Box<T>>>,
    max_size: usize,
    stats: Arc<Mutex<PoolStats>>,
}

/// Memory pool statistics
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// Number of `total_allocations`
    pub total_allocations: u64,
    /// Number of `total_deallocations`
    pub total_deallocations: u64,
    /// Number of `pool_hits`
    pub pool_hits: u64,
    /// Number of `pool_misses`
    pub pool_misses: u64,
    /// Number of `peak_usage`
    pub peak_usage: usize,
    /// Number of `current_usage`
    pub current_usage: usize,
}

impl<T> SafeMemoryPool<T>
where
    T: Default,
{
    /// Create new safe memory pool
    /// Creates a new instance
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        let cap = max_size.max(1);
        Self {
            pool: Arc::new(ArrayQueue::new(cap)),
            max_size: cap,
            stats: Arc::new(Mutex::new(PoolStats::default())),
        }
    }

    /// Acquire object from pool or create new one
    ///
    /// # Errors
    ///
    /// Returns an error if the stats mutex is poisoned.
    pub fn acquire(&self) -> Result<Box<T>, std::io::Error> {
        let mut stats = self
            .stats
            .lock()
            .map_err(|_| std::io::Error::other("Stats mutex poisoned"))?;

        let element = if let Some(element) = self.pool.pop() {
            stats.pool_hits += 1;
            element
        } else {
            stats.pool_misses += 1;
            stats.total_allocations += 1;
            Box::new(T::default())
        };

        stats.current_usage += 1;
        if stats.current_usage > stats.peak_usage {
            stats.peak_usage = stats.current_usage;
        }
        Ok(element)
    }

    /// Release object back to pool
    ///
    /// # Errors
    ///
    /// Returns an error if the stats mutex is poisoned.
    pub fn release(&self, element: Box<T>) -> Result<(), std::io::Error> {
        let mut stats = self
            .stats
            .lock()
            .map_err(|_| std::io::Error::other("Stats mutex poisoned"))?;

        if self.pool.push(element).is_err() {
            // Pool at capacity; drop the returned box.
        }

        stats.total_deallocations += 1;
        if stats.current_usage > 0 {
            stats.current_usage -= 1;
        }

        Ok(())
    }

    /// Get current pool size
    ///
    /// # Errors
    ///
    /// Currently always returns `Ok`; the `Result` is reserved for future instrumentation.
    pub fn pool_size(&self) -> Result<usize, std::io::Error> {
        Ok(self.pool.len())
    }

    /// Get pool statistics
    /// Gets stats
    ///
    /// # Errors
    ///
    /// Returns an error if the stats mutex is poisoned.
    pub fn get_stats(&self) -> Result<PoolStats, std::io::Error> {
        let stats = self
            .stats
            .lock()
            .map_err(|_| std::io::Error::other("Stats mutex poisoned"))?;
        Ok(stats.clone())
    }

    /// Clear the pool
    ///
    /// # Errors
    ///
    /// Currently always returns `Ok`.
    pub fn clear(&self) -> Result<(), std::io::Error> {
        while self.pool.pop().is_some() {}
        Ok(())
    }
}

impl<T> Clone for SafeMemoryPool<T> {
    fn clone(&self) -> Self {
        Self {
            pool: Arc::clone(&self.pool),
            max_size: self.max_size,
            stats: Arc::clone(&self.stats),
        }
    }
}

impl<T> Default for SafeMemoryPool<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(100)
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, PartialEq)]
    struct TestStruct {
        value: u64,
    }

    #[test]
    fn test_memory_pool_basic() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeMemoryPool::<TestStruct>::new(10);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        let item = pool.acquire()?;
        assert_eq!(item.value, 0);

        pool.release(item)?;

        let stats = pool.get_stats()?;
        assert_eq!(stats.total_allocations, 1);
        assert_eq!(stats.total_deallocations, 1);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_pool_reuse() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeMemoryPool::<TestStruct>::new(10);

        let item1 = pool.acquire()?;
        pool.release(item1)?;

        let _item2 = pool.acquire()?;

        let stats = pool.get_stats()?;
        assert_eq!(stats.pool_hits, 1);
        Ok(())
    }

    #[test]
    fn test_pool_max_size() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeMemoryPool::<TestStruct>::new(2);

        // Fill the pool
        let item1 = pool.acquire()?;
        let item2 = pool.acquire()?;
        let item3 = pool.acquire()?;

        pool.release(item1)?;
        pool.release(item2)?;
        pool.release(item3)?; // This should be discarded as pool is full

        let size = pool.pool_size()?;
        assert_eq!(size, 2, "Pool should not exceed max_size");
        Ok(())
    }

    #[test]
    fn test_pool_stats_tracking() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeMemoryPool::<TestStruct>::new(5);

        // Acquire 3 items
        let item1 = pool.acquire()?;
        let item2 = pool.acquire()?;
        let item3 = pool.acquire()?;

        let stats = pool.get_stats()?;
        assert_eq!(stats.total_allocations, 3);
        assert_eq!(stats.pool_misses, 3);
        assert_eq!(stats.pool_hits, 0);
        assert_eq!(stats.current_usage, 3);
        assert_eq!(stats.peak_usage, 3);

        // Release 2 items
        pool.release(item1)?;
        pool.release(item2)?;

        let stats = pool.get_stats()?;
        assert_eq!(stats.total_deallocations, 2);
        assert_eq!(stats.current_usage, 1);
        assert_eq!(stats.peak_usage, 3);

        // Acquire again (should hit pool)
        let _item4 = pool.acquire()?;
        let _item5 = pool.acquire()?;

        let stats = pool.get_stats()?;
        assert_eq!(stats.pool_hits, 2);
        assert_eq!(stats.current_usage, 3);

        pool.release(item3)?;
        Ok(())
    }

    #[test]
    fn test_pool_clear() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeMemoryPool::<TestStruct>::new(10);

        // Add items to pool
        let item1 = pool.acquire()?;
        let item2 = pool.acquire()?;
        pool.release(item1)?;
        pool.release(item2)?;

        assert_eq!(pool.pool_size()?, 2);

        // Clear the pool
        pool.clear()?;
        assert_eq!(pool.pool_size()?, 0);

        Ok(())
    }

    #[test]
    fn test_pool_clone() -> Result<(), Box<dyn std::error::Error>> {
        let pool1 = SafeMemoryPool::<TestStruct>::new(10);

        let item = pool1.acquire()?;
        pool1.release(item)?;

        // Clone shares the same underlying pool
        let pool2 = pool1.clone();

        let item2 = pool2.acquire()?;
        assert!(item2.value == 0);

        // Both pools share stats
        let stats1 = pool1.get_stats()?;
        let stats2 = pool2.get_stats()?;
        assert_eq!(stats1.pool_hits, stats2.pool_hits);

        pool2.release(item2)?;
        Ok(())
    }

    #[test]
    fn test_pool_default() {
        let pool = SafeMemoryPool::<TestStruct>::default();
        assert_eq!(pool.max_size, 100);
    }

    #[test]
    fn test_pool_peak_usage() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeMemoryPool::<TestStruct>::new(10);

        let item1 = pool.acquire()?;
        let item2 = pool.acquire()?;
        let item3 = pool.acquire()?;

        let stats = pool.get_stats()?;
        assert_eq!(stats.peak_usage, 3);

        pool.release(item1)?;
        pool.release(item2)?;

        let stats = pool.get_stats()?;
        assert_eq!(stats.peak_usage, 3, "Peak usage should remain at maximum");
        assert_eq!(stats.current_usage, 1);

        pool.release(item3)?;
        Ok(())
    }

    #[test]
    fn test_pool_multiple_acquire_release_cycles() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeMemoryPool::<TestStruct>::new(5);

        // Multiple cycles
        for _ in 0..3 {
            let items: Vec<_> = (0..5)
                .map(|_| pool.acquire())
                .collect::<Result<Vec<_>, _>>()?;
            for item in items {
                pool.release(item)?;
            }
        }

        let stats = pool.get_stats()?;
        assert_eq!(stats.total_allocations, 5); // Only allocated once
        assert_eq!(stats.total_deallocations, 15); // Released 3 times each
        assert_eq!(stats.pool_hits, 10); // 2nd and 3rd cycles hit pool

        Ok(())
    }

    #[test]
    fn test_pool_size_tracking() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeMemoryPool::<TestStruct>::new(3);

        assert_eq!(pool.pool_size()?, 0);

        let item1 = pool.acquire()?;
        assert_eq!(pool.pool_size()?, 0);

        pool.release(item1)?;
        assert_eq!(pool.pool_size()?, 1);

        let item2 = pool.acquire()?;
        let item3 = pool.acquire()?;
        pool.release(item2)?;
        pool.release(item3)?;
        assert_eq!(pool.pool_size()?, 2);

        Ok(())
    }

    #[test]
    fn test_pool_stats_clone() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeMemoryPool::<TestStruct>::new(5);

        let item = pool.acquire()?;
        pool.release(item)?;

        let stats1 = pool.get_stats()?;
        let stats2 = stats1.clone();

        assert_eq!(stats1.total_allocations, stats2.total_allocations);
        assert_eq!(stats1.pool_hits, stats2.pool_hits);
        assert_eq!(stats1.pool_misses, stats2.pool_misses);

        Ok(())
    }

    #[test]
    fn test_pool_concurrent_access() -> Result<(), Box<dyn std::error::Error>> {
        use std::sync::Arc;
        use std::thread;

        let pool = Arc::new(SafeMemoryPool::<TestStruct>::new(10));
        let mut handles = vec![];

        // Spawn threads that acquire and release
        for _ in 0..5 {
            let pool_clone = Arc::clone(&pool);
            handles.push(thread::spawn(move || {
                for _ in 0..10 {
                    let item = pool_clone
                        .acquire()
                        .expect("acquire should succeed in concurrent pool test");
                    // Do some "work"
                    std::thread::sleep(std::time::Duration::from_micros(1));
                    pool_clone
                        .release(item)
                        .expect("release should succeed in concurrent pool test");
                }
            }));
        }

        // Wait for all threads
        for handle in handles {
            handle
                .join()
                .expect("concurrent pool test thread should not panic");
        }

        let stats = pool.get_stats()?;
        assert_eq!(stats.total_deallocations, 50);
        assert_eq!(stats.current_usage, 0);

        Ok(())
    }
}
