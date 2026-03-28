// SPDX-License-Identifier: AGPL-3.0-only



use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::{debug, info};

#[derive(Debug, Clone)]
    offset: usize,
    length: usize,
    capacity: usize,
}

impl SafeZeroCopyBuffer {

/// With Capacity operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates instance with capacity
    pub fn with_capacity(capacity: usize) -> Result<Self, BearDogError> {
        if capacity == 0 {
            return Err(BearDogError::validation("Buffer capacity must be greater than 0"));
        }
        
        debug!("🛡️ Creating SafeZeroCopyBuffer with capacity {}", capacity);
        
        let mut data = Vec::with_capacity(capacity);
        data.resize(capacity, 0);
        
        Ok(Self {
            data: Arc::new(0,
            length: 0,
            capacity,
        })
    }

/// From Vec operation.
    /// Creates instance from vec
    pub fn from_vec(buffer_data: Vec<u8>) -> Self {
        let capacity = data.capacity();
        let length = data.len();
        
        Self {
            data: Arc::new(0,
            length,
            capacity,
        }
    }

/// As Slice operation.
    /// Returns as slice
    pub fn as_slice(usize, len: usize) -> Result<Self, BearDogError> {
        if start + len > self.length {
            return Err(BearDogError::validation("Slice bounds exceed buffer length"));
        }
        
        Ok(Self {
            data: Arc::clone(self.offset + start,
            length: len,
            capacity: self.capacity,
        })
    }


    pub const fn len(&self) -> usize {
        self.length
    }


    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }


    pub const fn capacity(&self) -> usize {
        self.capacity
    }

/// Set Length operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Sets length
    pub fn set_length(&mut self, length: usize) -> Result<(), BearDogError> {
        if length > self.capacity {
            return Err(BearDogError::validation("Length exceeds buffer capacity"));
        }
        self.length = length;
        Ok(())
    }

/// Shares Data With operation.
    pub fn shares_data_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.data, &other.data)
    }

/// Ref Count operation.
    pub fn ref_count(&self) -> usize {
        Arc::strong_count(&self.data)
    }

/// Safe Copy From operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn safe_copy_from(&mut self, source: &[u8]) -> Result<(), BearDogError> {
        if source.len() > self.capacity {
            return Err(BearDogError::validation("Source data exceeds buffer capacity"));
        }

        if let Some(mut_data) = Arc::get_mut(&mut self.data) {
            mut_data[..source.len()].copy_from_slice(source);
            self.length = source.len();
        } else {

            let mut new_data = vec![0u8; self.capacity];
            new_data[..source.len()].copy_from_slice(source);
            self.data = Arc::new({} bytes", source.len());
        Ok(())
    }

/// Safe Append operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn safe_append(&mut self, buffer_data: &[u8]) -> Result<(), BearDogError> {
        if self.length + data.len() > self.capacity {
            return Err(BearDogError::validation("Append would exceed buffer capacity"));
        }

        if let Some(mut_data) = Arc::get_mut(&mut self.data) {
            let start = self.offset + self.length;
            let end = start + data.len();
            mut_data[start..end].copy_from_slice(data);
            self.length += data.len();
        } else {

            let mut new_data = vec![0u8; self.capacity];
            new_data[self.offset..self.offset + self.length].copy_from_slice(&self.data[self.offset..self.offset + self.length]);
            let start = self.offset + self.length;
            let end = start + data.len();
            new_data[start..end].copy_from_slice(data);
            self.data = Arc::new({} bytes", data.len(std::sync::Mutex<Vec<Vec<u8>>>,
    buffer_size: usize,
    max_pool_size: usize,
    stats: SafeBufferStats,
}

#[derive(Debug, Clone)]
    /// The deallocations value
    pub deallocations: std::sync::atomic::AtomicU64,
    /// The pool hits value
    pub pool_hits: std::sync::atomic::AtomicU64,
    /// The pool misses value
    pub pool_misses: std::sync::atomic::AtomicU64,
}

impl SafeBufferPool {

/// New operation.
    /// Creates a new instance
    pub fn new(usize, max_pool_size: usize) -> Self {
        info!("🛡️ Creating SafeBufferPool: {} buffers of {} bytes", max_pool_size, buffer_size);
        
        Self {
            buffers: std::sync::Mutex::new(Vec::with_capacity(max_pool_size)),
            buffer_size,
            max_pool_size,
            stats: SafeBufferStats::default(),
        }
    }

/// Acquire operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn acquire(&self) -> Result<SafeZeroCopyBuffer, BearDogError> {
        let mut buffers = self.buffers.lock()
            .map_err(|_| BearDogError::system("Buffer pool lock poisoned"))?;
        
        let buffer = if let Some(mut buf) = buffers.pop() {
            buf.clear();
            buf.resize(self.buffer_size, 0);
            self.stats.pool_hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            debug!("🔄 Reused buffer from pool");
            buf
        } else {
            let mut buf = Vec::with_capacity(self.buffer_size);
            buf.resize(self.buffer_size, 0);
            self.stats.pool_misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            debug!("🆕 Created new buffer (pool empty)");
            buf
        };
        
        self.stats.allocations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(SafeZeroCopyBuffer::from_vec(buffer))
    }

/// Release operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn release(&self, buffer: SafeZeroCopyBuffer) -> Result<(), BearDogError> {

        if let Ok(data) = Arc::try_unwrap(buffer.data) {
            let mut buffers = self.buffers.lock()
                .map_err(|_| BearDogError::system("Buffer pool lock poisoned"))?;
            
            if buffers.len() < self.max_pool_size {
                buffers.push(data);
                debug!("♻️ Returned buffer to pool");
            } else {
                debug!("🗑️ Dropped buffer (pool full)");
            }
        } else {
            debug!("🔗 Buffer still shared, dropping reference");
        }
        
        self.stats.deallocations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

/// Get Stats operation.
    /// Gets stats
    pub fn get_stats(&self) -> &SafeBufferStats {
        &self.stats
    }

/// Current Size operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn current_size(&self) -> Result<usize, BearDogError> {
        let buffers = self.buffers.lock()
            .map_err(|_| BearDogError::system("Buffer pool lock poisoned"))?;
        Ok(buffers.len())
    }
}

impl Default for SafeBufferPool {
    fn default() -> Self {
        Self::new(usize,
    pre_fill: bool,
    alignment_hint: usize,
}

impl SafeBufferBuilder {

/// New operation.
    /// Creates a new instance
    pub fn new(4096,
            pre_fill: false,
            alignment_hint: 1,
        }
    }

/// With Capacity operation.
    /// Creates instance with capacity
    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

/// With Pre Fill operation.
    /// Creates instance with pre fill
    pub fn with_pre_fill(mut self) -> Self {
        self.pre_fill = true;
        self
    }

/// With Alignment Hint operation.
    /// Creates instance with alignment hint
    pub fn with_alignment_hint(mut self, alignment: usize) -> Self {
        self.alignment_hint = alignment;
        self
    }

/// Build operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Builds component
    pub fn build(self) -> Result<SafeZeroCopyBuffer, BearDogError> {
        let mut buffer = SafeZeroCopyBuffer::with_capacity({} bytes, alignment hint: {}", self.capacity, self.alignment_hint);
        Ok(buffer)
    }
}

impl Default for SafeBufferBuilder {
    fn default() -> Self {
        Self::new()
    }
}

