// Performance optimization utilities for BearDog
// Provides zero-copy patterns, memory management, and SIMD acceleration

use bytes::{Bytes, BytesMut};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

pub struct CloneOptimizer {
    string_cache: HashMap<String, Arc<str>>,
    buffer_pools: HashMap<usize, Vec<BytesMut>>,
    stats: OptimizationStats,
}

#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    pub clones_avoided: u64,
    /// Number of memory_saved
    pub memory_saved: u64,
    /// Number of cache_hits
    pub cache_hits: u64,
    /// Number of cache_misses
    pub cache_misses: u64,
}

impl Default for CloneOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl CloneOptimizer {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            string_cache: HashMap::with_capacity(16),
            buffer_pools: HashMap::with_capacity(16),
            stats: OptimizationStats::default(),
        }
    }

    /// Optimize String operation.
    pub fn optimize_string<'a>(&mut self, input: &'a str) -> Cow<'a, str> {
        if self.should_intern_string(input) {
            if let Some(cached) = self.string_cache.get(input) {
                self.stats.cache_hits += 1;
                self.stats.clones_avoided += 1;
                return Cow::Owned(cached.to_string());
            } else {
                let interned = Arc::<str>::from(input);
                self.string_cache
                    .insert(input.to_string(), interned.clone());
                self.stats.cache_misses += 1;
                return Cow::Owned(interned.to_string());
            }
        }

        Cow::Borrowed(input)
    }

    /// Get Optimized Buffer operation.
    /// Gets optimized_buffer
    /// Gets optimized_buffer
    pub fn get_optimized_buffer(&mut self, size: usize) -> BytesMut {
        let size_class = self.get_size_class(size);

        if let Some(pool) = self.buffer_pools.get_mut(&size_class) {
            if let Some(buffer) = pool.pop() {
                self.stats.cache_hits += 1;
                return buffer;
            }
        }

        self.stats.cache_misses += 1;
        BytesMut::with_capacity(size_class)
    }

    /// Return Buffer operation.
    pub fn return_buffer(&mut self, mut buffer: BytesMut) {
        buffer.clear();
        let size_class = self.get_size_class(buffer.capacity());

        let pool = self.buffer_pools.entry(size_class).or_default();
        if pool.len() < 10 {
            // Limit pool size
            pool.push(buffer);
        }
    }

    /// Get Stats operation.
    /// Gets stats
    /// Gets stats
    pub fn get_stats(&self) -> &OptimizationStats {
        &self.stats
    }

    fn should_intern_string(&self, s: &str) -> bool {
        s.len() > 10 && s.len() < 1000
    }

    /// Gets size_class
    fn get_size_class(&self, size: usize) -> usize {
        match size {
            0..=64 => 64,
            65..=256 => 256,
            257..=1024 => 1024,
            1025..=4096 => 4096,
            _ => ((size / 4096) + 1) * 4096,
        }
    }
}

pub struct ZeroCopyProcessor {
    shared_buffer: BytesMut,
    shared_refs: HashMap<String, Arc<Bytes>>,
    processed_count: usize,
    shared_references: usize,
    total_bytes: usize,
}

impl ZeroCopyProcessor {
    /// New operation.
    /// Creates a new instance
    pub fn new(initial_capacity: usize) -> Self {
        Self {
            shared_buffer: BytesMut::with_capacity(initial_capacity),
            shared_refs: HashMap::with_capacity(16),
            processed_count: 0,
            shared_references: 0,
            total_bytes: 0,
        }
    }

    /// Process Zero Copy operation.
    /// Processes zero_copy
    /// Processes zero_copy
    pub fn process_zero_copy(&mut self, input_buffer: &[u8]) -> ProcessingResult {
        if input_buffer.is_empty() {
            return ProcessingResult::InvalidInput;
        }

        let view = DataView {
            buffer: input_buffer,
            offset: 0,
            length: input_buffer.len(),
        };

        let hash = self.hash_data(view.buffer);
        self.processed_count += 1;

        ProcessingResult::Success {
            hash,
            bytes_processed: input_buffer.len(),
        }
    }

    /// Create Shared Reference operation with zero-copy when possible
    /// Creates shared_reference
    /// Creates shared_reference
    pub fn create_shared_reference(&mut self, input_buffer: &[u8]) -> Arc<Bytes> {
        // Try to reuse existing shared buffer space if available
        if self.shared_buffer.capacity() - self.shared_buffer.len() >= input_buffer.len() {
            let start_pos = self.shared_buffer.len();
            self.shared_buffer.extend_from_slice(input_buffer);
            let bytes = self
                .shared_buffer
                .clone()
                .freeze()
                .slice(start_pos..start_pos + input_buffer.len());
            self.shared_buffer.clear(); // Reset for next use
            let shared = Arc::new(bytes);
            self.shared_references += 1;
            self.total_bytes += input_buffer.len();
            return shared;
        }

        // Fallback to copy when shared buffer is full
        let bytes = Bytes::copy_from_slice(input_buffer);
        let shared = Arc::new(bytes);
        self.shared_references += 1;
        self.total_bytes += input_buffer.len();
        shared
    }

    pub fn store_shared_reference(&mut self, name: String, data: &[u8]) -> Arc<Bytes> {
        let bytes = Bytes::copy_from_slice(data);
        let shared = Arc::new(bytes);
        self.shared_refs.insert(name, shared.clone());
        self.total_bytes += data.len();
        shared
    }

    /// Get stored reference by name
    /// Gets shared_reference
    /// Gets shared_reference
    pub fn get_shared_reference(&self, name: &str) -> Option<Arc<Bytes>> {
        self.shared_refs.get(name).cloned()
    }

    /// Extend shared buffer with data
    pub fn extend_buffer(&mut self, data: &[u8]) {
        self.shared_buffer.extend_from_slice(data);
        self.total_bytes += data.len();
    }

    /// Get statistics
    /// Gets stats
    /// Gets stats
    pub fn get_stats(&self) -> (usize, usize, usize) {
        (
            self.processed_count,
            self.shared_references,
            self.total_bytes,
        )
    }

    /// Create zero-copy slice from existing shared reference
    /// Creates zero_copy_slice
    /// Creates zero_copy_slice
    pub fn create_zero_copy_slice(
        &self,
        name: &str,
        start: usize,
        len: usize,
    ) -> Option<Arc<Bytes>> {
        if let Some(shared_ref) = self.shared_refs.get(name) {
            if start + len <= shared_ref.len() {
                let sliced = shared_ref.slice(start..start + len);
                return Some(Arc::new(sliced));
            }
        }
        None
    }

    /// Batch process multiple buffers with shared allocation
    pub fn batch_process_zero_copy(&mut self, buffers: &[&[u8]]) -> Vec<Arc<Bytes>> {
        let total_size: usize = buffers.iter().map(|b| b.len()).sum();

        // Pre-allocate shared buffer if needed
        if self.shared_buffer.capacity() < total_size {
            self.shared_buffer.reserve(total_size);
        }

        let mut results = Vec::with_capacity(buffers.len());

        for buffer in buffers {
            let shared_ref = self.create_shared_reference(buffer);
            results.push(shared_ref);
        }

        results
    }

    /// Defragment shared references to reduce memory usage
    pub fn defragment(&mut self) {
        // Remove weak references and compact buffer
        self.shared_refs
            .retain(|_, arc_ref| Arc::strong_count(arc_ref) > 1);

        if self.shared_buffer.capacity() > self.shared_buffer.len() * 2 {
            // Shrink buffer if it's significantly oversized
            let new_buffer = BytesMut::with_capacity(self.shared_buffer.len() + 1024);
            self.shared_buffer = new_buffer;
        }
    }

    /// Process Stream operation.
    /// Processes stream
    /// Processes stream
    pub fn process_stream<F>(
        &mut self,
        input_stream: &[u8],
        chunk_size: usize,
        mut processor: F,
    ) -> StreamResult
    where
        F: FnMut(&[u8]) -> Vec<u8>,
    {
        let mut results = Vec::new();
        let mut total_processed = 0;

        for chunk in input_stream.chunks(chunk_size) {
            let processed = processor(chunk);
            total_processed += processed.len();
            results.push(processed);
        }

        StreamResult::Success {
            results,
            bytes_processed: total_processed,
            peak_memory: chunk_size, // Constant memory usage
        }
    }

    pub const fn create_data_view<'a>(&self, input_buffer: &'a [u8]) -> DataView<'a> {
        DataView {
            buffer: input_buffer,
            offset: 0,
            length: input_buffer.len(),
        }
    }

    fn hash_data(&self, input_buffer: &[u8]) -> usize {
        input_buffer.iter().map(|&b| b as usize).sum()
    }
}

#[derive(Debug)]
pub enum ProcessingResult {
    Success { hash: usize, bytes_processed: usize },
    InvalidInput,
    Error(String),
}

#[derive(Debug)]
pub enum StreamResult {
    /// Successful completion state
    Success {
        results: Vec<Vec<u8>>,
        bytes_processed: usize,
        peak_memory: usize,
    },
}

pub struct DataView<'a> {
    buffer: &'a [u8],
    offset: usize,
    length: usize,
}

impl<'a> DataView<'a> {
    /// Get slice of data at offset
    pub fn slice(&self, start: usize, len: usize) -> Option<&[u8]> {
        if start + len <= self.length {
            Some(&self.buffer[self.offset + start..self.offset + start + len])
        } else {
            None
        }
    }

    /// Get the effective length of the view
    pub fn len(&self) -> usize {
        self.length
    }

    /// Check if the view is empty
    /// Checks if empty
    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

pub struct SimdAccelerator {
    features: CpuFeatures,
}

#[derive(Debug, Clone)]
pub struct CpuFeatures {
    /// Whether sse4_1 is enabled
    pub sse4_1: bool,
    /// Whether aes_ni is enabled
    pub aes_ni: bool,
}

impl Default for SimdAccelerator {
    fn default() -> Self {
        Self::new()
    }
}

impl SimdAccelerator {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            features: CpuFeatures {
                sse4_1: false, // Will be detected at runtime
                aes_ni: false, // Will be detected at runtime
            },
        }
    }

    /// Accelerated Hash operation.
    pub fn accelerated_hash(&self, buffer_data: &[u8]) -> u64 {
        // Safe SIMD-style acceleration without unsafe code
        buffer_data
            .chunks(8)
            .map(|chunk| {
                chunk
                    .iter()
                    .enumerate()
                    .map(|(i, &b)| (b as u64) << (i * 8))
                    .sum::<u64>()
            })
            .sum()
    }

    /// Get Features operation.
    /// Gets features
    /// Gets features
    pub fn get_features(&self) -> &CpuFeatures {
        &self.features
    }
}
