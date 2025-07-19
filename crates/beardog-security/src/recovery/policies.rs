//! Recovery Policies
//!
//! This module handles recovery policies and trust boundaries.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::*;

/// Recovery policy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPolicy {
    /// Maximum recovery attempts per day
    pub max_attempts_per_day: u32,
    /// Cooldown period between attempts (hours)
    pub cooldown_hours: u32,
    /// Whether to require additional verification
    pub require_additional_verification: bool,
    /// Types of additional verification required
    pub additional_verification_types: Vec<VerificationType>,
}

impl Default for RecoveryPolicy {
    fn default() -> Self {
        Self {
            max_attempts_per_day: 3,
            cooldown_hours: 1,
            require_additional_verification: false,
            additional_verification_types: Vec::new(),
        }
    }
}

/// User-controlled recovery policy configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserRecoveryPolicy {
    /// User ID this policy belongs to
    pub user_id: String,
    /// Whether to allow mixed recovery (multiple methods)
    pub allow_mixed_recovery: bool,
    /// Trust boundaries configuration
    pub trust_boundaries: UserTrustBoundaries,
    /// Recovery contexts (family, work, emergency, etc.)
    pub recovery_contexts: Vec<RecoveryContext>,
    /// Global recovery settings
    pub global_settings: HashMap<String, String>,
}

/// User-defined trust boundaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTrustBoundaries {
    /// Maximum number of recovery attempts per context
    pub max_attempts_per_context: u32,
    /// Require multiple contexts for high-value operations
    pub require_multiple_contexts: bool,
    /// Minimum trust level required
    pub min_trust_level: u8,
    /// Trust decay settings
    pub trust_decay_enabled: bool,
}

/// Recovery context defined by user (family, work, emergency, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryContext {
    /// Context ID
    pub id: String,
    /// Context name
    pub name: String,
    /// Context description
    pub description: String,
    /// Shard allocation for this context
    pub shard_allocation: ShardAllocation,
    /// Trust requirements for this context
    pub trust_requirements: ContextTrustRequirements,
}

/// How shards are allocated to a recovery context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardAllocation {
    /// Number of shards allocated to this context
    pub shard_count: u32,
    /// Minimum shards required from this context
    pub min_required: u32,
    /// Backup strategies for this context
    pub backup_strategies: Vec<BackupStrategy>,
}

/// Context-specific trust requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTrustRequirements {
    /// Minimum trust level required
    pub min_trust_level: u8,
    /// Require identity verification
    pub require_identity_verification: bool,
    /// Verification methods allowed
    pub allowed_verification_methods: Vec<VerificationMethod>,
}



impl Default for UserTrustBoundaries {
    fn default() -> Self {
        Self {
            max_attempts_per_context: 3,
            require_multiple_contexts: false,
            min_trust_level: 50,
            trust_decay_enabled: true,
        }
    }
}
