// SPDX-License-Identifier: AGPL-3.0-or-later



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tracing::{debug, info};

pub struct SafeRingBuffer<T> {
    buffer: Arc<Mutex<VecDeque<T>>>,
    capacity: usize,
    stats: SafeRingBufferStats,
}

#[derive(Debug, Clone)]
    /// The total pops value
    pub total_pops: std::sync::atomic::AtomicU64,
    /// The buffer full events value
    pub buffer_full_events: std::sync::atomic::AtomicU64,
    /// The buffer empty events value
    pub buffer_empty_events: std::sync::atomic::AtomicU64,
}

impl<T> SafeRingBuffer<T> 
where
    T: Clone + Send + Sync,
{

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(capacity: usize) -> Result<Self, BearDogError> {
        if capacity == 0 {
            return Err(BearDogError::validation("Ring buffer capacity must be greater than 0"));
        }
        
        info!("🛡️ Creating SafeRingBuffer with capacity {}", capacity);
        
        Ok(Self {
            buffer: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            capacity,
            stats: SafeRingBufferStats::default(),
        })
    }

/// Push operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn push(&self, element: T) -> Result<Option<T>, BearDogError> {
        let mut buffer = self.buffer.lock()
            .map_err(|_| BearDogError::system("Ring buffer lock poisoned"))?;
        
        self.stats.total_pushes.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        let evicted = if buffer.len() >= self.capacity {
            self.stats.buffer_full_events.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            buffer.pop_front({}", buffer.len());
        
        Ok(evicted)
    }

/// Pop operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn pop(&self) -> Result<Option<T>, BearDogError> {
        let mut buffer = self.buffer.lock()
            .map_err(|_| BearDogError::system("Ring buffer lock poisoned"))?;
        
        self.stats.total_pops.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        let result = buffer.pop_front();
        
        if result.is_none() {
            self.stats.buffer_empty_events.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        
        debug!("✅ Safe pop completed, buffer size: {}", buffer.len());
        Ok(result)
    }

/// Len operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn len(&self) -> Result<usize, BearDogError> {
        let buffer = self.buffer.lock()
            .map_err(|_| BearDogError::system("Ring buffer lock poisoned"))?;
        Ok(buffer.len())
    }

/// Is Empty operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Checks if empty
    pub fn is_empty(&self) -> Result<bool, BearDogError> {
        let buffer = self.buffer.lock()
            .map_err(|_| BearDogError::system("Ring buffer lock poisoned"))?;
        Ok(buffer.is_empty())
    }


    pub const fn capacity(&self) -> usize {
        self.capacity
    }

/// Get Stats operation.
    /// Gets stats
    pub fn get_stats(&self) -> &SafeRingBufferStats {
        &self.stats
    }

/// Clear operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn clear(&self) -> Result<(), BearDogError> {
        let mut buffer = self.buffer.lock()
            .map_err(|_| BearDogError::system(Arc<Vec<u8>>,
    read_offset: std::sync::atomic::AtomicUsize,
    write_offset: std::sync::atomic::AtomicUsize,
    capacity: usize,
}

impl SafeMemoryMappedBuffer {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(capacity: usize) -> Result<Self, BearDogError> {
        if capacity == 0 {
            return Err(BearDogError::validation("Buffer capacity must be greater than 0"));
        }
        
        info!("🛡️ Creating SafeMemoryMappedBuffer with capacity {}", capacity);
        
        let mut data = Vec::with_capacity(capacity);
        data.resize(capacity, 0);
        
        Ok(Self {
            data: Arc::new(data),
            read_offset: std::sync::atomic::AtomicUsize::new(0),
            write_offset: std::sync::atomic::AtomicUsize::new(0),
            capacity,
        })
    }

/// Write operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn write(&self, buffer_data: &[u8]) -> Result<usize, BearDogError> {
        if data.is_empty({} bytes", data.len());
        Ok(data.len())
    }

/// Read operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn read(&self, buffer: &mut [u8]) -> Result<usize, BearDogError> {
        if buffer.is_empty() {
            return Ok(0);
        }

        let read_pos = self.read_offset.load(std::sync::atomic::Ordering::Acquire);
        let available = self.capacity.saturating_sub(read_pos);
        let to_read = buffer.len().min(available);
        
        if to_read > 0 {
            buffer[..to_read].copy_from_slice(&self.data[read_pos..read_pos + to_read]);
            self.read_offset.store(read_pos + to_read, std::sync::atomic::Ordering::Release);
        }
        
        debug!("✅ Safe read operation: {} bytes", to_read);
        Ok(to_read)
    }


    pub const fn capacity(&self) -> usize {
        self.capacity
    }

/// Read Position operation.
    pub fn read_position(&self) -> usize {
        self.read_offset.load(std::sync::atomic::Ordering::Acquire)
    }

/// Write Position operation.
    pub fn write_position(&self) -> usize {
        self.write_offset.load(std::sync::atomic::Ordering::Acquire)
    }
}

pub struct SafeSharedBufferPool {
    buffers: Arc<Mutex<Vec<Arc<Vec<u8>>>>>,
    buffer_size: usize,
    max_buffers: usize,
    stats: SafeSharedBufferStats,
}

#[derive(Debug, Clone)]
    /// The total deallocations value
    pub total_deallocations: std::sync::atomic::AtomicU64,
    /// The active buffers value
    pub active_buffers: std::sync::atomic::AtomicU64,
    /// The pool hits value
    pub pool_hits: std::sync::atomic::AtomicU64,
    /// The pool misses value
    pub pool_misses: std::sync::atomic::AtomicU64,
}

impl SafeSharedBufferPool {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(usize, max_buffers: usize) -> Result<Self, BearDogError> {
        if buffer_size == 0 || max_buffers == 0 {
            return Err(BearDogError::validation({} buffers of {} bytes", max_buffers, buffer_size);
        
        Ok(Self {
            buffers: Arc::new(Mutex::new(Vec::with_capacity(max_buffers))),
            buffer_size,
            max_buffers,
            stats: SafeSharedBufferStats::default(),
        })
    }

/// Acquire operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn acquire(&self) -> Result<Arc<Vec<u8>>, BearDogError> {
        let mut buffers = self.buffers.lock()
            .map_err(|_| BearDogError::system("Shared buffer pool lock poisoned"))?;
        
        self.stats.total_allocations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        let buffer = if let Some(buf) = buffers.pop() {
            self.stats.pool_hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            debug!("🔄 Reused shared buffer from pool");
            buf
        } else {
            self.stats.pool_misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let mut new_buffer = Vec::with_capacity(self.buffer_size);
            new_buffer.resize(self.buffer_size, 0);
            let shared_buffer = Arc::new(new_buffer);
            debug!("🆕 Created new shared buffer");
            shared_buffer
        };
        
        self.stats.active_buffers.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(buffer)
    }

/// Try Reclaim operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn try_reclaim(&self, buffer: Arc<Vec<u8>>) -> Result<bool, BearDogError> {

        if let Ok(vec_buffer) = Arc::try_unwrap(buffer) {
            let mut buffers = self.buffers.lock()
                .map_err(|_| BearDogError::system("Shared buffer pool lock poisoned"))?;
            
            if buffers.len() < self.max_buffers {
                buffers.push(Arc::new(vec_buffer));
                self.stats.total_deallocations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.stats.active_buffers.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                debug!("♻️ Reclaimed shared buffer to pool");
                return Ok(true);
            }
        }
        
        debug!("🔗 Shared buffer still in use or pool full");
        Ok(false)
    }

/// Get Stats operation.
    /// Gets stats
    pub fn get_stats(&self) -> &SafeSharedBufferStats {
        &self.stats
    }

/// Current Pool Size operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn current_pool_size(&self) -> Result<usize, BearDogError> {
        let buffers = self.buffers.lock()
            .map_err(|_| BearDogError::system(Arc<Mutex<std::collections::HashMap<String, std::sync::Weak<str>>>>,
    stats: SafeStringInternerStats,
}

#[derive(Debug, Clone)]
    /// The cache hits value
    pub cache_hits: std::sync::atomic::AtomicU64,
    /// The cache misses value
    pub cache_misses: std::sync::atomic::AtomicU64,
    /// The active strings value
    pub active_strings: std::sync::atomic::AtomicU64,
}

impl SafeStringInterner {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        info!("🛡️ Creating SafeStringInterner");
        
        Self {
            strings: Arc::new(Mutex::new(std::collections::HashMap::with_capacity(16))),
            stats: SafeStringInternerStats::default(AsRef<str>>(&self, s: S) -> Result<Arc<str>, BearDogError> {
        let s_ref = s.as_ref();
        
        let mut strings = self.strings.lock()
            .map_err(|_| BearDogError::system("String interner lock poisoned"))?;
        
        self.stats.total_interns.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        if let Some(weak_str) = strings.get(s_ref) {
            if let Some(arc_str) = weak_str.upgrade() {
                self.stats.cache_hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                debug!("🔄 String cache hit: {}", s_ref);
                return Ok(arc_str);
            } else {

                strings.remove(s_ref);
            }
        }

        self.stats.cache_misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.stats.active_strings.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        let arc_str: Arc<str> = Arc::from(s_ref);
        let weak_str = Arc::downgrade({}", s_ref);
        Ok(arc_str)
    }

/// Get Stats operation.
    /// Gets stats
    pub fn get_stats(&self) -> &SafeStringInternerStats {
        &self.stats
    }

/// Cleanup operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Cleans up resources
    pub fn cleanup(&self) -> Result<usize, BearDogError> {
        let mut strings = self.strings.lock()
            .map_err(|_| BearDogError::system("String interner lock poisoned"))?;
        
        let initial_count = strings.len();
        strings.retain(|_, weak_str| weak_str.strong_count() > 0);
        let cleaned = initial_count - strings.len();
        
        if cleaned > 0 {
            self.stats.active_strings.fetch_sub(cleaned as u64, std::sync::atomic::Ordering::Relaxed);
            debug!("🧹 Cleaned up {} expired string references", cleaned);
        }
        
        Ok(cleaned)
    }
}

impl Default for SafeStringInterner {
    fn default() -> Self {
        Self::new()
    }
}

static GLOBAL_STRING_INTERNER: std::sync::OnceLock<SafeStringInterner> = std::sync::OnceLock::new();

/// Global String Interner operation.
pub fn global_string_interner() -> &'static SafeStringInterner {
    GLOBAL_STRING_INTERNER.get_or_init(SafeStringInterner::new)
}

/// Intern String operation.
///
/// # Errors
/// Returns an error if the operation fails.
pub fn intern_string<S: AsRef<str>>(s: S) -> Result<Arc<str>, BearDogError> {
    global_string_interner().intern(s)
}

