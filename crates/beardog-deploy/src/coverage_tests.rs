// SPDX-License-Identifier: AGPL-3.0-only

//! Coverage expansion tests for beardog-deploy
//!
//! These tests exercise actual production code paths that were previously
//! untested. All tests are concurrent-safe — no sleeps, no serialization.

use crate::android::AndroidDeployment;
use crate::builder::RustBuilder;
use crate::device::{DeviceInfo, DeviceManager, DeviceStatus, DeviceType};
use crate::DeploymentConfig;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

// ============================================================================
// AndroidDeployment Tests - Exercise actual production code
// ============================================================================

#[test]
fn test_android_deployment_construction_variants() {
    let d1 = AndroidDeployment::new(Some("/opt/android-ndk".to_string()), 33);
    let d2 = AndroidDeployment::new(None, 28);
    let d3 = AndroidDeployment::new(Some(String::new()), 21);
    assert!(format!("{:?}", d1).contains("AndroidDeployment"));
    assert!(format!("{:?}", d2).contains("AndroidDeployment"));
    assert!(format!("{:?}", d3).contains("AndroidDeployment"));
}

#[test]
fn test_android_deployment_various_api_levels() {
    for api_level in [21, 28, 30, 33, 34, 35] {
        let deployment = AndroidDeployment::new(None, api_level);
        let debug = format!("{:?}", deployment);
        assert!(debug.contains(&api_level.to_string()));
    }
}

#[test]
fn test_android_deployment_verify_environment_exercises_path() {
    // Without NDK configured, exercises verify_environment → verify_all_components chain
    let deployment = AndroidDeployment::new(None, 33);
    let _ = deployment.verify_environment();
}

#[test]
fn test_android_deployment_verify_environment_invalid_ndk() {
    let deployment = AndroidDeployment::new(Some("/nonexistent/ndk/path".to_string()), 33);
    let _ = deployment.verify_environment();
}

// ============================================================================
// RustBuilder Tests - Exercise actual production code
// ============================================================================

#[test]
fn test_rust_builder_construction() {
    let paths = ["/tmp", "/home", ".", "./target", "/opt/project"];
    for path in &paths {
        let builder = RustBuilder::new(std::path::Path::new(path));
        assert!(format!("{:?}", builder).contains("RustBuilder"));
    }
}

// ============================================================================
// DeviceManager Tests - Exercise all public methods
// ============================================================================

#[test]
fn test_device_manager_check_device_fallback() {
    let manager = DeviceManager::new();
    let result = manager.check_device();
    assert!(result.is_ok());
    let device = result.unwrap();
    assert!(!device.id.is_empty());
    assert!(!device.name.is_empty());
}

#[test]
fn test_device_manager_check_devices_fallback() {
    let manager = DeviceManager::new();
    let result = manager.check_devices();
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}

#[test]
fn test_device_manager_default_trait() {
    let manager = DeviceManager::default();
    assert!(manager.check_device().is_ok());
}

#[test]
fn test_device_manager_deploy_app_missing_apk_debug() {
    let manager = DeviceManager::new();
    let result = manager.deploy_app(false);
    assert!(result.is_err());
}

#[test]
fn test_device_manager_deploy_app_missing_apk_release() {
    let manager = DeviceManager::new();
    let result = manager.deploy_app(true);
    assert!(result.is_err());
}

#[test]
fn test_device_manager_run_app_exercises_path() {
    let manager = DeviceManager::new();
    let _ = manager.run_app(&["--test".to_string()]);
}

#[test]
fn test_device_manager_show_logs_exercises_path() {
    let manager = DeviceManager::new();
    let _ = manager.show_logs("com.beardog.app", false);
}

#[test]
fn test_device_manager_detect_android_devices_exercises_path() {
    let manager = DeviceManager::new();
    let _ = manager.detect_android_devices();
}

#[test]
fn test_device_manager_deploy_to_android_missing_apk() {
    let manager = DeviceManager::new();
    let result = manager.deploy_to_android("test-device", "/nonexistent/app.apk");
    assert!(result.is_err());
}

// ============================================================================
// Device types serialization roundtrip
// ============================================================================

#[test]
fn test_device_info_full_roundtrip() {
    let mut metadata = HashMap::new();
    metadata.insert("key1".to_string(), "value1".to_string());
    metadata.insert("key2".to_string(), "value2".to_string());

    let original = DeviceInfo {
        id: "roundtrip-test".to_string(),
        name: "Roundtrip Test Device".to_string(),
        device_type: DeviceType::AndroidStrongBox,
        status: DeviceStatus::Connected,
        capabilities: vec!["crypto".to_string(), "strongbox".to_string()],
        metadata,
    };

    let json = serde_json::to_string(&original).unwrap();
    let restored: DeviceInfo = serde_json::from_str(&json).unwrap();
    assert_eq!(original.id, restored.id);
    assert_eq!(original.name, restored.name);
    assert_eq!(original.device_type, restored.device_type);
    assert_eq!(original.status, restored.status);
    assert_eq!(original.capabilities, restored.capabilities);
}

#[test]
fn test_device_type_all_variants_serialize() {
    for variant in [
        DeviceType::AndroidStrongBox,
        DeviceType::IosSecureEnclave,
        DeviceType::HardwareHsm,
        DeviceType::SoftwareHsm,
        DeviceType::Unknown,
    ] {
        let json = serde_json::to_string(&variant).unwrap();
        let restored: DeviceType = serde_json::from_str(&json).unwrap();
        assert_eq!(variant, restored);
    }
}

#[test]
fn test_device_status_all_variants_serialize() {
    for variant in [
        DeviceStatus::Available,
        DeviceStatus::Connected,
        DeviceStatus::Disconnected,
        DeviceStatus::Error,
    ] {
        let json = serde_json::to_string(&variant).unwrap();
        let restored: DeviceStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(variant, restored);
    }
}

// ============================================================================
// Concurrent safety tests — no sleeps, no serial
// ============================================================================

#[test]
fn test_device_manager_concurrent_checks() {
    let manager = Arc::new(DeviceManager::new());
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let mgr = Arc::clone(&manager);
            thread::spawn(move || {
                assert!(mgr.check_device().is_ok());
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn test_deployment_config_concurrent_serialization() {
    let config = Arc::new(DeploymentConfig {
        environment: "production".to_string(),
        region: "us-east-1".to_string(),
        instance_count: 10,
        monitoring_enabled: true,
    });

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let cfg = Arc::clone(&config);
            thread::spawn(move || {
                let json = serde_json::to_string(&*cfg).unwrap();
                let restored: DeploymentConfig = serde_json::from_str(&json).unwrap();
                assert_eq!(restored.instance_count, 10);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn test_device_manager_concurrent_creation() {
    let handles: Vec<_> = (0..8)
        .map(|_| {
            thread::spawn(|| {
                let manager = DeviceManager::default();
                // DeviceManager should be constructible concurrently
                let _ = format!("{:?}", manager);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
