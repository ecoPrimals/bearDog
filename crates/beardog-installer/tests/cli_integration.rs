// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used)]

//! Integration tests for the `beardog-installer` binary (CLI parsing and subcommands).

use std::path::PathBuf;
use std::process::Command;

fn beardog_installer_exe() -> PathBuf {
    if let Some(p) = std::env::var_os("CARGO_BIN_EXE_beardog_installer") {
        return PathBuf::from(p);
    }
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    manifest_dir
        .join("../../target")
        .join(profile)
        .join("beardog-installer")
}

#[test]
fn cli_help_prints_usage() {
    let out = Command::new(beardog_installer_exe())
        .arg("--help")
        .output()
        .expect("run installer --help");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("beardog-installer"));
    assert!(s.contains("install"));
}

#[test]
fn cli_version_prints_package_name() {
    let out = Command::new(beardog_installer_exe())
        .arg("--version")
        .output()
        .expect("run installer --version");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("beardog-installer"));
}

#[test]
fn cli_version_subcommand_prints_version() {
    let out = Command::new(beardog_installer_exe())
        .arg("version")
        .output()
        .expect("run installer version");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("beardog-installer"));
}

#[test]
fn cli_paths_subcommand_succeeds() {
    let out = Command::new(beardog_installer_exe())
        .arg("paths")
        .output()
        .expect("run installer paths");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("Binaries:"));
}

#[test]
fn cli_install_dry_run_does_not_require_source_binaries() {
    let out = Command::new(beardog_installer_exe())
        .arg("install")
        .arg("--dry-run")
        .output()
        .expect("run installer install --dry-run");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("Dry run"));
}

#[test]
fn cli_install_rejects_invalid_primal_slug() {
    let out = Command::new(beardog_installer_exe())
        .arg("install")
        .arg("--primals")
        .arg("not_valid!")
        .output()
        .expect("run installer with bad primal");
    assert!(!out.status.success());
    let s = String::from_utf8_lossy(&out.stderr);
    assert!(s.contains("Unknown primal"), "stderr: {s}");
}
