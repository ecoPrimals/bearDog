//! Configuration utility functions
//!
//! Provides validation and manipulation of BearDog configuration files.

use beardog_config::core::BearDogResult;
use beardog_config::BearDogConfig;

use std::fs;
use std::path::Path;

/// Configuration loading utilities
pub struct ConfigLoader;

impl ConfigLoader {
    /// Load configuration from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> BearDogResult<BearDogConfig> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;

        let config: BearDogConfig = toml::from_str(&content)?;

        Ok(config)
    }

    /// Load configuration from multiple possible locations
    pub fn load_with_fallback(primary: &str, fallbacks: &[&str]) -> BearDogResult<BearDogConfig> {
        if let Ok(config) = Self::load_from_file(primary) {
            return Ok(config);
        }

        for fallback in fallbacks {
            if let Ok(config) = Self::load_from_file(fallback) {
                return Ok(config);
            }
        }

        Err(anyhow::anyhow!("No valid configuration file found"))
    }

    /// Save configuration to a file
    pub fn save_to_file<P: AsRef<Path>>(config: &BearDogConfig, path: P) -> BearDogResult<()> {
        let content = toml::to_string_pretty(config)?;

        fs::write(path, content)?;
        Ok(())
    }
}

/// Load configuration from a file
pub fn load_config_file(path: &str) -> BearDogResult<BearDogConfig> {
    let config = ConfigLoader::load_from_file(path)?;

    // Validate the configuration
    config
        .validate()
        .map_err(|e| anyhow::anyhow!("Configuration validation failed: {}", e))?;

    Ok(config)
}

/// Validate a configuration file
pub fn validate_config_file(path: &str) -> bool {
    load_config_file(path).is_ok()
}

/// Get configuration file paths to search
pub fn get_config_paths() -> Vec<String> {
    vec![
        "./beardog.toml".to_string(),
        "./config/beardog.toml".to_string(),
        "/etc/beardog/beardog.toml".to_string(),
    ]
}

/// Automatically find and load configuration
pub fn auto_load_config() -> BearDogResult<BearDogConfig> {
    let paths = get_config_paths();

    for path in &paths {
        if Path::new(path).exists() {
            return load_config_file(path);
        }
    }

    Err(anyhow::anyhow!(
        "No configuration file found in standard locations"
    ))
}

/// Configuration utilities
#[allow(clippy::module_inception)]
pub mod config_utils {
    use super::*;

    /// Find configuration file in standard locations
    pub fn find_config_file() -> Option<String> {
        let paths = get_config_paths();

        paths
            .iter()
            .find(|path| validate_config_file(path))
            .cloned()
    }

    /// Merge configurations (base + override)
    pub fn merge_configs(_base: &BearDogConfig, override_config: &BearDogConfig) -> BearDogConfig {
        // For now, just return the override config
        // In a full implementation, this would merge the configurations
        override_config.clone()
    }
}

/// Check if a configuration file has required permissions (Unix only)
#[cfg(unix)]
pub fn check_config_permissions(path: &str) -> BearDogResult<bool> {
    use std::os::unix::fs::PermissionsExt;

    let path = Path::new(path);
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Config file not found: {}",
            path.to_string_lossy()
        ));
    }

    let metadata =
        fs::metadata(path).map_err(|e| anyhow::anyhow!("Failed to read file metadata: {}", e))?;

    let permissions = metadata.permissions();
    let mode = permissions.mode();

    // Check if file is readable by owner (at minimum)
    let owner_read = (mode & 0o400) != 0;

    // Check if file is NOT world-readable (security concern)
    let world_read = (mode & 0o004) != 0;

    Ok(owner_read && !world_read)
}

/// Check if a configuration file has required permissions (Windows/other)
#[cfg(not(unix))]
pub fn check_config_permissions(path: &str) -> BearDogResult<bool> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(anyhow::anyhow!("Config file not found: {}"));
    }

    // On Windows, just check if file is readable
    let _content =
        fs::read_to_string(path).map_err(|e| anyhow::anyhow!("Cannot read config file: {}", e))?;

    Ok(true)
}

/// Create a default configuration file
pub fn create_default_config(path: &str) -> BearDogResult<()> {
    let default_config = BearDogConfig::default();
    let content = toml::to_string_pretty(&default_config)
        .map_err(|e| anyhow::anyhow!("Failed to serialize default config: {}", e))?;

    fs::write(path, content).map_err(|e| anyhow::anyhow!("Failed to write config file: {}", e))?;

    // Set proper permissions on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o600); // Owner read/write only
        fs::set_permissions(path, perms)
            .map_err(|e| anyhow::anyhow!("Failed to set file permissions: {}", e))?;
    }

    Ok(())
}

/// Create a backup of a configuration file
pub fn backup_config_file(path: &str) -> BearDogResult<String> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(anyhow::anyhow!("Config file not found: {}", path.display()));
    }

    let backup_path = format!("{}.backup", path.to_string_lossy());
    fs::copy(path, &backup_path)
        .map_err(|e| anyhow::anyhow!("Failed to backup config file: {}", e))?;

    Ok(backup_path)
}

/// Validate configuration environment variables
pub fn validate_env_vars() -> Vec<String> {
    let mut errors = Vec::new();

    // Check for common misconfigurations in environment variables
    if let Ok(port) = std::env::var("BEARDOG_PORT") {
        if let Ok(port_num) = port.parse::<u16>() {
            if port_num < 1024 && port_num != 0 {
                errors.push(
                    "BEARDOG_PORT: Ports below 1024 typically require root privileges".to_string(),
                );
            }
        } else {
            errors.push("BEARDOG_PORT: Invalid port number format".to_string());
        }
    }

    if let Ok(db_url) = std::env::var("BEARDOG_DATABASE_URL") {
        if db_url.is_empty() {
            errors.push("BEARDOG_DATABASE_URL: Cannot be empty".to_string());
        }
    }

    if let Ok(log_level) = std::env::var("BEARDOG_LOG_LEVEL") {
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&log_level.to_lowercase().as_str()) {
            errors.push(format!(
                "BEARDOG_LOG_LEVEL: '{log_level}' is not a valid log level"
            ));
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_validate_valid_config() {
        let mut file = NamedTempFile::new().unwrap();
        let config = BearDogConfig::default();
        let toml_content = toml::to_string_pretty(&config).unwrap();
        file.write_all(toml_content.as_bytes()).unwrap();

        let path = file.path().to_str().unwrap();
        assert!(validate_config_file(path));
    }

    #[test]
    fn test_validate_invalid_config() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"invalid toml content [[[").unwrap();

        let path = file.path().to_str().unwrap();
        assert!(!validate_config_file(path));
    }

    #[test]
    fn test_validate_nonexistent_file() {
        assert!(!validate_config_file("/nonexistent/path/config.toml"));
    }

    #[test]
    fn test_load_config_file() {
        let mut file = NamedTempFile::new().unwrap();
        let config = BearDogConfig::default();
        let toml_content = toml::to_string_pretty(&config).unwrap();
        file.write_all(toml_content.as_bytes()).unwrap();

        let path = file.path().to_str().unwrap();
        let loaded_config = load_config_file(path).unwrap();

        // Basic validation that config was loaded
        assert_eq!(loaded_config.network.http.bind_address, "localhost:8080");
        assert_eq!(loaded_config.network.http.port, 8080);
    }

    #[test]
    fn test_create_default_config() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_str().unwrap();

        create_default_config(path).unwrap();

        // Verify the file was created and is valid
        assert!(validate_config_file(path));
    }

    #[test]
    fn test_validate_env_vars() {
        // Set some test environment variables
        std::env::set_var("BEARDOG_PORT", "8080");
        std::env::set_var("BEARDOG_LOG_LEVEL", "info");

        let errors = validate_env_vars();
        assert!(errors.is_empty());

        // Test invalid port
        std::env::set_var("BEARDOG_PORT", "not_a_number");
        let errors = validate_env_vars();
        assert!(!errors.is_empty());

        // Clean up
        std::env::remove_var("BEARDOG_PORT");
        std::env::remove_var("BEARDOG_LOG_LEVEL");
    }
}
