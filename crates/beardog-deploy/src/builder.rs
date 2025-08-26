

use beardog_errors::{BearDogError, BearDogResult};
use anyhow::Result as AnyhowResult;

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

    pub async fn build_android_app(&self, release: bool, target: &str) -> AnyhowResult<()> {
        info!("🔨 Building BearDog Android application...");

        self.setup_build_environment(target)?;

        self.build_android_library(release, target).await?;

        self.build_example_app(release, target).await?;
        info!("✅ Build completed successfully!");
        Ok(())
    }

    fn setup_build_environment(&self, target: &str) -> BearDogResult<()> {
        debug!("🔧 Setting up build environment for {}", target);

        let ndk_home = env::var("ANDROID_NDK_HOME")
            .or_else(|_| env::var("NDK_HOME"))
            .map_err(|_| BearDogError::system("ANDROID_NDK_HOME not set"))?;
        let ndk_path = PathBuf::from(ndk_home);

        let host_arch = if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
            "linux-x86_64"
        } else if cfg!(target_os = "macos") {
            "darwin-x86_64"
        } else {
            return Err(BearDogError::system("Unsupported host platform"));
        };
        let toolchain_path = ndk_path.join(format!("toolchains/llvm/prebuilt/{host_arch}/bin"));

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
                return Err(BearDogError::system(
                    format!("Unsupported target: {target}")
                ));
            }
        }
        debug!("✅ Build environment configured for {}", target);
        Ok(())
    }

    async fn build_android_library(&self, release: bool, target: &str) -> BearDogResult<()> {
        info!("📱 Building Android library...");
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} {msg}")
                .map_err(|e| BearDogError::system(format!("Progress bar template error: {e}")))?
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
            .map_err(|e| BearDogError::system(format!("Failed to start build: {e}")))?;
        pb.finish_and_clear();
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Build stderr: {}", stderr);
            return Err(BearDogError::system(format!("Android library build failed: {stderr}")));
        }
        info!("✅ Android library built successfully");
        Ok(())
    }

    async fn build_example_app(&self, _release: bool, target: &str) -> AnyhowResult<()> {
        info!("📖 Building example application...");
        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")?,
        );
        pb.set_message("Building example app...");
        let mut cmd = Command::new("cargo");
        cmd.current_dir(&self.project_root).args([
            "ndk",
            "-t",
            target,
            "build",
            "--example",
            "pixel8_native_app",
        ]);
        let output = cmd.output().await.map_err(|e| 
            BearDogError::system(format!("Failed to start example build: {e}"))
        )?;
        pb.finish_with_message("Example app built!");
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Example build stderr: {}", stderr);
            return Err(BearDogError::system(
                format!("Example app build failed: {stderr}")
            ).into());
        }
        info!("✅ Example application built successfully");
        Ok(())
    }

    #[allow(dead_code)] // Future deployment functionality
    pub fn get_build_output_path(&self, _release: bool, target: &str) -> PathBuf {
        let mut path = self.project_root.join("target").join(target);
            path = path.join("release");
            path = path.join("debug");
        path
    }

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
