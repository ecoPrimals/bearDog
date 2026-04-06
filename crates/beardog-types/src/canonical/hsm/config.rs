// SPDX-License-Identifier: AGPL-3.0-or-later

// Hardware Security Module configuration types
// Provides structured configuration definitions for HSM providers and settings

// Expect deprecated warnings in this module - LegacyHsmProviderType is intentionally kept for backward compatibility
#![expect(
    deprecated,
    reason = "legacy HSM provider types retained for backward compatibility during unified migration"
)]

use crate::constants::time;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

/// HSM provider types
/// HSM Provider Type - DEPRECATED, USE UNIFIED VERSION
///
/// ⚠️ **DEPRECATED**: This enum has been superseded by the capability-based version
/// in `hsm_unified::providers::HsmProviderType`.
///
/// **Migration Guide**:
/// ```rust,ignore
/// // Old (deprecated):
/// use beardog_types::canonical::hsm::config::LegacyHsmProviderType;
///
/// // New (recommended):
/// use beardog_types::canonical::hsm_unified::providers::HsmProviderType;
/// ```
///
/// The new version includes:
/// - Capability discovery support
/// - Security capability tracking
/// - Universal provider support
/// - Modern patterns
#[deprecated(
    since = "4.0.0",
    note = "Use hsm_unified::providers::HsmProviderType instead. See migration guide above."
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of hsm provider
#[derive(Default)]
pub enum LegacyHsmProviderType {
    /// Software-based HSM implementation
    #[default]
    Software,
    /// Hardware-based HSM
    Hardware,
    /// Network-attached HSM
    Network,
    /// Cloud HSM service
    Cloud,
    /// Mobile platform HSM (iOS Secure Enclave, Android `StrongBox`)
    Mobile,
    /// Custom HSM provider
    Custom {
        /// Custom HSM provider name
        name: String,
    },
}

#[expect(
    deprecated,
    reason = "Display for deprecated LegacyHsmProviderType kept for migration and diagnostics"
)]
impl fmt::Display for LegacyHsmProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Software => write!(f, "Software"),
            Self::Hardware => write!(f, "Hardware"),
            Self::Network => write!(f, "Network"),
            Self::Cloud => write!(f, "Cloud"),
            Self::Mobile => write!(f, "Mobile"),
            Self::Custom { name } => write!(f, "Custom({name})"),
        }
    }
}

// Re-export the modern, unified version as the canonical type
pub use crate::canonical::hsm_unified::providers::HsmProviderType;

/// HSM connection configuration
/// `ConnectionConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    /// Connection timeout in milliseconds
    pub timeout_ms: u32,
    /// Maximum retry attempts
    /// Number of `max_retries`
    pub max_retries: u32,
    /// Retry delay in milliseconds
    /// Number of `retry_delay_ms`
    pub retry_delay_ms: u32,
    /// Keep-alive interval in seconds
    /// Optional keep alive seconds
    pub keep_alive_seconds: Option<u32>,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            timeout_ms: std::env::var("BEARDOG_HSM_CONNECTION_TIMEOUT_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30_000),
            max_retries: std::env::var("BEARDOG_HSM_CONNECTION_MAX_RETRIES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            retry_delay_ms: std::env::var("BEARDOG_HSM_CONNECTION_RETRY_DELAY_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000),
            keep_alive_seconds: std::env::var("BEARDOG_HSM_KEEP_ALIVE_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .or(Some(300)),
        }
    }
}

/// HSM security configuration
/// `SecurityConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable strict security mode
    /// Whether `strict_mode` is enabled
    pub strict_mode: bool,
    /// Key validation level
    pub key_validation: String,
    /// Session timeout
    pub session_timeout: Duration,
    /// Enable audit logging
    /// Whether `audit_logging` is enabled
    pub audit_logging: bool,
    /// Access control policies
    /// Collection of access policies
    pub access_policies: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            strict_mode: true,
            key_validation: "strict".to_string(),
            session_timeout: Duration::from_secs(
                std::env::var("BEARDOG_HSM_SESSION_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(time::SECONDS_PER_HOUR),
            ),
            audit_logging: true,
            access_policies: vec![
                "authenticated_access".to_string(),
                "role_based_control".to_string(),
            ],
        }
    }
}

/// `AuthMethod`
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum AuthMethod {
    /// No authentication (testing only)
    /// None
    #[default]
    None,
    /// Password-based authentication
    Password {
        /// Authentication password
        password: String,
    },
    /// Certificate-based authentication
    Certificate {
        /// Path to certificate file
        cert_path: String,
        /// Path to private key file
        key_path: String,
    },
    /// Token-based authentication
    Token {
        /// Unique token identifier
        token_id: u32,
        /// Optional user PIN or secondary secret presented with the token handle.
        pin: Option<String>,
    },
    /// Biometric authentication
    Biometric {
        /// Biometric authentication method (fingerprint, face, etc.)
        method: String,
    },
}

/// Base HSM configuration
/// `HsmConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// HSM provider type
    pub provider: HsmProviderType,
    /// Connection configuration
    /// The connection value
    pub connection: ConnectionConfig,
    /// Security configuration
    /// The security value
    pub security: SecurityConfig,
    /// Authentication method
    /// The auth method value
    pub auth_method: AuthMethod,
    /// Operation timeout
    pub operation_timeout: Duration,
    /// Key cache size
    /// Optional cache size
    pub cache_size: Option<u32>,
    /// Custom configuration parameters
    /// Mapping of custom params
    pub custom_params: HashMap<String, String>,
}

impl Default for HsmConfig {
    fn default() -> Self {
        Self {
            provider: HsmProviderType::default(),
            connection: ConnectionConfig::default(),
            security: SecurityConfig::default(),
            auth_method: AuthMethod::default(),
            operation_timeout: Duration::from_secs(
                std::env::var("BEARDOG_HSM_OPERATION_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            cache_size: Some(100),
            custom_params: HashMap::new(),
        }
    }
}

/// Software HSM specific configuration
/// `SoftwareHsmConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareHsmConfig {
    /// Base HSM configuration
    /// The base value
    pub base: HsmConfig,
    /// The storage path value
    pub storage_path: String,
    /// Enable memory protection
    /// Whether `memory_protection` is enabled
    pub memory_protection: bool,
    /// Key encryption key
    /// Optional kek
    pub kek: Option<String>,
}

impl Default for SoftwareHsmConfig {
    fn default() -> Self {
        use crate::constants::domains::system::defaults::DEFAULT_TEMP_DIR;

        Self {
            base: HsmConfig::default(),
            storage_path: std::env::var("BEARDOG_HSM_STORAGE_PATH")
                .unwrap_or_else(|_| format!("{DEFAULT_TEMP_DIR}/hsm")),
            kek: None,
            memory_protection: true,
        }
    }
}

/// Hardware HSM specific configuration
/// `HardwareHsmConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareHsmConfig {
    /// Base HSM configuration
    /// The base value
    pub base: HsmConfig,
    /// Device path or identifier
    /// The device path value
    pub device_path: String,
    /// Hardware-specific settings
    /// Mapping of hardware settings
    pub hardware_settings: HashMap<String, String>,
    /// Firmware version requirement
    /// Optional min firmware version
    pub min_firmware_version: Option<String>,
    /// Enable hardware attestation
    /// Whether `hardware_attestation` is enabled
    pub hardware_attestation: bool,
}

impl Default for HardwareHsmConfig {
    fn default() -> Self {
        let mut settings = HashMap::new();
        settings.insert("slot_id".to_string(), "0".to_string());

        Self {
            base: HsmConfig::default(),
            device_path: "/dev/hsm0".to_string(),
            hardware_settings: settings,
            min_firmware_version: None,
            hardware_attestation: true,
        }
    }
}

/// Network HSM specific configuration
/// `NetworkHsmConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkHsmConfig {
    /// Base HSM configuration
    /// The base value
    pub base: HsmConfig,
    /// HSM server address
    /// The server address value
    pub server_address: String,
    /// Server port
    /// Number of port
    pub port: u16,
    /// Use TLS/SSL
    /// Whether `use_tls` is enabled
    pub use_tls: bool,
    /// TLS certificate path
    /// Optional tls cert path
    pub tls_cert_path: Option<String>,
    /// Load balancing configuration
    /// Optional load balancing
    pub load_balancing: Option<LoadBalancingConfig>,
}

impl Default for NetworkHsmConfig {
    fn default() -> Self {
        use crate::canonical::config::network::NetworkConfig;
        let network_config = NetworkConfig::default();

        Self {
            base: HsmConfig::default(),
            server_address: std::env::var("BEARDOG_HSM_SERVER")
                .unwrap_or_else(|_| network_config.default_host.clone()),
            port: std::env::var("BEARDOG_HSM_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(9000),
            use_tls: true,
            tls_cert_path: None,
            load_balancing: None,
        }
    }
}

/// `LoadBalancingConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Load balancing strategy
    /// The strategy value
    pub strategy: LoadBalancingStrategy,
    /// Server pool
    /// Collection of servers
    pub servers: Vec<ServerConfig>,
    /// Health check interval in seconds
    /// Number of `health_check_interval`
    pub health_check_interval: u32,
}

/// Load balancing strategies
/// `LoadBalancingStrategy`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Round-robin selection
    /// `RoundRobin`
    RoundRobin,
    /// Random selection
    /// Random
    Random,
    /// Least connections
    /// `LeastConnections`
    LeastConnections,
    /// Weighted round-robin
    /// `WeightedRoundRobin`
    WeightedRoundRobin,
}

/// `ServerConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server address
    /// The address value
    pub address: String,
    /// Server port
    /// Number of port
    pub port: u16,
    /// Number of weight
    pub weight: u32,
    /// Server enabled
    /// Whether feature is enabled
    pub enabled: bool,
}

/// Cloud HSM specific configuration
/// `CloudHsmConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudHsmConfig {
    /// Base HSM configuration
    /// The base value
    pub base: HsmConfig,
    /// Cloud provider name
    pub provider: String,
    /// Cloud region
    /// The region value
    pub region: String,
    /// Authentication credentials
    /// The credentials value
    pub credentials: CloudCredentials,
    /// Service endpoints
    /// The endpoints value
    pub endpoints: std::collections::HashMap<String, String>,
    /// Configuration options
    /// The options value
    pub options: std::collections::HashMap<String, String>,
}

impl Default for CloudHsmConfig {
    fn default() -> Self {
        Self {
            base: HsmConfig::default(),
            provider: "universal".to_string(),
            region: "us-east-1".to_string(),
            credentials: CloudCredentials::default(),
            endpoints: std::collections::HashMap::new(),
            options: std::collections::HashMap::new(),
        }
    }
}

/// Universal HSM provider (replaces hardcoded cloud providers)
/// `UniversalHsmProvider`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UniversalHsmProvider {
    /// Discovered HSM provider with capability-based identification
    Discovered {
        /// Provider identifier from discovery
        provider_id: String,
        /// Provider capability type
        capability_type: String,
        /// Provider endpoint
        endpoint: String,
    },
    // Legacy hardcoded providers eliminated for sovereignty compliance
    // Use Discovered variant with capability-based identification
}

/// Cloud credentials configuration
/// `CloudCredentials`
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloudCredentials {
    /// Access key ID
    pub access_key_id: String,
    /// Secret access key
    /// The secret access key value
    pub secret_access_key: String,
    /// Session token (optional)
    /// Optional session token
    pub session_token: Option<String>,
    /// Credentials expiration
    /// Optional expires at
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Comprehensive HSM configuration that can handle multiple provider types
/// `HsmProviderConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HsmProviderConfig {
    /// Software HSM configuration
    Software(SoftwareHsmConfig),
    /// Hardware HSM configuration
    Hardware(HardwareHsmConfig),
    /// Network HSM configuration
    Network(NetworkHsmConfig),
    /// Cloud HSM configuration
    Cloud(CloudHsmConfig),
}

impl Default for HsmProviderConfig {
    fn default() -> Self {
        Self::Software(SoftwareHsmConfig::default())
    }
}

impl HsmProviderConfig {
    /// Get the base HSM configuration
    #[must_use]
    pub const fn base_config(&self) -> &HsmConfig {
        match self {
            Self::Software(config) => &config.base,
            Self::Hardware(config) => &config.base,
            Self::Network(config) => &config.base,
            Self::Cloud(config) => &config.base,
        }
    }

    /// Get the base HSM configuration mutably
    /// Returns mutable reference to base config
    pub fn base_config_mut(&mut self) -> &mut HsmConfig {
        match self {
            Self::Software(config) => &mut config.base,
            Self::Hardware(config) => &mut config.base,
            Self::Network(config) => &mut config.base,
            Self::Cloud(config) => &mut config.base,
        }
    }

    /// Get the provider type
    #[must_use]
    pub fn provider_type(&self) -> HsmProviderType {
        self.base_config().provider.clone()
    }

    /// Check if the configuration is valid
    /// Validates input
    ///
    /// # Errors
    ///
    /// Returns an error if timeouts are zero or provider-specific required fields are missing.
    pub fn validate(&self) -> Result<(), String> {
        let base = self.base_config();

        // Validate operation timeout
        if base.operation_timeout.as_secs() == 0 {
            return Err("Operation timeout must be greater than 0".to_string());
        }

        // Validate connection timeout
        if base.connection.timeout_ms == 0 {
            return Err("Connection timeout must be greater than 0".to_string());
        }

        // Provider-specific validation
        match self {
            Self::Software(config) => {
                if config.storage_path.is_empty() {
                    return Err("Storage path cannot be empty for software HSM".to_string());
                }
            }
            Self::Hardware(config) => {
                if config.device_path.is_empty() {
                    return Err("Device path cannot be empty for hardware HSM".to_string());
                }
            }
            Self::Network(config) => {
                if config.server_address.is_empty() {
                    return Err("Server address cannot be empty for network HSM".to_string());
                }
                if config.port == 0 {
                    return Err("Port must be greater than 0 for network HSM".to_string());
                }
            }
            Self::Cloud(config) => {
                if config.region.is_empty() {
                    return Err("Region cannot be empty for cloud HSM".to_string());
                }
                if config.credentials.access_key_id.is_empty() {
                    return Err("Access key cannot be empty for cloud HSM".to_string());
                }
            }
        }

        Ok(())
    }

    /// Get a human-readable description of the configuration
    #[must_use]
    pub fn description(&self) -> String {
        match self {
            Self::Software(config) => {
                format!("Software HSM at {}", config.storage_path)
            }
            Self::Hardware(config) => {
                format!("Hardware HSM at {}", config.device_path)
            }
            Self::Network(config) => {
                format!("Network HSM at {}:{}", config.server_address, config.port)
            }
            Self::Cloud(config) => {
                format!("Cloud HSM ({:?}) in {}", config.provider, config.region)
            }
        }
    }
}

/// Fluent builder for [`HsmProviderConfig`] variants (software, hardware, network, cloud).
pub struct HsmConfigBuilder {
    config: HsmProviderConfig,
}

impl HsmConfigBuilder {
    /// Create a new software HSM configuration builder
    #[must_use]
    pub fn software() -> Self {
        Self {
            config: HsmProviderConfig::Software(SoftwareHsmConfig::default()),
        }
    }

    /// Create a new hardware HSM configuration builder
    #[must_use]
    pub fn hardware() -> Self {
        Self {
            config: HsmProviderConfig::Hardware(HardwareHsmConfig::default()),
        }
    }

    /// Create a new network HSM configuration builder
    #[must_use]
    pub fn network() -> Self {
        Self {
            config: HsmProviderConfig::Network(NetworkHsmConfig::default()),
        }
    }

    /// Create a new cloud HSM configuration builder
    #[must_use]
    pub fn cloud() -> Self {
        Self {
            config: HsmProviderConfig::Cloud(CloudHsmConfig::default()),
        }
    }

    /// Set the operation timeout
    #[must_use]
    pub fn operation_timeout(mut self, timeout: Duration) -> Self {
        self.config.base_config_mut().operation_timeout = timeout;
        self
    }

    /// Set the cache size
    #[must_use]
    pub fn cache_size(mut self, size: u32) -> Self {
        self.config.base_config_mut().cache_size = Some(size);
        self
    }

    /// Set the authentication method
    #[must_use]
    pub fn auth_method(mut self, method: AuthMethod) -> Self {
        self.config.base_config_mut().auth_method = method;
        self
    }

    /// Build the final configuration
    /// Builds component
    ///
    /// # Errors
    ///
    /// Returns an error if [`HsmProviderConfig::validate`] fails.
    pub fn build(self) -> Result<HsmProviderConfig, String> {
        self.config.validate()?;
        Ok(self.config)
    }
}

#[cfg(test)]
#[path = "config_tests.rs"]
mod tests;
