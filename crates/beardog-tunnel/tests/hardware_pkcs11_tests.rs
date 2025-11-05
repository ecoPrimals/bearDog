//! Hardware PKCS#11 Integration Tests
//!
//! These tests require real PKCS#11 hardware (e.g., SoloKeys) to be connected.
//! They will be skipped if hardware is not available.
//!
//! To run these tests:
//! ```bash
//! # With hardware connected:
//! cargo test --test hardware_pkcs11_tests -- --ignored
//!
//! # Or set environment variable:
//! BEARDOG_HARDWARE_TESTS=1 cargo test --test hardware_pkcs11_tests
//! ```

use beardog_tunnel::simple_hsm_client::SimplePkcs11Client;
use std::env;

/// Helper to check if hardware tests should run
fn should_run_hardware_tests() -> bool {
    env::var("BEARDOG_HARDWARE_TESTS").is_ok() || cfg!(feature = "hardware-tests")
}

/// Get the default PKCS#11 library path
fn get_pkcs11_library() -> String {
    env::var("BEARDOG_PKCS11_LIB")
        .unwrap_or_else(|_| "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so".to_string())
}

#[test]
#[ignore] // Requires hardware
fn test_pkcs11_client_initialization() {
    if !should_run_hardware_tests() {
        eprintln!("⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)");
        return;
    }

    let library = get_pkcs11_library();
    println!("📚 Testing with library: {}", library);

    let client = SimplePkcs11Client::new(library);
    let result = client.initialize();

    assert!(
        result.is_ok(),
        "Failed to initialize PKCS#11: {:?}",
        result.err()
    );
    println!("✅ PKCS#11 initialized successfully");

    // Cleanup
    let _ = client.finalize();
}

#[test]
#[ignore] // Requires hardware
fn test_list_devices() {
    if !should_run_hardware_tests() {
        eprintln!("⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)");
        return;
    }

    let library = get_pkcs11_library();
    let client = SimplePkcs11Client::new(library);

    client.initialize().expect("Failed to initialize");

    let devices = client.list_devices().expect("Failed to list devices");

    println!("✅ Found {} device(s)", devices.len());
    for (idx, device) in devices.iter().enumerate() {
        println!("  Device #{}", idx + 1);
        println!("    Slot ID: {}", device.slot_id);
        println!("    Label: {}", device.label);
        println!("    Manufacturer: {}", device.manufacturer);
        println!("    Model: {}", device.model);
        println!("    Serial: {}", device.serial_number);
    }

    // Test should pass even with 0 devices (means none connected)
    // But we expect at least 1 for actual hardware testing
    if devices.is_empty() {
        eprintln!("⚠️  No devices found. Connect hardware for full test.");
    }

    client.finalize().expect("Failed to finalize");
}

#[test]
#[ignore] // Requires hardware
fn test_entropy_collection_basic() {
    if !should_run_hardware_tests() {
        eprintln!("⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)");
        return;
    }

    let library = get_pkcs11_library();
    let client = SimplePkcs11Client::new(library);

    client.initialize().expect("Failed to initialize");

    let devices = client.list_devices().expect("Failed to list devices");
    if devices.is_empty() {
        eprintln!("⚠️  No devices found. Skipping entropy test.");
        client.finalize().ok();
        return;
    }

    let slot_id = devices[0].slot_id;
    println!("🎲 Testing entropy collection from slot {}", slot_id);

    let entropy = client
        .collect_entropy(slot_id, 256)
        .expect("Failed to collect entropy");

    assert_eq!(entropy.len(), 256, "Should collect exactly 256 bytes");
    println!("✅ Collected {} bytes of entropy", entropy.len());

    // Basic quality check - should have reasonable variety
    let unique_bytes: std::collections::HashSet<_> = entropy.iter().collect();
    let quality = (unique_bytes.len() as f64 / 256.0) * 100.0;
    println!(
        "📊 Unique byte values: {}/256 ({:.1}%)",
        unique_bytes.len(),
        quality
    );

    assert!(
        unique_bytes.len() > 50,
        "Entropy quality too low: only {} unique bytes",
        unique_bytes.len()
    );

    client.finalize().expect("Failed to finalize");
}

#[test]
#[ignore] // Requires hardware
fn test_entropy_collection_various_sizes() {
    if !should_run_hardware_tests() {
        eprintln!("⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)");
        return;
    }

    let library = get_pkcs11_library();
    let client = SimplePkcs11Client::new(library);

    client.initialize().expect("Failed to initialize");

    let devices = client.list_devices().expect("Failed to list devices");
    if devices.is_empty() {
        eprintln!("⚠️  No devices found. Skipping test.");
        client.finalize().ok();
        return;
    }

    let slot_id = devices[0].slot_id;

    // Test various sizes
    for size in [16, 64, 256, 1024, 4096] {
        println!("🎲 Testing {} bytes...", size);
        let entropy = client
            .collect_entropy(slot_id, size)
            .unwrap_or_else(|_| panic!("Failed to collect {} bytes", size));

        assert_eq!(entropy.len(), size, "Should collect exactly {} bytes", size);
        println!("  ✅ Collected {} bytes", entropy.len());
    }

    client.finalize().expect("Failed to finalize");
}

#[test]
#[ignore] // Requires hardware
fn test_multiple_device_collection() {
    if !should_run_hardware_tests() {
        eprintln!("⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)");
        return;
    }

    let library = get_pkcs11_library();
    let client = SimplePkcs11Client::new(library);

    client.initialize().expect("Failed to initialize");

    let devices = client.list_devices().expect("Failed to list devices");
    println!("✅ Found {} device(s)", devices.len());

    if devices.len() < 2 {
        eprintln!("⚠️  Need at least 2 devices for this test. Skipping.");
        client.finalize().ok();
        return;
    }

    // Collect from multiple devices
    for device in devices.iter().take(4) {
        println!("🎲 Collecting from slot {}...", device.slot_id);
        let entropy = client
            .collect_entropy(device.slot_id, 256)
            .unwrap_or_else(|_| panic!("Failed from slot {}", device.slot_id));

        assert_eq!(entropy.len(), 256);
        println!("  ✅ Collected {} bytes", entropy.len());
    }

    client.finalize().expect("Failed to finalize");
}

#[test]
#[ignore] // Requires hardware
fn test_entropy_quality_distribution() {
    if !should_run_hardware_tests() {
        eprintln!("⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)");
        return;
    }

    let library = get_pkcs11_library();
    let client = SimplePkcs11Client::new(library);

    client.initialize().expect("Failed to initialize");

    let devices = client.list_devices().expect("Failed to list devices");
    if devices.is_empty() {
        eprintln!("⚠️  No devices found. Skipping test.");
        client.finalize().ok();
        return;
    }

    let slot_id = devices[0].slot_id;
    println!("📊 Testing entropy quality from slot {}", slot_id);

    // Collect a larger sample
    let entropy = client
        .collect_entropy(slot_id, 4096)
        .expect("Failed to collect entropy");

    // Count byte frequency
    let mut frequency = [0u32; 256];
    for &byte in &entropy {
        frequency[byte as usize] += 1;
    }

    // All bytes should appear at least once in 4KB
    let unique_bytes = frequency.iter().filter(|&&count| count > 0).count();
    println!("  Unique bytes: {}/256", unique_bytes);

    // Check for reasonable distribution (not too uniform, not too skewed)
    let avg_frequency = 4096.0 / 256.0; // ~16
    let mut chi_square = 0.0;
    for &count in &frequency {
        let diff = count as f64 - avg_frequency;
        chi_square += (diff * diff) / avg_frequency;
    }
    println!("  Chi-square: {:.2}", chi_square);

    // Chi-square test (loose bounds for hardware RNG)
    // For 255 degrees of freedom, values between 200-320 are reasonable
    assert!(
        unique_bytes >= 200,
        "Poor entropy: only {} unique bytes",
        unique_bytes
    );
    assert!(
        chi_square < 500.0,
        "Distribution too skewed: chi-square = {:.2}",
        chi_square
    );

    println!("✅ Entropy quality acceptable");

    client.finalize().expect("Failed to finalize");
}

#[test]
#[ignore] // Requires hardware
fn test_concurrent_access() {
    if !should_run_hardware_tests() {
        eprintln!("⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)");
        return;
    }

    let library = get_pkcs11_library();
    let client = SimplePkcs11Client::new(library);

    client.initialize().expect("Failed to initialize");

    let devices = client.list_devices().expect("Failed to list devices");
    if devices.is_empty() {
        eprintln!("⚠️  No devices found. Skipping test.");
        client.finalize().ok();
        return;
    }

    let slot_id = devices[0].slot_id;

    // Note: SimplePkcs11Client isn't Arc'd, so we can't test true concurrency
    // But we can test sequential access patterns
    println!("🔄 Testing sequential access pattern...");

    for i in 0..10 {
        let entropy = client
            .collect_entropy(slot_id, 128)
            .unwrap_or_else(|_| panic!("Failed on iteration {}", i));
        assert_eq!(entropy.len(), 128);
        println!("  Iteration {}: ✅", i + 1);
    }

    println!("✅ Sequential access successful");

    client.finalize().expect("Failed to finalize");
}

#[test]
#[ignore] // Requires hardware
fn test_error_handling_invalid_slot() {
    if !should_run_hardware_tests() {
        eprintln!("⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)");
        return;
    }

    let library = get_pkcs11_library();
    let client = SimplePkcs11Client::new(library);

    client.initialize().expect("Failed to initialize");

    println!("🧪 Testing error handling with invalid slot...");

    // Try to access a slot that likely doesn't exist
    let result = client.collect_entropy(9999, 256);

    assert!(
        result.is_err(),
        "Should fail with invalid slot, but got: {:?}",
        result
    );
    println!("✅ Correctly handled invalid slot");

    client.finalize().expect("Failed to finalize");
}

#[test]
fn test_client_creation_without_hardware() {
    // This test should always pass - no hardware required
    let client = SimplePkcs11Client::new("/usr/lib/opensc-pkcs11.so".to_string());

    // Should be able to create client even without library present
    // Initialization will fail, but creation should succeed
    println!("✅ Client creation successful (no hardware required)");

    // Don't initialize or use - just testing creation
    drop(client);
}

#[test]
#[ignore] // Requires hardware
fn test_reinitialize_after_finalize() {
    if !should_run_hardware_tests() {
        eprintln!("⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)");
        return;
    }

    let library = get_pkcs11_library();
    let client = SimplePkcs11Client::new(library);

    println!("🔄 Testing re-initialization...");

    // First initialization
    client.initialize().expect("First init failed");
    println!("  ✅ First init");

    // Finalize
    client.finalize().expect("Finalize failed");
    println!("  ✅ Finalized");

    // Try to re-initialize (may fail depending on PKCS#11 implementation)
    // This is okay - some implementations don't support re-init
    match client.initialize() {
        Ok(_) => println!("  ✅ Re-init succeeded"),
        Err(e) => println!("  ⚠️  Re-init not supported: {}", e),
    }

    // Final cleanup
    let _ = client.finalize();
}

// Integration test: simulate full CLI workflow
#[test]
#[ignore] // Requires hardware
fn test_full_workflow_discovery_to_entropy() {
    if !should_run_hardware_tests() {
        eprintln!("⏭️  Skipping hardware test (set BEARDOG_HARDWARE_TESTS=1 to run)");
        return;
    }

    println!("🔬 Running full workflow test...\n");

    let library = get_pkcs11_library();
    let client = SimplePkcs11Client::new(library.clone());

    // Step 1: Initialize
    println!("1️⃣  Initializing PKCS#11...");
    client.initialize().expect("Failed to initialize");
    println!("   ✅ Initialized\n");

    // Step 2: Discover devices
    println!("2️⃣  Discovering devices...");
    let devices = client.list_devices().expect("Failed to list devices");
    println!("   ✅ Found {} device(s)\n", devices.len());

    if devices.is_empty() {
        eprintln!("⚠️  No devices found. Stopping workflow test.");
        client.finalize().ok();
        return;
    }

    // Step 3: Display device info
    println!("3️⃣  Device details:");
    for (idx, device) in devices.iter().enumerate() {
        println!("   Device #{}: {}", idx + 1, device.label);
        println!("     Slot: {}", device.slot_id);
        println!("     Manufacturer: {}", device.manufacturer);
        println!("     Model: {}", device.model);
    }
    println!();

    // Step 4: Collect entropy from first device
    println!("4️⃣  Collecting entropy...");
    let slot_id = devices[0].slot_id;
    let entropy = client
        .collect_entropy(slot_id, 1024)
        .expect("Failed to collect entropy");
    println!("   ✅ Collected {} bytes\n", entropy.len());

    // Step 5: Quality check
    println!("5️⃣  Quality assessment:");
    let unique: std::collections::HashSet<_> = entropy.iter().collect();
    let quality = (unique.len() as f64 / 256.0) * 100.0;
    println!("   Unique bytes: {}/256", unique.len());
    println!("   Quality: {:.1}%", quality);
    if quality > 90.0 {
        println!("   Assessment: ✅ Excellent");
    } else if quality > 70.0 {
        println!("   Assessment: ✅ Good");
    } else {
        println!("   Assessment: ⚠️  Acceptable");
    }
    println!();

    // Step 6: Cleanup
    println!("6️⃣  Cleaning up...");
    client.finalize().expect("Failed to finalize");
    println!("   ✅ Finalized\n");

    println!("✅ Full workflow test complete!");
}
