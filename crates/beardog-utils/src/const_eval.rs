// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use std::marker::PhantomData;

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

    #[must_use]
    pub const fn buffer_size(&self) -> usize {
        B
    }

    #[must_use]
    pub const fn cache_size(&self) -> usize {
        C
    }

    #[must_use]
    pub const fn logging_enabled(&self) -> bool {
        L
    }

    #[must_use]
    pub const fn hash_rounds(&self) -> u32 {
        H
    }

    #[must_use]
    pub const fn total_memory_usage(&self) -> usize {
        B * C + C * 64 // Buffer size + cache overhead
    }

    /// Validates input
    /// Validates input
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

pub struct ConstMath;

impl ConstMath {
    #[must_use]
    pub const fn factorial(n: u64) -> u64 {
        match n {
            0 | 1 => 1,
            _ => n * Self::factorial(n - 1),
        }
    }

    #[must_use]
    pub const fn pow(base: u64, exp: u32) -> u64 {
        match exp {
            0 => 1,
            1 => base,
            _ => {
                let half = Self::pow(base, exp / 2);
                if exp % 2 == 0 {
                    half * half
                } else {
                    base * half * half
                }
            }
        }
    }

    #[must_use]
    pub const fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    #[must_use]
    pub const fn lcm(a: u64, b: u64) -> u64 {
        (a * b) / Self::gcd(a, b)
    }

    #[must_use]
    pub const fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }

        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }

    #[must_use]
    pub const fn fibonacci(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => Self::fibonacci(n - 1) + Self::fibonacci(n - 2),
        }
    }
}

pub struct ConstTables;

impl ConstTables {
    pub const CRC32_TABLE: [u32; 256] = Self::generate_crc32_table();

    // Reference centralized sine table
    pub const SINE_TABLE: [f32; 360] = beardog_types::constants::domains::math::SINE_TABLE_360;

    pub const PRIMES_1000: [u16; 168] = Self::generate_primes_1000();

    const fn generate_crc32_table() -> [u32; 256] {
        let mut table = [0u32; 256];
        let mut i = 0;

        while i < 256 {
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
            if ConstMath::is_prime(num as u64) {
                primes[count] = num as u16;
                count += 1;
            }
            num += 1;
        }

        primes
    }

    #[must_use]
    pub fn crc32(input_bytes: &[u8]) -> u32 {
        let mut crc = 0xFFFF_FFFF;

        for &byte in input_bytes {
            let index = ((crc ^ u32::from(byte)) & 0xFF) as usize;
            crc = (crc >> 8) ^ Self::CRC32_TABLE[index];
        }

        !crc
    }

    #[must_use]
    pub const fn fast_sin(degrees: u16) -> f32 {
        let index = (degrees % 360) as usize;
        Self::SINE_TABLE[index]
    }

    /// Checks if small prime
    /// Checks if small prime
    #[must_use]
    pub fn is_small_prime(n: u16) -> bool {
        if n > 1000 {
            return ConstMath::is_prime(u64::from(n));
        }

        Self::PRIMES_1000.contains(&n)
    }
}

pub struct ConstStr;

impl ConstStr {
    #[must_use]
    pub const fn len(s: &str) -> usize {
        s.len()
    }

    #[must_use]
    pub const fn is_empty(s: &str) -> bool {
        s.len() == 0
    }

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

#[derive(Debug)]
pub struct ConstBuffer<const SIZE: usize> {
    data: [u8; SIZE],
    len: usize,
}

impl<const SIZE: usize> ConstBuffer<SIZE> {
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

    #[must_use]
    pub const fn capacity(&self) -> usize {
        SIZE
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.len == SIZE
    }

    #[must_use]
    pub const fn remaining(&self) -> usize {
        SIZE - self.len
    }

    pub fn push(&mut self, byte: u8) -> Result<(), BearDogError> {
        if self.len >= SIZE {
            return Err(BearDogError::system("Buffer full".to_string()));
        }

        self.data[self.len] = byte;
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        Some(self.data[self.len])
    }

    /// Returns as slice
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }

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

pub struct ConstMetrics;

impl ConstMetrics {
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

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_config() {
        let config = const_config! {
            buffer_size: 8192,
            max_connections: 1000,
            cache_size: 512,
            logging: true,
            hash_rounds: 12
        };

        assert_eq!(config.buffer_size(), 8192);
        assert_eq!(config.cache_size(), 512);
        assert!(config.logging_enabled());
        assert_eq!(config.hash_rounds(), 12);
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    #[test]
    fn test_const_math() {
        assert_eq!(ConstMath::factorial(5), 120);
        assert_eq!(ConstMath::pow(2, 10), 1024);
        assert_eq!(ConstMath::gcd(48, 18), 6);
        assert_eq!(ConstMath::lcm(4, 6), 12);
        assert!(ConstMath::is_prime(17));
        assert!(!ConstMath::is_prime(18));
        assert_eq!(ConstMath::fibonacci(10), 55);
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    fn test_const_tables() {
        let data = b"Hello, World!";
        let crc = ConstTables::crc32(data);
        assert!(crc != 0); // Should produce a valid CRC

        let sin_0 = ConstTables::fast_sin(0);
        let sin_90 = ConstTables::fast_sin(90);
        assert!((sin_0 - 0.0).abs() < 0.1);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!((sin_90 - 1.0).abs() < 0.1);

        assert!(ConstTables::is_small_prime(17));
        assert!(!ConstTables::is_small_prime(18));
    }

    #[test]
    fn test_const_buffer() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = ConstBuffer::<64>::new().map_err(|e| {
            tracing::error!("Operation failed ({}): {:?}", "Valid buffer", e);
            beardog_errors::BearDogError::internal(
                format_args!("Operation failed ({}): {:?}", "Valid buffer", e).to_string(),
                // TEST_CATEGORY: unit
                // TEST_DOMAIN: core
                // TEST_PRIORITY: normal
            )
        })?;

        assert_eq!(buffer.capacity(), 64);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());

        buffer.push(42).map_err(|e| {
            tracing::error!("Operation failed ({}): {:?}", "Push should succeed", e);
            beardog_errors::BearDogError::internal(
                format_args!("Operation failed ({}): {:?}", "Push should succeed", e).to_string(),
            )
        })?;
        assert_eq!(buffer.len(), 1);
        assert!(!buffer.is_empty());

        let value = buffer.pop().ok_or_else(|| {
            tracing::error!("Operation failed ({})", "Pop should succeed");
            beardog_errors::BearDogError::internal(
                "Operation failed: Pop should succeed".to_string(),
            )
        })?;
        assert_eq!(value, 42);
        assert!(buffer.is_empty());
        Ok(())
    }

    #[test]
    fn test_const_str() {
        assert_eq!(ConstStr::len("hello"), 5);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(ConstStr::is_empty(""));
        assert!(!ConstStr::is_empty("hello"));
        assert!(ConstStr::eq("hello", "hello"));
        assert!(!ConstStr::eq("hello", "world"));

        let hash1 = ConstStr::hash("test");
        let hash2 = ConstStr::hash("test");
        let hash3 = ConstStr::hash("different");
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_const_metrics() {
        let throughput = ConstMetrics::theoretical_throughput(8192, 1000, 4);
        assert!(throughput > 0);

        let memory = ConstMetrics::memory_requirements(8192, 10, 512, 64);
        assert_eq!(memory, 8192 * 10 + 512 * 64 + 10 * 64);
    }

    // ── Additional edge-case coverage ───────────────────────────────

    #[test]
    fn test_const_config_default() {
        let config: ConstConfig = ConstConfig::default();
        assert_eq!(config.buffer_size(), 4096);
        assert_eq!(config.cache_size(), 512);
        assert!(config.logging_enabled());
        assert_eq!(config.hash_rounds(), 12);
    }

    #[test]
    fn test_const_config_total_memory_usage() {
        let config: ConstConfig = ConstConfig::default();
        let expected = 4096 * 512 + 512 * 64;
        assert_eq!(config.total_memory_usage(), expected);
    }

    #[test]
    fn test_const_config_validate_ok() {
        let config: ConstConfig = ConstConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_const_config_validate_memory_limit() {
        // Create a config with huge buffer * cache to exceed 100MB
        let config: ConstConfig<524288, 512, true, 12> = ConstConfig::default();
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_const_config_validate_buffer_too_large() {
        // Buffer > 1MB limit
        let config: ConstConfig<{ 1024 * 1024 + 1 }, 8, true, 12> = ConstConfig::default();
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_const_config_new_valid() {
        let result = ConstConfig::<4096, 512, true, 12>::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_const_config_new_buffer_zero() {
        let result = ConstConfig::<0, 512, true, 12>::new();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Buffer size must be greater than 0");
    }

    #[test]
    fn test_const_config_new_cache_zero() {
        let result = ConstConfig::<4096, 0, true, 12>::new();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cache size must be greater than 0");
    }

    #[test]
    fn test_const_config_new_hash_rounds_too_low() {
        let result = ConstConfig::<4096, 512, true, 3>::new();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Hash rounds must be at least 4 for security"
        );
    }

    #[test]
    fn test_const_config_new_hash_rounds_too_high() {
        let result = ConstConfig::<4096, 512, true, 32>::new();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Hash rounds must not exceed 31 to prevent DoS"
        );
    }

    #[test]
    fn test_const_config_logging_disabled() {
        let config: ConstConfig<4096, 512, false, 12> = ConstConfig::default();
        assert!(!config.logging_enabled());
    }

    #[test]
    fn test_const_buffer_default() {
        let buffer = ConstBuffer::<64>::default();
        assert_eq!(buffer.capacity(), 64);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_const_buffer_full() {
        let mut buffer = ConstBuffer::<2>::new().unwrap();
        assert!(!buffer.is_full());
        assert_eq!(buffer.remaining(), 2);

        buffer.push(1).unwrap();
        assert!(!buffer.is_full());
        assert_eq!(buffer.remaining(), 1);

        buffer.push(2).unwrap();
        assert!(buffer.is_full());
        assert_eq!(buffer.remaining(), 0);
    }

    #[test]
    fn test_const_buffer_push_when_full() {
        let mut buffer = ConstBuffer::<1>::new().unwrap();
        buffer.push(42).unwrap();
        let err = buffer.push(99).unwrap_err();
        assert!(err.to_string().contains("Buffer full"));
    }

    #[test]
    fn test_const_buffer_pop_when_empty() {
        let mut buffer = ConstBuffer::<4>::new().unwrap();
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn test_const_buffer_as_slice() {
        let mut buffer = ConstBuffer::<8>::new().unwrap();
        buffer.push(10).unwrap();
        buffer.push(20).unwrap();
        buffer.push(30).unwrap();
        assert_eq!(buffer.as_slice(), &[10, 20, 30]);
    }

    #[test]
    fn test_const_buffer_clear() {
        let mut buffer = ConstBuffer::<8>::new().unwrap();
        buffer.push(1).unwrap();
        buffer.push(2).unwrap();
        buffer.clear();
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.as_slice(), &[] as &[u8]);
    }

    #[test]
    fn test_const_buffer_zero_size_error() {
        let result = ConstBuffer::<0>::new();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Buffer size must be greater than 0");
    }

    #[test]
    fn test_const_math_edge_cases() {
        assert_eq!(ConstMath::factorial(0), 1);
        assert_eq!(ConstMath::factorial(1), 1);
        assert_eq!(ConstMath::pow(5, 0), 1);
        assert_eq!(ConstMath::pow(5, 1), 5);
        assert_eq!(ConstMath::pow(2, 3), 8); // odd exp
        assert_eq!(ConstMath::pow(3, 4), 81); // even exp
        assert_eq!(ConstMath::gcd(10, 0), 10);
        assert!(!ConstMath::is_prime(0));
        assert!(!ConstMath::is_prime(1));
        assert!(ConstMath::is_prime(2));
        assert!(!ConstMath::is_prime(4));
        assert!(ConstMath::is_prime(7));
        assert_eq!(ConstMath::fibonacci(0), 0);
        assert_eq!(ConstMath::fibonacci(1), 1);
    }

    #[test]
    fn test_const_tables_is_small_prime_above_1000() {
        assert!(ConstTables::is_small_prime(1009)); // prime > 1000
        assert!(!ConstTables::is_small_prime(1010)); // composite > 1000
    }

    #[test]
    fn test_const_tables_fast_sin_wraparound() {
        let sin_360 = ConstTables::fast_sin(360);
        let sin_0 = ConstTables::fast_sin(0);
        assert!((sin_360 - sin_0).abs() < f64::EPSILON as f32);
    }

    #[test]
    fn test_const_str_eq_different_lengths() {
        assert!(!ConstStr::eq("hi", "hello"));
    }

    #[test]
    fn test_const_str_hash_empty() {
        assert_eq!(ConstStr::hash(""), 0);
    }

    #[test]
    fn test_const_metrics_zero_values() {
        let throughput = ConstMetrics::theoretical_throughput(64, 1, 1);
        assert!(throughput > 0);
    }
}
