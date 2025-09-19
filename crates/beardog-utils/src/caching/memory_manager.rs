// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::config::CacheConfig;
use beardog_errors::BearDogError;
use std::sync::{Arc, Mutex};

pub struct MemoryManager {
    config: CacheConfig,
    stats: Arc<Mutex<MemoryStats>>,
}

/// Memory usage statistics
#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    /// Number of total_allocated
    pub total_allocated: usize,
    /// Number of peak_usage
    pub peak_usage: usize,
    /// Number of current_usage
    pub current_usage: usize,
    /// Number of gc_cycles
    pub gc_cycles: u64,
}

impl MemoryManager {
    /// Create new memory manager
    /// Creates a new instance
    pub fn new(config: CacheConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            config,
            stats: Arc::new(Mutex::new(MemoryStats::default())),
        })
    }

    pub fn allocate(&self, size: usize) -> Result<(), BearDogError> {
        let mut stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock memory stats: {e}")))?;

        stats.current_usage += size;
        stats.total_allocated += size;

        if stats.current_usage > stats.peak_usage {
            stats.peak_usage = stats.current_usage;
        }

        Ok(())
    }

    /// Deallocate memory from cache entry
    pub fn deallocate(&self, size: usize) -> Result<(), BearDogError> {
        let mut stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock memory stats: {e}")))?;

        if stats.current_usage >= size {
            stats.current_usage -= size;
        } else {
            stats.current_usage = 0;
        }

        Ok(())
    }

    /// Trigger garbage collection
    pub fn garbage_collect(&self) -> Result<usize, BearDogError> {
        let mut stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock memory stats: {e}")))?;

        let freed = stats.current_usage / 10; // Simulate freeing 10% of memory
        stats.current_usage -= freed;
        stats.gc_cycles += 1;

        Ok(freed)
    }

    /// Get memory statistics
    /// Gets stats
    /// Gets stats
    pub fn get_stats(&self) -> Result<MemoryStats, BearDogError> {
        let stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock memory stats: {e}")))?;
        Ok(stats.clone())
    }

    /// Check if memory pressure is high
    /// Checks if memory pressure high
    /// Checks if memory pressure high
    pub fn is_memory_pressure_high(&self) -> Result<bool, BearDogError> {
        let stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock memory stats: {e}")))?;

        let estimated_max = self.config.estimated_memory_usage_mb() * 1024 * 1024;
        Ok(stats.current_usage > estimated_max * 8 / 10) // 80% threshold
    }

    /// Get memory efficiency ratio
    pub fn memory_efficiency(&self) -> Result<f64, BearDogError> {
        let stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock memory stats: {e}")))?;

        if stats.total_allocated == 0 {
            Ok(1.0)
        } else {
            Ok(stats.current_usage as f64 / stats.total_allocated as f64)
        }
    }

    /// Reset memory statistics
    pub fn reset_stats(&self) -> Result<(), BearDogError> {
        let mut stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock memory stats: {e}")))?;

        *stats = MemoryStats::default();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_manager_allocation() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        let manager = MemoryManager::new(config)?;

        // Test allocation
        manager.allocate(1024)?;
        let stats = manager.get_stats()?;
        assert_eq!(stats.current_usage, 1024);
        assert_eq!(stats.total_allocated, 1024);

        // Test deallocation
        manager.deallocate(512)?;
        let stats = manager.get_stats()?;
        assert_eq!(stats.current_usage, 512);

        Ok(())
    }

    #[test]
    fn test_memory_manager_gc() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        let manager = MemoryManager::new(config)?;

        manager.allocate(1000)?;
        let freed = manager.garbage_collect()?;

        assert_eq!(freed, 100); // 10% of 1000

        let stats = manager.get_stats()?;
        assert_eq!(stats.gc_cycles, 1);

        Ok(())
    }

    #[test]
    fn test_memory_pressure() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        let manager = MemoryManager::new(config)?;

        // Initially no pressure
        assert!(!manager.is_memory_pressure_high()?);

        // Allocate large amount to trigger pressure
        manager.allocate(100 * 1024 * 1024)?; // 100MB
        assert!(manager.is_memory_pressure_high()?);

        Ok(())
    }
}
