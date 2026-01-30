//! Integration Tests for `BearDog` CLI
//!
//! End-to-end tests for CLI commands and workflows

#![allow(deprecated)] // cargo_bin deprecation - will migrate to cargo_bin_cmd! macro
#![allow(clippy::unwrap_used, clippy::expect_used)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "BearDog - Sovereign Genetic Cryptography",
    ));
}

#[test]
fn test_entropy_help() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("entropy").arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Entropy collection and seed generation",
    ));
}

#[test]
fn test_key_help() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("key").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Key management operations"));
}

#[test]
fn test_hsm_discover() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("hsm").arg("discover");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("HSM Discovery"));
}

#[tokio::test]
#[ignore = "Requires interactive TTY for human entropy collection"]
async fn test_entropy_collection_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let seed_path = temp_dir.path().join("test-seed.json");

    // Set environment variable to lower threshold for testing
    std::env::set_var("BEARDOG_ENTROPY_QUALITY_THRESHOLD", "0.5");

    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("entropy")
        .arg("collect")
        .arg("--human-input")
        .arg("--device")
        .arg("software")
        .arg("--quality-tier")
        .arg("1")
        .arg("--output")
        .arg(&seed_path);

    cmd.assert().success().stdout(
        predicate::str::contains("BearDog Human Entropy Collection")
            .and(predicate::str::contains("Generated Entropy Seed")),
    );

    // Verify seed file exists
    assert!(seed_path.exists());

    // Verify seed file is valid JSON
    let seed_content = fs::read_to_string(&seed_path).unwrap();
    let _seed: serde_json::Value = serde_json::from_str(&seed_content).unwrap();

    // Clean up env var
    std::env::remove_var("BEARDOG_ENTROPY_QUALITY_THRESHOLD");
}

#[tokio::test]
async fn test_full_encryption_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    let encrypted_file = temp_dir.path().join("test.enc");
    let decrypted_file = temp_dir.path().join("test-decrypted.txt");

    // Create test file
    fs::write(&test_file, "Hello, BearDog!").unwrap();

    // Generate key
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("key")
        .arg("generate")
        .arg("--key-id")
        .arg("test-cli-key")
        .arg("--algorithm")
        .arg("aes256-gcm")
        .arg("--hsm")
        .arg("auto");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Key generated successfully"));

    // Encrypt file
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("encrypt")
        .arg("--key")
        .arg("test-cli-key")
        .arg("--input")
        .arg(&test_file)
        .arg("--output")
        .arg(&encrypted_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Encryption complete"));

    assert!(encrypted_file.exists());

    // Decrypt file
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("decrypt")
        .arg("--key")
        .arg("test-cli-key")
        .arg("--input")
        .arg(&encrypted_file)
        .arg("--output")
        .arg(&decrypted_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Decryption complete"));

    assert!(decrypted_file.exists());

    // Verify content matches
    let original = fs::read_to_string(&test_file).unwrap();
    let decrypted = fs::read_to_string(&decrypted_file).unwrap();
    assert_eq!(original, decrypted);

    // Clean up key
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("key")
        .arg("delete")
        .arg("--key-id")
        .arg("test-cli-key")
        .arg("--yes");

    cmd.assert().success();
}

#[test]
fn test_key_list() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("key").arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Available Keys"));
}

#[tokio::test]
#[ignore = "Requires interactive TTY for human entropy collection"]
async fn test_entropy_info() {
    let temp_dir = TempDir::new().unwrap();
    let seed_path = temp_dir.path().join("info-test-seed.json");

    // Set environment variable to lower threshold for testing
    std::env::set_var("BEARDOG_ENTROPY_QUALITY_THRESHOLD", "0.5");

    // First collect entropy
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("entropy")
        .arg("collect")
        .arg("--human-input")
        .arg("--device")
        .arg("software")
        .arg("--quality-tier")
        .arg("1")
        .arg("--output")
        .arg(&seed_path);

    cmd.assert().success();

    // Then check info
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("entropy").arg("info").arg("--seed").arg(&seed_path);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Entropy Seed Information"));

    // Clean up env var
    std::env::remove_var("BEARDOG_ENTROPY_QUALITY_THRESHOLD");
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("invalid-command");
    cmd.assert().failure();
}

#[test]
fn test_missing_required_args() {
    // Test encrypt without key
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("encrypt").arg("--input").arg("test.txt");
    cmd.assert().failure();
}

#[tokio::test]
async fn test_large_file_encryption() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("large.txt");
    let encrypted_file = temp_dir.path().join("large.enc");
    let decrypted_file = temp_dir.path().join("large-decrypted.txt");

    // Create 1MB test file
    let large_data = vec![b'X'; 1024 * 1024]; // 1MB
    fs::write(&test_file, large_data).unwrap();

    // Generate key
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("key")
        .arg("generate")
        .arg("--key-id")
        .arg("large-test-key")
        .arg("--algorithm")
        .arg("aes256-gcm")
        .arg("--hsm")
        .arg("auto");

    cmd.assert().success();

    // Encrypt
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("encrypt")
        .arg("--key")
        .arg("large-test-key")
        .arg("--input")
        .arg(&test_file)
        .arg("--output")
        .arg(&encrypted_file);

    cmd.assert().success();

    // Decrypt
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("decrypt")
        .arg("--key")
        .arg("large-test-key")
        .arg("--input")
        .arg(&encrypted_file)
        .arg("--output")
        .arg(&decrypted_file);

    cmd.assert().success();

    // Verify sizes
    let original_size = fs::metadata(&test_file).unwrap().len();
    let decrypted_size = fs::metadata(&decrypted_file).unwrap().len();
    assert_eq!(original_size, decrypted_size);

    // Clean up
    let mut cmd = Command::cargo_bin("beardog").unwrap();
    cmd.arg("key")
        .arg("delete")
        .arg("--key-id")
        .arg("large-test-key")
        .arg("--yes");

    cmd.assert().success();
}
