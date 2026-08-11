// SPDX-License-Identifier: AGPL-3.0-or-later
//! Additional coverage tests: mock ADB runner branches, deployment config, optimization flags.

#![allow(clippy::expect_used, clippy::unwrap_used, reason = "expect/unwrap acceptable for invariant failures in tests and bootstrap code")]

use crate::command_runner::CommandRunner;
use crate::command_runner::mock::MockAdbCommandRunner;
use crate::optimization::{
    BuildFeatures, DeploymentOptimizationConfig, OptimizationLevel, OptimizationSettings,
};
use crate::{DeploymentConfig, DeploymentManager, DeviceManager};
use std::io::ErrorKind;
use std::time::Duration;

// --- DeploymentManager ---

#[test]
fn deployment_manager_initialize_success_non_empty_env() {
    let mgr = DeploymentManager::new(DeploymentConfig::default());
    assert!(mgr.initialize().is_ok());
}

#[test]
fn deployment_config_default_matches_documented_defaults() {
    let c = DeploymentConfig::default();
    assert_eq!(c.environment, "development");
    assert_eq!(c.region, "local");
    assert_eq!(c.instance_count, 1);
    assert!(c.monitoring_enabled);
}

#[test]
fn deployment_config_debug_is_stable_shape() {
    let s = format!("{:?}", DeploymentConfig::default());
    assert!(s.contains("DeploymentConfig"));
}

// --- Optimization branches ---

#[test]
fn optimization_rustc_release_max_opt_with_feature_lto_fat() {
    let cfg = DeploymentOptimizationConfig {
        features: BuildFeatures {
            parallel_builds: false,
            incremental_builds: false,
            optimization: OptimizationSettings {
                lto: true,
                strip_symbols: false,
            },
        },
        target_cpu_optimization: Some("generic".to_string()),
        optimization_level: OptimizationLevel::ReleaseMaxOpt,
    };
    let flags = cfg.get_rustc_flags();
    assert!(flags.iter().any(|f| f.contains("opt-level=3")));
    assert!(flags.contains(&"-C lto=fat".to_string()));
    assert!(!flags.iter().any(|f| f.contains("strip=")));
    assert!(flags.iter().any(|f| f.contains("target-cpu=generic")));
}

#[test]
fn optimization_cargo_flags_debug_skips_release_and_incremental_when_disabled() {
    let cfg = DeploymentOptimizationConfig {
        features: BuildFeatures {
            parallel_builds: false,
            incremental_builds: false,
            optimization: OptimizationSettings::default(),
        },
        target_cpu_optimization: None,
        optimization_level: OptimizationLevel::Debug,
    };
    let flags = cfg.get_cargo_flags();
    assert!(!flags.contains(&"--release".to_string()));
    assert!(!flags.contains(&"--incremental".to_string()));
}

#[test]
fn optimization_cargo_flags_release_without_incremental() {
    let cfg = DeploymentOptimizationConfig {
        features: BuildFeatures {
            parallel_builds: true,
            incremental_builds: false,
            optimization: OptimizationSettings::default(),
        },
        target_cpu_optimization: None,
        optimization_level: OptimizationLevel::Release,
    };
    let flags = cfg.get_cargo_flags();
    assert!(flags.contains(&"--release".to_string()));
    assert!(!flags.contains(&"--incremental".to_string()));
}

#[test]
fn optimization_level_json_roundtrip_all_variants() {
    for level in [
        OptimizationLevel::Debug,
        OptimizationLevel::Release,
        OptimizationLevel::ReleaseLto,
        OptimizationLevel::ReleaseMaxOpt,
    ] {
        let json = serde_json::to_string(&level).expect("serialize");
        let back: OptimizationLevel = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(format!("{level:?}"), format!("{back:?}"));
    }
}

// --- MockAdbCommandRunner error paths ---

#[test]
fn mock_adb_devices_list_failure_returns_stderr() {
    let m = MockAdbCommandRunner {
        fail_adb_devices_exit: true,
        ..Default::default()
    };
    let out = m.run("adb", &["devices", "-l"]).expect("ok status struct");
    assert!(!out.status.success());
    assert!(!out.stderr.is_empty());
}

#[test]
fn mock_adb_install_failure_surfaces_install_failed() {
    let m = MockAdbCommandRunner {
        fail_adb_install: true,
        ..Default::default()
    };
    let out = m
        .run("adb", &["-s", "dev", "install", "x.apk"])
        .expect("output");
    assert!(!out.status.success());
}

#[test]
fn mock_adb_install_ambiguous_stdout_without_success_marker() {
    let m = MockAdbCommandRunner {
        install_stdout_without_success_marker: true,
        ..Default::default()
    };
    let out = m
        .run("adb", &["-s", "dev", "install", "x.apk"])
        .expect("output");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(!s.contains("Success"));
}

#[test]
fn mock_adb_getprop_failure_returns_nonzero() {
    let m = MockAdbCommandRunner {
        fail_getprop: true,
        ..Default::default()
    };
    let out = m
        .run(
            "adb",
            &["-s", "dev", "shell", "getprop", "ro.build.version.sdk"],
        )
        .expect("output");
    assert!(!out.status.success());
}

#[test]
fn mock_adb_logcat_snapshot_failure() {
    let m = MockAdbCommandRunner {
        fail_logcat_snapshot: true,
        ..Default::default()
    };
    let out = m
        .run("adb", &["-s", "dev", "logcat", "-d"])
        .expect("output");
    assert!(!out.status.success());
}

#[test]
fn mock_adb_shell_am_start_failure() {
    let m = MockAdbCommandRunner {
        fail_shell_am: true,
        ..Default::default()
    };
    let out = m
        .run("adb", &["-s", "dev", "shell", "am", "start", "-n", "x"])
        .expect("output");
    assert!(!out.status.success());
}

#[test]
fn mock_adb_pm_list_features_io_error() {
    let m = MockAdbCommandRunner {
        pm_list_features_io_error: true,
        ..Default::default()
    };
    let e = m
        .run("adb", &["-s", "dev", "shell", "pm", "list", "features"])
        .expect_err("io error");
    assert_eq!(e.kind(), ErrorKind::Other);
}

#[test]
fn mock_adb_spawn_failure_missing_binary() {
    let m = MockAdbCommandRunner {
        fail_adb_spawn: true,
        ..Default::default()
    };
    let e = m.run("adb", &["devices", "-l"]).expect_err("spawn");
    assert_eq!(e.kind(), ErrorKind::NotFound);
}

#[test]
fn mock_logcat_follow_bounded_timeout() {
    let m = MockAdbCommandRunner {
        logcat_follow_yields_timeout: true,
        ..Default::default()
    };
    let e = m
        .run_bounded("adb", &["-s", "dev", "logcat"], Duration::from_millis(1))
        .expect_err("timeout");
    assert_eq!(e.kind(), ErrorKind::TimedOut);
}

#[test]
fn mock_logcat_follow_bounded_exit_fail() {
    let m = MockAdbCommandRunner {
        logcat_follow_bounded_exit_fail: true,
        ..Default::default()
    };
    let out = m
        .run_bounded("adb", &["-s", "dev", "logcat"], Duration::from_secs(1))
        .expect("output");
    assert!(!out.status.success());
}

#[test]
fn mock_empty_devices_list_stdout() {
    let m = MockAdbCommandRunner::empty_devices();
    let out = m.run("adb", &["devices", "-l"]).expect("output");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("List of devices"));
}

#[test]
fn device_manager_with_failing_adb_spawn_check_devices_fallback() {
    let m = MockAdbCommandRunner {
        fail_adb_spawn: true,
        ..Default::default()
    };
    let mgr = DeviceManager::with_command_runner(Box::new(m));
    let devs = mgr.check_devices().expect("fallback");
    assert!(!devs.is_empty());
}

#[test]
fn device_manager_with_failing_adb_devices_exit_uses_fallback() {
    let m = MockAdbCommandRunner {
        fail_adb_devices_exit: true,
        ..Default::default()
    };
    let mgr = DeviceManager::with_command_runner(Box::new(m));
    assert!(mgr.check_device().is_ok());
}

#[test]
fn mock_adb_getprop_unknown_property_returns_empty_line() {
    let m = MockAdbCommandRunner::new();
    let out = m
        .run(
            "adb",
            &[
                "-s",
                "emulator-5554",
                "shell",
                "getprop",
                "nonexistent.prop",
            ],
        )
        .expect("output");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert_eq!(s.trim(), "");
}

#[test]
fn mock_adb_getprop_sdk_model_manufacturer() {
    let m = MockAdbCommandRunner::new();
    for (args, needle) in [
        (
            vec![
                "-s",
                "emulator-5554",
                "shell",
                "getprop",
                "ro.build.version.sdk",
            ],
            "33",
        ),
        (
            vec![
                "-s",
                "emulator-5554",
                "shell",
                "getprop",
                "ro.product.model",
            ],
            "Pixel_Test",
        ),
        (
            vec![
                "-s",
                "emulator-5554",
                "shell",
                "getprop",
                "ro.product.manufacturer",
            ],
            "Google",
        ),
    ] {
        let out = m.run("adb", &args).expect("output");
        assert!(out.status.success());
        assert!(String::from_utf8_lossy(&out.stdout).contains(needle));
    }
}

#[test]
fn build_features_serde_roundtrip() {
    let mut bf = BuildFeatures::default();
    bf.parallel_builds = false;
    let j = serde_json::to_string(&bf).expect("ser");
    let back: BuildFeatures = serde_json::from_str(&j).expect("de");
    assert!(!back.parallel_builds);
}

#[test]
fn optimization_settings_debug_non_empty() {
    let s = format!("{:?}", OptimizationSettings::default());
    assert!(s.contains("OptimizationSettings"));
}

#[test]
fn deployment_config_clone_preserves_fields() {
    let a = DeploymentConfig::default();
    let b = a.clone();
    assert_eq!(a.environment, b.environment);
    assert_eq!(a.instance_count, b.instance_count);
}
