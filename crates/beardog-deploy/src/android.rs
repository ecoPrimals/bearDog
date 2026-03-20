// SPDX-License-Identifier: AGPL-3.0-only

// Android deployment and build verification
//
// This module provides comprehensive Android deployment capabilities including
// Rust toolchain verification, NDK setup, and cargo-ndk installation.
// All operations maintain sovereignty compliance and zero hardcoded assumptions.

use beardog_errors::BearDogError;
use std::process::Command;
use tracing::{info, warn};

type Result<T> = std::result::Result<T, BearDogError>;

/// Android deployment manager
///
/// Handles all aspects of Android deployment including toolchain verification,
/// NDK setup, and build environment preparation.
#[derive(Debug)]
pub struct AndroidDeployment {
    /// Android NDK path (if configured)
    ndk_path: Option<String>,
    /// Target Android API level
    #[allow(dead_code)] // Will be used in future API-level specific operations
    api_level: u32,
}

impl AndroidDeployment {
    /// Creates a new Android deployment manager
    ///
    /// # Arguments
    /// * `ndk_path` - Optional path to Android NDK
    /// * `api_level` - Target Android API level
    ///
    /// # Returns
    /// A new `AndroidDeployment` instance
    #[must_use]
    pub const fn new(ndk_path: Option<String>, api_level: u32) -> Self {
        Self {
            ndk_path,
            api_level,
        }
    }

    /// Verifies the complete Android build environment
    ///
    /// including Rust toolchain, Android NDK, and cargo-ndk installation.
    ///
    /// # Returns
    /// `Ok(())` if all components are properly configured
    ///
    /// # Errors
    /// Returns error if any required component is missing or misconfigured
    pub fn verify_environment(&self) -> Result<()> {
        info!("🤖 Verifying Android deployment environment");

        // Verify all components in sequence
        self.verify_all_components()?;

        info!("✅ Android deployment environment verified successfully");
        Ok(())
    }

    /// Verifies all Android deployment components
    ///
    /// # Returns
    /// `Ok(())` if all components are verified
    ///
    /// # Errors
    /// Returns error if any component verification fails
    fn verify_all_components(&self) -> Result<()> {
        Self::verify_rust_installation()?;
        self.verify_android_ndk()?;
        Self::verify_cargo_ndk()?;
        Ok(())
    }

    /// Verifies Rust installation and Android targets
    ///
    /// Checks that Rust is installed and that required Android targets are available.
    /// Installs missing targets automatically if needed.
    ///
    /// # Returns
    /// `Ok(())` if Rust and Android targets are properly configured
    ///
    /// # Errors
    /// Returns error if Rust is not installed or target installation fails
    fn verify_rust_installation() -> Result<()> {
        info!("🦀 Checking Rust installation");

        // Check basic Rust installation
        Self::check_rust_compiler()?;

        // Check rustup for target management
        if !Self::check_rustup_available() {
            warn!("rustup not available, cannot manage Android targets automatically");
            return Ok(());
        }

        // Verify and install Android targets
        Self::verify_android_targets()?;

        info!("✅ Rust installation verified");
        Ok(())
    }

    /// Checks if Rust compiler is available
    ///
    /// # Returns
    /// `Ok(())` if rustc is available
    ///
    /// # Errors
    /// Returns error if Rust compiler is not found
    fn check_rust_compiler() -> Result<()> {
        if !Self::check_command_available("rustc") {
            return Err(BearDogError::system(
                "Rust compiler not found. Please install Rust.".to_string(),
            ));
        }
        Ok(())
    }

    ///
    /// # Returns
    /// `true` if rustup is available, `false` otherwise
    #[must_use]
    fn check_rustup_available() -> bool {
        Self::check_command_available("rustup")
    }

    /// Verifies that required Android targets are installed
    ///
    ///
    /// # Returns
    /// `Ok(())` if all targets are available
    ///
    /// # Errors
    /// Returns error if target installation fails
    fn verify_android_targets() -> Result<()> {
        let required_targets = [
            "aarch64-linux-android",
            "armv7-linux-androideabi",
            "x86_64-linux-android",
            "i686-linux-android",
        ];

        for target in &required_targets {
            if !Self::is_target_installed(target)? {
                info!("📱 Installing Android target: {target}");
                Self::install_target(target)?;
            }
        }

        Ok(())
    }

    /// Checks if a specific Rust target is installed
    ///
    /// # Arguments
    /// * `target` - Target triple to check
    ///
    /// # Returns
    /// `true` if target is installed, `false` otherwise
    ///
    /// # Errors
    /// Returns error if rustup command fails
    /// Checks if target installed
    fn is_target_installed(target: &str) -> Result<bool> {
        let output = Command::new("rustup")
            .args(["target", "list", "--installed"])
            .output()
            .map_err(|e| BearDogError::system(format!("Failed to check installed targets: {e}")))?;

        let installed_targets = String::from_utf8_lossy(&output.stdout);
        Ok(installed_targets.lines().any(|line| line.trim() == target))
    }

    /// Installs a specific Rust target
    ///
    /// # Arguments
    /// * `target` - Target triple to install
    ///
    /// # Returns
    /// `Ok(())` if installation succeeds
    ///
    /// # Errors
    /// Returns error if target installation fails
    fn install_target(target: &str) -> Result<()> {
        let status = Command::new("rustup")
            .args(["target", "add", target])
            .status()
            .map_err(|e| BearDogError::system(format!("Failed to run rustup: {e}")))?;

        if !status.success() {
            return Err(BearDogError::system(format!(
                "Failed to install target: {target}"
            )));
        }

        Ok(())
    }

    /// Verifies Android NDK installation
    ///
    /// Checks that the Android NDK is properly installed and configured.
    ///
    /// # Returns
    /// `Ok(())` if NDK is properly configured
    ///
    /// # Errors
    /// Returns error if NDK is not found or misconfigured
    fn verify_android_ndk(&self) -> Result<()> {
        info!("📱 Checking Android NDK");

        // Check for NDK path in various locations
        let ndk_path = self.find_ndk_path()?;
        info!("✅ Android NDK found at: {ndk_path}");

        Ok(())
    }

    /// Finds the Android NDK path from various sources
    ///
    /// # Returns
    /// NDK path if found
    ///
    /// # Errors
    /// Returns error if NDK cannot be located
    fn find_ndk_path(&self) -> Result<String> {
        // Check provided path first
        if let Some(ref path) = self.ndk_path {
            if std::path::Path::new(path).exists() {
                return Ok(path.clone());
            }
        }

        // Check environment variables
        if let Ok(ndk_home) = beardog_errors::process_env::var("ANDROID_NDK_HOME") {
            if std::path::Path::new(&ndk_home).exists() {
                return Ok(ndk_home);
            }
        }

        if let Ok(ndk_root) = beardog_errors::process_env::var("NDK_HOME") {
            if std::path::Path::new(&ndk_root).exists() {
                return Ok(ndk_root);
            }
        }

        Err(BearDogError::system(
            "Android NDK not found. Set ANDROID_NDK_HOME environment variable.".to_string(),
        ))
    }

    /// Verifies cargo-ndk installation
    ///
    /// Checks that cargo-ndk is installed and offers to install it if missing.
    ///
    /// # Returns
    /// `Ok(())` if cargo-ndk is available
    ///
    /// # Errors
    /// Returns error if cargo-ndk installation fails
    fn verify_cargo_ndk() -> Result<()> {
        info!("🔧 Checking cargo-ndk");

        if Self::check_command_available("cargo-ndk") {
            info!("✅ cargo-ndk is installed");
            return Ok(());
        }

        warn!("cargo-ndk not found");
        info!("🔧 Installing cargo-ndk...");
        Self::install_cargo_ndk()?;

        Ok(())
    }

    /// Installs cargo-ndk using cargo
    ///
    /// Downloads and installs the latest version of cargo-ndk from crates.io.
    ///
    /// # Returns
    /// `Ok(())` if installation succeeds
    ///
    /// # Errors
    /// Returns error if cargo-ndk installation fails
    fn install_cargo_ndk() -> Result<()> {
        info!("📦 Installing cargo-ndk from crates.io");

        let status = Command::new("cargo")
            .args(["install", "cargo-ndk"])
            .status()
            .map_err(|e| BearDogError::system(format!("Failed to run cargo install: {e}")))?;

        if !status.success() {
            return Err(BearDogError::system(
                "Failed to install cargo-ndk".to_string(),
            ));
        }

        info!("✅ cargo-ndk installed successfully");
        Ok(())
    }

    /// Checks if a command is available in the system PATH
    ///
    /// # Arguments
    /// * `command` - Command name to check
    ///
    /// # Returns
    /// `true` if command is available, `false` otherwise
    #[must_use]
    fn check_command_available(command: &str) -> bool {
        Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_ndk_path() {
        let d = AndroidDeployment::new(Some("/opt/ndk".to_string()), 33);
        assert!(format!("{:?}", d).contains("/opt/ndk"));
    }

    #[test]
    fn test_new_without_ndk_path() {
        let d = AndroidDeployment::new(None, 28);
        assert!(format!("{:?}", d).contains("None"));
    }

    #[test]
    fn test_check_command_available_rustc() {
        // rustc should always be available in our build environment
        assert!(AndroidDeployment::check_command_available("rustc"));
    }

    #[test]
    fn test_check_command_available_nonexistent() {
        assert!(!AndroidDeployment::check_command_available(
            "nonexistent_command_xyz_12345"
        ));
    }

    #[test]
    fn test_check_command_available_cargo() {
        assert!(AndroidDeployment::check_command_available("cargo"));
    }

    #[test]
    fn test_check_rustup_available() {
        // rustup should be available in dev environment
        let result = AndroidDeployment::check_rustup_available();
        // Just exercise the code path; result depends on environment
        let _ = result;
    }

    #[test]
    fn test_check_rust_compiler() {
        // Should succeed in our build environment
        assert!(AndroidDeployment::check_rust_compiler().is_ok());
    }

    #[test]
    fn test_find_ndk_path_with_provided_path_nonexistent() {
        let d = AndroidDeployment::new(Some("/nonexistent/ndk".to_string()), 33);
        let result = d.find_ndk_path();
        // Should fail because path doesn't exist and env vars likely not set
        if beardog_errors::process_env::var("ANDROID_NDK_HOME").is_err()
            && beardog_errors::process_env::var("NDK_HOME").is_err()
        {
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_find_ndk_path_no_path_no_env() {
        let d = AndroidDeployment::new(None, 33);
        if beardog_errors::process_env::var("ANDROID_NDK_HOME").is_err()
            && beardog_errors::process_env::var("NDK_HOME").is_err()
        {
            assert!(d.find_ndk_path().is_err());
        }
    }

    #[test]
    fn test_verify_environment_exercises_chain() {
        let d = AndroidDeployment::new(None, 33);
        // Exercise the full verify chain, result depends on environment
        let _ = d.verify_environment();
    }

    #[test]
    fn test_verify_android_ndk_no_ndk() {
        let d = AndroidDeployment::new(None, 33);
        if beardog_errors::process_env::var("ANDROID_NDK_HOME").is_err()
            && beardog_errors::process_env::var("NDK_HOME").is_err()
        {
            assert!(d.verify_android_ndk().is_err());
        }
    }

    #[test]
    fn test_verify_cargo_ndk() {
        // Exercise verify_cargo_ndk - will check if cargo-ndk is installed
        let _ = AndroidDeployment::verify_cargo_ndk();
    }

    #[test]
    fn test_is_target_installed() {
        // Check a target we know exists
        if AndroidDeployment::check_rustup_available() {
            // Check the host target - should be installed
            let result = AndroidDeployment::is_target_installed("x86_64-unknown-linux-gnu");
            assert!(result.is_ok());
        }
    }
}
