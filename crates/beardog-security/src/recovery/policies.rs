

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPolicy {

    pub max_attempts_per_day: u32,

    pub cooldown_hours: u32,

    pub require_additional_verification: bool,

    pub additional_verification_types: Vec<VerificationType>,
}
impl Default for RecoveryPolicy {}

    fn default() -> Self {
        Self {
            max_attempts_per_day: 3,
            cooldown_hours: 1,
            require_additional_verification: false,
            additional_verification_types: Vec::new(),
        }
    }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserRecoveryPolicy {

    pub user_id: String,

    pub allow_mixed_recovery: bool,

    pub trust_boundaries: UserTrustBoundaries,

    pub recovery_contexts: Vec<RecoveryContext>,

    pub global_settings: HashMap<String, String>,

pub struct UserTrustBoundaries {

    pub max_attempts_per_context: u32,

    pub require_multiple_contexts: bool,

    pub min_trust_level: u8,

    pub trust_decay_enabled: bool,

pub struct RecoveryContext {

    pub id: String,

    pub name: String,

    pub description: String,

    pub shard_allocation: ShardAllocation,

    pub trust_requirements: ContextTrustRequirements,

pub struct ShardAllocation {

    pub shard_count: u32,

    pub min_required: u32,

    pub backup_strategies: Vec<BackupStrategy>,

pub struct ContextTrustRequirements {

    pub require_identity_verification: bool,

    pub allowed_verification_methods: Vec<VerificationMethod>,}

impl Default for UserTrustBoundaries {
            max_attempts_per_context: 3,
            require_multiple_contexts: false,
            min_trust_level: 50,
            trust_decay_enabled: true,
