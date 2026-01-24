//! E2E tests for UniBin commands

use assert_cmd::Command;
use predicates::prelude::*;
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn test_beardog_help_shows_unibin_commands() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("server"))
        .stdout(predicate::str::contains("daemon"))
        .stdout(predicate::str::contains("client"))
        .stdout(predicate::str::contains("doctor"));
}

#[test]
fn test_beardog_version() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("beardog"))
        .stdout(predicate::str::contains("0.9.0"));
}

#[test]
fn test_server_help() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("server").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("socket"))
        .stdout(predicate::str::contains("family-id"))
        .stdout(predicate::str::contains("orchestrator-id"));
}

#[test]
fn test_daemon_help() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("daemon").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("socket"))
        .stdout(predicate::str::contains("pid-file"))
        .stdout(predicate::str::contains("log-file"));
}

#[test]
fn test_client_help() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("client").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("socket"))
        .stdout(predicate::str::contains("command"));
}

#[test]
fn test_doctor_help() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("doctor").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("comprehensive"))
        .stdout(predicate::str::contains("format"))
        .stdout(predicate::str::contains("component"));
}

#[test]
fn test_doctor_basic_check() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("doctor");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Health Report"))
        .stdout(predicate::str::contains("Version"))
        .stdout(predicate::str::contains("Entropy Sources"))
        .stdout(predicate::str::contains("Key Storage"));
}

#[test]
fn test_doctor_json_format() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("doctor").arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"status\""))
        .stdout(predicate::str::contains("\"checks\""));
}

#[test]
fn test_doctor_comprehensive() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("doctor").arg("--comprehensive");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Health Report"))
        .stdout(predicate::str::contains("HSM"))
        .stdout(predicate::str::contains("Crypto"));
}

#[test]
fn test_doctor_component_entropy() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("doctor").arg("--component").arg("entropy");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Entropy"));
}

#[test]
fn test_doctor_component_storage() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("doctor").arg("--component").arg("storage");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Key Storage"));
}

#[test]
fn test_doctor_component_crypto() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("doctor").arg("--component").arg("crypto");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Crypto"));
}

#[test]
fn test_client_requires_server() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("test.sock");

    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("client")
        .arg("--socket")
        .arg(socket_path.to_str().unwrap())
        .arg("--command")
        .arg("help");

    // Should fail because server is not running
    cmd.timeout(Duration::from_secs(2)).assert().failure();
}

#[test]
fn test_server_custom_socket() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("custom.sock");

    // Just verify the command accepts the argument
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("server")
        .arg("--help")
        .arg("--socket")
        .arg(socket_path.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn test_daemon_custom_paths() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("daemon.sock");
    let pid_path = temp_dir.path().join("daemon.pid");
    let log_path = temp_dir.path().join("daemon.log");

    // Just verify the command accepts the arguments
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("daemon")
        .arg("--help")
        .arg("--socket")
        .arg(socket_path.to_str().unwrap())
        .arg("--pid-file")
        .arg(pid_path.to_str().unwrap())
        .arg("--log-file")
        .arg(log_path.to_str().unwrap());

    cmd.assert().success();
}
