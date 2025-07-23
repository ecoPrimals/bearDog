//! Rust build management for Android targets

use crate::error::DeployError;
use anyhow::Result;

use indicatif::{ProgressBar, ProgressStyle};
use std::{env, path::PathBuf, process::Stdio};
use tokio::process::Command;
use tracing::{debug, info, warn};

pub struct RustBuilder {
    project_root: PathBuf,
}

impl RustBuilder {
    pub fn new(project_root: PathBuf) -> Self {
        Self { project_root }
    }

    /// Build the Android application
    pub async fn build_android_app(&self, release: bool, target: &str) -> Result<()> {
        info!("🔨 Building BearDog Android application...");

        // Setup environment
        self.setup_build_environment(target)?;

        // Build the Android library
        self.build_android_library(release, target).await?;

        // Build the example app
        self.build_example_app(release, target).await?;

        info!("✅ Build completed successfully!");
        Ok(())
    }

    fn setup_build_environment(&self, target: &str) -> Result<()> {
        debug!("🔧 Setting up build environment for {}", target);

        // Determine NDK paths
        let ndk_home = env::var("ANDROID_NDK_HOME")
            .or_else(|_| env::var("NDK_HOME"))
            .map_err(|_| DeployError::ndk_not_found("ANDROID_NDK_HOME not set"))?;

        let ndk_path = PathBuf::from(ndk_home);

        // Detect host architecture
        let host_arch = if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
            "linux-x86_64"
        } else if cfg!(target_os = "macos") {
            "darwin-x86_64"
        } else {
            return Err(DeployError::prerequisites_not_met("Unsupported host platform").into());
        };

        let toolchain_path = ndk_path.join(format!("toolchains/llvm/prebuilt/{host_arch}/bin"));

        // Set environment variables for cross-compilation
        let api_level = "28"; // Android 9 for StrongBox support

        match target {
            "aarch64-linux-android" => {
                env::set_var(
                    "CC_aarch64_linux_android",
                    toolchain_path.join(format!("aarch64-linux-android{api_level}-clang")),
                );
                env::set_var(
                    "CXX_aarch64_linux_android",
                    toolchain_path.join(format!("aarch64-linux-android{api_level}-clang++")),
                );
                env::set_var("AR_aarch64_linux_android", toolchain_path.join("llvm-ar"));
                env::set_var(
                    "CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER",
                    toolchain_path.join(format!("aarch64-linux-android{api_level}-clang")),
                );
            }
            _ => {
                return Err(DeployError::UnsupportedTarget {
                    target: target.to_string(),
                }
                .into());
            }
        }

        debug!("✅ Build environment configured for {}", target);
        Ok(())
    }

    async fn build_android_library(&self, release: bool, target: &str) -> Result<()> {
        info!("📱 Building Android library...");

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.blue} {msg}")?,
        );
        pb.set_message("Building Android library...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        let android_dir = self.project_root.join("android");
        let mut cmd = Command::new("cargo");

        cmd.current_dir(&android_dir)
            .args(["ndk", "-t", target, "build"]);

        if release {
            cmd.arg("--release");
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd
            .output()
            .await
            .map_err(|e| DeployError::build_failed(format!("Failed to start build: {e}")))?;

        pb.finish_and_clear();

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Build stderr: {}", stderr);
            return Err(DeployError::build_failed(format!(
                "Android library build failed: {stderr}"
            ))
            .into());
        }

        info!("✅ Android library built successfully");
        Ok(())
    }

    async fn build_example_app(&self, release: bool, target: &str) -> Result<()> {
        info!("📖 Building example application...");

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.green} {msg}")?,
        );
        pb.set_message("Building example app...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        let mut cmd = Command::new("cargo");

        cmd.current_dir(&self.project_root).args([
            "ndk",
            "-t",
            target,
            "build",
            "--example",
            "pixel8_native_app",
        ]);

        if release {
            cmd.arg("--release");
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd.output().await.map_err(|e| {
            DeployError::build_failed(format!("Failed to start example build: {e}"))
        })?;

        pb.finish_and_clear();

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Example build stderr: {}", stderr);
            return Err(
                DeployError::build_failed(format!("Example app build failed: {stderr}")).into(),
            );
        }

        info!("✅ Example application built successfully");
        Ok(())
    }

    #[allow(dead_code)] // Future deployment functionality
    pub fn get_build_output_path(&self, release: bool, target: &str) -> PathBuf {
        let mut path = self.project_root.join("target").join(target);

        if release {
            path = path.join("release");
        } else {
            path = path.join("debug");
        }

        path
    }

    #[allow(dead_code)] // Future deployment functionality
    pub fn get_example_binary_path(
        &self,
        release: bool,
        target: &str,
        example_name: &str,
    ) -> PathBuf {
        self.get_build_output_path(release, target)
            .join("examples")
            .join(format!("lib{example_name}.so"))
    }
}
