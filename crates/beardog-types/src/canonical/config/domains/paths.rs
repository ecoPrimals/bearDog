//! Path Configuration Module
//!
//! Centralized path configuration to eliminate hardcoded paths throughout the codebase.
//! This module provides environment-aware defaults and configuration-driven path management.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Centralized path configuration for BearDog
///
/// Provides environment-aware default paths for:
/// - Configuration files
/// - Data storage
/// - Logs
/// - Cache
/// - HSM storage
/// - Temporary files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathConfig {
    /// Base configuration directory
    pub config_dir: PathBuf,
    
    /// Data storage directory
    pub data_dir: PathBuf,
    
    /// Log file directory
    pub log_dir: PathBuf,
    
    /// Cache directory
    pub cache_dir: PathBuf,
    
    /// HSM/keystore directory
    pub keystore_dir: PathBuf,
    
    /// Temporary files directory
    pub temp_dir: PathBuf,
    
    /// Workflow storage directory
    pub workflow_dir: PathBuf,
}

impl Default for PathConfig {
    fn default() -> Self {
        Self::from_environment()
    }
}

impl PathConfig {
    /// Create path configuration from environment variables
    ///
    /// Environment variables (in order of precedence):
    /// - `BEARDOG_CONFIG_DIR` - Configuration directory
    /// - `BEARDOG_DATA_DIR` - Data storage directory
    /// - `BEARDOG_LOG_DIR` - Log directory
    /// - `BEARDOG_CACHE_DIR` - Cache directory
    /// - `BEARDOG_KEYSTORE_DIR` - Keystore directory
    /// - `BEARDOG_TEMP_DIR` - Temporary directory
    /// - `BEARDOG_WORKFLOW_DIR` - Workflow storage directory
    ///
    /// Falls back to platform-appropriate defaults if not set.
    pub fn from_environment() -> Self {
        let base_dir = Self::get_base_dir();
        
        Self {
            config_dir: Self::get_env_path("BEARDOG_CONFIG_DIR")
                .unwrap_or_else(|| Self::default_config_dir(&base_dir)),
            data_dir: Self::get_env_path("BEARDOG_DATA_DIR")
                .unwrap_or_else(|| Self::default_data_dir(&base_dir)),
            log_dir: Self::get_env_path("BEARDOG_LOG_DIR")
                .unwrap_or_else(|| Self::default_log_dir(&base_dir)),
            cache_dir: Self::get_env_path("BEARDOG_CACHE_DIR")
                .unwrap_or_else(|| Self::default_cache_dir(&base_dir)),
            keystore_dir: Self::get_env_path("BEARDOG_KEYSTORE_DIR")
                .unwrap_or_else(|| Self::default_keystore_dir(&base_dir)),
            temp_dir: Self::get_env_path("BEARDOG_TEMP_DIR")
                .unwrap_or_else(|| Self::default_temp_dir()),
            workflow_dir: Self::get_env_path("BEARDOG_WORKFLOW_DIR")
                .unwrap_or_else(|| Self::default_workflow_dir(&base_dir)),
        }
    }
    
    /// Get base directory based on platform and environment
    fn get_base_dir() -> PathBuf {
        if cfg!(test) {
            // Use temp directory for tests
            std::env::temp_dir().join("beardog-test")
        } else if let Ok(base) = std::env::var("BEARDOG_BASE_DIR") {
            PathBuf::from(base)
        } else if cfg!(target_os = "linux") {
            // Linux: Use XDG or /var/lib
            if let Ok(data_home) = std::env::var("XDG_DATA_HOME") {
                PathBuf::from(data_home).join("beardog")
            } else if let Ok(home) = std::env::var("HOME") {
                PathBuf::from(home).join(".local/share/beardog")
            } else {
                PathBuf::from("/var/lib/beardog")
            }
        } else if cfg!(target_os = "macos") {
            // macOS: Use ~/Library/Application Support
            if let Ok(home) = std::env::var("HOME") {
                PathBuf::from(home).join("Library/Application Support/BearDog")
            } else {
                PathBuf::from("/Library/Application Support/BearDog")
            }
        } else if cfg!(target_os = "windows") {
            // Windows: Use %APPDATA%
            if let Ok(appdata) = std::env::var("APPDATA") {
                PathBuf::from(appdata).join("BearDog")
            } else {
                PathBuf::from("C:\\ProgramData\\BearDog")
            }
        } else {
            // Fallback for unknown platforms
            PathBuf::from("./beardog-data")
        }
    }
    
    /// Get path from environment variable
    fn get_env_path(var_name: &str) -> Option<PathBuf> {
        std::env::var(var_name).ok().map(PathBuf::from)
    }
    
    /// Default configuration directory
    fn default_config_dir(base: &Path) -> PathBuf {
        if cfg!(test) {
            base.join("config")
        } else if cfg!(target_os = "linux") {
            PathBuf::from("/etc/beardog")
        } else {
            base.join("config")
        }
    }
    
    /// Default data directory
    fn default_data_dir(base: &Path) -> PathBuf {
        base.join("data")
    }
    
    /// Default log directory
    fn default_log_dir(base: &Path) -> PathBuf {
        if cfg!(test) {
            base.join("logs")
        } else if cfg!(target_os = "linux") {
            PathBuf::from("/var/log/beardog")
        } else {
            base.join("logs")
        }
    }
    
    /// Default cache directory
    fn default_cache_dir(base: &Path) -> PathBuf {
        if cfg!(target_os = "linux") {
            if let Ok(cache_home) = std::env::var("XDG_CACHE_HOME") {
                return PathBuf::from(cache_home).join("beardog");
            }
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home).join(".cache/beardog");
            }
        }
        base.join("cache")
    }
    
    /// Default keystore directory
    fn default_keystore_dir(base: &Path) -> PathBuf {
        base.join("keystore")
    }
    
    /// Default temporary directory
    fn default_temp_dir() -> PathBuf {
        std::env::temp_dir().join("beardog")
    }
    
    /// Default workflow storage directory
    fn default_workflow_dir(base: &Path) -> PathBuf {
        base.join("workflows")
    }
    
    /// Get configuration file path with fallback search
    ///
    /// Searches for configuration files in the following order:
    /// 1. Explicitly provided path
    /// 2. Current directory (./{filename})
    /// 3. ./config/{filename}
    /// 4. ./configs/{filename}
    /// 5. Config directory from environment
    /// 6. Platform-specific config directory
    pub fn find_config_file(&self, filename: &str) -> Option<PathBuf> {
        let search_paths = vec![
            PathBuf::from(format!("./{}", filename)),
            PathBuf::from(format!("./config/{}", filename)),
            PathBuf::from(format!("./configs/{}", filename)),
            self.config_dir.join(filename),
        ];
        
        search_paths.into_iter().find(|p| p.exists())
    }
    
    /// Ensure all directories exist, creating them if necessary
    ///
    /// # Errors
    ///
    /// Returns an error if directory creation fails
    pub fn ensure_directories(&self) -> Result<(), std::io::Error> {
        let dirs = [
            &self.config_dir,
            &self.data_dir,
            &self.log_dir,
            &self.cache_dir,
            &self.keystore_dir,
            &self.temp_dir,
            &self.workflow_dir,
        ];
        
        for dir in &dirs {
            if !dir.exists() {
                std::fs::create_dir_all(dir)?;
            }
        }
        
        Ok(())
    }
    
    /// Get a path relative to the data directory
    pub fn data_path(&self, relative: &str) -> PathBuf {
        self.data_dir.join(relative)
    }
    
    /// Get a path relative to the log directory
    pub fn log_path(&self, relative: &str) -> PathBuf {
        self.log_dir.join(relative)
    }
    
    /// Get a path relative to the cache directory
    pub fn cache_path(&self, relative: &str) -> PathBuf {
        self.cache_dir.join(relative)
    }
    
    /// Get a path relative to the keystore directory
    pub fn keystore_path(&self, relative: &str) -> PathBuf {
        self.keystore_dir.join(relative)
    }
    
    /// Get a path relative to the temporary directory
    pub fn temp_path(&self, relative: &str) -> PathBuf {
        self.temp_dir.join(relative)
    }
    
    /// Get a path relative to the workflow directory
    pub fn workflow_path(&self, relative: &str) -> PathBuf {
        self.workflow_dir.join(relative)
    }
}

/// Global path configuration instance
///
/// This is lazily initialized on first access and cached for the lifetime of the application.
pub fn global_path_config() -> &'static PathConfig {
    use std::sync::OnceLock;
    static PATH_CONFIG: OnceLock<PathConfig> = OnceLock::new();
    PATH_CONFIG.get_or_init(PathConfig::default)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_path_config_default() {
        let config = PathConfig::default();
        
        // All paths should be set
        assert!(!config.config_dir.as_os_str().is_empty());
        assert!(!config.data_dir.as_os_str().is_empty());
        assert!(!config.log_dir.as_os_str().is_empty());
        assert!(!config.cache_dir.as_os_str().is_empty());
        assert!(!config.keystore_dir.as_os_str().is_empty());
        assert!(!config.temp_dir.as_os_str().is_empty());
        assert!(!config.workflow_dir.as_os_str().is_empty());
    }
    
    #[test]
    fn test_relative_path_helpers() {
        let config = PathConfig::default();
        
        let data_path = config.data_path("test.db");
        assert!(data_path.to_string_lossy().ends_with("test.db"));
        
        let log_path = config.log_path("app.log");
        assert!(log_path.to_string_lossy().ends_with("app.log"));
        
        let cache_path = config.cache_path("metrics.cache");
        assert!(cache_path.to_string_lossy().ends_with("metrics.cache"));
    }
    
    #[test]
    fn test_environment_variable_override() {
        std::env::set_var("BEARDOG_CONFIG_DIR", "/custom/config");
        let config = PathConfig::from_environment();
        assert_eq!(config.config_dir, PathBuf::from("/custom/config"));
        std::env::remove_var("BEARDOG_CONFIG_DIR");
    }
    
    #[test]
    fn test_global_path_config() {
        let config1 = global_path_config();
        let config2 = global_path_config();
        
        // Should return the same instance
        assert_eq!(config1.config_dir, config2.config_dir);
    }
    
    #[test]
    fn test_config_serialization() {
        let config = PathConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: PathConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.config_dir, deserialized.config_dir);
        assert_eq!(config.data_dir, deserialized.data_dir);
    }
}

