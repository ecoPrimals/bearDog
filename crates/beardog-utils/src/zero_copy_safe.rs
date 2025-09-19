// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::{debug, info};

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

    #[must_use]
    pub const fn len(&self) -> usize {
        self.length
    }

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

#[cfg(test)]
#[allow(clippy::unwrap_used)] // Tests are allowed to use unwrap for assertions
mod tests {
    use super::*;

    #[test]
    fn test_safe_zero_copy_buffer() {
        let buffer = SafeZeroCopyBuffer::with_capacity(1024).unwrap(); // Test assertion
        assert_eq!(buffer.capacity(), 1024);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_zero_copy_slice() {
        let buffer_data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let buffer = SafeZeroCopyBuffer::from_vec(buffer_data);

        let slice = buffer.as_slice(2, 4).unwrap();
        // Test that we can access the slice data correctly
        assert_eq!(slice.len(), 4);
        assert!(buffer.shares_data_with(&slice));
    }

    #[test]
    fn test_buffer_pool() {
        let pool = SafeBufferPool::new(1024, 10);

        let buffer1 = pool.acquire().unwrap(); // Test assertion
        assert_eq!(buffer1.capacity(), 1024);

        pool.release(buffer1).unwrap(); // Test assertion

        let (available, max) = pool.stats().unwrap(); // Test assertion
        assert_eq!(max, 10);
        assert!(available <= max);
    }
}
