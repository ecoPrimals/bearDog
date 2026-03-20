// SPDX-License-Identifier: AGPL-3.0-only

//! [`Arc`]-backed byte views and a mutex pool for reusable `Vec` slabs.

use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::{debug, info};

/// Shared backing storage with `(offset, length)` window; cheap subslices clone the [`Arc`].
#[derive(Debug, Clone)]
pub struct SafeZeroCopyBuffer {
    data: Arc<Vec<u8>>,
    offset: usize,
    length: usize,
}

impl SafeZeroCopyBuffer {
    /// With Capacity operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Creates instance with capacity
    pub fn with_capacity(capacity: usize) -> Result<Self, BearDogError> {
        debug!("🛡️ Creating SafeZeroCopyBuffer with capacity {}", capacity);

        let buffer = vec![0u8; capacity];

        Ok(Self {
            data: Arc::new(buffer),
            offset: 0,
            length: 0,
        })
    }

    /// From Vec operation.
    /// Creates instance from vec
    #[must_use]
    pub fn from_vec(buffer: Vec<u8>) -> Self {
        let length = buffer.len();
        Self {
            data: Arc::new(buffer),
            offset: 0,
            length,
        }
    }

    /// As Slice operation.
    /// Returns as slice
    pub fn as_slice(&self, start: usize, len: usize) -> Result<Self, BearDogError> {
        if start + len > self.length {
            return Err(BearDogError::validation(
                "Slice bounds exceed buffer length",
            ));
        }

        Ok(Self {
            data: Arc::clone(&self.data),
            offset: self.offset + start,
            length: len,
        })
    }

    /// Logical byte length of the active window.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.length
    }

    /// True when the window length is zero.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Capacity operation.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Shares Data With operation.
    #[must_use]
    pub fn shares_data_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.data, &other.data)
    }

    /// Ref Count operation.
    #[must_use]
    pub fn ref_count(&self) -> usize {
        Arc::strong_count(&self.data)
    }
}

/// Mutex-protected stack of `Vec<u8>` capped at `max_pool_size`, each sized to `buffer_size`.
#[derive(Debug)]
pub struct SafeBufferPool {
    buffers: std::sync::Mutex<Vec<Vec<u8>>>,
    buffer_size: usize,
    max_pool_size: usize,
}

impl SafeBufferPool {
    /// New operation.
    /// Creates a new instance
    pub fn new(buffer_size: usize, max_pool_size: usize) -> Self {
        info!(
            "🛡️ Creating SafeBufferPool: {} buffers of {} bytes",
            max_pool_size, buffer_size
        );

        Self {
            buffers: std::sync::Mutex::new(Vec::with_capacity(max_pool_size)),
            buffer_size,
            max_pool_size,
        }
    }

    /// Acquire operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn acquire(&self) -> Result<SafeZeroCopyBuffer, BearDogError> {
        let mut buffers = self
            .buffers
            .lock()
            .map_err(|_| BearDogError::system("Buffer pool lock poisoned".to_string()))?;

        let buffer = if let Some(mut buf) = buffers.pop() {
            buf.clear();
            buf.resize(self.buffer_size, 0);
            buf
        } else {
            vec![0u8; self.buffer_size]
        };

        Ok(SafeZeroCopyBuffer::from_vec(buffer))
    }

    /// Release operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn release(&self, buffer: SafeZeroCopyBuffer) -> Result<(), BearDogError> {
        if let Ok(buffer_data) = Arc::try_unwrap(buffer.data) {
            let mut buffers = self
                .buffers
                .lock()
                .map_err(|_| BearDogError::system("Buffer pool lock poisoned".to_string()))?;

            if buffers.len() < self.max_pool_size {
                buffers.push(buffer_data);
            }
        }

        Ok(())
    }

    /// Stats operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn stats(&self) -> Result<(usize, usize), BearDogError> {
        let buffers = self
            .buffers
            .lock()
            .map_err(|_| BearDogError::system("Buffer pool lock poisoned".to_string()))?;

        Ok((buffers.len(), self.max_pool_size))
    }
}

impl Default for SafeBufferPool {
    fn default() -> Self {
        Self::new(8192, 100) // 8KB buffers, pool of 100
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
#[allow(clippy::unwrap_used)] // Tests are allowed to use unwrap for assertions
mod tests {
    use super::*;

    #[test]
    fn test_safe_zero_copy_buffer() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::with_capacity(1024)?; // Test assertion
        assert_eq!(buffer.capacity(), 1024);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        Ok(())
    }

    #[test]
    fn test_zero_copy_slice() -> Result<(), Box<dyn std::error::Error>> {
        let buffer_data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let buffer = SafeZeroCopyBuffer::from_vec(buffer_data);

        let slice = buffer.as_slice(2, 4)?;
        // Test that we can access the slice data correctly
        assert_eq!(slice.len(), 4);
        assert!(buffer.shares_data_with(&slice));
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_buffer_pool() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeBufferPool::new(1024, 10);

        let buffer1 = pool.acquire()?; // Test assertion
        assert_eq!(buffer1.capacity(), 1024);

        pool.release(buffer1)?; // Test assertion

        let (available, max) = pool.stats()?; // Test assertion
        assert_eq!(max, 10);
        assert!(available <= max);
        Ok(())
    }

    #[test]
    fn test_buffer_from_vec_non_empty() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = SafeZeroCopyBuffer::from_vec(data.clone());

        assert_eq!(buffer.len(), 5);
        assert!(!buffer.is_empty());
        assert!(buffer.capacity() >= 5);
    }

    #[test]
    fn test_buffer_from_vec_empty() {
        let data = vec![];
        let buffer = SafeZeroCopyBuffer::from_vec(data);

        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_buffer_with_capacity_zero() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::with_capacity(0)?;
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), 0);
        assert!(buffer.is_empty());
        Ok(())
    }

    #[test]
    fn test_buffer_with_capacity_large() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::with_capacity(10000)?;
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), 10000);
        Ok(())
    }

    #[test]
    fn test_slice_at_start() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3, 4, 5]);
        let slice = buffer.as_slice(0, 3)?;

        assert_eq!(slice.len(), 3);
        assert!(buffer.shares_data_with(&slice));
        Ok(())
    }

    #[test]
    fn test_slice_at_end() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3, 4, 5]);
        let slice = buffer.as_slice(3, 2)?;

        assert_eq!(slice.len(), 2);
        assert!(buffer.shares_data_with(&slice));
        Ok(())
    }

    #[test]
    fn test_slice_entire_buffer() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3, 4, 5]);
        let slice = buffer.as_slice(0, 5)?;

        assert_eq!(slice.len(), 5);
        assert!(buffer.shares_data_with(&slice));
        Ok(())
    }

    #[test]
    fn test_slice_out_of_bounds() {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3, 4, 5]);

        // Start + len > buffer length
        let result = buffer.as_slice(3, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_slice_start_at_end() {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3, 4, 5]);

        // Start at end with len > 0
        let result = buffer.as_slice(5, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_slice_zero_length() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3, 4, 5]);
        let slice = buffer.as_slice(2, 0)?;

        assert_eq!(slice.len(), 0);
        assert!(slice.is_empty());
        Ok(())
    }

    #[test]
    fn test_multiple_slices_share_data() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]);

        let slice1 = buffer.as_slice(0, 4)?;
        let slice2 = buffer.as_slice(4, 4)?;

        // All should share the same underlying data
        assert!(buffer.shares_data_with(&slice1));
        assert!(buffer.shares_data_with(&slice2));
        assert!(slice1.shares_data_with(&slice2));
        Ok(())
    }

    #[test]
    fn test_slice_of_slice() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let slice1 = buffer.as_slice(2, 4)?;
        let slice2 = slice1.as_slice(1, 2)?;

        assert_eq!(slice2.len(), 2);
        assert!(buffer.shares_data_with(&slice2));
        Ok(())
    }

    #[test]
    fn test_ref_count_single() {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3]);
        assert_eq!(buffer.ref_count(), 1);
    }

    #[test]
    fn test_ref_count_with_slice() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3, 4, 5]);
        let initial_count = buffer.ref_count();

        let _slice = buffer.as_slice(0, 3)?;
        // Slice should increase ref count
        assert!(buffer.ref_count() > initial_count);
        Ok(())
    }

    #[test]
    fn test_ref_count_multiple_slices() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3, 4, 5]);
        let initial_count = buffer.ref_count();

        let _slice1 = buffer.as_slice(0, 2)?;
        let _slice2 = buffer.as_slice(2, 2)?;

        // Multiple slices should increase ref count
        assert_eq!(buffer.ref_count(), initial_count + 2);
        Ok(())
    }

    #[test]
    fn test_buffer_clone() {
        let buffer1 = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3]);
        let buffer2 = buffer1.clone();

        assert_eq!(buffer1.len(), buffer2.len());
        assert!(buffer1.shares_data_with(&buffer2));
    }

    #[test]
    fn test_buffer_debug() {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3]);
        let debug_str = format!("{:?}", buffer);
        assert!(debug_str.contains("SafeZeroCopyBuffer"));
    }

    #[test]
    fn test_pool_new() {
        let pool = SafeBufferPool::new(512, 5);
        let debug_str = format!("{:?}", pool);
        assert!(debug_str.contains("SafeBufferPool"));
    }

    #[test]
    fn test_pool_acquire_multiple() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeBufferPool::new(256, 5);

        let buf1 = pool.acquire()?;
        let buf2 = pool.acquire()?;
        let buf3 = pool.acquire()?;

        assert_eq!(buf1.capacity(), 256);
        assert_eq!(buf2.capacity(), 256);
        assert_eq!(buf3.capacity(), 256);

        // Should not share data
        assert!(!buf1.shares_data_with(&buf2));
        assert!(!buf2.shares_data_with(&buf3));
        Ok(())
    }

    #[test]
    fn test_pool_acquire_release_acquire() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeBufferPool::new(128, 3);

        let buf1 = pool.acquire()?;
        assert_eq!(buf1.capacity(), 128);

        pool.release(buf1)?;

        let buf2 = pool.acquire()?;
        assert_eq!(buf2.capacity(), 128);
        Ok(())
    }

    #[test]
    fn test_pool_stats_empty() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeBufferPool::new(512, 10);
        let (available, max) = pool.stats()?;

        assert_eq!(max, 10);
        assert_eq!(available, 0); // No buffers released yet
        Ok(())
    }

    #[test]
    fn test_pool_stats_after_release() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeBufferPool::new(256, 5);

        let buf = pool.acquire()?;
        pool.release(buf)?;

        let (available, max) = pool.stats()?;
        assert_eq!(max, 5);
        assert_eq!(available, 1);
        Ok(())
    }

    #[test]
    fn test_pool_max_size_not_exceeded() -> Result<(), Box<dyn std::error::Error>> {
        let pool = SafeBufferPool::new(128, 3);

        // Acquire and release more than max
        for _ in 0..5 {
            let buf = pool.acquire()?;
            pool.release(buf)?;
        }

        let (available, max) = pool.stats()?;
        assert_eq!(max, 3);
        assert!(available <= max);
        Ok(())
    }

    #[test]
    fn test_pool_different_sizes() -> Result<(), Box<dyn std::error::Error>> {
        let pool1 = SafeBufferPool::new(128, 5);
        let pool2 = SafeBufferPool::new(256, 3);

        let buf1 = pool1.acquire()?;
        let buf2 = pool2.acquire()?;

        assert_eq!(buf1.capacity(), 128);
        assert_eq!(buf2.capacity(), 256);
        Ok(())
    }

    #[test]
    fn test_buffer_shares_data_with_self() {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3]);
        assert!(buffer.shares_data_with(&buffer));
    }

    #[test]
    fn test_buffers_do_not_share_data() {
        let buffer1 = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3]);
        let buffer2 = SafeZeroCopyBuffer::from_vec(vec![4, 5, 6]);

        assert!(!buffer1.shares_data_with(&buffer2));
    }

    #[test]
    fn test_capacity_matches_allocation() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::with_capacity(1000)?;
        assert!(buffer.capacity() >= 1000);
        Ok(())
    }

    #[test]
    fn test_empty_buffer_is_empty() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::with_capacity(100)?;
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        Ok(())
    }

    #[test]
    fn test_slice_preserves_empty_state() -> Result<(), Box<dyn std::error::Error>> {
        let buffer = SafeZeroCopyBuffer::from_vec(vec![1, 2, 3]);
        let slice = buffer.as_slice(1, 0)?;

        assert!(slice.is_empty());
        assert_eq!(slice.len(), 0);
        Ok(())
    }
}
