// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
//! # Tunnel Module - Secure Communication Infrastructure
//!
//! This module provides the core infrastructure for secure tunneling in the BearDog ecosystem.
//! It supports both BTSP (BearDog Tunnel Security Protocol) for primal-to-primal communication
//! and TLS/HTTPS for external API connections.
//!
//! ## Key Features
//!
//! - **Security Levels**: Configurable security from low-latency gaming to critical financial
//! - **Session Management**: Secure session lifecycle with automatic rotation
//! - **HSM Integration**: Hardware Security Module support for key protection
//! - **Performance Tuning**: Configurable for different use cases (gaming, production, etc.)
//!
//! ## Core Components
//!
//! - [`BStpConfig`]: Main BTSP configuration
//! - [`SecurityLevel`]: Security level enumeration
//! - [`SecureSession`]: Session management
//! - [`SessionManager`]: Multi-session coordination
//!
//! ## Example
//!
//! ```rust
//! use beardog_tunnel::tunnel::{BStpConfig, SecurityLevel};
//!
//! // Create a maximum security configuration
//! let config = BStpConfig::maximum_security();
//! assert_eq!(config.security_level, SecurityLevel::Critical);
//!
//! // Or use default high security
//! let default_config = BStpConfig::default();
//! assert_eq!(default_config.security_level, SecurityLevel::High);
//! ```

/// Configuration submodules
pub mod config;
/// Event handling and notifications
pub mod events;
/// Hardware Security Module integration
pub mod hsm;
/// Secure session management
pub mod session;

pub use config::*;
pub use events::*;
pub use session::*;

#[cfg(test)]
mod config_tests;

pub use beardog_errors::BearDogError;

/// Security level for tunnel configuration
///
/// Defines the trade-off between security and performance. Higher security levels
/// use longer keys, more frequent rotation, and stricter validation, at the cost
/// of higher latency and lower throughput.
///
/// # Security vs Performance
///
/// - `Low`: Minimal security for testing or internal development
/// - `Medium`: Balanced security for general-purpose applications
/// - `High` (default): Strong security for production use
/// - `Critical`: Maximum security for sensitive operations (financial, healthcare)
///
/// # Example
///
/// ```rust
/// use beardog_tunnel::tunnel::SecurityLevel;
///
/// // For production, use High or Critical
/// let production = SecurityLevel::High;
///
/// // For low-latency gaming, Medium may be appropriate
/// let gaming = SecurityLevel::Medium;
/// ```
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

/// BTSP (`BearDog` Tunnel Security Protocol) Configuration
///
/// Main configuration struct for secure tunnel setup. Controls security level,
/// session management, and key management policies.
///
/// # Fields
///
/// - `security_level`: Overall security posture (see [`SecurityLevel`])
/// - `session_timeout_seconds`: Maximum session lifetime before forced rotation
/// - `max_concurrent_sessions`: Limit for concurrent active sessions
/// - `key_management`: Unified key management configuration
///
/// # Preset Configurations
///
/// - [`BStpConfig::default()`]: High security for production
/// - [`BStpConfig::maximum_security()`]: Critical security with strict limits
/// - [`BStpConfig::competitive_gaming()`]: Optimized for low-latency gaming
///
/// # Example
///
/// ```rust
/// use beardog_tunnel::tunnel::{BStpConfig, SecurityLevel};
///
/// // Use maximum security preset
/// let config = BStpConfig::maximum_security();
/// assert_eq!(config.session_timeout_seconds, 1800); // 30 minutes
/// assert_eq!(config.max_concurrent_sessions, 100);
///
/// // Or customize from default
/// let mut custom = BStpConfig::default();
/// custom.session_timeout_seconds = 7200; // 2 hours
/// ```
#[derive(Debug, Clone)]
pub struct BStpConfig {
    /// The security level value
    pub security_level: SecurityLevel,
    /// Session timeout in seconds (auto-rotation after this period)
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

    /// Create a configuration optimized for competitive gaming
    ///
    /// Balances security with low latency and high throughput requirements.
    /// Uses longer session timeouts and supports many concurrent connections.
    ///
    /// # Use Case
    ///
    /// - Real-time multiplayer games
    /// - Low-latency communication
    /// - High concurrent user count
    ///
    /// # Security Note
    ///
    /// Still uses `SecurityLevel::High` and hardware-backed keys, but with
    /// relaxed rotation policies for better performance.
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
