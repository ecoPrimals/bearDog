// SPDX-License-Identifier: AGPL-3.0-or-later

// Core Security Types

use serde::{Deserialize, Serialize};

/// Core security configuration settings
///
/// Fundamental security parameters that control encryption, authentication,
/// compliance, and network security policies across the `BearDog` ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityCoreConfig {
    /// Enable security features globally
    /// Whether feature is enabled
    pub enabled: bool,
    /// Whether encryption is enabled
    pub encryption_enabled: bool,
    /// Enable comprehensive audit logging
    /// Whether `audit_logging` is enabled
    pub audit_logging: bool,
    /// Security compliance level (e.g., "FIPS-140-2", "Common Criteria")
    /// The compliance level value
    pub compliance_level: String,
    /// Maximum allowed authentication attempts
    /// Number of `max_auth_attempts`
    pub max_auth_attempts: u32,
    /// Session timeout in seconds
    pub session_timeout_seconds: u64,
}

/// Discrete security posture levels for policy and UI.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SecurityLevel {
    /// Low security level - basic protection
    #[default]
    /// Represents low variant
    Low,
    /// Medium variant
    Medium,
    /// High variant
    High,
    /// Critical variant
    Critical,
}
