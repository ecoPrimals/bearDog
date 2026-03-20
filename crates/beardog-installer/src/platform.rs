// SPDX-License-Identifier: AGPL-3.0-only

//! Platform detection and path discovery
//!
//! Zero hardcoded paths - uses XDG Base Directory spec (Linux),
//! macOS conventions, Windows standards, and Android paths.
//!
//! # Philosophy
//! - Zero hardcoding: Capability-based discovery
//! - Platform-agnostic: Works on all supported OS
//! - Standards-compliant: XDG, macOS, Windows conventions
//! - Modern: Async-ready, type-safe

use directories::{BaseDirs, ProjectDirs};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

/// Supported operating systems
///
/// Represents all operating systems supported by biomeOS genomeBins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperatingSystem {
    /// Linux with GNU libc
    Linux,
    /// Linux with musl libc (static linking)
    LinuxMusl,
    /// Android
    Android,
    /// macOS (Darwin)
    MacOS,
    /// Windows
    Windows,
    /// iOS
    Ios,
}

impl OperatingSystem {
    /// Detect OS at compile time
    ///
    /// Uses compile-time cfg attributes for zero runtime overhead.
    ///
    /// # Examples
    /// ```
    /// use beardog_installer::platform::OperatingSystem;
    ///
    /// let os = OperatingSystem::detect().expect("supported platform");
    /// println!("Running on: {:?}", os);
    /// ```
    pub fn detect() -> Result<Self, PlatformError> {
        #[cfg(target_os = "linux")]
        {
            // Check if running on Android
            if Self::is_android() {
                return Ok(Self::Android);
            }

            // Check if using musl libc
            #[cfg(target_env = "musl")]
            return Ok(Self::LinuxMusl);

            #[cfg(not(target_env = "musl"))]
            return Ok(Self::Linux);
        }

        #[cfg(target_os = "macos")]
        return Ok(Self::MacOS);

        #[cfg(target_os = "windows")]
        return Ok(Self::Windows);

        #[cfg(target_os = "ios")]
        return Ok(Self::Ios);

        #[cfg(not(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "windows",
            target_os = "ios"
        )))]
        Err(PlatformError::Unsupported {
            os: std::env::consts::OS.to_string(),
        })
    }

    /// Check if running on Android
    ///
    /// Android is technically Linux, but has different path conventions.
    fn is_android() -> bool {
        #[cfg(target_os = "linux")]
        {
            // Check for Android-specific indicators
            std::path::Path::new("/system/build.prop").exists()
                || beardog_errors::process_env::var("ANDROID_ROOT").is_ok()
                || beardog_errors::process_env::var("ANDROID_DATA").is_ok()
        }

        #[cfg(not(target_os = "linux"))]
        false
    }
}

/// Platform-agnostic paths (XDG-compliant where applicable)
///
/// Discovers standard directories for different platforms:
/// - Linux: XDG Base Directory Specification
/// - macOS: Apple's standard directories
/// - Windows: Windows Known Folders
/// - Android: Android-specific paths
///
/// # Philosophy
/// Zero hardcoding - all paths discovered via platform APIs.
#[derive(Debug, Clone)]
pub struct BiomeOSPaths {
    /// Binary installation directory
    ///
    /// - Linux: `$HOME/.local/bin` or `$XDG_DATA_HOME/biomeos/bin`
    /// - macOS: `$HOME/Library/Application Support/biomeos/bin`
    /// - Windows: `%LOCALAPPDATA%\biomeos\bin`
    /// - Android: `$ANDROID_DATA/data/org.biomeos.nucleus/bin`
    pub bin_dir: PathBuf,

    /// Data directory (persistent storage)
    ///
    /// - Linux: `$XDG_DATA_HOME/biomeos` or `$HOME/.local/share/biomeos`
    /// - macOS: `$HOME/Library/Application Support/biomeos`
    /// - Windows: `%LOCALAPPDATA%\biomeos`
    /// - Android: `$ANDROID_DATA/data/org.biomeos.nucleus`
    pub data_dir: PathBuf,

    /// Configuration directory
    ///
    /// - Linux: `$XDG_CONFIG_HOME/biomeos` or `$HOME/.config/biomeos`
    /// - macOS: `$HOME/Library/Preferences/biomeos`
    /// - Windows: `%APPDATA%\biomeos\config`
    /// - Android: `$ANDROID_DATA/data/org.biomeos.nucleus/config`
    pub config_dir: PathBuf,

    /// Runtime directory (temporary, fast storage)
    ///
    /// - Linux: `$XDG_RUNTIME_DIR/biomeos`
    /// - macOS: `$TMPDIR/biomeos`
    /// - Windows: `%TEMP%\biomeos`
    /// - Android: `/data/local/tmp/biomeos`
    pub runtime_dir: PathBuf,

    /// Cache directory (can be cleared)
    ///
    /// - Linux: `$XDG_CACHE_HOME/biomeos` or `$HOME/.cache/biomeos`
    /// - macOS: `$HOME/Library/Caches/biomeos`
    /// - Windows: `%LOCALAPPDATA%\biomeos\cache`
    /// - Android: `$ANDROID_DATA/data/org.biomeos.nucleus/cache`
    pub cache_dir: PathBuf,
}

impl BiomeOSPaths {
    /// Discover paths using platform standards (zero hardcoding)
    ///
    /// # Examples
    /// ```
    /// use beardog_installer::platform::BiomeOSPaths;
    ///
    /// let paths = BiomeOSPaths::discover().expect("home directory for paths");
    /// println!("Install to: {}", paths.bin_dir.display());
    /// ```
    ///
    /// # Errors
    /// Returns `PlatformError::NoHomeDir` if home directory cannot be determined.
    pub fn discover() -> Result<Self, PlatformError> {
        let project =
            ProjectDirs::from("org", "biomeos", "nucleus").ok_or(PlatformError::NoHomeDir)?;

        let base = BaseDirs::new().ok_or(PlatformError::NoHomeDir)?;

        Ok(Self {
            bin_dir: Self::discover_bin_dir(&project, &base)?,
            data_dir: project.data_dir().to_path_buf(),
            config_dir: project.config_dir().to_path_buf(),
            runtime_dir: Self::discover_runtime_dir()?,
            cache_dir: project.cache_dir().to_path_buf(),
        })
    }

    /// Discover binary installation directory (prefers user-space, no sudo)
    fn discover_bin_dir(project: &ProjectDirs, base: &BaseDirs) -> Result<PathBuf, PlatformError> {
        // 1. Try $HOME/.local/bin (most common, usually in PATH)
        let local_bin = base.home_dir().join(".local").join("bin");
        if local_bin.exists() {
            return Ok(local_bin);
        }

        // 2. Fallback to XDG_DATA_HOME/biomeos/bin
        let xdg_bin = project.data_dir().join("bin");
        Ok(xdg_bin)
    }

    /// Discover runtime directory (fast temporary storage)
    fn discover_runtime_dir() -> Result<PathBuf, PlatformError> {
        // 1. Try XDG_RUNTIME_DIR (Linux, guaranteed fast tmpfs)
        if let Ok(xdg_runtime) = beardog_errors::process_env::var("XDG_RUNTIME_DIR") {
            let runtime_dir = PathBuf::from(xdg_runtime).join("biomeos");
            return Ok(runtime_dir);
        }

        // 2. Try TMPDIR (macOS, Linux fallback)
        if let Ok(tmpdir) = beardog_errors::process_env::var("TMPDIR") {
            return Ok(PathBuf::from(tmpdir).join("biomeos"));
        }

        // 3. Try TEMP (Windows)
        if let Ok(temp) = beardog_errors::process_env::var("TEMP") {
            return Ok(PathBuf::from(temp).join("biomeos"));
        }

        // 4. Android fallback
        #[cfg(target_os = "android")]
        {
            return Ok(PathBuf::from("/data/local/tmp/biomeos"));
        }

        // 5. Final fallback: system temp
        Ok(std::env::temp_dir().join("biomeos"))
    }

    /// Create all directories if they don't exist
    ///
    /// # Examples
    /// ```no_run
    /// # use beardog_installer::platform::BiomeOSPaths;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let paths = BiomeOSPaths::discover()?;
    /// paths.ensure_exists().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ensure_exists(&self) -> Result<(), PlatformError> {
        use tokio::fs;

        for dir in [
            &self.bin_dir,
            &self.data_dir,
            &self.config_dir,
            &self.runtime_dir,
            &self.cache_dir,
        ] {
            fs::create_dir_all(dir)
                .await
                .map_err(|e| PlatformError::IoError {
                    path: dir.clone(),
                    source: e,
                })?;
        }

        Ok(())
    }

    /// All directories as slice
    pub fn all_dirs(&self) -> Vec<&PathBuf> {
        vec![
            &self.bin_dir,
            &self.data_dir,
            &self.config_dir,
            &self.runtime_dir,
            &self.cache_dir,
        ]
    }
}

/// Platform detection errors
#[derive(Debug, Error)]
pub enum PlatformError {
    /// No home directory found
    #[error("No home directory found (required for installation)")]
    NoHomeDir,

    /// Unsupported OS
    #[error("Unsupported operating system: {os}")]
    Unsupported {
        /// OS name from `std::env::consts::OS`
        os: String,
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

    #[test]
    fn test_detect_os() {
        let os = OperatingSystem::detect();
        assert!(os.is_ok(), "Should detect current OS");

        // Verify it's one of the supported OS
        let os = os.expect("OS detection");
        assert!(matches!(
            os,
            OperatingSystem::Linux
                | OperatingSystem::LinuxMusl
                | OperatingSystem::Android
                | OperatingSystem::MacOS
                | OperatingSystem::Windows
                | OperatingSystem::Ios
        ));
    }

    #[test]
    fn test_discover_paths() {
        let paths = BiomeOSPaths::discover().expect("discover paths");

        // Paths should be valid and contain expected patterns
        let bin_str = paths.bin_dir.to_str().expect("bin_dir utf-8");
        assert!(bin_str.contains("biomeos") || bin_str.contains(".local/bin"));

        // data_dir should contain either "biomeos" or "nucleus" (from ProjectDirs)
        let data_str = paths.data_dir.to_str().expect("data_dir utf-8");
        assert!(
            data_str.contains("biomeos") || data_str.contains("nucleus"),
            "data_dir should contain biomeos or nucleus, got: {}",
            data_str
        );

        // config_dir should contain either "biomeos" or "nucleus"
        let config_str = paths.config_dir.to_str().expect("config_dir utf-8");
        assert!(
            config_str.contains("biomeos") || config_str.contains("nucleus"),
            "config_dir should contain biomeos or nucleus, got: {}",
            config_str
        );

        assert!(
            paths
                .runtime_dir
                .to_str()
                .expect("runtime_dir utf-8")
                .contains("biomeos")
        );

        // cache_dir should contain either "biomeos" or "nucleus"
        let cache_str = paths.cache_dir.to_str().expect("cache_dir utf-8");
        assert!(
            cache_str.contains("biomeos") || cache_str.contains("nucleus"),
            "cache_dir should contain biomeos or nucleus, got: {}",
            cache_str
        );
    }

    #[tokio::test]
    async fn test_ensure_paths_exist() {
        use tempfile::TempDir;

        let temp = TempDir::new().expect("tempdir");
        let paths = BiomeOSPaths {
            bin_dir: temp.path().join("bin"),
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            runtime_dir: temp.path().join("runtime"),
            cache_dir: temp.path().join("cache"),
        };

        paths.ensure_exists().await.expect("ensure_exists");

        // All directories should exist
        assert!(paths.bin_dir.exists());
        assert!(paths.data_dir.exists());
        assert!(paths.config_dir.exists());
        assert!(paths.runtime_dir.exists());
        assert!(paths.cache_dir.exists());
    }

    #[test]
    fn test_all_dirs() {
        let paths = BiomeOSPaths::discover().expect("discover paths");
        let all = paths.all_dirs();

        assert_eq!(all.len(), 5);
        assert!(all.contains(&&paths.bin_dir));
        assert!(all.contains(&&paths.data_dir));
    }

    #[test]
    fn test_serialization() {
        let os = OperatingSystem::Linux;
        let json = serde_json::to_string(&os).expect("serialize OS");
        assert_eq!(json, "\"linux\"");

        let deserialized: OperatingSystem = serde_json::from_str(&json).expect("deserialize OS");
        assert_eq!(deserialized, OperatingSystem::Linux);
    }
}
