// SPDX-License-Identifier: AGPL-3.0-only

// Hardware HSM Configuration
//
// Consolidates hardware HSM configurations from beardog-tunnel and other locations.

use super::HsmConfigValidation;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **UNIFIED HARDWARE HSM CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedHardwareHsmConfig {
    /// Whether hardware HSM is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// List of available hardware HSM providers
    pub providers: Vec<HardwareHsmProvider>,
    /// Default hardware HSM provider to use
    pub default_provider: Option<String>,
    /// Whether `connection_pooling` is enabled
    pub connection_pooling: bool,
    /// Whether to enable load balancing across hardware HSMs
    /// Whether `load_balancing` is enabled
    pub load_balancing: bool,
}

impl Default for UnifiedHardwareHsmConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            providers: Vec::new(),
            default_provider: None,
            connection_pooling: true,
            load_balancing: true,
        }
    }
}

/// Hardware HSM provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareHsmProvider {
    /// Name of the hardware HSM provider
    /// Name of the item
    pub name: String,
    /// Type of hardware HSM and its specific configuration
    pub provider_type: HardwareHsmType,
    /// The connection value
    pub connection: HsmConnectionConfig,
    /// The authentication value
    pub authentication: HsmAuthConfig,
    /// The security value
    pub security: HsmSecurityConfig,
    pub performance: HsmPerformanceConfig,
    /// Whether this HSM provider is enabled
    /// Whether feature is enabled
    pub enabled: bool,
}

/// Hardware HSM types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of hardware hsm
pub enum HardwareHsmType {
    /// PKCS#11 HSM with library path
    PKCS11 {
        /// Path to the PKCS#11 library
        library_path: String,
    },
    /// Network-attached HSM
    NetworkHsm {
        /// HSM endpoint URL or IP address
        endpoint: String,
        protocol: String,
    },
    /// USB-connected HSM
    UsbHsm { device_path: String },
    /// Smart card HSM
    SmartCard {
        /// Smart card reader name
        reader_name: String,
    },
    Tpm {
        /// TPM version (1.2, 2.0, etc.)
        version: String,
    },
    /// Custom HSM provider
    Custom {
        /// Custom provider name
        provider_name: String,
        /// Custom configuration parameters
        config: HashMap<String, String>,
    },
}

/// HSM connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConnectionConfig {
    /// Connection timeout duration
    pub timeout: Duration,
    /// Maximum number of connection retries
    /// Number of `max_retries`
    pub max_retries: u32,
    /// Number of `pool_size`
    pub pool_size: u32,
    /// Whether `keep_alive` is enabled
    pub keep_alive: bool,
    /// Whether ssl is enabled
    pub ssl_enabled: bool,
    /// Additional connection parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, String>,
}

impl Default for HsmConnectionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(
                std::env::var("BEARDOG_HSM_HARDWARE_CONNECTION_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            max_retries: std::env::var("BEARDOG_HSM_HARDWARE_MAX_RETRIES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            pool_size: std::env::var("BEARDOG_HSM_POOL_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            keep_alive: true,
            ssl_enabled: true,
            parameters: HashMap::new(),
        }
    }
}

/// HSM authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmAuthConfig {
    /// The auth method value
    pub auth_method: AuthenticationMethod,
    /// The credentials value
    pub credentials: AuthenticationCredentials,
    pub session_timeout: Duration,
    /// Whether to automatically login to HSM
    /// Whether `auto_login` is enabled
    pub auto_login: bool,
}

/// Authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    /// Username and password authentication
    UsernamePassword,
    /// X.509 certificate authentication
    Certificate,
    /// API key authentication
    ApiKey,
    /// Hardware token authentication
    HardwareToken,
    /// Biometric authentication
    Biometric,
    /// No authentication required
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationCredentials {
    /// Username and password authentication
    UsernamePassword { username: String, password: String },
    /// Certificate-based authentication
    Certificate {
        /// Path to certificate file
        cert_path: String,
        /// Path to private key file
        key_path: String,
    },
    /// API key authentication
    ApiKey {
        /// API key identifier
        key_id: String,
        /// API key secret
        key_secret: String,
    },
    /// Hardware security token authentication
    HardwareToken {
        token_id: String,
        pin: Option<String>,
    },
    /// Biometric authentication method
    Biometric {
        /// Biometric authentication method (fingerprint, face, etc.)
        method: String,
        payload: Vec<u8>,
    },
    /// None variant
    None,
}

/// HSM security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmSecurityConfig {
    /// Encryption Enabled
    /// Whether encryption is enabled
    pub encryption_enabled: bool,
    /// Key Wrapping
    /// Whether `key_wrapping` is enabled
    pub key_wrapping: bool,
    /// Tamper Detection
    /// Whether `tamper_detection` is enabled
    pub tamper_detection: bool,
    /// Audit Logging
    /// Whether `audit_logging` is enabled
    pub audit_logging: bool,
    /// Access Control
    /// Whether `access_control` is enabled
    pub access_control: bool,
}

impl Default for HsmSecurityConfig {
    fn default() -> Self {
        Self {
            encryption_enabled: true,
            key_wrapping: true,
            tamper_detection: true,
            audit_logging: true,
            access_control: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmPerformanceConfig {
    /// Max Operations Per Second
    /// Optional max operations per second
    pub max_operations_per_second: Option<u32>,
    /// Batch Size
    /// Number of `batch_size`
    pub batch_size: u32,
    /// Concurrent Operations
    /// Number of `concurrent_operations`
    pub concurrent_operations: u32,
    /// Cache Enabled
    /// Whether cache is enabled
    pub cache_enabled: bool,
    /// Cache Size
    /// Number of `cache_size`
    pub cache_size: usize,
}

impl Default for HsmPerformanceConfig {
    fn default() -> Self {
        Self {
            max_operations_per_second: None,
            batch_size: std::env::var("BEARDOG_HSM_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            concurrent_operations: 4,
            cache_enabled: true,
            cache_size: std::env::var("BEARDOG_HSM_CACHE_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000), // 1000 entries default
        }
    }
}

impl HsmConfigValidation for UnifiedHardwareHsmConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled && self.providers.is_empty() {
            return Err(BearDogError::business(
                "At least one hardware HSM provider must be configured when enabled".to_string(),
            ));
        }

        for provider in &self.providers {
            if provider.name.is_empty() {
                return Err(BearDogError::business(
                    "Hardware HSM provider name cannot be empty".to_string(),
                ));
            }

            if provider.connection.timeout.is_zero() {
                return Err(BearDogError::business(
                    "Hardware HSM connection timeout must be greater than zero".to_string(),
                ));
            }

            if provider.connection.pool_size == 0 {
                return Err(BearDogError::business(
                    "Hardware HSM connection pool size must be greater than zero".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
