// SPDX-License-Identifier: AGPL-3.0-only

//! Targeted coverage tests for installer, validator, platform, arch, and types uncovered paths.

use crate::arch::Architecture;
use crate::platform::OperatingSystem;
use crate::types::{DeploymentReport, DeploymentStatus, PrimalName};
use crate::validator::{BinaryValidator, ValidationReport};

// -- Architecture: exhaustive to_rust_target combos --

#[test]
fn to_rust_target_all_linux_gnu() {
    assert_eq!(
        Architecture::X86_64.to_rust_target(&OperatingSystem::Linux),
        "x86_64-unknown-linux-gnu"
    );
    assert_eq!(
        Architecture::Aarch64.to_rust_target(&OperatingSystem::Linux),
        "aarch64-unknown-linux-gnu"
    );
    assert_eq!(
        Architecture::Riscv64.to_rust_target(&OperatingSystem::Linux),
        "riscv64gc-unknown-linux-gnu"
    );
}

#[test]
fn to_rust_target_linux_musl() {
    assert_eq!(
        Architecture::X86_64.to_rust_target(&OperatingSystem::LinuxMusl),
        "x86_64-unknown-linux-musl"
    );
    assert_eq!(
        Architecture::Aarch64.to_rust_target(&OperatingSystem::LinuxMusl),
        "aarch64-unknown-linux-musl"
    );
    assert_eq!(
        Architecture::Riscv64.to_rust_target(&OperatingSystem::LinuxMusl),
        "riscv64gc-unknown-linux-musl"
    );
}

#[test]
fn to_rust_target_android() {
    assert_eq!(
        Architecture::Aarch64.to_rust_target(&OperatingSystem::Android),
        "aarch64-linux-android"
    );
    assert_eq!(
        Architecture::X86_64.to_rust_target(&OperatingSystem::Android),
        "x86_64-linux-android"
    );
}

#[test]
fn to_rust_target_macos() {
    assert_eq!(
        Architecture::X86_64.to_rust_target(&OperatingSystem::MacOS),
        "x86_64-apple-darwin"
    );
    assert_eq!(
        Architecture::Aarch64.to_rust_target(&OperatingSystem::MacOS),
        "aarch64-apple-darwin"
    );
}

#[test]
fn to_rust_target_ios() {
    assert_eq!(
        Architecture::Aarch64.to_rust_target(&OperatingSystem::Ios),
        "aarch64-apple-ios"
    );
}

#[test]
fn to_rust_target_windows() {
    assert_eq!(
        Architecture::X86_64.to_rust_target(&OperatingSystem::Windows),
        "x86_64-pc-windows-gnu"
    );
    assert_eq!(
        Architecture::Aarch64.to_rust_target(&OperatingSystem::Windows),
        "aarch64-pc-windows-gnullvm"
    );
}

#[test]
fn to_rust_target_wasm32_any_os() {
    for os in [
        OperatingSystem::Linux,
        OperatingSystem::MacOS,
        OperatingSystem::Windows,
    ] {
        assert_eq!(
            Architecture::Wasm32.to_rust_target(&os),
            "wasm32-unknown-unknown"
        );
    }
}

#[test]
fn to_rust_target_unsupported_combos_return_unknown() {
    assert_eq!(
        Architecture::Riscv64.to_rust_target(&OperatingSystem::Android),
        "unknown"
    );
    assert_eq!(
        Architecture::Riscv64.to_rust_target(&OperatingSystem::MacOS),
        "unknown"
    );
    assert_eq!(
        Architecture::Riscv64.to_rust_target(&OperatingSystem::Windows),
        "unknown"
    );
    assert_eq!(
        Architecture::Riscv64.to_rust_target(&OperatingSystem::Ios),
        "unknown"
    );
    assert_eq!(
        Architecture::X86_64.to_rust_target(&OperatingSystem::Ios),
        "unknown"
    );
}

#[test]
fn binary_extension_windows_vs_unix() {
    assert_eq!(
        Architecture::X86_64.binary_extension(&OperatingSystem::Windows),
        ".exe"
    );
    assert_eq!(
        Architecture::X86_64.binary_extension(&OperatingSystem::Linux),
        ""
    );
    assert_eq!(
        Architecture::Aarch64.binary_extension(&OperatingSystem::MacOS),
        ""
    );
}

// -- PrimalName edge cases --

#[test]
fn parse_name_rejects_empty_and_special_chars() {
    assert_eq!(PrimalName::parse_name(""), None);
    assert_eq!(PrimalName::parse_name("  "), None);
    assert_eq!(PrimalName::parse_name("invalid!"), None);
    assert_eq!(PrimalName::parse_name("has space"), None);
    assert_eq!(PrimalName::parse_name("-starts-with-dash"), None);
    assert_eq!(PrimalName::parse_name("_starts-with-under"), None);
}

#[test]
fn parse_name_accepts_valid_slugs() {
    assert_eq!(
        PrimalName::parse_name("beardog"),
        Some(PrimalName::new("beardog"))
    );
    assert_eq!(
        PrimalName::parse_name("bear-dog"),
        Some(PrimalName::new("bear-dog"))
    );
    assert_eq!(
        PrimalName::parse_name("bear_dog_3"),
        Some(PrimalName::new("bear_dog_3"))
    );
    assert_eq!(
        PrimalName::parse_name("3rd-primal"),
        Some(PrimalName::new("3rd-primal"))
    );
}

#[test]
fn title_case_slug_empty_returns_empty() {
    assert_eq!(PrimalName::new("").display_name(), "");
}

#[test]
fn title_case_slug_multi_part() {
    assert_eq!(PrimalName::new("song-bird").display_name(), "Song Bird");
    assert_eq!(
        PrimalName::new("my_cool_primal").display_name(),
        "My Cool Primal"
    );
}

#[test]
fn genome_bundle_defaults_with_env_override() {
    let _guard = crate::test_env_lock::ECOPRIMALS_GENOME_TARGETS_LOCK
        .lock()
        .expect("ECOPRIMALS_GENOME_TARGETS test lock poisoned");
    beardog_errors::process_env::set_var("ECOPRIMALS_GENOME_TARGETS", "beardog,songbird");
    let defaults = PrimalName::genome_bundle_defaults();
    beardog_errors::process_env::remove_var("ECOPRIMALS_GENOME_TARGETS");

    assert_eq!(defaults.len(), 2);
    assert_eq!(defaults[0].name(), "beardog");
    assert_eq!(defaults[1].name(), "songbird");
}

#[test]
fn genome_bundle_defaults_empty_env_falls_back_to_manifest() {
    let _guard = crate::test_env_lock::ECOPRIMALS_GENOME_TARGETS_LOCK
        .lock()
        .expect("ECOPRIMALS_GENOME_TARGETS test lock poisoned");
    beardog_errors::process_env::set_var("ECOPRIMALS_GENOME_TARGETS", "   ");
    let defaults = PrimalName::genome_bundle_defaults();
    beardog_errors::process_env::remove_var("ECOPRIMALS_GENOME_TARGETS");

    assert!(
        !defaults.is_empty(),
        "empty env should fall back to manifest defaults"
    );
}

#[test]
fn primal_name_display_delegates_to_display_name() {
    let p = PrimalName::new("beardog");
    assert_eq!(format!("{p}"), p.display_name());
}

// -- DeploymentStatus Display --

#[test]
fn deployment_status_display_all_variants() {
    assert_eq!(format!("{}", DeploymentStatus::Pending), "Pending");
    assert_eq!(format!("{}", DeploymentStatus::Downloading), "Downloading");
    assert_eq!(format!("{}", DeploymentStatus::Installing), "Installing");
    assert_eq!(format!("{}", DeploymentStatus::Validating), "Validating");
    assert_eq!(format!("{}", DeploymentStatus::Complete), "Complete");
    assert_eq!(format!("{}", DeploymentStatus::RolledBack), "Rolled Back");
    assert_eq!(
        format!(
            "{}",
            DeploymentStatus::Failed {
                reason: "disk full".to_string()
            }
        ),
        "Failed: disk full"
    );
}

// -- DeploymentReport with failures --

#[test]
fn deployment_report_display_with_failures() {
    let report = DeploymentReport {
        total: 3,
        successes: 1,
        failures: vec![
            (PrimalName::new("songbird"), "binary not found".to_string()),
            (PrimalName::new("nestgate"), "checksum mismatch".to_string()),
        ],
        arch: Architecture::X86_64,
        os: OperatingSystem::Linux,
    };

    let text = format!("{report}");
    assert!(text.contains("Failures:"));
    assert!(text.contains("Songbird: binary not found"));
    assert!(text.contains("Nestgate: checksum mismatch"));
    assert!(text.contains("33.3%"));
    assert!(!report.is_success());
}

#[test]
fn deployment_report_display_without_failures() {
    let report = DeploymentReport {
        total: 2,
        successes: 2,
        failures: vec![],
        arch: Architecture::Aarch64,
        os: OperatingSystem::MacOS,
    };

    let text = format!("{report}");
    assert!(text.contains("Failures:  0"));
    assert!(text.contains("100.0%"));
    assert!(report.is_success());
}

#[test]
fn deployment_report_zero_total() {
    let report = DeploymentReport {
        total: 0,
        successes: 0,
        failures: vec![],
        arch: Architecture::X86_64,
        os: OperatingSystem::Linux,
    };
    let rate = report.success_rate();
    assert!(rate.abs() < 1e-9, "expected 0.0, got {rate}");
}

// -- ValidationReport: Display for all branches --

#[test]
fn validation_report_display_healthy() {
    let mut r = ValidationReport::new(PrimalName::new("beardog"));
    r.file_exists = true;
    r.is_executable = true;
    r.size_bytes = 5_000_000;
    r.size_reasonable = true;
    r.checksum = Some("abcdef1234567890abcdef1234567890".to_string());
    r.runs = true;
    r.healthy = true;

    let text = format!("{r}");
    assert!(text.contains("HEALTHY"));
    assert!(text.contains("abcdef1234567890..."));
}

#[test]
fn validation_report_display_unhealthy_no_checksum() {
    let r = ValidationReport::new(PrimalName::new("missing"));
    let text = format!("{r}");
    assert!(text.contains("UNHEALTHY"));
    assert!(!text.contains("SHA-256"));
}

#[test]
fn validation_report_is_healthy_requires_all_four() {
    let mut r = ValidationReport::new(PrimalName::new("t"));
    r.file_exists = true;
    r.is_executable = true;
    r.size_reasonable = true;
    r.runs = false;
    assert!(!r.is_healthy());

    r.runs = true;
    assert!(r.is_healthy());
}

// -- BinaryValidator: validate_binary for nonexistent --

#[tokio::test]
async fn validator_nonexistent_file_returns_unhealthy() {
    let v = BinaryValidator::new();
    let report = v
        .validate_binary(
            PrimalName::new("nofile"),
            std::path::Path::new("/nonexistent/bin/x"),
        )
        .await
        .expect("should return Ok with unhealthy report");
    assert!(!report.file_exists);
    assert!(!report.is_healthy());
}

// -- BinaryValidator: validate_binary for small file (under 1MB) --

#[tokio::test]
async fn validator_small_file_not_size_reasonable() {
    let tmp = tempfile::NamedTempFile::new().expect("temp file");
    std::fs::write(tmp.path(), "tiny").expect("write");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(tmp.path(), std::fs::Permissions::from_mode(0o755))
            .expect("chmod");
    }
    let v = BinaryValidator::new();
    let report = v
        .validate_binary(PrimalName::new("tiny"), tmp.path())
        .await
        .expect("should return Ok");
    assert!(report.file_exists);
    assert!(!report.size_reasonable);
    assert!(!report.is_healthy());
}

// -- BinaryValidator: Default trait --

#[test]
fn validator_default_is_new() {
    let _ = BinaryValidator;
}

// -- BinaryValidator: validate_all with mix --

#[tokio::test]
async fn validator_validate_all_mixed() {
    let v = BinaryValidator::new();
    let reports = v
        .validate_all(vec![
            (
                PrimalName::new("absent"),
                std::path::PathBuf::from("/nonexistent/absent"),
            ),
            (
                PrimalName::new("also-absent"),
                std::path::PathBuf::from("/nonexistent/also"),
            ),
        ])
        .await;
    assert_eq!(reports.len(), 2);
    assert!(reports.iter().all(|r| !r.is_healthy()));
}

// -- Platform: discover + ensure --

#[test]
fn biome_os_paths_discover_and_all_dirs() {
    let paths = crate::platform::PlatformPaths::discover().expect("discover paths");
    let dirs = paths.all_dirs();
    assert!(dirs.len() >= 4);
    for d in &dirs {
        assert!(!d.as_os_str().is_empty());
    }
}

#[tokio::test]
async fn biome_os_paths_ensure_exists_creates_dirs() {
    let paths = crate::platform::PlatformPaths::discover().expect("discover paths");
    paths
        .ensure_exists()
        .await
        .expect("ensure_exists should succeed or dirs already exist");
}

// -- DeploymentProgress --

#[test]
fn deployment_progress_pending() {
    let p = crate::types::DeploymentProgress::pending(PrimalName::new("beardog"));
    assert_eq!(p.percent, 0);
    assert_eq!(p.status, DeploymentStatus::Pending);
}
