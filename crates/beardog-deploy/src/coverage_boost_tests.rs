// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used)]

//! Targeted coverage tests for `command_runner`, `builder`, and `device` uncovered paths.

use crate::command_runner::mock::MockAdbCommandRunner;
use crate::command_runner::{CommandRunner, SystemCommandRunner};
use crate::device::{DeviceManager, DeviceStatus};
use std::io::ErrorKind;
use std::time::Duration;

#[test]
fn system_command_runner_run_bounded_times_out_on_slow_child() {
    let runner = SystemCommandRunner;
    let err = runner
        .run_bounded("sleep", &["10"], Duration::from_millis(50))
        .expect_err("sleep should exceed bounded timeout");
    assert_eq!(err.kind(), ErrorKind::TimedOut);
}

#[test]
fn system_command_runner_run_bounded_completes_fast_child() {
    let runner = SystemCommandRunner;
    let output = runner
        .run_bounded("echo", &["hello"], Duration::from_secs(5))
        .expect("echo should complete within timeout");
    assert!(output.status.success());
}

#[test]
fn device_manager_deploy_app_missing_apk_returns_error() {
    let manager = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
    let err = manager
        .deploy_app(false)
        .expect_err("deploy_app without APK should fail");
    let msg = format!("{err}");
    assert!(msg.contains("APK not found"), "unexpected error: {msg}");
}

#[test]
fn device_manager_run_app_exercises_mock() {
    let manager = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
    let _ = manager.run_app(&["--test".to_string()]);
}

#[test]
fn device_manager_show_logs_snapshot_success() {
    let manager = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
    manager
        .show_logs("com.beardog.coverage", false)
        .expect("snapshot logcat should succeed with default mock");
}

#[test]
fn device_manager_check_device_returns_connected_from_mock() {
    let manager = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
    let device = manager.check_device().expect("mock device");
    assert_eq!(device.id, "emulator-5554");
    assert_eq!(device.status, DeviceStatus::Connected);
}

#[test]
fn device_manager_check_devices_returns_one_from_mock() {
    let manager = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
    let devices = manager.check_devices().expect("mock devices");
    assert_eq!(devices.len(), 1);
}

#[test]
fn device_manager_empty_devices_falls_back_to_env() {
    let manager =
        DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::empty_devices()));
    let device = manager.check_device().expect("env fallback device");
    assert_eq!(device.id, "local_fallback");
    assert_eq!(device.status, DeviceStatus::Available);
}

#[test]
fn android_deployment_new_with_ndk_path() {
    let deploy = crate::android::AndroidDeployment::new(Some("/opt/ndk".to_string()), 33);
    assert!(format!("{deploy:?}").contains("/opt/ndk"));
}

#[test]
fn android_deployment_new_without_ndk_path() {
    let deploy = crate::android::AndroidDeployment::new(None, 28);
    assert!(format!("{deploy:?}").contains("None"));
}

#[test]
fn rust_builder_new_and_debug() {
    let tmp = tempfile::tempdir().expect("temp dir");
    let builder = crate::builder::RustBuilder::new(tmp.path());
    let dbg = format!("{builder:?}");
    assert!(dbg.contains("RustBuilder"));
}

#[cfg(test)]
#[test]
fn rust_builder_host_architecture_returns_supported() {
    if let Ok(arch) = crate::builder::RustBuilder::host_architecture_for_test() {
        assert!(
            arch.contains("linux") || arch.contains("darwin"),
            "unexpected arch: {arch}"
        );
    }
}

#[test]
fn deployment_config_default_values() {
    let config = crate::DeploymentConfig::default();
    assert_eq!(config.environment, "development");
    assert_eq!(config.region, "local");
    assert_eq!(config.instance_count, 1);
    assert!(config.monitoring_enabled);
}

#[test]
fn deployment_manager_initialize_validates_config() {
    let config = crate::DeploymentConfig::default();
    let manager = crate::DeploymentManager::new(config);
    manager
        .initialize()
        .expect("default config should be valid");
}

#[test]
fn deployment_manager_empty_env_fails_validation() {
    let config = crate::DeploymentConfig {
        environment: String::new(),
        ..crate::DeploymentConfig::default()
    };
    let manager = crate::DeploymentManager::new(config);
    let err = manager.initialize();
    assert!(err.is_err(), "empty environment should fail validation");
}
