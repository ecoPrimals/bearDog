// SPDX-License-Identifier: AGPL-3.0-only

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

use crate::{Architecture, OperatingSystem, platform::BiomeOSPaths, types::PrimalName};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use tracing::{debug, info, warn};

/// Binary installer
///
/// Handles installation of genomeBin binaries to the target system.
pub struct BinaryInstaller {
    paths: BiomeOSPaths,
    source_dir: PathBuf,
}

impl BinaryInstaller {
    /// Create new installer
    ///
    /// # Arguments
    /// - `paths`: Installation paths
    /// - `source_dir`: Directory containing compiled binaries
    pub const fn new(paths: BiomeOSPaths, source_dir: PathBuf) -> Self {
        Self { paths, source_dir }
    }

    /// Locate binary in source tree
    ///
    /// Searches for the primal's binary based on architecture and OS.
    ///
    /// # Examples
    /// ```no_run
    /// # use beardog_installer::installer::BinaryInstaller;
    /// # use beardog_installer::{BiomeOSPaths, Architecture, OperatingSystem, PrimalName};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let paths = BiomeOSPaths::discover()?;
    /// let source = std::path::PathBuf::from("./target/release");
    /// let installer = BinaryInstaller::new(paths, source);
    ///
    /// let arch = Architecture::X86_64;
    /// let os = OperatingSystem::Linux;
    /// let binary = installer.locate_binary(PrimalName::new(PrimalName::BEARDOG), &arch, &os)?;
    /// # Ok(())
    /// # }
    /// ```
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

        let paths = BiomeOSPaths {
            bin_dir: bin_dir.clone(),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        let installer = BinaryInstaller::new(paths, source_dir);

        // Install
        let installed = installer
            .install_binary(PrimalName::new(PrimalName::BEARDOG), &fake_binary)
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

        let paths = BiomeOSPaths {
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
            .uninstall_binary(PrimalName::new(PrimalName::BEARDOG))
            .await
            .expect("uninstall");

        assert!(!binary_path.exists());
    }

    #[tokio::test]
    async fn test_is_installed() {
        let temp = TempDir::new().expect("tempdir");
        let bin_dir = temp.path().join("bin");

        fs::create_dir_all(&bin_dir).await.expect("create bin dir");

        let paths = BiomeOSPaths {
            bin_dir: bin_dir.clone(),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        let installer = BinaryInstaller::new(paths, temp.path().to_path_buf());

        // Not installed initially
        assert!(
            !installer
                .is_installed(PrimalName::new(PrimalName::BEARDOG))
                .await
        );

        // Create binary
        fs::write(bin_dir.join("beardog"), b"test")
            .await
            .expect("write binary");

        // Now installed
        assert!(
            installer
                .is_installed(PrimalName::new(PrimalName::BEARDOG))
                .await
        );
    }

    #[test]
    fn test_binary_path() {
        let temp = TempDir::new().expect("tempdir");
        let bin_dir = temp.path().join("bin");

        let paths = BiomeOSPaths {
            bin_dir: bin_dir.clone(),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        let installer = BinaryInstaller::new(paths, temp.path().to_path_buf());

        assert_eq!(
            installer.binary_path(PrimalName::new(PrimalName::BEARDOG)),
            bin_dir.join("beardog")
        );
    }
}
