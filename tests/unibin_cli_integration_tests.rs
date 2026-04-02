// SPDX-License-Identifier: AGPL-3.0-only

#![allow(clippy::expect_used, clippy::unwrap_used)]
//! Integration tests for the `BearDog` `UniBin` CLI.
//!
//! Exercises the binary entry point (`src/main.rs`) via subprocess invocation,
//! ensuring clap parsing, version output, help text, and error paths are covered.

use std::process::Command;

fn beardog_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_beardog"))
}

#[test]
fn cli_version_flag_succeeds() {
    let output = beardog_bin()
        .arg("--version")
        .output()
        .expect("failed to run beardog --version");
    assert!(output.status.success(), "beardog --version should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("beardog"),
        "version output should contain crate name"
    );
}

#[test]
fn cli_help_flag_succeeds() {
    let output = beardog_bin()
        .arg("--help")
        .output()
        .expect("failed to run beardog --help");
    assert!(output.status.success(), "beardog --help should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Sovereign Genetic Cryptography"),
        "help should contain description"
    );
    assert!(
        stdout.contains("server"),
        "help should list server subcommand"
    );
    assert!(stdout.contains("key"), "help should list key subcommand");
}

#[test]
fn cli_version_subcommand_succeeds() {
    let output = beardog_bin()
        .arg("version")
        .output()
        .expect("failed to run beardog version");
    assert!(
        output.status.success(),
        "beardog version subcommand should exit 0"
    );
}

#[test]
fn cli_capabilities_subcommand_succeeds() {
    let output = beardog_bin()
        .arg("capabilities")
        .output()
        .expect("failed to run beardog capabilities");
    assert!(
        output.status.success(),
        "beardog capabilities subcommand should exit 0"
    );
}

#[test]
fn cli_no_args_shows_help_and_fails() {
    let output = beardog_bin()
        .output()
        .expect("failed to run beardog with no args");
    assert!(
        !output.status.success(),
        "beardog with no args should exit non-zero (missing subcommand)"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Usage") || stderr.contains("usage"),
        "should show usage hint"
    );
}

#[test]
fn cli_invalid_subcommand_fails() {
    let output = beardog_bin()
        .arg("nonexistent-subcommand")
        .output()
        .expect("failed to run beardog with invalid subcommand");
    assert!(
        !output.status.success(),
        "invalid subcommand should exit non-zero"
    );
}

#[test]
fn cli_key_help_succeeds() {
    let output = beardog_bin()
        .args(["key", "--help"])
        .output()
        .expect("failed to run beardog key --help");
    assert!(output.status.success(), "beardog key --help should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("generate") || stdout.contains("Generate"),
        "key help should list generate subcommand"
    );
}

#[test]
fn cli_entropy_help_succeeds() {
    let output = beardog_bin()
        .args(["entropy", "--help"])
        .output()
        .expect("failed to run beardog entropy --help");
    assert!(
        output.status.success(),
        "beardog entropy --help should exit 0"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("collect") || stdout.contains("Collect"),
        "entropy help should list collect subcommand"
    );
}

#[test]
fn cli_hsm_help_succeeds() {
    let output = beardog_bin()
        .args(["hsm", "--help"])
        .output()
        .expect("failed to run beardog hsm --help");
    assert!(output.status.success(), "beardog hsm --help should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("discover") || stdout.contains("Discover"),
        "hsm help should list discover subcommand"
    );
}

#[test]
fn cli_doctor_runs_without_crash() {
    let output = beardog_bin()
        .arg("doctor")
        .output()
        .expect("failed to run beardog doctor");
    assert!(
        output.status.success(),
        "beardog doctor should succeed (health diagnostics)"
    );
}
