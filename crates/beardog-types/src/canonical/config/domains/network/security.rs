//! # Network Security Configuration Module
//!
//! This module contains network security configurations including TLS, endpoint security, and DDoS protection.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Network security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityConfiguration {
    /// DDoS protection settings
    pub ddos_protection: DdosProtectionConfiguration,
    /// Enable rate limiting
    pub enable_rate_limiting: bool,
    /// Enable IP filtering
    pub enable_ip_filtering: bool,
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfiguration {
    /// Enable TLS
    pub enabled: bool,
    /// TLS verification mode
    pub verification_mode: TlsVerificationMode,
    /// Certificate path
    pub cert_path: Option<String>,
    /// Private key path
    pub key_path: Option<String>,
}

/// TLS verification modes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TlsVerificationMode {
    /// No verification
    None,
    /// Basic verification
    Basic,
    /// Full verification
    Full,
}

/// Endpoint security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointSecurityConfiguration {
    /// Enable authentication
    pub enable_authentication: bool,
    /// Enable authorization
    pub enable_authorization: bool,
    /// API key requirements
    pub require_api_key: bool,
}

/// DDoS protection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DdosProtectionConfiguration {
    /// Maximum requests per IP per minute
    pub max_requests_per_ip_per_minute: u64,
    /// Enable challenge response
    pub enable_challenge_response: bool,
    /// Block duration minutes
    pub block_duration_minutes: u64,
}

impl Default for NetworkSecurityConfiguration {
    fn default() -> Self {
        Self {
            ddos_protection: DdosProtectionConfiguration::default(),
            enable_rate_limiting: true,
            enable_ip_filtering: false,
        }
    }
}

impl Default for TlsConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            verification_mode: TlsVerificationMode::Basic,
            cert_path: None,
            key_path: None,
        }
    }
}

impl Default for EndpointSecurityConfiguration {
    fn default() -> Self {
        Self {
            enable_authentication: true,
            enable_authorization: true,
            require_api_key: false,
        }
    }
}

impl Default for DdosProtectionConfiguration {
    fn default() -> Self {
        Self {
            max_requests_per_ip_per_minute: 1000,
            enable_challenge_response: false,
            block_duration_minutes: 15,
        }
    }
} 