

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_types::canonical::hsm::{
    HsmConfig, HsmProviderType, ConnectionConfig, SecurityConfig, PerformanceConfig,
    AuthMethod, SoftwareHsmConfig, HsmTierConfig,
    HsmSecurityTier, TamperResistanceLevel, AttestationLevel, AttestationConfig,
    MemoryProtectionLevel, HsmCapabilities, HsmType, KeyStorageType,
    KeyType, Algorithm, CertificationLevel,
};
use beardog_types::canonical::config::production::RetryConfig;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
    /// Whether enable_encryption is enabled
    pub enable_encryption: bool,

    /// Number of pool_size
    pub pool_size: usize,
}
impl Default for MemoryConfig {}

    fn default(MemoryProtectionLevel::Medium, // Use canonical enum value
            enable_encryption: true,
            pool_size: 1024 * 1024, // 1MB
        }
    }

pub struct HardwareHsmConfig {

    /// The connection value
    pub connection: HsmConnectionConfig,


    pub auth_config: UnifiedAuthConfig,


    pub ha_config: Option<HaConfig>,

pub enum SmartphoneHsmConfig {

    /// Represents ios variant
    Ios(String,


    pub secure_enclave_config: SecureEnclaveConfig,


    pub keychain_config: KeychainConfig,

pub struct AndroidHsmConfig {

    /// The manufacturer value
    pub manufacturer: String,

    /// The model value
    pub model: String,


    pub android_version: String,

    /// Optional strongbox version
    pub strongbox_version: Option<String>,

    /// The strongbox implementation value
    pub strongbox_implementation: StrongBoxImplementation,


    pub keystore_config: KeystoreConfig,


    pub attestation_config: AttestationConfig,

#[derive(Debug, Clone)]
    /// Represents remote variant
    Remote {

        url: String,

        auth_token: String,

pub enum AuditEvent {


    /// Represents key generation variant
    KeyGeneration,


    /// Represents key usage variant
    KeyUsage,


    /// Represents key deletion variant
    KeyDeletion,


    /// Represents authentication variant
    Authentication,


    /// Represents configuration change variant
    ConfigurationChange,


    /// Represents all variant
    All,

pub struct SecureEnclaveConfig {

    /// Whether enable_biometric_auth is enabled
    pub enable_biometric_auth: bool,

    /// Whether require_user_presence is enabled
    pub require_user_presence: bool,

pub struct KeychainConfig {

    /// Optional access group
    pub access_group: Option<String>,

    /// The accessibility value
    pub accessibility: KeychainAccessibility,

    /// Whether sync_with_icloud is enabled
    pub sync_with_icloud: bool,

pub enum KeychainAccessibility {


    /// State indicating whenunlocked
    WhenUnlocked,


    /// Represents when unlocked this device only variant
    WhenUnlockedThisDeviceOnly,


    /// Represents after first unlock variant
    AfterFirstUnlock,


    /// Represents after first unlock this device only variant
    AfterFirstUnlockThisDeviceOnly,


    /// Represents when passcode set this device only variant
    WhenPasscodeSetThisDeviceOnly,

pub struct KeystoreConfig {

    /// The alias prefix value
    pub alias_prefix: String,

    /// Whether require_user_authentication is enabled
    pub require_user_authentication: bool,


    pub user_authentication_validity_duration: Option<u64>,

    /// Whether require_strongbox is enabled
    pub require_strongbox: bool,

pub struct KeyStoreConfig {

    /// The storage type value
    pub storage_type: KeyStorageType,

    /// The encryption key source value
    pub encryption_key_source: KeySource,

    /// Whether backup is enabled
    pub backup_enabled: bool,

    /// Number of cache_size
    pub cache_size: usize,


    pub file_config: Option<FileStorageConfig>,


    pub db_config: Option<DatabaseConfig>,

pub enum KeySource {


    /// State indicating derived
    Derived,


    /// Represents hardware variant
    Hardware,

    /// Represents environment variant
    Environment(String,

    /// Number of file_permissions
    pub file_permissions: u32,

    /// Optional backup path
    pub backup_path: Option<String>,

impl Default for AttestationConfig {
            require_hardware_attestation: false,
            accepted_attestation_levels: vec!["software".to_string()],
            attestation_timeout: Duration::from_secs(30),

impl Default for AndroidHsmConfig {
            manufacturer: "Google".to_string(),
            model: "Pixel".to_string(),
            android_version: "13".to_string(),
            strongbox_version: Some("1.0".to_string()),
            strongbox_implementation: StrongBoxImplementation::TitanM {
                version: "1.0".to_string(),
                security_level: "StrongBox".to_string(),
            },
            },
            },
            keystore_config: KeystoreConfig::default(),
            attestation_config: AttestationConfig::default(KeyStorageType::EncryptedFile,
            encryption_key_source: KeySource::Derived,
            backup_enabled: false,
            cache_size: 1000,
            file_config: Some(FileStorageConfig::default(None,}

impl Default for FileStorageConfig {
            storage_path: "/tmp/hsm_keys".to_string(0o600,
            backup_path: None,
impl Default for KeystoreConfig {
            alias_prefix: "beardog_".to_string(),
