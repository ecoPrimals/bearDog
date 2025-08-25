// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// HSM Configuration Types - CANONICAL INTEGRATION
///
/// **MODERNIZED** ✅ - This module now uses canonical HSM configuration types directly.
/// 
/// **MIGRATION COMPLETE**: All re-export bridges have been eliminated.
/// Import canonical types directly: `use beardog_types::canonical::hsm::*;`

// ✅ CANONICAL IMPORTS - Direct usage of canonical types
use beardog_types::canonical::hsm::{
    HsmConfig, HsmProviderType, ConnectionConfig, SecurityConfig, PerformanceConfig,
    AuthMethod, SoftwareHsmConfig, HsmTierConfig,
    HsmSecurityTier, TamperResistanceLevel, AttestationLevel, AttestationConfig,
    MemoryProtectionLevel, HsmCapabilities, HsmType, KeyStorageType,
    KeyType, Algorithm, CertificationLevel,
};
use beardog_types::config::network::core::RetryConfig;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
// CANONICAL IMPORT: use beardog_types::config::UnifiedSecurityConfig;
// CANONICAL IMPORT: use beardog_types::config::UnifiedMonitoringConfig;
// CANONICAL IMPORT: use beardog_types::config::UnifiedPerformanceConfig;
// CANONICAL IMPORT: use beardog_types::config::UnifiedDatabaseConfig;
// ✅ MIGRATION BRIDGES ELIMINATED
// Code should import canonical types directly for:
// - Better performance (no re-export overhead)
// - Cleaner dependencies (direct imports)
// - Canonical consistency (same import patterns)
// - Easier maintenance (fewer abstraction layers)
/// Memory configuration for HSM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Memory protection level
    pub protection_level: MemoryProtectionLevel,
    /// Enable memory encryption
    pub enable_encryption: bool,
    /// Memory pool size
    pub pool_size: usize,
}
impl Default for MemoryConfig {}


    fn default() -> Self {
        Self {
            protection_level: MemoryProtectionLevel::Medium, // Use canonical enum value
            enable_encryption: true,
            pool_size: 1024 * 1024, // 1MB
        }
    }
/// Hardware HSM configuration
pub struct HardwareHsmConfig {
    /// HSM connection configuration
    pub connection: HsmConnectionConfig,
    /// Authentication configuration
    pub auth_config: AuthConfig,
    /// High availability configuration
    pub ha_config: Option<HaConfig>,
/// Smartphone HSM configuration
pub enum SmartphoneHsmConfig {
    /// iOS HSM configuration
    Ios(IosHsmConfig),
    /// Android HSM configuration
    Android(AndroidHsmConfig),
/// iOS HSM configuration}


pub struct IosHsmConfig {
    /// iOS version
    pub ios_version: String,
    /// Secure Enclave configuration
    pub secure_enclave_config: SecureEnclaveConfig,
    /// Keychain configuration
    pub keychain_config: KeychainConfig,
/// Android HSM configuration
pub struct AndroidHsmConfig {
    /// Device manufacturer
    pub manufacturer: String,
    /// Device model
    pub model: String,
    /// Android version
    pub android_version: String,
    /// StrongBox version
    pub strongbox_version: Option<String>,
    /// StrongBox implementation
    pub strongbox_implementation: StrongBoxImplementation,
    /// Keystore configuration
    pub keystore_config: KeystoreConfig,
    /// Attestation configuration
    pub attestation_config: AttestationConfig,
// Use beardog_types::canonical::hsm::config for hybrid configurations
/// Security configuration for HSM
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
// MIGRATED: SecurityConfig -> use beardog_types::config::UnifiedSecurityConfig;


// MIGRATED: PerformanceConfig -> use beardog_types::config::UnifiedPerformanceConfig;


pub enum LoadBalancingStrategy {
    /// Round-robin load balancing
    RoundRobin,
    /// Least connections
    LeastConnections,
    /// Weighted round-robin
    WeightedRoundRobin,
    /// Performance-based routing
    PerformanceBased,
/// Health check configuration
// MIGRATED: HealthCheckConfig -> use beardog_types::config::UnifiedMonitoringConfig;


pub enum AuditLogDestination {
    /// Local file
    File,
    /// Syslog server
    Syslog {
        /// Syslog server address
        server: String,
        /// Syslog port
    /// Remote logging service
    Remote {
        /// Remote service URL
        url: String,
        /// Authentication token
        auth_token: String,
/// Audit events
pub enum AuditEvent {
    /// Key generation events
    KeyGeneration,
    /// Key usage events
    KeyUsage,
    /// Key deletion events
    KeyDeletion,
    /// Authentication events
    Authentication,
    /// Configuration changes
    ConfigurationChange,
    /// All events
    All,
/// Secure Enclave configuration for iOS}


pub struct SecureEnclaveConfig {
    /// Enable biometric authentication
    pub enable_biometric_auth: bool,
    /// Require user presence for operations
    pub require_user_presence: bool,
    /// Key attestation configuration
/// Keychain configuration for iOS
pub struct KeychainConfig {
    /// Keychain access group
    pub access_group: Option<String>,
    /// Keychain accessibility level
    pub accessibility: KeychainAccessibility,
    /// Synchronize with iCloud Keychain
    pub sync_with_icloud: bool,
/// Keychain accessibility levels
pub enum KeychainAccessibility {
    /// Accessible when unlocked
    WhenUnlocked,
    /// Accessible when unlocked this device only
    WhenUnlockedThisDeviceOnly,
    /// Accessible after first unlock
    AfterFirstUnlock,
    /// Accessible after first unlock this device only
    AfterFirstUnlockThisDeviceOnly,
    /// Accessible when passcode set this device only
    WhenPasscodeSetThisDeviceOnly,
/// Keystore configuration for Android}


pub struct KeystoreConfig {
    /// Key alias prefix
    pub alias_prefix: String,
    /// Require user authentication
    pub require_user_authentication: bool,
    /// User authentication validity duration in seconds
    pub user_authentication_validity_duration: Option<u64>,
    /// Require StrongBox if available
    pub require_strongbox: bool,
/// Key storage configuration
pub struct KeyStoreConfig {
    /// Storage type
    pub storage_type: KeyStorageType,
    /// Key source for encryption
    pub encryption_key_source: KeySource,
    /// Enable backup
    pub backup_enabled: bool,
    /// Cache size
    pub cache_size: usize,
    /// File storage configuration
    pub file_config: Option<FileStorageConfig>,
    /// Database configuration
    pub db_config: Option<DatabaseConfig>,
/// Key source for encryption
pub enum KeySource {
    /// Derive from password
    Derived,
    /// Hardware-backed key
    Hardware,
    /// Environment variable
    Environment(String),
    /// External key management system
    External(String),
/// File storage configuration}


pub struct FileStorageConfig {
    /// Storage directory path
    pub storage_path: String,
    /// File permissions (octal)
    pub file_permissions: u32,
    /// Backup path
    pub backup_path: Option<String>,
/// Database configuration
// MIGRATED: DatabaseConfig -> use beardog_types::config::UnifiedDatabaseConfig;


impl Default for AttestationConfig {
            require_hardware_attestation: false,
            accepted_attestation_levels: vec!["software".to_string()],
            attestation_timeout: Duration::from_secs(30),
// Removed Default implementation for deprecated LegacySoftwareHsmConfig}


impl Default for AndroidHsmConfig {
            manufacturer: "Google".to_string(),
            model: "Pixel".to_string(),
            android_version: "13".to_string(),
            strongbox_version: Some("1.0".to_string()),
            strongbox_implementation: StrongBoxImplementation::TitanM {
                version: "1.0".to_string(),
                security_level: "StrongBox".to_string(),
            },
            keystore_config: KeystoreConfig::default(),
            attestation_config: AttestationConfig::default(),
impl Default for KeyStoreConfig {
            storage_type: KeyStorageType::EncryptedFile,
            encryption_key_source: KeySource::Derived,
            backup_enabled: false,
            cache_size: 1000,
            file_config: Some(FileStorageConfig::default()),
            db_config: None,}


impl Default for FileStorageConfig {
            storage_path: "/tmp/hsm_keys".to_string(),
            file_permissions: 0o600,
            backup_path: None,
impl Default for KeystoreConfig {
            alias_prefix: "beardog_".to_string(),
            require_user_authentication: false,
            user_authentication_validity_duration: None,
            require_strongbox: false,}


pub struct HsmManagerConfig {
    pub hsm_configs: Vec<String>,
    pub health_config: HealthConfig,
    pub performance_config: PerformanceConfig,
}


pub struct HealthConfig {
    pub check_interval: Duration,}


impl Default for HealthConfig {
            check_interval: Duration::from_secs(60),
            failure_threshold: 3,}


impl Default for FailoverConfig {
            enable_automatic_failover: true,
            failover_timeout: Duration::from_secs(10),
            strategy: FailoverStrategy::Automatic,
            timeout_seconds: 10,
            health_check: HealthCheckConfig {
                interval_seconds: 60,
                timeout_seconds: 10,
                failure_threshold: 3,
                success_threshold: 1,
impl Default for PerformanceConfig {
            max_concurrent_operations: 10,
            operation_timeout: Duration::from_secs(30),
            max_operation_timeout: 30000,
            retry_config: RetryConfig::default(),
            connection_pool_size: 10,
