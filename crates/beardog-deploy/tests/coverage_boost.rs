#![allow(missing_docs, clippy::all)]

//! Integration coverage tests for `beardog-deploy` (fast, deterministic, no sleeps).

use beardog_deploy::android::AndroidDeployment;
use beardog_deploy::builder::RustBuilder;
use beardog_deploy::optimization::{
    BuildFeatures, DeploymentOptimizationConfig, OptimizationLevel, OptimizationSettings,
};
use beardog_deploy::{
    CommandRunner, DeploymentConfig, DeploymentManager, DeviceInfo, DeviceManager, DeviceStatus,
    DeviceType,
};
use beardog_errors::BearDogError;
use std::io;
use std::process::Output;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn exit_ok_status() -> std::process::ExitStatus {
    static ST: OnceLock<std::process::ExitStatus> = OnceLock::new();
    *ST.get_or_init(|| {
        std::process::Command::new("true")
            .status()
            .expect("true exit status")
    })
}

/// Same responses as unit-test `MockAdbCommandRunner` (integration tests cannot import `cfg(test)` mocks).
fn integration_adb_ok(args: &[&str]) -> Result<Output, io::Error> {
    if args.len() >= 2 && args[0] == "devices" && args[1] == "-l" {
        return Ok(Output {
            status: exit_ok_status(),
            stdout: b"List of devices attached\nemulator-5554\tdevice\n".to_vec(),
            stderr: vec![],
        });
    }

    if args.len() >= 5 && args[0] == "-s" && args[2] == "shell" && args[3] == "getprop" {
        let prop = args.get(4).copied().unwrap_or("");
        let val = match prop {
            "ro.build.version.sdk" => "33",
            "ro.product.model" => "Pixel_Test",
            "ro.product.manufacturer" => "Google",
            _ => "",
        };
        return Ok(Output {
            status: exit_ok_status(),
            stdout: format!("{val}\n").into_bytes(),
            stderr: vec![],
        });
    }

    if args.len() >= 6
        && args[0] == "-s"
        && args[2] == "shell"
        && args[3] == "pm"
        && args.get(4) == Some(&"list")
        && args.get(5) == Some(&"features")
    {
        return Ok(Output {
            status: exit_ok_status(),
            stdout: b"feature:android.hardware.strongbox_keystore\nfeature:android.hardware.fingerprint\n"
                .to_vec(),
            stderr: vec![],
        });
    }

    if args.len() >= 4 && args[0] == "-s" && args[2] == "install" {
        return Ok(Output {
            status: exit_ok_status(),
            stdout: b"Success\n".to_vec(),
            stderr: vec![],
        });
    }

    if args.len() >= 4 && args[0] == "-s" && args[2] == "shell" && args[3] == "am" {
        return Ok(Output {
            status: exit_ok_status(),
            stdout: vec![],
            stderr: vec![],
        });
    }

    if args.len() >= 3 && args[0] == "-s" && args[2] == "logcat" {
        return Ok(Output {
            status: exit_ok_status(),
            stdout: vec![],
            stderr: vec![],
        });
    }

    Ok(Output {
        status: exit_ok_status(),
        stdout: vec![],
        stderr: vec![],
    })
}

struct AlwaysFailRunner;

impl CommandRunner for AlwaysFailRunner {
    fn run(&self, _program: &str, _args: &[&str]) -> Result<Output, io::Error> {
        Err(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            "injected failure",
        ))
    }

    fn run_bounded(
        &self,
        _program: &str,
        _args: &[&str],
        _timeout: Duration,
    ) -> Result<Output, io::Error> {
        Err(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            "injected failure",
        ))
    }
}

struct NonSuccessStatusRunner;

fn exit_one() -> std::process::ExitStatus {
    static ST: OnceLock<std::process::ExitStatus> = OnceLock::new();
    *ST.get_or_init(|| {
        std::process::Command::new("false")
            .status()
            .expect("false exit status")
    })
}

impl CommandRunner for NonSuccessStatusRunner {
    fn run(&self, _program: &str, args: &[&str]) -> Result<Output, io::Error> {
        if args.len() >= 2 && args[0] == "devices" && args[1] == "-l" {
            return Ok(Output {
                status: exit_one(),
                stdout: vec![],
                stderr: b"adb failed".to_vec(),
            });
        }
        Ok(Output {
            status: std::process::Command::new("true").status().expect("true"),
            stdout: vec![],
            stderr: vec![],
        })
    }

    fn run_bounded(
        &self,
        program: &str,
        args: &[&str],
        timeout: Duration,
    ) -> Result<Output, io::Error> {
        let _ = timeout;
        self.run(program, args)
    }
}

struct LogcatFailRunner;

impl CommandRunner for LogcatFailRunner {
    fn run(&self, _program: &str, args: &[&str]) -> Result<Output, io::Error> {
        if args.len() >= 3 && args[0] == "-s" && args[2] == "logcat" && args.contains(&"-d") {
            return Ok(Output {
                status: exit_one(),
                stdout: vec![],
                stderr: b"logcat nope".to_vec(),
            });
        }
        integration_adb_ok(args)
    }

    fn run_bounded(
        &self,
        program: &str,
        args: &[&str],
        timeout: Duration,
    ) -> Result<Output, io::Error> {
        let _ = timeout;
        self.run(program, args)
    }
}

struct BoundedTimeoutRunner;

impl CommandRunner for BoundedTimeoutRunner {
    fn run(&self, _program: &str, args: &[&str]) -> Result<Output, io::Error> {
        integration_adb_ok(args)
    }

    fn run_bounded(
        &self,
        _program: &str,
        _args: &[&str],
        _timeout: Duration,
    ) -> Result<Output, io::Error> {
        Err(io::Error::new(io::ErrorKind::TimedOut, "forced timeout"))
    }
}

#[test]
fn deployment_manager_rejects_empty_environment() {
    let _g = ENV_LOCK.lock().expect("env lock");
    let cfg = DeploymentConfig {
        environment: String::new(),
        region: "local".to_string(),
        instance_count: 1,
        monitoring_enabled: false,
    };
    let mgr = DeploymentManager::new(cfg);
    let err = mgr.initialize().expect_err("empty env");
    assert!(
        matches!(err, BearDogError::System { .. }),
        "expected system error: {err:?}"
    );
}

#[test]
fn deployment_manager_default_initializes() {
    let cfg = DeploymentConfig::default();
    let mgr = DeploymentManager::new(cfg);
    mgr.initialize().expect("default config valid");
}

#[test]
fn deployment_config_roundtrip_json() {
    let c = DeploymentConfig {
        environment: "staging".to_string(),
        region: "eu-west".to_string(),
        instance_count: 3,
        monitoring_enabled: true,
    };
    let j = serde_json::to_string(&c).expect("ser");
    let back: DeploymentConfig = serde_json::from_str(&j).expect("de");
    assert_eq!(back.environment, "staging");
    assert_eq!(back.instance_count, 3);
}

#[test]
fn optimization_rustc_and_cargo_flags_all_levels() {
    let dbg = DeploymentOptimizationConfig::development();
    assert!(
        dbg.get_rustc_flags()
            .iter()
            .any(|s| s.contains("opt-level=0"))
    );
    assert!(!dbg.get_cargo_flags().contains(&"--release".to_string()));

    let rel = DeploymentOptimizationConfig::default();
    assert!(rel.get_cargo_flags().contains(&"--release".to_string()));

    let lto = DeploymentOptimizationConfig {
        features: BuildFeatures::default(),
        target_cpu_optimization: None,
        optimization_level: OptimizationLevel::ReleaseLto,
    };
    let rf = lto.get_rustc_flags();
    assert!(rf.iter().any(|s| s.contains("lto=thin")));

    let max = DeploymentOptimizationConfig::production();
    assert!(
        max.get_rustc_flags()
            .iter()
            .any(|s| s.contains("target-cpu=native"))
    );
}

#[test]
fn optimization_settings_and_build_features_serde() {
    let o = OptimizationSettings {
        lto: true,
        strip_symbols: false,
    };
    let json = serde_json::to_string(&o).expect("ser");
    let o2: OptimizationSettings = serde_json::from_str(&json).expect("de");
    assert!(o2.lto);

    let bf = BuildFeatures {
        parallel_builds: false,
        incremental_builds: true,
        optimization: OptimizationSettings::default(),
    };
    let j2 = serde_json::to_string(&bf).expect("ser");
    let bf2: BuildFeatures = serde_json::from_str(&j2).expect("de");
    assert!(!bf2.parallel_builds);
}

#[test]
fn android_deployment_constructors_debug() {
    let tmp = tempfile::tempdir().expect("tmp");
    let ndk = tmp.path().join("ndk");
    std::fs::create_dir_all(&ndk).expect("mkdir");
    let d = AndroidDeployment::new(Some(ndk.to_string_lossy().into_owned()), 33);
    assert!(format!("{d:?}").contains("AndroidDeployment"));
    let d2 = AndroidDeployment::new(None, 21);
    assert!(format!("{d2:?}").contains("None"));
}

#[test]
fn device_manager_detect_android_devices_adb_fails() {
    let mgr = DeviceManager::with_command_runner(Box::new(AlwaysFailRunner));
    let e = mgr.detect_android_devices().expect_err("adb should fail");
    let s = e.to_string();
    assert!(s.contains("adb") || s.contains("Failed"));
}

#[test]
fn device_manager_devices_list_non_success() {
    let mgr = DeviceManager::with_command_runner(Box::new(NonSuccessStatusRunner));
    let e = mgr.detect_android_devices().expect_err("nonzero adb");
    assert!(e.to_string().contains("adb") || e.to_string().contains("failed"));
}

#[test]
fn device_manager_deploy_app_install_failure() {
    let _g = ENV_LOCK.lock().expect("env lock");
    let tmp = tempfile::tempdir().expect("tmp");
    let apk_dir = tmp.path().join("android/app/build/outputs/apk/debug");
    std::fs::create_dir_all(&apk_dir).expect("apk dirs");
    std::fs::write(apk_dir.join("app-debug.apk"), b"x").expect("apk");
    let old = std::env::current_dir().expect("cwd");
    std::env::set_current_dir(tmp.path()).expect("chdir");
    let mgr = DeviceManager::with_command_runner(Box::new(AlwaysFailRunner));
    let e = mgr.deploy_app(false).expect_err("install fails");
    std::env::set_current_dir(old).expect("restore cwd");
    assert!(!e.to_string().is_empty());
}

#[test]
fn device_manager_run_app_launch_failure() {
    let mgr = DeviceManager::with_command_runner(Box::new(AlwaysFailRunner));
    let e = mgr.run_app(&[]).expect_err("launch");
    assert!(!e.to_string().is_empty());
}

#[test]
fn device_manager_show_logs_snapshot_fails() {
    let mgr = DeviceManager::with_command_runner(Box::new(LogcatFailRunner));
    let e = mgr.show_logs("com.example", false).expect_err("logcat");
    assert!(!e.to_string().is_empty());
}

#[test]
fn device_manager_show_logs_follow_bounded_timeout_maps_ok() {
    let mgr = DeviceManager::with_command_runner(Box::new(BoundedTimeoutRunner));
    mgr.show_logs("com.example", true)
        .expect("timeout branch returns Ok");
}

#[test]
fn device_manager_deploy_to_android_bad_output_without_success() {
    struct NoSuccessInstallRunner;
    impl CommandRunner for NoSuccessInstallRunner {
        fn run(&self, _program: &str, args: &[&str]) -> Result<Output, io::Error> {
            if args.len() >= 4 && args[0] == "-s" && args[2] == "install" {
                return Ok(Output {
                    status: std::process::Command::new("true").status().expect("true"),
                    stdout: b"Failure\n".to_vec(),
                    stderr: vec![],
                });
            }
            integration_adb_ok(args)
        }

        fn run_bounded(
            &self,
            _program: &str,
            args: &[&str],
            timeout: Duration,
        ) -> Result<Output, io::Error> {
            let _ = timeout;
            self.run("adb", args)
        }
    }

    let tmp = tempfile::tempdir().expect("tmp");
    let apk = tmp.path().join(format!("bad_{}.apk", std::process::id()));
    std::fs::write(&apk, b"x").expect("write");
    let mgr = DeviceManager::with_command_runner(Box::new(NoSuccessInstallRunner));
    let e = mgr
        .deploy_to_android("emulator-5554", apk.to_str().expect("utf8"))
        .expect_err("no Success in stdout");
    assert!(e.to_string().contains("Deployment") || e.to_string().contains("failed"));
}

#[test]
fn device_info_display_and_clone() {
    let d = DeviceInfo {
        id: "i1".to_string(),
        name: "n".to_string(),
        device_type: DeviceType::Unknown,
        status: DeviceStatus::Error,
        capabilities: vec!["a".to_string()],
        metadata: std::collections::HashMap::new(),
    };
    let dbg = format!("{:?}", d.clone());
    assert!(dbg.contains("i1"));
}

#[tokio::test]
async fn rust_builder_build_android_app_errors_on_empty_project_tree() {
    let tmp = tempfile::tempdir().expect("tmp");
    let b = RustBuilder::new(tmp.path());
    let err = b
        .build_android_app(false, "aarch64-linux-android")
        .await
        .expect_err("no workspace / NDK / android dir");
    assert!(!err.to_string().is_empty());
}
