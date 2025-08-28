

use beardog_types::canonical::hsm::{
    HsmConfig, HsmProviderType, ConnectionConfig, SecurityConfig, PerformanceConfig,
    AuthMethod, SoftwareHsmConfig, HsmTierConfig,
    HsmSecurityTier, TamperResistanceLevel, AttestationLevel, AttestationConfig,
    MemoryProtectionLevel, HsmCapabilities, HsmType, KeyStorageType,
    KeyType, Algorithm, CertificationLevel,
};
use beardog_types::canonical::configuration::production::RetryConfig;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {

    pub protection_level: MemoryProtectionLevel,

    pub enable_encryption: bool,

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

pub struct HardwareHsmConfig {

    pub connection: HsmConnectionConfig,

    pub auth_config: UnifiedAuthConfig,

    pub ha_config: Option<HaConfig>,

pub enum SmartphoneHsmConfig {

    Ios(IosHsmConfig),

    Android(AndroidHsmConfig),

pub struct IosHsmConfig {

    pub ios_version: String,

    pub secure_enclave_config: SecureEnclaveConfig,

    pub keychain_config: KeychainConfig,

pub struct AndroidHsmConfig {

    pub manufacturer: String,

    pub model: String,

    pub android_version: String,

    pub strongbox_version: Option<String>,

    pub strongbox_implementation: StrongBoxImplementation,

    pub keystore_config: KeystoreConfig,

    pub attestation_config: AttestationConfig,

#[derive(Debug, Clone, Serialize, Deserialize, Default)]

pub enum LoadBalancingStrategy {

    RoundRobin,

    LeastConnections,

    WeightedRoundRobin,

    PerformanceBased,

pub enum AuditLogDestination {

    File,

    Syslog {

        server: String,

    Remote {

        url: String,

        auth_token: String,

pub enum AuditEvent {

    KeyGeneration,

    KeyUsage,

    KeyDeletion,

    Authentication,

    ConfigurationChange,

    All,

pub struct SecureEnclaveConfig {

    pub enable_biometric_auth: bool,

    pub require_user_presence: bool,

pub struct KeychainConfig {

    pub access_group: Option<String>,

    pub accessibility: KeychainAccessibility,

    pub sync_with_icloud: bool,

pub enum KeychainAccessibility {

    WhenUnlocked,

    WhenUnlockedThisDeviceOnly,

    AfterFirstUnlock,

    AfterFirstUnlockThisDeviceOnly,

    WhenPasscodeSetThisDeviceOnly,

pub struct KeystoreConfig {

    pub alias_prefix: String,

    pub require_user_authentication: bool,

    pub user_authentication_validity_duration: Option<u64>,

    pub require_strongbox: bool,

pub struct KeyStoreConfig {

    pub storage_type: KeyStorageType,

    pub encryption_key_source: KeySource,

    pub backup_enabled: bool,

    pub cache_size: usize,

    pub file_config: Option<FileStorageConfig>,

    pub db_config: Option<DatabaseConfig>,

pub enum KeySource {

    Derived,

    Hardware,

    Environment(String),

    External(String),

pub struct FileStorageConfig {

    pub storage_path: String,

    pub file_permissions: u32,

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
