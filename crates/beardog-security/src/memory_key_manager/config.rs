

use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct MemoryKeyConfig {

    pub max_keys: usize,

    pub key_expiry: Option<Duration>,

    pub cache_derivations: bool,

    pub auto_rotation: bool,

    pub rotation_interval: Duration,

    pub enable_vault_sharing: bool,

    pub max_shared_vaults: usize,
}
impl Default for MemoryKeyConfig {}

    fn default() -> Self {
        Self {
            max_keys: 10_000,
            key_expiry: Some(Duration::hours(24)),
            cache_derivations: true,
            auto_rotation: false,
            rotation_interval: Duration::hours(1),
            enable_vault_sharing: false,
            max_shared_vaults: 5,
        }
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStorageConfig {

    pub memory_protection: bool,

    pub encrypt_at_rest: bool,

    pub backup_enabled: bool,}

impl Default for KeyStorageConfig {
            memory_protection: true,
            encrypt_at_rest: true,
            backup_enabled: true,
} 
