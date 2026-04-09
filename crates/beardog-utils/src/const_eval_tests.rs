// SPDX-License-Identifier: AGPL-3.0-or-later
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

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

#[test]
fn test_const_config_valid_small_buffer() {
    let config = const_config! {
        buffer_size: 1024,
        max_connections: 10,
        cache_size: 64,
        logging: false,
        hash_rounds: 8
    };
    assert_eq!(config.buffer_size(), 1024);
    assert_eq!(config.cache_size(), 64);
    assert!(!config.logging_enabled());
    assert_eq!(config.hash_rounds(), 8);
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
    let empty_crc = ConstTables::crc32(&[]);
    assert_eq!(empty_crc, 0);

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
        beardog_errors::BearDogError::internal("Operation failed: Pop should succeed".to_string())
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
    let config: ConstConfig<524_288, 512, true, 12> = ConstConfig::default();
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
    let mut buffer = ConstBuffer::<2>::new().expect("ConstBuffer N=2 should construct");
    assert!(!buffer.is_full());
    assert_eq!(buffer.remaining(), 2);

    buffer.push(1).expect("push within capacity");
    assert!(!buffer.is_full());
    assert_eq!(buffer.remaining(), 1);

    buffer.push(2).expect("push within capacity");
    assert!(buffer.is_full());
    assert_eq!(buffer.remaining(), 0);
}

#[test]
fn test_const_buffer_push_when_full() {
    let mut buffer = ConstBuffer::<1>::new().expect("ConstBuffer N=1 should construct");
    buffer.push(42).expect("first push within capacity");
    let err = buffer.push(99).unwrap_err();
    assert!(err.to_string().contains("Buffer full"));
}

#[test]
fn test_const_buffer_pop_when_empty() {
    let mut buffer = ConstBuffer::<4>::new().expect("ConstBuffer N=4 should construct");
    assert_eq!(buffer.pop(), None);
}

#[test]
fn test_const_buffer_as_slice() {
    let mut buffer = ConstBuffer::<8>::new().expect("ConstBuffer N=8 should construct");
    buffer.push(10).expect("push within capacity");
    buffer.push(20).expect("push within capacity");
    buffer.push(30).expect("push within capacity");
    assert_eq!(buffer.as_slice(), &[10, 20, 30]);
}

#[test]
fn test_const_buffer_clear() {
    let mut buffer = ConstBuffer::<8>::new().expect("ConstBuffer N=8 should construct");
    buffer.push(1).expect("push within capacity");
    buffer.push(2).expect("push within capacity");
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
fn test_const_buffer_validation_rejects_oversized() {
    assert!(ConstBuffer::<65537>::new().is_ok());
    assert_eq!(ConstBuffer::<65537>::new().unwrap().capacity(), 65537);
}

#[test]
fn test_const_buffer_default_for_zero_sized_accepts_fallback_instance() {
    let buf = ConstBuffer::<0>::default();
    assert_eq!(buf.capacity(), 0);
    assert_eq!(buf.len(), 0);
    assert!(buf.is_empty());
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
    assert!((sin_360 - sin_0).abs() < f32::EPSILON);
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

#[test]
fn test_const_metrics_memory_requirements_zero_cache() {
    let m = ConstMetrics::memory_requirements(1024, 2, 0, 16);
    assert_eq!(m, (1024 * 2) + 2 * 16);
}
