// SPDX-License-Identifier: AGPL-3.0-only

//! Binary validator - Health checks and validation
//!
//! Validates installed genomeBin binaries:
//! - File existence
//! - Executable permissions
//! - Checksums (SHA-256)
//! - Basic execution test (--version)
//! - Size validation
//!
//! # Philosophy
//! - Zero external commands (pure Rust)
//! - Platform-agnostic
//! - Comprehensive validation
//! - Clear error reporting

use crate::types::PrimalName;
use sha2::{Digest, Sha256};
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use thiserror::Error;
use tokio::fs;
use tracing::{debug, info};

/// Validation report
///
/// Contains all validation checks for a primal binary.
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Primal being validated
    pub primal: PrimalName,
    /// Binary exists
    pub file_exists: bool,
    /// Binary is executable
    pub is_executable: bool,
    /// Binary size in bytes
    pub size_bytes: u64,
    /// Size is reasonable (1MB - 100MB)
    pub size_reasonable: bool,
    /// SHA-256 checksum (hex)
    pub checksum: Option<String>,
    /// Binary can execute --version
    pub runs: bool,
    /// Overall health status
    pub healthy: bool,
}

impl ValidationReport {
    /// Create new validation report
    pub fn new(primal: PrimalName) -> Self {
        Self {
            primal,
            file_exists: false,
            is_executable: false,
            size_bytes: 0,
            size_reasonable: false,
            checksum: None,
            runs: false,
            healthy: false,
        }
    }

    /// Check if binary is healthy
    pub fn is_healthy(&self) -> bool {
        self.file_exists && self.is_executable && self.size_reasonable && self.runs
    }
}

impl std::fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Validation Report for {}:", self.primal.display_name())?;
        writeln!(
            f,
            "  File exists:    {}",
            if self.file_exists { "✓" } else { "✗" }
        )?;
        writeln!(
            f,
            "  Executable:     {}",
            if self.is_executable { "✓" } else { "✗" }
        )?;
        writeln!(f, "  Size:           {} bytes", self.size_bytes)?;
        writeln!(
            f,
            "  Size reasonable: {}",
            if self.size_reasonable { "✓" } else { "✗" }
        )?;
        if let Some(ref checksum) = self.checksum {
            writeln!(f, "  SHA-256:        {}...", &checksum[..16])?;
        }
        writeln!(f, "  Runs:           {}", if self.runs { "✓" } else { "✗" })?;
        writeln!(
            f,
            "  Overall:        {}",
            if self.healthy {
                "✓ HEALTHY"
            } else {
                "✗ UNHEALTHY"
            }
        )?;
        Ok(())
    }
}

/// Binary validator
///
/// Validates genomeBin binaries after installation.
pub struct BinaryValidator;

impl BinaryValidator {
    /// Create new validator
    pub fn new() -> Self {
        Self
    }

    /// Validate binary
    ///
    /// Performs comprehensive validation checks on an installed binary.
    ///
    /// # Examples
    /// ```no_run
    /// # use beardog_installer::validator::BinaryValidator;
    /// # use beardog_installer::Primal;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let validator = BinaryValidator::new();
    /// let binary_path = std::path::Path::new("/usr/local/bin/beardog");
    /// let report = validator.validate_binary(PrimalName::new(PrimalName::BEARDOG), binary_path).await?;
    ///
    /// if report.is_healthy() {
    ///     println!("Binary is healthy!");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn validate_binary(
        &self,
        primal: PrimalName,
        path: &Path,
    ) -> Result<ValidationReport, ValidationError> {
        let mut report = ValidationReport::new(primal.clone());

        // 1. Check file exists
        report.file_exists = path.exists();
        if !report.file_exists {
            return Ok(report);
        }

        // 2. Check size
        let metadata = fs::metadata(path)
            .await
            .map_err(|e| ValidationError::IoError {
                path: path.to_path_buf(),
                source: e,
            })?;
        report.size_bytes = metadata.len();
        report.size_reasonable = report.size_bytes > 1_000_000 && report.size_bytes < 100_000_000;

        // 3. Check executable permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            report.is_executable = (mode & 0o111) != 0;
        }

        #[cfg(not(unix))]
        {
            // On Windows, all .exe files are executable
            report.is_executable = true;
        }

        // 4. Compute checksum
        report.checksum = Some(self.compute_sha256(path).await?);

        // 5. Test execution (--version)
        report.runs = self.test_execution(path).await.unwrap_or(false);

        // 6. Overall health
        report.healthy = report.is_healthy();

        info!(
            "Validated {}: {}",
            primal.display_name(),
            if report.healthy {
                "✓ HEALTHY"
            } else {
                "✗ UNHEALTHY"
            }
        );

        Ok(report)
    }

    /// Compute SHA-256 checksum
    async fn compute_sha256(&self, path: &Path) -> Result<String, ValidationError> {
        let bytes = fs::read(path).await.map_err(|e| ValidationError::IoError {
            path: path.to_path_buf(),
            source: e,
        })?;

        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let result = hasher.finalize();

        Ok(format!("{:x}", result))
    }

    /// Test binary execution
    async fn test_execution(&self, path: &Path) -> Result<bool, ValidationError> {
        debug!("Testing execution: {} --version", path.display());

        // Spawn --version command with timeout
        let output = tokio::time::timeout(
            Duration::from_secs(5),
            tokio::task::spawn_blocking({
                let path = path.to_path_buf();
                move || Command::new(&path).arg("--version").output()
            }),
        )
        .await
        .map_err(|_| ValidationError::ExecutionTimeout {
            path: path.to_path_buf(),
        })?
        .map_err(|e| ValidationError::IoError {
            path: path.to_path_buf(),
            source: std::io::Error::other(e.to_string()),
        })?
        .map_err(|e| ValidationError::IoError {
            path: path.to_path_buf(),
            source: e,
        })?;

        Ok(output.status.success())
    }

    /// Validate multiple binaries
    pub async fn validate_all(
        &self,
        binaries: Vec<(PrimalName, std::path::PathBuf)>,
    ) -> Vec<ValidationReport> {
        let mut reports = Vec::new();

        for (primal, path) in binaries {
            match self.validate_binary(primal.clone(), &path).await {
                Ok(report) => reports.push(report),
                Err(e) => {
                    let mut report = ValidationReport::new(primal.clone());
                    report.healthy = false;
                    debug!("Validation failed for {}: {}", primal.display_name(), e);
                    reports.push(report);
                }
            }
        }

        reports
    }
}

impl Default for BinaryValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation errors
#[derive(Debug, Error)]
pub enum ValidationError {
    /// IO error
    #[error("IO error for path {path:?}: {source}")]
    IoError {
        /// Path that caused the error
        path: std::path::PathBuf,
        /// Underlying IO error
        source: std::io::Error,
    },

    /// Execution timeout
    #[error("Binary execution timed out: {path:?}")]
    ExecutionTimeout {
        /// Path that timed out
        path: std::path::PathBuf,
    },

    /// Execution failed
    #[error("Binary execution failed for {path:?}: {source}")]
    ExecutionFailed {
        /// Path that failed
        path: std::path::PathBuf,
        /// Error message
        #[source]
        source: std::io::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_validation_report_display() {
        let mut report = ValidationReport::new(PrimalName::new(PrimalName::BEARDOG));
        report.file_exists = true;
        report.is_executable = true;
        report.size_bytes = 5_000_000;
        report.size_reasonable = true;
        report.runs = true;
        report.healthy = true;

        let display = format!("{}", report);
        assert!(display.contains("BearDog"));
        assert!(display.contains("✓ HEALTHY"));
    }

    #[tokio::test]
    async fn test_validate_binary_not_found() {
        let validator = BinaryValidator::new();
        let path = Path::new("/nonexistent/binary");

        let report = validator
            .validate_binary(PrimalName::new(PrimalName::BEARDOG), path)
            .await
            .unwrap();

        assert!(!report.file_exists);
        assert!(!report.is_healthy());
    }

    #[tokio::test]
    async fn test_validate_binary_exists() {
        let temp = TempDir::new().unwrap();
        let binary_path = temp.path().join("beardog");

        // Create a fake binary
        fs::write(&binary_path, b"#!/bin/sh\necho 'BearDog v1.0.0'")
            .await
            .unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&binary_path).await.unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&binary_path, perms).await.unwrap();
        }

        let validator = BinaryValidator::new();
        let report = validator
            .validate_binary(PrimalName::new(PrimalName::BEARDOG), &binary_path)
            .await
            .unwrap();

        assert!(report.file_exists);
        assert!(report.is_executable);
        assert!(report.checksum.is_some());
        // Note: runs might be false if the fake binary doesn't actually execute
    }

    #[tokio::test]
    async fn test_compute_sha256() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.txt");
        fs::write(&file_path, b"test content").await.unwrap();

        let validator = BinaryValidator::new();
        let checksum = validator.compute_sha256(&file_path).await.unwrap();

        // SHA-256 of "test content"
        assert_eq!(checksum.len(), 64); // SHA-256 is 64 hex characters
        assert!(checksum.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[tokio::test]
    async fn test_validate_all() {
        let temp = TempDir::new().unwrap();

        // Create multiple fake binaries
        let binaries = vec![
            (
                PrimalName::new(PrimalName::BEARDOG),
                temp.path().join("beardog"),
            ),
            (
                PrimalName::new(PrimalName::SONGBIRD),
                temp.path().join("songbird"),
            ),
        ];

        for (_, path) in &binaries {
            fs::write(path, b"#!/bin/sh\necho test").await.unwrap();

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(path).await.unwrap().permissions();
                perms.set_mode(0o755);
                fs::set_permissions(path, perms).await.unwrap();
            }
        }

        let validator = BinaryValidator::new();
        let reports = validator.validate_all(binaries).await;

        assert_eq!(reports.len(), 2);
        assert!(reports.iter().all(|r| r.file_exists));
    }

    #[test]
    fn test_validation_report_is_healthy() {
        let mut report = ValidationReport::new(PrimalName::new(PrimalName::BEARDOG));
        assert!(!report.is_healthy());

        report.file_exists = true;
        report.is_executable = true;
        report.size_reasonable = true;
        report.runs = true;

        assert!(report.is_healthy());
    }
}
