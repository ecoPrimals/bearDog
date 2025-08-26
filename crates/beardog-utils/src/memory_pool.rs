

use beardog_errors::{BearDogError, BearDogResult};
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;
use tracing::{debug, info, warn};

#[derive(Debug)]
pub struct MemoryPool {

    available_blocks: Arc<Mutex<VecDeque<MemoryBlock>>>,

    block_size: usize,

    max_blocks: usize,

    allocated_count: Arc<Mutex<usize>>,

    stats: Arc<Mutex<PoolStats>>,
}

#[derive(Debug)]
pub struct MemoryBlock {

    data: Vec<u8>,

    pool: Arc<MemoryPool>,
}

#[derive(Debug, Clone, Default)]
pub struct PoolStats {

    pub total_allocations: u64,

    pub total_returns: u64,

    pub cache_hits: u64,

    pub cache_misses: u64,

    pub peak_concurrent: usize,
}

impl MemoryPool {

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

    pub fn get_block(self: &Arc<Self>) -> BearDogResult<PooledBuffer> {
        let mut stats = self.stats.lock();
        stats.total_allocations += 1;

        if let Some(block) = self.available_blocks.lock().pop_front() {
            stats.cache_hits += 1;
            debug!("♻️  Reused memory block from pool");

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

        let current_allocated = *self.allocated_count.lock();
        if current_allocated >= self.max_blocks {
            warn!("Memory pool exhausted: {} blocks in use", current_allocated);
            return Err(BearDogError::system("Memory pool exhausted"));
        }

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

    fn return_block(&self, mut block: MemoryBlock) {

        block.data.fill(0);

        let mut available = self.available_blocks.lock();
        if available.len() < self.max_blocks {
            available.push_back(block);
            
            let mut stats = self.stats.lock();
            stats.total_returns += 1;
            
            debug!("♻️  Returned memory block to pool");
        } else {
            debug!("🗑️  Dropped memory block (pool full)");
        }

        let mut allocated = self.allocated_count.lock();
        *allocated = allocated.saturating_sub(1);
    }

    pub fn get_stats(&self) -> PoolStats {
        self.stats.lock().clone()
    }

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

    pub fn minimal_default() -> Arc<Self> {

        Arc::new(Self {
            block_size: 1024, // Small default size
            max_blocks: 1,     // Minimal capacity
            available_blocks: Arc::new(Mutex::new(VecDeque::new())),
            allocated_count: Arc::new(Mutex::new(0)),
            stats: Arc::new(Mutex::new(PoolStats::default())),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PoolUtilization {
    pub allocated_blocks: usize,
    pub available_blocks: usize,
    pub total_capacity: usize,
    pub hit_rate_percent: f64,
    pub peak_concurrent: usize,
}

pub struct PooledBuffer {
    block: Option<MemoryBlock>,
}

impl PooledBuffer {
    fn new(block: MemoryBlock) -> Self {
        Self {
            block: Some(block),
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        if self.block.is_none() {
            tracing::error!("MemoryBuffer accessed with None block - reinitializing with empty buffer");

            let pool_ref = MemoryPool::minimal_default(); // This is already Arc<MemoryPool>
            self.block = Some(MemoryBlock { 
                data: vec![], 
                pool: pool_ref,
            });
        }

        match &mut self.block {
            Some(block) => &mut block.data,
            None => unreachable!("Block should be Some after initialization above"),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.block.as_ref().map(|block| block.data.as_slice()).unwrap_or_else(|| {
            tracing::error!("MemoryBuffer accessed with None block - returning empty slice");
            &[]
        })
    }

    pub fn len(&self) -> usize {
        self.block.as_ref().map(|block| block.data.len()).unwrap_or_else(|| {
            tracing::error!("MemoryBuffer accessed with None block - returning size 0");
            0
        })
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

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

pub struct GlobalPools {

    pub small: Arc<MemoryPool>,

    pub medium: Arc<MemoryPool>,

    pub large: Arc<MemoryPool>,
}

impl GlobalPools {

    pub fn new() -> BearDogResult<Self> {
        Ok(Self {
            small: MemoryPool::new(4 * 1024, 32, 128)?,      // 4KB x 32-128 blocks
            medium: MemoryPool::new(64 * 1024, 16, 64)?,     // 64KB x 16-64 blocks  
            large: MemoryPool::new(1024 * 1024, 4, 16)?,     // 1MB x 4-16 blocks
        })
    }

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
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Pool creation should succeed", e).to_string())
})?;
        let stats = pool.get_stats();
        assert_eq!(stats.total_allocations, 0);
        Ok(())
    }
    
    #[test]
    fn test_buffer_allocation_and_return() -> Result<(), Box<dyn std::error::Error>> {
        let pool = MemoryPool::new(1024, 2, 8).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Pool creation should succeed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Pool creation should succeed", e).to_string())
})?;

        let buffer = pool.get_block().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Buffer allocation should succeed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Buffer allocation should succeed", e).to_string())
})?;
        assert_eq!(buffer.len(), 1024);
        
        let stats_before = pool.get_stats();
        assert_eq!(stats_before.total_allocations, 1);

        drop(buffer);

        let _buffer2 = pool.get_block().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Buffer allocation should succeed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Buffer allocation should succeed", e).to_string())
})?;
        let stats_after = pool.get_stats();
        assert_eq!(stats_after.cache_hits, 1);
        Ok(())
    }
    
    #[test]
    fn test_global_pools() -> Result<(), Box<dyn std::error::Error>> {
        let pools = GlobalPools::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Global pools should initialize", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Global pools should initialize", e).to_string())
})?;

        let small_buf = pools.small.get_block().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Small buffer allocation should succeed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Small buffer allocation should succeed", e).to_string())
})?;
        assert_eq!(small_buf.len(), 4 * 1024);

        assert!(std::ptr::eq(pools.get_pool_for_size(1024), &pools.small));
        assert!(std::ptr::eq(pools.get_pool_for_size(32 * 1024), &pools.medium));
        assert!(std::ptr::eq(pools.get_pool_for_size(512 * 1024), &pools.large));
        Ok(())
    }
} 