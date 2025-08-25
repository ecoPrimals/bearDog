// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Android NDK and build environment management
use beardog_errors::BearDogError;
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
        self.check_ndk()?;
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
        self.setup_ndk_environment()?;
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
            .map_err(|_| BearDogError::deployment_with_stage("rustc not found in PATH", "rust_toolchain_check"))?;
        if !output.status.success() {
            return Err(BearDogError::deployment_with_stage("rustc command failed", "rust_toolchain_check").into());
        }
        debug!("Rust version: {}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    fn check_ndk(&self) -> Result<()> {
        let ndk_home = self
            .ndk_home
            .as_ref()
            .ok_or_else(|| BearDogError::deployment_with_stage("ANDROID_NDK_HOME not set", "ndk_validation"))?;
        if !ndk_home.exists() {
            return Err(BearDogError::deployment_with_stage(
                format!("NDK directory does not exist: {}", ndk_home.display()),
                "ndk_validation"
            ).into());
        }
        // Check for required NDK components
        let toolchain_dir = ndk_home.join("toolchains/llvm/prebuilt");
        if !toolchain_dir.exists() {
            return Err(BearDogError::deployment_with_stage("NDK toolchain directory not found", "ndk_validation").into());
        }
        debug!("NDK found at: {}", ndk_home.display());
        Ok(())
    }

    async fn check_cargo_ndk(&self) -> Result<()> {
        match Command::new("cargo-ndk")
            .arg("--version")
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
            .output()
            .await
            .map_err(|e| {
                BearDogError::deployment_with_stage(format!("Failed to install cargo-ndk: {e}"), "cargo_ndk_install")
            })?;
        
        if !output.status.success() {
            return Err(BearDogError::deployment_with_stage(
                format!("cargo-ndk installation failed: {}", String::from_utf8_lossy(&output.stderr)),
                "cargo_ndk_install"
            ).into());
        }
        Ok(())
    }

    async fn check_adb(&self) -> Result<()> {
        let output = Command::new("adb")
            .arg("version")
            .output()
            .await
            .map_err(|_| BearDogError::deployment_with_stage("adb not found in PATH", "adb_check"))?;
        
        if !output.status.success() {
            return Err(BearDogError::deployment_with_stage("adb command failed", "adb_check").into());
        }
        debug!("ADB version: {}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    async fn add_android_target(&self, target: &str) -> Result<()> {
        // Check if target is already installed
        let output = Command::new("rustup")
            .args(["target", "list", "--installed"])
            .output()
            .await
            .map_err(|e| BearDogError::deployment_with_stage(format!("Failed to list targets: {e}"), "rust_target_check"))?;
        let installed_targets = String::from_utf8_lossy(&output.stdout);
        if installed_targets.contains(target) {
            debug!("Target {} already installed", target);
            return Ok(());
        }
        
        info!("📱 Adding Android target: {}", target);
        let output = Command::new("rustup")
            .args(["target", "add", target])
            .output()
            .await
            .map_err(|e| BearDogError::deployment_with_stage(format!("Failed to add target: {e}"), "rust_target_add"))?;
        
        if !output.status.success() {
            return Err(BearDogError::deployment_with_stage(
                format!("Failed to add target {}: {}", target, String::from_utf8_lossy(&output.stderr)),
                "rust_target_add"
            ).into());
        }
        Ok(())
    }

    fn setup_ndk_environment(&self) -> Result<()> {
        let ndk_home = self.ndk_home.as_ref()
            .ok_or_else(|| BearDogError::deployment_with_stage("NDK not available", "ndk_validation"))?;
        // Detect host architecture
        let host_arch = if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
            "linux-x86_64"
        } else if cfg!(target_os = "macos") {
            "darwin-x86_64" // NDK uses x86_64 tools for both Intel and M1 Macs
        } else {
            return Err(BearDogError::deployment_with_stage("Unsupported host platform", "platform_check").into());
        };
        let toolchain_path = ndk_home.join(format!("toolchains/llvm/prebuilt/{host_arch}"));
        if !toolchain_path.exists() {
            return Err(BearDogError::deployment_with_stage(
                format!("Toolchain not found for host: {host_arch}"),
                "ndk_toolchain_check"
            ).into());
        }
        debug!("NDK toolchain path: {}", toolchain_path.display());
        Ok(())
    }

    fn get_ndk_home(&self) -> Result<&PathBuf, BearDogError> {
        self.ndk_home.as_ref()
            .ok_or_else(|| BearDogError::deployment_with_stage("NDK not available", "ndk_validation"))
    }

    #[allow(dead_code)] // Future deployment functionality
    pub fn get_ndk_toolchain_path(&self, host_arch: &str) -> Result<PathBuf, BearDogError> {
        let ndk_home = self.get_ndk_home()?;
        Ok(ndk_home.join(format!("toolchains/llvm/prebuilt/{host_arch}")))
    }

    #[allow(dead_code)]
    pub fn get_project_root(&self) -> &Path {
        &self.project_root
    }
}

impl Default for AndroidManager {
    fn default() -> Self {
        Self::new(std::env::current_dir().unwrap_or_else(|_| "/tmp".into()))
    }
}
