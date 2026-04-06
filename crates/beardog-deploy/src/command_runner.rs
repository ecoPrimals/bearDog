// SPDX-License-Identifier: AGPL-3.0-or-later

//! Pluggable command execution for [`crate::device::DeviceManager`].
//!
//! Production uses [`SystemCommandRunner`]. Unit tests use `MockAdbCommandRunner` from the
//! `command_runner::mock` submodule (compiled only under `#[cfg(test)]`) so the suite never shells out to `adb`.

use std::io;
use std::process::{Command, Output};
use std::time::{Duration, Instant};

/// Runs external programs (default: real `std::process::Command`).
pub trait CommandRunner: Send + Sync {
    /// Run `program` with `args` and wait for completion (captures stdout/stderr).
    fn run(&self, program: &str, args: &[&str]) -> Result<Output, io::Error>;

    /// Long-running command bounded by wall time (e.g. `adb logcat` follow mode).
    fn run_bounded(
        &self,
        program: &str,
        args: &[&str],
        timeout: Duration,
    ) -> Result<Output, io::Error>;
}

/// Production runner: executes `program` with `args`.
#[cfg_attr(test, allow(dead_code))]
pub(crate) struct SystemCommandRunner;

impl CommandRunner for SystemCommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<Output, io::Error> {
        Command::new(program).args(args).output()
    }

    fn run_bounded(
        &self,
        program: &str,
        args: &[&str],
        timeout: Duration,
    ) -> Result<Output, io::Error> {
        use std::thread;

        let mut child = Command::new(program).args(args).spawn()?;
        let start = Instant::now();
        loop {
            if let Some(status) = child.try_wait()? {
                return Ok(Output {
                    status,
                    stdout: vec![],
                    stderr: vec![],
                });
            }

            if start.elapsed() > timeout {
                let _ = child.kill();
                let _ = child.wait();
                return Err(io::Error::new(io::ErrorKind::TimedOut, "command timed out"));
            }

            // Polling backoff: `try_wait` is non-blocking; short sleep avoids a busy spin.
            thread::sleep(Duration::from_millis(50));
        }
    }
}

/// Test-only ADB-shaped [`CommandRunner`] (no subprocess; deterministic stdout).
#[cfg(test)]
pub(crate) mod mock {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::*;
    use std::sync::OnceLock;

    fn exit_ok() -> std::process::ExitStatus {
        static OK: OnceLock<std::process::ExitStatus> = OnceLock::new();
        *OK.get_or_init(|| {
            if cfg!(windows) {
                std::process::Command::new("cmd")
                    .args(["/C", "exit", "0"])
                    .status()
            } else {
                std::process::Command::new("true").status()
            }
            .expect("exit status")
        })
    }

    fn exit_fail() -> std::process::ExitStatus {
        static FAIL: OnceLock<std::process::ExitStatus> = OnceLock::new();
        *FAIL.get_or_init(|| {
            if cfg!(windows) {
                std::process::Command::new("cmd")
                    .args(["/C", "exit", "1"])
                    .status()
            } else {
                std::process::Command::new("false").status()
            }
            .expect("exit status")
        })
    }

    /// Mock ADB responses for tests (no subprocess, no `adb` on PATH required).
    #[derive(Debug, Clone, Default)]
    pub struct MockAdbCommandRunner {
        /// When true, `adb devices -l` returns no devices (exercises env fallback).
        pub empty_devices: bool,
        /// `adb install` exits non-zero (deployment error path).
        pub fail_adb_install: bool,
        /// `adb logcat -d` snapshot exits non-zero.
        pub fail_logcat_snapshot: bool,
        /// `adb shell getprop` exits non-zero.
        pub fail_getprop: bool,
        /// `install` stdout omits `Success` (exercises `deploy_to_android` ambiguous output path).
        pub install_stdout_without_success_marker: bool,
        /// `run_bounded` logcat follow returns `TimedOut` (exercises logcat follow timeout branch).
        pub logcat_follow_yields_timeout: bool,
        /// `run("adb", ...)` returns [`io::Error`] immediately (simulates missing `adb` binary).
        pub fail_adb_spawn: bool,
        /// `adb devices -l` exits non-zero (stderr path in [`crate::device::DeviceManager::detect_android_devices`]).
        pub fail_adb_devices_exit: bool,
        /// `adb shell am start ...` exits non-zero ([`crate::device::DeviceManager::run_app`] error path).
        pub fail_shell_am: bool,
        /// `adb shell pm list features` returns I/O error ([`crate::device::DeviceManager::has_strongbox_support`] Err branch).
        pub pm_list_features_io_error: bool,
        /// `run_bounded` logcat follow exits non-zero (not timeout) ([`crate::device::DeviceManager::show_logs`] follow branch).
        pub logcat_follow_bounded_exit_fail: bool,
    }

    impl MockAdbCommandRunner {
        /// Same as [`Default::default`].
        #[must_use]
        pub const fn new() -> Self {
            Self {
                empty_devices: false,
                fail_adb_install: false,
                fail_logcat_snapshot: false,
                fail_getprop: false,
                install_stdout_without_success_marker: false,
                logcat_follow_yields_timeout: false,
                fail_adb_spawn: false,
                fail_adb_devices_exit: false,
                fail_shell_am: false,
                pm_list_features_io_error: false,
                logcat_follow_bounded_exit_fail: false,
            }
        }

        /// Mock with no devices listed (forces env-based fallback).
        #[must_use]
        pub const fn empty_devices() -> Self {
            Self {
                empty_devices: true,
                fail_adb_install: false,
                fail_logcat_snapshot: false,
                fail_getprop: false,
                install_stdout_without_success_marker: false,
                logcat_follow_yields_timeout: false,
                fail_adb_spawn: false,
                fail_adb_devices_exit: false,
                fail_shell_am: false,
                pm_list_features_io_error: false,
                logcat_follow_bounded_exit_fail: false,
            }
        }
    }

    impl CommandRunner for MockAdbCommandRunner {
        fn run(&self, program: &str, args: &[&str]) -> Result<Output, io::Error> {
            if program != "adb" {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "mock only supports adb",
                ));
            }
            if self.fail_adb_spawn {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "mock adb spawn failed",
                ));
            }
            mock_adb_output(self, args)
        }

        fn run_bounded(
            &self,
            program: &str,
            args: &[&str],
            _timeout: Duration,
        ) -> Result<Output, io::Error> {
            if program != "adb" {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "mock only supports adb",
                ));
            }
            let is_logcat_follow =
                args.len() >= 3 && args[0] == "-s" && args[2] == "logcat" && !args.contains(&"-d");
            if self.logcat_follow_yields_timeout && is_logcat_follow {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "mock logcat follow timeout",
                ));
            }
            if self.logcat_follow_bounded_exit_fail && is_logcat_follow {
                return Ok(Output {
                    status: exit_fail(),
                    stdout: vec![],
                    stderr: b"logcat follow failed".to_vec(),
                });
            }
            Ok(Output {
                status: exit_ok(),
                stdout: vec![],
                stderr: vec![],
            })
        }
    }

    fn mock_adb_output(runner: &MockAdbCommandRunner, args: &[&str]) -> Result<Output, io::Error> {
        if args.len() >= 2 && args[0] == "devices" && args[1] == "-l" {
            if runner.fail_adb_devices_exit {
                return Ok(Output {
                    status: exit_fail(),
                    stdout: vec![],
                    stderr: b"adb: failed to list devices".to_vec(),
                });
            }
            let stdout = if runner.empty_devices {
                b"List of devices attached\n\n".to_vec()
            } else {
                b"List of devices attached\nemulator-5554\tdevice\nemulator-5556\toffline\n"
                    .to_vec()
            };
            return Ok(Output {
                status: exit_ok(),
                stdout,
                stderr: vec![],
            });
        }

        if args.len() >= 5 && args[0] == "-s" && args[2] == "shell" && args[3] == "getprop" {
            if runner.fail_getprop {
                return Ok(Output {
                    status: exit_fail(),
                    stdout: vec![],
                    stderr: b"getprop failed".to_vec(),
                });
            }
            let prop = args.get(4).copied().unwrap_or("");
            let val = match prop {
                "ro.build.version.sdk" => "33",
                "ro.product.model" => "Pixel_Test",
                "ro.product.manufacturer" => "Google",
                _ => "",
            };
            return Ok(Output {
                status: exit_ok(),
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
            if runner.pm_list_features_io_error {
                return Err(io::Error::other("mock pm list features io error"));
            }
            return Ok(Output {
                status: exit_ok(),
                stdout: b"feature:android.hardware.strongbox_keystore\nfeature:android.hardware.fingerprint\n"
                    .to_vec(),
                stderr: vec![],
            });
        }

        if args.len() >= 4 && args[0] == "-s" && args[2] == "install" {
            if runner.fail_adb_install {
                return Ok(Output {
                    status: exit_fail(),
                    stdout: vec![],
                    stderr: b"INSTALL_FAILED".to_vec(),
                });
            }
            let stdout = if runner.install_stdout_without_success_marker {
                b"Installed without expected marker line\n".to_vec()
            } else {
                b"Success\n".to_vec()
            };
            return Ok(Output {
                status: exit_ok(),
                stdout,
                stderr: vec![],
            });
        }

        if args.len() >= 4 && args[0] == "-s" && args[2] == "shell" && args[3] == "am" {
            if runner.fail_shell_am {
                return Ok(Output {
                    status: exit_fail(),
                    stdout: vec![],
                    stderr: b"am start failed".to_vec(),
                });
            }
            return Ok(Output {
                status: exit_ok(),
                stdout: vec![],
                stderr: vec![],
            });
        }

        if args.len() >= 3 && args[0] == "-s" && args[2] == "logcat" {
            let is_snapshot = args.contains(&"-d");
            if is_snapshot && runner.fail_logcat_snapshot {
                return Ok(Output {
                    status: exit_fail(),
                    stdout: vec![],
                    stderr: b"logcat failed".to_vec(),
                });
            }
            return Ok(Output {
                status: exit_ok(),
                stdout: vec![],
                stderr: vec![],
            });
        }

        Ok(Output {
            status: exit_ok(),
            stdout: vec![],
            stderr: vec![],
        })
    }
}

#[cfg(test)]
mod system_and_mock_coverage {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::mock::MockAdbCommandRunner;
    use super::{CommandRunner, SystemCommandRunner};
    use std::time::Duration;

    #[test]
    fn system_command_runner_run_executes_true_successfully() {
        let runner = SystemCommandRunner;
        let out = runner.run("true", &[]).expect("true should run");
        assert!(out.status.success());
    }

    #[test]
    fn system_command_runner_run_bounded_returns_status_for_fast_exiting_child() {
        let runner = SystemCommandRunner;
        let out = runner
            .run_bounded("true", &[], Duration::from_secs(2))
            .expect("bounded run");
        assert!(out.status.success());
    }

    #[test]
    fn mock_adb_runner_rejects_non_adb_program() {
        let m = MockAdbCommandRunner::new();
        let e = m.run("rustc", &[]).expect_err("mock is adb-only");
        assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
        let e2 = m
            .run_bounded("cargo", &[], Duration::from_millis(1))
            .expect_err("bounded mock is adb-only");
        assert_eq!(e2.kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn mock_devices_list_nonzero_exit_surfaces_stderr() {
        let mut m = MockAdbCommandRunner::new();
        m.fail_adb_devices_exit = true;
        let out = m.run("adb", &["devices", "-l"]).expect("output");
        assert!(!out.status.success());
        assert!(!out.stderr.is_empty());
    }

    #[test]
    fn mock_empty_devices_list() {
        let m = MockAdbCommandRunner::empty_devices();
        let out = m.run("adb", &["devices", "-l"]).expect("output");
        assert!(out.status.success());
        let s = String::from_utf8_lossy(&out.stdout);
        assert!(s.contains("List of devices"));
    }

    #[test]
    fn mock_install_failure_nonzero() {
        let mut m = MockAdbCommandRunner::new();
        m.fail_adb_install = true;
        let out = m
            .run("adb", &["-s", "emulator-5554", "install", "x.apk"])
            .expect("output");
        assert!(!out.status.success());
    }

    #[test]
    fn mock_install_ambiguous_stdout() {
        let mut m = MockAdbCommandRunner::new();
        m.install_stdout_without_success_marker = true;
        let out = m
            .run("adb", &["-s", "emulator-5554", "install", "x.apk"])
            .expect("output");
        assert!(out.status.success());
        assert!(!String::from_utf8_lossy(&out.stdout).contains("Success"));
    }

    #[test]
    fn mock_logcat_snapshot_failure() {
        let mut m = MockAdbCommandRunner::new();
        m.fail_logcat_snapshot = true;
        let out = m
            .run("adb", &["-s", "emulator-5554", "logcat", "-d"])
            .expect("output");
        assert!(!out.status.success());
    }

    #[test]
    fn mock_getprop_failure() {
        let mut m = MockAdbCommandRunner::new();
        m.fail_getprop = true;
        let out = m
            .run(
                "adb",
                &[
                    "-s",
                    "emulator-5554",
                    "shell",
                    "getprop",
                    "ro.build.version.sdk",
                ],
            )
            .expect("output");
        assert!(!out.status.success());
    }

    #[test]
    fn mock_shell_am_failure() {
        let mut m = MockAdbCommandRunner::new();
        m.fail_shell_am = true;
        let out = m
            .run(
                "adb",
                &["-s", "emulator-5554", "shell", "am", "start", "-n", "a/b"],
            )
            .expect("output");
        assert!(!out.status.success());
    }

    #[test]
    fn mock_pm_list_features_io_error() {
        let mut m = MockAdbCommandRunner::new();
        m.pm_list_features_io_error = true;
        let e = m
            .run(
                "adb",
                &["-s", "emulator-5554", "shell", "pm", "list", "features"],
            )
            .expect_err("io error");
        assert_eq!(e.kind(), std::io::ErrorKind::Other);
    }

    #[test]
    fn mock_logcat_follow_timeout() {
        let mut m = MockAdbCommandRunner::new();
        m.logcat_follow_yields_timeout = true;
        let e = m
            .run_bounded(
                "adb",
                &["-s", "emulator-5554", "logcat"],
                Duration::from_millis(1),
            )
            .expect_err("timeout");
        assert_eq!(e.kind(), std::io::ErrorKind::TimedOut);
    }

    #[test]
    fn mock_logcat_follow_exit_fail() {
        let mut m = MockAdbCommandRunner::new();
        m.logcat_follow_bounded_exit_fail = true;
        let out = m
            .run_bounded(
                "adb",
                &["-s", "emulator-5554", "logcat"],
                Duration::from_secs(2),
            )
            .expect("output");
        assert!(!out.status.success());
    }

    #[test]
    fn mock_fail_adb_spawn() {
        let mut m = MockAdbCommandRunner::new();
        m.fail_adb_spawn = true;
        let e = m.run("adb", &["devices", "-l"]).expect_err("spawn");
        assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
    }
}
