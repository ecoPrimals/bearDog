

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use tracing::{debug, trace};
use zeroize::Zeroize; // ✅ MODERNIZED: Only import what we use

#[derive(Clone, PartialEq, Eq)]
pub struct SafeSecureBuffer {

    data: Vec<u8>,

    name: Option<String>,

    created_at: std::time::Instant,
}
impl SafeSecureBuffer {

    /// Creates a new instance
    pub fn new(size: usize) -> Self {
        debug!("🔐 Creating secure buffer of {} bytes", size);
        Self {
            data: vec![0u8; size],
            name: None,
            created_at: std::time::Instant::now(),
        }
    }


    /// Creates instance from data
    pub fn from_data(data: Vec<u8>) -> Self {
        debug!(
            "🔐 Creating secure buffer from {} bytes of data",
            data.len()
        );
            data,



    pub fn named(size: usize, name: impl Into<&str>) -> Self {
        let name = name.into();
            "🔐 Creating named secure buffer "{}" of {} bytes",
            name, size
            name: Some(name),

    #[must_use] pub const fn len(&self) -> usize {
        self.data.len()

    #[must_use] pub const fn is_empty(&self) -> bool {
        self.data.is_empty()

    #[must_use] pub fn as_slice(&self) -> &[u8] {
        &self.data


    /// Returns as mut slice
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data



    pub fn copy_from_slice(&mut self, source: &[u8]) -> Result<(), SafeMemoryError> {
        if source.len() > self.data.len() {
            return Err(SafeMemoryError::BufferTooSmall {
                required: source.len(),
                available: self.data.len(),
            });
        self.data[..source.len()].copy_from_slice(source);
        trace!("📋 Copied {} bytes into secure buffer", source.len());



    pub fn resize(&mut self, new_size: usize) {
        let old_size = self.data.len();
        self.data.resize(new_size, 0);
            "📏 Resized secure buffer from {} to {} bytes",
            old_size, new_size



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

    #[must_use] pub fn age(&self) -> std::time::Duration {
        self.created_at.elapsed()



    pub fn explicit_zero(&mut self) {
        self.data.zeroize();
        trace!("🧹 Explicitly zeroed secure buffer");
impl Deref for SafeSecureBuffer {
    type Target = [u8];}


    fn deref(&self) -> &Self::Target {
impl DerefMut for SafeSecureBuffer {}

    /// Returns mutable reference to deref
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
                "🗑️ Dropping named secure buffer "{}" ({} bytes, age: {:?})",
                name,
                self.data.len(),
                age
            );
        } else {
                "🗑️ Dropping secure buffer ({} bytes, age: {:?})",

pub struct SafePinnedBuffer {

    data: Pin<Box<[u8]>>,

    metadata: BufferMetadata,}

#[derive(Debug)]
struct BufferMetadata {
    size: usize,
    pin_count: usize,}

impl SafePinnedBuffer {

        debug!("📌 Creating pinned secure buffer of {} bytes", size);

        let boxed_slice: Box<[u8]> = vec![0u8; size].into_boxed_slice();
        let data = Pin::new(boxed_slice);
        let metadata = BufferMetadata {
            size,
            pin_count: 1,
        };
        Self { data, metadata }

            "📌 Creating named pinned buffer "{}" of {} bytes",

    #[must_use] pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
        self.metadata.size
        self.metadata.size == 0


    /// Creates instance with mut slice
    pub fn with_mut_slice<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {

        let slice = Pin::get_mut(self.data.as_mut());
        f(slice)
        self.metadata.created_at.elapsed()

impl fmt::Debug for SafePinnedBuffer {
        f.debug_struct("SafePinnedBuffer")
            .field("len", &self.metadata.size)
            .field("name", &self.metadata.name)
            .field("pin_count", &self.metadata.pin_count)
            .field("data", &"[REDACTED]")}

impl Drop for SafePinnedBuffer {
        if let Some(ref name) = self.metadata.name {
                "🗑️ Dropping named pinned buffer "{}" ({} bytes, age: {:?})",
                name, self.metadata.size, age
                "🗑️ Dropping pinned buffer ({} bytes, age: {:?})",
                self.metadata.size, age

pub struct SafeMemoryPool {

    pools: std::collections::HashMap<usize, Vec<SafeSecureBuffer>>,

    stats: PoolStats,
#[derive(Debug, Default)]
struct PoolStats {
    allocations: usize,
    reuses: usize,
    total_bytes_managed: usize,}

impl SafeMemoryPool {

    /// Creates a new instance
    pub fn new() -> Self {
        debug!("🏊 Creating safe memory pool");
            pools: std::collections::HashMap::with_capacity(16),
            stats: PoolStats::default(),

    /// Gets buffer
    /// Gets buffer
    pub fn get_buffer(&mut self, size: usize) -> SafeSecureBuffer {
        if let Some(pool) = self.pools.get_mut(&size) {
            if let Some(buffer) = pool.pop() {
                self.stats.reuses += 1;
                debug!("♻️ Reused buffer from pool: {} bytes", size);
                return buffer;
            }

        self.stats.allocations += 1;
        self.stats.total_bytes_managed += size;
        debug!("🆕 Created new pooled buffer: {} bytes", size);
        SafeSecureBuffer::new(size)


    fn return_buffer(&mut self, mut buffer: SafeSecureBuffer) {
        let size = buffer.data.capacity();

        buffer.explicit_zero();

        self.pools.entry(size).or_default().push(buffer);

    #[must_use] pub const fn stats(&self) -> (usize, usize, usize) {
        (
            self.stats.allocations,
            self.stats.reuses,
            self.stats.total_bytes_managed,
        )



    pub fn clear(&mut self) {
        let total_buffers: usize = self.pools.values().map(|v| v.len()).sum();
        self.pools.clear();
        debug!("🧹 Cleared memory pool: {} buffers", total_buffers);
impl Default for SafeMemoryPool {}

    fn default() -> Self {
        Self::new()

pub use beardog_errors::BearDogError as SafeMemoryError;

pub struct SafeMemoryUtils;
impl SafeMemoryUtils {

    #[must_use] pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        use subtle::ConstantTimeEq;
        if a.len() != b.len() {
            return false;
        a.ct_eq(b).into()



    pub fn safe_copy(dest: &mut [u8], src: &[u8]) -> Result<(), SafeMemoryError> {
        if src.len() > dest.len() {
                required: src.len(),
                available: dest.len(),
        dest[..src.len()].copy_from_slice(src);

    #[must_use] pub fn safe_compare_padded(a: &[u8], b: &[u8], max_len: usize) -> bool {

        let mut padded_a = vec![0u8; max_len];
        let mut padded_b = vec![0u8; max_len];
        if a.len() <= max_len {
            padded_a[..a.len()].copy_from_slice(a);
        if b.len() <= max_len {
            padded_b[..b.len()].copy_from_slice(b);
        let result = Self::constant_time_eq(&padded_a, &padded_b) && a.len() == b.len();

        padded_a.zeroize();
        padded_b.zeroize();



pub fn safe_memory_examples() -> Result<(), BearDogError> {

        let mut buffer = SafeSecureBuffer::named(32, "example_key");
        buffer
            .copy_from_slice(b"secret key material here")
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?)"),
            })?;

    } // <- Memory zeroed here automatically

        let mut pool = SafeMemoryPool::new();
        {
            let buffer1 = pool.get_buffer(64);

            pool.return_buffer(buffer1); // Zeroed and returned to pool

            assert_eq!(buffer1.len(), 64);

            drop(buffer1);
            let _buffer2 = pool.get_buffer(64); // Reuses the same buffer

        let mut pinned = SafePinnedBuffer::named(32, "crypto_workspace");
        pinned.with_mut_slice(|slice| {

            slice[0] = 0x42;

            let _stable_ptr = slice.as_ptr();
        });

