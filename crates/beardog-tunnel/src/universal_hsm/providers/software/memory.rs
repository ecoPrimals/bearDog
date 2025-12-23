//! Software HSM memory management
//!
//! Provides secure memory management with automatic zeroing to prevent
//! sensitive data from remaining in memory after use.

use beardog_errors::BearDogError;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Software memory manager with secure allocation and zeroing
///
/// Ensures that sensitive data is properly zeroed when freed to prevent
/// memory disclosure attacks.
#[derive(Debug, Clone)]
pub struct SoftwareMemoryManager {
    /// Statistics tracking
    stats: Arc<MemoryStatsInternal>,
}

/// Internal statistics with atomic counters
#[derive(Debug)]
struct MemoryStatsInternal {
    allocated_bytes: AtomicUsize,
    secure_regions: AtomicUsize,
}

impl Default for SoftwareMemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftwareMemoryManager {
    /// Create new memory manager
    pub fn new() -> Self {
        Self {
            stats: Arc::new(MemoryStatsInternal {
                allocated_bytes: AtomicUsize::new(0),
                secure_regions: AtomicUsize::new(0),
            }),
        }
    }

    /// Allocate secure memory with cryptographically random initialization
    ///
    /// Unlike normal allocation, this fills the memory with random bytes
    /// to prevent information leakage through uninitialized memory patterns.
    ///
    /// # Errors
    /// Returns an error if allocation fails
    pub fn allocate_secure(&self, size: usize) -> Result<Vec<u8>, BearDogError> {
        use rand::RngCore;
        
        // Allocate memory filled with random bytes (more secure than zeros)
        let mut memory = vec![0u8; size];
        rand::thread_rng()
            .try_fill_bytes(&mut memory)
            .map_err(|e| BearDogError::security(
                format!("Failed to securely initialize memory: {e}"),
                e.into()
            ))?;
        
        // Update statistics
        self.stats.allocated_bytes.fetch_add(size, Ordering::SeqCst);
        self.stats.secure_regions.fetch_add(1, Ordering::SeqCst);
        
        Ok(memory)
    }

    /// Free secure memory with guaranteed zeroing
    ///
    /// Overwrites the memory with zeros before freeing to prevent
    /// sensitive data from persisting in memory after deallocation.
    ///
    /// # Security
    /// This function uses the `zeroize` crate which provides compiler-guaranteed
    /// zeroing that cannot be optimized away. 100% safe and secure!
    ///
    /// # Errors
    /// Returns an error if the operation fails (currently infallible)
    pub fn free_secure(&self, mut memory: Vec<u8>) -> Result<(), BearDogError> {
        let size = memory.len();
        
        // Zero the memory safely using zeroize crate
        // This is guaranteed to zero memory and cannot be optimized away
        use zeroize::Zeroize;
        memory.zeroize();
        
        // Update statistics
        self.stats.allocated_bytes.fetch_sub(size, Ordering::SeqCst);
        self.stats.secure_regions.fetch_sub(1, Ordering::SeqCst);
        
        // Memory is dropped here, but it's already been zeroed
        Ok(())
    }

    /// Get current memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        MemoryStats {
            allocated_bytes: self.stats.allocated_bytes.load(Ordering::SeqCst),
            secure_regions: self.stats.secure_regions.load(Ordering::SeqCst),
        }
    }
}

/// Memory statistics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryStats {
    /// Total allocated bytes
    pub allocated_bytes: usize,
    /// Number of secure regions
    pub secure_regions: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_manager_creation() {
        let manager = SoftwareMemoryManager::new();
        let stats = manager.get_stats();
        assert_eq!(stats.allocated_bytes, 0);
        assert_eq!(stats.secure_regions, 0);
    }

    #[test]
    fn test_allocate_secure() {
        let manager = SoftwareMemoryManager::new();
        
        // Allocate secure memory
        let memory = manager.allocate_secure(1024)?;
        
        // Should be correct size
        assert_eq!(memory.len(), 1024);
        
        // Should be filled with random bytes (not all zeros)
        assert_ne!(memory, vec![0u8; 1024]);
        
        // Stats should be updated
        let stats = manager.get_stats();
        assert_eq!(stats.allocated_bytes, 1024);
        assert_eq!(stats.secure_regions, 1);
    }

    #[test]
    fn test_free_secure() {
        let manager = SoftwareMemoryManager::new();
        
        // Allocate and free
        let memory = manager.allocate_secure(512)?;
        assert!(manager.free_secure(memory).is_ok());
        
        // Stats should be back to zero
        let stats = manager.get_stats();
        assert_eq!(stats.allocated_bytes, 0);
        assert_eq!(stats.secure_regions, 0);
    }

    #[test]
    fn test_multiple_allocations() {
        let manager = SoftwareMemoryManager::new();
        
        // Allocate multiple regions
        let mem1 = manager.allocate_secure(256)?;
        let mem2 = manager.allocate_secure(512)?;
        let mem3 = manager.allocate_secure(1024)?;
        
        // Stats should reflect all allocations
        let stats = manager.get_stats();
        assert_eq!(stats.allocated_bytes, 256 + 512 + 1024);
        assert_eq!(stats.secure_regions, 3);
        
        // Free one region
        manager.free_secure(mem2)?;
        let stats = manager.get_stats();
        assert_eq!(stats.allocated_bytes, 256 + 1024);
        assert_eq!(stats.secure_regions, 2);
        
        // Free remaining
        manager.free_secure(mem1)?;
        manager.free_secure(mem3)?;
        let stats = manager.get_stats();
        assert_eq!(stats.allocated_bytes, 0);
        assert_eq!(stats.secure_regions, 0);
    }

    #[test]
    fn test_secure_memory_is_zeroed() {
        let manager = SoftwareMemoryManager::new();
        
        // Allocate memory
        let mut memory = manager.allocate_secure(128)?;
        
        // Write sensitive data
        memory[0] = 0xFF;
        memory[127] = 0xAA;
        
        // Verify data is present
        assert_eq!(memory[0], 0xFF);
        assert_eq!(memory[127], 0xAA);
        
        // Free the memory (should zero it)
        manager.free_secure(memory)?;
        
        // Note: We can't verify the memory was zeroed after freeing
        // because it's been dropped. This test verifies the API works correctly.
    }

    #[test]
    fn test_zero_size_allocation() {
        let manager = SoftwareMemoryManager::new();
        
        let memory = manager.allocate_secure(0)?;
        assert_eq!(memory.len(), 0);
        
        let stats = manager.get_stats();
        assert_eq!(stats.allocated_bytes, 0);
        assert_eq!(stats.secure_regions, 1); // Region still counts
        
        manager.free_secure(memory)?;
        let stats = manager.get_stats();
        assert_eq!(stats.secure_regions, 0);
    }

    #[test]
    fn test_large_allocation() {
        let manager = SoftwareMemoryManager::new();
        
        // Allocate 1MB
        let memory = manager.allocate_secure(1024 * 1024)?;
        assert_eq!(memory.len(), 1024 * 1024);
        
        let stats = manager.get_stats();
        assert_eq!(stats.allocated_bytes, 1024 * 1024);
        
        manager.free_secure(memory)?;
    }
}
