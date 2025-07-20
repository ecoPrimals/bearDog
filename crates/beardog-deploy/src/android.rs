//! Android NDK and build environment management

use crate::error::DeployError;
use anyhow::Result;
use std::{
    env,
    path::{Path, PathBuf},
    process::Stdio,
};
use tokio::process::Command;
use tracing::{debug, info, warn};

#[derive(Debug)]
#[allow(dead_code)] // Future deployment functionality
pub struct AndroidManager {
    project_root: PathBuf,
    ndk_home: Option<PathBuf>,
}

impl AndroidManager {
    pub fn new(project_root: PathBuf) -> Self {
        let ndk_home = env::var("ANDROID_NDK_HOME")
            .or_else(|_| env::var("NDK_HOME"))
            .map(PathBuf::from)
            .ok();

        Self {
            project_root,
            ndk_home,
        }
    }

    /// Check all prerequisites for Android development
    pub async fn check_prerequisites(&self) -> Result<()> {
        info!("🔍 Checking Android development prerequisites...");

        // Check Rust installation
        self.check_rust().await?;
        info!("✅ Rust toolchain available");

        // Check Android NDK
        self.check_ndk().await?;
        info!("✅ Android NDK available");

        // Check cargo-ndk
        self.check_cargo_ndk().await?;
        info!("✅ cargo-ndk available");

        // Check ADB
        self.check_adb().await?;
        info!("✅ ADB available");

        Ok(())
    }

    /// Setup the build environment
    pub async fn setup_build_environment(&self) -> Result<()> {
        info!("🦀 Setting up Rust Android build environment...");

        // Add Android target if not present
        self.add_android_target("aarch64-linux-android").await?;
        info!("✅ Android target added");

        // Setup NDK environment variables
        self.setup_ndk_environment().await?;
        info!("✅ NDK environment configured");

        Ok(())
    }

    async fn check_rust(&self) -> Result<()> {
        let output = Command::new("rustc")
            .arg("--version")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|_| DeployError::rust_toolchain("rustc not found in PATH"))?;

        if !output.status.success() {
            return Err(DeployError::rust_toolchain("rustc command failed").into());
        }

        debug!("Rust version: {}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    async fn check_ndk(&self) -> Result<()> {
        let ndk_home = self
            .ndk_home
            .as_ref()
            .ok_or_else(|| DeployError::ndk_not_found("ANDROID_NDK_HOME not set"))?;

        if !ndk_home.exists() {
            return Err(DeployError::ndk_not_found(format!(
                "NDK directory does not exist: {}",
                ndk_home.display()
            ))
            .into());
        }

        // Check for required NDK components
        let toolchain_dir = ndk_home.join("toolchains/llvm/prebuilt");
        if !toolchain_dir.exists() {
            return Err(DeployError::ndk_not_found("NDK toolchain directory not found").into());
        }

        debug!("NDK found at: {}", ndk_home.display());
        Ok(())
    }

    async fn check_cargo_ndk(&self) -> Result<()> {
        match Command::new("cargo-ndk")
            .arg("--version")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                debug!(
                    "cargo-ndk version: {}",
                    String::from_utf8_lossy(&output.stdout)
                );
                Ok(())
            }
            _ => {
                warn!("cargo-ndk not found, installing...");
                self.install_cargo_ndk().await
            }
        }
    }

    async fn install_cargo_ndk(&self) -> Result<()> {
        info!("📦 Installing cargo-ndk...");

        let output = Command::new("cargo")
            .args(["install", "cargo-ndk"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| {
                DeployError::rust_toolchain(format!("Failed to install cargo-ndk: {e}"))
            })?;

        if !output.status.success() {
            return Err(DeployError::rust_toolchain(format!(
                "cargo-ndk installation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
            .into());
        }

        Ok(())
    }

    async fn check_adb(&self) -> Result<()> {
        let output = Command::new("adb")
            .arg("version")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|_| DeployError::prerequisites_not_met("adb not found in PATH"))?;

        if !output.status.success() {
            return Err(DeployError::prerequisites_not_met("adb command failed").into());
        }

        debug!("ADB version: {}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    async fn add_android_target(&self, target: &str) -> Result<()> {
        // Check if target is already installed
        let output = Command::new("rustup")
            .args(["target", "list", "--installed"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| DeployError::rust_toolchain(format!("Failed to list targets: {e}")))?;

        let installed_targets = String::from_utf8_lossy(&output.stdout);
        if installed_targets.contains(target) {
            debug!("Target {} already installed", target);
            return Ok(());
        }

        info!("📱 Adding Android target: {}", target);
        let output = Command::new("rustup")
            .args(["target", "add", target])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| DeployError::rust_toolchain(format!("Failed to add target: {e}")))?;

        if !output.status.success() {
            return Err(DeployError::rust_toolchain(format!(
                "Failed to add target {}: {}",
                target,
                String::from_utf8_lossy(&output.stderr)
            ))
            .into());
        }

        Ok(())
    }

    async fn setup_ndk_environment(&self) -> Result<()> {
        let ndk_home = self
            .ndk_home
            .as_ref()
            .ok_or_else(|| DeployError::ndk_not_found("NDK not available"))?;

        // Detect host architecture
        let host_arch = if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
            "linux-x86_64"
        } else if cfg!(target_os = "macos") {
            "darwin-x86_64" // NDK uses x86_64 tools for both Intel and M1 Macs
        } else {
            return Err(DeployError::prerequisites_not_met("Unsupported host platform").into());
        };

        let toolchain_path = ndk_home.join(format!("toolchains/llvm/prebuilt/{host_arch}"));

        if !toolchain_path.exists() {
            return Err(DeployError::ndk_not_found(format!(
                "Toolchain not found for host: {host_arch}"
            ))
            .into());
        }

        debug!("NDK toolchain path: {}", toolchain_path.display());
        Ok(())
    }

    #[allow(dead_code)] // Future deployment functionality
    pub fn get_ndk_toolchain_path(&self, host_arch: &str) -> Result<PathBuf> {
        let ndk_home = self
            .ndk_home
            .as_ref()
            .ok_or_else(|| DeployError::ndk_not_found("NDK not available"))?;

        Ok(ndk_home.join(format!("toolchains/llvm/prebuilt/{host_arch}")))
    }

    #[allow(dead_code)] // Future deployment functionality
    pub fn get_project_root(&self) -> &Path {
        &self.project_root
    }
}
