// Rust build system for Android deployment
//
// This module provides comprehensive Rust build capabilities for Android targets
// including library compilation, example app building, and toolchain configuration.
// All operations maintain sovereignty compliance and zero hardcoded assumptions.

use anyhow::Result as AnyhowResult;
use beardog_errors::BearDogError;
use std::{env, path::PathBuf, process::Stdio};
use tokio::process::Command;
use tracing::{debug, info};

///
/// example app compilation, and cross-compilation toolchain setup.
#[derive(Debug)]
pub struct RustBuilder {
    /// Project root directory path
    project_root: PathBuf,
}

impl RustBuilder {
    /// Creates a new Rust builder instance
    ///
    /// # Arguments
    /// * `project_root` - Path to the project root directory
    ///
    /// # Returns
    /// A new `RustBuilder` instance
    #[must_use]
    /// Creates a new instance
    pub fn new(project_root: &std::path::Path) -> Self {
        Self {
            project_root: project_root.to_path_buf(),
        }
    }

    /// Builds complete Android application
    ///
    /// library compilation, and example app building.
    ///
    /// # Arguments
    /// * `release` - Whether to build in release mode
    /// * `target` - Target architecture (e.g., "aarch64-linux-android")
    ///
    /// # Returns
    /// `Ok(())` if build succeeds
    ///
    /// # Errors
    /// Returns error if any build step fails
    /// Builds android_app
    /// Builds android_app
    pub async fn build_android_app(&self, release: bool, target: &str) -> AnyhowResult<()> {
        info!("🔨 Building BearDog Android application...");

        // Setup build environment for target
        Self::setup_build_environment(target)?;

        // Build library and example app
        self.build_android_library(release, target).await?;
        self.build_example_app(release, target).await?;

        info!("✅ Build completed successfully!");
        Ok(())
    }

    ///
    ///
    /// # Arguments
    /// * `target` - Target architecture to configure
    ///
    /// # Returns
    /// `Ok(())` if environment setup succeeds
    ///
    /// # Errors
    /// Returns error if NDK is not found or target is unsupported
    /// Sets valueup_build_environment
    fn setup_build_environment(target: &str) -> Result<(), BearDogError> {
        debug!("🔧 Setting up build environment for {target}");

        // Get NDK path from environment
        let ndk_home = Self::get_ndk_path()?;
        let ndk_path = PathBuf::from(ndk_home);

        // Determine host architecture
        let host_arch = Self::get_host_architecture()?;
        let toolchain_path = ndk_path.join(format!("toolchains/llvm/prebuilt/{host_arch}/bin"));

        // Configure toolchain for target
        Self::configure_target_toolchain(target, &toolchain_path)?;

        Ok(())
    }

    /// Gets Android NDK path from environment variables
    ///
    /// # Returns
    /// NDK path string
    ///
    /// # Errors
    /// Returns error if NDK path is not configured
    /// Gets ndk_path
    fn get_ndk_path() -> Result<String, BearDogError> {
        env::var("ANDROID_NDK_HOME")
            .or_else(|_| env::var("NDK_HOME"))
            .map_err(|_| BearDogError::system("ANDROID_NDK_HOME not set".to_string()))
    }

    ///
    /// # Returns
    /// Host architecture string
    ///
    /// # Errors
    /// Gets host_architecture
    fn get_host_architecture() -> Result<&'static str, BearDogError> {
        if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
            Ok("linux-x86_64")
        } else if cfg!(target_os = "macos") {
            Ok("darwin-x86_64")
        } else {
            Err(BearDogError::system(
                "Unsupported host platform".to_string(),
            ))
        }
    }

    ///
    /// # Arguments
    /// * `target` - Target architecture
    /// * `toolchain_path` - Path to NDK toolchain binaries
    ///
    /// # Returns
    /// `Ok(())` if configuration succeeds
    ///
    /// # Errors
    /// Returns error if target is unsupported
    fn configure_target_toolchain(
        target: &str,
        toolchain_path: &std::path::Path,
    ) -> Result<(), BearDogError> {
        let api_level = "28"; // Android 9 for StrongBox support

        match target {
            "aarch64-linux-android" => {
                Self::set_aarch64_toolchain(toolchain_path, api_level);
            }
            "armv7-linux-androideabi" => {
                Self::set_armv7_toolchain(toolchain_path, api_level);
            }
            _ => {
                return Err(BearDogError::system(format!(
                    "Unsupported target: {target}"
                )));
            }
        }

        Ok(())
    }

    /// Sets up aarch64 toolchain environment variables
    ///
    /// # Arguments
    /// * `toolchain_path` - Path to toolchain binaries
    /// * `api_level` - Android API level
    ///   Sets aarch64_toolchain
    fn set_aarch64_toolchain(toolchain_path: &std::path::Path, api_level: &str) {
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
            "RANLIB_aarch64_linux_android",
            toolchain_path.join("llvm-ranlib"),
        );
    }

    /// Sets up armv7 toolchain environment variables
    ///
    /// # Arguments
    /// * `toolchain_path` - Path to toolchain binaries
    /// * `api_level` - Android API level
    ///   Sets armv7_toolchain
    fn set_armv7_toolchain(toolchain_path: &std::path::Path, api_level: &str) {
        env::set_var(
            "CC_armv7_linux_androideabi",
            toolchain_path.join(format!("armv7a-linux-androideabi{api_level}-clang")),
        );
        env::set_var(
            "CXX_armv7_linux_androideabi",
            toolchain_path.join(format!("armv7a-linux-androideabi{api_level}-clang++")),
        );
        env::set_var("AR_armv7_linux_androideabi", toolchain_path.join("llvm-ar"));
        env::set_var(
            "RANLIB_armv7_linux_androideabi",
            toolchain_path.join("llvm-ranlib"),
        );
    }

    /// Builds the Android library
    ///
    ///
    /// # Arguments
    /// * `release` - Whether to build in release mode
    /// * `target` - Target architecture
    ///
    /// # Returns
    /// `Ok(())` if library build succeeds
    ///
    /// # Errors
    /// Returns error if library compilation fails
    /// Builds android_library
    async fn build_android_library(&self, release: bool, target: &str) -> AnyhowResult<()> {
        let mut cmd = Command::new("cargo");
        cmd.args(["build", "--lib", "--target", target])
            .current_dir(&self.project_root)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if release {
            cmd.arg("--release");
        }

        let output = cmd.output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BearDogError::system(format!("Library build failed: {stderr}")).into());
        }

        info!("✅ Android library built successfully");
        Ok(())
    }

    /// Builds the Android example application
    ///
    /// Compiles the example Android application using the built library.
    ///
    /// # Arguments
    /// * `release` - Whether to build in release mode
    /// * `target` - Target architecture
    ///
    /// # Returns
    /// `Ok(())` if example app build succeeds
    ///
    /// # Errors
    /// Returns error if example app compilation fails
    /// Builds example_app
    async fn build_example_app(&self, release: bool, target: &str) -> AnyhowResult<()> {
        let android_dir = self.project_root.join("android");

        let mut cmd = Command::new("cargo");
        cmd.args(["build", "--target", target])
            .current_dir(&android_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if release {
            cmd.arg("--release");
        }

        let output = cmd.output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BearDogError::system(format!("Example app build failed: {stderr}")).into());
        }

        info!("✅ Android example app built successfully");
        Ok(())
    }
}
