

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedSecretsConfig {
    pub vault: VaultConfig,
    pub encryption: EncryptionConfig,
    pub rotation: RotationConfig,
    pub access: AccessConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    pub enabled: bool,
    pub vault_url: String,
    pub auth_method: String,
    pub timeout: Duration,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            vault_url: "http://localhost:8200".to_string(),
            auth_method: "token".to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub algorithm: String,
    pub key_size: u32,
    pub enable_at_rest: bool,
    pub enable_in_transit: bool,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: "AES-256-GCM".to_string(),
            key_size: 256,
            enable_at_rest: true,
            enable_in_transit: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationConfig {
    pub enabled: bool,
    pub rotation_interval: Duration,
    pub auto_rotation: bool,
    pub backup_old_secrets: bool,
}

impl Default for RotationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rotation_interval: Duration::from_secs(86400 * 30), // 30 days
            auto_rotation: false,
            backup_old_secrets: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessConfig {
    pub audit_access: bool,
    pub access_timeout: Duration,
    pub max_concurrent_access: u32,
    pub require_authentication: bool,
}

impl Default for AccessConfig {
    fn default() -> Self {
        Self {
            audit_access: true,
            access_timeout: Duration::from_secs(300),
            max_concurrent_access: 10,
            require_authentication: true,
        }
    }
}
