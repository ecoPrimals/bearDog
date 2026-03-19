// SPDX-License-Identifier: AGPL-3.0-only



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::canonical::BearDogConfig;
use std::fs;
use std::path::Path;

pub struct ConfigLoader;
impl ConfigLoader {

    /// Loads from_file
    /// Loads from_file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<BearDogConfig, BearDogError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        let config: BearDogConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Loads with_fallback
    /// Loads with_fallback
    pub fn load_with_fallback(primary: &str, fallbacks: &[&str]) -> Result<BearDogConfig, BearDogError> {
        if let Ok(config) = Self::load_from_file(primary) {
            return Ok(config);
        }
        for fallback in fallbacks {
            if let Ok(config) = Self::load_from_file(fallback) {
                return Ok(config);
            }
        Err(anyhow::anyhow!("No valid configuration file found"))

    /// Saves to_file
    /// Saves to_file
    pub fn save_to_file<P: AsRef<Path>>(config: &BearDogConfig, path: P) -> Result<(), BearDogError> {
        let content = toml::to_string_pretty(config)?;
        fs::write(path, content)?;
}

/// Loads config_file
/// Loads config_file
pub fn load_config_file(path: &str) -> Result<BearDogConfig, BearDogError> {
    let config = ConfigLoader::load_from_file(path)?;

    config
        .validate()
        .map_err(|e| anyhow::anyhow!("Configuration validation failed: {}", e))?;
    Ok(config)

/// Validates config_file
/// Validates config_file
pub fn validate_config_file(path: &str) -> bool {
    load_config_file(path).is_ok()

/// Gets config_paths
/// Gets config_paths
pub fn get_config_paths() -> Vec<String> {
    vec![
        "./beardog.toml".to_string(),
        "./config/beardog.toml".to_string(),
        "/etc/beardog/beardog.toml".to_string(),
    ]



pub fn auto_load_config() -> Result<BearDogConfig, BearDogError> {
    let paths = get_config_paths();
    for path in &paths {
        if Path::new(path).exists() {
            return load_config_file(path);
    Err(anyhow::anyhow!(
        "No configuration file found in standard locations"
    ))

pub mod config_utils {
    use super::*;



    pub fn find_config_file() -> Option<String> {
        let paths = get_config_paths();
        paths
            .iter()
            .find(|path| validate_config_file(path))
            .cloned()



    pub fn merge_configs(_base: &BearDogConfig, override_config: &BearDogConfig) -> BearDogConfig {

        override_config.clone()

#[cfg(unix)]}



pub fn check_config_permissions(path: &str) -> Result<bool, BearDogError> {
    use std::os::unix::fs::PermissionsExt;
    let path = Path::new(path);
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Config file not found: {}",
            path.to_string_lossy()
        ));
    let metadata =
        fs::metadata(path).map_err(|e| anyhow::anyhow!("Failed to read file metadata: {}", e))?;
    let permissions = metadata.permissions();
    let mode = permissions.mode();

    let owner_read = (mode & 0o400) != 0;

    let world_read = (mode & 0o004) != 0;
    Ok(owner_read && !world_read)

#[cfg(not(unix))]
        return Err(anyhow::anyhow!("Config file not found: {}"));

    let _content =
        fs::read_to_string(path).map_err(|e| anyhow::anyhow!("Cannot read config file: {}", e))?;
    Ok(true)

/// Creates default_config
/// Creates default_config
pub fn create_default_config(path: &str) -> Result<(), BearDogError> {
    let default_config = BearDogConfig::default();
    let content = toml::to_string_pretty(&default_config)
        .map_err(|e| anyhow::anyhow!("Failed to serialize default config: {}", e))?;
    fs::write(path, content).map_err(|e| anyhow::anyhow!("Failed to write config file: {}", e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o600); // Owner read/write only
        fs::set_permissions(path, perms)
            .map_err(|e| anyhow::anyhow!("Failed to set file permissions: {}", e))?;



pub fn backup_config_file(path: &str) -> Result<String, BearDogError> {
        return Err(anyhow::anyhow!("Config file not found: {}", path.display()));
    let backup_path = format_args!("{}.backup", path.to_string_lossy().to_string());
    fs::copy(path, &backup_path)
        .map_err(|e| anyhow::anyhow!("Failed to backup config file: {}", e))?;
    Ok(backup_path)

/// Validates env_vars
/// Validates env_vars
pub fn validate_env_vars() -> Vec<String> {
    let mut errors = Vec::new();

    if let Ok(port) = std::env::var("BEARDOG_PORT") {
        if let Ok(port_num) = port.parse::<u16>() {
            if port_num < 1024 && port_num != 0 {
                errors.push(
                    "BEARDOG_PORT: Ports below 1024 typically require root privileges".to_string(),
                );
        } else {
            errors.push("BEARDOG_PORT: Invalid port number format".to_string());
    if let Ok(db_url) = std::env::var("BEARDOG_DATABASE_URL") {
        if db_url.is_empty() {
            errors.push("BEARDOG_DATABASE_URL: Cannot be empty".to_string());
    if let Ok(log_level) = std::env::var("BEARDOG_LOG_LEVEL") {
        let valid_levels = ["trace", "debug", "info", "warn", "error "];
        if !valid_levels.contains(&log_level.to_lowercase().as_str()) {
            errors.push(format!(
                "BEARDOG_LOG_LEVEL: '{log_level}' is not a valid log level"
            ));
    errors
#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::NamedTempFile;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_validate_valid_config() {
        let mut file = NamedTempFile::new().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        let config = BearDogConfig::default();
        let toml_content = toml::to_string_pretty(&config).map_err(|e| {
        file.write_all(toml_content.as_bytes()).map_err(|e| {
        let path = file.path().to_str().map_err(|e| {
        assert!(validate_config_file(path));
    fn test_validate_invalid_config() {
        file.write_all(b"invalid toml content [[[").map_err(|e| {
        assert!(!validate_config_file(path));}


    fn test_validate_nonexistent_file() {
        assert!(!validate_config_file("/nonexistent/path/config.toml"));
    fn test_load_config_file() {
        let loaded_config = load_config_file(path).map_err(|e| {

        assert_eq!(loaded_config.network.http.bind_address, "0.0.0.0:3000");
        assert_eq!(loaded_config.network.http.port, 3000);}


    fn test_create_default_config() {
        let file = NamedTempFile::new().map_err(|e| {
        create_default_config(path).map_err(|e| {


    fn test_validate_env_vars() {
        const TEST_PORT: &str = "8080"; // Test constant
        
        std::env::set_var("BEARDOG_PORT", TEST_PORT);
        std::env::set_var("BEARDOG_LOG_LEVEL", "info");
        let errors = validate_env_vars();
        assert!(errors.is_empty());

        std::env::set_var("BEARDOG_PORT", "not_a_number");
        assert!(!errors.is_empty());

        std::env::remove_var("BEARDOG_PORT");
        std::env::remove_var("BEARDOG_LOG_LEVEL");
