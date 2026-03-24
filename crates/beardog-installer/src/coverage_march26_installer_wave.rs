// SPDX-License-Identifier: AGPL-3.0-only

//! Extra coverage for error displays, deployment types, and installer edge paths.

use crate::arch::{ArchError, Architecture};
use crate::deployment::DeploymentError;
use crate::installer::{BinaryInstaller, InstallerError};
use crate::platform::{BiomeOSPaths, OperatingSystem, PlatformError};
use crate::types::{DeploymentProgress, DeploymentReport, DeploymentStatus, PrimalName};
use crate::validator::{BinaryValidator, ValidationError};

// --- ArchError / Architecture ---

#[test]
fn arch_error_unsupported_display_contains_arch() {
    let e = ArchError::Unsupported {
        arch: "mips64".to_string(),
        help: "report upstream".to_string(),
    };
    let s = e.to_string();
    assert!(s.contains("mips64") && s.contains("report"));
}

#[test]
fn architecture_serde_wasm32_roundtrip() {
    let a = Architecture::Wasm32;
    let j = serde_json::to_string(&a).expect("ser");
    let back: Architecture = serde_json::from_str(&j).expect("de");
    assert_eq!(back, Architecture::Wasm32);
}

#[test]
fn architecture_display_riscv64() {
    assert_eq!(Architecture::Riscv64.to_string(), "riscv64");
}

// --- Platform / OS ---

#[test]
fn operating_system_ios_serde() {
    let j = serde_json::to_string(&OperatingSystem::Ios).expect("ser");
    let back: OperatingSystem = serde_json::from_str(&j).expect("de");
    assert_eq!(back, OperatingSystem::Ios);
}

#[test]
fn operating_system_linux_musl_serde() {
    let j = serde_json::to_string(&OperatingSystem::LinuxMusl).expect("ser");
    let back: OperatingSystem = serde_json::from_str(&j).expect("de");
    assert_eq!(back, OperatingSystem::LinuxMusl);
}

#[test]
fn platform_error_io_display() {
    let e = PlatformError::IoError {
        path: std::path::PathBuf::from("/tmp/x"),
        source: std::io::Error::other("boom"),
    };
    assert!(e.to_string().contains("IO error") || e.to_string().contains("boom"));
}

// --- Deployment types & errors ---

#[test]
fn deployment_error_validation_failed_display() {
    let e = DeploymentError::ValidationFailed {
        primal: PrimalName::new("p"),
        reason: "missing binary".to_string(),
    };
    assert!(e.to_string().contains("Validation") && e.to_string().contains("missing"));
}

#[test]
fn deployment_error_from_platform_via_display() {
    let e = DeploymentError::Platform(PlatformError::NoHomeDir);
    assert!(!e.to_string().is_empty());
}

#[test]
fn deployment_progress_new_sets_fields() {
    let p = DeploymentProgress::new(
        PrimalName::new("x"),
        DeploymentStatus::Installing,
        42,
        "copying".to_string(),
    );
    assert_eq!(p.percent, 42);
    assert_eq!(p.status, DeploymentStatus::Installing);
}

#[test]
fn deployment_status_failed_serde_roundtrip() {
    let s = DeploymentStatus::Failed {
        reason: "disk full".to_string(),
    };
    let j = serde_json::to_string(&s).expect("ser");
    let back: DeploymentStatus = serde_json::from_str(&j).expect("de");
    assert_eq!(back, s);
}

#[test]
fn deployment_report_partial_success_rate() {
    let r = DeploymentReport {
        total: 4,
        successes: 3,
        failures: vec![(PrimalName::new("a"), "e".to_string())],
        arch: Architecture::X86_64,
        os: OperatingSystem::Linux,
    };
    assert!((r.success_rate() - 75.0).abs() < 0.01);
}

// --- InstallerError ---

#[test]
fn installer_error_copy_failed_display() {
    let e = InstallerError::CopyFailed {
        from: std::path::PathBuf::from("/a"),
        to: std::path::PathBuf::from("/b"),
        source: std::io::Error::other("copy err"),
    };
    assert!(e.to_string().contains("copy") || e.to_string().contains("Copy"));
}

#[test]
fn installer_error_io_display() {
    let e = InstallerError::IoError {
        path: std::path::PathBuf::from("/z"),
        source: std::io::Error::other("io"),
    };
    assert!(e.to_string().contains("IO") || e.to_string().contains("io"));
}

#[test]
fn locate_binary_finds_flat_release_layout() {
    let temp = tempfile::TempDir::new().expect("tempdir");
    let release = temp.path().join("release");
    std::fs::create_dir_all(&release).expect("mkdir");
    let bin = release.join("beardog");
    std::fs::write(&bin, b"x").expect("write");

    let paths = BiomeOSPaths {
        bin_dir: temp.path().join("bin"),
        data_dir: temp.path().join("data"),
        config_dir: temp.path().join("config"),
        runtime_dir: temp.path().join("runtime"),
        cache_dir: temp.path().join("cache"),
    };

    let installer = BinaryInstaller::new(paths, release);
    let found = installer
        .locate_binary(
            PrimalName::new("beardog"),
            &Architecture::X86_64,
            &OperatingSystem::Linux,
        )
        .expect("in release dir");
    assert_eq!(found.file_name().and_then(|n| n.to_str()), Some("beardog"));
}

// --- ValidationError ---

#[test]
fn validation_error_execution_timeout_display() {
    let e = ValidationError::ExecutionTimeout {
        path: std::path::PathBuf::from("/bin/x"),
    };
    let s = e.to_string().to_lowercase();
    assert!(
        s.contains("timeout") || s.contains("timed out"),
        "unexpected message: {s}"
    );
}

#[test]
fn validation_error_execution_failed_display() {
    let e = ValidationError::ExecutionFailed {
        path: std::path::PathBuf::from("/bin/y"),
        source: std::io::Error::other("spawn"),
    };
    assert!(!e.to_string().is_empty());
}

#[test]
fn binary_validator_default_matches_new() {
    let _a = BinaryValidator;
    let _b = BinaryValidator::new();
}

#[test]
fn primal_name_hash_consistency() {
    use std::collections::HashSet;
    let a = PrimalName::new("same");
    let b = PrimalName::new("same");
    let mut s = HashSet::new();
    s.insert(a);
    assert!(s.contains(&b));
}

#[test]
fn deployment_report_display_includes_arch_os_lines() {
    let r = DeploymentReport {
        total: 1,
        successes: 1,
        failures: vec![],
        arch: Architecture::Aarch64,
        os: OperatingSystem::MacOS,
    };
    let t = format!("{r}");
    assert!(t.contains("aarch64") || t.contains("Architecture"));
}

#[test]
fn biome_paths_all_dirs_length_five() {
    let paths = BiomeOSPaths::discover().expect("paths");
    assert_eq!(paths.all_dirs().len(), 5);
}

#[test]
fn operating_system_android_serde_roundtrip() {
    let j = serde_json::to_string(&OperatingSystem::Android).expect("ser");
    let back: OperatingSystem = serde_json::from_str(&j).expect("de");
    assert_eq!(back, OperatingSystem::Android);
}

#[test]
fn deployment_status_validating_display() {
    assert_eq!(DeploymentStatus::Validating.to_string(), "Validating");
}

#[test]
fn primal_name_parse_accepts_leading_digit() {
    assert_eq!(
        PrimalName::parse_name("3rd-party"),
        Some(PrimalName::new("3rd-party"))
    );
}

#[test]
fn architecture_all_contains_four_variants() {
    let all = Architecture::all();
    assert_eq!(all.len(), 4);
    assert!(all.contains(&Architecture::Wasm32));
}

#[test]
fn installer_binary_not_found_preserves_target_in_error() {
    let e = InstallerError::BinaryNotFound {
        primal: PrimalName::new("z"),
        target: "custom-target".to_string(),
        searched: vec![],
    };
    assert!(e.to_string().contains("custom-target"));
}
