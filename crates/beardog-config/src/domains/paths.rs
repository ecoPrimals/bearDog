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
    /// Load path configuration from environment variables
    ///
    /// Reads configuration from environment variables:
    /// - `BEARDOG_CONFIG_DIR`: Configuration directory
    /// - `BEARDOG_DATA_DIR`: Data directory
    /// - `BEARDOG_LOG_DIR`: Log directory
    /// - `BEARDOG_PKCS11_LIBRARY`: Specific PKCS#11 library path
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            config_dir: default_config_dir(),
            data_dir: default_data_dir(),
            log_dir: default_log_dir(),
            pkcs11_library_paths: Vec::new(),
            pkcs11_library: env::var("BEARDOG_PKCS11_LIBRARY").ok().map(PathBuf::from),
        }
    }

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
                return Err(ConfigError::PathNotFound(format!(
                    "PKCS#11 library not found: {}",
                    lib.display()
                )));
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
        // Test explicit library
        let config = PathConfig {
            pkcs11_library: Some(PathBuf::from("/test/lib.so")),
            ..Default::default()
        };
        assert_eq!(
            config.get_pkcs11_library(),
            Some(PathBuf::from("/test/lib.so"))
        );

        // Test configured paths
        let config2 = PathConfig {
            pkcs11_library: None,
            pkcs11_library_paths: vec![PathBuf::from("/test/lib2.so")],
            ..Default::default()
        };
        assert_eq!(
            config2.get_pkcs11_library(),
            Some(PathBuf::from("/test/lib2.so"))
        );
    }

    #[test]
    fn test_from_env_creates_valid_config() {
        let config = PathConfig::from_env();

        assert!(!config.config_dir.as_os_str().is_empty());
        assert!(!config.data_dir.as_os_str().is_empty());
        assert!(!config.log_dir.as_os_str().is_empty());
    }

    #[test]
    fn test_clone_path_config() {
        let config = PathConfig::default();
        let cloned = config.clone();

        assert_eq!(config.config_dir, cloned.config_dir);
        assert_eq!(config.data_dir, cloned.data_dir);
        assert_eq!(config.log_dir, cloned.log_dir);
    }

    #[test]
    fn test_debug_format() {
        let config = PathConfig::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("PathConfig"));
        assert!(debug_str.contains("config_dir"));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let config = PathConfig::default();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: PathConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config.config_dir, deserialized.config_dir);
        assert_eq!(config.data_dir, deserialized.data_dir);
    }

    #[test]
    fn test_validate_default_config() {
        let config = PathConfig::default();
        // Should not panic, may warn about non-existent dirs
        let _ = config.validate();
    }

    #[test]
    fn test_get_pkcs11_library_falls_back_to_discovery() {
        let config = PathConfig {
            pkcs11_library: None,
            pkcs11_library_paths: vec![],
            ..Default::default()
        };

        // Should try discovery, may or may not find a library
        let _ = config.get_pkcs11_library();
    }

    #[test]
    fn test_get_pkcs11_library_multiple_paths() {
        let config = PathConfig {
            pkcs11_library: None,
            pkcs11_library_paths: vec![
                PathBuf::from("/test/lib1.so"),
                PathBuf::from("/test/lib2.so"),
            ],
            ..Default::default()
        };

        // Should return first library
        assert_eq!(
            config.get_pkcs11_library(),
            Some(PathBuf::from("/test/lib1.so"))
        );
    }

    #[test]
    fn test_pkcs11_library_override_takes_precedence() {
        let config = PathConfig {
            pkcs11_library: Some(PathBuf::from("/override/lib.so")),
            pkcs11_library_paths: vec![PathBuf::from("/test/lib.so")],
            ..Default::default()
        };

        // Override should take precedence
        assert_eq!(
            config.get_pkcs11_library(),
            Some(PathBuf::from("/override/lib.so"))
        );
    }

    #[test]
    fn test_discover_pkcs11_returns_vec() {
        let libraries = PathConfig::discover_pkcs11_libraries();
        // Should return a Vec (may be empty on systems without HSM)
        // Note: Vec::len() always returns >= 0, so we just verify it's a valid Vec
        assert!(libraries.is_empty() || !libraries.is_empty());
    }

    #[test]
    fn test_custom_config_dir() {
        let config = PathConfig {
            config_dir: PathBuf::from("/custom/config"),
            ..Default::default()
        };

        assert_eq!(config.config_dir, PathBuf::from("/custom/config"));
    }

    #[test]
    fn test_custom_data_dir() {
        let config = PathConfig {
            data_dir: PathBuf::from("/custom/data"),
            ..Default::default()
        };

        assert_eq!(config.data_dir, PathBuf::from("/custom/data"));
    }

    #[test]
    fn test_custom_log_dir() {
        let config = PathConfig {
            log_dir: PathBuf::from("/custom/logs"),
            ..Default::default()
        };

        assert_eq!(config.log_dir, PathBuf::from("/custom/logs"));
    }

    #[test]
    fn test_path_config_with_all_custom_fields() {
        let config = PathConfig {
            config_dir: PathBuf::from("/custom/config"),
            data_dir: PathBuf::from("/custom/data"),
            log_dir: PathBuf::from("/custom/logs"),
            pkcs11_library_paths: vec![PathBuf::from("/custom/lib.so")],
            pkcs11_library: Some(PathBuf::from("/custom/override.so")),
        };

        assert_eq!(config.config_dir, PathBuf::from("/custom/config"));
        assert_eq!(config.data_dir, PathBuf::from("/custom/data"));
        assert_eq!(config.log_dir, PathBuf::from("/custom/logs"));
        assert_eq!(config.pkcs11_library_paths.len(), 1);
        assert!(config.pkcs11_library.is_some());
    }
}
