// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod config;
pub mod events;
pub mod hsm;
pub mod session;

pub use config::*;
pub use events::*;
pub use session::*;

#[cfg(test)]
mod config_tests;

pub use beardog_errors::BearDogError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
}

#[derive(Debug, Clone)]
pub struct BStpConfig {
    /// The security level value
    pub security_level: SecurityLevel,
    pub session_timeout_seconds: u64,
    /// Number of `max_concurrent_sessions`
    pub max_concurrent_sessions: u32,
    /// The key management value
    pub key_management: config::UnifiedProcessorConfig,
}

impl Default for BStpConfig {
    fn default() -> Self {
        Self {
            security_level: SecurityLevel::High,
            session_timeout_seconds: 3600,
            max_concurrent_sessions: 1000,
            key_management: config::UnifiedProcessorConfig::default(),
        }
    }
}

impl BStpConfig {
    /// Create a configuration with maximum security settings
    #[must_use]
    pub const fn maximum_security() -> Self {
        Self {
            security_level: SecurityLevel::Critical,
            session_timeout_seconds: 1800, // 30 minutes
            max_concurrent_sessions: 100,  // Limited for security
            key_management: config::UnifiedProcessorConfig {
                session_key_length: 64, // Longer keys for maximum security
                key_derivation_rounds: 50000,
                key_rotation_interval: std::time::Duration::from_secs(1800), // 30 minutes
                use_hardware_keys: true,
            },
        }
    }

    #[must_use]
    pub const fn competitive_gaming() -> Self {
        Self {
            security_level: SecurityLevel::High,
            session_timeout_seconds: 7200,  // 2 hours
            max_concurrent_sessions: 10000, // High throughput
            key_management: config::UnifiedProcessorConfig {
                session_key_length: 32,
                key_derivation_rounds: 10000,
                key_rotation_interval: std::time::Duration::from_secs(3600), // 1 hour
                use_hardware_keys: true,
            },
        }
    }
}

pub use session::{SecureSession, SessionManager};
