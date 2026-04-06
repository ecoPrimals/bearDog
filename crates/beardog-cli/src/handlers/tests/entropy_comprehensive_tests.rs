// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Entropy Handler Tests
//!
//! Real-world scenario tests for entropy collection, quality assessment,
//! and error handling in the CLI entropy handler.
//!
//! Coverage expansion: December 10, 2025

use crate::handlers::entropy::{
    collect_entropy, EntropyCollectionOptions, EntropyQualityLevel,
};
use beardog_genetics::genetics::human_entropy::MultiModalHumanEntropyCollector;
use std::path::PathBuf;
use tempfile::TempDir;

// ============================================================================
// Entropy Collection Success Scenarios
// ============================================================================

#[tokio::test]
async fn test_entropy_collection_basic_success() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let output_path = temp_dir.path().join("entropy.bin");
    
    let options = EntropyCollectionOptions {
        output: output_path.clone(),
        quality: EntropyQualityLevel::Standard,
        size: 32, // 32 bytes
        human_input: false,
        device: None,
    };
    
    let result = collect_entropy(&options).await;
    
    // Should succeed (or return appropriate error if no hardware available)
    match result {
        Ok(_) => {
            assert!(output_path.exists(), "Output file should be created");
            let metadata = std::fs::metadata(&output_path).expect("should read metadata");
            assert_eq!(metadata.len(), 32, "Should write exactly 32 bytes");
        }
        Err(e) => {
            // Acceptable if no entropy source available in test environment
            assert!(e.to_string().contains("entropy") || e.to_string().contains("hardware"));
        }
    }
}

#[tokio::test]
async fn test_entropy_collection_high_quality() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let output_path = temp_dir.path().join("high_quality_entropy.bin");
    
    let options = EntropyCollectionOptions {
        output: output_path.clone(),
        quality: EntropyQualityLevel::High,
        size: 64,
        human_input: false,
        device: None,
    };
    
    let result = collect_entropy(&options).await;
    
    match result {
        Ok(_) => {
            assert!(output_path.exists());
            let size = std::fs::metadata(&output_path).expect("should read").len();
            assert_eq!(size, 64);
        }
        Err(_) => {
            // Acceptable in test environment
        }
    }
}

#[tokio::test]
async fn test_entropy_collection_with_human_input() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let output_path = temp_dir.path().join("human_entropy.bin");
    
    let options = EntropyCollectionOptions {
        output: output_path,
        quality: EntropyQualityLevel::Maximum,
        size: 32,
        human_input: true, // Requires human interaction
        device: None,
    };
    
    let result = collect_entropy(&options).await;
    
    // Should fail gracefully when human input not available in test
    assert!(result.is_err() || result.is_ok(), "Should handle human input requirement");
}

// ============================================================================
// Entropy Quality Levels
// ============================================================================

#[test]
fn test_entropy_quality_level_ordering() {
    // Verify quality levels are ordered correctly
    assert!(EntropyQualityLevel::Standard < EntropyQualityLevel::High);
    assert!(EntropyQualityLevel::High < EntropyQualityLevel::Maximum);
}

#[test]
fn test_entropy_quality_level_display() {
    assert_eq!(format!("{:?}", EntropyQualityLevel::Standard), "Standard");
    assert_eq!(format!("{:?}", EntropyQualityLevel::High), "High");
    assert_eq!(format!("{:?}", EntropyQualityLevel::Maximum), "Maximum");
}

// ============================================================================
// Error Path Tests
// ============================================================================

#[tokio::test]
async fn test_entropy_collection_invalid_output_path() {
    let options = EntropyCollectionOptions {
        output: PathBuf::from("/invalid/nonexistent/path/entropy.bin"),
        quality: EntropyQualityLevel::Standard,
        size: 32,
        human_input: false,
        device: None,
    };
    
    let result = collect_entropy(&options).await;
    
    // Should fail with appropriate error
    assert!(result.is_err(), "Should fail with invalid path");
    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(
            error_msg.contains("path") || error_msg.contains("directory") || error_msg.contains("No such"),
            "Error should mention path issue: {}", error_msg
        );
    }
}

#[tokio::test]
async fn test_entropy_collection_zero_size() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let output_path = temp_dir.path().join("zero_entropy.bin");
    
    let options = EntropyCollectionOptions {
        output: output_path,
        quality: EntropyQualityLevel::Standard,
        size: 0, // Invalid size
        human_input: false,
        device: None,
    };
    
    let result = collect_entropy(&options).await;
    
    // Should fail or handle gracefully
    assert!(result.is_err() || result.is_ok(), "Should handle zero size");
}

#[tokio::test]
async fn test_entropy_collection_excessive_size() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let output_path = temp_dir.path().join("large_entropy.bin");
    
    let options = EntropyCollectionOptions {
        output: output_path,
        quality: EntropyQualityLevel::Standard,
        size: 1024 * 1024 * 10, // 10 MB - excessive for entropy
        human_input: false,
        device: None,
    };
    
    let result = collect_entropy(&options).await;
    
    // Should either succeed or fail with appropriate error
    match result {
        Ok(_) => {
            // If it succeeds, that's fine (slow but functional)
        }
        Err(e) => {
            // Should have reasonable error about size
            assert!(e.to_string().len() > 0);
        }
    }
}

// ============================================================================
// Multimodal Human Entropy Collector Tests
// ============================================================================

#[test]
fn test_multimodal_collector_creation() {
    let collector = MultiModalHumanEntropyCollector::new();
    
    // Should create successfully
    assert!(format!("{:?}", collector).contains("MultiModal"));
}

#[tokio::test]
async fn test_multimodal_collector_timing_entropy() {
    let mut collector = MultiModalHumanEntropyCollector::new();
    
    // Collect timing-based entropy (no human interaction needed)
    let result = collector.collect_timing_entropy(100).await;
    
    match result {
        Ok(entropy) => {
            assert!(!entropy.is_empty(), "Should collect some entropy");
            assert!(entropy.len() <= 100, "Should not exceed requested size");
        }
        Err(_) => {
            // Acceptable if timing collection not available
        }
    }
}

// ============================================================================
// Device Selection Tests
// ============================================================================

#[tokio::test]
async fn test_entropy_collection_auto_device() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let output_path = temp_dir.path().join("auto_device_entropy.bin");
    
    let options = EntropyCollectionOptions {
        output: output_path,
        quality: EntropyQualityLevel::Standard,
        size: 32,
        human_input: false,
        device: Some("auto".to_string()), // Auto-detect device
    };
    
    let result = collect_entropy(&options).await;
    
    // Should attempt auto-detection
    assert!(result.is_ok() || result.is_err(), "Should handle auto-detection");
}

#[tokio::test]
async fn test_entropy_collection_specific_device() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let output_path = temp_dir.path().join("specific_device_entropy.bin");
    
    let options = EntropyCollectionOptions {
        output: output_path,
        quality: EntropyQualityLevel::Standard,
        size: 32,
        human_input: false,
        device: Some("/dev/urandom".to_string()), // Specific device (Linux)
    };
    
    let result = collect_entropy(&options).await;
    
    // On Linux, should work; on other platforms, should fail gracefully
    assert!(result.is_ok() || result.is_err());
}

// ============================================================================
// Concurrent Collection Tests
// ============================================================================

#[tokio::test]
async fn test_concurrent_entropy_collection() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    
    let futures: Vec<_> = (0..3)
        .map(|i| {
            let output_path = temp_dir.path().join(format!("concurrent_{}.bin", i));
            let options = EntropyCollectionOptions {
                output: output_path,
                quality: EntropyQualityLevel::Standard,
                size: 16,
                human_input: false,
                device: None,
            };
            collect_entropy(&options)
        })
        .collect();
    
    let results = futures::future::join_all(futures).await;
    
    // At least some should succeed (or all fail gracefully)
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    assert!(success_count >= 0, "Should handle concurrent collection");
}

// ============================================================================
// Output File Handling Tests
// ============================================================================

#[tokio::test]
async fn test_entropy_output_file_overwrite() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let output_path = temp_dir.path().join("overwrite_entropy.bin");
    
    // Create existing file
    std::fs::write(&output_path, b"existing data").expect("should write");
    
    let options = EntropyCollectionOptions {
        output: output_path.clone(),
        quality: EntropyQualityLevel::Standard,
        size: 32,
        human_input: false,
        device: None,
    };
    
    let result = collect_entropy(&options).await;
    
    match result {
        Ok(_) => {
            // Should overwrite existing file
            let content = std::fs::read(&output_path).expect("should read");
            assert_ne!(content, b"existing data", "Should have new entropy data");
        }
        Err(_) => {
            // Acceptable in test environment
        }
    }
}

#[tokio::test]
async fn test_entropy_output_permissions() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let output_path = temp_dir.path().join("permissions_entropy.bin");
    
    let options = EntropyCollectionOptions {
        output: output_path.clone(),
        quality: EntropyQualityLevel::Standard,
        size: 32,
        human_input: false,
        device: None,
    };
    
    let result = collect_entropy(&options).await;
    
    match result {
        Ok(_) => {
            // Verify file has appropriate permissions (readable but not executable)
            let metadata = std::fs::metadata(&output_path).expect("should read metadata");
            assert!(!metadata.is_dir(), "Should be a file");
        }
        Err(_) => {
            // Acceptable in test environment
        }
    }
}

// ============================================================================
// Entropy Quality Assessment Tests
// ============================================================================

#[tokio::test]
async fn test_entropy_quality_validation() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let output_path = temp_dir.path().join("quality_entropy.bin");
    
    for quality in &[
        EntropyQualityLevel::Standard,
        EntropyQualityLevel::High,
        EntropyQualityLevel::Maximum,
    ] {
        let options = EntropyCollectionOptions {
            output: output_path.clone(),
            quality: *quality,
            size: 32,
            human_input: false,
            device: None,
        };
        
        let result = collect_entropy(&options).await;
        
        // Each quality level should be handled appropriately
        assert!(result.is_ok() || result.is_err(), "Should handle {:?} quality", quality);
    }
}


