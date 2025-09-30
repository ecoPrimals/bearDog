//! Platform-specific HSM configurations
//!
//! This module contains configuration structures for different HSM platforms
//! including Android, iOS, software, and `StrongBox` implementations.

use super::security::SecurityLevel;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Android HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidHsmConfig {
    /// Security level
    pub security_level: SecurityLevel,
    /// Enable `StrongBox` if available
    pub enable_strongbox: bool,
    /// Keystore alias prefix
    pub keystore_alias_prefix: String,
    /// Enable biometric authentication
    pub enable_biometric_auth: bool,
    /// Additional Android-specific parameters
    pub android_params: HashMap<String, String>,
}

impl Default for AndroidHsmConfig {
    fn default() -> Self {
        Self {
            security_level: SecurityLevel::HardwareBacked,
            enable_strongbox: true,
            keystore_alias_prefix: "beardog_".to_string(),
            enable_biometric_auth: true,
            android_params: HashMap::new(),
        }
    }
}

/// iOS HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosHsmConfig {
    /// Security level
    pub security_level: SecurityLevel,
    /// Enable Secure Enclave if available
    pub enable_secure_enclave: bool,
    /// Keychain service identifier
    pub keychain_service: String,
    /// Access group for keychain items
    pub access_group: Option<String>,
    /// Enable Touch ID/Face ID authentication
    pub enable_biometric_auth: bool,
    /// Additional iOS-specific parameters
    pub ios_params: HashMap<String, String>,
}

impl Default for IosHsmConfig {
    fn default() -> Self {
        Self {
            security_level: SecurityLevel::HardwareBacked,
            enable_secure_enclave: true,
            keychain_service: "com.ecoprimals.beardog".to_string(),
            access_group: None,
            enable_biometric_auth: true,
            ios_params: HashMap::new(),
        }
    }
}

/// Software HSM configuration for development and testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareHsmConfig {
    /// Security level (always Software for this implementation)
    pub security_level: SecurityLevel,
    /// Key storage directory
    pub key_storage_path: String,
    /// Enable encryption of stored keys
    pub encrypt_stored_keys: bool,
    /// Master password for key encryption
    pub master_password: Option<String>,
    /// Additional software HSM parameters
    pub software_params: HashMap<String, String>,
}

impl Default for SoftwareHsmConfig {
    fn default() -> Self {
        Self {
            security_level: SecurityLevel::Software,
            key_storage_path: "/tmp/beardog_hsm_keys".to_string(),
            encrypt_stored_keys: true,
            master_password: None,
            software_params: HashMap::new(),
        }
    }
}

/// Dedicated `StrongBox` HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrongBoxHsmConfig {
    /// Security level (always `StrongBox`)
    pub security_level: SecurityLevel,
    /// `StrongBox` keystore alias prefix
    pub alias_prefix: String,
    /// Additional `StrongBox` parameters
    pub strongbox_params: HashMap<String, String>,
}

impl Default for StrongBoxHsmConfig {
    fn default() -> Self {
        Self {
            security_level: SecurityLevel::StrongBox,
            alias_prefix: "beardog_strongbox_".to_string(),
            strongbox_params: HashMap::new(),
        }
    }
}

impl AndroidHsmConfig {
    /// Create a new Android HSM config with `StrongBox` enabled
    pub fn with_strongbox() -> Self {
        Self {
            enable_strongbox: true,
            security_level: SecurityLevel::StrongBox,
            ..Self::default()
        }
    }
    
    /// Create a new Android HSM config for testing
    pub fn for_testing() -> Self {
        Self {
            security_level: SecurityLevel::Software,
            enable_strongbox: false,
            enable_biometric_auth: false,
            ..Self::default()
        }
    }
}

impl IosHsmConfig {
    /// Create a new iOS HSM config with Secure Enclave enabled
    pub fn with_secure_enclave() -> Self {
        Self {
            enable_secure_enclave: true,
            security_level: SecurityLevel::HardwareBacked,
            ..Self::default()
        }
    }
    
    /// Create a new iOS HSM config for testing
    pub fn for_testing() -> Self {
        Self {
            security_level: SecurityLevel::Software,
            enable_secure_enclave: false,
            enable_biometric_auth: false,
            ..Self::default()
        }
    }
}

impl SoftwareHsmConfig {
    /// Create a new software HSM config for development
    pub fn for_development() -> Self {
        Self {
            encrypt_stored_keys: false,
            master_password: None,
            ..Self::default()
        }
    }
    
    /// Create a new software HSM config with encryption
    pub fn with_encryption(master_password: String) -> Self {
        Self {
            encrypt_stored_keys: true,
            master_password: Some(master_password),
            ..Self::default()
        }
    }
} 