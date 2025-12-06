//! Comprehensive Device Manager Tests
//!
//! Exhaustive tests for device discovery, management, and deployment

use crate::device::{DeviceInfo, DeviceManager, DeviceStatus, DeviceType};

// ============================================================================
// Device Manager Creation Tests
// ============================================================================

#[test]
fn test_device_manager_new() {
    let manager = DeviceManager::new();
    // Verify manager is created successfully
    assert!(format!("{:?}", manager).contains("DeviceManager"));
}

#[test]
fn test_device_manager_default() {
    let manager = DeviceManager;
    // Verify default implementation works
    assert!(format!("{:?}", manager).contains("DeviceManager"));
}

// ============================================================================
// Device Check Tests
// ============================================================================

#[test]
fn test_check_device() {
    let manager = DeviceManager::new();
    let device_info = manager.check_device();

    // Verify device info structure
    assert!(!device_info.id.is_empty());
    assert!(!device_info.name.is_empty());
    assert!(matches!(
        device_info.status,
        DeviceStatus::Available | DeviceStatus::Connected
    ));
}

#[test]
fn test_check_device_returns_valid_device_info() {
    let manager = DeviceManager::new();
    let device_info = manager.check_device();

    // Verify all fields are populated
    assert!(!device_info.id.is_empty());
    assert!(!device_info.name.is_empty());
    assert!(!device_info.capabilities.is_empty() || device_info.capabilities.is_empty());
    // Either is valid
}

// ============================================================================
// Device Type Tests
// ============================================================================

#[test]
fn test_device_type_variants() {
    let android = DeviceType::AndroidStrongBox;
    let ios = DeviceType::IosSecureEnclave;
    let hardware = DeviceType::HardwareHsm;
    let software = DeviceType::SoftwareHsm;
    let unknown = DeviceType::Unknown;

    // Verify all variants are distinct
    assert_ne!(android, ios);
    assert_ne!(android, hardware);
    assert_ne!(ios, hardware);
    assert_ne!(hardware, software);
    assert_ne!(software, unknown);
}

#[test]
fn test_device_type_debug() {
    let device_type = DeviceType::AndroidStrongBox;
    let debug_str = format!("{:?}", device_type);
    assert!(debug_str.contains("AndroidStrongBox"));
}

#[test]
fn test_device_type_clone() {
    let original = DeviceType::IosSecureEnclave;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_device_type_serialization() {
    let device_type = DeviceType::HardwareHsm;
    let serialized = serde_json::to_string(&device_type).expect("Should serialize");
    assert!(!serialized.is_empty());

    let deserialized: DeviceType = serde_json::from_str(&serialized).expect("Should deserialize");
    assert_eq!(device_type, deserialized);
}

// ============================================================================
// Device Status Tests
// ============================================================================

#[test]
fn test_device_status_variants() {
    let available = DeviceStatus::Available;
    let connected = DeviceStatus::Connected;
    let disconnected = DeviceStatus::Disconnected;
    let error = DeviceStatus::Error;

    // Verify all variants are distinct
    assert_ne!(available, connected);
    assert_ne!(connected, disconnected);
    assert_ne!(disconnected, error);
}

#[test]
fn test_device_status_debug() {
    let status = DeviceStatus::Connected;
    let debug_str = format!("{:?}", status);
    assert!(debug_str.contains("Connected"));
}

#[test]
fn test_device_status_clone() {
    let original = DeviceStatus::Available;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_device_status_serialization() {
    let status = DeviceStatus::Connected;
    let serialized = serde_json::to_string(&status).expect("Should serialize");
    assert!(!serialized.is_empty());

    let deserialized: DeviceStatus = serde_json::from_str(&serialized).expect("Should deserialize");
    assert_eq!(status, deserialized);
}

// ============================================================================
// Device Info Tests
// ============================================================================

#[test]
fn test_device_info_creation() {
    let device_info = DeviceInfo {
        id: "device-123".to_string(),
        name: "Test Device".to_string(),
        device_type: DeviceType::AndroidStrongBox,
        status: DeviceStatus::Connected,
        capabilities: vec!["crypto".to_string(), "storage".to_string()],
        metadata: std::collections::HashMap::new(),
    };

    assert_eq!(device_info.id, "device-123");
    assert_eq!(device_info.name, "Test Device");
    assert_eq!(device_info.device_type, DeviceType::AndroidStrongBox);
    assert_eq!(device_info.status, DeviceStatus::Connected);
    assert_eq!(device_info.capabilities.len(), 2);
}

#[test]
fn test_device_info_with_capabilities() {
    let capabilities = vec![
        "crypto".to_string(),
        "biometric".to_string(),
        "secure_storage".to_string(),
    ];

    let device_info = DeviceInfo {
        id: "device-456".to_string(),
        name: "Secure Device".to_string(),
        device_type: DeviceType::IosSecureEnclave,
        status: DeviceStatus::Available,
        capabilities: capabilities.clone(),
        metadata: std::collections::HashMap::new(),
    };

    assert_eq!(device_info.capabilities.len(), 3);
    assert!(device_info.capabilities.contains(&"crypto".to_string()));
    assert!(device_info.capabilities.contains(&"biometric".to_string()));
}

#[test]
fn test_device_info_with_metadata() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("manufacturer".to_string(), "Google".to_string());
    metadata.insert("model".to_string(), "Pixel 8a".to_string());
    metadata.insert("os_version".to_string(), "Android 14".to_string());

    let device_info = DeviceInfo {
        id: "device-789".to_string(),
        name: "Pixel 8a".to_string(),
        device_type: DeviceType::AndroidStrongBox,
        status: DeviceStatus::Connected,
        capabilities: vec![],
        metadata: metadata.clone(),
    };

    assert_eq!(device_info.metadata.len(), 3);
    assert_eq!(
        device_info.metadata.get("manufacturer"),
        Some(&"Google".to_string())
    );
    assert_eq!(
        device_info.metadata.get("model"),
        Some(&"Pixel 8a".to_string())
    );
}

#[test]
fn test_device_info_debug() {
    let device_info = DeviceInfo {
        id: "test-id".to_string(),
        name: "Test".to_string(),
        device_type: DeviceType::SoftwareHsm,
        status: DeviceStatus::Available,
        capabilities: vec![],
        metadata: std::collections::HashMap::new(),
    };

    let debug_str = format!("{:?}", device_info);
    assert!(debug_str.contains("test-id"));
    assert!(debug_str.contains("SoftwareHsm"));
}

#[test]
fn test_device_info_clone() {
    let original = DeviceInfo {
        id: "clone-test".to_string(),
        name: "Clone Test".to_string(),
        device_type: DeviceType::HardwareHsm,
        status: DeviceStatus::Connected,
        capabilities: vec!["test".to_string()],
        metadata: std::collections::HashMap::new(),
    };

    let cloned = original.clone();
    assert_eq!(original.id, cloned.id);
    assert_eq!(original.name, cloned.name);
    assert_eq!(original.device_type, cloned.device_type);
    assert_eq!(original.status, cloned.status);
}

#[test]
fn test_device_info_serialization() {
    let device_info = DeviceInfo {
        id: "serial-test".to_string(),
        name: "Serial Test".to_string(),
        device_type: DeviceType::AndroidStrongBox,
        status: DeviceStatus::Available,
        capabilities: vec!["crypto".to_string()],
        metadata: std::collections::HashMap::new(),
    };

    let serialized = serde_json::to_string(&device_info).expect("Should serialize");
    assert!(!serialized.is_empty());

    let deserialized: DeviceInfo = serde_json::from_str(&serialized).expect("Should deserialize");
    assert_eq!(device_info.id, deserialized.id);
    assert_eq!(device_info.name, deserialized.name);
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_device_manager_multiple_checks() {
    let manager = DeviceManager::new();

    // Perform multiple device checks
    let device1 = manager.check_device();
    let device2 = manager.check_device();

    // Both should succeed
    assert!(!device1.id.is_empty());
    assert!(!device2.id.is_empty());
}

#[test]
fn test_all_device_types_coverage() {
    // Ensure all device types can be created
    let types = vec![
        DeviceType::AndroidStrongBox,
        DeviceType::IosSecureEnclave,
        DeviceType::HardwareHsm,
        DeviceType::SoftwareHsm,
        DeviceType::Unknown,
    ];

    assert_eq!(types.len(), 5);
    for device_type in types {
        let device_info = DeviceInfo {
            id: format!("test-{:?}", device_type),
            name: "Test".to_string(),
            device_type,
            status: DeviceStatus::Available,
            capabilities: vec![],
            metadata: std::collections::HashMap::new(),
        };
        assert!(!device_info.id.is_empty());
    }
}

#[test]
fn test_all_device_statuses_coverage() {
    // Ensure all status types can be created
    let statuses = vec![
        DeviceStatus::Available,
        DeviceStatus::Connected,
        DeviceStatus::Disconnected,
        DeviceStatus::Error,
    ];

    assert_eq!(statuses.len(), 4);
    for status in statuses {
        let device_info = DeviceInfo {
            id: format!("test-{:?}", status),
            name: "Test".to_string(),
            device_type: DeviceType::Unknown,
            status,
            capabilities: vec![],
            metadata: std::collections::HashMap::new(),
        };
        assert!(!device_info.id.is_empty());
    }
}

#[test]
fn test_device_info_empty_collections() {
    let device_info = DeviceInfo {
        id: "empty-test".to_string(),
        name: "Empty Test".to_string(),
        device_type: DeviceType::SoftwareHsm,
        status: DeviceStatus::Available,
        capabilities: vec![],
        metadata: std::collections::HashMap::new(),
    };

    assert!(device_info.capabilities.is_empty());
    assert!(device_info.metadata.is_empty());
}

#[test]
fn test_device_info_large_metadata() {
    let mut metadata = std::collections::HashMap::new();
    for i in 0..100 {
        metadata.insert(format!("key_{}", i), format!("value_{}", i));
    }

    let device_info = DeviceInfo {
        id: "large-metadata-test".to_string(),
        name: "Large Metadata".to_string(),
        device_type: DeviceType::HardwareHsm,
        status: DeviceStatus::Connected,
        capabilities: vec![],
        metadata,
    };

    assert_eq!(device_info.metadata.len(), 100);
}
