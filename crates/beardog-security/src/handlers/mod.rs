//! Security Handlers - Modular Organization
//!
//! Refactored security handlers with focused responsibilities for maintainability.
//! 
//! ## Architecture
//! - `rate_limiting` - Request rate limiting and throttling
//! - `session_management` - User session creation and validation
//! - `mfa_handling` - Multi-factor authentication operations
//! - `threat_analysis` - Security threat detection and analysis
//! - `audit_management` - Security audit events and compliance
//! - `account_management` - Account locking and security policies
//! - `metrics_collection` - Security metrics and monitoring
//! - `maintenance` - Cleanup and maintenance operations
//! - `trait_implementation` - SecurityProvider trait implementation

use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};

use chrono::{Duration, Timelike, Utc};
use std::collections::HashMap;

use uuid::Uuid;

// Import AuditEvent from compliance crate
use beardog_compliance::AuditEvent;

// Import Argon2 for secure password verification
use argon2::{Argon2, PasswordHash, PasswordVerifier};

// Import memory key manager
use crate::memory_key_manager::{MemoryKeyConfig, MemoryKeyManager};

pub mod rate_limiting;
pub mod session_management;
pub mod mfa_handling;
pub mod threat_analysis;
pub mod audit_management;
pub mod account_management;
pub mod metrics_collection;
pub mod maintenance;
pub mod trait_implementation;

impl BearDogSecurityProvider {
    /// Create a new security provider instance
    pub async fn new(config: SecurityProviderConfig) -> BearDogResult<Self> {
        let rate_limiter = RateLimiter {
            config: RateLimitConfig::default(),
            state: HashMap::new(),
        };

        // Initialize standalone memory key manager for "crypto in your pocket"
        let _memory_key_manager = if let Some(_) = &config.memory_key_manager {
            let key_config = MemoryKeyConfig {
                max_keys: 1000, // Default value
                enable_vault_sharing: false, // Default value
                ..Default::default()
            };
            Some(MemoryKeyManager::new(key_config).await?)
        } else {
            None
        };

        // Initialize recovery system for distributed account recovery
        let _recovery_manager = Some(crate::recovery::RecoveryManager::new().await?);

        Ok(Self {
            config,
            rate_limiter,
            security_rules: SecurityRules {
                rules: Vec::new(),
                default_policy: PolicyDecision::Deny,
                context: SecurityContext::default(),
            },
            locked_accounts: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            metrics: SecurityProviderMetrics::default(),
            session_store: SessionStore::new(),
            audit_manager: AuditManager::new(),
        })
    }
} 