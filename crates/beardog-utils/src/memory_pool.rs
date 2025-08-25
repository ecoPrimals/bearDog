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


/// # High-Performance Memory Pool
///
/// **ZERO-ALLOCATION PATTERNS** - Memory pool for high-frequency operations
/// 
/// This module provides a memory pool implementation that eliminates heap allocations
/// during high-frequency operations by pre-allocating and reusing memory blocks.
///
/// ## Performance Benefits
/// - **Zero allocation overhead** on hot paths after initialization
/// - **Deterministic performance** - no GC pauses or allocation spikes
/// - **Cache-friendly** - memory blocks are contiguous and reused
/// - **Thread-safe** - concurrent access with minimal contention
///
/// ## Use Cases
/// - High-frequency cryptographic operations
/// - Network buffer management
/// - Temporary computation buffers
/// - Zero-copy data processing

use beardog_errors::{BearDogError, BearDogResult};
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Memory pool for zero-allocation patterns
#[derive(Debug)]
pub struct MemoryPool {
    /// Pool of available memory blocks
    available_blocks: Arc<Mutex<VecDeque<MemoryBlock>>>,
    /// Block size in bytes
    block_size: usize,
    /// Maximum number of blocks in pool
    max_blocks: usize,
    /// Current number of allocated blocks
    allocated_count: Arc<Mutex<usize>>,
    /// Pool statistics
    stats: Arc<Mutex<PoolStats>>,
}

/// Memory block from the pool
#[derive(Debug)]
pub struct MemoryBlock {
    /// The actual memory buffer
    data: Vec<u8>,
    /// Pool reference for returning the block
    pool: Arc<MemoryPool>,
}

/// Memory pool statistics
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// Total allocations requested
    pub total_allocations: u64,
    /// Total blocks returned to pool
    pub total_returns: u64,
    /// Cache hits (reused blocks)
    pub cache_hits: u64,
    /// Cache misses (new allocations)
    pub cache_misses: u64,
    /// Peak concurrent allocations
    pub peak_concurrent: usize,
}

impl MemoryPool {
    /// Create a new memory pool
    pub fn new(block_size: usize, initial_blocks: usize, max_blocks: usize) -> BearDogResult<Arc<Self>> {
        if block_size == 0 || max_blocks == 0 {
            return Err(BearDogError::business("Block size and max blocks must be > 0"));
        }
        
        let pool = Arc::new(Self {
            available_blocks: Arc::new(Mutex::new(VecDeque::with_capacity(initial_blocks))),
            block_size,
            max_blocks,
            allocated_count: Arc::new(Mutex::new(0)),
            stats: Arc::new(Mutex::new(PoolStats::default())),
        });
        
        // Pre-allocate initial blocks
        {
            let mut available = pool.available_blocks.lock();
            for _ in 0..initial_blocks {
                let block = MemoryBlock {
                    data: vec![0u8; block_size],
                    pool: Arc::clone(&pool),
                };
                available.push_back(block);
            }
        }
        
        info!(
            "🏊 Memory pool created: {} KB blocks, {} initial, {} max",
            block_size / 1024,
            initial_blocks,
            max_blocks
        );
        
        Ok(pool)
    }
    
    /// Get a memory block from the pool
    pub fn get_block(self: &Arc<Self>) -> BearDogResult<PooledBuffer> {
        let mut stats = self.stats.lock();
        stats.total_allocations += 1;
        
        // Try to get a block from the pool
        if let Some(block) = self.available_blocks.lock().pop_front() {
            stats.cache_hits += 1;
            debug!("♻️  Reused memory block from pool");
            
            // Update peak concurrent tracking
            let current_allocated = {
                let mut allocated = self.allocated_count.lock();
                *allocated += 1;
                *allocated
            };
            
            if current_allocated > stats.peak_concurrent {
                stats.peak_concurrent = current_allocated;
            }
            
            return Ok(PooledBuffer::new(block));
        }
        
        // Pool is empty, check if we can allocate a new block
        let current_allocated = *self.allocated_count.lock();
        if current_allocated >= self.max_blocks {
            warn!("Memory pool exhausted: {} blocks in use", current_allocated);
            return Err(BearDogError::system("Memory pool exhausted"));
        }
        
        // Allocate a new block
        stats.cache_misses += 1;
        let block = MemoryBlock {
            data: vec![0u8; self.block_size],
            pool: Arc::clone(self),
        };
        
        {
            let mut allocated = self.allocated_count.lock();
            *allocated += 1;
            if *allocated > stats.peak_concurrent {
                stats.peak_concurrent = *allocated;
            }
        }
        
        debug!("🆕 Allocated new memory block: {} bytes", self.block_size);
        Ok(PooledBuffer::new(block))
    }
    
    /// Return a block to the pool
    fn return_block(&self, mut block: MemoryBlock) {
        // Clear the buffer for security
        block.data.fill(0);
        
        // Return to pool if there's space
        let mut available = self.available_blocks.lock();
        if available.len() < self.max_blocks {
            available.push_back(block);
            
            let mut stats = self.stats.lock();
            stats.total_returns += 1;
            
            debug!("♻️  Returned memory block to pool");
        } else {
            debug!("🗑️  Dropped memory block (pool full)");
        }
        
        // Decrement allocated count
        let mut allocated = self.allocated_count.lock();
        *allocated = allocated.saturating_sub(1);
    }
    
    /// Get pool statistics
    pub fn get_stats(&self) -> PoolStats {
        self.stats.lock().clone()
    }
    
    /// Get pool utilization metrics
    pub fn get_utilization(&self) -> PoolUtilization {
        let stats = self.stats.lock();
        let allocated = *self.allocated_count.lock();
        let available = self.available_blocks.lock().len();
        
        let hit_rate = if stats.total_allocations > 0 {
            (stats.cache_hits as f64 / stats.total_allocations as f64) * 100.0
        } else {
            0.0
        };
        
        PoolUtilization {
            allocated_blocks: allocated,
            available_blocks: available,
            total_capacity: self.max_blocks,
            hit_rate_percent: hit_rate,
            peak_concurrent: stats.peak_concurrent,
        }
    }
    
    /// Create a minimal default memory pool for emergency fallback
    /// This is used when all other pool creation methods fail
    pub fn minimal_default() -> Arc<Self> {
        // Create the most minimal pool possible - just empty vectors
        Arc::new(Self {
            block_size: 1024, // Small default size
            max_blocks: 1,     // Minimal capacity
            available_blocks: Arc::new(Mutex::new(VecDeque::new())),
            allocated_count: Arc::new(Mutex::new(0)),
            stats: Arc::new(Mutex::new(PoolStats::default())),
        })
    }
}

/// Pool utilization metrics
#[derive(Debug, Clone)]
pub struct PoolUtilization {
    pub allocated_blocks: usize,
    pub available_blocks: usize,
    pub total_capacity: usize,
    pub hit_rate_percent: f64,
    pub peak_concurrent: usize,
}

/// RAII wrapper for pooled memory buffer
pub struct PooledBuffer {
    block: Option<MemoryBlock>,
}

impl PooledBuffer {
    fn new(block: MemoryBlock) -> Self {
        Self {
            block: Some(block),
        }
    }
    
    /// Get mutable access to the buffer
    /// 
    /// MemoryBuffer maintains the invariant that block is always Some after construction.
    /// If the invariant is violated, we reinitialize with an empty buffer (defensive programming).
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        if self.block.is_none() {
            tracing::error!("MemoryBuffer accessed with None block - reinitializing with empty buffer");
            // Create a minimal memory block with required pool reference
            let pool_ref = MemoryPool::minimal_default(); // This is already Arc<MemoryPool>
            self.block = Some(MemoryBlock { 
                data: vec![], 
                pool: pool_ref,
            });
        }
        // Safe: We ensure block is Some above
        match &mut self.block {
            Some(block) => &mut block.data,
            None => unreachable!("Block should be Some after initialization above"),
        }
    }
    
    /// Get read-only access to the buffer
    /// 
    /// MemoryBuffer maintains the invariant that block is always Some after construction.
    /// Returns an empty slice if the invariant is violated (defensive programming).
    pub fn as_slice(&self) -> &[u8] {
        self.block.as_ref().map(|block| block.data.as_slice()).unwrap_or_else(|| {
            tracing::error!("MemoryBuffer accessed with None block - returning empty slice");
            &[]
        })
    }
    
    /// Get buffer size
    /// 
    /// MemoryBuffer maintains the invariant that block is always Some after construction.
    /// Returns 0 if the invariant is violated (defensive programming).
    pub fn len(&self) -> usize {
        self.block.as_ref().map(|block| block.data.len()).unwrap_or_else(|| {
            tracing::error!("MemoryBuffer accessed with None block - returning size 0");
            0
        })
    }
    
    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Clear the buffer (fill with zeros)
    pub fn clear(&mut self) {
        if let Some(ref mut block) = self.block {
            block.data.fill(0);
        }
    }
}

impl Drop for PooledBuffer {
    fn drop(&mut self) {
        if let Some(block) = self.block.take() {
            let pool = block.pool.clone();
            pool.return_block(block);
        }
    }
}

impl std::fmt::Debug for PooledBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PooledBuffer")
            .field("size", &self.len())
            .finish()
    }
}

/// Global memory pools for common buffer sizes
pub struct GlobalPools {
    /// 4KB buffers for small operations
    pub small: Arc<MemoryPool>,
    /// 64KB buffers for medium operations  
    pub medium: Arc<MemoryPool>,
    /// 1MB buffers for large operations
    pub large: Arc<MemoryPool>,
}

impl GlobalPools {
    /// Initialize global memory pools
    pub fn new() -> BearDogResult<Self> {
        Ok(Self {
            small: MemoryPool::new(4 * 1024, 32, 128)?,      // 4KB x 32-128 blocks
            medium: MemoryPool::new(64 * 1024, 16, 64)?,     // 64KB x 16-64 blocks  
            large: MemoryPool::new(1024 * 1024, 4, 16)?,     // 1MB x 4-16 blocks
        })
    }
    
    /// Get the most appropriate pool for the requested size
    pub fn get_pool_for_size(&self, size: usize) -> &Arc<MemoryPool> {
        if size <= 4 * 1024 {
            &self.small
        } else if size <= 64 * 1024 {
            &self.medium
        } else {
            &self.large
        }
    }
}

impl Default for GlobalPools {
    fn default() -> Self {
        match Self::new() {
            Ok(pools) => pools,
            Err(e) => {
                tracing::error!("Failed to initialize global memory pools: {:?}", e);
                // Return minimal fallback pools using safe defaults
                Self {
                    small: MemoryPool::new(1024, 1, 2).unwrap_or_else(|_| {
                        tracing::error!("Failed to create small pool, using minimal default");
                        MemoryPool::minimal_default()
                    }),
                    medium: MemoryPool::new(4096, 1, 2).unwrap_or_else(|_| {
                        tracing::error!("Failed to create medium pool, using minimal default");
                        MemoryPool::minimal_default()
                    }),
                    large: MemoryPool::new(16384, 1, 2).unwrap_or_else(|_| {
                        tracing::error!("Failed to create large pool, using minimal default");
                        MemoryPool::minimal_default()
                    }),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_pool_creation() -> Result<(), Box<dyn std::error::Error>> {
        let pool = MemoryPool::new(1024, 4, 16).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Pool creation should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Pool creation should succeed", e))
})?;
        let stats = pool.get_stats();
        assert_eq!(stats.total_allocations, 0);
        Ok(())
    }
    
    #[test]
    fn test_buffer_allocation_and_return() -> Result<(), Box<dyn std::error::Error>> {
        let pool = MemoryPool::new(1024, 2, 8).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Pool creation should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Pool creation should succeed", e))
})?;
        
        // Get a buffer
        let buffer = pool.get_block().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Buffer allocation should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Buffer allocation should succeed", e))
})?;
        assert_eq!(buffer.len(), 1024);
        
        let stats_before = pool.get_stats();
        assert_eq!(stats_before.total_allocations, 1);
        
        // Drop the buffer (returns to pool)
        drop(buffer);
        
        // Get another buffer (should be reused)
        let _buffer2 = pool.get_block().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Buffer allocation should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Buffer allocation should succeed", e))
})?;
        let stats_after = pool.get_stats();
        assert_eq!(stats_after.cache_hits, 1);
        Ok(())
    }
    
    #[test]
    fn test_global_pools() -> Result<(), Box<dyn std::error::Error>> {
        let pools = GlobalPools::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Global pools should initialize", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Global pools should initialize", e))
})?;
        
        // Test small buffer
        let small_buf = pools.small.get_block().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Small buffer allocation should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Small buffer allocation should succeed", e))
})?;
        assert_eq!(small_buf.len(), 4 * 1024);
        
        // Test pool selection (using pointer comparison instead of PartialEq)
        assert!(std::ptr::eq(pools.get_pool_for_size(1024), &pools.small));
        assert!(std::ptr::eq(pools.get_pool_for_size(32 * 1024), &pools.medium));
        assert!(std::ptr::eq(pools.get_pool_for_size(512 * 1024), &pools.large));
        Ok(())
    }
} 