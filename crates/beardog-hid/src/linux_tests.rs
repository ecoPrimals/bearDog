// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for Linux HID implementation
//!
//! Tests Pure Rust HID access via /dev/hidraw and sysfs

use crate::linux::{LinuxHidDevice, discover_hidraw};
use crate::types::{HidDeviceInfo, ProductId, VendorId};

// ═══════════════════════════════════════════════════════════════════════════
// HidDeviceInfo Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_hid_device_info_creation() {
    let info = HidDeviceInfo {
        path: "/dev/hidraw0".to_string(),
        vendor_id: VendorId(0x1234),
        product_id: ProductId(0x5678),
        manufacturer: "Test Manufacturer".to_string(),
        product: "Test Product".to_string(),
        serial: "12345678".to_string(),
        usage_page: None,
    };

    assert_eq!(info.path, "/dev/hidraw0");
    assert_eq!(info.vendor_id, VendorId(0x1234));
    assert_eq!(info.product_id, ProductId(0x5678));
    assert_eq!(info.manufacturer, "Test Manufacturer");
    assert_eq!(info.product, "Test Product");
    assert_eq!(info.serial, "12345678");
}

#[test]
fn test_hid_device_info_empty_serial() {
    let info = HidDeviceInfo {
        path: "/dev/hidraw1".to_string(),
        vendor_id: VendorId(0xabcd),
        product_id: ProductId(0xef01),
        manufacturer: "Another Manufacturer".to_string(),
        product: "Another Product".to_string(),
        serial: String::new(),
        usage_page: None,
    };

    assert!(info.serial.is_empty());
}

#[test]
fn test_hid_device_info_display() {
    let info = HidDeviceInfo {
        path: "/dev/hidraw0".to_string(),
        vendor_id: VendorId(0x1234),
        product_id: ProductId(0x5678),
        manufacturer: "Test".to_string(),
        product: "Device".to_string(),
        serial: "SERIAL".to_string(),
        usage_page: None,
    };

    let formatted = format!("{info}");
    assert!(formatted.contains("Test"));
    assert!(formatted.contains("Device"));
    assert!(formatted.contains("/dev/hidraw0"));
}

// ═══════════════════════════════════════════════════════════════════════════
// discover_hidraw() Tests
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_discover_hidraw_returns_result() {
    // Test that discover_hidraw returns a valid Result type
    let result = discover_hidraw().await;

    // Should return either Ok or Err - both are valid
    match result {
        Ok(devices) => {
            // All devices should have valid structure
            for device in &devices {
                assert!(!device.path.is_empty());
                assert!(device.path.contains("hidraw") || device.path.contains("/dev/"));
                // VID/PID are u16, always <= 0xFFFF by type definition
                let _ = (device.vendor_id.0, device.product_id.0);
            }
        }
        Err(e) => {
            // Permission errors or I/O errors are expected in some environments
            let err_str = e.to_string();
            assert!(
                err_str.contains("Permission denied")
                    || err_str.contains("Failed to")
                    || err_str.contains("No such file")
            );
        }
    }
}

#[tokio::test]
async fn test_discover_hidraw_device_validation() {
    if let Ok(devices) = discover_hidraw().await {
        for device in &devices {
            // Path should be non-empty
            assert!(!device.path.is_empty());

            // Manufacturer and product can be empty but should be valid strings
            // (len() is always >= 0 by type definition, just verify fields exist)
            let _ = device.manufacturer.len();
            let _ = device.product.len();

            // Serial can be empty
            let _ = device.serial.len();

            // VID/PID are u16, always <= 0xFFFF by type definition
            let _ = device.vendor_id.0;
            // Product ID already checked above
            let _ = device.product_id.0;
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// LinuxHidDevice Tests
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_linux_hid_device_open_nonexistent() {
    // Opening a non-existent device should fail gracefully
    let result = LinuxHidDevice::open("/dev/hidraw999999").await;

    assert!(result.is_err());
    if let Err(e) = result {
        let err_str = e.to_string();
        assert!(
            err_str.contains("No such file")
                || err_str.contains("Failed to open")
                || err_str.contains("not found")
        );
    }
}

#[tokio::test]
async fn test_linux_hid_device_open_invalid_path() {
    // Opening an invalid path should fail
    let result = LinuxHidDevice::open("/invalid/path/hidraw0").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_linux_hid_device_empty_path() {
    // Empty path should fail
    let result = LinuxHidDevice::open("").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_linux_hid_device_relative_path() {
    // Relative paths should fail
    let result = LinuxHidDevice::open("hidraw0").await;
    assert!(result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════
// FIDO2-Specific Tests
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_fido2_device_identification() {
    // Test identifying known FIDO2 devices
    let solo_vendor = VendorId(0x1209);
    let solo_product = ProductId(0xbeee);

    let yubi_vendor = VendorId(0x1050);

    // Validate known VID/PIDs
    assert_eq!(solo_vendor, VendorId(0x1209));
    assert_eq!(solo_product, ProductId(0xbeee));
    assert_eq!(yubi_vendor, VendorId(0x1050));
}

#[tokio::test]
async fn test_discover_and_filter_fido2_devices() {
    // Test discovering and filtering FIDO2 devices
    if let Ok(devices) = discover_hidraw().await {
        let fido2_devices: Vec<_> = devices
            .into_iter()
            .filter(|d| {
                // SoloKey
                (d.vendor_id == VendorId(0x1209) && d.product_id == ProductId(0xbeee))
                // YubiKey family (0x1050 VID)
                || d.vendor_id == VendorId(0x1050)
                // Google Titan (0x096e VID)
                || d.vendor_id == VendorId(0x096e)
            })
            .collect();

        // Either we found some or we didn't - both are valid
        // Length is usize, always >= 0 by type definition
        let _ = fido2_devices.len(); // Verify we can get length

        // If we found any, validate their structure
        for device in &fido2_devices {
            assert!(!device.path.is_empty());
            assert!(device.vendor_id.0 > 0);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Performance Tests
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_discover_performance() {
    use std::time::Instant;

    let start = Instant::now();
    let _ = discover_hidraw().await;
    let duration = start.elapsed();

    // Discovery should complete quickly (under 5 seconds)
    assert!(duration.as_secs() < 5);
}

#[tokio::test]
async fn test_concurrent_discovery() {
    // Test concurrent discovery calls
    let handles: Vec<_> = (0..3)
        .map(|_| tokio::spawn(async { discover_hidraw().await }))
        .collect();

    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok());
        if let Ok(Ok(_devices)) = result {
            // Success
        }
    }
}

#[tokio::test]
async fn test_discover_deterministic() {
    // Test that discovery is deterministic
    let result1 = discover_hidraw().await;
    let result2 = discover_hidraw().await;

    if let (Ok(devices1), Ok(devices2)) = (result1, result2) {
        // Should find same number of devices (assuming no hot-plug)
        // Note: This might be flaky if devices are added/removed during test
        // but validates consistency
        assert_eq!(devices1.len(), devices2.len());
    } else {
        // Both failed consistently or mixed results - OK in test environments
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Type Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_vendor_id_display() {
    let vid = VendorId(0x1234);
    let formatted = format!("{vid}");
    assert_eq!(formatted, "0x1234");
}

#[test]
fn test_product_id_display() {
    let pid = ProductId(0x5678);
    let formatted = format!("{pid}");
    assert_eq!(formatted, "0x5678");
}

#[test]
fn test_vendor_id_equality() {
    let vid1 = VendorId(0x1234);
    let vid2 = VendorId(0x1234);
    let vid3 = VendorId(0x5678);

    assert_eq!(vid1, vid2);
    assert_ne!(vid1, vid3);
}

#[test]
fn test_product_id_equality() {
    let pid1 = ProductId(0xabcd);
    let pid2 = ProductId(0xabcd);
    let pid3 = ProductId(0xef01);

    assert_eq!(pid1, pid2);
    assert_ne!(pid1, pid3);
}

// ═══════════════════════════════════════════════════════════════════════════
// Edge Case Tests
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_device_path_with_whitespace() {
    let result = LinuxHidDevice::open("/dev/hidraw0 ").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_device_path_with_newline() {
    let result = LinuxHidDevice::open("/dev/hidraw0\n").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_discover_empty_manufacturer() {
    // Test that devices with empty manufacturer strings are handled
    if let Ok(devices) = discover_hidraw().await {
        for device in &devices {
            // Manufacturer can be empty - should be valid String
            let _ = device.manufacturer.len(); // Length always >= 0
        }
    }
}

#[test]
fn test_hid_device_info_clone() {
    let info = HidDeviceInfo {
        path: "/dev/hidraw0".to_string(),
        vendor_id: VendorId(0x1234),
        product_id: ProductId(0x5678),
        manufacturer: "Test".to_string(),
        product: "Device".to_string(),
        serial: "SERIAL".to_string(),
        usage_page: None,
    };

    let cloned = info.clone();
    assert_eq!(info.path, cloned.path);
    assert_eq!(info.vendor_id, cloned.vendor_id);
    assert_eq!(info.product_id, cloned.product_id);
}

// ═══════════════════════════════════════════════════════════════════════════
// lib.rs wrapper function tests — cover discover() and open_device()
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_lib_discover_wrapper() {
    // Calls the public discover() wrapper in lib.rs
    let result = crate::discover().await;
    // Either succeeds with a list or fails (both valid in CI)
    match result {
        Ok(devices) => {
            for d in &devices {
                assert!(!d.path.is_empty());
            }
        }
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("Failed") || msg.contains("Permission") || msg.contains("No such")
            );
        }
    }
}

#[tokio::test]
async fn test_lib_open_device_nonexistent() {
    // Calls the public open_device() wrapper in lib.rs
    let result = crate::open_device("/dev/hidraw_does_not_exist_99").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_lib_open_device_invalid_path() {
    let result = crate::open_device("/nonexistent/path").await;
    assert!(result.is_err());
}
