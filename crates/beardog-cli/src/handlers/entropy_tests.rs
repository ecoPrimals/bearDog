// SPDX-License-Identifier: AGPL-3.0-or-later

// Comprehensive tests for entropy handler
// Following BearDog standards: concurrent, robust, idiomatic
// NO sleeps, NO serial tests - only truly concurrent tests

use super::entropy::{
    base64_decode, base64_encode, calculate_entropy_quality, load_entropy_file, save_entropy_file,
};
use super::hsm_agnostic;
use tempfile::TempDir;

// ============================================================================
// HELPER FUNCTIONS (Extraction for testability)
// ============================================================================

#[test]
fn test_base64_encode_decode_roundtrip() {
    let original = b"hello world with some entropy bytes here";
    let encoded = base64_encode(original);
    let decoded = base64_decode(&encoded).expect("decode should succeed");

    assert_eq!(original, decoded.as_slice());
}

#[test]
fn test_base64_decode_invalid() {
    let result = base64_decode("not!!!valid!!!base64");
    assert!(result.is_err(), "should fail on invalid base64");
}

#[test]
fn test_entropy_quality_empty() {
    let quality = calculate_entropy_quality(&[]);
    assert_eq!(quality, 0.0, "empty bytes should have zero quality");
}

#[test]
fn test_entropy_quality_uniform_distribution() {
    // Create bytes with uniform distribution (high entropy)
    let mut bytes = Vec::new();
    for i in 0..=255 {
        bytes.push(i);
    }

    let quality = calculate_entropy_quality(&bytes);
    assert!(
        quality > 0.95,
        "uniform distribution should have high quality: {quality}"
    );
}

#[test]
fn test_entropy_quality_low_entropy() {
    // All same byte = low entropy
    let bytes = vec![0u8; 256];
    let quality = calculate_entropy_quality(&bytes);
    assert!(quality < 0.1, "uniform bytes should have low quality");
}

// ============================================================================
// AGNOSTIC HSM DISCOVERY TESTS (Concurrent)
// ============================================================================

#[tokio::test]
async fn test_discover_hsms_agnostic_works() {
    // Now uses universal agnostic discovery
    let result = hsm_agnostic::discover_all_hsms().await;
    assert!(result.is_ok(), "discovery should not error");

    let hsms = result.unwrap();
    // Should return vec (empty or with discovered HSMs)
    // Note: Vec::len() is always >= 0, this just verifies the call succeeds
    let _count = hsms.len();
}

#[tokio::test]
async fn test_concurrent_hsm_discovery_for_entropy() {
    // Test concurrent HSM discoveries (as entropy handler would do)
    let handles: Vec<_> = (0..10)
        .map(|_| tokio::spawn(async move { hsm_agnostic::discover_all_hsms().await }))
        .collect();

    for handle in handles {
        let result = handle.await.expect("task should not panic");
        assert!(result.is_ok(), "concurrent discovery should succeed");
    }
}

// ============================================================================
// ENTROPY FILE I/O TESTS (Concurrent)
// ============================================================================

#[test]
fn test_save_entropy_file() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let path = temp_dir.path().join("test_entropy.bin");

    let entropy_data = vec![1, 2, 3, 4, 5];
    let result = save_entropy_file(&entropy_data, path.to_str().unwrap());

    assert!(result.is_ok(), "should save file successfully");
    assert!(path.exists(), "file should exist");
}

#[test]
fn test_load_entropy_file() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let path = temp_dir.path().join("test_entropy.bin");

    let original_data = vec![1, 2, 3, 4, 5];
    save_entropy_file(&original_data, path.to_str().unwrap()).expect("save");

    let loaded_data = load_entropy_file(path.to_str().unwrap()).expect("load");
    assert_eq!(original_data, loaded_data, "data should match");
}

#[test]
fn test_load_nonexistent_file() {
    let result = load_entropy_file("/nonexistent/path/to/file.bin");
    assert!(result.is_err(), "should error on nonexistent file");
}

// ============================================================================
// CONCURRENT ENTROPY COLLECTION TESTS
// ============================================================================

#[tokio::test]
async fn test_concurrent_entropy_discovery() {
    // Multiple threads discovering HSMs concurrently
    // This simulates real-world concurrent entropy collection
    let handles: Vec<_> = (0..20)
        .map(|_| {
            tokio::spawn(async move {
                let hsms = hsm_agnostic::discover_all_hsms().await?;
                Ok::<usize, beardog_errors::BearDogError>(hsms.len())
            })
        })
        .collect();

    for handle in handles {
        let result = handle.await.expect("task should not panic");
        assert!(result.is_ok(), "concurrent entropy discovery should work");
    }
}

// ============================================================================
// ROBUSTNESS TESTS (No Sleeps, Pure Concurrency)
// ============================================================================

#[tokio::test]
async fn test_rapid_entropy_quality_calculations() {
    // Stress test: concurrent entropy quality calculations
    let test_data = [vec![0u8; 256], vec![1u8; 256], (0..=255).collect()];

    let handles: Vec<_> = (0..50)
        .map(|i| {
            let data = test_data[i % 3].clone();
            tokio::task::spawn_blocking(move || calculate_entropy_quality(&data))
        })
        .collect();

    for handle in handles {
        let quality = handle.await.expect("task should not panic");
        assert!((0.0..=1.0).contains(&quality), "quality in valid range");
    }
}

#[tokio::test]
async fn test_entropy_operations_under_load() {
    // Test entropy operations while system is under load
    let _background: Vec<_> = (0..20)
        .map(|_| {
            tokio::spawn(async move {
                for _ in 0..100 {
                    tokio::task::yield_now().await;
                }
            })
        })
        .collect();

    // HSM discovery should work under load
    let result = hsm_agnostic::discover_all_hsms().await;
    assert!(result.is_ok(), "entropy operations should work under load");

    // Entropy quality calculation should work under load
    let data: Vec<u8> = (0..=255).collect();
    let quality = calculate_entropy_quality(&data);
    assert!(quality > 0.9, "quality calculation should work under load");
}

// ============================================================================
// DATA INTEGRITY TESTS (Concurrent File Operations)
// ============================================================================

#[test]
fn test_concurrent_file_saves() {
    use std::sync::Arc;
    use std::thread;

    let temp_dir = Arc::new(TempDir::new().expect("create temp dir"));

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let temp_dir = Arc::clone(&temp_dir);
            thread::spawn(move || {
                let path = temp_dir.path().join(format!("entropy_{i}.bin"));
                let data: Vec<u8> = (0..=255).collect();
                save_entropy_file(&data, path.to_str().unwrap())
            })
        })
        .collect();

    for handle in handles {
        let result = handle.join().expect("thread should not panic");
        assert!(result.is_ok(), "concurrent file saves should succeed");
    }
}

#[test]
fn test_concurrent_file_loads() {
    use std::sync::Arc;
    use std::thread;

    let temp_dir = TempDir::new().expect("create temp dir");

    // Create test files
    for i in 0..10 {
        let path = temp_dir.path().join(format!("entropy_{i}.bin"));
        let data: Vec<u8> = (0..=255).collect();
        save_entropy_file(&data, path.to_str().unwrap()).expect("save");
    }

    let temp_dir = Arc::new(temp_dir);

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let temp_dir = Arc::clone(&temp_dir);
            thread::spawn(move || {
                let path = temp_dir.path().join(format!("entropy_{i}.bin"));
                load_entropy_file(path.to_str().unwrap())
            })
        })
        .collect();

    for handle in handles {
        let result = handle.join().expect("thread should not panic");
        assert!(result.is_ok(), "concurrent file loads should succeed");
        let data = result.unwrap();
        assert_eq!(data.len(), 256, "data should be complete");
    }
}
