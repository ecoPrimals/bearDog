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
pub struct SecurityConfig {
    /// Minimum security level required
    pub min_security_level: u32,
    /// Required certifications
    pub required_certifications: Vec<CertificationLevel>,
    /// Key rotation policy
    pub key_rotation_policy: KeyRotationPolicy,
    /// Audit configuration
    pub audit_config: AuditConfig,
/// Performance configuration for HSM operations}


pub struct PerformanceConfig {
    /// Maximum operation timeout in milliseconds
    pub max_operation_timeout: u64,
    /// Maximum concurrent operations
    pub max_concurrent_operations: u32,
    /// Connection pool size
    pub connection_pool_size: u32,
    /// Retry configuration
    pub retry_config: RetryConfig,
    pub operation_timeout: Duration,
/// Monitoring configuration for HSM operations
pub struct MonitoringConfig {
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Metrics collection interval in seconds
    pub metrics_interval: u64,
    /// Enable health checks
    pub enable_health_checks: bool,
    /// Health check interval in seconds
    pub health_check_interval: u64,
/// Alert thresholds for HSM monitoring
pub struct AlertThresholds {
    /// Maximum error rate before alerting (0.0 to 1.0)
    pub max_error_rate: f64,
    /// Maximum latency in milliseconds before alerting
    pub max_latency_ms: f64,
    /// Minimum availability percentage before alerting
    pub min_availability_percent: f64,
/// HSM connection configuration
pub struct HsmConnectionConfig {
    /// Connection type
    pub connection_type: ConnectionType,
    /// Connection parameters
    pub connection_params: HashMap<String, String>,
    /// Connection timeout in milliseconds
    pub timeout_ms: u64,
    /// Number of retry attempts
    pub retry_attempts: u32,
/// HSM connection types
pub enum ConnectionType {
    /// Network connection
    Network {
        /// Host address
        host: String,
        /// Port number
        port: u16,
        /// Use TLS encryption
        use_tls: bool,
    },
    /// Serial connection
    Serial {
        /// Serial port path
        port_path: String,
        /// Baud rate
        baud_rate: u32,
    /// USB connection
    Usb {
        /// USB vendor ID
        vendor_id: u16,
        /// USB product ID
        product_id: u16,
    /// Local connection (software HSM)
    Local,
/// Authentication configuration for HSM
pub struct AuthConfig {
    /// Authentication method
    pub auth_method: AuthMethod,
    /// Authentication credentials
    pub credentials: AuthCredentials,
    /// Multi-factor authentication configuration
    pub mfa_config: Option<MfaConfig>,
/// Authentication methods
pub enum AuthMethod {
    /// Password-based authentication
    Password,
    /// Certificate-based authentication
    Certificate,
    /// Smart card authentication
    SmartCard,
    /// Biometric authentication
    Biometric,
    /// Multi-factor authentication
    MultiFactor,
/// Authentication credentials}


pub enum AuthCredentials {
    /// Username and password
    Password {
        /// Username
        username: String,
        /// Password (should be encrypted)
        password: String,
    /// Certificate and private key
    Certificate {
        /// Certificate data
        certificate: Vec<u8>,
        /// Private key data
        private_key: Vec<u8>,
    /// Smart card configuration
    SmartCard {
        /// Smart card slot
        slot: u32,
        /// PIN
        pin: String,
    /// Biometric template
    Biometric {
        /// Biometric template data
        template: Vec<u8>,
        /// Biometric type
        biometric_type: String,
/// Multi-factor authentication configuration
/// High availability configuration
pub struct HaConfig {
    /// Cluster configuration
    pub cluster_config: ClusterConfig,
    /// Failover configuration
    pub failover_config: FailoverConfig,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingStrategy,
/// Cluster configuration for HA
pub struct ClusterConfig {
    /// Cluster nodes
    pub nodes: Vec<ClusterNode>,
    /// Quorum size
    pub quorum_size: u32,
    /// Cluster synchronization interval
    pub sync_interval_seconds: u64,
/// Cluster node configuration
pub struct ClusterNode {
    /// Node identifier
    pub node_id: String,
    /// Node address
    pub address: String,
    /// Node port
    pub port: u16,
    /// Node priority
    pub priority: u32,
/// Failover configuration
pub struct FailoverConfig {
    /// Failover strategy
    pub strategy: FailoverStrategy,
    /// Failover timeout in seconds
    pub timeout_seconds: u64,
    /// Health check configuration
    pub health_check: HealthCheckConfig,
    pub enable_automatic_failover: bool,
    pub failover_timeout: Duration,
/// Failover strategies
pub enum FailoverStrategy {
    /// Automatic failover
    Automatic,
    /// Manual failover
    Manual,
    /// Hybrid (automatic with manual override)
    Hybrid,
/// Load balancing strategies}


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
pub struct HealthCheckConfig {
    pub interval_seconds: u64,
    /// Health check timeout in seconds
    /// Number of consecutive failures before marking unhealthy
    pub failure_threshold: u32,
    /// Number of consecutive successes before marking healthy
    pub success_threshold: u32,
/// Key rotation policy
pub struct KeyRotationPolicy {
    /// Enable automatic key rotation
    pub enable_auto_rotation: bool,
    /// Rotation interval in days
    pub rotation_interval_days: u32,
    /// Key overlap period in days
    pub overlap_period_days: u32,
    /// Maximum key age in days
    pub max_key_age_days: u32,
/// Audit configuration
pub struct AuditConfig {
    /// Enable audit logging
    pub enable_audit_logging: bool,
    /// Audit log format
    pub log_format: AuditLogFormat,
    /// Audit log destination
    pub log_destination: AuditLogDestination,
    /// Events to audit
    pub audit_events: Vec<AuditEvent>,
/// Audit log formats
pub enum AuditLogFormat {
    /// JSON format
    #[default]
    Json,
    /// CEF (Common Event Format)
    Cef,
    /// Syslog format
    Syslog,
    /// Custom format
    Custom(String),
/// Audit log destinations}


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
pub struct DatabaseConfig {
    /// Database URL
    pub database_url: String,
    /// Connection timeout in seconds
    pub connection_timeout_seconds: u64,
    /// Enable encryption at rest
    pub enable_encryption_at_rest: bool,
/// Crypto backend types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CryptoBackend {
    /// OpenSSL backend
    OpenSsl,
    /// Ring backend
    Ring,
    /// RustCrypto backend
    RustCrypto,
    /// Hardware backend
    /// Custom backend
// Default implementations}


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
