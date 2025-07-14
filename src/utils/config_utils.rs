//! Configuration utility functions
//!
//! Provides validation and manipulation of BearDog configuration files.

use crate::config::BearDogConfig;
use crate::error::{BearDogError, BearDogResult};
use std::fs;
use std::path::Path;

/// Validate a configuration file
pub fn validate_config_file(path: &str) -> bool {
    let path = Path::new(path);

    // Check if file exists
    if !path.exists() {
        return false;
    }

    // Check if file is readable
    if !path.is_file() {
        return false;
    }

    // Try to read and parse the configuration
    match fs::read_to_string(path) {
        Ok(content) => {
            // Try to parse as TOML
            match toml::from_str::<BearDogConfig>(&content) {
                Ok(config) => {
                    // Validate the parsed configuration
                    config.validate().is_ok()
                }
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}

/// Load and validate configuration from file
pub fn load_config_file(path: &str) -> BearDogResult<BearDogConfig> {
    let content = fs::read_to_string(path).map_err(|e| {
        BearDogError::config(format!("Failed to read config file '{}': {}", path, e))
    })?;

    let config: BearDogConfig = toml::from_str(&content).map_err(|e| {
        BearDogError::config(format!("Failed to parse config file '{}': {}", path, e))
    })?;

    config.validate()?;
    Ok(config)
}

/// Check if a configuration file has required permissions (Unix only)
#[cfg(unix)]
pub fn check_config_permissions(path: &str) -> BearDogResult<bool> {
    use std::os::unix::fs::PermissionsExt;

    let path = Path::new(path);
    if !path.exists() {
        return Err(BearDogError::not_found(
            "config file",
            path.to_string_lossy().as_ref(),
        ));
    }

    let metadata = fs::metadata(path)
        .map_err(|e| BearDogError::config(format!("Failed to read file metadata: {}", e)))?;

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
        return Err(BearDogError::not_found(
            "config file",
            path.to_string_lossy(),
        ));
    }

    // On Windows, just check if file is readable
    fs::File::open(path)
        .map(|_| true)
        .map_err(|e| BearDogError::config(format!("Cannot read config file: {}", e)))
}

/// Create a default configuration file with secure permissions
pub fn create_default_config(path: &str) -> BearDogResult<()> {
    let config = BearDogConfig::default();
    let toml_content = toml::to_string_pretty(&config)
        .map_err(|e| BearDogError::config(format!("Failed to serialize default config: {}", e)))?;

    // Write the file
    fs::write(path, toml_content)
        .map_err(|e| BearDogError::config(format!("Failed to write config file: {}", e)))?;

    // Set secure permissions (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o600); // Owner read/write only
        fs::set_permissions(path, perms)
            .map_err(|e| BearDogError::config(format!("Failed to set file permissions: {}", e)))?;
    }

    Ok(())
}

/// Backup a configuration file
pub fn backup_config(path: &str) -> BearDogResult<String> {
    let source_path = Path::new(path);
    if !source_path.exists() {
        return Err(BearDogError::not_found("config file", path));
    }

    let backup_path = format!(
        "{}.backup.{}",
        path,
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );

    fs::copy(source_path, &backup_path)
        .map_err(|e| BearDogError::config(format!("Failed to backup config file: {}", e)))?;

    Ok(backup_path)
}

/// Merge two configurations (second config overrides first)
pub fn merge_configs(_base: BearDogConfig, override_config: BearDogConfig) -> BearDogConfig {
    // For now, just return the override config
    // In a more sophisticated implementation, you would merge individual fields
    override_config
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
                "BEARDOG_LOG_LEVEL: '{}' is not a valid log level",
                log_level
            ));
        }
    }

    errors
}

/// Get configuration search paths in order of priority
pub fn get_config_search_paths() -> Vec<String> {
    let mut paths = Vec::new();

    // 1. Current directory
    paths.push("./beardog.toml".to_string());
    paths.push("./config.toml".to_string());

    // 2. User config directory
    if let Some(home) = std::env::var_os("HOME") {
        let home_path = Path::new(&home);
        paths.push(
            home_path
                .join(".config/beardog/config.toml")
                .to_string_lossy()
                .to_string(),
        );
        paths.push(
            home_path
                .join(".beardog.toml")
                .to_string_lossy()
                .to_string(),
        );
    }

    // 3. System config directories
    paths.push("/etc/beardog/config.toml".to_string());
    paths.push("/usr/local/etc/beardog/config.toml".to_string());

    paths
}

/// Find the first valid configuration file in search paths
pub fn find_config_file() -> Option<String> {
    get_config_search_paths()
        .into_iter()
        .find(|path| validate_config_file(path))
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
        assert_eq!(loaded_config.network.host, "127.0.0.1");
        assert_eq!(loaded_config.network.port, 8080);
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
