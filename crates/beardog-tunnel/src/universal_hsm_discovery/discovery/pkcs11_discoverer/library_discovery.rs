// SPDX-License-Identifier: AGPL-3.0-or-later

//! Locating PKCS#11 shared libraries on the host (paths, env, platform scans).

use super::discoverer::Pkcs11Discoverer;
use super::types::PKCS11_COMMON_PATHS;
use beardog_errors::BearDogError;
use std::path::{Path, PathBuf};
use tracing::debug;

impl Pkcs11Discoverer {
    /// Find PKCS#11 libraries on the system
    ///
    /// # Errors
    /// Returns an error if library search fails
    pub(crate) async fn find_pkcs11_libraries(&self) -> Result<Vec<PathBuf>, BearDogError> {
        let mut libraries = Vec::new();

        // Check common paths
        for path_str in PKCS11_COMMON_PATHS {
            let path = Path::new(path_str);
            if path.exists() {
                debug!("Found PKCS#11 library: {:?}", path);
                libraries.push(path.to_path_buf());
            }
        }

        // Check custom paths
        for path in &self.custom_library_paths {
            if path.exists() {
                debug!("Found custom PKCS#11 library: {:?}", path);
                libraries.push(path.clone());
            }
        }

        // Check environment variable
        if let Ok(p11_lib) = beardog_errors::process_env::var("PKCS11_MODULE") {
            let path = PathBuf::from(p11_lib);
            if path.exists() {
                debug!("Found PKCS#11 library from env: {:?}", path);
                libraries.push(path);
            }
        }

        // Platform-specific discovery
        #[cfg(target_os = "macos")]
        {
            libraries.extend(self.find_macos_pkcs11_libraries().await?);
        }

        #[cfg(target_os = "windows")]
        {
            libraries.extend(self.find_windows_pkcs11_libraries().await?);
        }

        Ok(libraries)
    }

    /// Find PKCS#11 libraries on macOS
    ///
    /// # Errors
    /// Returns an error if discovery fails
    #[cfg(target_os = "macos")]
    async fn find_macos_pkcs11_libraries(&self) -> Result<Vec<PathBuf>, BearDogError> {
        let mut libraries = Vec::new();

        // Check Homebrew locations
        let homebrew_paths = ["/opt/homebrew/lib", "/usr/local/lib"];

        for base_path in &homebrew_paths {
            if let Ok(entries) = std::fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.contains("pkcs11") && name.ends_with(".dylib") {
                            debug!("Found macOS PKCS#11 library: {:?}", path);
                            libraries.push(path);
                        }
                    }
                }
            }
        }

        Ok(libraries)
    }

    /// Find PKCS#11 libraries on Windows
    ///
    /// # Errors
    /// Returns an error if discovery fails
    #[cfg(target_os = "windows")]
    async fn find_windows_pkcs11_libraries(&self) -> Result<Vec<PathBuf>, BearDogError> {
        let mut libraries = Vec::new();

        // Check common Windows locations
        let windows_paths = [
            r"C:\Program Files\OpenSC Project\OpenSC\pkcs11",
            r"C:\Program Files (x86)\OpenSC Project\OpenSC\pkcs11",
            r"C:\Windows\System32",
        ];

        for base_path in &windows_paths {
            if let Ok(entries) = std::fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.contains("pkcs11") && name.ends_with(".dll") {
                            debug!("Found Windows PKCS#11 library: {:?}", path);
                            libraries.push(path);
                        }
                    }
                }
            }
        }

        Ok(libraries)
    }
}
