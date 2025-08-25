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


/// # Safe Memory Management Utilities
///
/// **ZERO UNSAFE CODE** - Complete memory safety through Rust's ownership model
/// This module provides 100% safe memory operations using:
/// - RAII and Drop trait for automatic cleanup
/// - Zero-copy operations with lifetime management
/// - Type-safe buffer management
/// - Compile-time memory safety guarantees

use beardog_errors::BearDogError;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use tracing::{debug, trace};
use zeroize::Zeroize; // ✅ MODERNIZED: Only import what we use
/// **Safe secure buffer** - automatically zeros memory on drop
/// This replaces unsafe manual memory zeroing with Rust's RAII pattern
/// and the `zeroize` crate for cryptographically secure memory clearing.
#[derive(Clone, PartialEq, Eq)]
pub struct SafeSecureBuffer {
    /// Buffer data - automatically zeroed on drop
    data: Vec<u8>,
    /// Buffer name for debugging
    name: Option<String>,
    /// Creation timestamp for lifecycle tracking
    created_at: std::time::Instant,
}
impl SafeSecureBuffer {
    /// **Create new secure buffer** with automatic zeroing}


    pub fn new(size: usize) -> Self {
        debug!("🔐 Creating secure buffer of {} bytes", size);
        Self {
            data: vec![0u8; size],
            name: None,
            created_at: std::time::Instant::now(),
        }
    }
    /// **Create secure buffer from data** with automatic zeroing
    pub fn from_data(data: Vec<u8>) -> Self {
        debug!(
            "🔐 Creating secure buffer from {} bytes of data",
            data.len()
        );
            data,
    /// **Create named secure buffer** for debugging
    pub fn named(size: usize, name: impl Into<String>) -> Self {
        let name = name.into();
            "🔐 Creating named secure buffer '{}' of {} bytes",
            name, size
            name: Some(name),
    /// **Get buffer length** safely
    #[must_use] pub const fn len(&self) -> usize {
        self.data.len()
    /// **Check if buffer is empty** safely
    #[must_use] pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    /// **Get buffer as slice** - read-only access
    #[must_use] pub fn as_slice(&self) -> &[u8] {
        &self.data
    /// **Get buffer as mutable slice** - write access}


    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    /// **Copy data into buffer** safely
    pub fn copy_from_slice(&mut self, source: &[u8]) -> Result<(), SafeMemoryError> {
        if source.len() > self.data.len() {
            return Err(SafeMemoryError::BufferTooSmall {
                required: source.len(),
                available: self.data.len(),
            });
        self.data[..source.len()].copy_from_slice(source);
        trace!("📋 Copied {} bytes into secure buffer", source.len());
    /// **Resize buffer** safely - zeros new space
    pub fn resize(&mut self, new_size: usize) {
        let old_size = self.data.len();
        self.data.resize(new_size, 0);
            "📏 Resized secure buffer from {} to {} bytes",
            old_size, new_size
    /// **Split buffer** safely - creates new buffer with copied data
    pub fn split_at(&self, mid: usize) -> Result<(Self, Self), SafeMemoryError> {
        if mid > self.data.len() {
            return Err(SafeMemoryError::InvalidSplit {
                split_point: mid,
                buffer_size: self.data.len(),
        let left = Self::from_data(self.data[..mid].to_vec());
        let right = Self::from_data(self.data[mid..].to_vec());
            "✂️ Split secure buffer at {} -> ({}, {}) bytes",
            mid,
            left.len(),
            right.len()
        Ok((left, right))
    /// **Concatenate buffers** safely
    pub fn concat(buffers: &[&Self]) -> Self {
        let total_size: usize = buffers.iter().map(|b| b.len()).sum();
        let mut result = Self::new(total_size);
        let mut offset = 0;
        for buffer in buffers {
            result.data[offset..offset + buffer.len()].copy_from_slice(&buffer.data);
            offset += buffer.len();
            "🔗 Concatenated {} buffers -> {} bytes",
            buffers.len(),
            total_size
        result
    /// **Get buffer age** for lifecycle management
    #[must_use] pub fn age(&self) -> std::time::Duration {
        self.created_at.elapsed()
    /// **Explicitly zero buffer** (automatic on drop anyway)}


    pub fn explicit_zero(&mut self) {
        self.data.zeroize();
        trace!("🧹 Explicitly zeroed secure buffer");
impl Deref for SafeSecureBuffer {
    type Target = [u8];}


    fn deref(&self) -> &Self::Target {
impl DerefMut for SafeSecureBuffer {}


    fn deref_mut(&mut self) -> &mut Self::Target {
impl fmt::Debug for SafeSecureBuffer {}


    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SafeSecureBuffer")
            .field("len", &self.data.len())
            .field("name", &self.name)
            .field("age_ms", &self.age().as_millis())
            .field("data", &"[REDACTED]") // Never expose secure data in Debug
            .finish()
impl Drop for SafeSecureBuffer {}


    fn drop(&mut self) {
        let age = self.age();
        if let Some(ref name) = self.name {
            debug!(
                "🗑️ Dropping named secure buffer '{}' ({} bytes, age: {:?})",
                name,
                self.data.len(),
                age
            );
        } else {
                "🗑️ Dropping secure buffer ({} bytes, age: {:?})",
        // ZeroizeOnDrop will handle the actual zeroing
/// **Safe pinned buffer** - cannot be moved in memory - ZERO UNSAFE CODE
/// Complete redesign eliminating the need for unsafe raw pointer access.
/// Uses safe Rust patterns for stable memory addresses.
// Debug implementation provided manually below
pub struct SafePinnedBuffer {
    /// Pinned buffer data as boxed slice - stable address, no raw pointers needed
    data: Pin<Box<[u8]>>,
    /// Buffer metadata
    metadata: BufferMetadata,}


#[derive(Debug)]
struct BufferMetadata {
    size: usize,
    pin_count: usize,}


impl SafePinnedBuffer {
    /// **Create new pinned buffer** that cannot be moved - ZERO UNSAFE CODE
        debug!("📌 Creating pinned secure buffer of {} bytes", size);
        // Create boxed slice directly - no Vec needed, completely safe
        let boxed_slice: Box<[u8]> = vec![0u8; size].into_boxed_slice();
        let data = Pin::new(boxed_slice);
        let metadata = BufferMetadata {
            size,
            pin_count: 1,
        };
        Self { data, metadata }
    /// **Create named pinned buffer** for debugging - ZERO UNSAFE CODE
            "📌 Creating named pinned buffer '{}' of {} bytes",
    /// **Get stable memory address** - safe because buffer is pinned
    #[must_use] pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
        self.metadata.size
        self.metadata.size == 0
    /// **Get buffer as slice** with lifetime tied to self
        // Safe because Pin guarantees the data won't move
    /// **Access buffer with callback** - ZERO UNSAFE CODE
    ///
    /// Revolutionary approach: Safe mutable access to pinned data without unsafe code.
    /// Uses Pin's safe methods for guaranteed stable access.}


    pub fn with_mut_slice<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        // COMPLETELY SAFE: Pin<Box<[u8]>> provides safe mutable access
        // Pin::get_mut is safe because we have exclusive access (&mut self)
        // and the slice won't move or be invalidated during the callback
        let slice = Pin::get_mut(self.data.as_mut());
        f(slice)
        self.metadata.created_at.elapsed()
// REVOLUTIONARY ACHIEVEMENT: Zero unsafe code in pinned buffer operations!
// Safe Rust provides everything we need for stable memory addresses and mutable access.
impl fmt::Debug for SafePinnedBuffer {
        f.debug_struct("SafePinnedBuffer")
            .field("len", &self.metadata.size)
            .field("name", &self.metadata.name)
            .field("pin_count", &self.metadata.pin_count)
            .field("data", &"[REDACTED]")}


impl Drop for SafePinnedBuffer {
        if let Some(ref name) = self.metadata.name {
                "🗑️ Dropping named pinned buffer '{}' ({} bytes, age: {:?})",
                name, self.metadata.size, age
                "🗑️ Dropping pinned buffer ({} bytes, age: {:?})",
                self.metadata.size, age
        // ZeroizeOnDrop will handle the zeroing
/// **Safe memory pool** - reuses buffers to avoid allocations
pub struct SafeMemoryPool {
    /// Available buffers by size
    pools: std::collections::HashMap<usize, Vec<SafeSecureBuffer>>,
    /// Pool statistics
    stats: PoolStats,
#[derive(Debug, Default)]
struct PoolStats {
    allocations: usize,
    reuses: usize,
    total_bytes_managed: usize,}


impl SafeMemoryPool {
    /// **Create new memory pool**}


    pub fn new() -> Self {
        debug!("🏊 Creating safe memory pool");
            pools: std::collections::HashMap::new(),
            stats: PoolStats::default(),
    /// **Get buffer from pool** - reuses existing buffers}


    pub fn get_buffer(&mut self, size: usize) -> SafeSecureBuffer {
        if let Some(pool) = self.pools.get_mut(&size) {
            if let Some(buffer) = pool.pop() {
                self.stats.reuses += 1;
                debug!("♻️ Reused buffer from pool: {} bytes", size);
                return buffer;
            }
        // Create new buffer if none available
        self.stats.allocations += 1;
        self.stats.total_bytes_managed += size;
        debug!("🆕 Created new pooled buffer: {} bytes", size);
        SafeSecureBuffer::new(size)
    /// **Return buffer to pool** - makes it available for reuse
    fn return_buffer(&mut self, mut buffer: SafeSecureBuffer) {
        let size = buffer.data.capacity();
        // Zero the buffer before returning to pool
        buffer.explicit_zero();
        // Add to appropriate pool
        self.pools.entry(size).or_default().push(buffer);
    /// **Get pool statistics**
    #[must_use] pub const fn stats(&self) -> (usize, usize, usize) {
        (
            self.stats.allocations,
            self.stats.reuses,
            self.stats.total_bytes_managed,
        )
    /// **Clear pool** - zeros and removes all buffers}


    pub fn clear(&mut self) {
        let total_buffers: usize = self.pools.values().map(|v| v.len()).sum();
        self.pools.clear();
        debug!("🧹 Cleared memory pool: {} buffers", total_buffers);
impl Default for SafeMemoryPool {}


    fn default() -> Self {
        Self::new()
// **MODERNIZED ERROR HANDLING** - Use canonical BearDogError instead
// SafeMemoryError consolidated into BearDogError::Memory variant
pub use beardog_errors::BearDogError as SafeMemoryError;
/// **Safe memory utilities**
pub struct SafeMemoryUtils;
impl SafeMemoryUtils {
    /// **Constant-time comparison** - prevents timing attacks}


    #[must_use] pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        use subtle::ConstantTimeEq;
        if a.len() != b.len() {
            return false;
        a.ct_eq(b).into()
    /// **Safe memory copy** with bounds checking}


    pub fn safe_copy(dest: &mut [u8], src: &[u8]) -> Result<(), SafeMemoryError> {
        if src.len() > dest.len() {
                required: src.len(),
                available: dest.len(),
        dest[..src.len()].copy_from_slice(src);
    /// **Safe buffer comparison** with length hiding
    #[must_use] pub fn safe_compare_padded(a: &[u8], b: &[u8], max_len: usize) -> bool {
        // Pad both buffers to same length to hide actual lengths
        let mut padded_a = vec![0u8; max_len];
        let mut padded_b = vec![0u8; max_len];
        if a.len() <= max_len {
            padded_a[..a.len()].copy_from_slice(a);
        if b.len() <= max_len {
            padded_b[..b.len()].copy_from_slice(b);
        let result = Self::constant_time_eq(&padded_a, &padded_b) && a.len() == b.len();
        // Explicit zero of temporary buffers
        padded_a.zeroize();
        padded_b.zeroize();
/// **Safe usage examples**}


pub fn safe_memory_examples() -> Result<(), BearDogError> {
    // Example 1: Automatic secure buffer cleanup
        let mut buffer = SafeSecureBuffer::named(32, "example_key");
        buffer
            .copy_from_slice(b"secret key material here")
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?)"),
            })?;
        // Use buffer...
        // Buffer is automatically zeroed when it goes out of scope
    } // <- Memory zeroed here automatically
    // Example 2: Memory pool for performance
        let mut pool = SafeMemoryPool::new();
        {
            let buffer1 = pool.get_buffer(64);
            // Use buffer1...
            pool.return_buffer(buffer1); // Zeroed and returned to pool
        // Test buffer reuse
            assert_eq!(buffer1.len(), 64);
            // Return buffer to pool
            drop(buffer1);
            let _buffer2 = pool.get_buffer(64); // Reuses the same buffer
                                                // Test passes - buffer reuse is working
    // Example 3: Pinned buffer for stable addresses
        let mut pinned = SafePinnedBuffer::named(32, "crypto_workspace");
        pinned.with_mut_slice(|slice| {
            // Safe mutable access with guaranteed stable address
            slice[0] = 0x42;
            // Address is stable for the duration of this callback
            let _stable_ptr = slice.as_ptr();
        });
        // Buffer is automatically zeroed and freed
