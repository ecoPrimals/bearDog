//! Path configuration and discovery

use crate::error::{ConfigError, ConfigResult};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

/// Path configuration with platform-aware discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathConfig {
    /// Configuration directory
    #[serde(default = "default_config_dir")]
    pub config_dir: PathBuf,

    /// Data directory
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,

    /// Log directory
    #[serde(default = "default_log_dir")]
    pub log_dir: PathBuf,

    /// PKCS#11 library paths (auto-discovered if empty)
    #[serde(default)]
    pub pkcs11_library_paths: Vec<PathBuf>,

    /// Specific PKCS#11 library to use (overrides discovery)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkcs11_library: Option<PathBuf>,
}

impl Default for PathConfig {
    fn default() -> Self {
        Self {
            config_dir: default_config_dir(),
            data_dir: default_data_dir(),
            log_dir: default_log_dir(),
            pkcs11_library_paths: Vec::new(),
            pkcs11_library: None,
        }
    }
}

impl PathConfig {
    /// Validate path configuration
    pub fn validate(&self) -> ConfigResult<()> {
        // Check if config_dir exists or can be created
        if !self.config_dir.exists() {
            // In production, we might want to create it, but for validation just warn
            tracing::debug!("Config directory does not exist: {:?}", self.config_dir);
        }

        // Validate PKCS#11 library if specified
        if let Some(ref lib) = self.pkcs11_library {
            if !lib.exists() {
                return Err(ConfigError::PathNotFound(
                    format!("PKCS#11 library not found: {}", lib.display())
                ));
            }
        }

        Ok(())
    }

    /// Discover PKCS#11 libraries on the system
    ///
    /// Searches common locations based on the current platform.
    pub fn discover_pkcs11_libraries() -> Vec<PathBuf> {
        let mut paths = Vec::new();

        match env::consts::OS {
            "linux" => {
                let search_paths = vec![
                    "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so",
                    "/usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so",
                    "/usr/lib/softhsm/libsofthsm2.so",
                    "/usr/local/lib/softhsm/libsofthsm2.so",
                    "/usr/lib/pkcs11/opensc-pkcs11.so",
                ];

                for path_str in search_paths {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        paths.push(path);
                    }
                }
            }
            "macos" => {
                let search_paths = vec![
                    "/usr/local/lib/opensc-pkcs11.so",
                    "/opt/homebrew/lib/opensc-pkcs11.so",
                    "/usr/local/lib/softhsm/libsofthsm2.so",
                    "/opt/homebrew/lib/softhsm/libsofthsm2.so",
                ];

                for path_str in search_paths {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        paths.push(path);
                    }
                }
            }
            "windows" => {
                let search_paths = vec![
                    "C:\\Program Files\\OpenSC Project\\OpenSC\\pkcs11\\opensc-pkcs11.dll",
                    "C:\\SoftHSM2\\lib\\softhsm2.dll",
                ];

                for path_str in search_paths {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        paths.push(path);
                    }
                }
            }
            _ => {
                tracing::warn!("Unknown OS, cannot discover PKCS#11 libraries");
            }
        }

        paths
    }

    /// Get the PKCS#11 library to use
    ///
    /// Returns the explicitly configured library, or discovers available ones.
    pub fn get_pkcs11_library(&self) -> Option<PathBuf> {
        // Explicit library takes precedence
        if let Some(ref lib) = self.pkcs11_library {
            return Some(lib.clone());
        }

        // Then check configured paths
        if !self.pkcs11_library_paths.is_empty() {
            return self.pkcs11_library_paths.first().cloned();
        }

        // Finally, try discovery
        Self::discover_pkcs11_libraries().first().cloned()
    }
}

fn default_config_dir() -> PathBuf {
    env::var("BEARDOG_CONFIG_DIR")
        .ok()
        .map(PathBuf::from)
        .or_else(|| dirs::config_dir().map(|p| p.join("beardog")))
        .unwrap_or_else(|| PathBuf::from("/etc/beardog"))
}

fn default_data_dir() -> PathBuf {
    env::var("BEARDOG_DATA_DIR")
        .ok()
        .map(PathBuf::from)
        .or_else(|| dirs::data_dir().map(|p| p.join("beardog")))
        .unwrap_or_else(|| PathBuf::from("/var/lib/beardog"))
}

fn default_log_dir() -> PathBuf {
    env::var("BEARDOG_LOG_DIR")
        .ok()
        .map(PathBuf::from)
        .or_else(|| dirs::cache_dir().map(|p| p.join("beardog").join("logs")))
        .unwrap_or_else(|| PathBuf::from("/var/log/beardog"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_paths() {
        let config = PathConfig::default();
        assert!(!config.config_dir.as_os_str().is_empty());
        assert!(!config.data_dir.as_os_str().is_empty());
        assert!(!config.log_dir.as_os_str().is_empty());
    }

    #[test]
    fn test_pkcs11_discovery() {
        let libraries = PathConfig::discover_pkcs11_libraries();
        // May or may not find libraries depending on system
        println!("Found {} PKCS#11 libraries", libraries.len());
    }

    #[test]
    fn test_get_pkcs11_library() {
        let mut config = PathConfig::default();

        // Test explicit library
        config.pkcs11_library = Some(PathBuf::from("/test/lib.so"));
        assert_eq!(
            config.get_pkcs11_library(),
            Some(PathBuf::from("/test/lib.so"))
        );

        // Test configured paths
        config.pkcs11_library = None;
        config.pkcs11_library_paths = vec![PathBuf::from("/test/lib2.so")];
        assert_eq!(
            config.get_pkcs11_library(),
            Some(PathBuf::from("/test/lib2.so"))
        );
    }
}

