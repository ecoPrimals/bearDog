// SPDX-License-Identifier: AGPL-3.0-or-later

//! Compile-time configuration, math, lookup tables, and sizing helpers.
//!
//! Types here are usable in `const` contexts where possible so hot-path defaults
//! and static limits can be validated without runtime work.

use beardog_errors::BearDogError;
use std::marker::PhantomData;

/// Fixed numeric limits for buffers, caches, logging, and hash iteration count.
///
/// Generic parameters encode those limits as types so they propagate through
/// `const` evaluation and [`Self::total_memory_usage`].
#[derive(Debug, Clone)]
pub struct ConstConfig<
    const BUFFER_SIZE: usize = 4096,
    const CACHE_SIZE: usize = 512,
    const ENABLE_LOGGING: bool = true,
    const HASH_ROUNDS: u32 = 12,
> {
    _phantom: PhantomData<()>,
}

impl<const B: usize, const C: usize, const L: bool, const H: u32> Default
    for ConstConfig<B, C, L, H>
{
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<const B: usize, const C: usize, const L: bool, const H: u32> ConstConfig<B, C, L, H> {
    /// Constructs a config after checking buffer/cache size and hash-round bounds.
    ///
    /// # Errors
    ///
    /// Returns a static error message if buffer or cache size is zero, or hash rounds are out of range.
    pub const fn new() -> Result<Self, &'static str> {
        if B == 0 {
            return Err("Buffer size must be greater than 0");
        }
        if C == 0 {
            return Err("Cache size must be greater than 0");
        }
        if C == 0 {
            return Err("Cache size must be greater than 0");
        }
        if H < 4 {
            return Err("Hash rounds must be at least 4 for security");
        }
        if H > 31 {
            return Err("Hash rounds must not exceed 31 to prevent DoS");
        }

        Ok(Self {
            _phantom: PhantomData,
        })
    }

    /// Returns the configured primary buffer size in bytes (`B`).
    #[must_use]
    pub const fn buffer_size(&self) -> usize {
        B
    }

    /// Returns the configured cache entry count (`C`).
    #[must_use]
    pub const fn cache_size(&self) -> usize {
        C
    }

    /// Whether logging is enabled at compile time (`L`).
    #[must_use]
    pub const fn logging_enabled(&self) -> bool {
        L
    }

    /// Number of hash rounds (`H`) used for cost tuning in dependent algorithms.
    #[must_use]
    pub const fn hash_rounds(&self) -> u32 {
        H
    }

    /// Rough upper bound on memory implied by buffer × cache plus per-cache overhead.
    #[must_use]
    pub const fn total_memory_usage(&self) -> usize {
        B * C + C * 64 // Buffer size + cache overhead
    }

    /// Rejects configurations whose implied memory or buffer size exceeds safe caps.
    ///
    /// # Errors
    ///
    /// Returns an error if total implied memory or buffer size exceeds internal limits.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.total_memory_usage() > 100 * 1024 * 1024 {
            // 100MB limit
            return Err(BearDogError::validation(
                "Configuration exceeds memory limit",
            ));
        }

        if B > 1024 * 1024 {
            // 1MB buffer limit
            return Err(BearDogError::validation("Buffer size too large"));
        }
        Ok(())
    }
}

/// Pure `const` integer math helpers (factorials, powers, primes, Fibonacci).
pub struct ConstMath;

impl ConstMath {
    /// Computes `n!` recursively; suitable only for small `n` in `const` contexts.
    #[must_use]
    pub const fn factorial(n: u64) -> u64 {
        match n {
            0 | 1 => 1,
            _ => n * Self::factorial(n - 1),
        }
    }

    /// Integer exponentiation by squaring: `base^exp`.
    #[must_use]
    pub const fn pow(base: u64, exp: u32) -> u64 {
        match exp {
            0 => 1,
            1 => base,
            _ => {
                let half = Self::pow(base, exp / 2);
                if exp.is_multiple_of(2) {
                    half * half
                } else {
                    base * half * half
                }
            }
        }
    }

    /// Greatest common divisor (Euclidean algorithm).
    #[must_use]
    pub const fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }

    /// Least common multiple: `(a * b) / gcd(a, b)`.
    #[must_use]
    pub const fn lcm(a: u64, b: u64) -> u64 {
        (a * b) / Self::gcd(a, b)
    }

    /// Trial division primality test for small `n` (adequate for table generation).
    #[must_use]
    pub const fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n.is_multiple_of(2) {
            return false;
        }

        let mut i = 3;
        while i * i <= n {
            if n.is_multiple_of(i) {
                return false;
            }
            i += 2;
        }
        true
    }

    /// `n`th Fibonacci number via simple recursion (not for large `n`).
    #[must_use]
    pub const fn fibonacci(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => Self::fibonacci(n - 1) + Self::fibonacci(n - 2),
        }
    }
}

/// Precomputed lookup tables and CRC32 helpers for `const`/hot paths.
pub struct ConstTables;

impl ConstTables {
    /// IEEE-style CRC-32 polynomial table for byte-wise accumulation.
    pub const CRC32_TABLE: [u32; 256] = Self::generate_crc32_table();

    /// Degrees → sine samples (one entry per degree, wraps with `% 360`).
    pub const SINE_TABLE: [f32; 360] = beardog_types::constants::domains::math::SINE_TABLE_360;

    /// All primes ≤ 1000 in order (168 entries), for fast small-prime checks.
    pub const PRIMES_1000: [u16; 168] = Self::generate_primes_1000();

    const fn generate_crc32_table() -> [u32; 256] {
        let mut table = [0u32; 256];
        let mut i = 0;

        while i < 256 {
            #[expect(
                clippy::cast_possible_truncation,
                reason = "CRC table index 0..256 fits u32"
            )]
            let mut crc = i as u32;
            let mut j = 0;

            while j < 8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0xEDB8_8320;
                } else {
                    crc >>= 1;
                }
                j += 1;
            }

            table[i] = crc;
            i += 1;
        }

        table
    }

    // Note: generate_sine_table and const_sin moved to beardog-types/constants/domains/math.rs
    // We now reference the centralized SINE_TABLE_360 instead

    const fn generate_primes_1000() -> [u16; 168] {
        let mut primes = [0u16; 168];
        let mut count = 0;
        let mut num = 2;

        while num <= 1000 && count < 168 {
            #[expect(clippy::cast_sign_loss, reason = "primes table uses positive num only")]
            let n = num as u64;
            if ConstMath::is_prime(n) {
                #[expect(clippy::cast_sign_loss, reason = "primes ≤1000 stored as u16")]
                #[expect(
                    clippy::cast_possible_truncation,
                    reason = "primes ≤1000 stored as u16"
                )]
                let p = num as u16;
                primes[count] = p;
                count += 1;
            }
            num += 1;
        }

        primes
    }

    /// Standard CRC-32 over `input_bytes` using [`Self::CRC32_TABLE`].
    #[must_use]
    pub fn crc32(input_bytes: &[u8]) -> u32 {
        let mut crc = 0xFFFF_FFFF;

        for &byte in input_bytes {
            let index = ((crc ^ u32::from(byte)) & 0xFF) as usize;
            crc = (crc >> 8) ^ Self::CRC32_TABLE[index];
        }

        !crc
    }

    /// Table lookup sine for integer degrees; wraps at 360.
    #[must_use]
    pub const fn fast_sin(degrees: u16) -> f32 {
        let index = (degrees % 360) as usize;
        Self::SINE_TABLE[index]
    }

    /// Returns true if `n` is prime, using [`Self::PRIMES_1000`] for `n ≤ 1000`.
    #[must_use]
    pub fn is_small_prime(n: u16) -> bool {
        if n > 1000 {
            return ConstMath::is_prime(u64::from(n));
        }

        Self::PRIMES_1000.contains(&n)
    }
}

/// `const`-friendly UTF-8 string length, equality, and simple rolling hash.
pub struct ConstStr;

impl ConstStr {
    /// Byte length of `s` (same as [`str::len`]).
    #[must_use]
    pub const fn len(s: &str) -> usize {
        s.len()
    }

    /// True when `s` has zero bytes.
    #[must_use]
    pub const fn is_empty(s: &str) -> bool {
        s.len() == 0
    }

    /// Byte-wise equality of two strings (not Unicode normalization).
    #[must_use]
    pub const fn eq(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let mut i = 0;

        while i < a_bytes.len() {
            if a_bytes[i] != b_bytes[i] {
                return false;
            }
            i += 1;
        }

        true
    }

    /// Polynomial rolling hash (factor 31); not cryptographic.
    #[must_use]
    pub const fn hash(s: &str) -> u64 {
        let bytes = s.as_bytes();
        let mut hash = 0u64;
        let mut i = 0;

        while i < bytes.len() {
            hash = hash.wrapping_mul(31).wrapping_add(bytes[i] as u64);
            i += 1;
        }

        hash
    }
}

/// Fixed-capacity byte buffer on the stack; length tracks used prefix of `data`.
#[derive(Debug)]
pub struct ConstBuffer<const SIZE: usize> {
    data: [u8; SIZE],
    len: usize,
}

impl<const SIZE: usize> ConstBuffer<SIZE> {
    /// Allocates zeroed storage when `SIZE` is in `(0, 1_048_576]`.
    ///
    /// # Errors
    ///
    /// Returns a static error message if `SIZE` is zero or exceeds 1 MiB.
    pub const fn new() -> Result<Self, &'static str> {
        if SIZE == 0 {
            return Err("Buffer size must be greater than 0");
        }
        if SIZE > 1024 * 1024 {
            return Err("Buffer size too large (max 1MB)");
        }

        Ok(Self {
            data: [0; SIZE],
            len: 0,
        })
    }

    /// Maximum bytes storable (`SIZE`).
    #[must_use]
    pub const fn capacity(&self) -> usize {
        SIZE
    }

    /// Current logical length (bytes written).
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// True when no bytes are stored.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// True when [`Self::len`] equals capacity.
    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.len == SIZE
    }

    /// Free space before the buffer is full.
    #[must_use]
    pub const fn remaining(&self) -> usize {
        SIZE - self.len
    }

    /// Appends one byte, erroring if at capacity.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is full.
    pub fn push(&mut self, byte: u8) -> Result<(), BearDogError> {
        if self.len >= SIZE {
            return Err(BearDogError::system("Buffer full".to_string()));
        }

        self.data[self.len] = byte;
        self.len += 1;
        Ok(())
    }

    /// Removes and returns the last byte, or `None` if empty.
    pub fn pop(&mut self) -> Option<u8> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        Some(self.data[self.len])
    }

    /// View of the filled prefix only.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }

    /// Resets length to zero and zero-fills the backing array.
    pub fn clear(&mut self) {
        self.len = 0;
        self.data.fill(0);
    }
}

impl<const SIZE: usize> Default for ConstBuffer<SIZE> {
    fn default() -> Self {
        Self::new().unwrap_or(Self {
            data: [0u8; SIZE],
            len: 0,
        })
    }
}

/// Builds a [`ConstConfig`] from literal parameters, validating at macro expansion/runtime init.
///
/// On validation failure logs an error and falls back to [`Default`].
#[macro_export]
macro_rules! const_config {
    (
        buffer_size: $buffer:expr,
        max_connections: $connections:expr,
        cache_size: $cache:expr,
        logging: $logging:expr,
        hash_rounds: $rounds:expr
    ) => {{
        const CONFIG: Result<ConstConfig<$buffer, $cache, $logging, $rounds>, &'static str> =
            ConstConfig::<$buffer, $cache, $logging, $rounds>::new();

        match CONFIG {
            Ok(config) => match config.validate() {
                Ok(_) => config,
                Err(e) => {
                    tracing::error!("Invalid configuration: {}, using default", e);
                    Default::default()
                }
            },
            Err(e) => {
                tracing::error!("Configuration error: {}, using default", e);
                Default::default()
            }
        }
    }};
}

/// Back-of-the-envelope throughput and memory formulas for sizing studies.
pub struct ConstMetrics;

impl ConstMetrics {
    /// Estimates items/sec given buffer size, per-buffer processing time, and parallelism.
    #[must_use]
    pub const fn theoretical_throughput(
        buffer_size: usize,
        processing_time_ns: u64,
        parallelism: usize,
    ) -> u64 {
        let items_per_buffer = buffer_size / 64; // Assume 64-byte items
        let buffers_per_second = 1_000_000_000 / processing_time_ns;

        items_per_buffer as u64 * buffers_per_second * parallelism as u64
    }

    /// Sums buffer, cache (64 B per entry), and per-buffer metadata memory.
    #[must_use]
    pub const fn memory_requirements(
        buffer_size: usize,
        buffer_count: usize,
        cache_size: usize,
        metadata_per_item: usize,
    ) -> usize {
        let buffer_memory = buffer_size * buffer_count;
        let cache_memory = cache_size * 64; // Assume 64-byte cache entries
        let metadata_memory = buffer_count * metadata_per_item;

        buffer_memory + cache_memory + metadata_memory
    }
}

#[cfg(test)]
#[path = "const_eval_tests.rs"]
mod tests;
