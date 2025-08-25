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


use chrono::{DateTime, Utc};
/// Canonical Android HSM Types
///
/// **UNIFIED ANDROID HSM DEFINITIONS** for the BearDog ecosystem
/// This module provides the canonical definitions for Android StrongBox and related types.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Android StrongBox HSM implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidStrongBoxHsm {
    /// Device information
    pub device_info: AndroidDeviceInfo,
    /// HSM configuration
    pub config: AndroidHsmConfig,
    /// Connection status
    pub connected: bool,
    /// Last health check
    pub last_health_check: Option<DateTime<Utc>>,
    /// Key cache for performance
    pub key_cache: HashMap<String, AndroidCachedKey>,
}
/// **CANONICAL** Android Device Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidDeviceInfo {
    /// Device manufacturer (e.g., "Google", "Samsung")
    pub manufacturer: String,
    /// Device model (e.g., "Pixel 8", "Galaxy S24")
    pub model: String,
    /// Android version
    pub android_version: String,
    /// StrongBox version/implementation
    pub strongbox_version: Option<String>,
    /// Security patch level
    pub security_patch_level: Option<String>,
}

/// Android HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidHsmConfig {
    /// Key alias prefix
    pub alias_prefix: String,
    /// Enable hardware attestation
    pub enable_attestation: bool,
    /// Require user presence for operations
    pub require_user_presence: bool,
    /// Key validity duration (seconds)
    pub key_validity_duration: Option<u64>,
    /// Additional configuration parameters
    pub additional_config: HashMap<String, String>,
}

/// Android Keystore interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidKeystore {
    /// Whether StrongBox is enabled
    pub strongbox_enabled: bool,
    /// Available algorithms
    pub available_algorithms: Vec<String>,
    /// Maximum key size supported
    pub max_key_size: u32,
    /// Whether attestation is supported
    pub supports_attestation: bool,
}

/// Attestation configuration for Android HSM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationConfig {
    /// Whether attestation is required
    pub require_hardware_attestation: bool,
    /// Accepted attestation levels
    pub accepted_attestation_levels: Vec<String>,
    /// Whether attestation is enabled (compatibility)
    pub enabled: bool,
    /// Challenge timeout in seconds (compatibility)
    pub challenge_timeout_secs: u64,
}

/// Attestation result for Android HSM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationResult {
    /// Whether attestation was successful
    pub valid: bool,
    /// Device verified as genuine
    pub device_verified: bool,
    /// Boot state verified
    pub boot_verified: bool,
    /// Application integrity verified
    pub app_verified: bool,
    /// Timestamp of attestation
    pub timestamp: DateTime<Utc>,
    /// Additional details
    pub details: HashMap<String, String>,
}

/// Android Health Monitor for HSM monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidHealthMonitor {
    /// Whether monitoring is enabled
    pub monitoring_enabled: bool,
    /// Health check interval in seconds
    pub check_interval_seconds: u64,
    /// Maximum allowed error rate
    pub max_error_rate: f64,
}

/// Cached key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidCachedKey {
    /// Key identifier
    pub key_id: String,
    /// Key type
    pub key_type: String,
    /// Cached at timestamp
    pub cached_at: DateTime<Utc>,
    /// Last access timestamp
    pub last_access: DateTime<Utc>,
    /// Access count
    pub access_count: u64,
}

// Default implementations
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
            additional_config: HashMap::new(),
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
            key_id: String::new(),
            key_type: "Unknown".to_string(),
            cached_at: Utc::now(),
            last_access: Utc::now(),
            access_count: 0,
        }
    }
}
