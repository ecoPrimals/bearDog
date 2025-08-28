use anyhow::Result;
use beardog_errors::BearDogError;
use std::{
    env,
    path::{Path, PathBuf},
    process::Stdio,
};
use tokio::process::Command;
use tracing::{debug, info, warn};

#[derive(Debug)]

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

    pub async fn check_prerequisites(&self) -> Result<()> {
        info!("🔍 Checking Android development prerequisites...");

        self.check_rust().await?;
        info!("✅ Rust toolchain available");

        self.check_ndk()?;
        info!("✅ Android NDK available");

        self.check_cargo_ndk().await?;
        info!("✅ cargo-ndk available");

        self.check_adb().await?;
        info!("✅ ADB available");
        Ok(())
    }

    pub async fn setup_build_environment(&self) -> Result<()> {
        info!("🦀 Setting up Rust Android build environment...");

        self.add_android_target("aarch64-linux-android").await?;
        info!("✅ Android target added");

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
            .map_err(|_| BearDogError::system("rustc not found in PATH"))?;
        if !output.status.success() {
            return Err(BearDogError::system("rustc command failed").into());
        }
        debug!("Rust version: {}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    fn check_ndk(&self) -> Result<()> {
        let ndk_home = self
            .ndk_home
            .as_ref()
            .ok_or_else(|| BearDogError::system("ANDROID_NDK_HOME not set"))?;
        if !ndk_home.exists() {
            return Err(BearDogError::system(format!(
                "NDK directory does not exist: {}",
                ndk_home.display()
            ))
            .into());
        }

        let toolchain_dir = ndk_home.join("toolchains/llvm/prebuilt");
        if !toolchain_dir.exists() {
            return Err(BearDogError::system("NDK toolchain directory not found").into());
        }
        debug!("NDK found at: {}", ndk_home.display());
        Ok(())
    }

    async fn check_cargo_ndk(&self) -> Result<()> {
        match Command::new("cargo-ndk").arg("--version").output().await {
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
            .map_err(|e| BearDogError::system(format!("Failed to install cargo-ndk: {e}")))?;

        if !output.status.success() {
            return Err(BearDogError::system(format!(
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
            .output()
            .await
            .map_err(|_| BearDogError::system("adb not found in PATH"))?;

        if !output.status.success() {
            return Err(BearDogError::system("adb command failed").into());
        }
        debug!("ADB version: {}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    async fn add_android_target(&self, target: &str) -> Result<()> {
        let output = Command::new("rustup")
            .args(["target", "list", "--installed"])
            .output()
            .await
            .map_err(|e| BearDogError::system(format!("Failed to list targets: {e}")))?;
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
            .map_err(|e| BearDogError::system(format!("Failed to add target: {e}")))?;

        if !output.status.success() {
            return Err(BearDogError::system(format!(
                "Failed to add target {}: {}",
                target,
                String::from_utf8_lossy(&output.stderr)
            ))
            .into());
        }
        Ok(())
    }

    fn setup_ndk_environment(&self) -> Result<()> {
        let ndk_home = self
            .ndk_home
            .as_ref()
            .ok_or_else(|| BearDogError::system("NDK not available"))?;

        #[cfg(not(target_os = "linux"))]
        {
            return Err(
                BearDogError::system(format!("Toolchain not found for host: {host_arch}")).into(),
            );
        }

        let host_arch = std::env::consts::ARCH;
        if host_arch != "x86_64" {
            return Err(
                BearDogError::system(format!("Toolchain not found for host: {host_arch}")).into(),
            );
        }
        let toolchain_path = ndk_home.join(format!("toolchains/llvm/prebuilt/{host_arch}"));
        if !toolchain_path.exists() {
            return Err(
                BearDogError::system(format!("Toolchain not found for host: {host_arch}")).into(),
            );
        }
        debug!("NDK toolchain path: {}", toolchain_path.display());
        Ok(())
    }

    #[allow(dead_code)] // Future NDK management functionality
    fn get_ndk_home(&self) -> Result<&PathBuf, BearDogError> {
        self.ndk_home
            .as_ref()
            .ok_or_else(|| BearDogError::system("NDK not available"))
    }

    #[allow(dead_code)] // Future NDK toolchain functionality
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
