// SPDX-License-Identifier: AGPL-3.0-only

//! Pluggable command execution for [`crate::device::DeviceManager`].
//!
//! Tests inject `MockAdbCommandRunner` so the suite never blocks on `adb`.

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

/// Test-only mock implementations for ADB command execution.
///
/// Test-only mock implementations for ADB command execution.
#[cfg(test)]
pub(crate) mod mock {
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

    /// Mock ADB responses for tests (no subprocess, no `adb` on PATH required).
    #[derive(Debug, Clone, Default)]
    pub struct MockAdbCommandRunner {
        /// When true, `adb devices -l` returns no devices (exercises env fallback).
        pub empty_devices: bool,
    }

    impl MockAdbCommandRunner {
        /// Same as [`Default::default`].
        #[must_use]
        pub const fn new() -> Self {
            Self {
                empty_devices: false,
            }
        }

        /// Mock with no devices listed (forces env-based fallback).
        #[must_use]
        pub const fn empty_devices() -> Self {
            Self {
                empty_devices: true,
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
            mock_adb_output(self.empty_devices, args)
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
            let _ = args;
            Ok(Output {
                status: exit_ok(),
                stdout: vec![],
                stderr: vec![],
            })
        }
    }

    fn mock_adb_output(empty_devices: bool, args: &[&str]) -> Result<Output, io::Error> {
        if args.len() >= 2 && args[0] == "devices" && args[1] == "-l" {
            let stdout = if empty_devices {
                b"List of devices attached\n\n".to_vec()
            } else {
                b"List of devices attached\nemulator-5554\tdevice\n".to_vec()
            };
            return Ok(Output {
                status: exit_ok(),
                stdout,
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
            return Ok(Output {
                status: exit_ok(),
                stdout: b"feature:android.hardware.strongbox_keystore\nfeature:android.hardware.fingerprint\n"
                    .to_vec(),
                stderr: vec![],
            });
        }

        if args.len() >= 4 && args[0] == "-s" && args[2] == "install" {
            return Ok(Output {
                status: exit_ok(),
                stdout: b"Success\n".to_vec(),
                stderr: vec![],
            });
        }

        if args.len() >= 4 && args[0] == "-s" && args[2] == "shell" && args[3] == "am" {
            return Ok(Output {
                status: exit_ok(),
                stdout: vec![],
                stderr: vec![],
            });
        }

        if args.len() >= 3 && args[0] == "-s" && args[2] == "logcat" {
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
}
