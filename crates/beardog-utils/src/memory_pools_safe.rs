// Safe memory pools for BearDog
// Provides thread-safe memory management without unsafe code

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct SafeMemoryPool<T> {
    pool: Arc<Mutex<VecDeque<Box<T>>>>,
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
    #[must_use] pub fn new(max_size: usize) -> Self {
        Self {
            pool: Arc::new(Mutex::new(VecDeque::with_capacity(max_size))),
            max_size,
            stats: Arc::new(Mutex::new(PoolStats::default())),
        }
    }

    /// Acquire object from pool or create new one
    pub fn acquire(&self) -> Result<Box<T>, std::io::Error> {
        let mut pool = self
            .pool
            .lock()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Pool mutex poisoned"))?;

        let mut stats = self
            .stats
            .lock()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Stats mutex poisoned"))?;

        if let Some(element) = pool.pop_front() {
            stats.pool_hits += 1;
            stats.current_usage += 1;
            if stats.current_usage > stats.peak_usage {
                stats.peak_usage = stats.current_usage;
            }
            Ok(element)
        } else {
            stats.pool_misses += 1;
            stats.total_allocations += 1;
            stats.current_usage += 1;
            if stats.current_usage > stats.peak_usage {
                stats.peak_usage = stats.current_usage;
            }
            Ok(Box::new(T::default()))
        }
    }

    /// Release object back to pool
    pub fn release(&self, element: Box<T>) -> Result<(), std::io::Error> {
        let mut pool = self
            .pool
            .lock()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Pool mutex poisoned"))?;

        let mut stats = self
            .stats
            .lock()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Stats mutex poisoned"))?;

        if pool.len() < self.max_size {
            pool.push_back(element);
        }

        stats.total_deallocations += 1;
        if stats.current_usage > 0 {
            stats.current_usage -= 1;
        }

        Ok(())
    }

    /// Get current pool size
    pub fn pool_size(&self) -> Result<usize, std::io::Error> {
        let pool = self
            .pool
            .lock()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Pool mutex poisoned"))?;
        Ok(pool.len())
    }

    /// Get pool statistics
    /// Gets stats
    /// Gets stats
    pub fn get_stats(&self) -> Result<PoolStats, std::io::Error> {
        let stats = self
            .stats
            .lock()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Stats mutex poisoned"))?;
        Ok(stats.clone())
    }

    /// Clear the pool
    pub fn clear(&self) -> Result<(), std::io::Error> {
        let mut pool = self
            .pool
            .lock()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Pool mutex poisoned"))?;
        pool.clear();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, PartialEq)]
    struct TestStruct {
        value: u64,
    }

    #[test]
    fn test_memory_pool_basic() {
        let pool = SafeMemoryPool::<TestStruct>::new(10);

        let item = pool.acquire().unwrap();
        assert_eq!(item.value, 0);

        pool.release(item).unwrap();

        let stats = pool.get_stats().unwrap();
        assert_eq!(stats.total_allocations, 1);
        assert_eq!(stats.total_deallocations, 1);
    }

    #[test]
    fn test_pool_reuse() {
        let pool = SafeMemoryPool::<TestStruct>::new(10);

        let item1 = pool.acquire().unwrap();
        pool.release(item1).unwrap();

        let _item2 = pool.acquire().unwrap();

        let stats = pool.get_stats().unwrap();
        assert_eq!(stats.pool_hits, 1);
    }
}
