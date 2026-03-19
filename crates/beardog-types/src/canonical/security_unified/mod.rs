// SPDX-License-Identifier: AGPL-3.0-only

// Unified Security Configuration

use serde::{Deserialize, Serialize};

/// Core security module
/// Core functionality
/// Core functionality
pub mod core;

///
/// Provides unified security settings and policies that apply across
/// all components and services within the `BearDog` system.
/// This configuration ensures consistent security posture and compliance
/// with organizational security requirements.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalSecurityConfig {
    /// Core security settings
    /// The core value
    pub core: core::SecurityCoreConfig,
    /// Enable security features globally
    /// Whether feature is enabled
    pub enabled: bool,
    /// Whether encryption is enabled
    pub encryption_enabled: bool,
    /// Enable comprehensive audit logging
    /// Whether `audit_logging` is enabled
    pub audit_logging: bool,
}

///
/// while maintaining clear semantic meaning in the codebase.
pub type SecurityConfig = CanonicalSecurityConfig;
