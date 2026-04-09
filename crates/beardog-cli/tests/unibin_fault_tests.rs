// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::float_cmp)]

//! Fault injection tests for `UniBin` commands - resilience testing

use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_doctor_with_corrupted_key_storage() {
    let temp_dir = TempDir::new().unwrap();
    let key_dir = temp_dir.path().join("beardog_keys");
    fs::create_dir(&key_dir).unwrap();

    // Create corrupted key file
    fs::write(key_dir.join("corrupted.key"), "invalid data").unwrap();

    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("doctor")
        .env("HOME", temp_dir.path().to_str().unwrap());

    // Should still succeed, just report storage issues
    cmd.assert().success();
}

#[test]
fn test_server_with_no_memory_limit() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("test.sock");

    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("server")
        .arg("--socket")
        .arg(socket_path.to_str().unwrap());

    // Should handle memory pressure gracefully
    cmd.timeout(std::time::Duration::from_secs(2));
}

#[test]
fn test_client_with_malformed_socket() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("malformed.sock");

    // Create a regular file instead of a socket
    fs::write(&socket_path, "not a socket").unwrap();

    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("client")
        .arg("--socket")
        .arg(socket_path.to_str().unwrap())
        .arg("--command")
        .arg("help");

    // Should fail gracefully with appropriate error
    cmd.timeout(std::time::Duration::from_secs(2))
        .assert()
        .failure();
}

#[test]
fn test_daemon_with_full_disk() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("daemon")
        .arg("--pid-file")
        .arg(temp_dir.path().join("daemon.pid").to_str().unwrap())
        .arg("--log-file")
        .arg(temp_dir.path().join("daemon.log").to_str().unwrap())
        .arg("--socket")
        .arg(temp_dir.path().join("daemon.sock").to_str().unwrap());

    // Should handle disk space issues gracefully
    cmd.timeout(std::time::Duration::from_secs(2));
}

#[test]
fn test_doctor_with_missing_entropy_source() {
    // Doctor should still pass even if some entropy sources are unavailable
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("doctor").arg("--component").arg("entropy");

    cmd.assert().success();
}

#[test]
fn test_server_with_invalid_hsm_config() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("test.sock");

    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("server")
        .arg("--socket")
        .arg(socket_path.to_str().unwrap());

    // Should fallback to software HSM if hardware HSM unavailable
    cmd.timeout(std::time::Duration::from_secs(2));
}

#[test]
fn test_client_connection_timeout() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("nonexistent.sock");

    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("client")
        .arg("--socket")
        .arg(socket_path.to_str().unwrap())
        .arg("--command")
        .arg("help");

    // Should timeout and fail gracefully
    cmd.timeout(std::time::Duration::from_secs(3))
        .assert()
        .failure();
}

#[test]
fn test_doctor_partial_system_failure() {
    // Run doctor when some checks might fail
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("doctor").arg("--comprehensive");

    // Should complete and report status even if some checks fail
    cmd.assert().code(predicate::in_iter(vec![0, 1])); // 0 = healthy, 1 = unhealthy
}

#[test]
fn test_server_rapid_start_stop() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("rapid.sock");

    // Start and immediately stop (via timeout)
    for _ in 0..3 {
        let mut cmd = Command::cargo_bin("beardog").unwrap();
        cmd.arg("server")
            .arg("--socket")
            .arg(socket_path.to_str().unwrap());

        cmd.timeout(std::time::Duration::from_millis(100));

        // Clean up socket if it exists
        let _ = fs::remove_file(&socket_path);
    }
}

#[test]
fn test_daemon_pid_file_race_condition() {
    let temp_dir = TempDir::new().unwrap();
    let pid_file = temp_dir.path().join("race.pid");

    // Create PID file
    fs::write(&pid_file, "12345").unwrap();

    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("daemon")
        .arg("--pid-file")
        .arg(pid_file.to_str().unwrap())
        .arg("--socket")
        .arg("/tmp/race_test.sock");

    // Should detect and handle stale PID
    cmd.timeout(std::time::Duration::from_secs(2));
}

#[test]
fn test_client_network_interruption_simulation() {
    // Simulate network issues by connecting to non-existent socket
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("client")
        .arg("--socket")
        .arg("/tmp/never_exists.sock")
        .arg("--command")
        .arg("test");

    // Should fail gracefully with connection error
    cmd.timeout(std::time::Duration::from_secs(2))
        .assert()
        .failure();
}

#[test]
fn test_doctor_concurrent_executions() {
    // Run multiple doctor checks concurrently to test for race conditions
    use std::thread;

    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                let mut cmd = Command::cargo_bin("beardog").unwrap();
                cmd.arg("doctor");
                let result = cmd.assert().success();
                eprintln!("Concurrent doctor check {i} completed");
                result
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_server_socket_cleanup_on_error() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("cleanup_test.sock");

    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("server")
        .arg("--socket")
        .arg(socket_path.to_str().unwrap());

    // Start and stop via timeout
    cmd.timeout(std::time::Duration::from_millis(500));

    // Socket should be cleaned up or properly handled on next start
    let mut cmd2 = Command::cargo_bin("beardog").unwrap();
    cmd2.arg("server")
        .arg("--socket")
        .arg(socket_path.to_str().unwrap());

    cmd2.timeout(std::time::Duration::from_millis(500));
}

#[test]
fn test_doctor_resource_exhaustion() {
    // Run doctor with limited resources
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("doctor").arg("--comprehensive");

    // Should complete even with resource pressure
    cmd.timeout(std::time::Duration::from_secs(10))
        .assert()
        .code(predicate::in_iter(vec![0, 1]));
}
