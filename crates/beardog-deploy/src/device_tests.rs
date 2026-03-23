// SPDX-License-Identifier: AGPL-3.0-only

use super::*;

#[test]
fn test_device_manager_default_matches_new() {
    assert!(format!("{:?}", DeviceManager::default()).contains("DeviceManager"));
    let _ = DeviceManager::new();
}

#[test]
fn test_device_type_serde_roundtrip() {
    for dt in [
        DeviceType::AndroidStrongBox,
        DeviceType::IosSecureEnclave,
        DeviceType::HardwareHsm,
        DeviceType::SoftwareHsm,
        DeviceType::Unknown,
    ] {
        let json = serde_json::to_string(&dt).expect("DeviceType serializes");
        let back: DeviceType = serde_json::from_str(&json).expect("DeviceType roundtrips");
        assert_eq!(dt, back);
    }
}

#[test]
fn test_device_status_serde_roundtrip() {
    for st in [
        DeviceStatus::Available,
        DeviceStatus::Connected,
        DeviceStatus::Disconnected,
        DeviceStatus::Error,
    ] {
        let json = serde_json::to_string(&st).expect("DeviceStatus serializes");
        let back: DeviceStatus = serde_json::from_str(&json).expect("DeviceStatus roundtrips");
        assert_eq!(st, back);
    }
}

#[test]
fn test_device_info_serde_roundtrip() {
    let info = DeviceInfo {
        id: "id-1".to_string(),
        name: "n".to_string(),
        device_type: DeviceType::HardwareHsm,
        status: DeviceStatus::Connected,
        capabilities: vec!["a".to_string()],
        metadata: HashMap::from([("k".to_string(), "v".to_string())]),
    };
    let json = serde_json::to_string(&info).expect("DeviceInfo serializes");
    let back: DeviceInfo = serde_json::from_str(&json).expect("DeviceInfo roundtrips");
    assert_eq!(info.id, back.id);
    assert_eq!(info.capabilities, back.capabilities);
}

#[test]
fn test_detect_device_type_strongbox_env() {
    let mut map = HashMap::new();
    map.insert("DEVICE_STRONGBOX_CAPABLE".to_string(), "true".to_string());
    let get = |k: &str| map.get(k).cloned();
    assert_eq!(
        DeviceManager::detect_device_type_from_env_with(&get),
        DeviceType::AndroidStrongBox
    );
}

#[test]
fn test_detect_device_type_secure_enclave_env() {
    let mut map = HashMap::new();
    map.insert("DEVICE_STRONGBOX_CAPABLE".to_string(), "false".to_string());
    map.insert(
        "DEVICE_SECURE_ENCLAVE_CAPABLE".to_string(),
        "true".to_string(),
    );
    let get = |k: &str| map.get(k).cloned();
    assert_eq!(
        DeviceManager::detect_device_type_from_env_with(&get),
        DeviceType::IosSecureEnclave
    );
}

#[test]
fn test_detect_device_type_hardware_hsm_env() {
    let mut map = HashMap::new();
    map.insert(
        "DEVICE_HARDWARE_HSM_CAPABLE".to_string(),
        "true".to_string(),
    );
    let get = |k: &str| map.get(k).cloned();
    assert_eq!(
        DeviceManager::detect_device_type_from_env_with(&get),
        DeviceType::HardwareHsm
    );
}

#[test]
fn test_detect_capabilities_from_env_all_flags() {
    let mut map = HashMap::new();
    map.insert("DEVICE_STRONGBOX_CAPABLE".to_string(), "true".to_string());
    map.insert("DEVICE_BIOMETRIC_CAPABLE".to_string(), "true".to_string());
    map.insert(
        "DEVICE_SECURE_STORAGE_CAPABLE".to_string(),
        "true".to_string(),
    );
    let get = |k: &str| map.get(k).cloned();
    let caps = DeviceManager::detect_capabilities_from_env_with(&get);
    assert!(caps.contains(&"strongbox".to_string()));
    assert!(caps.contains(&"biometric_auth".to_string()));
    assert!(caps.contains(&"secure_storage".to_string()));
}

#[test]
fn test_build_device_metadata_env_overrides() {
    let mut map = HashMap::new();
    map.insert("DEVICE_STORAGE_BYTES".to_string(), "2048".to_string());
    map.insert("DEVICE_MANUFACTURER".to_string(), "Acme".to_string());
    map.insert("DEVICE_MODEL".to_string(), "X1".to_string());
    map.insert("DEVICE_OS_VERSION".to_string(), "14".to_string());
    let get = |k: &str| map.get(k).cloned();
    let m = DeviceManager::build_device_metadata_with("dev-xyz", &get);
    assert_eq!(m["device_id"], "dev-xyz");
    assert_eq!(m["storage_available"], "2048");
    assert_eq!(m["manufacturer"], "Acme");
    assert_eq!(m["model"], "X1");
    assert_eq!(m["os_version"], "14");
}

#[test]
fn test_detect_device_type_from_env_default() {
    let get = |_k: &str| -> Option<String> { None };
    assert_eq!(
        DeviceManager::detect_device_type_from_env_with(&get),
        DeviceType::SoftwareHsm
    );
}

#[test]
fn test_detect_capabilities_from_env_defaults() {
    let get = |_k: &str| -> Option<String> { None };
    let caps = DeviceManager::detect_capabilities_from_env_with(&get);
    assert!(caps.contains(&"software_crypto".to_string()));
    assert!(caps.contains(&"basic_auth".to_string()));
}

#[test]
fn test_build_device_metadata_structure() {
    let metadata =
        DeviceManager::build_device_metadata_with("test-123", &|_k| -> Option<String> { None });
    assert_eq!(metadata["device_id"], "test-123");
    assert!(metadata.contains_key("storage_available"));
    assert!(metadata.contains_key("strongbox_supported"));
    assert!(metadata.contains_key("secure_enclave_supported"));
    assert!(metadata.contains_key("manufacturer"));
    assert!(metadata.contains_key("model"));
    assert!(metadata.contains_key("os_version"));
    assert_eq!(metadata.len(), 7);
}

#[test]
fn test_build_device_metadata_various_ids() {
    for id in [
        "a",
        "device-1",
        "pixel8a_strongbox",
        "long-id-with-many-parts",
    ] {
        let m = DeviceManager::build_device_metadata_with(id, &|_| None);
        assert_eq!(m["device_id"], id);
    }
}

#[test]
fn test_check_device_env_fallback_with_empty_mock() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::empty_devices()));
    let d = mgr.check_device().expect("fallback device");
    assert_eq!(d.id, "local_fallback");
}

#[test]
fn test_detect_android_devices_exercises_adb() {
    let mgr = DeviceManager::new();
    let _ = mgr.detect_android_devices();
}

#[test]
fn test_check_device_always_returns_device() {
    let mgr = DeviceManager::new();
    let device = mgr
        .check_device()
        .expect("should always return a device via fallback");
    assert!(!device.id.is_empty());
    assert!(!device.name.is_empty());
    assert!(
        matches!(
            device.status,
            DeviceStatus::Available | DeviceStatus::Connected
        ),
        "status should be Available or Connected"
    );
}

#[test]
fn test_check_devices_returns_nonempty() {
    let mgr = DeviceManager::new();
    let devices = mgr.check_devices().expect("should return devices");
    assert!(!devices.is_empty());
}

#[test]
fn test_deploy_app_debug_no_apk() {
    let mgr = DeviceManager::new();
    let result = mgr.deploy_app(false);
    assert!(result.is_err());
}

#[test]
fn test_deploy_app_release_no_apk() {
    let mgr = DeviceManager::new();
    let result = mgr.deploy_app(true);
    assert!(result.is_err());
}

#[test]
fn test_deploy_to_android_nonexistent_apk() {
    let mgr = DeviceManager::new();
    let result = mgr.deploy_to_android("dev-0", "/nonexistent.apk");
    assert!(result.is_err());
}

#[test]
fn test_run_app_exercises_path() {
    let mgr = DeviceManager::new();
    let _ = mgr.run_app(&[]);
    let _ = mgr.run_app(&["--verbose".to_string()]);
}

#[test]
fn test_show_logs_exercises_path() {
    let mgr = DeviceManager::new();
    let _ = mgr.show_logs("com.beardog.test", false);
    let _ = mgr.show_logs("com.beardog.test", true);
}

#[test]
fn test_device_type_eq_and_clone() {
    let a = DeviceType::AndroidStrongBox;
    let b = a.clone();
    assert_eq!(a, b);
    assert_ne!(a, DeviceType::IosSecureEnclave);
}

#[test]
fn test_device_status_eq_and_clone() {
    let a = DeviceStatus::Available;
    let b = a.clone();
    assert_eq!(a, b);
    assert_ne!(a, DeviceStatus::Error);
}

#[test]
fn test_device_info_debug_format() {
    let info = DeviceInfo {
        id: "dbg-test".to_string(),
        name: "Debug".to_string(),
        device_type: DeviceType::Unknown,
        status: DeviceStatus::Disconnected,
        capabilities: vec![],
        metadata: HashMap::new(),
    };
    let dbg = format!("{:?}", info);
    assert!(dbg.contains("dbg-test"));
    assert!(dbg.contains("Unknown"));
    assert!(dbg.contains("Disconnected"));
}

#[test]
fn test_detect_device_capabilities_exercises_path() {
    let mgr = DeviceManager::new();
    let caps = mgr.detect_device_capabilities("fake-device", false);
    assert!(caps.contains(&"android".to_string()));
    assert!(caps.contains(&"keystore".to_string()));
    assert!(!caps.contains(&"strongbox".to_string()));
}

#[test]
fn test_detect_device_capabilities_with_strongbox() {
    let mgr = DeviceManager::new();
    let caps = mgr.detect_device_capabilities("fake-device", true);
    assert!(caps.contains(&"strongbox".to_string()));
}

#[test]
fn test_has_strongbox_support_exercises_path() {
    let mgr = DeviceManager::new();
    let _ = mgr.has_strongbox_support("fake-device");
}

#[test]
fn test_get_device_property_exercises_path() {
    let mgr = DeviceManager::new();
    let _ = mgr.get_device_property("fake-device", "ro.build.version.sdk");
}

#[test]
fn test_detect_android_devices_mock_emulator_full_parse() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
    let devices = mgr.detect_android_devices().expect("mock adb");
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].id, "emulator-5554");
    assert!(devices[0].metadata.contains_key("api_level"));
    assert_eq!(devices[0].device_type, DeviceType::AndroidStrongBox);
}

#[test]
fn test_check_device_prefers_adb_when_emulator_listed() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
    let d = mgr.check_device().expect("device");
    assert_eq!(d.id, "emulator-5554");
    assert_eq!(d.status, DeviceStatus::Connected);
}

#[test]
fn test_check_devices_via_adb_mock() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
    let v = mgr.check_devices().expect("devices");
    assert_eq!(v.len(), 1);
    assert!(v[0].name.contains("Pixel_Test"));
}

#[test]
fn test_get_device_property_sdk_from_mock() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
    let v = mgr
        .get_device_property("emulator-5554", "ro.build.version.sdk")
        .expect("sdk");
    assert_eq!(v, "33");
}

#[test]
fn test_deploy_to_android_success_with_temp_apk() {
    let mut path = std::env::temp_dir();
    path.push(format!("beardog_deploy_apk_{}.apk", std::process::id()));
    std::fs::write(&path, b"dummy").expect("write apk");
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
    let r = mgr.deploy_to_android("emulator-5554", path.to_str().expect("utf8"));
    let _ = std::fs::remove_file(&path);
    assert!(r.is_ok());
}

#[test]
fn test_deploy_to_android_adb_install_nonzero() {
    let mut path = std::env::temp_dir();
    path.push(format!("beardog_deploy_fail_{}.apk", std::process::id()));
    std::fs::write(&path, b"dummy").expect("write apk");
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner {
        fail_adb_install: true,
        ..MockAdbCommandRunner::new()
    }));
    let r = mgr.deploy_to_android("emulator-5554", path.to_str().expect("utf8"));
    let _ = std::fs::remove_file(&path);
    assert!(r.is_err());
}

#[test]
fn test_deploy_to_android_stdout_without_success_token() {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "beardog_deploy_ambiguous_{}.apk",
        std::process::id()
    ));
    std::fs::write(&path, b"dummy").expect("write apk");
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner {
        install_stdout_without_success_marker: true,
        ..MockAdbCommandRunner::new()
    }));
    let r = mgr.deploy_to_android("emulator-5554", path.to_str().expect("utf8"));
    let _ = std::fs::remove_file(&path);
    assert!(r.is_err());
}

#[test]
fn test_get_device_property_shell_failure() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner {
        fail_getprop: true,
        ..MockAdbCommandRunner::new()
    }));
    let r = mgr.get_device_property("emulator-5554", "ro.build.version.sdk");
    assert!(r.is_err());
}

#[test]
fn test_show_logs_snapshot_logcat_error() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner {
        fail_logcat_snapshot: true,
        ..MockAdbCommandRunner::new()
    }));
    let r = mgr.show_logs("com.beardog.test", false);
    assert!(r.is_err());
}

#[test]
fn test_show_logs_follow_timeout_treated_as_success() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner {
        logcat_follow_yields_timeout: true,
        ..MockAdbCommandRunner::new()
    }));
    let r = mgr.show_logs("com.beardog.test", true);
    assert!(r.is_ok());
}

#[test]
fn test_detect_android_devices_spawn_failure() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner {
        fail_adb_spawn: true,
        ..MockAdbCommandRunner::new()
    }));
    let err = mgr
        .detect_android_devices()
        .expect_err("adb spawn should fail");
    let msg = err.to_string();
    assert!(
        msg.contains("Failed to execute adb") || msg.contains("adb"),
        "unexpected error: {msg}"
    );
}

#[test]
fn test_detect_android_devices_list_command_nonzero() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner {
        fail_adb_devices_exit: true,
        ..MockAdbCommandRunner::new()
    }));
    let err = mgr
        .detect_android_devices()
        .expect_err("adb devices -l should fail");
    assert!(
        err.to_string().contains("adb command failed"),
        "unexpected: {}",
        err
    );
}

#[test]
fn test_run_app_shell_am_failure() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner {
        fail_shell_am: true,
        ..MockAdbCommandRunner::new()
    }));
    let err = mgr.run_app(&[]).expect_err("am start should fail");
    assert!(
        err.to_string().contains("App launch failed") || err.to_string().contains("failed"),
        "unexpected: {}",
        err
    );
}

#[test]
fn test_show_logs_follow_bounded_exit_failure() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner {
        logcat_follow_bounded_exit_fail: true,
        ..MockAdbCommandRunner::new()
    }));
    let err = mgr
        .show_logs("com.beardog.test", true)
        .expect_err("logcat follow should report failure");
    assert!(
        err.to_string().contains("Logcat exited with error"),
        "unexpected: {}",
        err
    );
}

#[test]
fn test_has_strongbox_false_when_pm_list_io_error() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner {
        pm_list_features_io_error: true,
        ..MockAdbCommandRunner::new()
    }));
    assert!(!mgr.has_strongbox_support("emulator-5554"));
}

#[test]
fn test_detect_android_devices_parses_device_skips_offline() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
    let devices = mgr
        .detect_android_devices()
        .expect("mock adb lists one online device");
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].id, "emulator-5554");
}

#[test]
fn test_get_device_property_unknown_prop_returns_empty() {
    let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
    let v = mgr
        .get_device_property("emulator-5554", "ro.nonexistent.prop")
        .expect("getprop succeeds with empty value");
    assert!(v.is_empty());
}
