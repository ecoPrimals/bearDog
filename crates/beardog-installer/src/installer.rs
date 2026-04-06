// SPDX-License-Identifier: AGPL-3.0-or-later

//! Binary installer - Core installation logic
//!
//! Handles the actual installation of genomeBin binaries:
//! - Locating binaries in source tree
//! - Copying to installation directory
//! - Setting permissions
//! - Uninstallation
//!
//! # Philosophy
//! - Zero external commands (pure Rust file operations)
//! - Async for concurrency
//! - Platform-agnostic (works everywhere)
//! - Atomic operations (all-or-nothing)

use crate::{Architecture, OperatingSystem, platform::PlatformPaths, types::PrimalName};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use tracing::{debug, info, warn};

/// Binary installer
///
/// Handles installation of genomeBin binaries to the target system.
pub struct BinaryInstaller {
    paths: PlatformPaths,
    source_dir: PathBuf,
}

impl BinaryInstaller {
    /// Create new installer
    ///
    /// # Arguments
    /// - `paths`: Installation paths
    /// - `source_dir`: Directory containing compiled binaries
    pub const fn new(paths: PlatformPaths, source_dir: PathBuf) -> Self {
        Self { paths, source_dir }
    }

    /// Locate binary in source tree
    ///
    /// Searches for the primal's binary based on architecture and OS.
    ///
    /// # Examples
    /// ```no_run
    /// # use beardog_installer::installer::BinaryInstaller;
    /// # use beardog_installer::{PlatformPaths, Architecture, OperatingSystem, PrimalName};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let paths = PlatformPaths::discover()?;
    /// let source = std::path::PathBuf::from("./target/release");
    /// let installer = BinaryInstaller::new(paths, source);
    ///
    /// let arch = Architecture::X86_64;
    /// let os = OperatingSystem::Linux;
    /// let binary = installer.locate_binary(PrimalName::new("beardog"), &arch, &os)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if no matching binary is found under the expected source paths for the
    /// given architecture and OS.
    pub fn locate_binary(
        &self,
        primal: PrimalName,
        arch: &Architecture,
        os: &OperatingSystem,
    ) -> Result<PathBuf, InstallerError> {
        let target = arch.to_rust_target(os);
        let extension = arch.binary_extension(os);
        let binary_name = format!("{}{}", primal.name(), extension);

        // Try multiple locations
        let mut candidates = vec![
            // 1. Target-specific directory (cross-compilation)
            self.source_dir.join(&target).join(&binary_name),
            // 2. Release directory (same-arch compilation)
            self.source_dir.join(&binary_name),
        ];

        // 3. Parent's target directory (if parent exists)
        if let Some(parent) = self.source_dir.parent() {
            candidates.push(parent.join(&target).join(&binary_name));
        }

        for candidate in &candidates {
            if candidate.exists() {
                debug!("Found binary at: {}", candidate.display());
                return Ok(candidate.clone());
            }
        }

        Err(InstallerError::BinaryNotFound {
            primal,
            target,
            searched: candidates
                .into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
        })
    }

    /// Install binary
    ///
    /// Copies binary to installation directory and sets executable permissions.
    ///
    /// # Errors
    /// Returns error if:
    /// - Binary cannot be located
    /// - Installation directory is not writable
    /// - Copy operation fails
    pub async fn install_binary(
        &self,
        primal: PrimalName,
        source: &Path,
    ) -> Result<PathBuf, InstallerError> {
        // Ensure bin directory exists
        fs::create_dir_all(&self.paths.bin_dir)
            .await
            .map_err(|e| InstallerError::IoError {
                path: self.paths.bin_dir.clone(),
                source: e,
            })?;

        // Destination path
        let dest = self.paths.bin_dir.join(primal.name());

        info!("Installing {} to {}", primal.display_name(), dest.display());

        // Copy binary (atomic operation)
        fs::copy(source, &dest)
            .await
            .map_err(|e| InstallerError::CopyFailed {
                from: source.to_path_buf(),
                to: dest.clone(),
                source: e,
            })?;

        // Set executable permissions (Unix-like systems)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&dest)
                .await
                .map_err(|e| InstallerError::IoError {
                    path: dest.clone(),
                    source: e,
                })?
                .permissions();
            perms.set_mode(0o755); // rwxr-xr-x
            fs::set_permissions(&dest, perms)
                .await
                .map_err(|e| InstallerError::IoError {
                    path: dest.clone(),
                    source: e,
                })?;
        }

        info!("✓ {} installed successfully", primal.display_name());
        Ok(dest)
    }

    /// Uninstall binary
    ///
    /// Removes the primal's binary from the installation directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the installed binary exists but cannot be removed.
    pub async fn uninstall_binary(&self, primal: PrimalName) -> Result<(), InstallerError> {
        let binary_path = self.paths.bin_dir.join(primal.name());

        if !binary_path.exists() {
            warn!(
                "{} not found at {}",
                primal.display_name(),
                binary_path.display()
            );
            return Ok(());
        }

        info!(
            "Uninstalling {} from {}",
            primal.display_name(),
            binary_path.display()
        );

        fs::remove_file(&binary_path)
            .await
            .map_err(|e| InstallerError::IoError {
                path: binary_path.clone(),
                source: e,
            })?;

        info!("✓ {} uninstalled successfully", primal.display_name());
        Ok(())
    }

    /// Check if binary is installed
    pub async fn is_installed(&self, primal: PrimalName) -> bool {
        let binary_path = self.paths.bin_dir.join(primal.name());
        binary_path.exists()
    }

    /// Get installed binary path
    pub fn binary_path(&self, primal: PrimalName) -> PathBuf {
        self.paths.bin_dir.join(primal.name())
    }
}

/// Installer errors
#[derive(Debug, Error)]
pub enum InstallerError {
    /// Binary not found in source tree
    #[error("Binary not found for {primal:?} (target: {target}). Searched:\n{}", searched.join("\n"))]
    BinaryNotFound {
        /// The primal being installed
        primal: PrimalName,
        /// Target triple (e.g., x86_64-unknown-linux-gnu)
        target: String,
        /// Paths that were searched
        searched: Vec<String>,
    },

    /// Copy operation failed
    #[error("Failed to copy binary from {from:?} to {to:?}: {source}")]
    CopyFailed {
        /// Source path
        from: PathBuf,
        /// Destination path
        to: PathBuf,
        /// Underlying IO error
        source: std::io::Error,
    },

    /// IO error
    #[error("IO error for path {path:?}: {source}")]
    IoError {
        /// Path that caused the error
        path: PathBuf,
        /// Underlying IO error
        source: std::io::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arch::Architecture;
    use crate::platform::OperatingSystem;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_install_binary() {
        let temp = TempDir::new().expect("tempdir");
        let source_dir = temp.path().join("source");
        let bin_dir = temp.path().join("bin");

        fs::create_dir_all(&source_dir)
            .await
            .expect("create source dir");

        // Create a fake binary
        let fake_binary = source_dir.join("beardog");
        fs::write(&fake_binary, b"#!/bin/sh\necho test")
            .await
            .expect("write fake binary");

        let paths = PlatformPaths {
            bin_dir: bin_dir.clone(),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        let installer = BinaryInstaller::new(paths, source_dir);

        // Install
        let installed = installer
            .install_binary(PrimalName::new("beardog"), &fake_binary)
            .await
            .expect("install binary");

        assert!(installed.exists());
        assert_eq!(installed, bin_dir.join("beardog"));

        // Check it's executable (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::metadata(&installed)
                .await
                .expect("metadata")
                .permissions();
            assert_eq!(perms.mode() & 0o111, 0o111); // Executable
        }
    }

    #[tokio::test]
    async fn test_uninstall_binary() {
        let temp = TempDir::new().expect("tempdir");
        let bin_dir = temp.path().join("bin");

        fs::create_dir_all(&bin_dir).await.expect("create bin dir");

        // Create installed binary
        let binary_path = bin_dir.join("beardog");
        fs::write(&binary_path, b"test")
            .await
            .expect("write binary");

        let paths = PlatformPaths {
            bin_dir: bin_dir.clone(),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        let installer = BinaryInstaller::new(paths, temp.path().to_path_buf());

        assert!(binary_path.exists());

        // Uninstall
        installer
            .uninstall_binary(PrimalName::new("beardog"))
            .await
            .expect("uninstall");

        assert!(!binary_path.exists());
    }

    #[tokio::test]
    async fn test_is_installed() {
        let temp = TempDir::new().expect("tempdir");
        let bin_dir = temp.path().join("bin");

        fs::create_dir_all(&bin_dir).await.expect("create bin dir");

        let paths = PlatformPaths {
            bin_dir: bin_dir.clone(),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        let installer = BinaryInstaller::new(paths, temp.path().to_path_buf());

        // Not installed initially
        assert!(!installer.is_installed(PrimalName::new("beardog")).await);

        // Create binary
        fs::write(bin_dir.join("beardog"), b"test")
            .await
            .expect("write binary");

        // Now installed
        assert!(installer.is_installed(PrimalName::new("beardog")).await);
    }

    #[test]
    fn test_binary_path() {
        let temp = TempDir::new().expect("tempdir");
        let bin_dir = temp.path().join("bin");

        let paths = PlatformPaths {
            bin_dir: bin_dir.clone(),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        let installer = BinaryInstaller::new(paths, temp.path().to_path_buf());

        assert_eq!(
            installer.binary_path(PrimalName::new("beardog")),
            bin_dir.join("beardog")
        );
    }

    #[test]
    fn test_locate_binary_prefers_target_subdirectory() {
        let temp = TempDir::new().expect("tempdir");
        let source_dir = temp.path().join("release");
        std::fs::create_dir_all(source_dir.join("x86_64-unknown-linux-gnu")).expect("mkdir");

        let target_bin = source_dir.join("x86_64-unknown-linux-gnu").join("beardog");
        std::fs::write(&target_bin, b"x").expect("write");

        let paths = PlatformPaths {
            bin_dir: temp.path().join("bin"),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        let installer = BinaryInstaller::new(paths, source_dir);
        let arch = Architecture::X86_64;
        let os = OperatingSystem::Linux;

        let found = installer
            .locate_binary(PrimalName::new("beardog"), &arch, &os)
            .expect("locate");
        assert_eq!(found, target_bin);
    }

    #[test]
    fn test_locate_binary_finds_sibling_target_tree() {
        let temp = TempDir::new().expect("tempdir");
        let release = temp.path().join("release");
        let gnu_dir = temp.path().join("x86_64-unknown-linux-gnu");
        std::fs::create_dir_all(&release).expect("mkdir release");
        std::fs::create_dir_all(&gnu_dir).expect("mkdir gnu");

        let candidate = gnu_dir.join("beardog");
        std::fs::write(&candidate, b"y").expect("write");

        let paths = PlatformPaths {
            bin_dir: temp.path().join("bin"),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        let installer = BinaryInstaller::new(paths, release);
        let arch = Architecture::X86_64;
        let os = OperatingSystem::Linux;

        let found = installer
            .locate_binary(PrimalName::new("beardog"), &arch, &os)
            .expect("locate");
        assert_eq!(found, candidate);
    }

    #[test]
    fn test_locate_binary_not_found_lists_candidates() {
        let temp = TempDir::new().expect("tempdir");
        let source_dir = temp.path().join("empty");
        std::fs::create_dir_all(&source_dir).expect("mkdir");

        let paths = PlatformPaths {
            bin_dir: temp.path().join("bin"),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        let installer = BinaryInstaller::new(paths, source_dir);
        let arch = Architecture::X86_64;
        let os = OperatingSystem::Linux;

        let err = installer
            .locate_binary(PrimalName::new("missing-primal"), &arch, &os)
            .expect_err("missing binary");
        let msg = err.to_string();
        assert!(
            msg.contains("Binary not found"),
            "unexpected message: {msg}"
        );
        assert!(msg.contains("x86_64-unknown-linux-gnu"));
    }

    #[tokio::test]
    async fn test_uninstall_binary_missing_is_ok() {
        let temp = TempDir::new().expect("tempdir");
        let bin_dir = temp.path().join("bin");
        std::fs::create_dir_all(&bin_dir).expect("mkdir");

        let paths = PlatformPaths {
            bin_dir: bin_dir.clone(),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        let installer = BinaryInstaller::new(paths, temp.path().to_path_buf());
        installer
            .uninstall_binary(PrimalName::new("nope"))
            .await
            .expect("uninstall missing should succeed");
    }

    #[tokio::test]
    async fn test_install_binary_errors_when_bin_dir_path_is_a_file() {
        let temp = TempDir::new().expect("tempdir");
        let bin_path = temp.path().join("bin");
        tokio::fs::write(&bin_path, b"not-a-directory")
            .await
            .expect("create file where bin dir should be");

        let paths = PlatformPaths {
            bin_dir: bin_path.clone(),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        let source_dir = temp.path().join("src");
        tokio::fs::create_dir_all(&source_dir)
            .await
            .expect("mkdir source");
        let src_bin = source_dir.join("beardog");
        tokio::fs::write(&src_bin, b"hello")
            .await
            .expect("write source");

        let installer = BinaryInstaller::new(paths, source_dir);
        let err = installer
            .install_binary(PrimalName::new("beardog"), &src_bin)
            .await
            .expect_err("create_dir_all on file path should fail");
        assert!(
            matches!(err, InstallerError::IoError { .. }),
            "unexpected error: {err:?}"
        );
    }

    #[test]
    fn test_locate_binary_resolves_windows_exe_name() {
        let temp = TempDir::new().expect("tempdir");
        let release = temp.path().join("release");
        let target_dir = release.join("x86_64-pc-windows-gnu");
        std::fs::create_dir_all(&target_dir).expect("mkdir");
        let exe = target_dir.join("beardog.exe");
        std::fs::write(&exe, b"x").expect("write exe");

        let paths = PlatformPaths {
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
                &OperatingSystem::Windows,
            )
            .expect("locate windows exe");
        assert!(found.to_string_lossy().contains("beardog.exe"));
    }

    #[test]
    fn test_installer_error_binary_not_found_display() {
        let err = InstallerError::BinaryNotFound {
            primal: PrimalName::new("missing"),
            target: "x86_64-unknown-linux-gnu".to_string(),
            searched: vec!["/a".to_string(), "/b".to_string()],
        };
        let s = err.to_string();
        assert!(s.contains("Binary not found") && s.contains("missing"));
    }
}
