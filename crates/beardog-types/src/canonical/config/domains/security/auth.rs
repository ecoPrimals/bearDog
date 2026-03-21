// SPDX-License-Identifier: AGPL-3.0-only

//! Authentication and Authorization Configuration
//!
//! This module provides authentication, authorization, consensus, and access control
//! configuration structures for the BearDog security system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Authentication configuration - consolidates `AuthConfig` and related structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfiguration {
    /// Session timeout in seconds
    pub session_timeout_seconds: u64,
    /// Maximum login attempts before lockout
    pub max_login_attempts: u32,
    /// Enable multi-factor authentication
    pub enable_mfa: bool,
    /// Password hash rounds
    pub hash_rounds: u32,
    /// JWT token expiry duration
    pub jwt_expiry_seconds: u64,
    /// Enable cross-node authentication
    pub enable_cross_node_auth: bool,
    /// Authentication provider type
    pub provider_type: String,
}

impl AuthenticationConfiguration {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_parsed;

        Self {
            session_timeout_seconds: get_parsed(source, "BEARDOG_SESSION_TIMEOUT_SECS", 3600), // 1 hour
            max_login_attempts: get_parsed(source, "BEARDOG_MAX_LOGIN_ATTEMPTS", 5),
            enable_mfa: true,
            hash_rounds: get_parsed(source, "BEARDOG_PASSWORD_HASH_ROUNDS", 12),
            jwt_expiry_seconds: get_parsed(source, "BEARDOG_JWT_EXPIRY_SECS", 3600), // 1 hour
            enable_cross_node_auth: true,
            provider_type: "unified".to_string(),
        }
    }
}

impl Default for AuthenticationConfiguration {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

/// Authorization configuration - consolidates authorization-related configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationConfiguration {
    /// Enable role-based access control
    pub enable_rbac: bool,
    /// Enable attribute-based access control
    pub enable_abac: bool,
    /// Default user role
    pub default_role: String,
    /// Permission cache timeout seconds
    pub permission_cache_timeout: u64,
    /// Enable capability-based authorization
    pub enable_capability_auth: bool,
    /// Consensus configuration for distributed authorization
    pub consensus: ConsensusConfiguration,
}

/// Consensus configuration for distributed systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfiguration {
    /// Consensus algorithm type
    pub algorithm: String,
    /// Minimum consensus threshold
    pub threshold: f64,
    /// Consensus timeout in seconds
    pub timeout_seconds: u64,
    /// Enable Byzantine fault tolerance
    pub enable_bft: bool,
}

impl ConsensusConfiguration {
    /// Create configuration from a config source (modern pattern)
    ///
    /// This method uses dependency injection to accept any configuration source,
    /// making it thread-safe and testable without global state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_types::canonical::config::source::{ConfigSource, EnvConfigSource};
    /// use beardog_types::canonical::config::domains::security::ConsensusConfiguration;
    ///
    /// let source = EnvConfigSource::new();
    /// let config = ConsensusConfiguration::from_source(&source);
    /// ```
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::{get_bool, get_parsed};

        Self {
            algorithm: source.get_or("BEARDOG_CONSENSUS_ALGORITHM", "raft"),
            threshold: get_parsed(source, "BEARDOG_CONSENSUS_THRESHOLD", 0.67), // 2/3 majority
            timeout_seconds: get_parsed(source, "BEARDOG_CONSENSUS_TIMEOUT_SECS", 30),
            enable_bft: get_bool(source, "BEARDOG_CONSENSUS_BFT_ENABLED", true),
        }
    }
}

impl Default for ConsensusConfiguration {
    fn default() -> Self {
        // Production default uses environment variables
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

/// Access control configuration - consolidates `MembershipConfig` and related structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfiguration {
    /// Enable ecosystem membership model
    pub enable_ecosystem_membership: bool,
    /// Default membership level
    pub default_membership_level: String,
    /// Auto-evolution settings
    pub auto_evolution: AutoEvolutionConfiguration,
    /// Legacy integration settings
    pub legacy_integration: LegacyIntegrationConfiguration,
    /// Health monitoring for access control
    pub health_monitoring: HealthMonitoringConfiguration,
}

/// Auto-evolution configuration for adaptive access control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoEvolutionConfiguration {
    /// Enable automatic evolution of access patterns
    pub enable_auto_evolution: bool,
    /// Evolution threshold
    pub evolution_threshold: f64,
    /// Evolution interval in seconds
    pub evolution_interval_seconds: u64,
    /// Maximum evolution steps per interval
    pub max_evolution_steps: u32,
}

impl Default for AutoEvolutionConfiguration {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "default pool size fits u32 for evolution step limits"
    )]
    fn default() -> Self {
        Self {
            enable_auto_evolution: true,
            evolution_threshold: std::env::var("BEARDOG_EVOLUTION_THRESHOLD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.8),
            evolution_interval_seconds: std::env::var("BEARDOG_EVOLUTION_INTERVAL_SECS")
                .ok()
                .and_then(|i| i.parse().ok())
                .unwrap_or(3600), // 1 hour default
            max_evolution_steps: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE
                as u32,
        }
    }
}

/// Legacy integration configuration for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyIntegrationConfiguration {
    /// Enable legacy system integration
    pub enable_legacy_integration: bool,
    /// Legacy system timeout
    pub legacy_timeout_seconds: u64,
    /// Legacy authentication methods
    pub legacy_auth_methods: Vec<String>,
    /// Migration timeline
    pub migration_deadline: Option<String>,
}

/// Health monitoring configuration for security systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfiguration {
    /// Enable security health monitoring
    pub enable_monitoring: bool,
    /// Health check interval in seconds
    pub check_interval_seconds: u64,
    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
    /// Enable automated remediation
    pub enable_auto_remediation: bool,
}
