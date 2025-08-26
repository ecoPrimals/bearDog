

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidStrongBoxHsm {

    pub device_info: AndroidDeviceInfo,

    pub config: AndroidHsmConfig,

    pub connected: bool,

    pub last_health_check: Option<DateTime<Utc>>,

    pub key_cache: HashMap<String, AndroidCachedKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidDeviceInfo {

    pub manufacturer: String,

    pub model: String,

    pub android_version: String,

    pub strongbox_version: Option<String>,

    pub security_patch_level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidHsmConfig {

    pub alias_prefix: String,

    pub enable_attestation: bool,

    pub require_user_presence: bool,

    pub key_validity_duration: Option<u64>,

    pub additional_config: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidKeystore {

    pub strongbox_enabled: bool,

    pub available_algorithms: Vec<String>,

    pub max_key_size: u32,

    pub supports_attestation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationConfig {

    pub require_hardware_attestation: bool,

    pub accepted_attestation_levels: Vec<String>,

    pub enabled: bool,

    pub challenge_timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationResult {

    pub valid: bool,

    pub device_verified: bool,

    pub boot_verified: bool,

    pub app_verified: bool,

    pub timestamp: DateTime<Utc>,

    pub details: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidHealthMonitor {

    pub monitoring_enabled: bool,

    pub check_interval_seconds: u64,

    pub max_error_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidCachedKey {

    pub key_id: String,

    pub key_type: String,

    pub cached_at: DateTime<Utc>,

    pub last_access: DateTime<Utc>,

    pub access_count: u64,
}

impl Default for AndroidDeviceInfo {
    fn default() -> Self {
        Self {
            manufacturer: "Google".to_string(),
            model: "Pixel".to_string(),
            android_version: "13".to_string(),
            strongbox_version: Some("1.0".to_string()),
            security_patch_level: Some("2023-01".to_string()),
        }
    }
}

impl Default for AndroidHsmConfig {
    fn default() -> Self {
        Self {
            alias_prefix: "beardog".to_string(),
            enable_attestation: true,
            require_user_presence: true,
            key_validity_duration: Some(3600),
            additional_config: HashMap::with_capacity(16),
        }
    }
}

impl Default for AttestationConfig {
    fn default() -> Self {
        Self {
            require_hardware_attestation: false,
            accepted_attestation_levels: vec!["software".to_string()],
            enabled: false,
            challenge_timeout_secs: 30,
        }
    }
}

impl Default for AndroidCachedKey {
    fn default() -> Self {
        Self {
            key_id: String::with_capacity(64),
            key_type: "Unknown".to_string(),
            cached_at: Utc::now(),
            last_access: Utc::now(),
            access_count: 0,
        }
    }
}
