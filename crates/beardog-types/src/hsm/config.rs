// SPDX-License-Identifier: AGPL-3.0-only

//! HSM configuration types
//!
//! Provides production-ready configuration types for HSM operations,
//! replacing temporary stub implementations.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Database configuration for HSM key storage and audit logs
///
/// Supports multiple database backends with connection pooling and encryption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database connection URL
    /// Format: `postgresql://user:pass@host:port/db` or `sqlite:///path/to/db.sqlite`
    pub url: String,

    /// Maximum number of connections in the pool
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    /// Minimum number of idle connections to maintain
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,

    /// Connection timeout in seconds
    #[serde(default = "default_connection_timeout_secs")]
    pub connection_timeout_secs: u64,

    /// Enable query logging (disable in production for performance)
    #[serde(default)]
    pub enable_query_logging: bool,

    /// Enable connection encryption (TLS/SSL)
    #[serde(default = "default_true")]
    pub enable_encryption: bool,

    /// Maximum query execution time in seconds (prevents long-running queries)
    #[serde(default = "default_max_query_timeout_secs")]
    pub max_query_timeout_secs: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite::memory:".to_string(),
            max_connections: default_max_connections(),
            min_connections: default_min_connections(),
            connection_timeout_secs: default_connection_timeout_secs(),
            enable_query_logging: false,
            enable_encryption: true,
            max_query_timeout_secs: default_max_query_timeout_secs(),
        }
    }
}

impl DatabaseConfig {
    /// Creates a PostgreSQL configuration
    pub fn postgres(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            max_connections: 50,
            min_connections: 5,
            connection_timeout_secs: 30,
            enable_query_logging: false,
            enable_encryption: true,
            max_query_timeout_secs: 300,
        }
    }

    /// Creates a SQLite configuration
    pub fn sqlite(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let url = format!("sqlite://{}", path.display());
        Self {
            url,
            max_connections: 10,
            min_connections: 1,
            connection_timeout_secs: 10,
            enable_query_logging: false,
            enable_encryption: true,
            max_query_timeout_secs: 60,
        }
    }

    /// Creates an in-memory SQLite configuration (for testing)
    #[must_use]
    pub fn memory() -> Self {
        Self {
            url: "sqlite::memory:".to_string(),
            max_connections: 10,
            min_connections: 1,
            connection_timeout_secs: 5,
            enable_query_logging: true,
            enable_encryption: false,
            max_query_timeout_secs: 30,
        }
    }

    /// Gets connection timeout as Duration
    #[must_use]
    pub fn connection_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout_secs)
    }

    /// Gets max query timeout as Duration
    #[must_use]
    pub fn max_query_timeout(&self) -> Duration {
        Duration::from_secs(self.max_query_timeout_secs)
    }
}

/// Key storage type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyStorageType {
    /// In-memory storage (ephemeral, for testing)
    Memory,
    /// File system storage with encryption
    FileSystem,
    /// Database storage (PostgreSQL/SQLite)
    Database,
    /// Hardware security module
    Hardware,
    /// Cloud key management service
    CloudKms,
}

/// Key store configuration for cryptographic key management
///
/// Provides secure storage with encryption, caching, and backup capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStoreConfig {
    /// Storage backend type
    pub storage_type: KeyStorageType,

    /// Storage path (for filesystem/database backends)
    pub path: PathBuf,

    /// Enable at-rest encryption for keys
    #[serde(default = "default_true")]
    pub encrypted: bool,

    /// Cache size (number of keys to keep in memory)
    #[serde(default = "default_cache_size")]
    pub cache_size: usize,

    /// Enable automatic backup
    #[serde(default = "default_true")]
    pub enable_backup: bool,

    /// Backup interval in seconds
    #[serde(default = "default_backup_interval_secs")]
    pub backup_interval_secs: u64,

    /// Maximum age of keys in days (0 = no expiration)
    #[serde(default)]
    pub max_key_age_days: u32,

    /// Enable key rotation
    #[serde(default = "default_true")]
    pub enable_key_rotation: bool,

    /// Key rotation interval in days
    #[serde(default = "default_rotation_interval_days")]
    pub rotation_interval_days: u32,
}

impl Default for KeyStoreConfig {
    fn default() -> Self {
        Self {
            storage_type: KeyStorageType::FileSystem,
            path: PathBuf::from("/var/lib/beardog/keystore"),
            encrypted: true,
            cache_size: default_cache_size(),
            enable_backup: true,
            backup_interval_secs: default_backup_interval_secs(),
            max_key_age_days: 0,
            enable_key_rotation: true,
            rotation_interval_days: default_rotation_interval_days(),
        }
    }
}

impl KeyStoreConfig {
    /// Creates a memory-based configuration (for testing)
    #[must_use]
    pub fn memory() -> Self {
        Self {
            storage_type: KeyStorageType::Memory,
            path: PathBuf::from("/tmp/keystore"),
            encrypted: false,
            cache_size: 100,
            enable_backup: false,
            backup_interval_secs: 0,
            max_key_age_days: 0,
            enable_key_rotation: false,
            rotation_interval_days: 0,
        }
    }

    /// Creates a filesystem-based configuration
    pub fn filesystem(path: impl Into<PathBuf>) -> Self {
        Self {
            storage_type: KeyStorageType::FileSystem,
            path: path.into(),
            encrypted: true,
            cache_size: 1024,
            enable_backup: true,
            backup_interval_secs: 3600,
            max_key_age_days: 365,
            enable_key_rotation: true,
            rotation_interval_days: 90,
        }
    }

    /// Creates a database-based configuration
    pub fn database(path: impl Into<PathBuf>) -> Self {
        Self {
            storage_type: KeyStorageType::Database,
            path: path.into(),
            encrypted: true,
            cache_size: 2048,
            enable_backup: true,
            backup_interval_secs: 1800,
            max_key_age_days: 365,
            enable_key_rotation: true,
            rotation_interval_days: 90,
        }
    }

    /// Gets backup interval as Duration
    #[must_use]
    pub fn backup_interval(&self) -> Duration {
        Duration::from_secs(self.backup_interval_secs)
    }
}

/// Comprehensive HSM configuration
///
/// Combines database, key store, and provider configurations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// Database configuration for audit logs and metadata
    pub database: DatabaseConfig,

    /// Key store configuration for cryptographic keys
    pub keystore: KeyStoreConfig,

    /// HSM provider type
    pub provider_type: String,

    /// Enable hardware acceleration
    #[serde(default = "default_true")]
    pub enable_hardware_acceleration: bool,

    /// Enable audit logging
    #[serde(default = "default_true")]
    pub enable_audit_logging: bool,
}

impl Default for HsmConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
            keystore: KeyStoreConfig::default(),
            provider_type: "software".to_string(),
            enable_hardware_acceleration: true,
            enable_audit_logging: true,
        }
    }
}

// Default value functions for serde
fn default_max_connections() -> u32 {
    50
}
fn default_min_connections() -> u32 {
    5
}
fn default_connection_timeout_secs() -> u64 {
    30
}
fn default_max_query_timeout_secs() -> u64 {
    300
}
fn default_cache_size() -> usize {
    1024
}
fn default_backup_interval_secs() -> u64 {
    3600
}
fn default_rotation_interval_days() -> u32 {
    90
}
fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert_eq!(config.url, "sqlite::memory:");
        assert_eq!(config.max_connections, 50);
        assert!(config.enable_encryption);
    }

    #[test]
    fn test_database_config_postgres() {
        let config = DatabaseConfig::postgres("postgresql://localhost/beardog");
        assert!(config.url.starts_with("postgresql://"));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.max_connections, 50);
    }

    #[test]
    fn test_keystore_config_default() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let config = KeyStoreConfig::default();
        assert_eq!(config.storage_type, KeyStorageType::FileSystem);
        assert!(config.encrypted);
        assert!(config.enable_backup);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_keystore_config_memory() {
        let config = KeyStoreConfig::memory();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.storage_type, KeyStorageType::Memory);
        assert!(!config.encrypted);
        assert!(!config.enable_backup);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_hsm_config_default() {
        let config = HsmConfig::default();
        assert_eq!(config.provider_type, "software");
        assert!(config.enable_hardware_acceleration);
        assert!(config.enable_audit_logging);
    }

    // Additional comprehensive tests
    #[test]
    fn test_database_config_sqlite() {
        let config = DatabaseConfig::sqlite("/tmp/beardog.db");
        assert!(config.url.starts_with("sqlite://"));
        assert!(config.url.contains("/tmp/beardog.db"));
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 1);
    }

    #[test]
    fn test_database_config_memory() {
        let config = DatabaseConfig::memory();
        assert_eq!(config.url, "sqlite::memory:");
        assert_eq!(config.max_connections, 10);
        assert!(config.enable_query_logging);
        assert!(!config.enable_encryption);
    }

    #[test]
    fn test_database_config_connection_timeout() {
        let config = DatabaseConfig::default();
        let timeout = config.connection_timeout();
        assert_eq!(timeout.as_secs(), 30);
    }

    #[test]
    fn test_database_config_max_query_timeout() {
        let config = DatabaseConfig::default();
        let timeout = config.max_query_timeout();
        assert_eq!(timeout.as_secs(), 300);
    }

    #[test]
    fn test_database_config_clone() {
        let config1 = DatabaseConfig::default();
        let config2 = config1.clone();
        assert_eq!(config1.url, config2.url);
        assert_eq!(config1.max_connections, config2.max_connections);
    }

    #[test]
    fn test_key_storage_type_variants() {
        let types = [
            KeyStorageType::Memory,
            KeyStorageType::FileSystem,
            KeyStorageType::Database,
            KeyStorageType::Hardware,
            KeyStorageType::CloudKms,
        ];
        assert_eq!(types.len(), 5);
    }

    #[test]
    fn test_key_storage_type_equality() {
        assert_eq!(KeyStorageType::Memory, KeyStorageType::Memory);
        assert_ne!(KeyStorageType::Memory, KeyStorageType::FileSystem);
        assert_eq!(KeyStorageType::Hardware, KeyStorageType::Hardware);
    }

    #[test]
    fn test_keystore_config_filesystem() {
        let config = KeyStoreConfig::filesystem("/var/lib/beardog");
        assert_eq!(config.storage_type, KeyStorageType::FileSystem);
        assert!(config.encrypted);
        assert!(config.enable_backup);
        assert_eq!(config.cache_size, 1024);
        assert_eq!(config.backup_interval_secs, 3600);
        assert_eq!(config.max_key_age_days, 365);
        assert_eq!(config.rotation_interval_days, 90);
    }

    #[test]
    fn test_keystore_config_database() {
        let config = KeyStoreConfig::database("/var/lib/beardog/db");
        assert_eq!(config.storage_type, KeyStorageType::Database);
        assert!(config.encrypted);
        assert_eq!(config.cache_size, 2048);
        assert_eq!(config.backup_interval_secs, 1800);
    }

    #[test]
    fn test_keystore_config_backup_interval() {
        let config = KeyStoreConfig::default();
        let interval = config.backup_interval();
        assert_eq!(interval.as_secs(), 3600);
    }

    #[test]
    fn test_keystore_config_clone() {
        let config1 = KeyStoreConfig::default();
        let config2 = config1.clone();
        assert_eq!(config1.storage_type, config2.storage_type);
        assert_eq!(config1.encrypted, config2.encrypted);
    }

    #[test]
    fn test_hsm_config_clone() {
        let config1 = HsmConfig::default();
        let config2 = config1.clone();
        assert_eq!(config1.provider_type, config2.provider_type);
        assert_eq!(
            config1.enable_hardware_acceleration,
            config2.enable_hardware_acceleration
        );
    }

    #[test]
    fn test_hsm_config_custom() {
        let config = HsmConfig {
            database: DatabaseConfig::postgres("postgresql://localhost/hsm"),
            keystore: KeyStoreConfig::filesystem("/secure/keys"),
            provider_type: "hardware".to_string(),
            enable_hardware_acceleration: true,
            enable_audit_logging: true,
        };

        assert_eq!(config.provider_type, "hardware");
        assert!(config.enable_hardware_acceleration);
    }

    // Serialization tests
    #[test]
    fn test_database_config_serialization() {
        let config = DatabaseConfig::default();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());

        let json_str = json.unwrap();
        let deserialized: Result<DatabaseConfig, _> = serde_json::from_str(&json_str);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_keystore_config_serialization() {
        let config = KeyStoreConfig::default();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());

        let json_str = json.unwrap();
        let deserialized: Result<KeyStoreConfig, _> = serde_json::from_str(&json_str);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_hsm_config_serialization() {
        let config = HsmConfig::default();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());

        let json_str = json.unwrap();
        let deserialized: Result<HsmConfig, _> = serde_json::from_str(&json_str);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_key_storage_type_serialization() {
        let storage_type = KeyStorageType::Hardware;
        let json = serde_json::to_string(&storage_type);
        assert!(json.is_ok());

        let json_str = json.unwrap();
        let deserialized: Result<KeyStorageType, _> = serde_json::from_str(&json_str);
        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap(), KeyStorageType::Hardware);
    }

    // Default function tests
    #[test]
    fn test_default_functions() {
        assert_eq!(default_max_connections(), 50);
        assert_eq!(default_min_connections(), 5);
        assert_eq!(default_connection_timeout_secs(), 30);
        assert_eq!(default_max_query_timeout_secs(), 300);
        assert_eq!(default_cache_size(), 1024);
        assert_eq!(default_backup_interval_secs(), 3600);
        assert_eq!(default_rotation_interval_days(), 90);
        assert!(default_true());
    }

    // Integration tests
    #[test]
    fn test_complete_hsm_configuration() {
        let hsm_config = HsmConfig {
            database: DatabaseConfig::postgres("postgresql://localhost:5432/hsm_db"),
            keystore: KeyStoreConfig::filesystem("/var/lib/beardog/keys"),
            provider_type: "yubihsm".to_string(),
            enable_hardware_acceleration: true,
            enable_audit_logging: true,
        };

        assert!(hsm_config.database.url.contains("postgresql"));
        assert_eq!(hsm_config.keystore.storage_type, KeyStorageType::FileSystem);
        assert_eq!(hsm_config.provider_type, "yubihsm");
        assert!(hsm_config.enable_hardware_acceleration);
    }
}
